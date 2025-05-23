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

    assert_eq!(body.content.question, new_flash_card.question);
    assert_eq!(body.content.answer, new_flash_card.answer);
    assert!(
        !body.content.id.to_string().is_empty(),
        "ID should be generated"
    );
}

#[tokio::test]
async fn create_flashcard_returns_400_for_invalid_data() {
    // Arrange
    let api = TestApi::spawn().await;
    let new_flash_card = NewFlashCard {
        question: "".to_string(),
        answer: "".to_string(),
        topic: "".to_string(),
        tags: vec![],
        difficulty: 99,
    };

    // Act
    let response = api.create_flashcard(&new_flash_card).await;

    // Assert
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_returns_500_if_there_is_a_fatal_database_error() {
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

#[tokio::test]
async fn create_flashcard_returns_409_for_duplicate_question() {
    let api = TestApi::spawn().await;
    let flashcard = NewFlashCard {
        question: "What is Rust?".to_string(),
        answer: "A system programming language.".to_string(),
        topic: "intro".to_string(),
        tags: vec!["basics".to_string()],
        difficulty: 1,
    };

    // First insertion should succeed
    let first = api.create_flashcard(&flashcard).await;
    assert_eq!(first.status(), StatusCode::OK);

    // Second insertion with same question should fail
    let second = api.create_flashcard(&flashcard).await;
    assert_eq!(second.status(), StatusCode::CONFLICT);
}
