use std::sync::Arc;

use appstate::AppState;
use axum::routing::post;
use axum::{routing::get, Router};
use cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers};
use dotenv::dotenv;
use handlers::assets::get_assets;
use handlers::block_numbers::{get_block_numbers, get_block_numbers_by_chain};
use handlers::health::health_check;
use handlers::notifications::{add_notification, get_latest_notification, get_notification_by_id};
use models::notification::NotificationRepo;
use reqwest::Method;
use tokio::net::TcpListener;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
mod appstate;
mod cache;
mod handlers;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let host = "0.0.0.0";
    let port = "3001";
    let cached_assets = Arc::new(AssetsCache::new());
    let block_numbers = Arc::new(BlockNumbers::new().await);

    let notification_repo = Arc::new(
        NotificationRepo::new()
            .await
            .expect("Failed to create notification repo"),
    );

    let appstate = Arc::new(AppState {
        cached_assets,
        block_numbers: block_numbers.clone(),
        notification_repo,
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
        .route("/health", get(health_check))
        .route("/add-notification", post(add_notification))
        .route("/notifications/{id}", get(get_notification_by_id))
        .route("/notifications", get(get_latest_notification))
        .layer(cors)
        .with_state(appstate);

    let addr = format!("{}:{}", host, port);
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", &addr);
    axum::serve(tcp_listener, app).await.unwrap();
}
