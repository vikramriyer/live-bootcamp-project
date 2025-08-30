use std::collections::HashSet;
use crate::domain::{BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<(String, usize)>,
}

impl HashsetBannedTokenStore {
    pub async fn store_tokens(&mut self, token: String, exp: usize) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert((token, exp));
        Ok(())
    }

    pub async fn is_token_exists(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let exists = self.banned_tokens.iter().any(|(banned_token, _)| banned_token == token);
        Ok(exists)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_tokens() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "test.jwt.token".to_string();
        let exp = 1234567890;
        
        let result = store.store_tokens(token.clone(), exp).await;
        assert!(result.is_ok());
        
        // Verify token was stored
        assert!(store.banned_tokens.contains(&(token, exp)));
    }

    #[tokio::test]
    async fn test_is_token_exists() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "test.jwt.token".to_string();
        let exp = 1234567890;
        
        // Test non-existent token
        let result = store.is_token_exists(&token).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
        
        // Add token and test existing token
        store.store_tokens(token.clone(), exp).await.unwrap();
        let result = store.is_token_exists(&token).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
        
        // Test different token
        let different_token = "different.jwt.token";
        let result = store.is_token_exists(different_token).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[tokio::test]
    async fn test_multiple_tokens() {
        let mut store = HashsetBannedTokenStore::default();
        let tokens = vec![
            ("token1.jwt.test".to_string(), 1111111111),
            ("token2.jwt.test".to_string(), 2222222222),
            ("token3.jwt.test".to_string(), 3333333333),
        ];
        
        // Store multiple tokens
        for (token, exp) in &tokens {
            let result = store.store_tokens(token.clone(), *exp).await;
            assert!(result.is_ok());
        }
        
        // Verify all tokens exist
        for (token, _) in &tokens {
            let result = store.is_token_exists(token).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), true);
        }
        
        // Verify store contains correct number of tokens
        assert_eq!(store.banned_tokens.len(), 3);
    }

    #[tokio::test]
    async fn test_duplicate_token_same_expiration() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "duplicate.jwt.token".to_string();
        let exp = 1234567890;
        
        // Store token twice
        store.store_tokens(token.clone(), exp).await.unwrap();
        store.store_tokens(token.clone(), exp).await.unwrap();
        
        // HashSet should only contain one copy
        assert_eq!(store.banned_tokens.len(), 1);
        assert!(store.banned_tokens.contains(&(token, exp)));
    }

    #[tokio::test]
    async fn test_same_token_different_expiration() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "same.jwt.token".to_string();
        let exp1 = 1111111111;
        let exp2 = 2222222222;
        
        // Store same token with different expirations
        store.store_tokens(token.clone(), exp1).await.unwrap();
        store.store_tokens(token.clone(), exp2).await.unwrap();
        
        // Both entries should exist (different tuples)
        assert_eq!(store.banned_tokens.len(), 2);
        assert!(store.banned_tokens.contains(&(token.clone(), exp1)));
        assert!(store.banned_tokens.contains(&(token.clone(), exp2)));
        
        // is_token_exists should return true (finds either)
        let result = store.is_token_exists(&token).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}