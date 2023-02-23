import { exit } from "process";

import 'dotenv/config';

let alchemyApiKey = process.env.ALCHEMY_API_KEY;
if (alchemyApiKey) {
    alchemyApiKey = alchemyApiKey!;
} else {
    console.error("ALCHEMY_API_KEY not set.");
    exit(-1);
}
let infuraApiKey = process.env.INFURA_API_KEY;
if (infuraApiKey) {
    infuraApiKey = infuraApiKey!;
} else {
    console.error("INFURA_API_KEY not set.");
    exit(-1);
}

interface Network {
    name: string,
    rpcUrl: string,
    chainlink_eth_usd?: string
}

let networks: Network[] =
    [
        {
            name: "anvil",
            rpcUrl: "http://localhost:8545",
            chainlink_eth_usd: "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720" // fake
        },
        {
            name: "goerli",
            rpcUrl: `https://eth-goerli.alchemyapi.io/v2/${alchemyApiKey}`,
        },
        {
            name: "sepolia",
            rpcUrl: `https://sepolia.infura.io/v3/${infuraApiKey}`
        },
        {
            name: "mainnet",
            rpcUrl: `https://eth-mainnet.g.alchemy.com/v2/${alchemyApiKey}`,
            chainlink_eth_usd: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419"
        },
        {
            name: "optimism",
            rpcUrl: `https://opt-mainnet.g.alchemy.com/v2/${alchemyApiKey}`,
            chainlink_eth_usd: "0x13e3Ee699D1909E989722E753853AE30b17e08c5"
        }
    ]

export default class NetworkConfig {
    public static getRpc(networkName: string): undefined | string {
        return networks.find(network => network.name.toLowerCase() === networkName.toLowerCase())?.rpcUrl;
    }

    public static getChainlinkEthUsd(networkName: string): undefined | string {
        return networks.find(network => network.name.toLowerCase() === networkName.toLowerCase())?.chainlink_eth_usd;
    }
}