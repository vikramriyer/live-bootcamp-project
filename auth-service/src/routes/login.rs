use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Serialize, Deserialize};
use axum_extra::extract::CookieJar;
use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password, UserStoreError, data_stores::{TwoFACode, LoginAttemptId}}};
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

    let user_store = state.user_store.read().await;
    
    match user_store.validate_user(&email, &password).await {
        Ok(_) => {
            // Get user to check 2FA requirement
            match user_store.get_user(&email).await {
                Ok(user) => {
                    if user.requires_2fa() {
                        handle_2fa(&email, &state, jar).await
                    } else {
                        handle_no_2fa(&email, jar).await
                    }
                }
                Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
            }
        }
        Err(UserStoreError::UserNotFound) | Err(UserStoreError::InvalidCredentials) => {
            (jar, Err(AuthAPIError::IncorrectCredentials))
        }
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();
    
    let mut two_fa_store = state.two_fa_code_store.write().await;
    match two_fa_store.add_code(email.clone(), login_attempt_id.clone(), two_fa_code).await {
        Ok(_) => {
            (
                jar,
                Ok((
                    StatusCode::PARTIAL_CONTENT,
                    Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
                        message: "2FA required".to_owned(),
                        login_attempt_id: login_attempt_id.as_ref().to_owned(),
                    }))
                ))
            )
        }
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
    }
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let auth_cookie = match auth::generate_auth_cookie(email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };
    
    let updated_jar = jar.add(auth_cookie);
    
    (
        updated_jar,
        Ok((StatusCode::OK, Json(LoginResponse::RegularAuth)))
    )
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}