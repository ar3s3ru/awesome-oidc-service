use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/health")]
pub async fn health() -> impl Responder {
    "I am ready"
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostUsersRequest {
    email: String,
    first_name: String,
    last_name: String,
}

#[post("/users")]
pub async fn post_users(user: web::Json<PostUsersRequest>) -> impl Responder {
    tracing::debug!(?user);
    HttpResponse::NotImplemented()
}
