use axum::{extract::State, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_derive::Serialize;
use std::collections::HashSet;
use tracing::{event, Level};

use crate::types::{AppState, Error, Result, Role};

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
        group by u.id, u.username
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
pub struct AddUserResult {
    created_user_id: String,
    created: bool,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse> {
    let user_id: String = uuid::Uuid::new_v4().into();
    let created_user_id: Option<(String,)> = sqlx::query_as(
        r#"insert into users (id, username) values (?, ?)
            on conflict(username) do nothing
            returning id"#,
    )
    .bind(&user_id)
    .bind(&payload.username)
    .fetch_optional(&state.conn)
    .await?;

    let actual_user_id = if let Some((user_id,)) = created_user_id {
        user_id
    } else {
        let existing_user_id: Option<(String,)> =
            sqlx::query_as(r#"select id from users where username = ?"#)
                .bind(&payload.username)
                .fetch_optional(&state.conn)
                .await?;
        existing_user_id.ok_or(Error::UserCreationError)?.0
    };

    event!(
        Level::INFO,
        "upserted user {} ({actual_user_id}",
        payload.username
    );

    sqlx::query(
        r#"insert into users_roles (user_id, role_id) values (?, ?)
            on conflict(user_id, role_id) do nothing"#,
    )
    .bind(&actual_user_id)
    .bind(Role::Editor.to_id())
    .execute(&state.conn)
    .await?;

    Ok(Json(AddUserResult {
        created: true,
        created_user_id: actual_user_id,
    }))
}
