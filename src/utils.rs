use std::fs;

use serde::{Deserialize, Serialize};

use crate::models::assets::Config;

#[derive(Deserialize, Serialize)]
pub struct ConfigData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainnet: Option<Config>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testnet: Option<Config>,
}
pub fn load_config() -> ConfigData {
    let config_file = "config.json";
    let config_str = fs::read_to_string(config_file).unwrap();
    let config: ConfigData = serde_json::from_str(&config_str).unwrap();
    config
}
