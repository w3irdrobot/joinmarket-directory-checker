use std::path::Path;

use joinmarket_directory_checker::config::{create_default_config, load_config};
use joinmarket_directory_checker::server::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = if Path::new("config.toml").exists() {
        load_config("config.toml")?
    } else {
        println!("⚠️  config.toml not found, using default configuration");
        create_default_config()
    };

    println!("💡 Create config.toml to customize endpoints and settings");
    println!("🔧 Configuration loaded:");
    println!("   Server: {}:{}", config.server.host, config.server.port);
    println!("   Proxy: {}:{}", config.proxy.host, config.proxy.port);
    println!("   Endpoints: {}", config.endpoints.len());
    println!(
        "   Check interval: {}s",
        config.monitoring.check_interval_seconds
    );
    println!(
        "   Connection timeout: {}s",
        config.monitoring.connection_timeout_seconds
    );

    // Start the web server (monitoring starts automatically)
    println!(
        "🚀 Server running on http://{}:{}",
        config.server.host, config.server.port
    );
    println!(
        "📊 Dashboard available at http://{}:{}/",
        config.server.host, config.server.port
    );

    run_server(config).await?;

    Ok(())
}
