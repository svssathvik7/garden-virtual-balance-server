use std::{collections::HashMap, sync::Arc};

use crate::{models::assets::Network, services::assets::NetworkResponse};

pub struct AssetsCache {
    pub testnet_assets: Arc<HashMap<String, NetworkResponse>>,
    pub mainnet_assets: Arc<HashMap<String, NetworkResponse>>,
}

impl AssetsCache {
    pub fn new(configs: Arc<Vec<HashMap<String, Network>>>) -> Self {
        let mut mainnet_assets = HashMap::new();
        let mut testnet_assets = HashMap::new();

        for config in (*configs).iter() {
            for (identifier, network) in config {
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
        }

        AssetsCache {
            testnet_assets: Arc::new(testnet_assets),
            mainnet_assets: Arc::new(mainnet_assets),
        }
    }
}
