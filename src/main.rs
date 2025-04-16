use std::env;
use std::sync::Arc;

use appstate::AppState;
use axum::{routing::get, Router};
use cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers};
use dotenv::dotenv;
use reqwest::Method;
use services::assets::get_assets;
use services::block_numbers::get_block_numbers;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
mod appstate;
mod cache;
mod models;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let host = env::var("HOST").expect("Host must be set");
    let port = env::var("PORT").expect("Port must be set");
    let cached_assets = Arc::new(AssetsCache::default());
    let block_numbers = Arc::new(Mutex::new(BlockNumbers::default()));

    let appstate = Arc::new(AppState {
        cached_assets: cached_assets.clone(),
        block_numbers: block_numbers.clone(),
    });

    let block_numbers_clone = block_numbers.clone();
    // Start the block numbers cron job
    tokio::spawn(async move {
        BlockNumbers::start_cron(block_numbers_clone).await;
    });

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any());

    let app = Router::new()
        .route("/assets/{network_type}", get(get_assets))
        .route("/assets", get(get_assets))
        .route("/blocknumbers/{network_type}", get(get_block_numbers))
        .route("/blocknumbers", get(get_block_numbers))
        .layer(cors)
        .with_state(appstate);
    let addr = format!("{}:{}", host, port);
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", &addr);
    axum::serve(tcp_listener, app).await.unwrap();
}
