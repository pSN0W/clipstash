pub mod ctx;
pub mod renderer;
pub mod form;

// rocket::Responder let us return an error code with the error
// 500 is a server error while 404 is client (optimally no server error)
// We can't use thiserror::From here because it is uncompaitible with rocket
#[derive(rocket::Responder)]
pub enum PageError {
    #[response(status = 500)]
    Serialization(String),

    #[response(status = 500)]
    Render(String),

    #[response(status = 404)]
    NotFound(String),

    #[response(status = 500)]
    Internal(String),
}

impl From<handlebars::RenderError> for PageError{
    fn from(err:handlebars::RenderError) -> Self {
        Self::Render(format!("{}",err))
    }
}

impl From<serde_json::Error> for PageError{
    fn from(err:serde_json::Error) -> Self {
        Self::Serialization(format!("{}",err))
    }
}