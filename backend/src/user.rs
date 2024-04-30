use std::collections::HashSet;

use axum::{extract::State, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_derive::Serialize;

use crate::types::{AppState, Result, Role};

#[derive(Serialize)]
pub struct User {
    id: String,
    username: String,
    roles: Vec<Role>,
}

#[derive(sqlx::FromRow)]
pub struct UserRow {
    id: String,
    username: String,
    role_names: String,
}

#[debug_handler]
pub async fn list(State(state): State<AppState>) -> Result<impl IntoResponse> {
    #[derive(Serialize)]
    pub struct ListUserResponse {
        total: usize,
        items: Vec<User>,
        page: usize,
    }

    let items = sqlx::query_as::<_, UserRow>(
        r#"
        select u.id, u.username, group_concat(r.name, ",") role_names
        from users u
        join users_roles ur on u.id = ur.user_id
        join roles r on ur.role_id = r.id
        "#,
    )
    .fetch_all(&state.conn)
    .await?
    .into_iter()
    .map(
        |UserRow {
             id,
             username,
             role_names,
         }| User {
            id,
            username,
            roles: role_names
                .split(',')
                .map(Role::from)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
        },
    )
    .collect::<Vec<_>>();

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

    sqlx::query("insert into users_roles (user_id, role_id) values (?, ?)")
        .bind(&created_user_id)
        .bind(Role::Editor.to_id())
        .execute(&state.conn)
        .await?;

    Ok(Json(CreateUserResult { created_user_id }))
}
