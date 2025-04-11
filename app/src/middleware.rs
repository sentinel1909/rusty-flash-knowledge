// app/src/middleware.rs

// dependencies
use crate::configuration::AuthConfig;
use crate::errors::ApiError;
use pavex::middleware::Processing;
use pavex::request::RequestHead;

// function which tests the validity of the API key, contained in Authorization: Bearer, in the request header
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
