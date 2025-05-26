use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Guild {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateGuildPayload {
    pub name: String,
    pub owner_id: Uuid,
}
