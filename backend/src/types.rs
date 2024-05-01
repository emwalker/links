use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("not found")]
    NotFound,

    #[error("server error")]
    InternalServerError,

    #[error("failed to create user")]
    UserCreationError,
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

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub is_admin: bool,
}

#[derive(Serialize)]
pub struct ErrorMap(BTreeMap<String, Vec<String>>);

impl ErrorMap {
    pub fn empty() -> Self {
        Self(BTreeMap::new())
    }

    pub fn from_error(field: String, message: String) -> Self {
        Self(BTreeMap::from([(field, vec![message])]))
    }

    pub fn messages(&self, field: &str) -> Vec<String> {
        self.0.get(field).cloned().unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
