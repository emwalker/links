use anyhow::Result;
use axum::{
    routing::{get, post},
    Json, Router,
};
use recommendations::user;
use serde_derive::Serialize;
use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
struct AppState {
    _conn: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let db_url = "./data/development.sqlite3?mode=rwc";
    let conn = SqlitePool::connect(db_url)
        .await
        .expect("failed to open sqlite");
    let state = AppState { _conn: conn };

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(user::list))
        .route("/users", post(user::create))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize)]
struct Root {
    message: String,
    status: String,
}

async fn root() -> Json<Root> {
    Json(Root {
        message: "Recommendations, v1".to_string(),
        status: "up".to_string(),
    })
}
