use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use axum_extra::extract::CookieJar;
use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password, UserStoreError}};
use crate::utils::auth;

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let auth_cookie = match auth::generate_auth_cookie(&email) {
        Ok(auth_cookie) => auth_cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    let user_store = state.user_store.read().await;
    
    match user_store.validate_user(&email, &password).await {
        Ok(_) => (updated_jar, Ok(StatusCode::OK.into_response())),
        Err(UserStoreError::UserNotFound) | Err(UserStoreError::InvalidCredentials) => {
            (updated_jar, Err(AuthAPIError::IncorrectCredentials))
        }
        Err(_) => (updated_jar, Err(AuthAPIError::UnexpectedError)),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}