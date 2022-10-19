pragma solidity ^0.8.0;


contract Storage {

    address public owner;
    uint solution;

    function setOwner(address newOwner) external {
        owner = newOwner;
    }

    function setSoution(uint _solution) external {
        solution = _solution;
    }

    function solve(uint) external view returns (uint) {
        return solution;
    }

}

contract Solver {

    bytes constant PAYLOAD = abi.encodeWithSelector(0x00000003, 0x8e481dc7860000000000000081fb129e0c33d8b1318b24e28966260bec976a66);
    address constant CHALLANGE = 0xC8565A653B27FB4Ae88d69e1865A2748b137805a;
    address constant STORAGE = 0x6E2dB9b9F17f639AeD6E14aaF2d1c9A0bFED490B;

    uint constant MASK = (1<<8) - 1;

    constructor() {
        Storage(STORAGE).setOwner(address(this));
        Storage(STORAGE).setSoution(getSolution());

        CHALLANGE.call(PAYLOAD);
    }

    function getSolution() internal view returns (uint) {
        uint seed = _getSolveInput(block.number, address(this));
        return _solve(seed);
    }

    function bytes32ToUint(bytes32 x) internal pure returns (uint y) {
        assembly { y := x }
    }

    function _getSolveInput(uint blockNum, address sender) internal pure returns (uint) {
        return bytes32ToUint(keccak256(abi.encodePacked(
            uint(blockNum),
            uint(0x00),
            uint(uint160(sender))
        )));
    }

    function _solve(uint p) internal pure returns (uint result) {
        assembly {
            let evenSum
            let oddSum
            for { let i } lt(i, 32) { i := add(i, 1) } { 
                let v := and(MASK, shr(mul(8, i), p))
                switch and(v, 1)
                    case 0 {
                        evenSum := add(evenSum, v)
                    }
                    default {
                        oddSum := add(oddSum, v)
                    }
            }
            result := or(shl(128, evenSum), oddSum)
        }
    }

}
