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