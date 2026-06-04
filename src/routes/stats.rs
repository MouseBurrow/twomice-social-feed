use crate::errors::SocialFeedError;
use crate::service;
use axum::extract::State;
use axum::Json;
use config::app_data::AppData;
use custom_headers::user_id::UserId;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
pub struct UserStats {
    pub nib_count: i64,
    pub squeak_count: i64,
    pub upvote_count: i64,
    pub following_count: i64,
}

#[derive(Deserialize)]
struct PostStatsResponse {
    nib_count: i64,
    squeak_count: i64,
    upvote_count: i64,
}

pub async fn get_user_stats(
    State(app): State<AppData>,
    user_id: UserId,
) -> Result<Json<UserStats>, SocialFeedError> {
    let uid: i64 = user_id.into();

    let following_count = service::get_following_count(&app.pool, uid).await?;

    let post_service_url =
        env::var("POST_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8082".to_string());

    let client = reqwest::Client::new();
    let url = format!("{}/internal/stats/{}", post_service_url, uid);

    let post_stats = client
        .get(&url)
        .send()
        .await
        .map_err(|_| SocialFeedError::UpstreamError)?
        .json::<PostStatsResponse>()
        .await
        .map_err(|_| SocialFeedError::UpstreamError)?;

    Ok(Json(UserStats {
        nib_count: post_stats.nib_count,
        squeak_count: post_stats.squeak_count,
        upvote_count: post_stats.upvote_count,
        following_count,
    }))
}
