// app/src/queries.rs

// dependencies
use crate::{UpdatedFlashCard, models::FlashCard};
use jiff_sqlx::ToSqlx;
use pavex::time::Timestamp as PavexTimestamp;
use sqlx::PgPool;
use uuid::Uuid;

// function which queries the database and returns all the flash cards
pub async fn list_flashcards(pool: PgPool) -> Result<Vec<FlashCard>, sqlx::Error> {
    let flash_cards: Vec<FlashCard> =
        sqlx::query_as("SELECT * FROM flashcards ORDER BY created_at DESC;")
            .fetch_all(&pool)
            .await?;

    Ok(flash_cards)
}

// function which queries the database and returns all the flash cards filtered by topic
pub async fn list_flashcards_by_topic(
    pool: PgPool,
    topic: &str,
) -> Result<Vec<FlashCard>, sqlx::Error> {
    let flash_cards: Vec<FlashCard> = sqlx::query_as(
        "SELECT *
            FROM flashcards
            WHERE topic ILIKE $1
            ORDER BY created_at DESC;",
    )
    .bind(topic)
    .fetch_all(&pool)
    .await?;

    Ok(flash_cards)
}

// function which queries the database and returns a single flash card give an id
pub async fn list_flashcard(pool: PgPool, id: Uuid) -> Result<FlashCard, sqlx::Error> {
    let flash_card = sqlx::query_as("SELECT * FROM flashcards WHERE id = $1;")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(flash_card)
}

// function which queries the database and returns a list of available tags
pub async fn list_tags(pool: PgPool) -> Result<Vec<String>, sqlx::Error> {
    let tags: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT UNNEST(tags) AS tag
FROM flashcards
WHERE tags IS NOT NULL
ORDER BY tag ASC;",
    )
    .fetch_all(&pool)
    .await?;

    Ok(tags)
}

// function which queries the database and returns a list of available topics
pub async fn list_topics(pool: PgPool) -> Result<Vec<String>, sqlx::Error> {
    let topics: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT topic
        FROM flashcards
        ORDER BY topic ASC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(topics)
}

// function which queries the database and returns all the flash cards
pub async fn create_flashcard(
    pool: PgPool,
    new_card: &FlashCard,
) -> Result<FlashCard, sqlx::Error> {
    let new_flash_card: FlashCard =
        sqlx::query_as("INSERT INTO flashcards (id, question, answer, topic, tags, difficulty, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;")
            .bind(new_card.id)
            .bind(&new_card.question)
            .bind(&new_card.answer)
            .bind(&new_card.topic)
            .bind(&new_card.tags)
            .bind(new_card.difficulty)
            .bind(new_card.created_at)
            .bind(new_card.updated_at)
            .fetch_one(&pool)
            .await?;

    Ok(new_flash_card)
}

// function which queries the database, given a flash card id, and deletes that entry
pub async fn delete_flashcard(pool: PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
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
    updated_card: &UpdatedFlashCard,
) -> Result<FlashCard, sqlx::Error> {
    let updated_flash_card: FlashCard = sqlx::query_as("UPDATE flashcards SET question = $1, answer = $2, topic = $3, tags = $4, difficulty = $5, updated_at = $6 WHERE id = $7 RETURNING *;")
        .bind(&updated_card.question)
        .bind(&updated_card.answer)
        .bind(&updated_card.topic)
        .bind(&updated_card.tags)
        .bind(updated_card.difficulty)
        .bind(Some(PavexTimestamp::now().to_sqlx()))
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(updated_flash_card)
}

// function which queries the database and returns a random flash card
pub async fn random_flashcard(pool: PgPool) -> Result<Option<FlashCard>, sqlx::Error> {
    let random_card =
        sqlx::query_as::<_, FlashCard>("SELECT * FROM flashcards ORDER BY RANDOM() LIMIT 1;")
            .fetch_optional(&pool)
            .await?;

    Ok(random_card)
}
