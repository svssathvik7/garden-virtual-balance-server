use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

use serde_json::json;
use tokio::{sync::Mutex, time};

use crate::{models::assets::Config, utils::load_config};

#[derive(Clone)]
pub struct BlockNumbers {
    pub rpcs: HashMap<String, Vec<String>>,
    pub mainnet: HashMap<String, u64>,
    pub testnet: HashMap<String, u64>,
    pub client: reqwest::Client,
}

#[derive(PartialEq)]
pub enum NetworkType {
    MAINNET,
    TESTNET,
}

#[derive(PartialEq)]
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
            return *self.mainnet.get(&chain).unwrap_or(&0);
        } else {
            return *self.testnet.get(&chain).unwrap_or(&0);
        }
    }
    pub async fn start_cron(block_numbers: Arc<Mutex<BlockNumbers>>) {
        println!("Cron serviced!");
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;
            println!("Triggering cron");

            {
                let mut lock = block_numbers.lock().await;
                lock.update_block_numbers().await;
            }

            println!("Finished blocknumbers cron");
        }
    }

    pub async fn update_block_numbers(&mut self) {
        for data in self.testnet.clone() {
            let chain = data.0.clone();
            let blocknumber = self
                .get_chain_blocknumber(chain.clone(), NetworkType::TESTNET)
                .await;
            self.testnet.insert(chain, blocknumber);
        }
        for data in self.mainnet.clone() {
            let chain = data.0.clone();
            let blocknumber = self
                .get_chain_blocknumber(chain.clone(), NetworkType::TESTNET)
                .await;
            self.mainnet.insert(chain, blocknumber);
        }
    }
    pub async fn get_btc_block_number(&self, rpc: String) -> Result<u64, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let endpoint = format!("{}blocks/tip/height", rpc);
        let response = client.get(endpoint).send().await?;
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
        let mut blocknumbers = BlockNumbers {
            rpcs: HashMap::new(),
            mainnet: HashMap::new(),
            testnet: HashMap::new(),
            client: reqwest::Client::new(),
        };
        let mut rpcs = HashMap::new();
        let config = load_config();
        let configs: Vec<Config> = vec![config.mainnet, config.testnet]
            .into_iter()
            .flatten()
            .collect();
        for config in configs {
            if config.network_type == "testnet" {
                for data in config.blockchain.testnet {
                    rpcs.insert(data.0.clone(), data.1.rpc);
                    blocknumbers.testnet.insert(data.0, 0);
                }
            } else if config.network_type == "mainnet" {
                for data in config.blockchain.mainnet {
                    rpcs.insert(data.0.clone(), data.1.rpc);
                    blocknumbers.mainnet.insert(data.0, 0);
                }
            }
        }
        blocknumbers.rpcs = rpcs;
        blocknumbers
    }
}
