use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::{config::Config, database::{DatabaseFactory, DatabasePool, MasterKey, MasterKeyRepository, MySqlMasterKeyRepository, PgMasterKeyRepository}, utils::{compute_sha256, generate_random_key}};

// 生成 Master Key
pub async fn generate_master_key(
    expires_in_days: u32, 
    description: Option<String>,
    config_path: Option<String>,
    database_url: Option<String>,
) -> anyhow::Result<()> {
    let raw_key = generate_random_key("mk_");
    let key_hash = compute_sha256(&raw_key);

    let expires_at = if expires_in_days > 0 {
        Some(Utc::now() + Duration::days(expires_in_days as i64))
    } else {
        None
    };

    let master_key = MasterKey {
        id: Uuid::new_v4(),
        key_hash,
        enabled: true,
        expires_at,
        description,
        created_at: Utc::now(),
        last_used_at: None,
    };

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
    match pool {
        DatabasePool::MySql(p) => {
            let repo = MySqlMasterKeyRepository::new(p);
            repo.create(&master_key).await?;
        }
        DatabasePool::Postgres(p) => {
            let repo = PgMasterKeyRepository::new(p);
            repo.create(&master_key).await?;
        }
    }

    println!("\n✅ Master Key generated successfully!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("⚠️  IMPORTANT: Save this key now! It will not be shown again.");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Key: {}", raw_key);
    println!("ID:  {}", master_key.id);
    println!("Expires: {}", expires_at.map_or("Never".to_string(), |e| e.to_rfc3339()));
    
    Ok(())
}

