use crate::helpers::{get_random_email, TestApp};
use auth_service::{routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "loginAttemptId": "test123",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": get_random_email(),
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": "test123"
        }),
        serde_json::json!({}),
        serde_json::json!({
            "wrong_field": "test"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "",
            "loginAttemptId": "test123",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": "invalid-email",
            "loginAttemptId": "test123", 
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": "",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": "test123",
            "2FACode": ""
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    // First, signup a user with 2FA enabled
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    
    // Login to trigger 2FA
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    
    // Test cases with incorrect credentials
    let test_cases = [
        // Wrong email
        serde_json::json!({
            "email": get_random_email(), // Different email
            "loginAttemptId": json_body.login_attempt_id,
            "2FACode": "123456"
        }),
        // Wrong login_attempt_id  
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "wrong-attempt-id",
            "2FACode": "123456"
        }),
        // Wrong 2FA code
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": json_body.login_attempt_id,
            "2FACode": "999999" // Wrong code
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-2fa with the 2FA code from the first login request. This should fail.
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    // First, signup a user with 2FA enabled
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    
    // First login to trigger 2FA
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
    });
    let first_response = app.post_login(&login_body).await;
    assert_eq!(first_response.status().as_u16(), 206);
    
    let first_login_body = first_response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    
    // Second login to trigger 2FA (this should overwrite the first code)
    let second_response = app.post_login(&login_body).await;
    assert_eq!(second_response.status().as_u16(), 206);
    
    let second_login_body = second_response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    
    // Verify the login_attempt_ids are different
    assert_ne!(first_login_body.login_attempt_id, second_login_body.login_attempt_id);
    
    // Try to verify 2FA with the old (first) login attempt ID and any 2FA code
    // This should fail because the first code has been overwritten
    let verify_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": first_login_body.login_attempt_id, // Old attempt ID
        "2FACode": "123456"
    });
    
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    // First, signup a user with 2FA enabled
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    
    // Login to trigger 2FA
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    
    // Get the actual 2FA code from the store
    let email = auth_service::domain::Email::parse(random_email.clone()).unwrap();
    let two_fa_code = {
        let two_fa_store = app.two_fa_code_store.read().await;
        let (_, code) = two_fa_store.get_code(&email).await.unwrap();
        code
    }; // Read lock is dropped here
    
    // Verify 2FA with correct credentials
    let verify_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": json_body.login_attempt_id,
        "2FACode": two_fa_code.as_ref()
    });
    
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);
    
    // Assert that the JWT auth cookie gets set
    let cookies = response.cookies().collect::<Vec<_>>();
    assert!(cookies.iter().any(|cookie| cookie.name() == JWT_COOKIE_NAME), 
            "JWT auth cookie should be set after successful 2FA verification");
    
    // Verify that the 2FA code is removed from the store after successful verification
    let two_fa_store = app.two_fa_code_store.read().await;
    assert!(two_fa_store.get_code(&email).await.is_err(), "2FA code should be removed after successful verification");
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    // First, signup a user with 2FA enabled
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    
    // Login to trigger 2FA
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    
    // Get the actual 2FA code from the store
    let email = auth_service::domain::Email::parse(random_email.clone()).unwrap();
    let two_fa_code = {
        let two_fa_store = app.two_fa_code_store.read().await;
        let (_, code) = two_fa_store.get_code(&email).await.unwrap();
        code
    }; // Read lock is dropped here
    
    // Verify 2FA with correct credentials (first time - should succeed)
    let verify_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": json_body.login_attempt_id,
        "2FACode": two_fa_code.as_ref()
    });
    
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);
    
    // Try to use the same 2FA code again (second time - should fail with 401)
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401, "Using the same 2FA code twice should return 401");
}