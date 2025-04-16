use std::{collections::HashMap, fs};

use crate::{models::assets::Config, services::assets::NetworkResponse, utils::load_config};

pub struct AssetsCache {
    pub testnet_assets: HashMap<String, NetworkResponse>,
    pub mainnet_assets: HashMap<String, NetworkResponse>,
}

impl Default for AssetsCache {
    fn default() -> Self {
        let config = load_config();
        let mut cached_assets = AssetsCache {
            mainnet_assets: HashMap::new(),
            testnet_assets: HashMap::new(),
        };
        let configs = vec![config.mainnet.unwrap(), config.testnet.unwrap()];
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
                cached_assets.testnet_assets = response;
            } else if config.network_type == "mainnet" {
                cached_assets.mainnet_assets = response;
            }
        }
        cached_assets
    }
}
