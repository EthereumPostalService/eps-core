// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Script.sol";
import "../src/EthereumPostalService.sol";
import "../src/ChainlinkPostagePriceModule.sol";
import "../test/FakeChainlink.sol";

contract DeployLocal is Script {
    uint256 public constant COST = 5 * (10 ** 18);

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("ANVIL_PK");
        vm.startBroadcast(deployerPrivateKey);

        FakeChainlink fakeChainlink = new FakeChainlink();
        ChainlinkPostagePriceModule postagePriceModule =
            new ChainlinkPostagePriceModule(AggregatorV3Interface(fakeChainlink), COST);
        EthereumPostalService eps = new EthereumPostalService(IPostagePriceModule(postagePriceModule), 
        hex"022c5e3d3c0ce6ab0763aa539725a64ebe642431609987ee6a0a9735f2c18ba290");

        console.log("FakeChainlink:", address(fakeChainlink));
        console.log("ChainlinkPostagePriceModule:", address(postagePriceModule));
        console.log("EthereumPostalService:", address(eps));

        vm.stopBroadcast();
    }
}
