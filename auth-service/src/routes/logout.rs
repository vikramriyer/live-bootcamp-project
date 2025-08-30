use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    match validate_token(&token, None).await {
        Ok(claims) => {
            // Add token to banned store
            let mut banned_store = state.banned_token_store.write().await;
            if let Err(_) = banned_store.store_tokens(token, claims.exp).await {
                return (jar, Err(AuthAPIError::UnexpectedError));
            }

            let updated_jar = jar.remove(JWT_COOKIE_NAME);
            (updated_jar, Ok(StatusCode::OK))
        },
        Err(_) => (jar, Err(AuthAPIError::InvalidToken)),
    }
}