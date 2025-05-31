use actix_web::web;
use crate::handlers::{register_user, login_user, logout_user, get_users};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(get_users);
    cfg.service(login_user);
    cfg.service(logout_user);
}
