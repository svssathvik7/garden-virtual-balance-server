use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "networkType")]
    pub network_type: String,
    pub orderbook: String,
    pub port: u16,
    #[serde(rename = "baseFeePercent")]
    pub base_fee_percent: f64,
    #[serde(rename = "coingeckoURL")]
    pub coingecko_url: String,
    pub networks: HashMap<String, Network>,
    pub blockchain: BlockchainConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(skip_serializing)]
    // pub rpc: String,  dont send in response to client basically serde ignore this field
    #[serde(rename = "fillerAddresses")]
    pub filler_addresses: Vec<String>,
    #[serde(rename = "networkLogo")]
    pub network_logo: String,
    pub explorer: String,
    #[serde(rename = "networkType")]
    pub network_type: String,
    pub name: String,
    #[serde(rename = "assetConfig")]
    pub asset_config: Vec<Asset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub name: String,
    pub decimals: u8,
    pub symbol: String,
    #[serde(default)]
    #[serde(rename = "baseFees")]
    pub base_fees: u64,
    pub logo: String,
    #[serde(rename = "coinGeckoId")]
    pub coin_gecko_id: String,
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
    pub mainnet: HashMap<String, NetworkRpc>,
    pub testnet: HashMap<String, NetworkRpc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkRpc {
    pub rpc: Vec<String>,
}
