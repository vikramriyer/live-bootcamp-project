use auth_service::{
    app_state::AppState, services::{hashmap_user_store::HashmapUserStore, HashmapTwoFACodeStore, HashsetBannedTokenStore, MockEmailClient}, utils::constants::prod,
    Application,
};
use std::sync::Arc;
use tokio::sync::RwLock;


#[tokio::main]
async fn main() {
    let user_store: Arc<RwLock<dyn auth_service::domain::data_stores::UserStore + Send + Sync>> = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store: Arc<RwLock<dyn auth_service::domain::data_stores::BannedTokenStore + Send + Sync>> = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store: Arc<RwLock<dyn auth_service::domain::data_stores::TwoFACodeStore + Send + Sync>> = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client: Arc<RwLock<dyn auth_service::domain::EmailClient + Send + Sync>> = Arc::new(RwLock::new(MockEmailClient));
    
    let app_state = AppState::new(user_store, banned_token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

#[tokio::main]
async fn main() {
    // We will use this PostgreSQL pool in the next task! 
    let pg_pool = configure_postgresql().await;

    //...
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database! 
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}
