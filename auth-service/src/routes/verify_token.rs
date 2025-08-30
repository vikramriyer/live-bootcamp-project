use axum::{extract::State, response::IntoResponse, http::StatusCode, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    match validate_token(&request.token).await {
        Ok(_) => {
            // Check if token is banned
            let banned_store = state.banned_token_store.read().await;
            match banned_store.is_token_exists(&request.token).await {
                Ok(true) => Err(AuthAPIError::InvalidToken), // Token is banned
                Ok(false) => Ok(StatusCode::OK.into_response()), // Token is valid and not banned
                Err(_) => Err(AuthAPIError::UnexpectedError),
            }
        },
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
