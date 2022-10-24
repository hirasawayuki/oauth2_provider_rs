use actix_web::HttpResponse;
use anyhow::Context;
use askama::Template;

use super::error::HandlerError;

#[derive(Template)]
#[template(path="../templates/index.html")]
struct IndexTemplate{}

pub async fn index() -> Result<HttpResponse, HandlerError> {
    let html = IndexTemplate{};
    let res_body = html.render().context("failed to render template")?;

    Ok(HttpResponse::Ok().content_type("text/html").body(res_body))
}
