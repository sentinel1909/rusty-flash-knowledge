// app/src/queries.rs

// dependencies
use crate::models::FlashCard;
use sqlx::PgPool;

// function which queries the database and returns all the flash cards
pub async fn list_flashcards(pool: PgPool) -> Vec<FlashCard> {
    let flash_cards: Vec<FlashCard> =
        sqlx::query_as("SELECT * FROM flashcards ORDER BY created_at DESC;")
            .fetch_all(&pool)
            .await
            .unwrap();

    flash_cards
}
