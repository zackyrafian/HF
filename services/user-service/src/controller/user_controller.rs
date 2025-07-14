use actix_web::{ 
    get, post, web, HttpRequest, HttpResponse, Responder, cookie::Cookie, 
};
use sqlx::PgPool;

use crate::model::user::{RegisterUser, LoginData};
use crate::service::user_service;
use crate::utils::jwt::{verify_jwt};

#[post("/register")]
pub async fn register_user(
    db: web::Data<PgPool>,
    user: web::Json<RegisterUser>,
) -> impl Responder {
    match user_service::register_user(user.into_inner(), &db).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Register error: {}", e)),
    }
}

#[post("/login")]
pub async fn login_user(
    db: web::Data<PgPool>,
    data: web::Json<LoginData>,
) -> impl Responder {
    match user_service::login_user(data.into_inner(), &db).await {
        Ok((token, user)) => {
            let cookie = Cookie::build("auth_token", token)
                .path("/")
                .http_only(true)
                .secure(false)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(user)
        },
        Err(e) => HttpResponse::Unauthorized().body(format!("Login error: {}", e)),
    }
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
pub async fn get_me(
    db: web::Data<PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let token = match req.cookie("auth_token") {
        Some(t) => t.value().to_string(),
        None => return HttpResponse::Unauthorized().body("No auth token"),
    };

    let claims = match verify_jwt(&token) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    match user_service::get_user_by_email(&claims.sub, &db).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

#[get("/users")]
pub async fn get_users(
    db: web::Data<PgPool>,
) -> impl Responder {
    match user_service::get_all_users(&db).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Fetch error: {}", e)),
    }
}