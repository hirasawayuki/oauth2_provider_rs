use actix_web::HttpResponse;
use askama::Template;

use super::error::HtmlError;

#[derive(Template)]
#[template(path="../templates/index.html")]
struct IndexTemplate{}

pub async fn index() -> Result<HttpResponse, HtmlError> {
    let html = IndexTemplate{};
    match html.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(_) => Err(HtmlError::Status5XX),
    }
}
