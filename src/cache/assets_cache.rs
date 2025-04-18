use std::{collections::HashMap, sync::Arc};

use crate::{models::assets::Config, services::assets::NetworkResponse, utils::load_config};

pub struct AssetsCache {
    pub testnet_assets: Arc<HashMap<String, NetworkResponse>>,
    pub mainnet_assets: Arc<HashMap<String, NetworkResponse>>,
}

impl Default for AssetsCache {
    fn default() -> Self {
        let mut mainnet_assets = HashMap::new();
        let mut testnet_assets = HashMap::new();
        let configs: Vec<Config> = load_config();
        for config in configs {
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
            if config.network_type == "testnet" {
                testnet_assets = response;
            } else if config.network_type == "mainnet" {
                mainnet_assets = response;
            }
        }
        AssetsCache {
            testnet_assets: Arc::new(testnet_assets),
            mainnet_assets: Arc::new(mainnet_assets),
        }
    }
}
