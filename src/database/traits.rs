use async_trait::async_trait;
use uuid::Uuid;
use crate::{database::models::MasterKey, error::Result};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    // validate api key
    async fn validate_key(&self, key_hash: &str) -> Result<Option<super::models::AuthContext>>;
}

#[async_trait]
pub trait MasterkeyRepository: Send + Sync {
    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<MasterKey>>;
    async fn list_all(&self) -> Result<Vec<MasterKey>>;
    async fn create(&self, key: &MasterKey) -> Result<()>;
    async fn disable(&self, key_id: &Uuid) -> Result<()>;
    async fn enable(&self, key_id: &Uuid) -> Result<()>;
}