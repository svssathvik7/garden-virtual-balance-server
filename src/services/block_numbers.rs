use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers},
    models::response::ApiResponse,
};

pub async fn get_block_numbers(
    State(cached_block_numbers): State<Arc<Mutex<BlockNumbers>>>,
    State(cached_assets): State<Arc<AssetsCache>>,
    network_type: Option<Path<String>>,
) -> Result<axum::Json<ApiResponse<HashMap<String, u64>>>, axum::http::StatusCode> {
    match network_type {
        Some(Path(network_type)) => {
            if network_type == "testnet" {
                return Ok(Json(ApiResponse {
                    data: cached_block_numbers.lock().unwrap().testnet.clone(),
                }));
            } else {
                return Ok(Json(ApiResponse {
                    data: cached_block_numbers.lock().unwrap().mainnet.clone(),
                }));
            }
        }
        None => {
            let mut response = cached_block_numbers.lock().unwrap().mainnet.clone();
            response.extend(cached_block_numbers.lock().unwrap().testnet.clone());
            return Ok(Json(ApiResponse { data: response }));
        }
    }
}
