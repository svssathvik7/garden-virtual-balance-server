use std::{collections::HashMap, error::Error, fs};

use alloy::signers::k256::elliptic_curve::rand_core::block;
use serde_json::json;

use crate::{models::assets::Config, services::assets::NetworkResponse};

use super::assets_cache::AssetsCache;

#[derive(Clone)]
pub struct BlockNumbers {
    pub rpcs: HashMap<String, Vec<String>>,
    pub mainnet: HashMap<String, u64>,
    pub testnet: HashMap<String, u64>,
    pub client: reqwest::Client,
}

impl BlockNumbers {
    pub async fn cron(&mut self) {
        let interval = 5;
        loop {
            self.update_block_numbers().await;
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
        }
    }
    pub async fn update_block_numbers(&mut self) {
        for data in self.testnet.clone() {
            let chain = data.0.clone();
            if chain.contains("bitcoin") {
                let mut blocknumber = 0;
                let rpcs = self.rpcs.get(&chain.clone()).unwrap();
                for rpc in rpcs {
                    blocknumber = match self.get_btc_block_number(rpc.to_string()).await {
                        Ok(blocknumber) => blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
                self.testnet.insert(chain, blocknumber);
            } else if chain.contains("arbitrum") {
                let mut blocknumber = 0;
                let rpcs = self.rpcs.get(&chain.clone()).unwrap();
                for rpc in rpcs {
                    blocknumber = match self.fetch_arbitrum_l1_block_number(&rpc.to_string()).await
                    {
                        Ok(blocknumber) => blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
                self.testnet.insert(chain, blocknumber);
            } else if chain.contains("starknet") {
                let mut blocknumber = 0;
                let rpcs = self.rpcs.get(&chain.clone()).unwrap();
                for rpc in rpcs {
                    blocknumber = match self.fetch_starknet_block_number(&rpc.to_string()).await {
                        Ok(blocknumber) => blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
                self.testnet.insert(chain, blocknumber);
            } else {
                let mut blocknumber = 0;
                let rpcs = self.rpcs.get(&chain.clone()).unwrap();
                for rpc in rpcs {
                    blocknumber = match self.fetch_ethereum_block_number(&rpc.to_string()).await {
                        Ok(blocknumber) => blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
                self.testnet.insert(chain, blocknumber);
            }
        }
        for data in self.mainnet.clone() {
            let chain = data.0.clone();
            if chain.contains("bitcoin") {
                let mut blocknumber = 0;
                let rpcs = self.rpcs.get(&chain.clone()).unwrap();
                for rpc in rpcs {
                    blocknumber = match self.get_btc_block_number(rpc.to_string()).await {
                        Ok(blocknumber) => blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
                self.testnet.insert(chain, blocknumber);
            } else if chain.contains("arbitrum") {
                let mut blocknumber = 0;
                let rpcs = self.rpcs.get(&chain.clone()).unwrap();
                for rpc in rpcs {
                    blocknumber = match self.fetch_arbitrum_l1_block_number(&rpc.to_string()).await
                    {
                        Ok(blocknumber) => blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
                self.testnet.insert(chain, blocknumber);
            } else if chain.contains("starknet") {
                let mut blocknumber = 0;
                let rpcs = self.rpcs.get(&chain.clone()).unwrap();
                for rpc in rpcs {
                    blocknumber = match self.fetch_starknet_block_number(&rpc.to_string()).await {
                        Ok(blocknumber) => blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
                self.testnet.insert(chain, blocknumber);
            } else if chain.contains("solana") {
                let mut blocknumber = 0;
                let rpcs = self.rpcs.get(&chain.clone()).unwrap();
                for rpc in rpcs {
                    blocknumber = match self.fetch_solana_block_number(&rpc.to_string()).await {
                        Ok(blocknumber) => blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
                self.testnet.insert(chain, blocknumber);
            } else {
                let mut blocknumber = 0;
                let rpcs = self.rpcs.get(&chain.clone()).unwrap();
                for rpc in rpcs {
                    blocknumber = match self.fetch_ethereum_block_number(&rpc.to_string()).await {
                        Ok(blocknumber) => blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number: {}", e);
                            continue;
                        }
                    };
                }
                self.testnet.insert(chain, blocknumber);
            }
        }
    }
    pub async fn get_btc_block_number(&self, rpc: String) -> Result<u64, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let endpoint = format!("{}/blocks/tip/height", rpc);
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
        let config_files = vec!["./mainnetconfig.json", "./testnetconfig.json"];
        for config_file in config_files {
            let config_str = fs::read_to_string(config_file)
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .unwrap();
            let config: Config = serde_json::from_str(&config_str)
                .map_err(|e| {
                    eprintln!("Error parsing {}: {:?}", config_file, e);
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                })
                .unwrap();
            if config_file.contains("testnet") {
                for data in config.blockchain.testnet {
                    rpcs.insert(data.0.clone(), data.1.rpc);
                    blocknumbers.testnet.insert(data.0, 0);
                }
            } else if config_file.contains("mainnet") {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::cache::blocknumbers_cache::BlockNumbers;
}
