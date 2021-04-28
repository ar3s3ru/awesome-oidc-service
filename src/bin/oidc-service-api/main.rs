mod api;
use actix_web::{get, App, HttpServer, Responder};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer().json();
    tracing_subscriber::registry().with(fmt_layer).try_init();

    HttpServer::new(|| App::new().service(api::health))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
