// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Test.sol";
import "../src/EthereumPostalService.sol";
import "../src/ChainlinkPostagePriceModule.sol";
import "../src/test/MailingNFT.sol";
import "./FakeChainlink.sol";

contract MailingNFTTest is Test {
    uint256 public constant COST = 5 * (10 ** 18);

    function test_mailingNFT() public {
        FakeChainlink fakeChainlink = new FakeChainlink();
        ChainlinkPostagePriceModule postagePriceModule =
            new ChainlinkPostagePriceModule(AggregatorV3Interface(fakeChainlink), COST);
        EthereumPostalService eps = new EthereumPostalService(IPostagePriceModule(postagePriceModule), "\x00\x01");
        MailingNFT nft = new MailingNFT(address(eps));

        uint256 weiPostage = 5 * (10 ** 18) / 1600;
        nft.mintPay{value: weiPostage}();
    }

    function test_refund() public {
        FakeChainlink fakeChainlink = new FakeChainlink();
        ChainlinkPostagePriceModule postagePriceModule =
            new ChainlinkPostagePriceModule(AggregatorV3Interface(fakeChainlink), COST);
        EthereumPostalService eps = new EthereumPostalService(IPostagePriceModule(postagePriceModule), "\x00\x01");
        MailingNFT nft = new MailingNFT(address(eps));

        address alice = address(uint160(uint256(keccak256("alice"))));
        vm.label(alice, "Alice");

        uint256 weiPostage = eps.getPostageWei();
        uint256 prebalance = 100 ether;
        hoax(alice, prebalance);
        nft.mintPay{value: 2 * weiPostage}(); // overpay by 2x

        assertEq(address(nft).balance, 0);
        assertApproxEqAbs(address(alice).balance, prebalance - weiPostage, 10 * (10 ** 9));
    }
}
