// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ShadyToken {
    mapping(address => uint256) public balances;
    address owner;

    constructor() {
        owner = msg.sender;
    }

    // Scanner should flag this tx.origin usage
    function transfer(address to, uint256 amount) public {
        require(tx.origin == owner, "Not authorized");
        balances[to] += amount;
    }

    // Scanner should flag this selfdestruct
    function emergencyWithdraw() public {
        selfdestruct(payable(owner));
    }
}
