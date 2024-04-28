use axum::{response::IntoResponse, Json};
// use axum_macros::debug_handler;
use serde::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct User;

pub async fn list() -> impl IntoResponse {
    let users: Vec<User> = vec![];
    Json(users)
}

#[derive(Deserialize)]
pub struct CreateUser;

#[derive(Serialize)]
pub struct CreateUserResult;

pub async fn create(Json(_payload): Json<CreateUser>) -> impl IntoResponse {
    Json(CreateUserResult)
}
