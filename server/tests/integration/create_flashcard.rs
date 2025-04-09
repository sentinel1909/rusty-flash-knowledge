// server/src/tests/integration/create_flashcard.rs

// dependencies
use crate::helpers::TestApi;
use app::models::NewFlashCard;
use app::routes::flashcards::FlashCardResponse;
use pavex::http::StatusCode;

#[tokio::test]
async fn create_flashcard_returns_200_for_valid_data() {
    // Arrange
    let api = TestApi::spawn().await;
    let new_flash_card = NewFlashCard {
        question: "new test question".to_string(),
        answer: "new test answer".to_string(),
        topic: "new test topic".to_string(),
        tags: vec!["newtag1".to_string(), "newtag2".to_string()],
        difficulty: 1,
    };

    // Act
    let response = api.create_flashcard(&new_flash_card).await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.json::<FlashCardResponse>().await.unwrap();

    assert_eq!(body.question, new_flash_card.question);
    assert_eq!(body.answer, new_flash_card.answer);
    assert!(!body.id.to_string().is_empty(), "ID should be generated");
}

#[tokio::test]
async fn create_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let api = TestApi::spawn().await;
    let new_flash_card = NewFlashCard {
        question: "new test question".to_string(),
        answer: "new test answer".to_string(),
        topic: "new test topic".to_string(),
        tags: vec!["newtag1".to_string(), "newtag2".to_string()],
        difficulty: 1,
    };

    // Sabotage the database
    sqlx::query("ALTER TABLE flashcards DROP COLUMN question;")
        .execute(&api.api_db_pool)
        .await
        .unwrap();

    // Act
    let response = api.create_flashcard(&new_flash_card).await;

    // Assert
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
