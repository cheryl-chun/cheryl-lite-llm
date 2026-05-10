use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub key_id: Uuid,
    pub key_hash: String,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub models: Vec<String>,
    pub user_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VirtualKeyRow {
    pub id: Uuid,
    pub key_hash: String,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub models: serde_json::Value,
    pub user_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
}