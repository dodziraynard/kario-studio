# kario-studio

Backend API for AI-driven motion video generation. Accepts a text prompt, generates a voice-over, analyses audio timing, and renders an MP4.

## How it works

**Generate pipeline** (`POST /api/generate`)

1. **Writing script** — AI writes a punchy voice-over script from your prompt
2. **Generating voiceover** — Script is converted to speech and saved as `assets/vo.mp3`
3. **Analysing audio timing** — Word-level timestamps are extracted from the audio
4. **Designing scenes** — AI writes `video_scene.rs` timed to the narration
5. **Rendering video** — Scene is compiled and rendered to `output.mp4`

**Edit pipeline** (`POST /api/projects/:project_id/edit`)

Reuses the audio from the most recent completed run for the project (steps 1–3 are skipped):

1. **Designing scenes** — AI rewrites the scene with your new prompt and the existing audio
2. **Rendering video** — Compiled and rendered to a new `output.mp4`

## API

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/generate` | Start a new video generation job |
| `POST` | `/api/projects/:project_id/edit` | Edit an existing project (reuse audio) |
| `GET` | `/api/projects` | List all projects, newest first |
| `GET` | `/api/projects/:project_id/jobs` | List all jobs for a project |
| `GET` | `/api/jobs/:job_id` | Get status and steps for a job |
| `GET` | `/api/renders/:project_id_simple/:run_num/output.mp4` | Download a rendered MP4 |
| `DELETE` | `/api/projects/:project_id` | Delete a project and all its jobs |

### POST /api/generate

```json
{ "prompt": "Create a launch video for acme.com" }
```

Response:

```json
{ "job_id": "...", "project_id": "..." }
```

### Job status

```json
{
  "id": "...",
  "project_id": "...",
  "prompt": "Create a launch video for acme.com",
  "status": "running",
  "steps": [
    { "name": "Writing script", "status": "done", "message": "42 words" },
    { "name": "Generating voiceover", "status": "done" },
    { "name": "Analysing audio timing", "status": "running" }
  ],
  "created_at": "2026-05-02T10:00:00Z",
  "updated_at": "2026-05-02T10:01:30Z",
  "render_url": null
}
```

`status` is one of: `pending` `running` `done` `failed`

Each step's `status` is one of: `pending` `running` `done` `failed`

## Setup

### Prerequisites

- Rust (stable)
- `claude` CLI on `PATH`
- `whisperx` (path configured in `worker.rs`)
- MongoDB Atlas (or local)
- ElevenLabs API key

### Environment

Create a `.env` file in the workspace root:

```env
MONGODB_URI=mongodb+srv://...
ELEVENLABS_API_KEY=sk_...
OPENAI_API_KEY=sk-...
```

### Run

```bash
cargo run --release --bin kario-api
```

API listens on `http://0.0.0.0:8080`.

## Storage layout

``` 
storage/
  renders/          # rendered MP4s, served at /api/renders/*
  workspaces/       # per-run isolated build dirs
  templates/        # base video_scene.rs templates
  cache/target/     # shared Cargo build cache (incremental)
```

## Data

All jobs are stored in MongoDB:

- **Database:** `kario_studio`
- **Collection:** `jobs`

Projects are not stored as separate documents — they are derived on the fly by grouping jobs that share a `project_id`.
