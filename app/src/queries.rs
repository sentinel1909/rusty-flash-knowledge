// app/src/queries.rs

// dependencies
use crate::configuration::DatabaseConfig;
use crate::models::FlashCard;

// function which queries the database and returns all the flash cards
pub async fn list_flashcards(db: &DatabaseConfig) -> Vec<FlashCard> {
    let pool = db.get_pool().await.unwrap();
    
    let flash_cards: Vec<FlashCard> = sqlx::query_as("SELECT * FROM flashcards ORDER BY created_at DESC;")
        .fetch_all(&pool)
        .await
        .unwrap();

    flash_cards
}
