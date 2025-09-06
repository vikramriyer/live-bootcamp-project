use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;

use crate::{app_state::AppState, domain::{AuthAPIError, Email, data_stores::{LoginAttemptId, TwoFACode}}, utils::auth::generate_auth_cookie};

pub async fn verify_2fa(
    State(state): State<AppState>, 
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Parse and validate email
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    
    // Validate that string fields are not empty
    if request.login_attempt_id.trim().is_empty() || request.two_fa_code.trim().is_empty() {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    }

    // Parse login attempt ID and 2FA code
    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
        Ok(id) => id,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };
    
    let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
        Ok(code) => code,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    // Get the stored code for this email
    let two_fa_store = state.two_fa_code_store.read().await;
    match two_fa_store.get_code(&email).await {
        Ok((stored_login_attempt_id, stored_code)) => {
            // Check if the provided credentials match the stored ones
            if stored_login_attempt_id == login_attempt_id && stored_code == two_fa_code {
                // Drop the read lock before acquiring write lock
                drop(two_fa_store);
                
                // Remove the used 2FA code from the store
                {
                    let mut two_fa_store = state.two_fa_code_store.write().await;
                    if let Err(_) = two_fa_store.remove_code(&email).await {
                        return (jar, Err(AuthAPIError::UnexpectedError));
                    }
                } // Write lock is dropped here
                
                // Generate JWT token and set auth cookie
                let auth_cookie = match generate_auth_cookie(&email) {
                    Ok(cookie) => cookie,
                    Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
                };
                
                // Return response with cookie
                let updated_jar = jar.add(auth_cookie);
                (updated_jar, Ok(StatusCode::OK.into_response()))
            } else {
                (jar, Err(AuthAPIError::IncorrectCredentials))
            }
        }
        Err(_) => (jar, Err(AuthAPIError::IncorrectCredentials)),
    }
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String,
}
