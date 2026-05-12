use std::sync::Arc;
use crate::database::{AuthRepository};

#[derive(Clone)]
pub struct DatabaseContext {
    pool: DatabasePool,

    pub auth_repo: Arc<dyn AuthRepository>,
}

#[derive(Clone)]
pub enum DatabasePool {
    MySql(sqlx::MySqlPool),
    Postgres(sqlx::PgPool),
}

impl DatabaseContext {
    pub fn new(
        pool: DatabasePool,
        auth_repo: Arc<dyn AuthRepository>,
    ) -> Self {
        Self {
            pool,
            auth_repo,
        }
    }

    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }
}