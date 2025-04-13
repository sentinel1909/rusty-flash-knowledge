// app/src/routes/health.rs

use pavex::http::StatusCode;

// function which responds with a 200 OK and empty body
pub fn check_health() -> StatusCode {
    StatusCode::OK
}
