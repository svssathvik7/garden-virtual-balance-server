use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cache::blocknumbers_cache::NetworkType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "networkLogo")]
    pub network_logo: String,
    pub explorer: String,
    #[serde(rename = "networkType")]
    pub network_type: NetworkType,
    pub name: String,
    #[serde(rename = "assetConfig")]
    pub asset_config: Vec<Asset>,
    #[serde(skip_serializing)]
    pub rpcs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub name: String,
    pub decimals: u8,
    pub symbol: String,
    pub logo: String,
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "atomicSwapAddress")]
    pub atomic_swap_address: String,
    #[serde(default)]
    pub min_amount: String,
    #[serde(default)]
    pub max_amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainnet: Option<HashMap<String, NetworkRpc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testnet: Option<HashMap<String, NetworkRpc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localnet: Option<HashMap<String, NetworkRpc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkRpc {
    pub rpc: Vec<String>,
}
