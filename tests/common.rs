// 测试公共工具函数和辅助函数
use std::fs;
use tempfile::TempDir;

// 创建临时配置文件
pub fn create_temp_config(content: &str) -> (TempDir, String) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, content).unwrap();
    let path_str = config_path.to_str().unwrap().to_string();
    (temp_dir, path_str)
}

// 标准测试配置
pub fn standard_test_config() -> &'static str {
    r#"
[server]
host = "127.0.0.1"
port = 3000

[database]
url = "mysql://test:test@localhost/test_db"

[providers.ark]
api_key = "test_ark_key"
"#
}
