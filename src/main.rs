use axum::{routing::get, Router};
use services::assets::get_assets;
use tokio::net::TcpListener;
mod models;
mod services;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/assets/{network_type}", get(get_assets))
        .route("/assets", get(get_assets));
    let addr = "0.0.0.0:3001";
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", &addr);
    axum::serve(tcp_listener, app).await.unwrap();
}
