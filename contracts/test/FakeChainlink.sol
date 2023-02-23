// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

contract FakeChainlink is AggregatorV3Interface {
    constructor() {}

    function decimals() public pure returns (uint8) {
        return 8;
    }

    function latestRoundData() public pure returns (uint80, int256, uint256, uint256, uint80) {
        return (0, 160000000000, 0, 0, 0);
    }

    function description() external pure returns (string memory) {
        revert("unimplemented");
    }

    function version() external pure returns (uint256) {
        revert("unimplemented");
    }

    function getRoundData(uint80) external pure returns (uint80, int256, uint256, uint256, uint80) {
        revert("unimplemented");
    }
}
