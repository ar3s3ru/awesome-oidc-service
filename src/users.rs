use anyhow::Result;

struct User {
    id: String,
    email: String,
    first_name: String,
    last_name: String,
}

trait UsersRepository {
    fn create(mut self, user: &User) -> Result<()>;
}

struct InMemoryUserRepository {


}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self
    }
}

impl UsersRepository for InMemoryUserRepository {
    fn create(mut self, user: &User) -> Result<()> {
        todo!()
    }
}