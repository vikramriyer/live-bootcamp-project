use std::collections::HashMap;

use crate::domain::{User, UserStoreError, Email, Password};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
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

    pub fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
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
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let password = Password::parse("Password123!".to_string()).unwrap();
        let user = User::new(email, password, false);
        
        let result = store.add_user(user.clone());
        assert!(result.is_ok());
        
        // Test adding duplicate user
        let duplicate_result = store.add_user(user);
        assert_eq!(duplicate_result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let password = Password::parse("Password123!".to_string()).unwrap();
        let user = User::new(email.clone(), password, false);
        
        // Test getting non-existent user
        let nonexistent_email = Email::parse("nonexistent@example.com".to_string()).unwrap();
        let result = store.get_user(&nonexistent_email);
        assert_eq!(result, Err(UserStoreError::UserNotFound));
        
        // Add user and test getting existing user
        store.add_user(user.clone()).unwrap();
        let result = store.get_user(&email);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email, email);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let password = Password::parse("Password123!".to_string()).unwrap();
        let user = User::new(email.clone(), password.clone(), false);
        store.add_user(user).unwrap();
        
        // Test valid credentials
        let result = store.validate_user(&email, &password);
        assert!(result.is_ok());
        
        // Test invalid password
        let wrong_password = Password::parse("WrongPassword123!".to_string()).unwrap();
        let result = store.validate_user(&email, &wrong_password);
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));
        
        // Test non-existent user
        let nonexistent_email = Email::parse("nonexistent@example.com".to_string()).unwrap();
        let result = store.validate_user(&nonexistent_email, &password);
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}