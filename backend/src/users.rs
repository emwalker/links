use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
// use axum_macros::debug_handler;
use serde_derive::Serialize;

use crate::store::users;
use crate::types::{AppState, Pagination, Result, User};

pub async fn fetch_all(
    State(state): State<AppState>,
    query: Query<Pagination>,
) -> Result<impl IntoResponse> {
    #[derive(Serialize)]
    pub struct ListUserResponse {
        total: u32,
        items: Vec<User>,
        page: u32,
        per_page: u32,
    }

    let pagination = query.0;
    let (items, total) = users::fetch_all(&state.conn, &pagination, None).await?;

    Ok(Json(ListUserResponse {
        page: 1,
        per_page: 10,
        total,
        items,
    }))
}

pub async fn fetch_one(
    State(state): State<AppState>,
    Path((user_id,)): Path<(String,)>,
) -> Result<(StatusCode, impl IntoResponse)> {
    #[derive(Serialize)]
    pub struct Response {
        user: Option<User>,
    }

    let user =
        users::fetch_optional(&state.conn, Some(users::Search { id: Some(user_id) })).await?;

    let status = if user.is_some() {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    };

    Ok((status, Json(Response { user })))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<users::CreatePayload>,
) -> Result<(StatusCode, impl IntoResponse)> {
    let result = users::create(&state.conn, &payload).await?;
    let status = if result.created {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    };
    Ok((status, Json(result)))
}
