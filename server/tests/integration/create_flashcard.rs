// server/src/tests/integration/create_flashcard.rs

// dependencies
use crate::helpers::TestApi;
use app::models::NewFlashCard;
use app::routes::flashcards::FlashCardResponse;
use pavex::http::StatusCode;

#[tokio::test]
async fn create_flashcard_works() {
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
