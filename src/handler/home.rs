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
    let user_id = match ident.id() {
        Ok(user_id) => user_id,
        Err(_) => { return Err(HtmlError::Status5XX); }
    };

    let html = match user::find_by_id(&user_id, &connection_pool).await {
        Ok(user) =>  {
            UserTemplate {
                name: user.name,
                email: user.email,
            }
        },
        Err(_) => { return Err(HtmlError::Status4XX); }
    };

    match html.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(_) => { return Err(HtmlError::Status5XX); }
    }
}
