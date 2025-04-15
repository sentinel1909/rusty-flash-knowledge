// server/src/tests/integration/list_flashcard_topics.rs

// dependencies
use crate::helpers::TestApi;
use app::models::NewFlashCard;
use pavex::http::StatusCode;

#[tokio::test]
async fn list_flashcards_by_topic_returns_200_and_only_matching_cards() {
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
        .get_flashcards_by_topic(Some(&matching_card.topic))
        .await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await.unwrap();
    assert!(body.contains("memory"));
    assert!(!body.contains("pattern matching"));
}

#[tokio::test]
async fn list_flashcards_returns_all_when_no_topic_given() {
    // Arrange
    let api = TestApi::spawn().await;

    let new_card_1 = NewFlashCard {
        question: "What is pattern matching?".to_string(),
        answer: "A control flow mechanism in Rust.".to_string(),
        topic: "syntax".to_string(),
        tags: vec!["match".to_string()],
        difficulty: 2,
    };

    let new_card_2 = NewFlashCard {
        question: "What is the ownership model?".to_string(),
        answer: "A set of rules that governs how Rust manages memory.".to_string(),
        topic: "memory".to_string(),
        tags: vec!["ownership".to_string(), "memory".to_string()],
        difficulty: 3,
    };

    api.create_flashcard(&new_card_1).await;

    api.create_flashcard(&new_card_2).await;

    // Act
    let response = api.get_flashcards_by_topic(None).await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await.unwrap();

    assert!(body.contains("syntax"));
    assert!(body.contains("memory"));
}
