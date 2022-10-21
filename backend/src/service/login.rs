use crate::repository::user::UserRepository;

pub struct LoginService {
    user_repository: Box<dyn UserRepository>
}

impl LoginService {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        return Self{ user_repository };
    }

    pub async fn verify_credentials(&self, email: &str, password: &str) -> anyhow::Result<bool> {
        let user = self.user_repository.find_by_email(email).await?;

        return user.verify_password(password);
    }
}


