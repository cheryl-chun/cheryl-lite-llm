use cheryl_lite_llm::config::Config;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_from_file() {
    // 创建临时目录
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.toml");

    // 写入测试配置
    let config_content = r#"
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "mysql://test:test@localhost/test"
max_connections = 5

[providers.ark]
api_key = "test_key"
base_url = "https://test.com"
"#;
    fs::write(&config_path, config_content).unwrap();

    // 加载配置
    let config = Config::from_file(config_path.to_str().unwrap()).unwrap();

    // 验证
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.database.url, "mysql://test:test@localhost/test");
    assert_eq!(config.database.max_connections, 5);
    assert_eq!(config.providers.len(), 1);
    assert!(config.providers.contains_key("ark"));
}

#[test]
fn test_config_default_values() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.toml");

    // 不指定 max_connections（测试默认值）
    let config_content = r#"
[server]
host = "127.0.0.1"
port = 3000

[database]
url = "mysql://root@localhost/db"

[providers.openai]
api_key = "sk-test"
"#;
    fs::write(&config_path, config_content).unwrap();

    let config = Config::from_file(config_path.to_str().unwrap()).unwrap();

    // 验证默认值
    assert_eq!(config.database.max_connections, 10);
}

#[test]
fn test_config_missing_file() {
    let result = Config::from_file("nonexistent.toml");
    assert!(result.is_err());
}

#[test]
fn test_config_invalid_toml() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("invalid.toml");

    // 写入无效的 TOML
    fs::write(&config_path, "invalid toml content [[[").unwrap();

    let result = Config::from_file(config_path.to_str().unwrap());
    assert!(result.is_err());
}
