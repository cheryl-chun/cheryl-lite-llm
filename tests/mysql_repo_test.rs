use cheryl_lite_llm::database::{MasterKey, MasterKeyRepository, MySqlMasterKeyRepository, MySqlVirtualKeyRepository, VirtualKey, VirtualKeyRepository};
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_create_and_find_virtual_key() {
    unsafe {
        std::env::set_var("RUST_LOG", "sqlx=debug");
    }
    tracing_subscriber::fmt::init();

    let pool = sqlx::MySqlPool::connect("mysql://root:200102@localhost/cheryl_lite_llm")
    .await
    .expect("Failed to connect");

    let repo = MySqlVirtualKeyRepository::new(pool);

    let key = VirtualKey {
        id: Uuid::new_v4(),
        key_hash: "test_hash_123".to_string(),
        enabled: true,
        expires_at: None,
        models: vec!["gpt-4".to_string()],
        user_id: Some("user_123".to_string()),
        team_id: None,
        created_by: Uuid::new_v4(),
        description: Some("test key".to_string()),
        created_at: Utc::now(),
        last_used_at: None,
    };

    repo.create(&key).await.unwrap();

    let found = repo.find_by_hash("test_hash_123").await.unwrap();

    assert!(found.is_some());
    assert_eq!(found.unwrap().key_id, key.id);

    repo.delete(&key.id).await.unwrap();
}

#[tokio::test]
async fn test_create_and_find_master_key() {
    let pool = sqlx::MySqlPool::connect("mysql://root:200102@localhost/cheryl_lite_llm")
    .await
    .expect("Failed to connect");

    let repo = MySqlMasterKeyRepository::new(pool);

    let key = MasterKey {
        id: Uuid::new_v4(),
        key_hash: "test_master_hash_456".to_string(),
        enabled: true,
        expires_at: None,
        description: Some("test master key".to_string()),
        created_at: Utc::now(),
        last_used_at: None,
    };

    repo.create(&key).await.unwrap();
    
    // 测试查找
    let found = repo.find_by_hash("test_master_hash_456").await.unwrap();
    assert!(found.is_some());
    
    let found_key = found.unwrap();
    assert_eq!(found_key.id, key.id);
    assert_eq!(found_key.enabled, true);
    assert_eq!(found_key.description, Some("test master key".to_string()));

    // 测试禁用
    repo.disable(&key.id).await.unwrap();
    let disabled = repo.find_by_hash("test_master_hash_456").await.unwrap().unwrap();
    assert_eq!(disabled.enabled, false);

    // 测试启用
    repo.enable(&key.id).await.unwrap();
    let enabled = repo.find_by_hash("test_master_hash_456").await.unwrap().unwrap();
    assert_eq!(enabled.enabled, true);

    // 清理，MasterKey 没有 delete 方法，可以手动执行 SQL
    let cleanup_pool = sqlx::MySqlPool::connect("mysql://root:200102@localhost/cheryl_lite_llm")
    .await
    .unwrap();
    sqlx::query("DELETE FROM master_keys WHERE id = ?")
        .bind(key.id.to_string())
        .execute(&cleanup_pool)  
        .await
        .unwrap();
}