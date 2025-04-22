use std::env;
use std::sync::Arc;

use appstate::AppState;
use axum::{routing::get, Router};
use cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers};
use dotenv::dotenv;
use reqwest::Method;
use services::assets::get_assets;
use services::block_numbers::{get_block_numbers, get_block_numbers_by_chain};
use tokio::net::TcpListener;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
mod appstate;
mod cache;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let host = env::var("HOST").expect("Host must be set");
    let port = env::var("PORT").expect("Port must be set");
    let cached_assets = Arc::new(AssetsCache::default());
    let block_numbers = Arc::new(BlockNumbers::new().await);

    let appstate = Arc::new(AppState {
        cached_assets: cached_assets.clone(),
        block_numbers: block_numbers.clone(),
    });

    // spawn a new thread to update the block numbers every 5 seconds
    tokio::spawn(async move {
        block_numbers.start_cron().await;
    });

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any());

    let app = Router::new()
        .route("/assets/{network_type}", get(get_assets))
        .route("/assets", get(get_assets))
        .route(
            "/blocknumbers/{network_type}",
            get(get_block_numbers_by_chain),
        )
        .route("/blocknumbers", get(get_block_numbers))
        .layer(cors)
        .with_state(appstate);

    let addr = format!("{}:{}", host, port);
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", &addr);
    axum::serve(tcp_listener, app).await.unwrap();
}
