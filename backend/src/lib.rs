#![forbid(unsafe_code)]

pub mod store;
pub mod types;
pub mod user;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
