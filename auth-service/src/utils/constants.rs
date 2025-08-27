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
        // Check if we're in a test environment by looking for common test indicators
        if cfg!(test) || std_env::var("CARGO_CFG_TEST").is_ok() || std_env::args().any(|arg| arg.contains("test")) {
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

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
}

pub const JWT_COOKIE_NAME: &str = "jwt";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}