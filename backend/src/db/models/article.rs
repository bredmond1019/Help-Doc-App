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
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedArticle {
    pub article_id: Uuid,
    pub summary: String,
    pub summary_embedding: Vec<f32>,
    pub bullet_points: Vec<String>,
    pub keywords: Vec<String>,
    pub keyword_embedding: Vec<f32>,
    pub semantic_chunks: Vec<String>,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulletPointEmbedding {
    pub article_id: Uuid,
    pub key_point_index: usize,
    pub embedding: Vec<f32>,
}
