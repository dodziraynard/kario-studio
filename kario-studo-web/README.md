# kario-studio-web

Frontend for [kario-studio](../), a node-graph AI video editor.

## Prerequisites

- Node.js 18+
- The `kario-studio` Rust backend running on `http://localhost:8080`

## Setup

```bash
npm install
```

## Development

```bash
npm run dev
```

Opens at `http://localhost:5173`. API requests to `/api/*` are proxied to the backend at `http://localhost:8080`.

## Production build

```bash
npm run build
npm run preview
```

## Running the backend

From the `kario-studio` directory:

```bash
cargo run
```

The backend must be running before the frontend can load or create projects.

## Routes

| Path | Description |
|---|---|
| `/` | Project dashboard |
| `/:projectId` | Editor canvas for an existing project |
| `/new` | New project canvas (blank, creates project on first prompt) |
