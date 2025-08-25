use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
}


fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).unwrap_or_else(|_| {
        // Provide a default test secret when JWT_SECRET is not set (e.g., in CI/test environments)
        if cfg!(test) {
            "test-secret-key-for-testing-only-never-use-in-production".to_string()
        } else {
            panic!("JWT_SECRET must be set in production environments")
        }
    });
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
}

pub const JWT_COOKIE_NAME: &str = "jwt";