// app/src/routes/flashcards.rs

// dependencies
use crate::configuration::DatabaseConfig;
use crate::errors::ApiError;
use crate::models::{FlashCard, NewFlashCard, UpdatedFlashCard};
use crate::queries::{
    create_flashcard, delete_flashcard, list_flashcard, list_flashcards, list_flashcards_by_tag,
    list_flashcards_by_topic, list_tags, list_topics, random_flashcard, update_flashcard,
};
use pavex::request::body::JsonBody;
use pavex::request::path::PathParams;
use pavex::request::query::QueryParams;
use pavex::response::{Response, body::Json};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgDatabaseError;
use uuid::Uuid;

// struct type to represent the path parameters of an incoming request
#[PathParams]
pub struct FlashCardParams {
    pub id: String,
}

// struct type to represent the query parameters of an incoming request
#[derive(Deserialize)]
pub struct SearchParams {
    pub topic: Option<String>,
    pub tag: Option<String>,
}

// struct type to represent the data for a flash card
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FlashCardContent {
    pub id: Uuid,
    pub question: String,
    pub answer: String,
    pub topic: String,
}

// implement the From trait to convert the FlashCard type into a FlashCardResponse type
impl From<FlashCard> for FlashCardContent {
    fn from(card: FlashCard) -> Self {
        Self {
            id: card.id,
            question: card.question,
            answer: card.answer,
            topic: card.topic,
        }
    }
}

// struct type to represent a flash card response
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FlashCardResponse {
    pub msg: String,
    pub content: FlashCardContent,
}

// struct type to represent a response wrapping the list of topics
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TagsResponse {
    pub msg: String,
    pub content: Vec<String>,
}

// struct type to represent a response wrapping the list of topics
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TopicsResponse {
    pub msg: String,
    pub content: Vec<String>,
}

// handler which lists all the flash cards in the database; accepts query parameters
// either topic or tag and returns the appropriate results
pub async fn list_flashcards_handler(
    db: &DatabaseConfig,
    params: &QueryParams<SearchParams>,
) -> Result<Response, ApiError> {
    let pool = db.get_pool().await;

    let flash_cards = match &params.0.topic {
        Some(topic) => list_flashcards_by_topic(pool, topic).await?,
        None => match &params.0.tag {
            Some(tag) => list_flashcards_by_tag(pool, tag).await?,
            None => list_flashcards(pool).await?,
        },
    };

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

// handler which retrieves a list of flash card tags from the database
pub async fn list_flashcard_tags_handler(db: &DatabaseConfig) -> Result<Response, ApiError> {
    let pool = db.get_pool().await;

    let tags: Vec<String> = list_tags(pool).await?;

    let response_body: TagsResponse = TagsResponse {
        msg: "success".to_string(),
        content: tags,
    };

    let json = Json::new(response_body)?;

    Ok(Response::ok().set_typed_body(json))
}

// handler which retrieves a list of flash card topics from the database
pub async fn list_flashcard_topics_handler(db: &DatabaseConfig) -> Result<Response, ApiError> {
    let pool = db.get_pool().await;

    let topics: Vec<String> = list_topics(pool).await?;

    let response_body: TopicsResponse = TopicsResponse {
        msg: "success".to_string(),
        content: topics,
    };

    let json = Json::new(response_body)?;

    Ok(Response::ok().set_typed_body(json))
}

// handler which creates a new flash card in the database
pub async fn create_flashcard_handler(
    db: &DatabaseConfig,
    body: &JsonBody<NewFlashCard>,
) -> Result<Response, ApiError> {
    let pool = db.get_pool().await;
    let new_flash_card = FlashCard::try_from(body.0.clone())?;
    let created_flash_card = match create_flashcard(pool, &new_flash_card).await {
        Ok(card) => card,
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if let Some(pg_err) = db_err.try_downcast_ref::<PgDatabaseError>() {
                    if pg_err.constraint() == Some("flashcards_question_key") {
                        return Err(ApiError::DuplicateQuestion(new_flash_card.question));
                    }
                }
            }

            return Err(ApiError::from(e)); // use the original error
        }
    };
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
    let pool = db.get_pool().await;
    let updated_flash_card = update_flashcard(pool, id, &body.0).await?;
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

// handler which retrieves a random flash card from the database
pub async fn random_flashcard_handler(db: &DatabaseConfig) -> Result<Response, ApiError> {
    let pool = db.get_pool().await;

    let random_card = random_flashcard(pool).await?;

    match random_card {
        Some(card) => {
            let response = FlashCardResponse {
                msg: "success".to_string(),
                content: FlashCardContent::from(card),
            };
            let json = Json::new(response)?;
            Ok(Response::ok().set_typed_body(json))
        }
        None => Err(ApiError::NotFound("No flashcards available".into())),
    }
}
