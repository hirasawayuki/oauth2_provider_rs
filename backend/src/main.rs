use actix_web::{Responder, HttpServer, HttpResponse, get, App, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Manual Hello World!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    }).bind(("0.0.0.0", 8080))?
    .run()
    .await
}
