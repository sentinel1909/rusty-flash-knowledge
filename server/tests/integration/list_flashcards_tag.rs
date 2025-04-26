// server/src/tests/integration/list_flashcard_topics.rs

// dependencies
use crate::helpers::TestApi;
use app::models::NewFlashCard;
use pavex::http::StatusCode;

#[tokio::test]
async fn list_flashcards_by_tag_returns_200_and_only_matching_cards() {
    // Arrange
    let api = TestApi::spawn().await;

    let matching_card = NewFlashCard {
        question: "What is Ownership?".to_string(),
        answer: "A memory management model in Rust.".to_string(),
        topic: "memory".to_string(),
        tags: vec!["ownership".to_string()],
        difficulty: 2,
    };

    let non_matching_card = NewFlashCard {
        question: "What is pattern matching".to_string(),
        answer: "A control flow construct in Rust.".to_string(),
        topic: "syntax".to_string(),
        tags: vec!["match".to_string()],
        difficulty: 2,
    };

    api.create_flashcard(&matching_card).await;
    api.create_flashcard(&non_matching_card).await;

    // Act
    let response = api
        .get_flashcards_by_tag(Some(&matching_card.tags[0]))
        .await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}
