mod api;

use actix_web::{web, App, HttpServer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use awesome_oidc_service::users::{InMemoryUserRepository, User, UsersService};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer().json();
    tracing_subscriber::registry().with(fmt_layer).try_init()?;

    let repository = InMemoryUserRepository::default();
    let service = UsersService::new(repository);

    let app_data = web::Data::new(service);

    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:password@localhost/test").await?;

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(api::health)
            .service(api::post_users)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await?;

    Ok(())
}
