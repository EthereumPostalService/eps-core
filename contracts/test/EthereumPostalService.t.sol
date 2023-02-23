// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Test.sol";
import "../src/EthereumPostalService.sol";
import "../src/ChainlinkPostagePriceModule.sol";
import "./FakeChainlink.sol";

contract EthereumPostalServiceTest is Test {
    address public constant MAINNET_CHAINLINK_ETH_USD = 0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419;
    uint256 public constant COST = 5 * (10 ** 18);

    uint256 mainnetFork;

    FakeChainlink fakeChainlink;
    ChainlinkPostagePriceModule postagePriceModule;
    EthereumPostalService eps;

    function setUp() public {
        string memory MAINNET_RPC_URL = vm.envString("MAINNET_RPC_URL");
        mainnetFork = vm.createFork(MAINNET_RPC_URL);
        vm.selectFork(mainnetFork);

        fakeChainlink = new FakeChainlink();
        postagePriceModule = new ChainlinkPostagePriceModule(AggregatorV3Interface(fakeChainlink), COST);
        eps = new EthereumPostalService(IPostagePriceModule(postagePriceModule), "\x00\x01");
    }

    function test_mainnetFork() public {
        ChainlinkPostagePriceModule mPostagePriceModule =
            new ChainlinkPostagePriceModule(AggregatorV3Interface(MAINNET_CHAINLINK_ETH_USD), COST);
        EthereumPostalService mEPS = new EthereumPostalService(IPostagePriceModule(mPostagePriceModule), "\x00\x01");
        uint256 postageWei = mEPS.getPostageWei();

        uint256 upper = 5 * (10 ** 18) / 100;
        uint256 lower = 5 * (10 ** 18) / 10000;

        assertTrue(lower <= postageWei);
        assertTrue(upper >= postageWei);

        EthereumPostalService.PostalAddress memory addr =
            EthereumPostalService.PostalAddress("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler");
        mEPS.sendMail{value: postageWei}(addr, "Heeeey bro");
    }

    function test_fakeChainlink() public {
        uint256 postageWei = eps.getPostageWei();

        uint256 result = 5 * (10 ** 18) / 1600;
        assertEq(result, postageWei);

        EthereumPostalService.PostalAddress memory addr =
            EthereumPostalService.PostalAddress("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler");
        eps.sendMail{value: postageWei}(addr, "Heeeey bro");

        assertEq(address(eps).balance, postageWei);

        address alice = address(uint160(uint256(keccak256("alice"))));
        vm.label(alice, "Alice");

        eps.transfer(alice);
        assertEq(address(alice).balance, postageWei);
    }

    function test_pause() public {
        uint256 postageWei = eps.getPostageWei();

        uint256 result = 5 * (10 ** 18) / 1600;
        assertEq(result, postageWei);

        EthereumPostalService.PostalAddress memory addr =
            EthereumPostalService.PostalAddress("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler");
        eps.togglePause();
        vm.expectRevert(Paused.selector);
        eps.sendMail{value: postageWei}(addr, "Heeeey bro");
    }

    function test_refund() public {
        uint256 postageWei = eps.getPostageWei();

        uint256 result = 5 * (10 ** 18) / 1600;
        assertEq(result, postageWei);

        EthereumPostalService.PostalAddress memory addr =
            EthereumPostalService.PostalAddress("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler");
        vm.prank(address(0));
        eps.sendMail{value: postageWei * 2}(addr, "Heeeey bro");

        assertEq(address(eps).balance, postageWei);
    }
}
