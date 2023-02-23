// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "../MailSender.sol";

contract MailingNFT is ERC721URIStorage, MailSender {
    uint256 public tokenCounter;

    constructor(address epsAddr) ERC721("COLLECTION", "TICKER") MailSender(epsAddr) {
        tokenCounter = 0;
    }

    function mintPay() public payable sendsMail returns (uint256) {
        uint256 newItemId = tokenCounter;
        _mint(msg.sender, newItemId);
        _setTokenURI(newItemId, "https://www.sec.gov/files/sec-logo.png");
        tokenCounter += 1;
        return newItemId;
    }

    function mailFields()
        public
        virtual
        override
        returns (
            string memory addressLine1,
            string memory addressLine2,
            string memory city,
            string memory countryCode,
            string memory postalOrZip,
            string memory name,
            string memory htmlMessage
        )
    {
        return ("100 F Street, NE", "", "WashingtonDC", "US", "20549", "Gary Gensler", "gm");
    }
}
