// server/tests/integration/test_middleware.rs

use crate::helpers::TestApi;
use pavex::http::StatusCode;

#[tokio::test]
async fn returns_401_without_api_key() {
    let api = TestApi::spawn().await;

    let response = api.get_no_api_key().await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn returns_401_with_invalid_api_key() {
    let api = TestApi::spawn().await;

    let response = api.get_invalid_api_key().await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
