use actix_web::{error, HttpServer, HttpResponse, App, web, Error};
use serde::{Serialize, Deserialize};
use tinytemplate::TinyTemplate;

async fn index(
    template: web::Data<TinyTemplate<'_>>,
) -> Result<HttpResponse, Error> {
    let s = template.render("index.html", &serde_json::Value::Null)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn signup(
    template: web::Data<TinyTemplate<'_>>,
) -> Result<HttpResponse, Error> {
    let s = template.render("signup.html", &serde_json::Value::Null)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Serialize, Deserialize)]
pub struct LoginParams {
    email: String,
    password: String,
}

async fn post_login(params: web::Form<LoginParams>) -> Result<HttpResponse, Error> {
    println!("hogheogehogehogeo");
    println!("email: {} password: {}", params.email, params.password);
    Ok(HttpResponse::Ok().content_type("text/html").body(""))
}

async fn login(
    template: web::Data<TinyTemplate<'_>>,
) -> Result<HttpResponse, Error> {
    println!("hogehoge");
    let s = template.render("login.html", &serde_json::Value::Null)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mut tt = TinyTemplate::new();
        tt.add_template("index.html", INDEX).unwrap();
        tt.add_template("signup.html", SIGNUP).unwrap();
        tt.add_template("login.html", LOGIN).unwrap();

        App::new()
            .app_data(web::Data::new(tt))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/signup").route(web::get().to(signup)))
            .service(web::resource("/login").route(web::get().to(login)))
            .service(web::resource("/post_login").route(web::post().to(post_login)))
    }).bind(("0.0.0.0", 8080))?
    .run()
    .await
}

static INDEX: &str = include_str!("../templates/index.html");
static SIGNUP: &str = include_str!("../templates/signup.html");
static LOGIN: &str = include_str!("../templates/login.html");
