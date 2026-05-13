use std::sync::Arc;

use crate::{config::Config, database::{DatabaseFactory, DatabasePool, MasterKey, MasterKeyRepository, MySqlMasterKeyRepository, PgMasterKeyRepository}};

pub async fn enabled_master_key(
    id: uuid::Uuid,
    config_path: Option<String>,
    database_url: Option<String>,
) -> anyhow::Result<()> {
    let db_url = match (config_path, database_url) {
        (Some(config_path), _) => {
            let config = Config::from_file(&config_path)?;
            config.database.url
        }
        (None, Some(url)) => {
            url
        }
        (None, None) => {
            anyhow::bail!("Either --config or --database-url must be provided");
        }
    };

    let pool = DatabaseFactory::create_pool_from_url(&db_url).await?;
    
    let repo: Arc<dyn MasterKeyRepository> = match pool {
        DatabasePool::MySql(p) => Arc::new(MySqlMasterKeyRepository::new(p)) ,
        DatabasePool::Postgres(p) => Arc::new(PgMasterKeyRepository::new(p)),
    };

    repo.enable(&id).await?;
    println!("✅ Master Key {} enabled successfully", id);

    Ok(())
}
