use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use awesome_oidc_service::users::{InMemoryUserRepository, User, UsersService};

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

impl From<PostUsersRequest> for User {
    fn from(input: PostUsersRequest) -> Self {
        Self {
            email: input.email,
            first_name: input.first_name,
            last_name: input.last_name,
        }
    }
}

#[post("/users")]
pub async fn post_users(user: web::Json<PostUsersRequest>) -> impl Responder {
    tracing::debug!(?user);
    let repository = InMemoryUserRepository::default();
    let mut service = UsersService::new(repository);
    service.create_user(user.into_inner().into()).await.unwrap();
    HttpResponse::Created()
}
