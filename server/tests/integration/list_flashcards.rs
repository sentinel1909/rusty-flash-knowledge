use crate::helpers::TestApi;
use app::models::FlashCard;
use pavex::http::StatusCode;
use serde_json::to_string_pretty;

#[tokio::test]
async fn list_flashcards_works() {
    let api = TestApi::spawn().await;
    let cards: Vec<FlashCard> = Vec::new();

    let response = api.get_flashcards().await;

    assert_eq!(response.status(), StatusCode::OK);

    let expected_body = to_string_pretty(&cards).unwrap();

    let response_body = response.text().await.unwrap();

    assert_eq!(response_body, expected_body);
}
