// app/src/routes/flashcards.rs

// dependencies
use crate::configuration::DatabaseConfig;
use crate::errors::ApiError;
use crate::models::{FlashCard, NewFlashCard, UpdatedFlashCard};
use crate::queries::{
    create_flashcard, delete_flashcard, list_flashcard, list_flashcards, update_flashcard,
};
use pavex::request::body::JsonBody;
use pavex::request::path::PathParams;
use pavex::response::{Response, body::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// struct type to represent the path parameters of an incoming request
#[PathParams]
pub struct FlashCardParams {
    pub id: String,
}

// struct type to represent the data for a flash card
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FlashCardContent {
    pub id: Uuid,
    pub question: String,
    pub answer: String,
}

// implement the From trait to convert the FlashCard type into a FlashCardResponse type
impl From<FlashCard> for FlashCardContent {
    fn from(card: FlashCard) -> Self {
        Self {
            id: card.id,
            question: card.question,
            answer: card.answer,
        }
    }
}

// struct type to represent a flash card response
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FlashCardResponse {
    pub msg: String,
    pub content: FlashCardContent,
}

// handler which lists all the flash cards in the database
pub async fn list_flashcards_handler(db: &DatabaseConfig) -> Result<Response, ApiError> {
    let pool = db.get_pool().await;
    let flash_cards = list_flashcards(pool).await?;
    let response_body: Vec<FlashCardResponse> = flash_cards
        .into_iter()
        .map(|flash_card| FlashCardResponse {
            msg: "success".to_string(),
            content: FlashCardContent::from(flash_card),
        })
        .collect();
    let json = Json::new(response_body)?;
    Ok(Response::ok().set_typed_body(json))
}

// handler which retrieves a flash card by id from the database
pub async fn list_flashcard_handler(
    db: &DatabaseConfig,
    params: &PathParams<FlashCardParams>,
) -> Result<Response, ApiError> {
    let id = Uuid::parse_str(&params.0.id).map_err(ApiError::UuidError)?;
    let pool = db.get_pool().await;
    let flash_card = list_flashcard(pool, id).await?;
    let response_body: FlashCardResponse = FlashCardResponse {
        msg: "success".to_string(),
        content: FlashCardContent::from(flash_card),
    };
    let json = Json::new(response_body)?;
    Ok(Response::ok().set_typed_body(json))
}

// handler which creates a new flash card in the database
pub async fn create_flashcard_handler(
    db: &DatabaseConfig,
    body: &JsonBody<NewFlashCard>,
) -> Result<Response, ApiError> {
    let body = body.0.clone();
    let pool = db.get_pool().await;
    let new_flash_card = FlashCard::try_from(body)?;
    let created_flash_card = create_flashcard(pool, new_flash_card).await?;
    let response_body: FlashCardResponse = FlashCardResponse {
        msg: "success".to_string(),
        content: FlashCardContent::from(created_flash_card),
    };
    let json = Json::new(response_body)?;
    Ok(Response::ok().set_typed_body(json))
}

// handler which updates a flash card in the database, given and id
pub async fn update_flashcard_handler(
    db: &DatabaseConfig,
    body: &JsonBody<UpdatedFlashCard>,
    params: &PathParams<FlashCardParams>,
) -> Result<Response, ApiError> {
    let id = Uuid::parse_str(&params.0.id).map_err(ApiError::UuidError)?;
    let body = body.0.clone();
    let pool = db.get_pool().await;
    let updated_flash_card = update_flashcard(pool, id, body).await?;
    let response_body: FlashCardResponse = FlashCardResponse {
        msg: "success".to_string(),
        content: FlashCardContent::from(updated_flash_card),
    };
    let json = Json::new(response_body)?;
    Ok(Response::ok().set_typed_body(json))
}

// handler which deletes a flash card from the database, given an id
pub async fn delete_flashcard_handler(
    db: &DatabaseConfig,
    params: &PathParams<FlashCardParams>,
) -> Result<Response, ApiError> {
    let id = Uuid::parse_str(&params.0.id).map_err(ApiError::UuidError)?;
    let pool = db.get_pool().await;
    let deleted = delete_flashcard(pool, id).await?;
    if deleted == 0 {
        return Err(ApiError::NotFound(format!(
            "Flashcard with id {} not found",
            id
        )));
    }

    Ok(Response::no_content())
}
