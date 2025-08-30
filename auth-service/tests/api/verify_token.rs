use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let malformed_body = serde_json::json!({
        "invalid_field": "test"
    });
    let response = app.post_verify_token(&malformed_body).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let email = crate::helpers::get_random_email();
    let signup_body = serde_json::json!({
        "email": email,
        "password": "Password123!",
        "requires2FA": false
    });
    
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": email,
        "password": "Password123!"
    });
    
    let login_response = app.post_login(&login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);

    let auth_cookie = login_response
        .cookies()
        .find(|cookie| cookie.name() == auth_service::utils::constants::JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let token = auth_cookie.value();

    let verify_body = serde_json::json!({
        "token": token
    });
    
    let response = app.post_verify_token(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    
    let invalid_body = serde_json::json!({
        "token": "invalid.jwt.token"
    });
    
    let response = app.post_verify_token(&invalid_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;

    let email = crate::helpers::get_random_email();
    let signup_body = serde_json::json!({
        "email": email,
        "password": "Password123!",
        "requires2FA": false
    });
    
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": email,
        "password": "Password123!"
    });
    
    let login_response = app.post_login(&login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);

    // Extract token from login response
    let auth_cookie = login_response
        .cookies()
        .find(|cookie| cookie.name() == auth_service::utils::constants::JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let token = auth_cookie.value().to_string();

    // Verify token is initially valid
    let verify_body = serde_json::json!({
        "token": token
    });
    
    let response = app.post_verify_token(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Logout to ban the token
    let logout_response = app.logout().await;
    assert_eq!(logout_response.status().as_u16(), 200);

    // Now verify that the banned token returns 401
    let response = app.post_verify_token(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);
}
