use std::sync::Arc;
use crate::database::traits::{VirtualKeyRepository, MasterKeyRepository};

#[derive(Clone)]
pub struct DatabaseContext {
    pool: DatabasePool,

    pub virtual_key_repo: Arc<dyn VirtualKeyRepository>,
    pub master_key_repo: Arc<dyn MasterKeyRepository>,
}

#[derive(Clone)]
pub enum DatabasePool {
    MySql(sqlx::MySqlPool),
    Postgres(sqlx::PgPool),
}

impl DatabaseContext {
    pub fn new(
        pool: DatabasePool,
        virtual_key_repo: Arc<dyn VirtualKeyRepository>,
        master_key_repo: Arc<dyn MasterKeyRepository>,
    ) -> Self {
        Self {
            pool,
            virtual_key_repo,
            master_key_repo,
        }
    }

    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }
}