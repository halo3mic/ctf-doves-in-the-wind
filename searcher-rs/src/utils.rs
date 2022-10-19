use ethers::abi::{ethereum_types::BigEndianHash};
use ethers::{types::{H256, H160, U256, H512}};
use ethers::utils::keccak256;
use hex::FromHex;

pub fn add_to_uint(x: &str) -> U256 {
    let add: H160 = x.parse().unwrap();
    return H256::into_uint(&add.into());
}

pub fn parse_h256(x: &str) -> H256 {
    let xb = Vec::from_hex(x.trim_start_matches("0x")).expect("Invalid Hex String");
    H256::from_slice(&xb)
}

pub fn parse_h512(x: &str) -> H512 {
    let xb = Vec::from_hex(x.trim_start_matches("0x")).expect("Invalid Hex String");
    H512::from_slice(&xb)
}

pub fn parse_hex(x: &str) -> U256 {
    H256::into_uint(&parse_h256(x))
}

pub fn h256_to_uint(x: &H256) -> U256 {
    H256::into_uint(&x)
}

pub fn uint_to_h256(x: &U256) -> H256 {
    H256::from_uint(&x)
}

pub fn sha256_uint(x: &U256) -> H256 {
    H256::from_slice(&keccak256(H256::from_uint(x).as_bytes()))
}

 