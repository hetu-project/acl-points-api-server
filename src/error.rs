use error_macros::ErrorWithCode;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, ErrorWithCode)]
pub enum Error {
    #[error("No operator config found at this path: {0}")]
    #[code(30003)]
    ConfigMissing(PathBuf),

    #[error("Unknown error occurred.")]
    #[code(30002)]
    UnknownError,

    #[error("Everything is fine.")]
    #[code(200)]
    Success,

    #[error(transparent)]
    #[code(30001)]
    SeaOrmDBError(#[from] sea_orm::DbErr),
}
