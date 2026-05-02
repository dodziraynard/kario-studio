
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::Db;

pub type JobId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Done,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Pending,
    Running,
    Done,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub status: StepStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<Utc>>,
}

impl Step {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: StepStatus::Pending,
            message: None,
            started_at: None,
            finished_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobState {
    #[serde(with = "uuid::serde::hyphenated")]
    pub id: JobId,
    #[serde(with = "uuid::serde::hyphenated")]
    pub project_id: JobId,
    #[serde(alias = "template")]
    pub prompt: String,
    pub status: JobStatus,
    pub steps: Vec<Step>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl JobState {
    pub fn new(id: JobId, project_id: JobId, prompt: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id,
            project_id,
            prompt: prompt.into(),
            status: JobStatus::Pending,
            steps: Vec::new(),
            created_at: now,
            updated_at: now,
            render_url: None,
            error: None,
        }
    }
}

/// Thin wrapper around `Db` — no in-memory cache, every read/write goes to MongoDB.
#[derive(Clone)]
pub struct JobStore {
    db: Db,
}

impl JobStore {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn insert(&self, job: JobState) -> anyhow::Result<()> {
        self.db.upsert_job(&job).await
    }

    pub async fn get(&self, id: &JobId) -> anyhow::Result<Option<JobState>> {
        self.db.get_job(id).await
    }

    pub async fn jobs_for_project(&self, project_id: &JobId) -> anyhow::Result<Vec<JobState>> {
        self.db.jobs_for_project(project_id).await
    }

    pub async fn list_projects(&self) -> anyhow::Result<Vec<serde_json::Value>> {
        self.db.list_projects().await
    }

    pub async fn delete_project(&self, project_id: &JobId) -> anyhow::Result<u64> {
        self.db.delete_project(project_id).await
    }

    /// Embed the prompt and find the best matching template name.
    /// Falls back to "product_launch" if the collection is empty or search fails.
    pub async fn pick_template(&self, prompt: &str) -> String {
        match crate::worker::embed_prompt(prompt).await {
            Ok(embedding) => match self.db.find_best_template(&embedding).await {
                Ok(Some(name)) => {
                    log::info!("🎨 Template selected by similarity: {name}");
                    name
                }
                Ok(None) => {
                    log::info!("🎨 No templates in DB, using product_launch");
                    "product_launch".to_string()
                }
                Err(e) => {
                    log::warn!("Template vector search failed: {e:#}, using product_launch");
                    "product_launch".to_string()
                }
            },
            Err(e) => {
                log::warn!("Embedding failed: {e:#}, using product_launch");
                "product_launch".to_string()
            }
        }
    }

    /// Fetch, mutate, and re-persist a job.
    pub async fn update(&self, id: &JobId, f: impl FnOnce(&mut JobState)) -> anyhow::Result<()> {
        let Some(mut job) = self.db.get_job(id).await? else { return Ok(()) };
        f(&mut job);
        job.updated_at = Utc::now();
        self.db.upsert_job(&job).await
    }

    pub async fn step_start(&self, id: &JobId, name: &str) {
        let _ = self.update(id, |j| {
            if let Some(s) = j.steps.iter_mut().find(|s| s.name == name) {
                s.status = StepStatus::Running;
                s.started_at = Some(Utc::now());
            } else {
                let mut s = Step::new(name);
                s.status = StepStatus::Running;
                s.started_at = Some(Utc::now());
                j.steps.push(s);
            }
        }).await;
    }

    pub async fn step_done(&self, id: &JobId, name: &str, message: Option<String>) {
        let _ = self.update(id, |j| {
            if let Some(s) = j.steps.iter_mut().find(|s| s.name == name) {
                s.status = StepStatus::Done;
                s.finished_at = Some(Utc::now());
                s.message = message.clone();
            }
        }).await;
    }

    pub async fn step_fail(&self, id: &JobId, name: &str, message: String) {
        let _ = self.update(id, |j| {
            if let Some(s) = j.steps.iter_mut().find(|s| s.name == name) {
                s.status = StepStatus::Failed;
                s.finished_at = Some(Utc::now());
                s.message = Some(message.clone());
            }
        }).await;
    }
}
