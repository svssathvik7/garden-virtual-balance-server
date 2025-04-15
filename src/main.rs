use std::sync::Arc;

use appstate::AppState;
use axum::{routing::get, Router};
use cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers};
use services::assets::get_assets;
use services::block_numbers::get_block_numbers;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
mod appstate;
mod cache;
mod models;
mod services;

#[tokio::main]
async fn main() {
    let cached_assets = Arc::new(AssetsCache::default());
    let mut block_numbers = Arc::new(Mutex::new(BlockNumbers::default()));

    let appstate = Arc::new(AppState {
        cached_assets: cached_assets.clone(),
        block_numbers: block_numbers.clone(),
    });

    // Start the block numbers cron job
    tokio::spawn(async move {
        block_numbers.lock().await.cron().await;
    });

    let app = Router::new()
        .route("/assets/{network_type}", get(get_assets))
        .route("/assets", get(get_assets))
        .route("/blocknumbers/{network_type}", get(get_block_numbers))
        .route("/blocknumbers", get(get_block_numbers))
        .with_state(appstate);
    let addr = "0.0.0.0:3001";
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", &addr);
    axum::serve(tcp_listener, app).await.unwrap();
}
