use anyhow::Result;
use axum::{
    http::Method,
    routing::{get, post},
    Json, Router,
};
use links::{topics, types::AppState, users};
use serde_derive::Serialize;
use sqlx::sqlite::SqlitePool;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::INFO);
    let tracing_layer = tracing_subscriber::fmt::layer().compact().with_ansi(false);
    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    let db_url = "./data/development.db?mode=rwc";
    let conn = SqlitePool::connect(db_url)
        .await
        .expect("failed to open sqlite");
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
        .route("/users/:user_id", get(users::fetch_one))
        .layer(TraceLayer::new_for_http())
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
        message: "Links, v1".to_string(),
        status: "up".to_string(),
    })
}
