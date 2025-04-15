use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    appstate::AppState,
    cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers},
    models::response::ApiResponse,
};

pub async fn get_block_numbers(
    State(appstate): State<Arc<AppState>>,
    network_type: Option<Path<String>>,
) -> Result<axum::Json<ApiResponse<HashMap<String, u64>>>, axum::http::StatusCode> {
    let cached_block_numbers = appstate.block_numbers.clone();
    match network_type {
        Some(Path(network_type)) => {
            if network_type == "testnet" {
                return Ok(Json(ApiResponse {
                    data: cached_block_numbers.lock().await.testnet.clone(),
                }));
            } else {
                return Ok(Json(ApiResponse {
                    data: cached_block_numbers.lock().await.mainnet.clone(),
                }));
            }
        }
        None => {
            let mut response = cached_block_numbers.lock().await.mainnet.clone();
            response.extend(cached_block_numbers.lock().await.testnet.clone());
            return Ok(Json(ApiResponse { data: response }));
        }
    }
}
