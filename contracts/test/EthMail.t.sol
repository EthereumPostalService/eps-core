// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Test.sol";
import "../src/EthMail.sol";
import "../src/ChainlinkPostagePriceModule.sol";
import "../src/test/FakeChainlink.sol";

contract EthMailTest is Test {
    address public constant MAINNET_CHAINLINK_ETH_USD = 0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419;
    uint256 public constant COST = 5 * (10 ** 18);

    uint256 mainnetFork;

    FakeChainlink fakeChainlink;
    ChainlinkPostagePriceModule postagePriceModule;
    EthMail ethMail;

    function setUp() public {
        string memory MAINNET_RPC_URL = vm.envString("MAINNET_RPC_URL");
        mainnetFork = vm.createFork(MAINNET_RPC_URL);
        vm.selectFork(mainnetFork);

        fakeChainlink = new FakeChainlink();
        postagePriceModule = new ChainlinkPostagePriceModule(AggregatorV3Interface(fakeChainlink), COST);
        ethMail = new EthMail(IPostagePriceModule(postagePriceModule), "\x00\x01");
    }

    function test_mainnetFork() public {
        ChainlinkPostagePriceModule mPostagePriceModule =
            new ChainlinkPostagePriceModule(AggregatorV3Interface(MAINNET_CHAINLINK_ETH_USD), COST);
        EthMail mEthMail = new EthMail(IPostagePriceModule(mPostagePriceModule), "\x00\x01");
        uint256 postageWei = mEthMail.getPostageWei();

        uint256 upper = 5 * (10 ** 18) / 100;
        uint256 lower = 5 * (10 ** 18) / 10000;

        assertTrue(lower <= postageWei);
        assertTrue(upper >= postageWei);

        EthMail.PostalAddress memory addr =
            EthMail.PostalAddress("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler");
        mEthMail.sendMail{value: postageWei}(addr, "Heeeey bro");
    }

    function test_fakeChainlink() public {
        uint256 postageWei = ethMail.getPostageWei();

        uint256 result = 5 * (10 ** 18) / 1600;
        assertEq(result, postageWei);

        EthMail.PostalAddress memory addr =
            EthMail.PostalAddress("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler");
        ethMail.sendMail{value: postageWei}(addr, "Heeeey bro");
    }

    function test_pause() public {
        uint256 postageWei = ethMail.getPostageWei();

        uint256 result = 5 * (10 ** 18) / 1600;
        assertEq(result, postageWei);

        EthMail.PostalAddress memory addr =
            EthMail.PostalAddress("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler");
        ethMail.togglePause();
        vm.expectRevert(Paused.selector);
        ethMail.sendMail{value: postageWei}(addr, "Heeeey bro");
    }

    function test_refund() public {
        uint256 postageWei = ethMail.getPostageWei();

        uint256 result = 5 * (10 ** 18) / 1600;
        assertEq(result, postageWei);

        EthMail.PostalAddress memory addr =
            EthMail.PostalAddress("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler");
        vm.prank(address(0));
        ethMail.sendMail{value: postageWei * 2}(addr, "Heeeey bro");

        assertEq(address(ethMail).balance, postageWei);
    }
}
