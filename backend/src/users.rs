use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
// use axum_macros::debug_handler;
use serde_derive::Serialize;

use crate::store::users;
use crate::types::{AppState, Result, User};

pub async fn fetch_all(State(state): State<AppState>) -> Result<impl IntoResponse> {
    #[derive(Serialize)]
    pub struct ListUserResponse {
        total: usize,
        items: Vec<User>,
        page: usize,
    }

    let items = users::fetch_all(&state.conn, None).await?;

    Ok(Json(ListUserResponse {
        page: 1,
        total: items.len(),
        items,
    }))
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