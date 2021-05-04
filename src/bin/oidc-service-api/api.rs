use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/health")]
async fn health() -> impl Responder {
    "I am ready"
}

#[derive(Deserialize, Debug)]
struct PostUsersRequest {
    email: String,
    firstName: String,
    lastName: String,
}

#[post("/users")]
async fn post_users(user: web::Json<PostUsersRequest>) -> impl Responder {
    tracing::debug!(?user);
    HttpResponse::NotImplemented()
}
