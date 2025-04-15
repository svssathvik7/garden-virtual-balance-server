use std::{collections::HashMap, fs};

use crate::{models::assets::Config, services::assets::NetworkResponse};

pub struct AssetsCache {
    pub testnet_assets: HashMap<String, NetworkResponse>,
    pub mainnet_assets: HashMap<String, NetworkResponse>,
}

impl Default for AssetsCache {
    fn default() -> Self {
        let config_files = vec!["./mainnetconfig.json", "./testnetconfig.json"];
        let mut cached_assets = AssetsCache {
            mainnet_assets: HashMap::new(),
            testnet_assets: HashMap::new(),
        };
        for config_file in config_files {
            let mut response: HashMap<String, NetworkResponse> = HashMap::new();
            let config_str = fs::read_to_string(config_file)
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .unwrap();
            let config: Config = serde_json::from_str(&config_str)
                .map_err(|e| {
                    eprintln!("Error parsing {}: {:?}", config_file, e);
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                })
                .unwrap();

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
            if config_file.contains("testnet") {
                cached_assets.testnet_assets = response;
            } else if config_file.contains("mainnet") {
                cached_assets.mainnet_assets = response;
            }
        }
        cached_assets
    }
}
