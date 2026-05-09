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
    init_providers();

    let config_path = "config.toml";
    let config = Config::from_file(config_path).map_err(|e| {
        tracing::error!("Failed to load config {}: {}", config_path, e);
        tracing::error!("Please create config.toml file. See config.example for reference.");
        anyhow::anyhow!(e)
    })?;

    let mut router = Router::new();

    tracing::info!("Config loaded successfully");

    // 使用工厂创建 providers（工厂从注册表查找）
    for (provider_type, provider_config) in config.providers.iter() {
        match ProviderFactory::create(provider_type, provider_config) {
            Ok(provider) => {
                router.register(provider);
                tracing::info!("Registered provider: {}", provider_type);
            }
            Err(e) => {
                tracing::warn!("Failed to create provider {}: {}", provider_type, e);
            }
        }
    }

    let app_state = AppState::new(router);

    let app = create_router(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
