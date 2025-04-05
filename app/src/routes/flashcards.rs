// app/src/routes/flashcards.rs

// dependencies
use crate::configuration::DatabaseConfig;
use crate::models::{FlashCard, NewFlashCard};
use crate::queries::list_flashcards;
use pavex::request::body::JsonBody;
use pavex::request::path::PathParams;
use pavex::response::{Response, body::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// struct type to represent the path parameters of an incoming request
#[PathParams]
pub struct FlashCardParams {
    pub id: u64,
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
    let pool = db.get_pool().await.unwrap();
    let flash_cards = list_flashcards(pool).await;
    let response_body: Vec<FlashCardResponse> = flash_cards.into_iter().map(Into::into).collect();
    let json = Json::new(response_body).expect("Unable to serialize response body.");
    Response::ok().set_typed_body(json)
}

// handler which retrieves a flash card by id from the database
pub async fn get_flashcard(db: &DatabaseConfig, params: &PathParams<FlashCardParams>) -> Response {
    todo!()
}

// handler which creates a new flash card in the database
pub async fn create_flashcard(db: &DatabaseConfig, body: &JsonBody<NewFlashCard>) -> Response {
    todo!()
}

// handler which updates a flash card in the database, given and id
pub async fn update_flashcard(
    db: &DatabaseConfig,
    body: &JsonBody<NewFlashCard>,
    params: &PathParams<FlashCardParams>,
) -> Response {
    todo!()
}

// handler which deletes a flash card from the database, given an id
pub async fn delete_flashcard(
    db: &DatabaseConfig,
    params: &PathParams<FlashCardParams>,
) -> Response {
    todo!()
}
