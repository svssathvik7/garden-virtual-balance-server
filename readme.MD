# Info v2 server

## Endpoints:

- ### /assets

Returns the list of both garden supported mainnet and testnet assets.

- ### /assets/testnet

Returns the list of testnet assets supported by garden.

- ### /assets/mainnet

Returns the list of mainnet assets supported by garden.

- ### /blocknumbers (previously /blocknumber)

Returns the current blocknumbers of each supported chain (both mainnet and testnet).

- ### /blocknumbers/testnet

Returns the current blocknumbers of each testnet chain supported by garden.

- ### /blocknumbers/mainnet

Returns the current blocknumbers of each mainnet chain supported by garden.

## Schemas

- ### /assets/:chain

```json
{
  "arbitrum_sepolia": {
    "chainId": "421614",
    "fillerAddresses": ["0x0205ed605dc5f032be4c709e01dd1f38ddfc3beb"],
    "networkLogo": "https://garden-finance.imgix.net/chain_images/arbitrumSepolia.svg",
    "explorer": "https://sepolia.arbiscan.io",
    "networkType": "testnet",
    "name": "Arbitrum Sepolia",
    "assetConfig": [
      {
        "name": "Wrapped Bitcoin",
        "decimals": 8,
        "symbol": "WBTC",
        "baseFees": 0,
        "logo": "https://garden-finance.imgix.net/token-images/wbtc.svg",
        "coinGeckoId": "wrapped-bitcoin",
        "tokenAddress": "0xD8a6E3FCA403d79b6AD6216b60527F51cc967D39",
        "atomicSwapAddress": "0x795Dcb58d1cd4789169D5F938Ea05E17ecEB68cA",
        "min_amount": "",
        "max_amount": ""
      }
    ],
    "identifier": "arbitrum_sepolia"
  },
  "bitcoin_testnet": {
    "chainId": "3",
    "fillerAddresses": ["tb1q3rfu7wn2m83trw068w78yqe6palnh0zweq3fe3"],
    "networkLogo": "https://garden-finance.imgix.net/token-images/bitcoin.svg",
    "explorer": "https://mempool.space/testnet4/",
    "networkType": "testnet",
    "name": "Bitcoin Testnet",
    "assetConfig": [
      {
        "name": "Bitcoin",
        "decimals": 8,
        "symbol": "BTC",
        "baseFees": 1000,
        "logo": "https://garden-finance.imgix.net/token-images/bitcoin.svg",
        "coinGeckoId": "bitcoin",
        "tokenAddress": "primary",
        "atomicSwapAddress": "primary",
        "min_amount": "",
        "max_amount": ""
      }
    ],
    "identifier": "bitcoin_testnet"
  }
}
```

- ### /blocknumbers/:chain

```json
{
  "mainnet": {
    "ethereum": 22279307,
    "arbitrum": 22279306,
    "bitcoin": 892631,
    "base": 28995039,
    "bera": 3766061
  },
  "testnet": {
    "arbitrum_sepolia": 8128152,
    "bitcoin_testnet": 77684
  }
}
```
