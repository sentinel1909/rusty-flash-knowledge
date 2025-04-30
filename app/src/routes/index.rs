// app/src/routes/index.rs

// dependencies
use pavex::{
    http::StatusCode,
    response::{Response, body::Html},
};
use pavex_template::{Context, TemplateEngine, TemplateError};

// error handler for the index endpoint
pub fn template_error2response(e: &TemplateError) -> StatusCode {
    match e {
        TemplateError::Tera(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// handler which returns the index page template
pub fn get(template: &TemplateEngine) -> Result<Response, TemplateError> {
    let context = Context::new();

    let body: Html = template.render("index.html", &context)?.into();

    let response = Response::ok().set_typed_body(body);

    Ok(response)
}
