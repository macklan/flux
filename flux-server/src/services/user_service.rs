use crate::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, String>;
    async fn find_user_by_username(&self, name: &str) -> Option<User>;
    // Add more as needed
}

pub struct MockUserService;

#[async_trait]
impl UserService for MockUserService {
    async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, String> {
        Ok(User {
            id: 1,
            name,
            email,
            password_hash: User::hash_password(&password),
        })
    }

    async fn find_user_by_username(&self, name: &str) -> Option<User> {
        if name == "exists" {
            Some(User {
                id: 1,
                name: name.to_string(),
                email: "exists@example.com".to_string(),
                password_hash: User::hash_password("password"),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user_success() {
        let service = MockUserService;
        let user = service
            .create_user(
                "Alice".to_string(),
                "alice@example.com".to_string(),
                "secret".to_string(),
            )
            .await
            .unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
        assert!(user.verify_password("secret"));
    }

    #[tokio::test]
    async fn test_find_user_by_username_found() {
        let service = MockUserService;
        let user = service.find_user_by_username("exists").await;
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.name, "exists");
        assert!(user.verify_password("password"));
    }

    #[tokio::test]
    async fn test_find_user_by_username_not_found() {
        let service = MockUserService;
        let user = service.find_user_by_username("missing").await;
        assert!(user.is_none());
    }
}
