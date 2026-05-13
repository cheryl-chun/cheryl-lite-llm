use async_trait::async_trait;
use crate::{config::DatabaseConfig, database::{DatabaseContext, DatabasePool}};
use anyhow::Result;

#[async_trait]
pub trait DatabaseBuilder: Send + Sync {
    async fn create_pool(&self, config: &DatabaseConfig) -> Result<DatabasePool>;
    async fn build(&self, config: &DatabaseConfig) -> Result<DatabaseContext>;
}