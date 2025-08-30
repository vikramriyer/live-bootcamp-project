use axum::{extract::State, response::IntoResponse, http::StatusCode, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    match validate_token(&request.token, Some(&state.banned_token_store)).await {
        Ok(_) => Ok(StatusCode::OK.into_response()),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
