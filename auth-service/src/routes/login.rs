use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password, UserStoreError}};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };
    
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let user_store = state.user_store.read().await;
    
    match user_store.validate_user(&email, &password).await {
        Ok(_) => Ok(StatusCode::OK.into_response()),
        Err(UserStoreError::UserNotFound) | Err(UserStoreError::InvalidCredentials) => {
            Err(AuthAPIError::IncorrectCredentials)
        }
        Err(_) => Err(AuthAPIError::UnexpectedError),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}