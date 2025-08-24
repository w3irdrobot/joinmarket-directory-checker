use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use tower::ServiceBuilder;

use crate::monitor::{SharedStatusStore, start_background_monitoring};
use crate::templates::dashboard_page;
use crate::types::Config;

pub type AppState = SharedStatusStore;

pub async fn create_app(config: Config) -> Router {
    // Start background monitoring and get shared status store
    let status_store = start_background_monitoring(config).await;

    Router::new()
        .route("/", get(dashboard_handler))
        .route("/health", get(health_handler))
        .with_state(status_store)
        .layer(ServiceBuilder::new())
}

async fn dashboard_handler(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let status_store = state.read().await;
    let html = dashboard_page(&status_store);
    Ok(Html(html.into_string()))
}

async fn health_handler() -> &'static str {
    "OK"
}

pub async fn run_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app(config.clone()).await;

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("🚀 Server running on http://{}", addr);
    println!("📊 Dashboard available at http://{}/", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
