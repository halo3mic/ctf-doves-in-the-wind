use ethers::types::{U256, H512};
use ethers::abi::ethabi::ethereum_types::U512;
use crate::utils::uint_to_h256;

use super::utils::{ parse_hex, sha256_uint, h256_to_uint };

use ethers::abi::{ethereum_types::BigEndianHash};

pub const MAX_ITER: u64 = (2u64).pow(40);

pub struct Solver {
    caller_address: U256,
    some_contract: U256,
    base_prehash: U256,
    target_prefix: U256,
    target_suffix: U256,
    max_iter: u64,
    offset: u64,
}

impl Solver {
    
    pub fn new(
        caller_address: U256, 
        some_contract: U256, 
        offset: Option<u64>,
        max_iter: Option<u64>
    ) -> Self {
        let base_prehash = parse_hex("0x0000000300000000000000000000000000000000000000000000000000000000");
        let target_prefix = U256::from(0x036b);
        let target_suffix = U256::from(0xd073);
        return Self {
            max_iter: max_iter.unwrap_or(MAX_ITER),
            offset: offset.unwrap_or(0),
            caller_address,
            some_contract,
            base_prehash,
            target_prefix,
            target_suffix,
        };
    }

    pub fn solve(&self) -> Option<H512> {
        let index = self.find_index();
        let paydata = self.assemble_paydata(index?);
        return Some(paydata)
    }

    fn find_index(&self) -> Option<U256> {
        for i in self.offset..self.max_iter {
            let bn_i = U256::from(i);
            let prehash = self.assemble_prehash(bn_i);
            let hash = sha256_uint(&prehash);
            let hash_uint = h256_to_uint(&hash);
            if self.prefix_match(hash_uint) && self.suffix_match(hash_uint) {
                println!("Prehash: {:?}", uint_to_h256(&prehash));
                println!("Hash: {:?}", hash);
                return Some(bn_i);
            }
        }
        None
    }

    fn prefix_match(&self, x: U256) -> bool {
        (x >> 240) == self.target_prefix
    }

    fn suffix_match(&self, x: U256) -> bool {
        let suffix_mask = (U256::one() << 16) - U256::one();
        (x & suffix_mask) == self.target_suffix
    }

    fn assemble_prehash(&self, index: U256) -> U256 {
        self.base_prehash | (self.caller_address << 64) | index
    }

    fn assemble_paydata(&self, index: U256) -> H512 {
        let payload_uint = (U512::from(&self.base_prehash) << 256) | 
            (U512::from(self.some_contract) << (256-32)) |
            (U512::from(index) << 416);
        return H512::from_uint(&payload_uint)
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::{ uint_to_h256, add_to_uint, parse_h512 };

    fn dummy_solver() -> Solver {
        return Solver::new(
            add_to_uint("0x663F3ad617193148711d28f5334eE4Ed07016602"),
            add_to_uint("0xC8565A653B27FB4Ae88d69e1865A2748b137805a"),
            None, None
        );
    }

    #[test]
    fn test_prefix_match_1() {
        let solver = dummy_solver();
        let hash = parse_hex("0x036b5f43005213fe8f0ba000a802b91cb981236c94bf68769c21d74bb8134346");
        assert!(solver.prefix_match(hash));
    }

    #[test]
    fn test_prefix_match_2() {
        let solver = dummy_solver();
        let hash = parse_hex("0x136b5f43005213fe8f0ba000a802b91cb981236c94bf68769c21d74bb8134346");
        assert!(!solver.prefix_match(hash));
    }

    #[test]
    fn test_prefix_match_3() {
        let solver = dummy_solver();
        let hash = parse_hex("0x036c5f43005213fe8f0ba000a802b91cb981236c94bf68769c21d74bb8134346");
        assert!(!solver.prefix_match(hash));
    }

    #[test]
    fn test_suffix_match_1() {
        let solver = dummy_solver();
        let hash = parse_hex("0x245c850d31a3ca94a665f0d0425165284d11199c4358f7108749828ac6f8d073");
        assert!(solver.suffix_match(hash));
    }

    #[test]
    fn test_suffix_match_2() {
        let solver = dummy_solver();
        let hash = parse_hex("0x245c850d31a3ca94a665f0d0425165284d11199c4358f7108749828ac6f8e073");
        assert!(!solver.suffix_match(hash));
    }

    #[test]
    fn test_suffix_match_3() {
        let solver = dummy_solver();
        let hash = parse_hex("0x245c850d31a3ca94a665f0d0425165284d11199c4358f7108749828ac6f8d072");
        assert!(!solver.suffix_match(hash));
    }

    #[test]
    fn test_assemble_prehash() {
        let solver = dummy_solver();
        let calc = solver.assemble_prehash(U256::from(223));
        let expected = parse_hex("0x00000003663f3ad617193148711d28f5334ee4ed0701660200000000000000df");
        assert_eq!(uint_to_h256(&calc), uint_to_h256(&expected));
    }

    #[test]
    fn test_assemble_paydata() {
        let solver = dummy_solver();
        let calc = solver.assemble_paydata(U256::from(333));
        let expected = parse_h512("0x00000003000000000000014d00000000c8565a653b27fb4ae88d69e1865a2748b137805a00000000000000000000000000000000000000000000000000000000");
        assert_eq!(calc, expected);
    }

}