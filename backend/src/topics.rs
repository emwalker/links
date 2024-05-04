use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
// use axum_macros::debug_handler;
use serde_derive::Serialize;

use crate::store::{topics, users};
use crate::types::{AppState, Pagination, Result, Topic};

pub async fn fetch_all(
    State(state): State<AppState>,
    query: Query<Pagination>,
) -> Result<impl IntoResponse> {
    #[derive(Serialize)]
    pub struct FetchAllResponse {
        total: u32,
        items: Vec<Topic>,
        page: u32,
        per_page: u32,
    }

    let pagination = query.0;
    let (items, total) = topics::fetch_all(&state.conn, &pagination, None).await?;

    Ok(Json(FetchAllResponse {
        page: pagination.page.try_into().unwrap_or(1),
        per_page: pagination.per_page.try_into().unwrap_or(10),
        total,
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

#[derive(Deserialize)]
pub struct CreatePayload {
    name: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(CreatePayload { name }): Json<CreatePayload>,
) -> Result<(StatusCode, impl IntoResponse)> {
    // TODO: Get owner id from request authentication
    let result = topics::create(
        &state.conn,
        &topics::CreatePayload {
            name,
            owner_id: users::ROOT_ID.to_owned(),
        },
    )
    .await?;

    let status = if result.created {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    };
    Ok((status, Json(result)))
}
