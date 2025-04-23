// app/lib.rs

// module declarations
mod blueprint;
pub mod configuration;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod queries;
pub mod routes;
pub mod telemetry;

// re-exports
pub use blueprint::blueprint;
pub use errors::*;
pub use middleware::*;
pub use models::*;
pub use queries::*;
