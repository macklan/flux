use crate::schema::builder::{ColumnSchema, TableSchema};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::SaltString;
use rand_core::OsRng;
use rand_core::RngCore;

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(id: i32, name: String, email: String, password_hash: String) -> Self {
        User {
            id,
            name,
            email,
            password_hash,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if !self.email.contains('@') {
            return Err("Email must be valid".to_string());
        }
        Ok(())
    }

    pub fn schema() -> TableSchema {
        TableSchema {
            name: "users".to_string(),
            columns: vec![
                ColumnSchema {
                    name: "id".to_string(),
                    data_type: "INT PRIMARY KEY AUTO_INCREMENT".to_string(),
                },
                ColumnSchema {
                    name: "name".to_string(),
                    data_type: "VARCHAR(255)".to_string(),
                },
                ColumnSchema {
                    name: "email".to_string(),
                    data_type: "VARCHAR(255)".to_string(),
                },
                ColumnSchema {
                    name: "password_hash".to_string(),
                    data_type: "VARCHAR(255)".to_string(),
                },
            ],
        }
    }

    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        hash
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.password_hash);
        if let Ok(parsed_hash) = parsed_hash {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        } else {
            false
        }
    }

    pub fn from_registration(id: i32, name: String, email: String, password: &str) -> Self {
        let password_hash = Self::hash_password(password);
        Self::new(id, name, email, password_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_validation() {
        // Bad name
        let user = User::new(
            1,
            "".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
        );
        assert!(user.validate().is_err());

        // Bad email
        let user = User::new(
            1,
            "Test".to_string(),
            "invalidemail".to_string(),
            "hash".to_string(),
        );
        assert!(user.validate().is_err());

        // Ok
        let user = User::new(
            1,
            "Test".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
        );
        assert!(user.validate().is_ok());
    }

    #[test]
    fn test_password_hash_and_verify() {
        let password = "super_secret";
        let hash = User::hash_password(password);
        let user = User::new(1, "Test".to_string(), "test@example.com".to_string(), hash);
        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrong_password"));
    }

    #[test]
    fn test_form_registration() {
        let password = "super_secret";
        let user = User::from_registration(
            1,
            "Test".to_string(),
            "test@example.com".to_string(),
            password,
        );
        assert!(user.verify_password(password));
        assert!(!user.verify_password("bad"));
    }
}
