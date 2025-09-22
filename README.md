# uomi
===================================================================================
Base Camp #1: Deploying to a Testnet:

...// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BasicMath {
    function adder(uint _a, uint _b) external pure returns (uint sum, bool error) {
        unchecked {
            uint result = _a + _b;
            if (result >= _a && result >= _b) {
                return (result, false);
            } else {
                return (0, true);
            }
        }
    }

    function subtractor(uint _a, uint _b) external pure returns (uint difference, bool error) {
        unchecked {
            if (_b <= _a) {
                return (_a - _b, false);
            } else {
                return (0, true);
            }
        }
    }
}

===================================================================================

Base Camp #2: Control Structures:

// SPDX-License-Identifier: MIT

pragma solidity 0.8.17;

contract ControlStructures {
    error AfterHours(uint256 time);
    error AtLunch();

    function fizzBuzz(uint256 _number) public pure returns (string memory response) {
        if (_number % 3 == 0 && _number % 5 == 0) {
            return "FizzBuzz";
        } else if (_number % 3 == 0) {
            return "Fizz";
        } else if (_number % 5 == 0) {
            return "Buzz";
        } else {
            return "Splat";
        }
    }

    function doNotDisturb(uint256 _time) public pure returns (string memory result) {
        assert(_time < 2400);

        if (_time > 2200 || _time < 800) {
            revert AfterHours(_time);
        } else if (_time >= 1200 && _time <= 1299) {
            revert("At lunch!");
        } else if (_time >= 800 && _time <= 1199) {
            return "Morning!";
        } else if (_time >= 1300 && _time <= 1799) {
            return "Afternoon!";
        } else if (_time >= 1800 && _time <= 2200) {
            return "Evening!";
        }
    }
}

===================================================================================
