use actix_web::web;
use crate::handlers::register_user;
use crate::handlers::get_users;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(get_users);
    //     web::scope("/users")
    //         .route("", web::get().to(handlers::get_users))
    //         .route("/register", web::post().to(handlers::register_user))
    // );
}
