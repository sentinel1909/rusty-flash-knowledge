// app/src/routes/mod.rs

// modules into scope
pub mod flashcards;
pub mod health;
pub mod index;
pub mod preflight;
pub mod static_server;

// dependencies
use pavex::blueprint::{
    Blueprint,
    router::{DELETE, GET, OPTIONS, POST, PUT},
};
use pavex::f;

// protected routes, require an API key to access
fn api_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.pre_process(f!(crate::middleware::validate_api_key))
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

// public routes, no API key required
fn public_api_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.post_process(f!(crate::middleware::add_cors_headers));
    bp.route(GET, "/flashcards/health", f!(self::health::check_health));
    bp.route(
        GET,
        "/flashcards/random",
        f!(self::flashcards::random_flashcard_handler),
    )
    .error_handler(f!(crate::errors::api_error2response));
    bp.route(
        GET,
        "/flashcards/tags",
        f!(self::flashcards::list_flashcard_tags_handler),
    )
    .error_handler(f!(crate::api_error2response));
    bp.route(
        GET,
        "/flashcards/topics",
        f!(self::flashcards::list_flashcard_topics_handler),
    )
    .error_handler(f!(crate::api_error2response));
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
        OPTIONS,
        "/flashcards",
        f!(self::preflight::preflight_handler),
    );
    bp.route(
        OPTIONS,
        "/flashcards/{id}",
        f!(self::preflight::preflight_handler),
    );
    bp.route(
        OPTIONS,
        "/flashcards/random",
        f!(self::preflight::preflight_handler),
    );
    bp
}

// web asset routes
fn web_bp() -> Blueprint {
    let mut bp = Blueprint::new();
     bp.route(GET, "/", f!(self::index::get))
        .error_handler(f!(crate::routes::index::template_error2response));
    bp.route(GET, "/static/{filename}", f!(self::static_server::get))
        .error_handler(f!(crate::routes::static_server::static_error2response));
    bp
}

// combine the public and private routes and register them
pub fn register(bp: &mut Blueprint) {
    bp.domain("rusty-flash-knowledge.net").nest(public_api_bp());
    bp.domain("api.rusty-flash-knowledge.net")
        .prefix("/v1")
        .nest(api_bp());
    bp.domain("app.rusty-flash-knowledge.net").nest(web_bp());
}
