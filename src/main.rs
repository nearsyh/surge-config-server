use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Responder, Result};

#[get("/health")]
async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(true))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
