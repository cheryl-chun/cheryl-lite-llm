async fn verify_master_key(key: &str, database_url: &str) -> anyhow::Result<()> {
    use sha2::{Sha256, Digest};
    use chrono::Utc;
    
    println!("Verifying master key...");
    
    // 计算 hash
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let key_hash = format!("{:x}", hasher.finalize());
    
    // 连接数据库
    let pool = sqlx::MySqlPool::connect(database_url).await?;
    
    // 查询 key
    let result = sqlx::query!(
        r#"
        SELECT id, enabled, expires_at, key_type as "key_type: String", description
        FROM virtual_keys
        WHERE key_hash = ? AND key_type = 'master'
        "#,
        key_hash
    )
    .fetch_optional(&pool)
    .await?;
    
    match result {
        Some(row) => {
            println!();
            println!("✅ Master Key Found!");
            println!("  ├─ ID: {}", row.id);
            println!("  ├─ Enabled: {}", row.enabled);
            if let Some(desc) = row.description {
                println!("  ├─ Description: {}", desc);
            }
            
            if let Some(expires_at) = row.expires_at {
                let now = Utc::now();
                if now > expires_at {
                    println!("  └─ Status: ❌ EXPIRED at {}", expires_at);
                    std::process::exit(1);
                } else {
                    println!("  └─ Expires At: {}", expires_at);
                }
            } else {
                println!("  └─ Expires At: Never");
            }
            
            if !row.enabled {
                println!();
                println!("⚠️  WARNING: This key is disabled!");
                std::process::exit(1);
            }
            
            println!();
            println!("✓ Key is valid and active");
        }
        None => {
            println!();
            println!("❌ Master Key NOT Found!");
            println!("  The key hash does not exist in the database or is not a master key.");
            std::process::exit(1);
        }
    }
    
    Ok(())
}