use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::env;

mod db;
mod handlers;
mod routes;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("127.0.0.1:{}", port);

    let pool = db::connect().await;

    println!("Server running at http://{}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);


        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(addr)?
    .run()
    .await
}
