// app/src/models.rs

// data models for the rusty-flash-knowledge api

// dependencies
use crate::errors::FlashcardValidationError;
use jiff_sqlx::{Timestamp as SqlxTimestamp, ToSqlx};
use pavex::time::Timestamp as PavexTimestamp;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// struct type to represent a flash card
#[derive(Clone, Debug, Eq, FromRow, PartialEq)]
pub struct FlashCard {
    pub id: Uuid,
    pub question: String,
    pub answer: String,
    pub topic: String,
    pub tags: Vec<String>,
    pub difficulty: i32,
    pub created_at: SqlxTimestamp,
    pub updated_at: Option<SqlxTimestamp>,
}

// implement the TryFrom trait, which aids in converting new data into the domain data model
impl TryFrom<NewFlashCard> for FlashCard {
    type Error = FlashcardValidationError;

    fn try_from(new: NewFlashCard) -> Result<Self, Self::Error> {
        if new.question.trim().is_empty() {
            return Err(FlashcardValidationError::EmptyQuestion);
        }

        if new.answer.trim().is_empty() {
            return Err(FlashcardValidationError::EmptyAnswer);
        }

        if new.topic.trim().is_empty() {
            return Err(FlashcardValidationError::EmptyTopic);
        }

        if new.tags.is_empty() {
            return Err(FlashcardValidationError::EmptyTags);
        }

        if new.difficulty < 1 || new.difficulty > 5 {
            return Err(FlashcardValidationError::InvalidDifficulty);
        }

        let now = PavexTimestamp::now().to_sqlx();

        Ok(Self {
            id: Uuid::new_v4(),
            question: new.question.trim().to_string(),
            answer: new.answer.trim().to_string(),
            topic: new.topic.trim().to_string(),
            tags: new.tags,
            difficulty: new.difficulty,
            created_at: now,
            updated_at: None,
        })
    }
}

// struct type to represent a new flash card, coming in as an input
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFlashCard {
    pub question: String,
    pub answer: String,
    pub topic: String,
    pub tags: Vec<String>,
    pub difficulty: i32,
}

// struct type to represent an updated flash card, coming in as input
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedFlashCard {
    pub question: Option<String>,
    pub answer: Option<String>,
    pub topic: Option<String>,
    pub tags: Option<Vec<String>>,
    pub difficulty: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::FlashcardValidationError;

    #[test]
    fn valid_flashcard_conversion() {
        let new = NewFlashCard {
            question: "What is Rust?".to_string(),
            answer: "A systems programming language.".to_string(),
            topic: "intro".to_string(),
            tags: vec!["memory-safe".to_string(), "fast".to_string()],
            difficulty: 3,
        };

        let result = FlashCard::try_from(new);

        assert!(result.is_ok());
        let card = result.unwrap();
        assert_eq!(card.question, "What is Rust?");
        assert_eq!(card.answer, "A systems programming language.");
        assert_eq!(card.topic, "intro");
        assert!(!card.tags.is_empty());
        assert_eq!(card.difficulty, 3);
    }

    #[test]
    fn empty_question_is_invalid() {
        let new = NewFlashCard {
            question: "   ".to_string(),
            answer: "valid answer".to_string(),
            topic: "valid topic".to_string(),
            tags: vec!["valid tag".to_string()],
            difficulty: 1,
        };

        let result = FlashCard::try_from(new);
        assert_eq!(result, Err(FlashcardValidationError::EmptyQuestion));
    }

    #[test]
    fn empty_answer_is_invalid() {
        let new = NewFlashCard {
            question: "Valid question".to_string(),
            answer: "".to_string(),
            topic: "valid topic".to_string(),
            tags: vec!["valid tag".to_string()],
            difficulty: 1,
        };

        let result = FlashCard::try_from(new);
        assert_eq!(result, Err(FlashcardValidationError::EmptyAnswer));
    }

    #[test]
    fn empty_topic_is_invalid() {
        let new = NewFlashCard {
            question: "valid question".to_string(),
            answer: "valid answer".to_string(),
            topic: "".to_string(),
            tags: vec!["valid tag".to_string()],
            difficulty: 1,
        };

        let result = FlashCard::try_from(new);
        assert_eq!(result, Err(FlashcardValidationError::EmptyTopic));
    }

    #[test]
    fn empty_tags_are_invalid() {
        let new = NewFlashCard {
            question: "valid question".to_string(),
            answer: "valid answer".to_string(),
            topic: "valid topic".to_string(),
            tags: vec![],
            difficulty: 1,
        };

        let result = FlashCard::try_from(new);
        assert_eq!(result, Err(FlashcardValidationError::EmptyTags));
    }

    #[test]
    fn difficulty_out_of_bounds_is_invalid() {
        let new = NewFlashCard {
            question: "Valid question".to_string(),
            answer: "Valid answer".to_string(),
            topic: "valid topic".to_string(),
            tags: vec!["valid tag".to_string()],
            difficulty: 99,
        };

        let result = FlashCard::try_from(new);
        assert_eq!(result, Err(FlashcardValidationError::InvalidDifficulty));
    }
}
