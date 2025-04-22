use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

use serde_json::json;
use tokio::{sync::RwLock, time};

use crate::{models::assets::Network, utils::load_config};

#[derive(Clone)]
pub struct UpdateBlockNumberResponse {
    pub mainnet: HashMap<String, u64>,
    pub testnet: HashMap<String, u64>,
}

pub struct BlockNumbers {
    pub rpcs: Arc<HashMap<String, Vec<String>>>,
    pub mainnet: RwLock<HashMap<String, u64>>,
    pub testnet: RwLock<HashMap<String, u64>>,
    pub client: reqwest::Client,
}

#[derive(PartialEq)]
pub enum NetworkType {
    MAINNET,
    TESTNET,
}

#[derive(PartialEq, Debug)]
pub enum SupportedChains {
    ETHEREUM,
    ARBITRUM,
    SOLANA,
    STARKNET,
    BITCOIN,
}

impl BlockNumbers {
    pub async fn get_chain_type(&self, chain: String) -> SupportedChains {
        if chain.contains("arbitrum") {
            SupportedChains::ARBITRUM
        } else if chain.contains("solana") {
            SupportedChains::SOLANA
        } else if chain.contains("starknet") {
            SupportedChains::STARKNET
        } else if chain.contains("bitcoin") {
            SupportedChains::BITCOIN
        } else {
            SupportedChains::ETHEREUM
        }
    }
    pub async fn get_chain_blocknumber(&self, chain: String, network_type: NetworkType) -> u64 {
        let chain_name = self.get_chain_type(chain.clone()).await;
        let rpcs = self.rpcs.get(&chain.clone()).unwrap();
        match chain_name {
            SupportedChains::BITCOIN => {
                for rpc in rpcs {
                    match self.get_btc_block_number(rpc.to_string()).await {
                        Ok(blcknumber) => {
                            return blcknumber;
                        }
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
            }
            SupportedChains::ARBITRUM => {
                for rpc in rpcs {
                    match self.fetch_arbitrum_l1_block_number(&rpc.to_string()).await {
                        Ok(blcknumber) => {
                            return blcknumber;
                        }
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
            }
            SupportedChains::STARKNET => {
                for rpc in rpcs {
                    match self.fetch_starknet_block_number(&rpc.to_string()).await {
                        Ok(blcknumber) => {
                            return blcknumber;
                        }
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
            }
            SupportedChains::SOLANA => {
                for rpc in rpcs {
                    match self.fetch_solana_block_number(&rpc.to_string()).await {
                        Ok(blcknumber) => return blcknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
            }
            _ => {
                for rpc in rpcs {
                    match self.fetch_ethereum_block_number(&rpc.to_string()).await {
                        Ok(blcknumber) => {
                            return blcknumber;
                        }
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
            }
        }
        // fallback on failure to fetch blocknumber is to return the last successfull fetched value, if there is no value, return 0
        if network_type == NetworkType::MAINNET {
            return *self.mainnet.read().await.get(&chain).unwrap_or(&0);
        } else {
            return *self.testnet.read().await.get(&chain).unwrap_or(&0);
        }
    }
    pub async fn write_blocknumbers(&self, updated_block_numbers: UpdateBlockNumberResponse) {
        {
            let mut mainnet_guard = self.mainnet.write().await;
            *mainnet_guard = updated_block_numbers.mainnet;
        }
        {
            let mut testnet_guard = self.testnet.write().await;
            *testnet_guard = updated_block_numbers.testnet;
        }
    }

    pub async fn start_cron(block_numbers: Arc<BlockNumbers>) {
        let mut interval = time::interval(Duration::from_secs(5));
        interval.tick().await;

        loop {
            interval.tick().await;
            println!("Fetching block numbers for all chains");

            let updated_block_numbers = block_numbers.update_block_numbers().await;
            block_numbers
                .write_blocknumbers(updated_block_numbers)
                .await;

            println!("Finished fetching block numbers for all chains");
        }
    }

    pub async fn update_block_numbers(&self) -> UpdateBlockNumberResponse {
        let mut testnet = HashMap::new();
        let mut mainnet = HashMap::new();
        for data in self.testnet.read().await.clone() {
            let chain = data.0.clone();
            let blocknumber = self
                .get_chain_blocknumber(chain.clone(), NetworkType::TESTNET)
                .await;
            testnet.insert(chain, blocknumber);
        }
        for data in self.mainnet.read().await.clone() {
            let chain = data.0.clone();
            let blocknumber = self
                .get_chain_blocknumber(chain.clone(), NetworkType::TESTNET)
                .await;
            mainnet.insert(chain, blocknumber);
        }
        return UpdateBlockNumberResponse { mainnet, testnet };
    }
    pub async fn get_btc_block_number(&self, rpc: String) -> Result<u64, Box<dyn Error>> {
        let endpoint = format!("{}blocks/tip/height", rpc);
        let response = self.client.get(endpoint).send().await?;
        let block_number = response.text().await.unwrap().parse()?;
        println!("BTC block num - {:?}", block_number);
        Ok(block_number)
    }

    pub async fn fetch_ethereum_block_number(&self, rpc_url: &str) -> Result<u64, Box<dyn Error>> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_blockNumber",
            "params": []
        });

        let res: serde_json::Value = self
            .client
            .post(rpc_url)
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        let hex_str = res["result"]
            .as_str()
            .ok_or("Invalid eth_blockNumber response")?;

        Ok(u64::from_str_radix(&hex_str.trim_start_matches("0x"), 16)?)
    }

    pub async fn fetch_arbitrum_l1_block_number(
        &self,
        rpc_url: &str,
    ) -> Result<u64, Box<dyn Error>> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getBlockByNumber",
            "params": ["latest", false]
        });

        let res: serde_json::Value = self
            .client
            .post(rpc_url)
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        let l1_block_number = res["result"]["l1BlockNumber"]
            .as_str()
            .ok_or("Missing l1BlockNumber")?;

        Ok(u64::from_str_radix(
            &l1_block_number.trim_start_matches("0x"),
            16,
        )?)
    }

    pub async fn fetch_solana_block_number(&self, rpc_url: &str) -> Result<u64, Box<dyn Error>> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSlot",
            "params": [{ "commitment": "confirmed" }]
        });

        let res: serde_json::Value = self
            .client
            .post(rpc_url)
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(res["result"].as_u64().ok_or("Invalid getSlot response")?)
    }

    pub async fn fetch_starknet_block_number(&self, rpc_url: &str) -> Result<u64, Box<dyn Error>> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "starknet_blockNumber",
            "params": []
        });

        let res: serde_json::Value = self
            .client
            .post(rpc_url)
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        match &res["result"] {
            serde_json::Value::Number(n) => Ok(n.as_u64().ok_or("Invalid number")?),
            _ => Err("Unexpected starknet block number format".into()),
        }
    }
}

impl Default for BlockNumbers {
    fn default() -> Self {
        let mut testnet = HashMap::new();
        let mut mainnet = HashMap::new();
        let mut rpcs = HashMap::new();
        let configs: Vec<HashMap<String, Network>> = load_config();
        for config in configs {
            for (identifier, config) in config {
                if config.network_type == "testnet" {
                    testnet.insert(identifier.clone(), 0);
                    rpcs.insert(identifier.clone(), config.rpcs.clone());
                } else if config.network_type == "mainnet" {
                    mainnet.insert(identifier.clone(), 0);
                    rpcs.insert(identifier.clone(), config.rpcs.clone());
                }
            }
        }
        BlockNumbers {
            rpcs: Arc::new(rpcs),
            mainnet: RwLock::new(mainnet),
            testnet: RwLock::new(testnet),
            client: reqwest::Client::new(),
        }
    }
}
