// app/src/queries.rs

// dependencies
use crate::errors::ApiError;
use crate::{UpdatedFlashCard, models::FlashCard};
use jiff_sqlx::ToSqlx;
use pavex::time::Timestamp as PavexTimestamp;
use sqlx::PgPool;
use uuid::Uuid;

// function which queries the database and returns all the flash cards
pub async fn list_flashcards(pool: PgPool) -> Result<Vec<FlashCard>, ApiError> {
    let flash_cards: Vec<FlashCard> =
        sqlx::query_as("SELECT * FROM flashcards ORDER BY created_at DESC;")
            .fetch_all(&pool)
            .await?;

    Ok(flash_cards)
}

// function which queries the database and returns a single flash card give an id
pub async fn list_flashcard(pool: PgPool, id: Uuid) -> Result<FlashCard, ApiError> {
    let flash_card = sqlx::query_as("SELECT * FROM flashcards WHERE id = $1;")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(flash_card)
}

// function which queries the database and returns all the flash cards
pub async fn create_flashcard(pool: PgPool, new_card: FlashCard) -> Result<FlashCard, ApiError> {
    let new_flash_card: FlashCard =
        sqlx::query_as("INSERT INTO flashcards (id, question, answer, topic, tags, difficulty, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;")
            .bind(new_card.id)
            .bind(new_card.question)
            .bind(new_card.answer)
            .bind(new_card.topic)
            .bind(new_card.tags)
            .bind(new_card.difficulty)
            .bind(new_card.created_at)
            .bind(new_card.updated_at)
            .fetch_one(&pool)
            .await?;

    Ok(new_flash_card)
}

// function which queries the database, given a flash card id, and deletes that entry
pub async fn delete_flashcard(pool: PgPool, id: Uuid) -> Result<u64, ApiError> {
    let result = sqlx::query("DELETE FROM flashcards WHERE id = $1;")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(result.rows_affected())
}

// function which queries the database, given a flashcard id, and updates that entry
pub async fn update_flashcard(
    pool: PgPool,
    id: Uuid,
    updated_card: UpdatedFlashCard,
) -> Result<FlashCard, ApiError> {
    let updated_flash_card: FlashCard = sqlx::query_as("UPDATE flashcards SET question = $1, answer = $2, topic = $3, tags = $4, difficulty = $5, updated_at = $6 WHERE id = $7 RETURNING *;")
        .bind(updated_card.question)
        .bind(updated_card.answer)
        .bind(updated_card.topic)
        .bind(updated_card.tags)
        .bind(updated_card.difficulty)
        .bind(Some(PavexTimestamp::now().to_sqlx()))
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(updated_flash_card)
}
