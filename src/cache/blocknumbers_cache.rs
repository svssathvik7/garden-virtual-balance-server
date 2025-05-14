use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

use moka::future::{Cache, CacheBuilder};
use serde_json::json;
use tokio::time;

use crate::{
    models::assets::{Network, NetworkType},
    utils::load_config,
};
pub struct BlockNumbers {
    pub rpcs: Arc<HashMap<String, Vec<String>>>,
    pub mainnet: Cache<String, u64>,
    pub testnet: Cache<String, u64>,
    pub localnet: Cache<String, u64>,
    pub client: reqwest::Client,
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
    pub async fn new() -> Self {
        let testnet = CacheBuilder::new(100).build();
        let mainnet = CacheBuilder::new(100).build();
        let localnet = CacheBuilder::new(100).build();
        let mut rpcs = HashMap::new();
        let configs: Vec<HashMap<String, Network>> = load_config();
        for config in configs {
            for (identifier, config) in config {
                match config.network_type {
                    NetworkType::TESTNET => {
                        testnet.insert(identifier.clone(), 0).await;
                        rpcs.insert(identifier.clone(), config.rpcs.clone());
                    }
                    NetworkType::MAINNET => {
                        mainnet.insert(identifier.clone(), 0).await;
                        rpcs.insert(identifier.clone(), config.rpcs.clone());
                    }
                    NetworkType::LOCALNET => {
                        localnet.insert(identifier.clone(), 0).await;
                        rpcs.insert(identifier.clone(), config.rpcs.clone());
                    }
                }
            }
        }
        BlockNumbers {
            rpcs: Arc::new(rpcs),
            mainnet,
            testnet,
            localnet,
            client: reqwest::Client::new(),
        }
    }
    pub async fn get_chain_type(&self, chain: Arc<String>) -> SupportedChains {
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
    pub async fn get_chain_blocknumber(
        &self,
        chain: Arc<String>,
        network_type: NetworkType,
    ) -> u64 {
        let chain_name = self.get_chain_type(chain.clone()).await;
        let rpcs = self.rpcs.get(&*chain).unwrap();
        match chain_name {
            SupportedChains::BITCOIN => {
                for rpc in rpcs {
                    match self.get_btc_block_number(rpc.to_string()).await {
                        Ok(blocknumber) => {
                            return blocknumber;
                        }
                        Err(e) => {
                            eprintln!("Error fetching block number chain: {} {}", chain, e);
                            continue;
                        }
                    };
                }
            }
            SupportedChains::ARBITRUM => {
                for rpc in rpcs {
                    let result = if network_type == NetworkType::LOCALNET {
                        self.fetch_ethereum_block_number(&rpc.to_string()).await
                    } else {
                        self.fetch_arbitrum_l1_block_number(&rpc.to_string()).await
                    };

                    match result {
                        Ok(blocknumber) => return blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number chain: {} {}", chain, e);
                            continue;
                        }
                    };
                }
            }
            SupportedChains::STARKNET => {
                for rpc in rpcs {
                    match self.fetch_starknet_block_number(&rpc.to_string()).await {
                        Ok(blocknumber) => {
                            return blocknumber;
                        }
                        Err(e) => {
                            eprintln!("Error fetching block number chain: {} {}", chain, e);
                            continue;
                        }
                    };
                }
            }
            SupportedChains::SOLANA => {
                for rpc in rpcs {
                    match self.fetch_solana_block_number(&rpc.to_string()).await {
                        Ok(blocknumber) => return blocknumber,
                        Err(e) => {
                            eprintln!("Error fetching block number chain: {} {}", chain, e);
                            continue;
                        }
                    };
                }
            }
            _ => {
                for rpc in rpcs {
                    println!("Fetching block number for chain: {}", chain);
                    match self.fetch_ethereum_block_number(&rpc.to_string()).await {
                        Ok(blocknumber) => {
                            return blocknumber;
                        }
                        Err(e) => {
                            eprintln!("Error fetching block number chain: {} {}", chain, e);
                            continue;
                        }
                    };
                }
            }
        }
        // fallback on failure to fetch blocknumber is to return the last successfull fetched value, if there is no value, return 0
        if network_type == NetworkType::MAINNET {
            return self.mainnet.get(&*chain).await.unwrap_or(0);
        } else if network_type == NetworkType::TESTNET {
            return self.testnet.get(&*chain).await.unwrap_or(0);
        } else {
            return self.localnet.get(&*chain).await.unwrap_or(0);
        }
    }

    pub async fn start_cron(&self) {
        let mut interval = time::interval(Duration::from_secs(5));
        interval.tick().await;

        loop {
            interval.tick().await;
            println!("Fetching block numbers for all chains");

            self.update_block_numbers().await;

            println!("Finished fetching block numbers for all chains");
        }
    }

    pub async fn update_block_numbers(&self) {
        // Create three separate futures for each network type
        let mainnet_future = async {
            let mut futures = Vec::new();
            println!("Fetching MAINNET blocknumbers");
            for data in self.mainnet.iter() {
                let chain = data.0.clone();

                // Spawn a task for each chain in mainnet
                futures.push(async move {
                    let blocknumber = self
                        .get_chain_blocknumber(chain.clone(), NetworkType::MAINNET)
                        .await;
                    (chain, blocknumber)
                });
            }
            // Wait for all mainnet chain updates to complete
            let results = futures::future::join_all(futures).await;
            for (chain, blocknumber) in results {
                self.mainnet.insert((*chain).clone(), blocknumber).await;
            }
        };

        let testnet_future = async {
            let mut futures = Vec::new();
            println!("Fetching TESTNET blocknumbers");
            for data in self.testnet.iter() {
                let chain = data.0.clone();
                // Spawn a task for each chain in testnet
                futures.push(async move {
                    let blocknumber = self
                        .get_chain_blocknumber(chain.clone(), NetworkType::TESTNET)
                        .await;
                    (chain, blocknumber)
                });
            }
            // Wait for all testnet chain updates to complete
            let results = futures::future::join_all(futures).await;
            for (chain, blocknumber) in results {
                self.testnet.insert((*chain).clone(), blocknumber).await;
            }
        };

        let localnet_future = async {
            let mut futures = Vec::new();
            println!("Fetching LOCALNET blocknumbers");
            for data in self.localnet.iter() {
                let chain = data.0.clone();
                // Spawn a task for each chain in localnet
                futures.push(async move {
                    let blocknumber = self
                        .get_chain_blocknumber(chain.clone(), NetworkType::LOCALNET)
                        .await;
                    (chain, blocknumber)
                });
            }
            // Wait for all localnet chain updates to complete
            let results = futures::future::join_all(futures).await;
            for (chain, blocknumber) in results {
                self.localnet.insert((*chain).clone(), blocknumber).await;
            }
        };

        // Execute all three network futures concurrently
        futures::join!(mainnet_future, testnet_future, localnet_future);
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
