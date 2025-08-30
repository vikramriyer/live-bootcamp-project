use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::Deserialize;

pub async fn verify_token(
    Json(_request): Json<VerifyTokenRequest>
) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
