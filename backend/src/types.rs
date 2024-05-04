use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
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

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let error = json!({
            "message": format!("{}", self)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct AppState {
    pub conn: SqlitePool,
}

#[derive(Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Editor,
    Visitor,
}

impl Role {
    pub fn to_id(&self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Editor => "editor",
            Self::Visitor => "visitor",
        }
    }
}

impl From<&str> for Role {
    fn from(name: &str) -> Self {
        match name {
            "admin" => Self::Admin,
            "editor" => Self::Editor,
            _ => Self::Visitor,
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
pub struct Topic {
    pub id: String,
    pub name: String,
    pub updated_at: String,
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

#[derive(Deserialize)]
#[serde(default)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
        }
    }
}
