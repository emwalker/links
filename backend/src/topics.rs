use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
// use axum_macros::debug_handler;
use serde_derive::Serialize;

use crate::store::topics;
use crate::types::{AppState, Result, Topic};

pub async fn fetch_all(State(state): State<AppState>) -> Result<impl IntoResponse> {
    #[derive(Serialize)]
    pub struct FetchAllResponse {
        total: usize,
        items: Vec<Topic>,
        page: usize,
    }

    let items = topics::fetch_all(&state.conn, None).await?;

    Ok(Json(FetchAllResponse {
        page: 1,
        total: items.len(),
        items,
    }))
}

pub async fn fetch_one(
    State(state): State<AppState>,
    Path((topic_id,)): Path<(String,)>,
) -> Result<(StatusCode, impl IntoResponse)> {
    #[derive(Serialize)]
    pub struct Response {
        topic: Option<Topic>,
    }

    let topic =
        topics::fetch_optional(&state.conn, Some(topics::Search { id: Some(topic_id) })).await?;

    let status = if topic.is_some() {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    };

    Ok((status, Json(Response { topic })))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<topics::CreatePayload>,
) -> Result<(StatusCode, impl IntoResponse)> {
    let result = topics::create(&state.conn, &payload).await?;
    let status = if result.created {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    };
    Ok((status, Json(result)))
}
