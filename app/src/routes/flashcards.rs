// app/src/routes/flashcards.rs

// dependencies
use crate::configuration::DatabaseConfig;
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
    pub id: Uuid,
}

// struct type to represent a flash card as a response body
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FlashCardResponse {
    pub id: Uuid,
    pub question: String,
    pub answer: String,
}

// implement the From trait to convert the FlashCard type into a FlashCardResponse type
impl From<FlashCard> for FlashCardResponse {
    fn from(card: FlashCard) -> Self {
        Self {
            id: card.id,
            question: card.question,
            answer: card.answer,
        }
    }
}

// handler which lists all the flash cards in the database
pub async fn list_flashcards_handler(db: &DatabaseConfig) -> Response {
    let pool = db.get_pool().await;
    let flash_cards = list_flashcards(pool).await;
    let response_body: Vec<FlashCardResponse> = flash_cards
        .into_iter()
        .map(|flash_card| FlashCardResponse::from(flash_card))
        .collect();
    let json = Json::new(response_body).expect("Unable to serialize response body.");
    Response::ok().set_typed_body(json)
}

// handler which retrieves a flash card by id from the database
pub async fn list_flashcard_handler(
    db: &DatabaseConfig,
    params: &PathParams<FlashCardParams>,
) -> Response {
    let pool = db.get_pool().await;
    let flash_card = list_flashcard(pool, params.0.id.clone()).await;
    let response_body: FlashCardResponse = FlashCardResponse::from(flash_card);
    let json = Json::new(response_body).expect("Unable to serialize response body.");
    Response::ok().set_typed_body(json)
}

// handler which creates a new flash card in the database
pub async fn create_flashcard_handler(
    db: &DatabaseConfig,
    body: &JsonBody<NewFlashCard>,
) -> Response {
    let pool = db.get_pool().await;
    let new_flash_card = FlashCard::try_from(body.0.clone()).unwrap();
    let created_flash_card = create_flashcard(pool, new_flash_card).await;
    let response_body: FlashCardResponse = FlashCardResponse::from(created_flash_card);
    let json = Json::new(response_body).expect("Unable to serialize response body.");
    Response::ok().set_typed_body(json)
}

// handler which updates a flash card in the database, given and id
pub async fn update_flashcard_handler(
    db: &DatabaseConfig,
    body: &JsonBody<UpdatedFlashCard>,
    params: &PathParams<FlashCardParams>,
) -> Response {
    let pool = db.get_pool().await;
    let updated_flash_card = update_flashcard(pool, params.0.id.clone(), body.0.clone()).await;
    let response_body: FlashCardResponse = FlashCardResponse::from(updated_flash_card);
    let json = Json::new(response_body).expect("Unable to serialize response body.");
    Response::ok().set_typed_body(json)
}

// handler which deletes a flash card from the database, given an id
pub async fn delete_flashcard_handler(
    db: &DatabaseConfig,
    params: &PathParams<FlashCardParams>,
) -> Response {
    let pool = db.get_pool().await;
    delete_flashcard(pool, params.0.id.clone()).await;
    Response::no_content()
}
