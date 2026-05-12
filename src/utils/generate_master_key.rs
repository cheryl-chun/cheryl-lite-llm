// 生成 Master Key
fn generate_master_key(expires_in_days: u32, description: Option<String>) {
    use chrono::{Utc, Duration};
    
    let generator = MasterKeyGenerator::new();
    let key = generator.generate();
    
    let expires_at = if expires_in_days > 0 {
        Some(Utc::now() + Duration::days(expires_in_days as i64))
    } else {
        None
    };
    
    println!();
    println!("╔════════════════════════════════════════════════════╗");
    println!("║          Master Key Generated Successfully         ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!();
    println!("📋 Master Key Details:");
    println!("  ├─ Raw Key: {}", key.raw_key);
    println!("  ├─ Key Hash: {}", key.key_hash);
    if let Some(desc) = &description {
        println!("  ├─ Description: {}", desc);
    }
    if let Some(exp) = expires_at {
        println!("  └─ Expires At: {}", exp.format("%Y-%m-%d %H:%M:%S UTC"));
    } else {
        println!("  └─ Expires At: Never");
    }
    println!();
    println!("⚠️  SECURITY WARNING:");
    println!("  • The raw key will NOT be shown again");
    println!("  • Store it securely (e.g., 1Password, Vault, AWS Secrets Manager)");
    println!("  • Do NOT commit it to version control");
    println!("  • Do NOT share it via insecure channels");
    println!();
    println!("💾 SQL to insert into database:");
    println!();
    println!("INSERT INTO virtual_keys (");
    println!("  id, key_hash, key_type, enabled, expires_at, models, description");
    println!(") VALUES (");
    println!("  UUID(),");
    println!("  '{}',", key.key_hash);
    println!("  'master',");
    println!("  TRUE,");
    if let Some(exp) = expires_at {
        println!("  '{}',", exp.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("  NULL,");
    }
    println!("  JSON_ARRAY('*'),");
    if let Some(desc) = &description {
        println!("  '{}'", desc.replace("'", "''"));  // 转义单引号
    } else {
        println!("  NULL");
    }
    println!(");");
    println!();
    println!("════════════════════════════════════════════════════");
}