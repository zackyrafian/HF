use actix_web::{App, HttpServer, web};
use db::connect;

mod db;
mod handlers;
mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = connect().await;

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Server running on http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))   
            .configure(routes::config)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}
