// server/src/tests/integration/update_flashcard.rs

// dependencies
use crate::helpers::TestApi;
use app::models::{FlashCard, NewFlashCard, UpdatedFlashCard};
use app::routes::flashcards::FlashCardResponse;
use pavex::http::StatusCode;
use uuid::Uuid;

#[tokio::test]
async fn update_flashcard_works() {
    // Arrange
    let api = TestApi::spawn().await;
    let new_flash_card = NewFlashCard {
        question: "new test question".to_string(),
        answer: "new test answer".to_string(),
        topic: "new test topic".to_string(),
        tags: vec!["newtag1".to_string(), "newtag2".to_string()],
        difficulty: 1,
    };

    let flash_card = FlashCard::try_from(new_flash_card).unwrap();

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
