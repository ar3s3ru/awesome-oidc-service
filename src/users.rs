use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
struct User {
    email: String,
    first_name: String,
    last_name: String,
}

#[derive(Debug, thiserror::Error)]
enum RepositoryError {
    #[error("user not found")]
    NotFound,

    #[error("user already exists")]
    AlreadyExists,

    #[error("repository failed: {0}")]
    Other(#[from] anyhow::Error),
}

#[async_trait]
trait UsersRepository {
    async fn create(&mut self, user: User) -> Result<(), RepositoryError>;
    async fn get(&self, email: &str) -> Result<User, RepositoryError>;
}

#[derive(Debug, Clone, Default)]
struct InMemoryUserRepository {
    inner: Arc<RwLock<HashMap<String, User>>>,
}

#[async_trait]
impl UsersRepository for InMemoryUserRepository {
    async fn create(&mut self, user: User) -> Result<(), RepositoryError> {
        if self.inner.read().unwrap().get(&user.email).is_some() {
            return Err(RepositoryError::AlreadyExists);
        }

        self.inner.write().unwrap().insert(user.email.clone(), user);

        Ok(())
    }

    async fn get(&self, email: &str) -> Result<User, RepositoryError> {
        self.inner
            .read()
            .unwrap()
            .get(email)
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound)
    }
}

#[derive(Debug, thiserror::Error)]
enum CreateUserError {
    #[error("user email cannot be empty")]
    EmptyEmail,

    #[error("user first name cannot be empty")]
    EmptyFirstName,

    #[error("user last name cannot be empty")]
    EmptyLastName,

    #[error("failed to save user to repository: {0}")]
    Repository(#[from] RepositoryError),
}

#[derive(Debug)]
struct UsersService<R>
where
    R: UsersRepository,
{
    repository: R,
}

impl<R> UsersService<R>
where
    R: UsersRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_user(&mut self, user: User) -> Result<(), CreateUserError> {
        if user.email.is_empty() {
            return Err(CreateUserError::EmptyEmail);
        }

        if user.first_name.is_empty() {
            return Err(CreateUserError::EmptyFirstName);
        }

        if user.last_name.is_empty() {
            return Err(CreateUserError::EmptyLastName);
        }

        self.repository
            .create(user)
            .await
            .map_err(CreateUserError::Repository)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_user() {
        let repository = InMemoryUserRepository::default();
        let mut user_service = UsersService::new(repository.clone());

        let user = User {
            email: "john@doe.com".to_owned(),
            first_name: "John".to_owned(),
            last_name: "Doe".to_owned(),
        };

        assert!(user_service.create_user(user.clone()).await.is_ok());
        assert_eq!(user, repository.get(&user.email).await.unwrap())
    }

    #[tokio::test]
    async fn cannot_create_users_with_same_email() {
        let repository = InMemoryUserRepository::default();
        let mut user_service = UsersService::new(repository.clone());

        let user = User {
            email: "john@doe.com".to_owned(),
            first_name: "John".to_owned(),
            last_name: "Doe".to_owned(),
        };

        assert!(user_service.create_user(user.clone()).await.is_ok());
        assert!(user_service.create_user(user.clone()).await.is_err());
    }
}
