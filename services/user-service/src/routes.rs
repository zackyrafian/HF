use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(handlers::register_user))
            .route("/login", web::post().to(handlers::login_user))
            .route("", web::get().to(handlers::get_users)),
    );
}