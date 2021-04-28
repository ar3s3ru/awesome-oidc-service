mod api;
use actix_web::{get, App, HttpServer, Responder};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api::health))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
