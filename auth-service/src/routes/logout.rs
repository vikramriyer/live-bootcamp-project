use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    match validate_token(&token).await {
        Ok(_) => {
            let updated_jar = jar.remove(JWT_COOKIE_NAME);
            (updated_jar, Ok(StatusCode::OK))
        },
        Err(_) => (jar, Err(AuthAPIError::InvalidToken)),
    }
}