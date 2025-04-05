// app/src/models.rs

// data models for the rusty-flash-knowledge api

// dependencies
use crate::errors::FlashcardValidationError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// struct type to represent a flash card
#[derive(Debug, Deserialize, Eq, Serialize, FromRow, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FlashCard {
    pub id: Uuid,
    pub question: String,
    pub answer: String,
    pub topic: Option<String>,
    pub tags: Option<Vec<String>>,
    pub difficulty: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

        if let Some(d) = new.difficulty {
            if !(1..=5).contains(&d) {
                return Err(FlashcardValidationError::InvalidDifficulty(d));
            }
        }

        Ok(Self {
            id: Uuid::new_v4(),
            question: new.question.trim().to_string(),
            answer: new.answer.trim().to_string(),
            topic: new.topic.map(|s| s.trim().to_string()),
            tags: new.tags,
            difficulty: new.difficulty,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

// struct type to represent a new flash card, coming in as an input
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFlashCard {
    pub question: String,
    pub answer: String,
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
            topic: Some("intro".to_string()),
            tags: Some(vec!["memory-safe".to_string(), "fast".to_string()]),
            difficulty: Some(3),
        };

        let result = FlashCard::try_from(new);

        assert!(result.is_ok());
        let card = result.unwrap();
        assert_eq!(card.question, "What is Rust?");
        assert_eq!(card.difficulty, Some(3));
    }

    #[test]
    fn empty_question_is_invalid() {
        let new = NewFlashCard {
            question: "   ".to_string(),
            answer: "Valid answer".to_string(),
            topic: None,
            tags: None,
            difficulty: Some(2),
        };

        let result = FlashCard::try_from(new);
        assert_eq!(result, Err(FlashcardValidationError::EmptyQuestion));
    }

    #[test]
    fn empty_answer_is_invalid() {
        let new = NewFlashCard {
            question: "Valid question".to_string(),
            answer: "".to_string(),
            topic: None,
            tags: None,
            difficulty: Some(2),
        };

        let result = FlashCard::try_from(new);
        assert_eq!(result, Err(FlashcardValidationError::EmptyAnswer));
    }

    #[test]
    fn difficulty_out_of_bounds_is_invalid() {
        let new = NewFlashCard {
            question: "Valid question".to_string(),
            answer: "Valid answer".to_string(),
            topic: None,
            tags: None,
            difficulty: Some(99),
        };

        let result = FlashCard::try_from(new);
        assert_eq!(result, Err(FlashcardValidationError::InvalidDifficulty(99)));
    }
}
