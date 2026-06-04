use crate::errors::SocialFeedError;
use easy_errors::map_sqlx_error;
use sqlx::{Pool, Postgres};

pub async fn get_followed_boards(
    pool: &Pool<Postgres>,
    user_id: i64,
) -> Result<Vec<String>, SocialFeedError> {
    let boards: Vec<String> = sqlx::query_scalar(
        "SELECT board_name FROM follows WHERE follower_id = $1 ORDER BY created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error::<SocialFeedError>)?;

    Ok(boards)
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
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::BIGINT FROM follows WHERE follower_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error::<SocialFeedError>)?
    .unwrap_or(0);

    Ok(count)
}
