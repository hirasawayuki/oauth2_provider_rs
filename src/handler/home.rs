use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use askama::Template;
use sqlx::MySqlPool;

use crate::repository::user;

use super::error::HtmlError;

#[derive(Template)]
#[template(path="../templates/user/show.html")]
struct UserTemplate{
    name: String,
    email: String,
}

pub async fn index(ident: Identity, connection_pool: web::Data<MySqlPool>) -> Result<HttpResponse, HtmlError> {
    if let Ok(user_id) = ident.id() {
        match user::find_by_id(&user_id, &connection_pool).await {
            Ok(user) =>  {
                let html = UserTemplate {
                    name: user.name,
                    email: user.email,
                };

                if let Ok(body) = html.render() {
                    return Ok(HttpResponse::Ok().content_type("text/html").body(body));
                }
            },
            _ => {}
        }
    }
    return Err(HtmlError::Status4XX);
}
