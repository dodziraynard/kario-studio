//! Per-job worker.
//!
//! Pipeline for each request:
//!   1. Resolve the kario-studio repo root.
//!   2. Ensure a **persistent build workspace** exists at `storage/cache/workspace/`
//!      (copied once from `exporter-base/`, workspace-detached, never deleted).
//!   3. Pick the next sequential run number for the project.
//!   4. Acquire a global mutex so only one cargo invocation runs at a time
//!      (Cargo cannot share the build workspace safely across concurrent builds).
//!   5. Invoke `claude --print` (headless) with the user prompt to generate a new
//!      `video_scene.rs` and write it into the persistent workspace.
//!   6. `cargo run --release` from the **fixed** workspace path so Cargo fingerprints
//!      match across runs – only `video_scene.rs` recompiles.
//!   7. Move the rendered mp4 to `storage/renders/<project>/<run>/output.mp4`.
//!   8. Update the job state with the result.

use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use anyhow::{Context, Result};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::Mutex,
};
use uuid::Uuid;

use crate::{JobId, JobStatus, JobStore};

/// Global mutex – serialises cargo runs so they share the build workspace safely.
static BUILD_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
fn build_lock() -> &'static Mutex<()> {
    BUILD_LOCK.get_or_init(|| Mutex::new(()))
}

const DEFAULT_FPS: u32 = 30;

/// Walk upward from the current executable until we find the workspace `Cargo.toml`.
fn studio_root() -> PathBuf {
    let exe = std::env::current_exe().unwrap_or_default();
    let mut dir = exe.parent().map(Path::to_path_buf).unwrap_or_default();
    for _ in 0..6 {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists()
            && std::fs::read_to_string(&manifest)
                .map(|s| s.contains("[workspace]"))
                .unwrap_or(false)
        {
            return dir;
        }
        match dir.parent() {
            Some(p) => dir = p.to_path_buf(),
            None => break,
        }
    }
    std::env::current_dir().unwrap_or_default()
}

pub fn project_dir(project_id: &Uuid) -> PathBuf {
    studio_root()
        .join("storage/workspaces")
        .join(project_id.simple().to_string())
}

pub fn render_project_dir(project_id: &Uuid) -> PathBuf {
    studio_root()
        .join("storage/renders")
        .join(project_id.simple().to_string())
}

pub fn render_path(project_id: &Uuid, run_num: u32) -> PathBuf {
    render_project_dir(project_id)
        .join(run_num.to_string())
        .join("output.mp4")
}

/// Next sequential run number (1-based) by inspecting existing run dirs.
fn next_run_number(project_dir: &Path) -> u32 {
    std::fs::read_dir(project_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| e.file_name().to_str()?.parse::<u32>().ok())
        .max()
        .unwrap_or(0)
        + 1
}

// ── filesystem helpers ───────────────────────────────────────────────────────

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_name() == "target" {
            continue;
        }
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// Append a `[workspace]` table so Cargo treats the copied package as its own
/// standalone workspace instead of attaching it to kario-studio's.
fn detach_workspace(cargo_toml: &Path) -> Result<()> {
    let original = std::fs::read_to_string(cargo_toml)
        .with_context(|| format!("read {}", cargo_toml.display()))?;
    if original.contains("\n[workspace]") || original.starts_with("[workspace]") {
        return Ok(());
    }
    let sep = if original.ends_with('\n') { "" } else { "\n" };
    std::fs::write(cargo_toml, format!("{original}{sep}\n[workspace]\n"))
        .with_context(|| format!("write {}", cargo_toml.display()))?;
    Ok(())
}

// ── Claude codegen ────────────────────────────────────────────────────────────────────────

/// Read the current `video_scene.rs` from the build workspace as context for Claude.
fn read_current_scene(bw: &Path) -> String {
    std::fs::read_to_string(bw.join("src/video_scene.rs")).unwrap_or_default()
}

const ELEVENLABS_VOICE_ID: &str = "JBFqnCBsd6RMkjVDRZzb"; // "George" — change as needed
const WHISPERX_BIN: &str =
    "/Users/speechdata/repositories/kario-dataset/scripts/.venv/bin/whisperx";
const FALLBACK_TEMPLATE: &str = "product_launch";

/// Call OpenAI text-embedding-3-small and return the 1536-dim vector.
pub async fn embed_prompt(text: &str) -> Result<Vec<f64>> {
    let api_key = std::env::var("OPENAI_API_KEY").context("OPENAI_API_KEY env var not set")?;
    let body = serde_json::json!({
        "input": text,
        "model": "text-embedding-3-small"
    });
    let resp = reqwest::Client::new()
        .post("https://api.openai.com/v1/embeddings")
        .bearer_auth(&api_key)
        .json(&body)
        .send()
        .await
        .context("OpenAI embeddings request")?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("OpenAI embeddings error {status}: {text}");
    }
    let json: serde_json::Value = resp.json().await.context("parse embeddings response")?;
    let embedding = json["data"][0]["embedding"]
        .as_array()
        .context("missing embedding array")?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0))
        .collect();
    Ok(embedding)
}

/// Copy the selected template's files (video_scene.rs + any asset dirs) into the run dir.
fn apply_base_template(root: &Path, run_dir: &Path, template_name: &str) -> Result<()> {
    let template_dir = root.join("storage/templates").join(template_name);
    if !template_dir.is_dir() {
        // Fall back to the default template if the selected one doesn't exist on disk.
        log::warn!(
            "Template '{template_name}' not found on disk, falling back to {FALLBACK_TEMPLATE}"
        );
        return apply_base_template(root, run_dir, FALLBACK_TEMPLATE);
    }
    for entry in std::fs::read_dir(&template_dir)
        .with_context(|| format!("read template dir {}", template_dir.display()))?
    {
        let entry = entry?;
        let name = entry.file_name();
        let from = entry.path();
        if name == "video_scene.rs" {
            std::fs::copy(&from, run_dir.join("src/video_scene.rs"))
                .with_context(|| format!("copy {}", from.display()))?;
        } else if entry.file_type()?.is_dir() {
            copy_dir_recursive(&from, &run_dir.join(&name))
                .with_context(|| format!("copy dir {}", from.display()))?;
        } else {
            std::fs::copy(&from, run_dir.join(&name))
                .with_context(|| format!("copy {}", from.display()))?;
        }
    }
    Ok(())
}

/// Run the claude CLI, streaming JSON output to logs. Returns the final text result.
/// Pass `add_dir = Some(path)` only when Claude needs file-system access.
async fn run_claude(prompt: &str, add_dir: Option<&Path>) -> Result<String> {
    let mut cmd = Command::new("claude");
    cmd.arg("--print")
        .arg("--output-format")
        .arg("stream-json")
        .arg("--verbose");

    if let Some(dir) = add_dir {
        // Scene generation: write access only — no exploring, no web.
        cmd.arg("--add-dir")
            .arg(dir)
            .arg("--allowedTools")
            .arg("Write,Edit");
    } else {
        // Script generation: web-only, no filesystem access, no agent tools.
        cmd.arg("--allowedTools")
            .arg("WebFetch,WebSearch")
            .arg("--disallowedTools")
            .arg("Task,Bash,Explore");
    }
    let mut child = cmd
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdin(std::process::Stdio::piped())
        .env("CLAUDE_CODE_MAX_OUTPUT_TOKENS", "64000")
        .spawn()
        .context("spawn claude CLI — is `claude` on PATH?")?;

    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        stdin
            .write_all(prompt.as_bytes())
            .await
            .context("write prompt to claude stdin")?;
    }

    let mut result_text = String::new();
    if let Some(stdout) = child.stdout.take() {
        let mut lines = BufReader::new(stdout).lines();
        while let Some(line) = lines.next_line().await? {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) {
                if let Some(text) = v["message"]["content"]
                    .as_array()
                    .and_then(|arr| arr.first())
                    .and_then(|c| c["text"].as_str())
                {
                    if !text.is_empty() {
                        eprint!("{text}");
                    }
                }
                if v["type"] == "tool_use" {
                    if let Some(name) = v["name"].as_str() {
                        log::info!("  🔧 claude tool: {name}");
                    }
                }
                if v["type"] == "result" {
                    let cost = v["total_cost_usd"].as_f64().unwrap_or(0.0);
                    let ms = v["duration_ms"].as_u64().unwrap_or(0);
                    log::info!("  💰 claude done in {ms}ms (${cost:.4})");
                    if let Some(t) = v["result"].as_str() {
                        result_text = t.to_string();
                    }
                }
            }
        }
        eprintln!();
    }

    let status = child.wait().await.context("wait for claude")?;
    if !status.success() {
        anyhow::bail!("claude exited with {status}");
    }
    Ok(result_text)
}

/// Step 1 — Ask Claude to write a VO script for the user's prompt.
async fn generate_script(prompt: &str) -> Result<String> {
    log::info!("📝 Generating script…");
    let script = run_claude(
        &format!(
            r#"Write a concise, punchy voice-over script for a 30–50 second product video.
Output ONLY the spoken words — no stage directions, no scene descriptions, no markdown, no preamble.
Do NOT explore files or ask questions. Write the script immediately based solely on the request below.
If the request contains a URL, use WebFetch to get context from it first, then write the script.

User request: {prompt}"#
        ),
        None,
    )
    .await?;
    log::info!(
        "📝 Script ({} chars): {}",
        script.len(),
        &script[..script.len().min(120)]
    );
    Ok(script)
}

/// Step 2 — Send script to ElevenLabs TTS, save mp3 to `run_dir/assets/vo.mp3`.
async fn generate_tts(script: &str, run_dir: &Path) -> Result<PathBuf> {
    let api_key =
        std::env::var("ELEVENLABS_API_KEY").context("ELEVENLABS_API_KEY env var not set")?;

    let assets_dir = run_dir.join("assets");
    tokio::fs::create_dir_all(&assets_dir).await?;
    let mp3_path = assets_dir.join("vo.mp3");

    log::info!("🔊 Calling ElevenLabs TTS…");

    let body = serde_json::json!({
        "text": script,
        "model_id": "eleven_turbo_v2_5",
        "voice_settings": { "stability": 0.5, "similarity_boost": 0.75 }
    });

    let url = format!("https://api.elevenlabs.io/v1/text-to-speech/{ELEVENLABS_VOICE_ID}");

    let response = reqwest::Client::new()
        .post(&url)
        .header("xi-api-key", &api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .context("ElevenLabs request failed")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("ElevenLabs error {status}: {body}");
    }

    let bytes = response.bytes().await.context("read ElevenLabs response")?;
    tokio::fs::write(&mp3_path, &bytes)
        .await
        .context("write vo.mp3")?;
    log::info!(
        "🔊 TTS saved to {} ({} bytes)",
        mp3_path.display(),
        bytes.len()
    );
    Ok(mp3_path)
}

// ── phrase-level timing (mirrors pipeline.py `_phrases_from_words`) ──────────

#[derive(Debug)]
struct Phrase {
    text: String,
    start: f64,
    end: f64,
}

/// Conjunctions that trigger a phrase split *before* the word (same list as pipeline.py).
const CONJUNCTIONS: &[&str] = &[
    "and", "but", "or", "so", "yet", "nor", "although", "because", "since", "while",
];

/// Re-group word-level timestamps into sub-sentence phrases.
///
/// Splits AFTER any word ending with `, . ! ? ;`
/// Splits BEFORE a conjunction when there are already words in the current phrase.
fn phrases_from_words(words: &[serde_json::Value]) -> Vec<Phrase> {
    let mut phrases: Vec<Phrase> = Vec::new();
    let mut current: Vec<&serde_json::Value> = Vec::new();

    let flush = |current: &mut Vec<&serde_json::Value>, phrases: &mut Vec<Phrase>| {
        if current.is_empty() {
            return;
        }
        let text = current
            .iter()
            .map(|w| w["word"].as_str().unwrap_or("").trim())
            .collect::<Vec<_>>()
            .join(" ");
        let start = current
            .first()
            .and_then(|w| w["start"].as_f64())
            .unwrap_or(0.0);
        let end = current
            .last()
            .and_then(|w| w["end"].as_f64())
            .unwrap_or(start);
        phrases.push(Phrase { text, start, end });
        current.clear();
    };

    for w in words {
        let text = w["word"].as_str().unwrap_or("").trim();
        let bare = text
            .trim_end_matches(|c: char| matches!(c, '.' | ',' | '!' | '?' | ';' | ':'))
            .to_lowercase();

        // Split before conjunction when we already have content
        if CONJUNCTIONS.contains(&bare.as_str()) && !current.is_empty() {
            flush(&mut current, &mut phrases);
        }

        current.push(w);

        // Split after punctuation
        if text.ends_with([',', '.', '!', '?', ';']) {
            flush(&mut current, &mut phrases);
        }
    }
    flush(&mut current, &mut phrases);
    phrases
}

/// Parse the whisperx JSON and return phrase-level timings as a formatted string
/// suitable for inclusion in Claude's prompt.
fn build_phrase_timings(timings_json: &str) -> String {
    let v: serde_json::Value = match serde_json::from_str(timings_json) {
        Ok(v) => v,
        Err(_) => return timings_json.to_string(),
    };

    // Try word_segments first; fall back to flattening segments[].words[]
    let flat_fallback: Vec<serde_json::Value>;
    let words: &[serde_json::Value] = if let Some(ws) = v["word_segments"].as_array() {
        ws
    } else if let Some(segs) = v["segments"].as_array() {
        flat_fallback = segs
            .iter()
            .flat_map(|s| s["words"].as_array().cloned().unwrap_or_default())
            .collect();
        &flat_fallback
    } else {
        return timings_json.to_string();
    };

    // Filter to words that have both start and end
    let valid: Vec<serde_json::Value> = words
        .iter()
        .filter(|w| w["start"].as_f64().is_some() && w["end"].as_f64().is_some())
        .cloned()
        .collect();

    if valid.is_empty() {
        return timings_json.to_string();
    }

    let phrases = phrases_from_words(&valid);

    let total_dur = phrases.last().map(|p| p.end).unwrap_or(0.0);
    let mut out = format!("// {n} phrases, total {total_dur:.2}s\n", n = phrases.len());
    out.push_str("// format: start_s | duration_s | text\n");
    for p in &phrases {
        let dur = p.end - p.start;
        out.push_str(&format!("{:.3} | {:.3} | {}\n", p.start, dur, p.text));
    }
    out
}

/// Step 3 — Run whisperx on the mp3, return the word-level JSON as a string.
async fn run_whisperx(mp3_path: &Path, run_dir: &Path) -> Result<String> {
    log::info!("⏱️  Running whisperx for word timings…");
    let output_dir = run_dir.join("assets");

    let status = Command::new(WHISPERX_BIN)
        .arg(mp3_path)
        .arg("--model")
        .arg("base")
        .arg("--language")
        .arg("en")
        .arg("--output_dir")
        .arg(&output_dir)
        .arg("--output_format")
        .arg("json")
        .arg("--compute_type")
        .arg("float32")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .await
        .context("spawn whisperx")?;

    if !status.success() {
        anyhow::bail!("whisperx exited with {status}");
    }

    // whisperx writes <stem>.json next to the audio file in output_dir
    let stem = mp3_path.file_stem().unwrap_or_default().to_string_lossy();
    let json_path = output_dir.join(format!("{stem}.json"));
    let json = tokio::fs::read_to_string(&json_path)
        .await
        .with_context(|| format!("read whisperx output {}", json_path.display()))?;

    log::info!("⏱️  Whisperx done ({} bytes of timing data)", json.len());
    Ok(json)
}

/// Step 4 — Ask Claude to write video_scene.rs using the script + word timings.
async fn generate_scene(
    prompt: &str,
    script: &str,
    phrase_timings: &str,
    run_dir: &Path,
) -> Result<()> {
    let run_str = run_dir.to_string_lossy();
    let current = read_current_scene(run_dir);

    let full_prompt = format!(
        r#"You are a Rust code generator for a motion-graphics video engine called kario.

You have write access to the run directory at `{run_str}`.
Edit `src/video_scene.rs` in place to create a video matching the user's request.

## Voice-over script
```
{script}
```

## Phrase-level timing data
Each line: `start_seconds | duration_seconds | spoken text`
Use these timestamps to lock visual beats to the narration — each visual element
that corresponds to a phrase MUST appear on screen at or just before that phrase's
start time. The VO audio file is at `assets/vo.mp3` relative to the run dir.

```
{phrase_timings}
```

## Required contract for src/video_scene.rs
- Declare `pub fn build() -> Scene` as the only public symbol.
- `use kario_base::{{Asset, Audio, AudioAsset, Clip, Composition, Duration, Scene, ...}}` — import what you need.
- Canvas is 1280×720. Use `const W: f32 = 1280.0; const H: f32 = 720.0;`.
- Set `VO_SRC` to `"assets/vo.mp3"` and add it as an Audio layer on the scene.
- Return a fully configured `Scene` with at least one `Composition`.
- Do NOT emit markdown — write valid Rust only.

## Visual design rules (must follow)
- **No paragraphs of text on screen.** Short labels, single keywords, or brief phrases only — never multi-sentence blocks.
- **Use visual metaphors to support the voice-over.** Each scene beat should reinforce the narration with a concrete visual idea (icons, shapes, motion, colour transitions) rather than just repeating the words as text.

## Current src/video_scene.rs (base template for reference)
```rust
{current}
```

## User request
{prompt}"#
    );

    log::info!("🎬 Generating video_scene.rs with script + timings…");
    run_claude(&full_prompt, Some(run_dir)).await?;
    log::info!("✅ Claude finished writing video_scene.rs");
    Ok(())
}

const MAX_FIX_ATTEMPTS: u32 = 3;

/// Run `cargo check` on the run dir, returning `Ok(())` on success or
/// `Err(compiler_stderr)` with the filtered error lines on failure.
async fn cargo_check_scene(run_dir: &Path, shared_target: &Path) -> Result<(), String> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let output = Command::new(&cargo)
        .arg("check")
        .arg("--manifest-path")
        .arg(run_dir.join("Cargo.toml"))
        .arg("--message-format=short")
        .env("CARGO_TARGET_DIR", shared_target)
        .env("RUSTFLAGS", "-A warnings")
        .output()
        .await
        .map_err(|e| format!("spawn cargo check: {e}"))?;

    if output.status.success() {
        return Ok(());
    }

    let raw = String::from_utf8_lossy(&output.stderr);
    // Keep only error lines, drop warnings and "aborting" summary
    let errors: Vec<&str> = raw
        .lines()
        .filter(|l| l.contains("error") && !l.starts_with("warning"))
        .take(30)
        .collect();
    Err(errors.join("\n"))
}

/// Ask Claude to fix a broken `video_scene.rs` given the compiler errors.
async fn fix_scene(
    prompt: &str,
    script: &str,
    phrase_timings: &str,
    errors: &str,
    run_dir: &Path,
) -> Result<()> {
    let run_str = run_dir.to_string_lossy();
    let current = read_current_scene(run_dir);

    let fix_prompt = format!(
        r#"You are a Rust code generator for a motion-graphics video engine called kario.

The file `src/video_scene.rs` you previously wrote failed to compile.
Read the compiler errors carefully, then edit `src/video_scene.rs` in place to fix them.
Do NOT rewrite the whole file unless necessary — make targeted fixes only.

## Visual design rules (preserve these during fixes)
- **No paragraphs of text on screen.** Short labels, single keywords, or brief phrases only.
- **Use visual metaphors to support the voice-over.** Reinforce narration with concrete visuals, not text repetition.

## Compiler errors
```
{errors}
```

## Current (broken) src/video_scene.rs
```rust
{current}
```

## Context (for reference — do not change audio setup or timing values)
Voice-over script: {script}

Phrase timings (start | duration | text):
{phrase_timings}

Run directory: `{run_str}`
User request: {prompt}"#
    );

    run_claude(&fix_prompt, Some(run_dir)).await?;
    Ok(())
}

// ── pipeline ─────────────────────────────────────────────────────────────────

/// Set up the run directory (copy chassis + template). Returns (run_dir, output_path).
async fn run_job_setup(
    project_id: JobId,
    prompt: &str,
    template_name: &str,
) -> Result<(PathBuf, PathBuf)> {
    let root = studio_root();

    let proj_dir = project_dir(&project_id);
    tokio::fs::create_dir_all(&proj_dir).await?;

    let run_num = next_run_number(&proj_dir);
    let run_dir = proj_dir.join(run_num.to_string());

    let output = render_path(&project_id, run_num);
    if let Some(p) = output.parent() {
        tokio::fs::create_dir_all(p).await?;
    }

    // Copy exporter-base into this run's isolated directory.
    let src = root.join("exporter-base");
    let run_dir_clone = run_dir.clone();
    tokio::task::spawn_blocking(move || copy_dir_recursive(&src, &run_dir_clone))
        .await
        .context("copy task panicked")?
        .context("copy exporter-base to run dir")?;
    detach_workspace(&run_dir.join("Cargo.toml"))?;

    // Copy the selected template into the run dir.
    let run_dir_clone = run_dir.clone();
    let root_clone = root.clone();
    let template_name_owned = template_name.to_string();
    let template_name_log = template_name_owned.clone();
    tokio::task::spawn_blocking(move || {
        apply_base_template(&root_clone, &run_dir_clone, &template_name_owned)
    })
    .await
    .context("template copy panicked")?
    .context("apply base template")?;

    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  kario run #{run_num}  •  {}", run_dir.display());
    eprintln!("  template: {template_name_log} | prompt: {prompt}");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    Ok((run_dir, output))
}

// Thin wrapper that drives run_job_inner and emits step updates to the store.
async fn run_job(
    store: &Arc<JobStore>,
    run_id: JobId,
    project_id: JobId,
    prompt: &str,
) -> Result<PathBuf> {
    let root = studio_root();

    let template_name = store.pick_template(prompt).await;
    let (run_dir, output) = run_job_setup(project_id, prompt, &template_name).await?;

    // Step 1: script
    store.step_start(&run_id, "Writing script").await;
    eprintln!("[1/5] 📝  Generating VO script…");
    let script = match generate_script(prompt).await {
        Ok(s) => {
            store
                .step_done(
                    &run_id,
                    "Writing script",
                    Some(format!("{} words", s.split_whitespace().count())),
                )
                .await;
            s
        }
        Err(e) => {
            store
                .step_fail(&run_id, "Writing script", format!("{e:#}"))
                .await;
            return Err(e);
        }
    };
    eprintln!(
        "[1/5] ✅  Script ({} words)",
        script.split_whitespace().count()
    );

    // Persist the script so future edit runs can reuse it.
    tokio::fs::create_dir_all(run_dir.join("assets")).await?;
    tokio::fs::write(run_dir.join("assets/script.txt"), &script).await?;

    // Step 2: TTS
    store.step_start(&run_id, "Generating voiceover").await;
    eprintln!("[2/5] 🔊  Generating voiceover audio…");
    let mp3_path = match generate_tts(&script, &run_dir).await {
        Ok(p) => {
            store.step_done(&run_id, "Generating voiceover", None).await;
            p
        }
        Err(e) => {
            store
                .step_fail(&run_id, "Generating voiceover", format!("{e:#}"))
                .await;
            return Err(e);
        }
    };
    eprintln!("[2/5] ✅  Audio saved → {}", mp3_path.display());

    // Step 3: whisperx
    store.step_start(&run_id, "Analysing audio timing").await;
    eprintln!("[3/5] ⏱️   Analysing audio timing…");
    let timings_json = match run_whisperx(&mp3_path, &run_dir).await {
        Ok(j) => {
            store
                .step_done(&run_id, "Analysing audio timing", None)
                .await;
            j
        }
        Err(e) => {
            store
                .step_fail(&run_id, "Analysing audio timing", format!("{e:#}"))
                .await;
            return Err(e);
        }
    };
    eprintln!("[3/5] ✅  Timings ready ({} bytes)", timings_json.len());

    // Step 4: scene codegen
    let phrase_timings = build_phrase_timings(&timings_json);
    store.step_start(&run_id, "Designing scenes").await;
    eprintln!("[4/5] 🎬  Designing scenes…");
    if let Err(e) = generate_scene(prompt, &script, &phrase_timings, &run_dir).await {
        store
            .step_fail(&run_id, "Designing scenes", format!("{e:#}"))
            .await;
        return Err(e);
    }
    store.step_done(&run_id, "Designing scenes", None).await;
    eprintln!("[4/5] ✅  Scenes written");

    // Step 5: build
    store.step_start(&run_id, "Rendering video").await;
    let _guard = build_lock().lock().await;
    let shared_target = root.join("storage/cache/target");

    for attempt in 1..=MAX_FIX_ATTEMPTS {
        eprintln!("\n[5/5] 🏗️   cargo check (attempt {attempt}/{MAX_FIX_ATTEMPTS})…");
        match cargo_check_scene(&run_dir, &shared_target).await {
            Ok(()) => {
                eprintln!("[5/5] ✅  Compile OK — running…");
                break;
            }
            Err(errors) => {
                eprintln!("[5/5] ❌  Compile errors:\n{errors}");
                if attempt == MAX_FIX_ATTEMPTS {
                    let msg =
                        format!("cargo check failed after {MAX_FIX_ATTEMPTS} attempts:\n{errors}");
                    store
                        .step_fail(&run_id, "Rendering video", msg.clone())
                        .await;
                    anyhow::bail!("{msg}");
                }
                eprintln!(
                    "[5/5] 🔧  Fixing issues (attempt {attempt}/{})…",
                    MAX_FIX_ATTEMPTS - 1
                );
                fix_scene(prompt, &script, &phrase_timings, &errors, &run_dir).await?;
            }
        }
    }

    eprintln!("[5/5] 🎬  cargo run --release…");
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let status = Command::new(&cargo)
        .arg("run")
        .arg("--release")
        .arg("--manifest-path")
        .arg(run_dir.join("Cargo.toml"))
        .arg("--")
        .arg(&output)
        .arg(DEFAULT_FPS.to_string())
        .current_dir(&run_dir)
        .env("CARGO_TARGET_DIR", &shared_target)
        .env("RUST_LOG", "info")
        .status()
        .await
        .context("spawn cargo")?;

    if !status.success() {
        let msg = format!("cargo run exited with {status}");
        store
            .step_fail(&run_id, "Rendering video", msg.clone())
            .await;
        anyhow::bail!("{msg}");
    }

    store.step_done(&run_id, "Rendering video", None).await;
    eprintln!("[5/5] ✅  Render complete → {}", output.display());
    eprintln!();
    Ok(output)
}

pub async fn spawn_worker(store: Arc<JobStore>, project_id: JobId, run_id: JobId, prompt: String) {
    log::info!("📥 Job {run_id} (project {project_id}) prompt: {prompt:?}");
    store
        .update(&run_id, |j| j.status = JobStatus::Running)
        .await;

    match run_job(&store, run_id, project_id, &prompt).await {
        Ok(output) => {
            log::info!("✅ Job {run_id} → {}", output.display());
            let url = output
                .strip_prefix(studio_root().join("storage/renders"))
                .ok()
                .and_then(|p| p.to_str().map(str::to_string))
                .map(|rel| format!("/api/renders/{rel}"))
                .unwrap_or_else(|| format!("/api/renders/{run_id}"));
            store
                .update(&run_id, |j| {
                    j.status = JobStatus::Done;
                    j.render_url = Some(url.clone());
                })
                .await;
        }
        Err(err) => {
            log::error!("❌ Job {run_id} failed: {err:#}");
            store
                .update(&run_id, |j| {
                    j.status = JobStatus::Failed;
                    j.error = Some(format!("{err:#}"));
                })
                .await;
        }
    }
}

// ── Edit pipeline ─────────────────────────────────────────────────────────────

/// Edit pipeline — skips script/TTS/whisperx, reuses assets from the latest run.
async fn run_job_edit(
    store: &Arc<JobStore>,
    run_id: JobId,
    project_id: JobId,
    prompt: &str,
) -> Result<PathBuf> {
    let root = studio_root();
    let proj_dir = project_dir(&project_id);

    // Find the most recent run dir that has audio assets (skip in-progress runs).
    let prev_run = std::fs::read_dir(&proj_dir)
        .context("read project dir")?
        .flatten()
        .filter_map(|e| {
            e.file_name()
                .to_str()?
                .parse::<u32>()
                .ok()
                .map(|n| (n, e.path()))
        })
        .filter(|(_, p)| p.join("assets/vo.mp3").exists())
        .max_by_key(|(n, _)| *n)
        .map(|(_, p)| p)
        .with_context(|| format!("no completed runs with assets found for project {project_id}"))?;

    let mp3_path = prev_run.join("assets/vo.mp3");
    let json_path = prev_run.join("assets/vo.json");
    let script_path = prev_run.join("assets/script.txt");

    anyhow::ensure!(mp3_path.exists(), "previous run missing assets/vo.mp3");
    anyhow::ensure!(json_path.exists(), "previous run missing assets/vo.json");

    let script = tokio::fs::read_to_string(&script_path)
        .await
        .context("read script.txt — was a generate job completed for this project?")?;
    let timings_json = tokio::fs::read_to_string(&json_path)
        .await
        .context("read vo.json")?;

    // New run dir
    let run_num = next_run_number(&proj_dir);
    let run_dir = proj_dir.join(run_num.to_string());

    let output = render_path(&project_id, run_num);
    if let Some(p) = output.parent() {
        tokio::fs::create_dir_all(p).await?;
    }

    // Copy exporter-base chassis
    let src = root.join("exporter-base");
    let run_dir_clone = run_dir.clone();
    tokio::task::spawn_blocking(move || copy_dir_recursive(&src, &run_dir_clone))
        .await
        .context("copy task panicked")?
        .context("copy exporter-base to run dir")?;
    detach_workspace(&run_dir.join("Cargo.toml"))?;

    // Apply selected template
    let template_name = store.pick_template(prompt).await;
    let run_dir_clone = run_dir.clone();
    let root_clone = root.clone();
    let template_name_owned = template_name.clone();
    tokio::task::spawn_blocking(move || {
        apply_base_template(&root_clone, &run_dir_clone, &template_name_owned)
    })
    .await
    .context("template copy panicked")?
    .context("apply base template")?;

    // Copy assets from previous run
    let assets_dst = run_dir.join("assets");
    tokio::fs::create_dir_all(&assets_dst).await?;
    tokio::fs::copy(&mp3_path, assets_dst.join("vo.mp3"))
        .await
        .context("copy vo.mp3")?;
    tokio::fs::copy(&json_path, assets_dst.join("vo.json"))
        .await
        .context("copy vo.json")?;
    tokio::fs::write(assets_dst.join("script.txt"), &script).await?;

    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  kario edit run #{run_num}  •  {}", run_dir.display());
    eprintln!("  prompt: {prompt}");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Step 1: codegen
    let phrase_timings = build_phrase_timings(&timings_json);
    store.step_start(&run_id, "Designing scenes").await;
    eprintln!("[1/2] 🎬  Designing scenes…");
    if let Err(e) = generate_scene(prompt, &script, &phrase_timings, &run_dir).await {
        store
            .step_fail(&run_id, "Designing scenes", format!("{e:#}"))
            .await;
        return Err(e);
    }
    store.step_done(&run_id, "Designing scenes", None).await;
    eprintln!("[1/2] ✅  Scenes written");

    // Step 2: build
    store.step_start(&run_id, "Rendering video").await;
    let _guard = build_lock().lock().await;
    let shared_target = root.join("storage/cache/target");

    for attempt in 1..=MAX_FIX_ATTEMPTS {
        eprintln!("\n[2/2] 🏗️   cargo check (attempt {attempt}/{MAX_FIX_ATTEMPTS})…");
        match cargo_check_scene(&run_dir, &shared_target).await {
            Ok(()) => {
                eprintln!("[2/2] ✅  Compile OK — running…");
                break;
            }
            Err(errors) => {
                eprintln!("[2/2] ❌  Compile errors:\n{errors}");
                if attempt == MAX_FIX_ATTEMPTS {
                    let msg =
                        format!("cargo check failed after {MAX_FIX_ATTEMPTS} attempts:\n{errors}");
                    store
                        .step_fail(&run_id, "Rendering video", msg.clone())
                        .await;
                    anyhow::bail!("{msg}");
                }
                eprintln!("[2/2] 🔧  Fixing issues…");
                fix_scene(prompt, &script, &phrase_timings, &errors, &run_dir).await?;
            }
        }
    }

    eprintln!("[2/2] 🎬  cargo run --release…");
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let status = Command::new(&cargo)
        .arg("run")
        .arg("--release")
        .arg("--manifest-path")
        .arg(run_dir.join("Cargo.toml"))
        .arg("--")
        .arg(&output)
        .arg(DEFAULT_FPS.to_string())
        .current_dir(&run_dir)
        .env("CARGO_TARGET_DIR", &shared_target)
        .env("RUST_LOG", "info")
        .status()
        .await
        .context("spawn cargo")?;

    if !status.success() {
        let msg = format!("cargo run exited with {status}");
        store
            .step_fail(&run_id, "Rendering video", msg.clone())
            .await;
        anyhow::bail!("{msg}");
    }

    store.step_done(&run_id, "Rendering video", None).await;
    eprintln!("[2/2] ✅  Render complete → {}", output.display());
    eprintln!();
    Ok(output)
}

pub async fn spawn_worker_edit(
    store: Arc<JobStore>,
    project_id: JobId,
    run_id: JobId,
    prompt: String,
) {
    log::info!("📥 Edit job {run_id} (project {project_id}) prompt: {prompt:?}");
    store
        .update(&run_id, |j| j.status = JobStatus::Running)
        .await;

    match run_job_edit(&store, run_id, project_id, &prompt).await {
        Ok(output) => {
            log::info!("✅ Edit job {run_id} → {}", output.display());
            let url = output
                .strip_prefix(studio_root().join("storage/renders"))
                .ok()
                .and_then(|p| p.to_str().map(str::to_string))
                .map(|rel| format!("/api/renders/{rel}"))
                .unwrap_or_else(|| format!("/api/renders/{run_id}"));
            store
                .update(&run_id, |j| {
                    j.status = JobStatus::Done;
                    j.render_url = Some(url.clone());
                })
                .await;
        }
        Err(err) => {
            log::error!("❌ Edit job {run_id} failed: {err:#}");
            store
                .update(&run_id, |j| {
                    j.status = JobStatus::Failed;
                    j.error = Some(format!("{err:#}"));
                })
                .await;
        }
    }
}
