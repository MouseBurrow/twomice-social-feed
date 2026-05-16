use axum::routing::{get, post};
use axum::{Json, Router};
use config::app_data::AppData;
use config::config::Config;
use config::logger;
use serde_json::json;

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": "social-feed" }))
}

async fn get_feed() -> Json<serde_json::Value> {
    Json(json!({ "status": "stub", "feed": [] }))
}

async fn set_preferences() -> Json<serde_json::Value> {
    Json(json!({ "status": "stub", "message": "feed preferences not implemented" }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let config = Config::load("social-feed");
    let app_data = AppData::new(config.clone()).await?;
    let addr = format!("0.0.0.0:{}", config.port);

    let app = Router::new()
        .route("/health", get(health))
        .route("/feed", get(get_feed))
        .route("/feed/preferences", post(set_preferences))
        .with_state(app_data);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
