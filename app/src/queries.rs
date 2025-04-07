// app/src/queries.rs

// dependencies
use crate::models::FlashCard;
use sqlx::PgPool;
use uuid::Uuid;

// function which queries the database and returns all the flash cards
pub async fn list_flashcards(pool: PgPool) -> Vec<FlashCard> {
    let flash_cards: Vec<FlashCard> =
        sqlx::query_as("SELECT * FROM flashcards ORDER BY created_at DESC;")
            .fetch_all(&pool)
            .await
            .unwrap();

    flash_cards
}

// function which queries the database and returns all the flash cards
pub async fn create_flashcard(pool: PgPool, new_card: FlashCard) -> FlashCard {
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
            .await
            .unwrap();

    new_flash_card
}

// function which queries the database, given a flash card id, and deletes that entry
pub async fn delete_flashcard(pool: PgPool, id: Uuid) {
    sqlx::query("DELETE FROM flashcards WHERE id = $1;")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
}
