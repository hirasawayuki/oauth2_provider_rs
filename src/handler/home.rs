use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use anyhow::Context;
use askama::Template;
use sqlx::MySqlPool;

use crate::repository::user;

use super::error::HandlerError;

#[derive(Template)]
#[template(path="../templates/user/show.html")]
struct UserTemplate{
    name: String,
    email: String,
}

pub async fn index(ident: Identity, connection_pool: web::Data<MySqlPool>) -> Result<HttpResponse, HandlerError> {
    let user_id = ident.id()?;
    let user = user::find_by_id(&user_id, &connection_pool).await?;
    let html = UserTemplate{
        name: user.name,
        email: user.email,
    };

    let res_body = html.render().context("failed to render template")?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(res_body));
}
