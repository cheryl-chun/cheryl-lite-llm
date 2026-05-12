use cheryl_lite_llm::{
    adapters::{ProviderFactory, init_providers},
    config::Config,
    router::Router,
    server::{AppState, create_router},
};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // 初始化所有 providers（注册到工厂）
    cheryl_lite_llm::init();

    let config_path = "config.toml";
    let config = Config::from_file(config_path).map_err(|e| {
        tracing::error!("Failed to load config {}: {}", config_path, e);
        tracing::error!("Please create config.toml file. See config.example for reference.");
        anyhow::anyhow!(e)
    })?;

    tracing::info!("Config loaded successfully");

    // 从配置构造应用状态（包含所有依赖）
    let app_state = AppState::from_config(&config).await?;

    // 创建 HTTP 路由器
    let app = create_router(app_state);

    // 绑定地址并启动服务
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
