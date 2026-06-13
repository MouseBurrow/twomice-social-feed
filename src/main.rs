mod errors;
mod routes;
pub(crate) mod service;

use axum::routing::{get, put};
use axum::Router;
use config::server;

use routes::following::{
    follow_board, get_follower_count, get_following, internal_get_following, unfollow_board,
};
use routes::stats::get_user_stats;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::serve(
        "feed",
        Router::new()
            .route("/users/me/following", get(get_following))
            .route(
                "/users/me/following/:board_id",
                put(follow_board).delete(unfollow_board),
            )
            .route("/users/me/stats", get(get_user_stats))
            .route("/internal/following", get(internal_get_following))
            .route("/b/:topic/followers", get(get_follower_count)),
    )
    .await
}
