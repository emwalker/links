use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
// use axum_macros::debug_handler;
use serde_derive::Serialize;

use crate::store;
use crate::types::{AppState, Result, Topic};

#[derive(Serialize)]
pub struct ListTopicResponse {
    total: usize,
    items: Vec<Topic>,
    page: usize,
}

pub async fn list(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let items = store::topics::fetch_all(&state.conn, None).await?;

    Ok(Json(ListTopicResponse {
        page: 1,
        total: items.len(),
        items,
    }))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<store::topics::CreatePayload>,
) -> Result<(StatusCode, impl IntoResponse)> {
    let result = store::topics::create(&state.conn, &payload).await?;
    let status = if result.created {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    };
    Ok((status, Json(result)))
}
