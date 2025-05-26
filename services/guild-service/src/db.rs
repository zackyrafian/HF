use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connect_to_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
