// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "./EthereumPostalService.sol";

abstract contract MailSender {
    EthereumPostalService public eps;

    constructor(address _ethereumPostalService) {
        eps = EthereumPostalService(_ethereumPostalService);
    }

    modifier sendsMail() {
        (
            string memory addressLine1,
            string memory addressLine2,
            string memory city,
            string memory countryCode,
            string memory postalOrZip,
            string memory name,
            string memory htmlMessage
        ) = mailFields();

        EthereumPostalService.PostalAddress memory postalAddress =
            EthereumPostalService.PostalAddress(addressLine1, addressLine2, city, countryCode, postalOrZip, name);
        uint256 postageCost = eps.getPostageWei();
        eps.sendMail{value: postageCost}(postalAddress, htmlMessage);

        if (msg.value > postageCost) {
            bool refunded = payable(address(msg.sender)).send(msg.value - postageCost);
            if (!refunded) {
                revert RefundFailed(msg.sender);
            }
        }

        _;
    }

    modifier sendsEncryptedMail() {
        (
            string memory addressLine1,
            string memory addressLine2,
            string memory city,
            string memory countryCode,
            string memory postalOrZip,
            string memory name,
            string memory htmlMessage
        ) = mailFields();

        EthereumPostalService.PostalAddress memory postalAddress =
            EthereumPostalService.PostalAddress(addressLine1, addressLine2, city, countryCode, postalOrZip, name);
        uint256 postageCost = eps.getPostageWei();
        eps.sendEncryptedMail{value: postageCost}(postalAddress, htmlMessage, true, true);

        if (msg.value > postageCost) {
            bool refunded = payable(address(msg.sender)).send(msg.value - postageCost);
            if (!refunded) {
                revert RefundFailed(msg.sender);
            }
        }

        _;
    }

    function mailFields()
        public
        virtual
        returns (
            string memory addressLine1,
            string memory addressLine2,
            string memory city,
            string memory countryCode,
            string memory postalOrZip,
            string memory name,
            string memory htmlMessage
        );
}
