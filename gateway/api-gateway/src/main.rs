use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

mod routes;
use routes::proxy_user_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Gateway running at http://localhost:8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/api/user/{tail:.*}", web::to(proxy_user_service))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
