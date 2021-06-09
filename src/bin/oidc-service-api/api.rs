use std::error::Error as StdError;

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use awesome_oidc_service::users::{
    CreateUserError, InMemoryUserRepository, RepositoryError, User, UsersService,
};

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
pub async fn post_users(
    service: web::Data<UsersService<InMemoryUserRepository>>,
    user: web::Json<PostUsersRequest>,
) -> impl Responder {
    tracing::debug!(?user);

    let result = service
        .into_inner()
        .create_user(user.into_inner().into())
        .await;

    match result {
        Ok(()) => HttpResponse::Created(),
        Err(CreateUserError::Repository(RepositoryError::AlreadyExists)) => {
            HttpResponse::Conflict()
        }
        Err(_) => HttpResponse::InternalServerError(),
    }

    // NOTE: a different error handling strategy using downcasting.
    //
    // let err = match result {
    //     Ok(()) => return HttpResponse::Created(),
    //     Err(e) => e,
    // };

    // let err_ref = &err as &(dyn StdError);

    // if let Some(RepositoryError::AlreadyExists) = err_ref.downcast_ref::<RepositoryError>() {
    //     return HttpResponse::Conflict();
    // }

    // // Default error returned
    // HttpResponse::InternalServerError()
}
