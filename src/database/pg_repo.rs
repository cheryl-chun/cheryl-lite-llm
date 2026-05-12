use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::config::DatabaseConfig;
use crate::database::builder::DatabaseBuilder;
use crate::database::models::MasterKey;
use crate::database::traits::MasterkeyRepository;
use crate::database::{AuthContext, AuthRepository, DatabaseContext, DatabasePool, VirtualKeyRow};
use crate::error::{ProxyError, Result};

pub struct PgRepository {
    pool: sqlx::PgPool,
}

pub struct PgDatabaseBuilder;

impl PgRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool
        }
    }
}

#[async_trait]
impl AuthRepository for PgRepository {
    async fn validate_key(&self, key_hash: &str) -> Result<Option<AuthContext>> {
        let row_result = sqlx::query_as::<_, VirtualKeyRow>(
            r#"
            SELECT id, key_hash, enabled, expires_at, models, user_id, team_id
            FROM virtual_keys
            WHERE key_hash = $1
            "#,
        )
        .bind(key_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ProxyError::Database(format!("database error: {}", e.to_string())))?;

        match row_result {
            Some(row) => {
                let models = serde_json::from_value(row.models).unwrap_or_default();
                return Ok(Some(AuthContext {
                    key_id: row.id,
                    key_hash: row.key_hash,
                    enabled: row.enabled,
                    expires_at: row.expires_at,
                    models,
                    user_id: row.user_id,
                    team_id: row.team_id,
                }));
            }
            None => Ok(None),
        }
    }
}

#[async_trait]
impl MasterkeyRepository for PgRepository {
    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<MasterKey>> {
        todo!()
    }

    async fn list_all(&self) -> Result<Vec<MasterKey>> {
        todo!()
    }

    async fn create(&self, key: &MasterKey) -> Result<()> {
        todo!()
    }

    async fn disable(&self, key_id: &Uuid) -> Result<()> {
        todo!()
    }

    async fn enable(&self, key_id: &Uuid) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl DatabaseBuilder for PgDatabaseBuilder {
    async fn create_pool(&self, config: &DatabaseConfig) -> anyhow::Result<DatabasePool> {
        let pool = sqlx::PgPool::connect(&config.url).await?;
        Ok(DatabasePool::Postgres(pool))
    }

    async fn build(&self, config: &DatabaseConfig) -> anyhow::Result<DatabaseContext> {
        let pool = self.create_pool(config).await?;

        let auth_repo: Arc<dyn AuthRepository> = match &pool {
            DatabasePool::Postgres(pg_pool) => {
                Arc::new(PgRepository::new(pg_pool.clone()))
            }
            _ => unreachable!("Expected PostgreSQL pool"),
        };

        Ok(DatabaseContext::new(pool, auth_repo))
    }
}

pub fn register() {
    use crate::database::DatabaseFactory;
    DatabaseFactory::register("postgres", Arc::new(PgDatabaseBuilder));
}