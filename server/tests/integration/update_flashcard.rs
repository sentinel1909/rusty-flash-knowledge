// server/src/tests/integration/update_flashcard.rs

// dependencies
use crate::helpers::TestApi;
use app::models::{FlashCard, UpdatedFlashCard};
use app::routes::flashcards::FlashCardResponse;
use jiff_sqlx::ToSqlx;
use pavex::http::StatusCode;
use pavex::time::Timestamp as PavexTimestamp;
use uuid::Uuid;

#[tokio::test]
async fn update_flashcard_returns_200_and_updated_flash_card() {
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

    let updated_flash_card = UpdatedFlashCard {
        question: Some("updated test question".to_string()),
        answer: Some("updated test answer".to_string()),
        topic: Some("updated topic".to_string()),
        tags: Some(vec![
            "differenttag1".to_string(),
            "differenttag2".to_string(),
        ]),
        difficulty: Some(2),
    };

    // Act
    let response = api
        .update_flashcard(&updated_flash_card, id.to_string())
        .await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.json::<FlashCardResponse>().await.unwrap();
    assert_eq!(body.id, id);
    assert_eq!(body.question, "updated test question");
    assert_eq!(body.answer, "updated test answer");
}
