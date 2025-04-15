use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    appstate::AppState,
    cache::{assets_cache::AssetsCache, blocknumbers_cache::BlockNumbers},
    models::response::ApiResponse,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockNumbersResponse {
    pub mainnet: Option<HashMap<String, u64>>,
    pub testnet: Option<HashMap<String, u64>>,
}

pub async fn get_block_numbers(
    State(appstate): State<Arc<AppState>>,
    network_type: Option<Path<String>>,
) -> Result<axum::Json<ApiResponse<BlockNumbersResponse>>, axum::http::StatusCode> {
    let cached_block_numbers = appstate.block_numbers.lock().await.clone();
    match network_type {
        Some(Path(network_type)) => {
            if network_type == "testnet" {
                return Ok(Json(ApiResponse {
                    data: BlockNumbersResponse {
                        testnet: Some(cached_block_numbers.testnet.clone()),
                        mainnet: None,
                    },
                }));
            } else {
                return Ok(Json(ApiResponse {
                    data: BlockNumbersResponse {
                        mainnet: Some(cached_block_numbers.mainnet.clone()),
                        testnet: None,
                    },
                }));
            }
        }
        None => {
            return Ok(Json(ApiResponse {
                data: BlockNumbersResponse {
                    mainnet: Some(cached_block_numbers.mainnet.clone()),
                    testnet: Some(cached_block_numbers.testnet.clone()),
                },
            }));
        }
    }
}
