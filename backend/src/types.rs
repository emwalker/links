use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not found")]
    NotFound,
    #[error("server error")]
    InternalServerError,
}

pub type Result<T> = std::result::Result<T, Error>;
