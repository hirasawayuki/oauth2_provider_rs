use std::fmt;

use actix_web::{ResponseError, HttpResponse};
use askama::Template;
use serde::Serialize;

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

#[derive(Debug)]
pub enum JsonError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    InternalServerError,
}

#[derive(Serialize)]
pub struct ErrorBody {
    status: String,
    message: String,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for JsonError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::BadRequest(message) => {
                HttpResponse::BadRequest().json(ErrorBody{
                    status: String::from("400"),
                    message: String::from("invalid request: ") + &message,
                })
            },
            Self::Unauthorized(message) => {
                HttpResponse::BadRequest().json(ErrorBody{
                    status: String::from("401"),
                    message: String::from("unauthorized: ") + &message,
                })
            },
            Self::NotFound(message) => {
                HttpResponse::BadRequest().json(ErrorBody{
                    status: String::from("404"),
                    message: String::from("not found: ") + &message,
                })
            }
            Self::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorBody{
                    status: String::from("500"),
                    message: String::from("internal server error: "),
                })
            }
        }
    }
}
