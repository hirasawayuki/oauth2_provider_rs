use std::fmt;

use actix_web::{ResponseError, HttpResponse};
use askama::Template;

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

#[derive(Template)]
#[template(path="../templates/error/4xx.html")]
struct BadRequestTemplate{}

#[derive(Template)]
#[template(path="../templates/error/5xx.html")]
struct InternalServerErrorTemplate{}

#[derive(Debug)]
pub enum HtmlError {
    Status4XX,
    Status5XX,
}

impl fmt::Display for HtmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for HtmlError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Status4XX => {
                let html = BadRequestTemplate{};
                let res_body = html.render().unwrap();
                HttpResponse::BadRequest().content_type("text/html").body(res_body)
            },
            Self::Status5XX => {
                let html = InternalServerErrorTemplate{};
                let res_body = html.render().unwrap();
                HttpResponse::BadRequest().content_type("text/html").body(res_body)
            }
        }
    }
}

pub enum JsonError {
    BadRequest,
    Unauthorized,
    Forbidden,
    InternalServerError,
}
