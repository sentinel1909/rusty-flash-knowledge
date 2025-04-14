// app/src/errors.rs

// dependencies
use pavex::response::Response;
use pavex::{http::StatusCode, response::body::errors::JsonSerializationError};
use serde::Serialize;
use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid API key")]
    ApiKeyError,

    #[error("Questions must be unique: {0}")]
    DuplicateQuestion(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Error serializing response data: {0}")]
    SerializationError(#[from] JsonSerializationError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Uuid parsing error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("Error validating incoming data: {0}")]
    ValidationError(#[from] FlashcardValidationError),
}

#[derive(Debug, PartialEq, Error)]
pub enum FlashcardValidationError {
    #[error("Question field cannot be empty.")]
    EmptyQuestion,

    #[error("Answer field cannot be empty.")]
    EmptyAnswer,

    #[error("Topic field cannot be empty.")]
    EmptyTopic,

    #[error("Tags field cannot be empty.")]
    EmptyTags,

    #[error("Invalid difficulty level. Difficulty must be between 1 and 5")]
    InvalidDifficulty,
}

#[derive(Serialize)]
struct ErrorResponse {
    msg: String,
    status: u16,
    details: String,
}

// error handler for the static server endpoint
pub fn api_error2response(error: &ApiError) -> Response {
    let status = match error {
        ApiError::ApiKeyError => StatusCode::UNAUTHORIZED,
        ApiError::DuplicateQuestion(_) => StatusCode::CONFLICT,
        ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        ApiError::NotFound(_) => StatusCode::NOT_FOUND,
        ApiError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        ApiError::UuidError(_) => StatusCode::BAD_REQUEST,
        ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
    };

    let payload = ErrorResponse {
        msg: "Error".to_string(),
        status: status.as_u16(),
        details: error.to_string(),
    };

    let json = serde_json::to_string(&payload).unwrap_or_else(|_| {
        r#"{"msg":"Error","status":500,"details":"Internal server error formatting error response"}"#.to_string()
    });

    Response::new(status).set_typed_body(json)
}
