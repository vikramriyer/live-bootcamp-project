use super::{User, Email, Password};
use crate::services::{HashmapUserStore, HashsetBannedTokenStore};

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        self.add_user(user)
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.get_user(email)
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        self.validate_user(email, password)
    }
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

pub trait BannedTokenStore {
    async fn store_tokens(&mut self, token: String, exp: usize) -> Result<(), BannedTokenStoreError>;
    async fn is_token_exists(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

impl BannedTokenStore for HashsetBannedTokenStore {
    async fn store_tokens(&mut self, token: String, exp: usize) -> Result<(), BannedTokenStoreError> {
        todo!();
    }

    async fn is_token_exists(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        todo!();
    }
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
}