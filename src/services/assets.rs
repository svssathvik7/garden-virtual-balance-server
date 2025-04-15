use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use crate::{appstate::AppState, models::assets::Asset};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetData {
    pub networks: HashMap<String, NetworkResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkResponse {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "fillerAddresses")]
    pub filler_addresses: Vec<String>,
    #[serde(rename = "networkLogo")]
    pub network_logo: String,
    pub explorer: String,
    #[serde(rename = "networkType")]
    pub network_type: String,
    pub name: String,
    #[serde(rename = "assetConfig")]
    pub asset_config: Vec<Asset>,
    pub identifier: String,
}

pub async fn get_assets(
    State(appstate): State<Arc<AppState>>,
    network_type: Option<Path<String>>,
) -> Result<axum::Json<HashMap<String, NetworkResponse>>, axum::http::StatusCode> {
    let mut response = HashMap::new();
    let cached_assets = appstate.cached_assets.clone();
    match network_type {
        Some(Path(network_type)) => {
            if network_type == "testnet" {
                response = cached_assets.testnet_assets.clone();
            } else if network_type == "mainnet" {
                response = cached_assets.mainnet_assets.clone();
            }
        }
        None => {
            response = cached_assets.mainnet_assets.clone();
            response.extend(cached_assets.testnet_assets.clone());
        }
    }
    Ok(axum::Json(response))
}
