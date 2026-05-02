mod db;
mod jobs;
mod routes;
mod worker;

use std::{net::SocketAddr, sync::Arc};

use axum::{routing::{delete, get, post}, Router};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub use jobs::{JobId, JobState, JobStatus, JobStore, Step, StepStatus};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv(); // load .env if present
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            use std::io::Write;
            let level_style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "{level_style}[{}]{level_style:#} {}",
                record.level(),
                record.args()
            )
        })
        .init();

    // Connect to MongoDB — required.
    let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let db = db::Db::connect(&uri).await?;
    log::info!("🍃 Connected to MongoDB");
    let store = Arc::new(JobStore::new(db));

    let renders_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists() && std::fs::read_to_string(p.join("Cargo.toml")).map(|s| s.contains("[workspace]")).unwrap_or(false))
        .map(|p| p.join("storage/renders"))
        .unwrap_or_else(|| std::path::PathBuf::from("storage/renders"));

    let app = Router::new()
        .route("/api/generate", post(routes::generate))
        .route("/api/projects", get(routes::list_projects))
        .route("/api/projects/{project_id}/edit", post(routes::edit))
        .route("/api/projects/{project_id}/jobs", get(routes::project_jobs))
        .route("/api/projects/{project_id}", delete(routes::delete_project))
        .route("/api/jobs/{job_id}", get(routes::job_status))
        .nest_service("/api/renders", ServeDir::new(&renders_dir))
        .layer(CorsLayer::permissive())
        .with_state(store);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    log::info!("🚀 kario-api listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
