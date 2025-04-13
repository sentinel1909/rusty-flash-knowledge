// server/tests/integration/random_flashcard.rs

// dependencies
use crate::helpers::TestApi;
use app::models::NewFlashCard;
use pavex::http::StatusCode;
use reqwest::header::HOST;

#[tokio::test]
async fn get_random_flashcard_returns_200() {
    let api = TestApi::spawn().await;

    // Seed one flashcard
    let card = NewFlashCard {
        question: "Random question?".to_string(),
        answer: "Random answer!".to_string(),
        topic: "random".to_string(),
        tags: vec!["rand".to_string()],
        difficulty: 1,
    };
    api.create_flashcard(&card).await;

    let response = api
        .api_client
        .get(format!("{}/flashcards/random", api.api_address))
        .header(HOST, "rusty-flash-knowledge.net")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.text().await.unwrap();
    assert!(body.contains("Random question"));
}

#[tokio::test]
async fn get_random_flashcard_returns_404_when_no_flashcards_exist() {
    // Arrange
    let api = TestApi::spawn().await;

    // Act
    let response = api
        .api_client
        .get(format!("{}/flashcards/random", api.api_address))
        .header(HOST, "rusty-flash-knowledge.net")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
