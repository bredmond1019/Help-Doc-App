use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub categories: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewArticle {
    pub id: String,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedArticle {
    pub article_id: String,
    pub summary: String,
    pub key_points: Vec<String>,
    pub keywords: Vec<String>,
    pub semantic_chunks: Vec<String>,
    pub embeddings: Vec<f32>,
    pub categories: Vec<String>,
}
