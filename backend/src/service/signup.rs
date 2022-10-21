use crate::{utils::hash_password::hash_password, repository::user::UserRepository};

pub struct SignupService {
    user_repository: Box<dyn UserRepository>
}

impl SignupService {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        return Self{ user_repository };
    }

    pub async fn register_user(&self, name: &str, email: &str, password: &str) -> anyhow::Result<()> {
        let password_hash = hash_password(password)?;
        self.user_repository.create(name, email, &password_hash).await?;

        return anyhow::Ok(());
    }
}


