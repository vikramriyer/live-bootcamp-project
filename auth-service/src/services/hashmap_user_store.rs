use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        
        if user.password() == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password123".to_string(), false);
        
        let result = store.add_user(user.clone());
        assert!(result.is_ok());
        
        // Test adding duplicate user
        let duplicate_result = store.add_user(user);
        assert_eq!(duplicate_result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password123".to_string(), false);
        
        // Test getting non-existent user
        let result = store.get_user("nonexistent@example.com");
        assert_eq!(result, Err(UserStoreError::UserNotFound));
        
        // Add user and test getting existing user
        store.add_user(user.clone()).unwrap();
        let result = store.get_user("test@example.com");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password123".to_string(), false);
        store.add_user(user).unwrap();
        
        // Test valid credentials
        let result = store.validate_user("test@example.com", "password123");
        assert!(result.is_ok());
        
        // Test invalid password
        let result = store.validate_user("test@example.com", "wrongpassword");
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));
        
        // Test non-existent user
        let result = store.validate_user("nonexistent@example.com", "password123");
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}