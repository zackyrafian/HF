use actix_web::{post, get, web, HttpResponse, Responder};
use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use serde::Serialize;

use crate::models::RegisterUser;

#[derive(Serialize)]
struct User {
    id: i64,
    username: String,
    email: String,
}

#[post("/register")]
pub async fn register_user(
    user: web::Json<RegisterUser>,
    db_pool: web::Data<PgPool>
) -> impl Responder {
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(pw) => pw,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        user.username,
        user.email,
        hashed_password,
    )
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "User registered successfully",
                "user_id": record.id,
            }))
        },
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}

#[get("/users")]
pub async fn get_users(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email FROM users
        "#
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Failed to fetch users: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}
