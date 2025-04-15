// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {Example} from "../src/example1.sol";

contract FooTest is Test {
    uint256 value;

    function test_main() external {
        Example example = new Example();
        value += simple_call();
        
        /*
        if (value == 0) {
            value = simple_call();
        } else {
            value = value + simple_call();
        }
        for (uint256 i = 0; i < 2; i++) {
            value += simple_call();
        }
        */
    }

    function simple_call() public pure returns (uint256) {
        return 1;
    }
}
