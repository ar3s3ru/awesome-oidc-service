use sqlx::postgres::PgPool;
use crate::users::{UsersRepository, User, RepositoryError};
use async_trait::async_trait;

pub struct PostgresUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UsersRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<(), RepositoryError>{
        unimplemented!()
    }
    async fn get(&self, email: &str) -> Result<User, RepositoryError>{
        unimplemented!()
    }
}
