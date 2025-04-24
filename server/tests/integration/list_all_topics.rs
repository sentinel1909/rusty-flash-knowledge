// server/tests/integration/list_all_topics.rs

// dependencies
use crate::helpers::TestApi;
use app::models::NewFlashCard;
use pavex::http::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TopicsResponse {
    msg: String,
    content: Vec<String>,
}


#[tokio::test]
async fn list_all_topics_returns_200_ok() {
    // Arrange
    let api = TestApi::spawn().await;

    let card_1 = NewFlashCard {
        question: "What is ownership?".to_string(),
        answer: "A memory management model in Rust.".to_string(),
        topic: "memory".to_string(),
        tags: vec!["ownership".to_string()],
        difficulty: 2,
    };

    let card_2 = NewFlashCard {
        question: "What is borrowing?".to_string(),
        answer: "A way to reference data without taking ownership.".to_string(),
        topic: "memory".to_string(), // ✅ Overlapping topic with card_1
        tags: vec!["borrowing".to_string()],
        difficulty: 3,
    };

    let card_3 = NewFlashCard {
        question: "What is pattern matching?".to_string(),
        answer: "A control flow mechanism in Rust.".to_string(),
        topic: "syntax".to_string(),
        tags: vec!["match".to_string()],
        difficulty: 1,
    };

    api.create_flashcard(&card_1).await;
    api.create_flashcard(&card_2).await;
    api.create_flashcard(&card_3).await;

    // Act
    let response = api.get_all_topics().await;
    println!("{:?}", response);

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let TopicsResponse { msg, content } = response.json().await.unwrap();

assert_eq!(msg, "success");
assert!(content.contains(&"memory".to_string()));
assert!(content.contains(&"syntax".to_string()));
assert_eq!(content.len(), 2);

}
