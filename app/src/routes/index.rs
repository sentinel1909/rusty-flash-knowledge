// app/src/routes/index.rs

// dependencies
use pavex::{
    cookie::{ResponseCookie, ResponseCookies},
    http::StatusCode,
    response::{Response, body::Html},
    time::Zoned,
};
use pavex_tera_template::{Context, TemplateEngine, TemplateError};

// error handler for the index endpoint
pub fn template_error2response(e: &TemplateError) -> StatusCode {
    match e {
        TemplateError::Tera(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// handler which returns the index page template
pub fn get(response_cookies: &mut ResponseCookies, template: &TemplateEngine) -> Result<Response, TemplateError> {
    let now = Zoned::now().to_string();
    let cookie = ResponseCookie::new("last_visited", now)
        .set_path("/");

    response_cookies.insert(cookie);
    let context = Context::new();

    let body: Html = template.render("index.html", &context)?.into();

    let response = Response::ok().set_typed_body(body);

    Ok(response)
}
