use sqlx::postgres::PgPoolOptions;
use std::env;

pub async connect() -> sqlx:Pool<sqlx::Postgres> { 
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to databse")
}