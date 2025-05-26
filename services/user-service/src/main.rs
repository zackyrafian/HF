mod models;
mod handlers;
mod routes;
mod db;

use actix_web::{App, HttpServer};
use db::connect;
use routes::config;
use actix_web::web;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = connect().await;

    info!("Connected to database");

    let port = std::env::var("PORT").unwrap_or("8081".to_string());

    println!("Starting user-service at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}