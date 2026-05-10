use async_trait::async_trait;
use crate::{error::Result};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    // validate api key
    async fn validate_key(&self, key_hash: &str) -> Result<Option<super::models::AuthContext>>;
}