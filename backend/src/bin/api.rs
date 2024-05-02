use anyhow::Result;
use axum::{
    http::Method,
    routing::{get, post},
    Json, Router,
};
use recommendations::{topics, types::AppState, users};
use serde_derive::Serialize;
use sqlx::sqlite::SqlitePool;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let db_url = "./data/development.db?mode=rwc";
    let conn = SqlitePool::connect(db_url)
        .await
        .expect("failed to open sqlite");
    recommendations::MIGRATOR
        .run(&conn)
        .await
        .expect("failed to run migrations");

    let state = AppState { conn };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .layer(cors)
        .route("/", get(root))
        .route("/topics", get(topics::fetch_all))
        .route("/topics/:topic_id", get(topics::fetch_one))
        .route("/topics", post(topics::create))
        .route("/users", get(users::fetch_all))
        .route("/users", post(users::create))
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
