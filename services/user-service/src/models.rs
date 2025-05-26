use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User { 
    pub id: Uuid,
    pub username: String,
    pub email : String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct RegisterUser { 
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser { 
    pub email: String,
    pub password: String,
}