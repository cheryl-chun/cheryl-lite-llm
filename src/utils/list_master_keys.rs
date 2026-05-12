async fn list_master_keys(database_url: &str) -> anyhow::Result<()> {
    println!("Fetching master keys from database...");
    
    let pool = sqlx::MySqlPool::connect(database_url).await?;
    
    let keys = sqlx::query!(
        r#"
        SELECT 
            id, 
            key_hash, 
            enabled, 
            expires_at, 
            description,
            created_at
        FROM virtual_keys
        WHERE key_type = 'master'
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await?;
    
    if keys.is_empty() {
        println!();
        println!("No master keys found in database.");
        return Ok(());
    }
    
    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    Master Keys List                           ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    
    for (i, key) in keys.iter().enumerate() {
        println!("{}. Key ID: {}", i + 1, key.id);
        println!("   ├─ Hash Preview: {}...", &key.key_hash[..16]);
        println!("   ├─ Status: {}", if key.enabled { "✅ Enabled" } else { "❌ Disabled" });
        if let Some(desc) = &key.description {
            println!("   ├─ Description: {}", desc);
        }
        if let Some(exp) = key.expires_at {
            use chrono::Utc;
            let now = Utc::now();
            if now > exp {
                println!("   ├─ Expires: ❌ EXPIRED at {}", exp);
            } else {
                println!("   ├─ Expires: {}", exp);
            }
        } else {
            println!("   ├─ Expires: Never");
        }
        println!("   └─ Created: {}", key.created_at);
        println!();
    }
    
    println!("Total: {} master key(s)", keys.len());
    
    Ok(())
}