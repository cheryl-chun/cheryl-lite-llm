use cheryl_lite_llm::{
    clis::{disable_master_key, enabled_master_key, generate_master_key, list_master_key}, config::Config, server::{AppState, create_router}
};
use clap::{Parser, Subcommand};
use tracing_subscriber;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "cheryl_lite_llm")]
#[command(about = "A lightweight LLM gateway")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server {
        #[arg(short, long, default_value = "config.toml")]
        config: String,
    },
    GenerateMasterKey {
        /// Expiration in days (0 means never expires)
        #[arg(short, long, default_value = "0")]
        expires_in_days: u32,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long)]
        config: Option<String>,

        #[arg(short = 'u', long)]
        database_url: Option<String>,
    },
    DisableMaster {
        #[arg(short, long)]
        id: Uuid,

        #[arg(short, long)]
        config: Option<String>,

        #[arg(short = 'u', long)]
        database_url: Option<String>,
    },
    EnableMaster {
        #[arg(short, long)]
        id: Uuid,

        #[arg(short, long)]
        config: Option<String>,

        #[arg(short = 'u', long)]
        database_url: Option<String>,
    },
    ListMasters {
        #[arg(short, long)]
        config: Option<String>,
        #[arg(short, long)]
        database_url: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    cheryl_lite_llm::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Server { config } => run_server(&config).await?,
        Commands::GenerateMasterKey {
            expires_in_days,
            description,
            config,
            database_url,
        } => {
            generate_master_key(expires_in_days, description, config, database_url).await?;
        }
        Commands::DisableMaster {
            id, config, database_url
        } => {
            disable_master_key(id, config, database_url).await?;
        }
        Commands::EnableMaster { id, config, database_url } => {
            enabled_master_key(id, config, database_url).await?;
        }
        Commands::ListMasters {database_url, config } => {
            list_master_key(config, database_url).await?;
        }
    }

    Ok(())
}

async fn run_server(config_path: &str) -> anyhow::Result<()> {
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
