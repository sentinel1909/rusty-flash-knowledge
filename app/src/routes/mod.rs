pub mod flashcards;
pub mod ping;

use pavex::blueprint::{
    Blueprint,
    router::{DELETE, GET, POST, PUT},
};
use pavex::f;

fn api_bp() -> Blueprint {
    let mut bp = Blueprint::new();
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
    bp.domain("api.rusty-flash-knowledge.net")
        .prefix("/v1")
        .nest(api_bp());
}
