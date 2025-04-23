// app/src/routes/health.rs

// dependencies
use pavex::http::StatusCode;

// handler function which responds with a 200 OK and empty body
pub fn check_health() -> StatusCode {
    StatusCode::OK
}
