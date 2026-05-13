use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============= Master Key 相关 =============

/// Master Key 业务模型
#[derive(Debug, Clone)]
pub struct MasterKey {
    pub id: Uuid,
    pub key_hash: String,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Master Key 数据库行映射
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MasterKeyRow {
    pub id: String,  // UUID 在数据库中是 CHAR(36)
    pub key_hash: String,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Master Key 认证上下文
#[derive(Debug, Clone)]
pub struct MasterAuthContext {
    pub key_id: Uuid,
    pub key_hash: String,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

// ============= Virtual Key 相关 =============

/// Virtual Key 业务模型
#[derive(Debug, Clone)]
pub struct VirtualKey {
    pub id: Uuid,
    pub key_hash: String,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub models: Vec<String>,
    pub user_id: Option<String>,
    pub team_id: Option<String>,
    pub created_by: Uuid,  // 创建者的 Master Key ID
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Virtual Key 数据库行映射
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VirtualKeyRow {
    pub id: String,  // UUID 在数据库中是 CHAR(36)
    pub key_hash: String,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub models: serde_json::Value,  // JSON array in database
    pub user_id: Option<String>,
    pub team_id: Option<String>,
    pub created_by: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Virtual Key 认证上下文
#[derive(Debug, Clone)]
pub struct VirtualAuthContext {
    pub key_id: Uuid,
    pub key_hash: String,
    pub enabled: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub models: Vec<String>,
    pub user_id: Option<String>,
    pub team_id: Option<String>,
}

