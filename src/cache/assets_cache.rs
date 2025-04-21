use std::{collections::HashMap, sync::Arc};

use crate::{models::assets::Network, services::assets::NetworkResponse, utils::load_config};

pub struct AssetsCache {
    pub testnet_assets: Arc<HashMap<String, NetworkResponse>>,
    pub mainnet_assets: Arc<HashMap<String, NetworkResponse>>,
}

impl Default for AssetsCache {
    fn default() -> Self {
        let mut mainnet_assets = HashMap::new();
        let mut testnet_assets = HashMap::new();
        let configs: HashMap<String, Network> = load_config();

        for (identifier, network) in configs {
            let network_data = NetworkResponse {
                chain_id: network.chain_id.clone(),
                network_logo: network.network_logo.clone(),
                explorer: network.explorer.clone(),
                network_type: network.network_type.clone(),
                name: network.name.clone(),
                asset_config: network.asset_config.clone(),
                identifier: identifier.clone(),
            };

            if network.network_type == "testnet" {
                testnet_assets.insert(identifier.clone(), network_data);
            } else if network.network_type == "mainnet" {
                mainnet_assets.insert(identifier.clone(), network_data);
            }
        }

        AssetsCache {
            testnet_assets: Arc::new(testnet_assets),
            mainnet_assets: Arc::new(mainnet_assets),
        }
    }
}
