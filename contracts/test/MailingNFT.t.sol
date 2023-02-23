// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Test.sol";
import "../src/EthMail.sol";
import "../src/ChainlinkPostagePriceModule.sol";
import "../src/test/FakeChainlink.sol";
import "../src/test/MailingNFT.sol";

contract EthMailTest is Test {
    uint256 public constant COST = 5 * (10 ** 18);

    function test_mailingNFT() public {
        FakeChainlink fakeChainlink = new FakeChainlink();
        ChainlinkPostagePriceModule postagePriceModule =
            new ChainlinkPostagePriceModule(AggregatorV3Interface(fakeChainlink), COST);
        EthMail ethMail = new EthMail(IPostagePriceModule(postagePriceModule), "\x00\x01");
        MailingNFT nft = new MailingNFT(address(ethMail));

        uint256 weiPostage = 5 * (10 ** 18) / 1600;
        nft.mintPay{value: weiPostage}();
    }

    function test_refund() public {
        FakeChainlink fakeChainlink = new FakeChainlink();
        ChainlinkPostagePriceModule postagePriceModule =
            new ChainlinkPostagePriceModule(AggregatorV3Interface(fakeChainlink), COST);
        EthMail ethMail = new EthMail(IPostagePriceModule(postagePriceModule), "\x00\x01");
        MailingNFT nft = new MailingNFT(address(ethMail));

        address alice = address(uint160(uint256(keccak256("alice"))));
        vm.label(alice, "Alice");

        uint256 weiPostage = ethMail.getPostageWei();
        uint256 prebalance = 100 ether;
        hoax(alice, prebalance);
        nft.mintPay{value: 2 * weiPostage}(); // overpay by 2x

        assertEq(address(nft).balance, 0);
        assertApproxEqAbs(address(alice).balance, prebalance - weiPostage, 10 * (10 ** 9));
    }
}
