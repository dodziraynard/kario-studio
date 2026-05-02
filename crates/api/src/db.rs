use anyhow::{Context, Result};
use mongodb::{
    bson::{doc, to_document, Bson, Document},
    options::ClientOptions,
    Client, Collection,
};

use crate::jobs::{JobId, JobState};

const DB_NAME: &str = "kario_studio";
const TEMPLATES_COLL: &str = "templates";
const JOBS_COLL: &str = "jobs";

#[derive(Clone)]
pub struct Db {
    jobs: Collection<JobState>,
    templates: Collection<Document>,
}

impl Db {
    pub async fn connect(uri: &str) -> Result<Self> {
        let opts = ClientOptions::parse(uri)
            .await
            .context("parse MONGODB_URI")?;
        let client = Client::with_options(opts).context("create mongo client")?;
        let jobs = client.database(DB_NAME).collection::<JobState>(JOBS_COLL);
        let templates = client.database(DB_NAME).collection::<Document>(TEMPLATES_COLL);
        Ok(Self { jobs, templates })
    }

    /// Upsert a job document (keyed on `id`).
    pub async fn upsert_job(&self, job: &JobState) -> Result<()> {
        let id_str = job.id.to_string();
        let doc = to_document(job).context("serialize job")?;
        self.jobs
            .update_one(doc! { "id": &id_str }, doc! { "$set": doc })
            .upsert(true)
            .await
            .context("upsert job")?;
        Ok(())
    }

    pub async fn get_job(&self, id: &JobId) -> Result<Option<JobState>> {
        self.jobs
            .find_one(doc! { "id": id.to_string() })
            .await
            .context("find_one job")
    }

    pub async fn jobs_for_project(&self, project_id: &JobId) -> Result<Vec<JobState>> {
        use mongodb::options::FindOptions;
        let opts = FindOptions::builder()
            .sort(doc! { "created_at": 1 })
            .build();
        let mut cursor = self
            .jobs
            .find(doc! { "project_id": project_id.to_string() })
            .with_options(opts)
            .await
            .context("find jobs for project")?;
        let mut jobs = Vec::new();
        while cursor.advance().await.context("cursor advance")? {
            match cursor.deserialize_current() {
                Ok(job) => jobs.push(job),
                Err(e) => log::warn!("Failed to deserialize job: {e}"),
            }
        }
        Ok(jobs)
    }

    pub async fn list_projects(&self) -> Result<Vec<serde_json::Value>> {
        use mongodb::bson::Document;
        // Aggregate: group by project_id, get counts + latest job metadata
        let pipeline = vec![
            doc! { "$sort": { "created_at": 1 } },
            doc! {
                "$group": {
                    "_id": "$project_id",
                    "job_count": { "$sum": 1 },
                    "created_at": { "$first": "$created_at" },
                    "updated_at": { "$last": "$updated_at" },
                    "latest_job_id": { "$last": "$id" },
                    "latest_status": { "$last": "$status" },
                    "latest_render_url": { "$last": "$render_url" },
                }
            },
            doc! { "$sort": { "updated_at": -1 } },
        ];
        let mut cursor = self
            .jobs
            .aggregate(pipeline)
            .await
            .context("aggregate projects")?;
        let mut projects = Vec::new();
        while cursor.advance().await.context("cursor advance")? {
            let raw: Document = cursor
                .deserialize_current()
                .context("deserialize project agg")?;
            projects.push(serde_json::json!({
                "project_id":        raw.get_str("_id").unwrap_or(""),
                "job_count":         raw.get_i32("job_count").unwrap_or(0),
                "created_at":        raw.get("created_at").and_then(|v| v.as_str()).unwrap_or(""),
                "updated_at":        raw.get("updated_at").and_then(|v| v.as_str()).unwrap_or(""),
                "latest_job_id":     raw.get_str("latest_job_id").unwrap_or(""),
                "latest_status":     raw.get_str("latest_status").unwrap_or(""),
                "latest_render_url": raw.get_str("latest_render_url").ok(),
            }));
        }
        Ok(projects)
    }

    pub async fn delete_project(&self, project_id: &JobId) -> Result<u64> {
        let result = self.jobs
            .delete_many(doc! { "project_id": project_id.to_string() })
            .await
            .context("delete project jobs")?;
        Ok(result.deleted_count)
    }

    /// Vector search the templates collection for the best match.
    /// The `embedding` must be 1536-dimensional (text-embedding-3-small).
    /// Returns the template `name` field, or `None` if the collection is empty.
    pub async fn find_best_template(&self, embedding: &[f64]) -> Result<Option<String>> {
        let query_vec: Vec<Bson> = embedding.iter().map(|&f| Bson::Double(f)).collect();
        let pipeline = vec![
            doc! {
                "$vectorSearch": {
                    "index": "template_embedding_index",
                    "path": "embedding",
                    "queryVector": query_vec,
                    "numCandidates": 20,
                    "limit": 1
                }
            },
            doc! { "$project": { "name": 1, "_id": 0 } },
        ];
        let mut cursor = self.templates
            .aggregate(pipeline)
            .await
            .context("vector search templates")?;
        if cursor.advance().await.context("cursor advance")? {
            let doc: Document = cursor.deserialize_current().context("deserialize template")?;
            return Ok(doc.get_str("name").ok().map(str::to_string));
        }
        Ok(None)
    }
}
