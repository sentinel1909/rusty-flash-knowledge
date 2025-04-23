// app/src/middleware.rs

// dependencies
use crate::configuration::AuthConfig;
use crate::errors::ApiError;
use pavex::http::HeaderValue;
use pavex::middleware::Processing;
use pavex::request::RequestHead;
use pavex::response::Response;

// pre-processing middleware function  which tests the validity of the API key, contained in Authorization: Bearer, in the request header
pub async fn validate_api_key(
    auth_config: &AuthConfig,
    request: &RequestHead,
) -> Result<Processing, ApiError> {
    let is_authorized = request
        .headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|value| value == format!("Bearer {}", auth_config.api_key))
        .unwrap_or(false);

    if !is_authorized {
        return Err(ApiError::ApiKeyError);
    }

    Ok(Processing::Continue)
}

// post-processing function to add CORS related headers
pub fn add_cors_headers(response: Response) -> Response {
    let mut response = response;

    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("Content-Type, Authorization"),
    );

    response
}
