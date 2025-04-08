// app/src/errors.rs

// error type to represent possible data validation failure variants
#[derive(Debug, PartialEq)]
pub enum FlashcardValidationError {
    EmptyQuestion,
    EmptyAnswer,
    EmptyTopic,
    EmptyTags,
    InvalidDifficulty,
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
