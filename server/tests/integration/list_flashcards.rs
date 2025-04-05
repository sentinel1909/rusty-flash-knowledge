use crate::helpers::TestApi;
use app::models::FlashCard;
use chrono::Utc;
use pavex::http::StatusCode;
use uuid::Uuid;

#[tokio::test]
async fn list_flashcards_works() {
    // Arrange
    let flash_card = FlashCard {
        id: Uuid::new_v4(),
        question: "Test Question".to_string(),
        answer: "test answer".to_string(),
        topic: Some("lifetimes".to_string()),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        difficulty: Some(4),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let api = TestApi::spawn().await;

    sqlx::query("INSERT INTO flashcards (id, question, answer, topic, tags, difficulty, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(flash_card.id)
        .bind(&flash_card.question)
        .bind(&flash_card.answer)
        .bind(&flash_card.topic)
        .bind(&flash_card.tags)
        .bind(flash_card.difficulty)
        .bind(flash_card.created_at)
        .bind(flash_card.updated_at)
        .execute(&api.db_pool)
        .await
        .unwrap();

    // Act
    let response = api.get_flashcards().await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let response_body: Vec<FlashCard> = response.json().await.unwrap();
    let expected_body = vec![flash_card];

    assert_eq!(response_body, expected_body);

    // Clean up
    sqlx::query("TRUNCATE flashcards")
        .execute(&api.db_pool)
        .await
        .unwrap();
}
