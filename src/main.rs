use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers};
use services::assets::get_assets;
use services::block_numbers::get_block_numbers;
use tokio::net::TcpListener;
mod cache;
mod models;
mod services;

#[tokio::main]
async fn main() {
    let cached_assets = Arc::new(AssetsCache::default());
    let block_numbers = Arc::new(Mutex::new(BlockNumbers::default()));

    // Start the block numbers cron job
    let block_numbers_clone = block_numbers.clone();
    tokio::spawn(async move {
        let mut block_numbers = block_numbers_clone.lock().unwrap();
        block_numbers.cron().await;
    });

    let app = Router::new()
        .route("/assets/{network_type}", get(get_assets))
        .route("/assets", get(get_assets))
        .route("/blocknumbers/{network_type}", get(get_block_numbers))
        .route("/blocknumbers", get(get_block_numbers))
        .with_state(cached_assets)
        .with_state(block_numbers);
    let addr = "0.0.0.0:3001";
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", &addr);
    axum::serve(tcp_listener, app).await.unwrap();
}
