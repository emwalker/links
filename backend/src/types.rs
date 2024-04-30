use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("not found")]
    NotFound,

    #[error("server error")]
    InternalServerError,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct AppState {
    pub conn: SqlitePool,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self),
        )
            .into_response()
    }
}

#[derive(Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Editor,
    Unknown,
}

impl Role {
    pub fn to_id(&self) -> &'static str {
        match self {
            Self::Admin => "79197f85-fb60-486f-b9fe-0aa0b10dabe2",
            Self::Editor => "99d8335a-1c23-4ad3-a10f-7e63fb3599d2",
            Self::Unknown => "793dd5d3-7bf2-41b7-bd18-b2d6ba3d02c2",
        }
    }
}

impl From<&str> for Role {
    fn from(name: &str) -> Self {
        match name {
            "admin" => Self::Admin,
            "editor" => Self::Editor,
            _ => Self::Unknown,
        }
    }
}
