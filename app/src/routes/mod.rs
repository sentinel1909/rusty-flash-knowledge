// app/src/routes/mod.rs

// modules into scope
pub mod flashcards;
pub mod ping;

// dependencies
use pavex::blueprint::{
    Blueprint,
    router::{DELETE, GET, POST, PUT},
};
use pavex::f;

// public routes, no API key required
fn public_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.route(
        GET,
        "/flashcards/random",
        f!(self::flashcards::random_flashcard_handler),
    )
    .error_handler(f!(crate::errors::api_error2response));

    bp
}

// protected routes, require an API key to access
fn api_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.pre_process(f!(crate::middleware::validate_api_key))
        .error_handler(f!(crate::errors::api_error2response));
    bp.route(GET, "/ping", f!(self::ping::get));
    bp.route(
        GET,
        "/flashcards",
        f!(self::flashcards::list_flashcards_handler),
    )
    .error_handler(f!(crate::errors::api_error2response));
    bp.route(
        GET,
        "/flashcards/{id}",
        f!(self::flashcards::list_flashcard_handler),
    )
    .error_handler(f!(crate::errors::api_error2response));
    bp.route(
        POST,
        "/flashcards",
        f!(self::flashcards::create_flashcard_handler),
    )
    .error_handler(f!(crate::errors::api_error2response));
    bp.route(
        PUT,
        "/flashcards/{id}",
        f!(self::flashcards::update_flashcard_handler),
    )
    .error_handler(f!(crate::errors::api_error2response));
    bp.route(
        DELETE,
        "/flashcards/{id}",
        f!(self::flashcards::delete_flashcard_handler),
    )
    .error_handler(f!(crate::errors::api_error2response));
    bp
}

pub fn register(bp: &mut Blueprint) {
    bp.domain("rusty-flash-knowledge.net").nest(public_bp());
    bp.domain("api.rusty-flash-knowledge.net")
        .prefix("/v1")
        .nest(api_bp());
}
