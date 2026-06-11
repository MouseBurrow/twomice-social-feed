use crate::errors::SocialFeedError;
use crate::service;
use crate::service::FollowedBoardInfo;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use config::app_data::AppData;
use custom_headers::user_id::UserId;
use serde::Deserialize;

pub async fn get_following(
    State(app): State<AppData>,
    user_id: UserId,
) -> Result<Json<Vec<FollowedBoardInfo>>, SocialFeedError> {
    let boards = service::get_followed_boards(&app.pool, user_id.into()).await?;
    Ok(Json(boards))
}

pub async fn follow_board(
    State(app): State<AppData>,
    Path(board_name): Path<String>,
    user_id: UserId,
) -> Result<StatusCode, SocialFeedError> {
    service::follow_board(&app.pool, user_id.into(), &board_name).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn unfollow_board(
    State(app): State<AppData>,
    Path(board_name): Path<String>,
    user_id: UserId,
) -> Result<StatusCode, SocialFeedError> {
    service::unfollow_board(&app.pool, user_id.into(), &board_name).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct InternalFollowingQuery {
    pub user_id: i64,
}

pub async fn internal_get_following(
    State(app): State<AppData>,
    Query(query): Query<InternalFollowingQuery>,
) -> Result<Json<Vec<FollowedBoardInfo>>, SocialFeedError> {
    let boards = service::get_followed_boards(&app.pool, query.user_id).await?;
    Ok(Json(boards))
}
