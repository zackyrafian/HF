use axum::Router;
use std::net::SocketAddr;
use dotenvy::dotenv;
use tracing_subscriber;
mod db;
mod models;
mod handlers;
mod router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::connect_to_db().await;
    let app = router::guild::guild_routes(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Listening on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
