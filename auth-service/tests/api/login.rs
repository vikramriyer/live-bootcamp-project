use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "password": "Password123!"
        }),
        serde_json::json!({
            "email": "test@example.com"
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
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
            "password": "Password123!"
        }),
        serde_json::json!({
            "email": "invalid-email",
            "password": "Password123!"
        }),
        serde_json::json!({
            "email": "test@example.com",
            "password": "short"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
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

    // First create a user
    let signup_body = serde_json::json!({
        "email": "test@example.com",
        "password": "Password123!",
        "requires2FA": false
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Test cases for incorrect credentials
    let test_cases = [
        serde_json::json!({
            "email": "test@example.com",
            "password": "WrongPassword123!"
        }),
        serde_json::json!({
            "email": "nonexistent@example.com", 
            "password": "Password123!"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
    }
}