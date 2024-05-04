use crate::types::{Error, ErrorMap, Pagination, Result, Topic};
use serde::Serialize;
use sqlx::{error::ErrorKind, QueryBuilder, Sqlite, SqlitePool};
use tracing::{event, Level};

pub const ROOT_ID: &str = "63fa2799-f9ba-41d2-8f8b-49c8eac659fc";

#[derive(Default)]
pub struct Search {
    pub id: Option<String>,
}

impl Search {
    fn add_where(query: &Option<Self>, builder: &mut QueryBuilder<Sqlite>) {
        if let Some(query) = query {
            if !query.is_empty() {
                builder.push("where ");

                if let Some(id) = &query.id {
                    builder.push("t.id = ");
                    builder.push_bind(id.to_string());
                }
            }
        }
    }

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

pub async fn fetch_all(
    conn: &SqlitePool,
    pagination: &Pagination,
    search: Option<Search>,
) -> Result<(Vec<Topic>, u32)> {
    let mut builder = QueryBuilder::new(
        r#"
        select t.id, t.name, t.updated_at
        from topics t
        "#,
    );
    Search::add_where(&search, &mut builder);

    let &Pagination { page, per_page } = pagination;
    let offset = page.saturating_sub(1).saturating_mul(per_page) as i64;
    builder.push("limit ");
    builder.push_bind(per_page);
    builder.push(" offset ");
    builder.push_bind(offset);

    let users = builder
        .build_query_as::<Row>()
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(Topic::from)
        .collect::<Vec<_>>();

    let mut builder = QueryBuilder::new("select count(*) from topics t ");
    Search::add_where(&search, &mut builder);
    let total = builder.build_query_scalar::<i32>().fetch_one(conn).await?;

    Ok((users, total as u32))
}

pub async fn fetch_optional(conn: &SqlitePool, search: Option<Search>) -> Result<Option<Topic>> {
    let (users, _count) = fetch_all(
        conn,
        &Pagination {
            page: 1,
            per_page: 10,
        },
        search,
    )
    .await?;
    Ok(users.into_iter().next())
}

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
