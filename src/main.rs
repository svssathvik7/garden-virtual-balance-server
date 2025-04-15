use std::sync::Arc;

use axum::{routing::get, Router};
use cache::assets_cache::AssetsCache;
use services::assets::get_assets;
use tokio::net::TcpListener;
mod cache;
mod models;
mod services;

#[tokio::main]
async fn main() {
    let cached_assets = Arc::new(AssetsCache::default());
    let app = Router::new()
        .route("/assets/{network_type}", get(get_assets))
        .route("/assets", get(get_assets))
        .with_state(cached_assets);
    let addr = "0.0.0.0:3001";
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", &addr);
    axum::serve(tcp_listener, app).await.unwrap();
}
