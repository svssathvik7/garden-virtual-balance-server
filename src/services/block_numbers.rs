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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainnet: Option<HashMap<String, u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testnet: Option<HashMap<String, u64>>,
}

pub async fn get_block_numbers(
    State(appstate): State<Arc<AppState>>,
    network_type: Option<Path<String>>,
) -> Result<axum::Json<BlockNumbersResponse>, axum::http::StatusCode> {
    println!("I am into get_block_numbers {:?}", network_type);

    let cached_block_numbers = appstate.block_numbers.lock().await;
    println!("Got lock on block_numbers");

    match network_type {
        Some(Path(network_type)) => {
            if network_type == "testnet" {
                return Ok(Json(BlockNumbersResponse {
                    testnet: Some(cached_block_numbers.testnet.clone()),
                    mainnet: None,
                }));
            } else {
                return Ok(Json(BlockNumbersResponse {
                    mainnet: Some(cached_block_numbers.mainnet.clone()),
                    testnet: None,
                }));
            }
        }
        None => {
            return Ok(Json(BlockNumbersResponse {
                mainnet: Some(cached_block_numbers.mainnet.clone()),
                testnet: Some(cached_block_numbers.testnet.clone()),
            }));
        }
    }
}
