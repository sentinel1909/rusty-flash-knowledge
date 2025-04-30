//app/src/routes/static_file.rs

// dependencies
use pavex::http::{HeaderValue, StatusCode};
use pavex::request::RequestHead;
use pavex::response::Response;
use pavex::response::body::{
    TypedBody,
    raw::{Bytes, Full},
};
use pavex_static_files::{ServeError, StaticFile, StaticServer};

// error handler for the static server endpoint
pub fn static_error2response(e: &ServeError) -> StatusCode {
    match e {
        ServeError::NotFound => StatusCode::NOT_FOUND,
        ServeError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// the StaticServer type from `pavex_static_files` returns a `StaticFile` type, embodying the asset
// to be served. To deal with the orphan rule around foreign types, we do a "NewType" pattern on it,
// this enables implementing the `TypedBody` trait from Pavex.
struct ServedStaticFile(StaticFile);

// implement the `TypedBody` trait for our ServedStaticFile type
impl TypedBody for ServedStaticFile {
    type Body = Full<Bytes>;

    fn content_type(&self) -> HeaderValue {
        HeaderValue::from_str(&self.0.mime_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"))
    }

    fn body(self) -> Self::Body {
        Full::new(self.0.body.into())
    }
}

// handler function which responds with a 200 OK and the requested static file
pub fn get(
    static_server: &StaticServer,
    request_head: &RequestHead,
) -> Result<Response, ServeError> {
    let request_path = request_head.target.path();

    let file = static_server.read_file(request_path)?;

    Ok(Response::ok().set_typed_body(ServedStaticFile(file)))
}
