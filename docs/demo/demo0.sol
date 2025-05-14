// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.10;

contract demo0 {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        require(msg.sender == address(0), "xxxx");
        number++;
    }
}
