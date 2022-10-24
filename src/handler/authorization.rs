use actix_web::{HttpRequest, HttpResponse, web};

use super::error::HandlerError;

pub struct AuthorizeQuery {
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    state: String,
}

pub async fn authorize(query: web::Query<AuthorizeQuery>) -> Result<HttpResponse, HandlerError> {
    // find_by_client_id
    // check redirect_uri
    // check scope == "all"
    // generate code
    // redirect_uri?state={state}&code={code}
    return Ok(HttpResponse::Ok().content_type("text/html").body(""));
}

// pub async fn get_token() {
// check code
// generate access_token and refresh_token
// response access_token and refresh_tokena and expired_at
// }
