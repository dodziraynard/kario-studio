use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{worker, JobState, JobStore};

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
}

pub async fn list_projects(
    State(store): State<Arc<JobStore>>,
) -> (StatusCode, Json<serde_json::Value>) {
    match store.list_projects().await {
        Ok(projects) => (StatusCode::OK, Json(serde_json::Value::Array(projects))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("{e:#}") }))),
    }
}

pub async fn generate(
    State(store): State<Arc<JobStore>>,
    Json(req): Json<GenerateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let job_id = Uuid::new_v4();
    let project_id = Uuid::new_v4();
    let job = JobState::new(job_id, project_id, &req.prompt);
    if let Err(e) = store.insert(job).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("{e:#}") })));
    }

    let store_clone = Arc::clone(&store);
    let prompt = req.prompt.clone();
    tokio::spawn(async move {
        worker::spawn_worker(store_clone, project_id, job_id, prompt).await;
    });

    (StatusCode::ACCEPTED, Json(serde_json::json!({ "job_id": job_id, "project_id": project_id })))
}

pub async fn edit(
    State(store): State<Arc<JobStore>>,
    Path(project_id): Path<Uuid>,
    Json(req): Json<GenerateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let job_id = Uuid::new_v4();
    let job = JobState::new(job_id, project_id, &req.prompt);
    if let Err(e) = store.insert(job).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("{e:#}") })));
    }

    let store_clone = Arc::clone(&store);
    let prompt = req.prompt.clone();
    tokio::spawn(async move {
        worker::spawn_worker_edit(store_clone, project_id, job_id, prompt).await;
    });

    (StatusCode::ACCEPTED, Json(serde_json::json!({ "job_id": job_id, "project_id": project_id })))
}

pub async fn job_status(
    State(store): State<Arc<JobStore>>,
    Path(job_id): Path<Uuid>,
) -> (StatusCode, Json<serde_json::Value>) {
    match store.get(&job_id).await {
        Ok(Some(job)) => (StatusCode::OK, Json(serde_json::to_value(job).unwrap_or_default())),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "job not found" }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("{e:#}") }))),
    }
}

pub async fn project_jobs(
    State(store): State<Arc<JobStore>>,
    Path(project_id): Path<Uuid>,
) -> (StatusCode, Json<serde_json::Value>) {
    match store.jobs_for_project(&project_id).await {
        Ok(jobs) => (StatusCode::OK, Json(serde_json::to_value(jobs).unwrap_or_default())),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("{e:#}") }))),
    }
}

pub async fn delete_project(
    State(store): State<Arc<JobStore>>,
    Path(project_id): Path<Uuid>,
) -> (StatusCode, Json<serde_json::Value>) {
    match store.delete_project(&project_id).await {
        Ok(count) => {
            // Best-effort: remove workspace and render dirs from disk
            let ws = crate::worker::project_dir(&project_id);
            let renders = crate::worker::render_project_dir(&project_id);
            let _ = std::fs::remove_dir_all(&ws);
            let _ = std::fs::remove_dir_all(&renders);
            (StatusCode::OK, Json(serde_json::json!({ "deleted_jobs": count })))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": format!("{e:#}") }))),
    }
}

