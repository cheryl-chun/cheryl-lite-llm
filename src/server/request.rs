use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateVirtualKeyRequest {
    pub user_id: Option<String>,
    pub team_id: Option<String>,
    pub models: Vec<String>,
    #[serde(default)]
    pub expires_in_days: u32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateVirtualKeyResponse {
    pub key: String,
    pub key_id: Uuid,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListVirtualKeyResponse {
    pub keys: Vec<VirtualKeyInfo>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct GetVirtualKeyResponse {
    pub key: VirtualKeyInfo
}

#[derive(Debug, Serialize)]
pub struct GetKeysByUserResponse {
    pub user_id: String,
    pub keys: Vec<VirtualKeyInfo>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct VirtualKeyInfo {
    pub id: Uuid,
    pub user_id: Option<String>,
    pub team_id: Option<String>,
    pub models: Vec<String>,
    pub enabled: bool,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub last_used_at: Option<String>,
    pub description: Option<String>,
    pub created_by: Uuid,
}
