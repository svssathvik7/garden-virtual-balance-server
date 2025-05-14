use std::{collections::HashMap, fs, vec};

use serde::{Deserialize, Serialize};

use crate::models::assets::{Network, NetworkType};

#[derive(Deserialize, Serialize)]
pub struct ConfigData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainnet: Option<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testnet: Option<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localnet: Option<Network>,
}
pub fn load_config() -> HashMap<String, Network> {
    let config_file = "config.json";
    let config_str = match fs::read_to_string(config_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading config file: {}", e);
            return HashMap::new();
        }
    };

    let config: HashMap<String, Network> = match serde_json::from_str(&config_str) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Error parsing config JSON: {}", e);
            return HashMap::new();
        }
    };

    let parsed_config: HashMap<String, Network> = config
        .iter()
        .map(|(key, network)| (key.clone(), network.clone()))
        .collect();

    parsed_config
}
