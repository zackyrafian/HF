use actix_web::web;
// use crate::controller::{register_user, login_user, logout_user, get_me, get_users};
use crate::controller::user_controller::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(login_user);
    cfg.service(logout_user);
    cfg.service(get_me);
    cfg.service(get_users);
}