use std::fs;

use serde::{Deserialize, Serialize};

use crate::models::assets::Config;

#[derive(Deserialize, Serialize)]
pub struct ConfigData {
    pub mainnet: Option<Config>,
    pub testnet: Option<Config>,
}
pub fn load_config() -> ConfigData {
    let config_file = "config.json";
    let config_str = fs::read_to_string(config_file).unwrap();
    let config: ConfigData = serde_json::from_str(&config_str).unwrap();
    config
}
