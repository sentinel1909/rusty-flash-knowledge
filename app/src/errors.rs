// app/src/errors.rs

// dependencies
use pavex::{http::StatusCode, response::body::errors::JsonSerializationError};

// error type to represent possible API failure variants
#[derive(Debug)]
pub enum ApiError {
    ValidationError(FlashcardValidationError),
    SerializationError(JsonSerializationError),
    DatabaseError(sqlx::Error),
}

// error type to represent possible data validation failure variants
#[derive(Debug, PartialEq)]
pub enum FlashcardValidationError {
    EmptyQuestion,
    EmptyAnswer,
    EmptyTopic,
    EmptyTags,
    InvalidDifficulty,
}

// implement the From trait to convert from a FlashcardValidationError to an ApiError
impl From<FlashcardValidationError> for ApiError {
    fn from(err: FlashcardValidationError) -> Self {
        ApiError::ValidationError(err)
    }
}

// implement the From trait to convert from a JsonSerializationError to an ApiError
impl From<JsonSerializationError> for ApiError {
    fn from(err: JsonSerializationError) -> Self {
        ApiError::SerializationError(err)
    }
}

// implement the From trait to convert from a DatabaseError to an ApiError
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::DatabaseError(err)
    }
}

// error handler for the static server endpoint
pub fn api_error2response(e: &ApiError) -> StatusCode {
    match e {
        ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
        ApiError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// implement the Error trait for the ApiError type
impl std::error::Error for ApiError {}

// implement the Display trait for the ApiError type
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidationError(err) => write!(f, "Error validating incoming data: {}", err),
            Self::SerializationError(err) => {
                write!(f, "Error serializing response data: {}", err)
            }
            Self::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

// implement the Error trait for the FlashcardValidationError type
impl std::error::Error for FlashcardValidationError {}

// implement the Display trait for the FlashcardValidationError type
impl std::fmt::Display for FlashcardValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyQuestion => write!(f, "Question field cannot be empty."),
            Self::EmptyAnswer => write!(f, "Answer field cannot be empty."),
            Self::EmptyTopic => write!(f, "Topic field cannot be empty."),
            Self::EmptyTags => write!(f, "Tags field cannot be empty."),
            Self::InvalidDifficulty => write!(
                f,
                "Invalid difficulty level. Difficulty must be between 1 and 5"
            ),
        }
    }
}
