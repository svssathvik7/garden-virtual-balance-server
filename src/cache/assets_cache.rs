use std::{collections::HashMap, sync::Arc};

use crate::{handlers::assets::NetworkResponse, models::assets::Network, utils::load_config};

pub struct AssetsCache {
    pub testnet_assets: Arc<HashMap<String, NetworkResponse>>,
    pub mainnet_assets: Arc<HashMap<String, NetworkResponse>>,
    pub localnet_assets: Arc<HashMap<String, NetworkResponse>>,
}

impl AssetsCache {
    pub fn new() -> Self {
        let mut mainnet_assets = HashMap::new();
        let mut testnet_assets = HashMap::new();
        let mut localnet_assets = HashMap::new();
        let configs: Vec<HashMap<String, Network>> = load_config();

        for config in configs {
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
                } else if network.network_type == "localnet" {
                    localnet_assets.insert(identifier.clone(), network_data);
                }
            }
        }

        println!("Localnet config {:?}", localnet_assets.len());

        AssetsCache {
            testnet_assets: Arc::new(testnet_assets),
            mainnet_assets: Arc::new(mainnet_assets),
            localnet_assets: Arc::new(localnet_assets),
        }
    }
}
