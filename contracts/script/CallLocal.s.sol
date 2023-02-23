// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Script.sol";
import "../src/EthereumPostalService.sol";
import "../src/test/FakeChainlink.sol";

contract CallLocal is Script {
    uint256 public constant COST = 5 * (10 ** 18);

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("ANVIL_PK");
        vm.startBroadcast(deployerPrivateKey);

        EthereumPostalService eps = EthereumPostalService(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0);
        uint256 postageCost = eps.getPostageWei();
        EthereumPostalService.PostalAddress memory addr =
            EthereumPostalService.PostalAddress("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler");
        eps.sendMail{value: postageCost}(addr, "Heeeey bro");

        vm.stopBroadcast();
    }
}
