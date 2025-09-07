use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_database_url();
}


fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).unwrap_or_else(|_| {
        // Provide a default test secret only when running unit tests
        if cfg!(test) {
            "test-secret-key-for-testing-only-never-use-in-production-minimum-32-chars".to_string()
        } else {
            panic!("JWT_SECRET must be set in production environments")
        }
    });
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

fn set_database_url() -> String {
    dotenv().ok(); // Load environment variables
    std_env::var(env::DATABASE_URL_ENV_VAR).unwrap_or_else(|_| {
        if cfg!(test) {
            "postgresql://postgres:password@localhost:5432/test_db".to_string()
        } else {
            panic!("DATABASE_URL must be set in production environments")
        }
    })
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
}

pub const JWT_COOKIE_NAME: &str = "jwt";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}

