#![forbid(unsafe_code)]

pub mod store;
pub mod topics;
pub mod types;
pub mod users;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
