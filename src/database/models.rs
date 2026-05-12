use chrono::{DateTime, Utc};
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

#[derive(Debug, Clone)]
pub struct MasterKey {
    pub id: Uuid,
    pub key_hash: String,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub decription: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct MasterKeyRow {
    id: String,
    key_hash: String,
    enabled: bool,
    expires_at: Option<DateTime<Utc>>,
    description: Option<String>,
    created_at: DateTime<Utc>,
}