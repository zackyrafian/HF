use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::models::RegisterUser;

pub async fn register_user(user: web::Json<RegisterUser>) -> HttpResponse {
    println!("Register request: {:?}", user); 

    HttpResponse::Created().json(json!({
        "message": "User created",
        "user": {
            "username": user.username,
            "email": user.email
        }
    }))
}


pub async fn get_users() -> HttpResponse {
    HttpResponse::Ok().body("Hello from /users")
}
