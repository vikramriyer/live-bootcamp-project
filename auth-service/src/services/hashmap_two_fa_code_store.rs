use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        self.codes
            .get(email)
            .cloned()
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_and_get_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::parse("123456".to_string()).unwrap();

        let result = store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await;
        assert!(result.is_ok());

        let retrieved = store.get_code(&email).await;
        assert!(retrieved.is_ok());
        let (retrieved_id, retrieved_code) = retrieved.unwrap();
        assert_eq!(retrieved_id, login_attempt_id);
        assert_eq!(retrieved_code, code);
    }

    #[tokio::test]
    async fn test_get_code_not_found() {
        let store = HashmapTwoFACodeStore::default();
        let email = Email::parse("nonexistent@example.com".to_string()).unwrap();

        let result = store.get_code(&email).await;
        assert_eq!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::parse("123456".to_string()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id, code)
            .await
            .unwrap();

        let result = store.remove_code(&email).await;
        assert!(result.is_ok());

        let get_result = store.get_code(&email).await;
        assert_eq!(get_result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn test_remove_nonexistent_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("nonexistent@example.com".to_string()).unwrap();

        let result = store.remove_code(&email).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_overwrite_existing_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id1 = LoginAttemptId::default();
        let code1 = TwoFACode::parse("123456".to_string()).unwrap();
        let login_attempt_id2 = LoginAttemptId::default();
        let code2 = TwoFACode::parse("654321".to_string()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id1, code1)
            .await
            .unwrap();

        store
            .add_code(email.clone(), login_attempt_id2.clone(), code2.clone())
            .await
            .unwrap();

        let retrieved = store.get_code(&email).await.unwrap();
        assert_eq!(retrieved.0, login_attempt_id2);
        assert_eq!(retrieved.1, code2);
    }

    #[tokio::test]
    async fn test_multiple_emails() {
        let mut store = HashmapTwoFACodeStore::default();
        let email1 = Email::parse("user1@example.com".to_string()).unwrap();
        let email2 = Email::parse("user2@example.com".to_string()).unwrap();
        let login_attempt_id1 = LoginAttemptId::default();
        let login_attempt_id2 = LoginAttemptId::default();
        let code1 = TwoFACode::parse("111111".to_string()).unwrap();
        let code2 = TwoFACode::parse("222222".to_string()).unwrap();

        store
            .add_code(email1.clone(), login_attempt_id1.clone(), code1.clone())
            .await
            .unwrap();
        store
            .add_code(email2.clone(), login_attempt_id2.clone(), code2.clone())
            .await
            .unwrap();

        let retrieved1 = store.get_code(&email1).await.unwrap();
        let retrieved2 = store.get_code(&email2).await.unwrap();

        assert_eq!(retrieved1.0, login_attempt_id1);
        assert_eq!(retrieved1.1, code1);
        assert_eq!(retrieved2.0, login_attempt_id2);
        assert_eq!(retrieved2.1, code2);
    }
}