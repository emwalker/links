use crate::types::{Error, ErrorMap, Result, User};
use serde::{Deserialize, Serialize};
use sqlx::{error::ErrorKind, QueryBuilder, SqlitePool};
use tracing::{event, Level};

pub const ROOT_ID: &str = "2db58326-ddfa-4561-9ae2-232aa5c32277";

#[derive(Default)]
pub struct Search {
    pub id: Option<String>,
}

impl Search {
    fn is_empty(&self) -> bool {
        self.id.is_none()
    }
}

#[derive(sqlx::FromRow)]
struct Row {
    id: String,
    username: String,
    name: String,
    is_admin: bool,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        let Row {
            id,
            username,
            name,
            is_admin,
        } = row;
        Self {
            id,
            username,
            name,
            is_admin,
        }
    }
}

pub async fn fetch_all(conn: &SqlitePool, search: Option<Search>) -> Result<Vec<User>> {
    let mut query = QueryBuilder::new(
        r#"
            select u.id, u.username, u.name, u.is_admin
            from users u
            "#,
    );

    if let Some(search) = search {
        if !search.is_empty() {
            query.push("where ");

            if let Some(id) = &search.id {
                query.push("u.id = ");
                query.push_bind(id.to_string());
            }
        }
    }

    let users = query
        .build_query_as::<Row>()
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(User::from)
        .collect::<Vec<_>>();

    Ok(users)
}

#[derive(Deserialize)]
pub struct CreatePayload {
    pub username: String,
}

#[derive(Serialize)]
pub struct CreateResult {
    pub user_id: Option<String>,
    pub errors: ErrorMap,
    pub created: bool,
}

pub async fn create(conn: &SqlitePool, payload: &CreatePayload) -> Result<CreateResult> {
    let id: String = uuid::Uuid::new_v4().into();
    let result = sqlx::query(r#"insert into users (id, username) values (?, ?)"#)
        .bind(&id)
        .bind(&payload.username)
        .execute(conn)
        .await;

    match result {
        Ok(_) => {
            event!(Level::INFO, r#"created user {} ({id})"#, payload.username,);

            Ok(CreateResult {
                user_id: Some(id),
                errors: ErrorMap::empty(),
                created: true,
            })
        }

        Err(err) => {
            if let Some(err) = err.as_database_error() {
                if err.kind() == ErrorKind::UniqueViolation {
                    let message = format!(r#"Username already taken: {}"#, payload.username);
                    event!(Level::INFO, message);

                    return Ok(CreateResult {
                        user_id: None,
                        errors: ErrorMap::from_error("username".into(), message),
                        created: false,
                    });
                }
            }

            event!(Level::WARN, "failed to create user: {err}");
            Err(Error::Database(err))
        }
    }
}
