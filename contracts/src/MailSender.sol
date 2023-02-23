// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "./EthMail.sol";

abstract contract MailSender {
    EthMail public ethMail;

    constructor(address _ethMailAddress) {
        ethMail = EthMail(_ethMailAddress);
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

        EthMail.PostalAddress memory postalAddress =
            EthMail.PostalAddress(addressLine1, addressLine2, city, countryCode, postalOrZip, name);
        uint256 postageCost = ethMail.getPostageWei();
        ethMail.sendMail{value: postageCost}(postalAddress, htmlMessage);

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

        EthMail.PostalAddress memory postalAddress =
            EthMail.PostalAddress(addressLine1, addressLine2, city, countryCode, postalOrZip, name);
        uint256 postageCost = ethMail.getPostageWei();
        ethMail.sendEncryptedMail{value: postageCost}(postalAddress, htmlMessage, true, true);

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
