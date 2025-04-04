// app/src/routes/flashcards.rs

// dependencies
use crate::models::{FlashCard, NewFlashCard};
use pavex::request::body::JsonBody;
use pavex::request::path::PathParams;
use pavex::response::{Response, body::Json};

// struct type to represent the path parameters of an incoming request
#[PathParams]
pub struct FlashCardParams {
    pub id: u64,
}

// handler which lists all the flash cards in the database
pub async fn list_flashcards_handler() -> Response {
    let cards: Vec<FlashCard> = Vec::new();

    let json = Json::new(cards).expect("Unable to serialize response body.");
    Response::ok().set_typed_body(json)
}

// handler which retrieves a flash card by id from the database
pub async fn get_flashcard(params: &PathParams<FlashCardParams>) -> Response {
    todo!()
}

// handler which creates a new flash card in the database
pub async fn create_flashcard(body: &JsonBody<NewFlashCard>) -> Response {
    todo!()
}

// handler which updates a flash card in the database, given and id
pub async fn update_flashcard(
    body: &JsonBody<NewFlashCard>,
    params: &PathParams<FlashCardParams>,
) -> Response {
    todo!()
}

// handler which deletes a flash card from the database, given an id
pub async fn delete_flashcard(params: &PathParams<FlashCardParams>) -> Response {
    todo!()
}
