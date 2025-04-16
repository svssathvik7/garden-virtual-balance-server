use core::net;
use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::appstate::AppState;

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
) -> Result<Json<BlockNumbersResponse>, axum::http::StatusCode> {
    let cached_block_numbers = appstate.block_numbers.lock().await;

    match network_type {
        Some(Path(network_type)) => {
            if network_type == "testnet" {
                return Ok(Json(BlockNumbersResponse {
                    mainnet: None,
                    testnet: Some(cached_block_numbers.testnet.clone()),
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

pub async fn get_block_numbers_by_chain(
    State(appstate): State<Arc<AppState>>,
    network_type: Path<String>,
) -> Result<Json<HashMap<String, u64>>, axum::http::StatusCode> {
    let cached_block_numbers = appstate.block_numbers.lock().await;
    let network_type = network_type.0;
    if network_type == "testnet" {
        return Ok(Json(cached_block_numbers.testnet.clone()));
    } else {
        return Ok(Json(cached_block_numbers.mainnet.clone()));
    }
}
