use auth_service::{utils::constants::JWT_COOKIE_NAME};
use reqwest::{Url, cookie::CookieStore};

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    let response = app.logout().await;
    assert_eq!(response.status(), 400)
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;
    assert_eq!(response.status(), 401)
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    // Create a user
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Login to get a valid JWT cookie
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Get the JWT token value before logout
    let url = &"http://127.0.0.1".parse().unwrap();
    let cookies = app.cookie_jar.cookies(url).unwrap();
    let jwt_cookie = cookies.to_str().unwrap();
    let token_value = jwt_cookie
        .split(&format!("{}=", JWT_COOKIE_NAME))
        .nth(1)
        .unwrap()
        .split(';')
        .next()
        .unwrap();

    // First logout should succeed
    let response = app.logout().await;
    assert_eq!(response.status(), 200);

    // Verify token was added to banned store
    let banned_store = app.banned_token_store.read().await;
    assert!(banned_store.is_token_exists(token_value).await.unwrap());
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    // Create a user
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Login to get a valid JWT cookie
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // First logout should succeed
    let response = app.logout().await;
    assert_eq!(response.status(), 200);

    // second should fail
    let response = app.logout().await;
    assert_eq!(response.status(), 400);
}
