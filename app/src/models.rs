// app/src/models.rs

// data models for the rusty-flash-knowledge api

// dependencies
use pavex::time::Timestamp;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// struct type to represent a flash card
#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct FlashCard {
  pub id: Uuid,
  pub question: String,
  pub answer: String,
  pub topic: Option<String>,
  pub tags: Option<Vec<String>>,
  pub difficulty: Option<i32>,
  pub created_at: Timestamp,
  pub updated_at: Timestamp,
}

// struct type to represent a new flash card, coming in as an input
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFlashCard {
  pub question: String,
  pub answer: String,
  pub topic: Option<String>,
  pub tags: Option<Vec<String>>,
  pub difficulty: Option<i32>,
}