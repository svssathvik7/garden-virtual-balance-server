use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::{appstate::AppState, cache::blocknumbers_cache::NetworkType, models::assets::Asset};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetData {
    pub networks: HashMap<String, NetworkResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkResponse {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "networkLogo")]
    pub network_logo: String,
    pub explorer: String,
    #[serde(rename = "networkType")]
    pub network_type: NetworkType,
    pub name: String,
    #[serde(rename = "assetConfig")]
    pub asset_config: Vec<Asset>,
    pub identifier: String,
}

pub async fn get_assets(
    State(appstate): State<Arc<AppState>>,
    network_type: Option<Path<NetworkType>>,
) -> Result<axum::Json<HashMap<String, NetworkResponse>>, axum::http::StatusCode> {
    let mut response = HashMap::new();
    let cached_assets = appstate.cached_assets.clone();
    match network_type {
        Some(Path(network_type)) => match network_type {
            NetworkType::TESTNET => {
                response = cached_assets.testnet_assets.clone().deref().to_owned();
            }
            NetworkType::MAINNET => {
                response = cached_assets.mainnet_assets.clone().deref().to_owned();
            }
            NetworkType::LOCALNET => {
                response = cached_assets.localnet_assets.clone().deref().to_owned();
            }
        },
        None => {
            response = cached_assets.mainnet_assets.clone().deref().to_owned();
            response.extend(cached_assets.testnet_assets.clone().deref().to_owned());
            response.extend(cached_assets.localnet_assets.clone().deref().to_owned());
        }
    }
    Ok(axum::Json(response))
}
