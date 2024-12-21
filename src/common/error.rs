use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use error_macros::ErrorWithCode;
use std::path::PathBuf;
use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug, ErrorWithCode)]
pub enum AppError {
    #[error("Everything is fine.")]
    #[code(200)]
    Success,

    #[error("IO error:{0}")]
    IoError(#[from] std::io::Error),

    #[error("No operator config found at this path: {0}")]
    #[code(30003)]
    ConfigMissing(PathBuf),

    #[error("Unknown error occurred.")]
    #[code(30002)]
    UnknownError,

    #[error(transparent)]
    #[code(30001)]
    SeaOrmDBError(#[from] sea_orm::DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::Success => StatusCode::OK,
            Self::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ConfigMissing(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SeaOrmDBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(serde_json::json!({"error":self.to_string()}))).into_response()
    }
}
