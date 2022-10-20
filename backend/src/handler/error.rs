use std::fmt;

#[derive(Debug)]
pub struct HandlerError {
    err: anyhow::Error,
}

impl actix_web::error::ResponseError for HandlerError {}

impl From<anyhow::Error> for HandlerError {
    fn from(err: anyhow::Error) -> HandlerError {
        HandlerError { err }
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}
