use axum::extract::Path;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::models::{
    apiresponse::ApiResponse,
    assets::{Asset, Config},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetData {
    pub networks: HashMap<String, NetworkResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    Path(network_type): Path<String>,
) -> Result<axum::Json<ApiResponse<HashMap<String, NetworkResponse>>>, axum::http::StatusCode> {
    let config_file = match network_type.as_str() {
        "mainnet" => "./mainnetconfig.json",
        "testnet" => "./testnetconfig.json",
        _ => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    let config_str = fs::read_to_string(config_file)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let config: Config = serde_json::from_str(&config_str).map_err(|e| {
        eprintln!("Error parsing: {:?}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut response: HashMap<String, NetworkResponse> = HashMap::new();
    for (identifier, network) in config.networks.iter() {
        let network_data = NetworkResponse {
            chain_id: network.chain_id.clone(),
            filler_addresses: network.filler_addresses.clone(),
            network_logo: network.network_logo.clone(),
            explorer: network.explorer.clone(),
            network_type: network.network_type.clone(),
            name: network.name.clone(),
            asset_config: network.asset_config.clone(),
            identifier: identifier.clone(),
        };
        response.insert(identifier.clone(), network_data);
    }

    Ok(axum::Json(ApiResponse { data: response }))
}
