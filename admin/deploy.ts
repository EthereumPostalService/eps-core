import { exit } from "process";
import { parse } from "ts-command-line-args";
import NetworkConfig from "./network-config";
import { ContractFactory, JsonRpcProvider, Wallet } from "ethers";
import { abi as EPSCoreAbi, bytecode as EPSCoreBytecode } from "../contracts/out/EthereumPostalService.sol/EthereumPostalService.json";
import { abi as ChainlinkPostageModuleAbi, bytecode as ChainlinkPostageModuleBytecode } from "../contracts/out/ChainlinkPostagePriceModule.sol/ChainlinkPostagePriceModule.json";

interface Args {
    network: string,
    pk: string,
    postage_usd: string,
    encryption_pub_key: string
}

export async function deploy() {
    const args = parse<Args>({
        network: String,
        pk: String,
        postage_usd: String,
        encryption_pub_key: String,
    });

    let rpcUrl = NetworkConfig.getRpc(args.network);
    if (rpcUrl === undefined) {
        console.error(`Could not find config for ${args.network}`);
        exit(-1);
    }

    let chainlinkEthUsd = NetworkConfig.getChainlinkEthUsd(args.network);
    if (chainlinkEthUsd === undefined) {
        console.error(`Could not find chainlink ETH / USD address for ${args.network}`);
        exit(-1);
    }

    let postageUsdDecimals: number = parseFloat(args.postage_usd);
    if (postageUsdDecimals > 10 || postageUsdDecimals < 0.25) {
        console.error(`PostageUSD outside of expected range ${postageUsdDecimals}`);
        exit(-1);
    }
    let postageBigDecimals = BigInt(postageUsdDecimals * (10 ** 4)) * (10n ** 14n);

    let provider = new JsonRpcProvider(rpcUrl!);
    let signer = new Wallet(args.pk, provider);
    console.log(`Connected to local wallet with address: ${await signer.getAddress()}`);


    let epsFactory = new ContractFactory(EPSCoreAbi, EPSCoreBytecode, signer);
    let chainlinkModuleFactory = new ContractFactory(ChainlinkPostageModuleAbi, ChainlinkPostageModuleBytecode, signer);

    let deployChainlink = await chainlinkModuleFactory.deploy(chainlinkEthUsd, postageBigDecimals);
    await deployChainlink.waitForDeployment();
    let chainlinkPostagePriceModuleAddr = await deployChainlink.getAddress();
    console.log(`ChainlinkPostagePriceModule deployed to: ${chainlinkPostagePriceModuleAddr}`);

    let deployEps = await epsFactory.deploy(chainlinkPostagePriceModuleAddr, args.encryption_pub_key);
    await deployEps.waitForDeployment();
    let addr = await deployEps.getAddress();
    console.log(`EthereumPostalService deployed to: ${addr}`);
}

deploy().then().catch(err => {
    console.error(err);
    exit(-1);
})