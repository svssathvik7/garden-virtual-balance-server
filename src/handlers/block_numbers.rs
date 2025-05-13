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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localnet: Option<HashMap<String, u64>>,
}

pub async fn get_block_numbers(
    State(appstate): State<Arc<AppState>>,
    network_type: Option<Path<String>>,
) -> Result<Json<BlockNumbersResponse>, axum::http::StatusCode> {
    let cached_block_numbers = appstate.block_numbers.clone();

    match network_type {
        Some(Path(network_type)) => {
            if network_type == "testnet" {
                return Ok(Json(BlockNumbersResponse {
                    mainnet: None,
                    testnet: Some(
                        cached_block_numbers
                            .testnet
                            .iter()
                            .map(|entry| ((*entry.0).clone(), entry.1))
                            .collect(),
                    ),
                    localnet: None,
                }));
            } else if network_type == "mainnet" {
                return Ok(Json(BlockNumbersResponse {
                    mainnet: Some(
                        cached_block_numbers
                            .mainnet
                            .iter()
                            .map(|entry| ((*entry.0).clone(), entry.1))
                            .collect(),
                    ),
                    testnet: None,
                    localnet: None,
                }));
            } else if network_type == "localnet" {
                return Ok(Json(BlockNumbersResponse {
                    mainnet: None,
                    testnet: None,
                    localnet: Some(
                        cached_block_numbers
                            .localnet
                            .iter()
                            .map(|entry| ((*entry.0).clone(), entry.1))
                            .collect(),
                    ),
                }));
            } else {
                return Err(axum::http::StatusCode::NOT_FOUND);
            }
        }
        None => {
            return Ok(Json(BlockNumbersResponse {
                mainnet: Some(
                    cached_block_numbers
                        .mainnet
                        .clone()
                        .iter()
                        .map(|entry| ((*entry.0).clone(), entry.1))
                        .collect(),
                ),
                testnet: Some(
                    cached_block_numbers
                        .testnet
                        .iter()
                        .map(|entry| ((*entry.0).clone(), entry.1))
                        .collect(),
                ),
                localnet: Some(
                    cached_block_numbers
                        .localnet
                        .iter()
                        .map(|entry| ((*entry.0).clone(), entry.1))
                        .collect(),
                ),
            }));
        }
    }
}

pub async fn get_block_numbers_by_chain(
    State(appstate): State<Arc<AppState>>,
    network_type: Path<String>,
) -> Result<Json<HashMap<String, u64>>, axum::http::StatusCode> {
    let cached_block_numbers = appstate.block_numbers.clone();
    let network_type = network_type.0;
    if network_type == "testnet" {
        return Ok(Json(
            cached_block_numbers
                .testnet
                .iter()
                .map(|entry| ((*entry.0).clone(), entry.1))
                .collect(),
        ));
    } else if network_type == "mainnet" {
        return Ok(Json(
            cached_block_numbers
                .mainnet
                .iter()
                .map(|entry| ((*entry.0).clone(), entry.1))
                .collect(),
        ));
    } else if network_type == "localnet" {
        return Ok(Json(
            cached_block_numbers
                .mainnet
                .iter()
                .map(|entry| ((*entry.0).clone(), entry.1))
                .collect(),
        ));
    } else {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }
}
