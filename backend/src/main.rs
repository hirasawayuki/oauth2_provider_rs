use actix_web::{error, HttpServer, HttpResponse, App, web, Error};
use tinytemplate::TinyTemplate;

async fn index(
    template: web::Data<TinyTemplate<'_>>,
) -> Result<HttpResponse, Error> {
    let s = template.render("index.html", &serde_json::Value::Null)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mut tt = TinyTemplate::new();
        tt.add_template("index.html", INDEX).unwrap();
        App::new()
            .app_data(web::Data::new(tt))
            .service(web::resource("/").route(web::get().to(index)))
    }).bind(("0.0.0.0", 8080))?
    .run()
    .await
}

static INDEX: &str = include_str!("../templates/index.html");
