use crate::types::{Error, ErrorMap, Result, Topic};
use serde::{Deserialize, Serialize};
use sqlx::{error::ErrorKind, QueryBuilder, SqlitePool};
use tracing::{event, Level};

pub const ROOT_ID: &str = "63fa2799-f9ba-41d2-8f8b-49c8eac659fc";

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
    name: String,
    updated_at: String,
}

impl From<Row> for Topic {
    fn from(row: Row) -> Self {
        let Row {
            id,
            name,
            updated_at,
        } = row;
        Self {
            id,
            name,
            updated_at,
        }
    }
}

pub async fn fetch_all(conn: &SqlitePool, search: Option<Search>) -> Result<Vec<Topic>> {
    let mut query = QueryBuilder::new(
        r#"
        select t.id, t.name, t.updated_at
        from topics t
        "#,
    );

    if let Some(search) = search {
        if !search.is_empty() {
            query.push("where ");

            if let Some(id) = &search.id {
                query.push("t.id = ");
                query.push_bind(id.to_string());
            }
        }
    }

    let users = query
        .build_query_as::<Row>()
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(Topic::from)
        .collect::<Vec<_>>();

    Ok(users)
}

#[derive(Deserialize)]
pub struct CreatePayload {
    pub owner_id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateResult {
    pub topic_id: Option<String>,
    pub errors: ErrorMap,
    pub created: bool,
}

pub async fn create(conn: &SqlitePool, payload: &CreatePayload) -> Result<CreateResult> {
    let id: String = uuid::Uuid::new_v4().into();
    let result = sqlx::query(r#"insert into topics (id, owner_id, name) values (?, ?, ?)"#)
        .bind(&id)
        .bind(&payload.owner_id)
        .bind(&payload.name)
        .execute(conn)
        .await;

    match result {
        Ok(_) => {
            event!(Level::INFO, r#"created topic {} ({id})"#, payload.name,);

            Ok(CreateResult {
                topic_id: Some(id),
                errors: ErrorMap::empty(),
                created: true,
            })
        }

        Err(err) => {
            if let Some(err) = err.as_database_error() {
                if err.kind() == ErrorKind::UniqueViolation {
                    let message = format!(r#"Topic name already taken: {}"#, payload.name);
                    event!(Level::INFO, message);

                    return Ok(CreateResult {
                        topic_id: None,
                        errors: ErrorMap::from_error("name".into(), message),
                        created: false,
                    });
                }
            }

            event!(Level::WARN, "failed to create topic: {err}");
            Err(Error::Database(err))
        }
    }
}
