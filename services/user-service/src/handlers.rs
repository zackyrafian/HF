use actix_web::{web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::json;
use sqlx::PgPool;
use crate::models::{RegisterUser, LoginUser};

pub async fn register_user(
    pool: web::Data<PgPool>, 
    user: web::Json::<RegisterUser>,
) -> HttpResponse { 
    
    let user_id = uuid::Uuid::new_v4();

    let hash_password = match hash(&user.password, DEFAULT_COST) { 
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError()
            .json(json!({ "error": "Failed to hash passowrd"})),
    };

    if let Err(e) = sqlx::query!(
        "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
        user_id,
        &user.username,
        &user.email,
        &hash_password
    )
    .execute(pool.as_ref())
    .await
    {
        eprintln!("Database error: {}", e);
        return HttpResponse::InternalServerError()
            .json(json!({ "error": "Failed to save user"}));
    }

    HttpResponse::Created().json(json!({
        "message": "User created successfully",
        "user": { 
            "id": user_id,
            "username" : &user.username,
            "email": &user.email
        }
    }))
}

pub async fn login_user(credentials: web::Json<LoginUser>) -> HttpResponse { 
    HttpResponse::Ok().json(json!({
        "message": "Login endpoint",
        "email" : &credentials.email
    }))
}

pub async fn get_users() -> HttpResponse { 
    HttpResponse::Ok().json(json!({ "message": "Get all users"}))
}

