use crate::helpers::TestApi;
use pavex::http::StatusCode;
use reqwest::header::HOST;

#[tokio::test]
async fn health_returns_200_ok() {
    let api = TestApi::spawn().await;

    let response = api
        .api_client
        .get(format!("{}/flashcards/health", api.api_address))
        .header(HOST, "rusty-flash-knowledge.net")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
