use std::collections::HashMap;

use async_trait::async_trait;

#[derive(Debug)]
struct User {
    email: String,
    first_name: String,
    last_name: String,
}

#[async_trait]
trait UsersRepository {
    async fn create(&mut self, user: User) -> anyhow::Result<()>;
}

#[derive(Debug, Default)]
struct InMemoryUserRepository {
    inner: HashMap<String, User>,
}

#[async_trait]
impl UsersRepository for InMemoryUserRepository {
    async fn create(&mut self, user: User) -> anyhow::Result<()> {
        if self.inner.get(&user.email).is_some() {
            return Err(anyhow::Error::msg("user already exists"));
        }

        self.inner.insert(user.email.clone(), user);

        Ok(())
    }
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
    pub async fn create_user(&mut self, user: User) -> anyhow::Result<()> {
        unimplemented!()
    }
}
