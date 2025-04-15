pragma solidity ^0.8.0;

import "./Parent.sol";

contract Frontier is Parent {
    uint256 public value;

    function setValxxue3(uint256 _value) public {
        setOtherValue(_value);
        setValue(_value + 20);
    }

    function setValue2(uint256 _value) public {
        setValue(_value + 10);
    }

    function setValue(uint256 _value) public {
        value = _value;
    }

    function getValue() public view returns (uint256) {
        return value;
    }

    function getValue2() public view returns (uint256) {
        return getValue() + 10;
    }

    function add() public {
        addMore(1);
    }

    function addMore(uint256 _value) public {
        value += _value;
    }
}
