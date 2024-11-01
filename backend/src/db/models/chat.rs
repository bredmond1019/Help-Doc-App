use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub context: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub session_id: Uuid,
    pub content: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub referenced_article_ids: Vec<Uuid>,
}
