use axum::{
    routing::{get, post},
    Router,
};
use crate::handlers::guild_handler::{create_guild, get_guild};
use sqlx::PgPool;

pub fn guild_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/guild", post(create_guild))
        .route("/guild/:id", get(get_guild))
        .with_state(pool)
}
