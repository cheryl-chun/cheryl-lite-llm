use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::config::DatabaseConfig;
use crate::database::builder::DatabaseBuilder;
use crate::database::models::{MasterKey, MasterKeyRow};
use crate::database::traits::{MasterKeyRepository, VirtualKeyRepository};
use crate::database::{VirtualAuthContext, DatabaseContext, DatabasePool, VirtualKeyRow};
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
impl VirtualKeyRepository for PgRepository {
    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<VirtualAuthContext>> {
        let row = sqlx::query_as::<_, VirtualKeyRow>(
            r#"
            SELECT id, key_hash, enabled, expires_at, models, user_id, team_id
            FROM virtual_keys
            WHERE key_hash = $1
            "#,
        )
        .bind(key_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ProxyError::Database(format!("Failed to query virtual key: {}", e)))?;

        match row {
            Some(row) => {
                let models = serde_json::from_value(row.models).unwrap_or_default();
                let key_id = Uuid::parse_str(&row.id)
                    .map_err(|e| ProxyError::Database(format!("Invalid UUID: {}", e)))?;

                Ok(Some(VirtualAuthContext {
                    key_id,
                    key_hash: row.key_hash,
                    enabled: row.enabled,
                    expires_at: row.expires_at,
                    models,
                    user_id: row.user_id,
                    team_id: row.team_id,
                }))
            }
            None => Ok(None),
        }
    }

    async fn list_all(&self) -> Result<Vec<crate::database::models::VirtualKey>> {
        todo!("Implement create for VirtualKeyRepository")
    }

    async fn create(&self, _key: &crate::database::models::VirtualKey) -> Result<()> {
        todo!("Implement create for VirtualKeyRepository")
    }

    async fn disable(&self, _key_id: &Uuid) -> Result<()> {
        todo!("Implement disable for VirtualKeyRepository")
    }

    async fn enable(&self, _key_id: &Uuid) -> Result<()> {
        todo!("Implement enable for VirtualKeyRepository")
    }

    async fn delete(&self, _key_id: &Uuid) -> Result<()> {
        todo!("Implement delete for VirtualKeyRepository")
    }
}

#[async_trait]
impl MasterKeyRepository for PgRepository {
    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<MasterKey>> {
        let row = sqlx::query_as::<_, MasterKeyRow>(
            r#"
            SELECT id, key_hash, enabled, expires_at, description, created_at, last_used_at
            FROM master_keys
            WHERE key_hash = $1
            "#,
        )
        .bind(key_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ProxyError::Database(format!("Failed to query master key: {}", e)))?;

        match row {
            Some(row) => {
                let id = Uuid::parse_str(&row.id)
                    .map_err(|e| ProxyError::Database(format!("Invalid UUID: {}", e)))?;

                Ok(Some(MasterKey {
                    id,
                    key_hash: row.key_hash,
                    enabled: row.enabled,
                    expires_at: row.expires_at,
                    description: row.description,
                    created_at: row.created_at,
                    last_used_at: row.last_used_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn list_all(&self) -> Result<Vec<MasterKey>> {
        let rows = sqlx::query_as::<_, MasterKeyRow>(
            r#"
            SELECT id, key_hash, enabled, expires_at, description, created_at, last_used_at
            FROM master_keys
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ProxyError::Database(format!("Failed to list master keys: {}", e)))?;

        let mut keys = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id)
                .map_err(|e| ProxyError::Database(format!("Invalid UUID: {}", e)))?;

            keys.push(MasterKey {
                id,
                key_hash: row.key_hash,
                enabled: row.enabled,
                expires_at: row.expires_at,
                description: row.description,
                created_at: row.created_at,
                last_used_at: row.last_used_at,
            });
        }

        Ok(keys)
    }

    async fn create(&self, key: &MasterKey) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO master_keys (id, key_hash, enabled, expires_at, description, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(key.id.to_string())
        .bind(&key.key_hash)
        .bind(key.enabled)
        .bind(key.expires_at)
        .bind(&key.description)
        .bind(key.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| ProxyError::Database(format!("Failed to create master key: {}", e)))?;

        Ok(())
    }

    async fn disable(&self, key_id: &Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE master_keys
            SET enabled = false
            WHERE id = $1
            "#,
        )
        .bind(key_id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| ProxyError::Database(format!("Failed to disable master key: {}", e)))?;

        Ok(())
    }

    async fn enable(&self, key_id: &Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE master_keys
            SET enabled = true
            WHERE id = $1
            "#,
        )
        .bind(key_id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| ProxyError::Database(format!("Failed to enable master key: {}", e)))?;

        Ok(())
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

        let (virtual_key_repo, master_key_repo) = match &pool {
            DatabasePool::Postgres(pg_pool) => {
                let repo = Arc::new(PgRepository::new(pg_pool.clone()));
                let virtual_repo: Arc<dyn VirtualKeyRepository> = repo.clone();
                let master_repo: Arc<dyn MasterKeyRepository> = repo;
                (virtual_repo, master_repo)
            }
            _ => unreachable!("Expected PostgreSQL pool"),
        };

        Ok(DatabaseContext::new(pool, virtual_key_repo, master_key_repo))
    }
}

pub fn register() {
    use crate::database::DatabaseFactory;
    DatabaseFactory::register("postgres", Arc::new(PgDatabaseBuilder));
}