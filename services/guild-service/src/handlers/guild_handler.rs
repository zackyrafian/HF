use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use chrono::Utc;

use crate::models::guild::{Guild, CreateGuildPayload};
use sqlx::PgPool;

#[axum::debug_handler]
pub async fn create_guild(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateGuildPayload>,
) -> impl IntoResponse {
    let guild_id = Uuid::new_v4();
    let created_at = Utc::now().naive_utc();

    let result = sqlx::query_as!(
        Guild,
        "INSERT INTO guilds (id, name, owner_id, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
        guild_id,
        payload.name,
        payload.owner_id,
        created_at,
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(guild) => (StatusCode::CREATED, Json(guild)).into_response(),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn get_guild(
    State(pool): State<PgPool>,
    Path(guild_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Guild,
        "SELECT id, name, owner_id, created_at FROM guilds WHERE id = $1",
        guild_id,
    )
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(guild)) => Json(guild).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
