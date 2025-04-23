// app/src/routes/preflight.rs

// dependencies
use pavex::http::HeaderValue;
use pavex::response::Response;

// handler function to add CORS related headers for public routes
pub fn preflight_handler() -> Response {
    let mut response = Response::no_content();

    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, OPTIONS"),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("Content-Type, Authorization"),
    );

    response
}
