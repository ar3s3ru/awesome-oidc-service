use crate::users::{RepositoryError, User, UsersRepository};
use async_trait::async_trait;
use sqlx::postgres::PgPool;

// Schema for PostgresUserRepository

pub struct PostgresUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UsersRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<(), RepositoryError> {
        unimplemented!()
    }
    async fn get(&self, email: &str) -> Result<User, RepositoryError> {
        unimplemented!()
    }
}
