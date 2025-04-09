// server/src/tests/integration/create_flashcard.rs

// dependencies
use crate::helpers::TestApi;
use app::models::FlashCard;
use jiff_sqlx::ToSqlx;
use pavex::http::StatusCode;
use pavex::time::Timestamp as PavexTimestamp;
use uuid::Uuid;

#[tokio::test]
async fn delete_flashcard_returns_no_content_when_successful() {
    // Arrange
    let api = TestApi::spawn().await;
    let flash_card = FlashCard {
        id: Uuid::new_v4(),
        question: "test question".to_string(),
        answer: "test answer".to_string(),
        topic: "test topic".to_string(),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        difficulty: 1,
        created_at: PavexTimestamp::now().to_sqlx(),
        updated_at: None,
    };

    let id: Uuid = sqlx::query_scalar("INSERT INTO flashcards (id, question, answer, topic, tags, difficulty, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id;")
        .bind(flash_card.id)
        .bind(flash_card.question)
        .bind(flash_card.answer)
        .bind(flash_card.topic)
        .bind(flash_card.tags)
        .bind(flash_card.difficulty)
        .bind(flash_card.created_at)
        .bind(flash_card.updated_at)
        .fetch_one(&api.api_db_pool)
        .await
        .unwrap();

    // Act
    let response = api.delete_flashcard(id.to_string()).await;

    // Assert
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let deleted = sqlx::query("SELECT * FROM flashcards WHERE id = $1;")
        .bind(id)
        .fetch_optional(&api.api_db_pool)
        .await
        .unwrap();

    assert!(deleted.is_none())
}
