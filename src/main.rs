use axum::routing::{get, post};
use axum::Router;
use config::health::health_response;
use config::server;
use serde_json::json;

async fn health() -> axum::Json<serde_json::Value> {
    health_response("social-feed")
}

async fn get_feed() -> axum::Json<serde_json::Value> {
    axum::Json(json!({ "status": "stub", "feed": [] }))
}

async fn set_preferences() -> axum::Json<serde_json::Value> {
    axum::Json(json!({ "status": "stub", "message": "feed preferences not implemented" }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::serve(
        "social-feed",
        Router::new()
            .route("/health", get(health))
            .route("/feed", get(get_feed))
            .route("/feed/preferences", post(set_preferences)),
    )
    .await
}
