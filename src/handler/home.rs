use actix_web::HttpResponse;

use super::error::HandlerError;

pub async fn index() -> Result<HttpResponse, HandlerError> {
    return Ok(HttpResponse::Ok().content_type("text/html").body("login successful!"));
}
