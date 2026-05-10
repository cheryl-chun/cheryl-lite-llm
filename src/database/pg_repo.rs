use async_trait::async_trait;

use crate::database::{AuthContext, VirtualKeyRow, AuthRepository};
use crate::error::{ProxyError, Result};

pub struct PgRepository {
    pool: sqlx::PgPool,
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
            None => Err(ProxyError::Database(format!(
                "Not found key_hash {}",
                key_hash
            ))),
        }
    }
}
