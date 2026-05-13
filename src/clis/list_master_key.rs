use std::sync::Arc;

use crate::{config::Config, database::{DatabaseFactory, DatabasePool, MasterKeyRepository, MySqlMasterKeyRepository, PgMasterKeyRepository}};

pub async fn list_master_key(
    config_path: Option<String>,
    database_url: Option<String>,
) -> anyhow::Result<()> {
    let url = match (config_path, database_url) {
        (Some(config_path), _) => {
            let config = Config::from_file(&config_path)?;
            config.database.url
        }
        (None, Some(url)) => url,
        (None, None) => {
            anyhow::bail!("Either --config or --database-url provided")
        }
    };

    let pool = DatabaseFactory::create_pool_from_url(&url).await?;
    let repo: Arc<dyn MasterKeyRepository> = match pool {
        DatabasePool::MySql(p) => Arc::new(MySqlMasterKeyRepository::new(p)),
        DatabasePool::Postgres(p) => Arc::new(PgMasterKeyRepository::new(p)),
    };

    let keys = repo.list_all().await?;

    if keys.is_empty() {
        println!("\n📭 No master keys found.");
        println!("Use 'generate-master-key' command to create one.\n");
        return Ok(());
    }

    println!("\n📋 Master Keys ({})", keys.len());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{:<12} {:<38} {:<20} {}", "Status", "ID", "Created", "Description");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    for key in keys {
        let status = if !key.enabled {
            "❌ DISABLED"
        } else if let Some(expires_at) = key.expires_at {
            if chrono::Utc::now() > expires_at {
                "⏰ EXPIRED "
            } else {
                "✅ ACTIVE  "
            }
        } else {
            "✅ ACTIVE  "
        };

        let created = key.created_at.format("%Y-%m-%d %H:%M").to_string();

        let description = key.description
            .as_deref()
            .unwrap_or("N/A");
        let description_display = if description.len() > 50 {
            format!("{}...", &description[..47])
        } else {
            description.to_string()
        };

        println!(
            "{:<12} {:<38} {:<20} {}",
            status,
            key.id.to_string(),
            created,
            description_display
        );
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    Ok(())
}