use kario_base::{Asset, Context, Duration, Id, Scene};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        let resp = ureq::get(url)
            .set(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .call()
            .map_err(|e| format!("Failed to GET {}: {}", url, e))?;
        let mut bytes = Vec::new();
        resp.into_reader()
            .read_to_end(&mut bytes)
            .map_err(|e| format!("Failed to read bytes from {}: {}", url, e))?;
        Ok(bytes)
    } else {
        let mut file = File::open(Path::new(url))
            .map_err(|e| format!("Failed to open file {}: {}", url, e))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| format!("Failed to read file {}: {}", url, e))?;
        Ok(buf)
    }
}

pub fn preload_assets(scene: &mut Scene, debug: bool) {
    let images: Vec<(Id, String)> = scene
        .get_images()
        .into_iter()
        .map(|(id, img)| (id.clone(), img.src.clone()))
        .collect();

    for (id, src) in images {
        if src.is_empty() {
            continue;
        }
        if debug {
            log::info!("Preloading image: {}", src);
        }
        match fetch_bytes(&src) {
            Ok(data) => match image::load_from_memory(&data) {
                Ok(img) => {
                    let rgba = img.to_rgba8();
                    let (w, h) = rgba.dimensions();
                    scene.update_image_data(&id, w, h, rgba.into_raw());
                }
                Err(e) => log::warn!("Failed to decode image '{}': {}", src, e),
            },
            Err(e) => log::warn!("Failed to fetch image '{}': {}", src, e),
        }
    }

    let fonts: Vec<(Id, String)> = scene
        .get_fonts()
        .into_iter()
        .map(|(id, font)| (id.clone(), font.src.clone()))
        .collect();

    for (id, src) in fonts {
        if src.is_empty() {
            continue;
        }
        if debug {
            log::info!("Preloading font: {}", src);
        }
        match fetch_bytes(&src) {
            Ok(data) => {
                scene.update_font_data(&id, data);
            }
            Err(e) => log::warn!("Failed to fetch font '{}': {}", src, e),
        }
    }
}

struct AudioTrack {
    temp_path: PathBuf,
    start_time: f32,
    duration: f32,
    offset: f32,
    volume: f32,
    speed: f32,
}

fn collect_audio_tracks(scene: &Scene, debug: bool) -> Vec<AudioTrack> {
    let mut tracks = Vec::new();
    for audio in scene.main_composition.audios.iter().filter(|a| !a.deleted) {
        let Some(Asset::Audio(asset)) = scene.assets.get(&audio.asset_id) else {
            continue;
        };
        if asset.src.is_empty() {
            continue;
        }
        if debug {
            log::info!("Loading audio: {}", asset.src);
        }
        match fetch_bytes(&asset.src) {
            Ok(bytes) => {
                let temp_path = std::env::temp_dir().join(format!("kario_audio_{}", audio.id.0));
                if let Err(e) = std::fs::write(&temp_path, &bytes) {
                    log::warn!("Failed to write audio temp file: {}", e);
                    continue;
                }
                tracks.push(AudioTrack {
                    temp_path,
                    start_time: audio.start_time,
                    duration: audio.duration,
                    offset: audio.offset,
                    volume: audio.volume,
                    speed: audio.speed,
                });
            }
            Err(e) => log::warn!("Failed to fetch audio '{}': {}", asset.src, e),
        }
    }
    tracks
}

pub fn export_scene(
    scene: &mut Scene,
    output: PathBuf,
    fps: u32,
    debug: bool,
) -> Result<(), String> {
    let total_start = Instant::now();

    let width = scene.main_composition.width as u32;
    let height = scene.main_composition.height as u32;
    if width == 0 || height == 0 {
        return Err("Scene has zero-size composition".to_string());
    }

    preload_assets(scene, debug);
    let audio_tracks = collect_audio_tracks(scene, debug);
    log::info!("🎵 Audio tracks: {}", audio_tracks.len());

    let duration = match scene.main_composition.duration {
        Duration::Seconds(s) => s,
        Duration::Auto => scene.main_composition.computed_duration(),
    };

    let frame_count = (duration * fps as f32).ceil() as usize + 1;
    let times: Vec<f32> = (0..frame_count)
        .map(|i| i as f32 / fps as f32)
        .filter(|&t| t <= duration)
        .collect();

    log::info!(
        "🎬 Exporting — {} frames ({}x{}, {}fps) → {:?}",
        times.len(),
        width,
        height,
        fps,
        output
    );

    let mut renderer = pollster::block_on(kario_renderer::Renderer::new_offscreen(width, height))?;
    let mut ctx = Context::default();

    let mut cmd = Command::new("ffmpeg");
    cmd.args([
        "-y",
        "-f",
        "rawvideo",
        "-pixel_format",
        "rgba",
        "-video_size",
        &format!("{}x{}", width, height),
        "-framerate",
        &fps.to_string(),
        "-i",
        "-",
    ]);

    for track in &audio_tracks {
        cmd.arg("-i").arg(&track.temp_path);
    }

    if !audio_tracks.is_empty() {
        let mut filter_parts: Vec<String> = audio_tracks
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let delay_ms = (t.start_time * 1000.0) as u64;
                // atrim cuts the source to the requested duration;
                // adelay then shifts the trimmed clip to its start_time in the timeline.
                format!(
                    "[{}:a]atrim=start={:.6}:duration={:.6},adelay={}|{},volume={},atempo={}[a{}]",
                    i + 1,
                    t.offset,
                    t.duration * t.speed, // compensate so output duration == t.duration
                    delay_ms,
                    delay_ms,
                    t.volume,
                    t.speed,
                    i
                )
            })
            .collect();

        let mix_inputs: String = (0..audio_tracks.len())
            .map(|i| format!("[a{}]", i))
            .collect();
        filter_parts.push(format!(
            "{}amix=inputs={}:dropout_transition=0[aout]",
            mix_inputs,
            audio_tracks.len()
        ));

        let filter = filter_parts.join(";");
        if debug {
            log::info!("ffmpeg filter_complex: {}", filter);
        }
        cmd.args(["-filter_complex", &filter]);
        cmd.args(["-map", "0:v", "-map", "[aout]"]);
        cmd.args([
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv444p",
            "-c:a",
            "libmp3lame",
            "-b:a",
            "192k",
        ]);
        log::info!(
            "🔀 Muxing {} audio track(s) into output",
            audio_tracks.len()
        );
    } else {
        cmd.args(["-c:v", "libx264", "-pix_fmt", "yuv444p"]);
        log::info!("📹 No audio tracks — exporting video only");
    }

    cmd.arg(output.to_str().unwrap());

    let mut child = cmd
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn ffmpeg: {}", e))?;

    let stdin = child.stdin.as_mut().ok_or("Failed to open ffmpeg stdin")?;
    let render_start = Instant::now();

    for (idx, &time) in times.iter().enumerate() {
        if debug {
            log::info!("Rendering frame {} at t={:.3}s", idx, time);
        }

        scene.render(&mut ctx, &mut renderer, time);
        let pixels = renderer.render_to_rgba()?;

        let expected = (width * height * 4) as usize;
        if pixels.len() != expected {
            return Err(format!(
                "Frame {} wrong size: got {}, expected {}",
                idx,
                pixels.len(),
                expected
            ));
        }

        stdin
            .write_all(&pixels)
            .map_err(|e| format!("Failed writing frame {}: {}", idx, e))?;
    }

    let status = child
        .wait()
        .map_err(|e| format!("Failed waiting for ffmpeg: {}", e))?;

    if !status.success() {
        return Err(format!("ffmpeg exited with error: {}", status));
    }

    log::info!("✅ Export complete: {:?}", output);

    if debug {
        let render_elapsed = render_start.elapsed();
        log::info!(
            "⏱️  Render+encode: {:.0}ms ({:.1}ms/frame)",
            render_elapsed.as_secs_f64() * 1000.0,
            render_elapsed.as_secs_f64() * 1000.0 / times.len() as f64
        );
        log::info!(
            "⏱️  Total: {:.0}ms",
            total_start.elapsed().as_secs_f64() * 1000.0
        );
    }

    Ok(())
}
