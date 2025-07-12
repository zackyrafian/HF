use actix_web::{post, get, web, HttpResponse, HttpRequest, Responder, cookie::Cookie};
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Serialize;

use crate::models::{RegisterUser, LoginData};
use crate::utils::jwt::{create_jwt, verify_jwt};


#[derive(Serialize)]
struct User {
    id: i64,
    username: String,
    email: String,
}

#[post("/register")]
pub async fn register_user(
    user: web::Json<RegisterUser>,
    db_pool: web::Data<PgPool>,
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
        Ok(record) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User registered successfully",
            "user_id": record.id,
        })),
        Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
    }
}

#[post("/login")]
pub async fn login_user(
    db_pool: web::Data<PgPool>,
    form: web::Json<LoginData>,
) -> impl Responder {
    let user = sqlx::query!(
        "SELECT id, username, email, password FROM users WHERE email = $1",
        form.email
    )
    .fetch_optional(db_pool.get_ref())
    .await;

    let user = match user {
        Ok(Some(user)) => user,
        _ => return HttpResponse::Unauthorized().body("Invalid credentials"),
    };

    if !verify(&form.password, &user.password).unwrap_or(false) {
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    let token = match create_jwt(&user.email) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Token creation failed"),
    };

    let cookie = Cookie::build("auth_token", token)
        .path("/")
        .http_only(true)
        .secure(false)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({
            "message": "Login successful",
            "user": {
                "id": user.id,
                "username": user.username,
                "email": user.email,
            }
        }))
}

#[post("/logout")]
pub async fn logout_user() -> impl Responder {
    let cookie = Cookie::build("auth_token", "")
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(0))
        .finish();

    HttpResponse::Ok().cookie(cookie).body("Logged out")
}


#[get("/me")]
pub async fn get_me(req: HttpRequest, db_pool: web::Data<PgPool>) -> impl Responder {
    let token = req.cookie("auth_token")
        .map(|c| c.value().to_string());

    let token = match token {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("No auth token"),
    };

    let claims = match verify_jwt(&token) { 
        Ok(claims) => claims, 
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let email = claims.sub;

    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE email = $1",
        &email
    )
    .fetch_optional(db_pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().body("User not found"),
    }
}


#[get("/users")]
pub async fn get_users(db_pool: web::Data<PgPool>) -> impl Responder {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users"
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match users {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users"),
    }
}