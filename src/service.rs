use crate::errors::SocialFeedError;
use easy_errors::map_sqlx_error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::collections::HashSet;

#[derive(Serialize)]
pub struct FollowedBoardInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub post_count: i64,
}

#[derive(Deserialize)]
struct BoardSummary {
    name: String,
    description: String,
    post_count: i64,
}

pub async fn get_followed_boards(
    pool: &Pool<Postgres>,
    user_id: i64,
) -> Result<Vec<FollowedBoardInfo>, SocialFeedError> {
    let boards: Vec<String> = sqlx::query_scalar(
        "SELECT board_name FROM follows WHERE follower_id = $1 ORDER BY created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error::<SocialFeedError>)?;

    let followed_set: HashSet<&str> = boards.iter().map(|s| s.as_str()).collect();

    let post_service_url =
        std::env::var("POST_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8082".to_string());

    let client = reqwest::Client::new();
    let url = format!("{}/mcf/active", post_service_url);

    let active_boards = client
        .get(&url)
        .send()
        .await
        .map_err(|_| SocialFeedError::UpstreamError)?
        .json::<Vec<BoardSummary>>()
        .await
        .map_err(|_| SocialFeedError::UpstreamError)?;

    let result: Vec<FollowedBoardInfo> = active_boards
        .into_iter()
        .filter(|b| followed_set.contains(b.name.as_str()))
        .map(|b| FollowedBoardInfo {
            id: b.name.clone(),
            name: b.name.clone(),
            description: b.description,
            post_count: b.post_count,
        })
        .collect();

    Ok(result)
}

pub async fn follow_board(
    pool: &Pool<Postgres>,
    user_id: i64,
    board_name: &str,
) -> Result<(), SocialFeedError> {
    sqlx::query(
        "INSERT INTO follows (follower_id, board_name) VALUES ($1, $2)
         ON CONFLICT (follower_id, board_name) DO NOTHING",
    )
    .bind(user_id)
    .bind(board_name)
    .execute(pool)
    .await
    .map_err(map_sqlx_error::<SocialFeedError>)?;

    Ok(())
}

pub async fn unfollow_board(
    pool: &Pool<Postgres>,
    user_id: i64,
    board_name: &str,
) -> Result<(), SocialFeedError> {
    sqlx::query("DELETE FROM follows WHERE follower_id = $1 AND board_name = $2")
        .bind(user_id)
        .bind(board_name)
        .execute(pool)
        .await
        .map_err(map_sqlx_error::<SocialFeedError>)?;

    Ok(())
}

pub async fn get_following_count(
    pool: &Pool<Postgres>,
    user_id: i64,
) -> Result<i64, SocialFeedError> {
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*)::BIGINT FROM follows WHERE follower_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(map_sqlx_error::<SocialFeedError>)?
            .unwrap_or(0);

    Ok(count)
}
