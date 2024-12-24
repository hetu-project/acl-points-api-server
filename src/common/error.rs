use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use error_macros::ErrorWithCode;
use std::path::PathBuf;
use thiserror::Error;
use validator::ValidationErrors;

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

    #[error(transparent)]
    #[code(30001)]
    ValidationError(#[from] ValidationErrors),

    #[error("url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("user existed: {0}")]
    UserExisted(String),

    #[error("user not existed: {0}")]
    UserUnExisted(String),

    #[error("{0}")]
    CustomError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::Success => StatusCode::OK,
            Self::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ConfigMissing(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SeaOrmDBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CustomError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UrlParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::UserExisted(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::UserUnExisted(_) => StatusCode::UNPROCESSABLE_ENTITY,
        };

        (status, Json(serde_json::json!({"error":self.to_string()}))).into_response()
    }
}
