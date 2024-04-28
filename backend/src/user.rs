use axum::{extract::State, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_derive::Serialize;

use crate::types::{AppState, Result};

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    id: String,
    username: String,
}

#[debug_handler]
pub async fn list(State(state): State<AppState>) -> Result<impl IntoResponse> {
    #[derive(Serialize)]
    pub struct ListUserResponse {
        total: usize,
        items: Vec<User>,
        page: usize,
    }

    let items = sqlx::query_as::<_, User>("select id, username from users")
        .fetch_all(&state.conn)
        .await?;

    Ok(Json(ListUserResponse {
        page: 1,
        total: items.len(),
        items,
    }))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct CreateUserResult {
    created_user_id: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse> {
    let created_user_id: String = uuid::Uuid::new_v4().into();
    sqlx::query("insert into users (id, username) values (?, ?)")
        .bind(&created_user_id)
        .bind(&payload.username)
        .execute(&state.conn)
        .await?;

    Ok(Json(CreateUserResult { created_user_id }))
}
