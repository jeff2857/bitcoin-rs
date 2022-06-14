use num_bigint::BigInt;

use crate::utils::{little_endian_to_int, bits_to_target, hash256, merkle_root};

pub struct Block {
    /// version 4 bytes, little-endian
    pub version: u32,
    pub merkle_root: Vec<u8>,
    pub bits: Vec<u8>,
    /// hashes of transactions
    pub tx_hashes: Vec<Vec<u8>>,
}

impl Block {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn parse(serialization: &[u8]) -> Self {
        let version = little_endian_to_int(&serialization[0..4]).to_u32_digits().1[0];
        unimplemented!()
    }

}

impl Block {
    pub fn serialize(&self) -> Vec<u8> {
        unimplemented!()
    }

    pub fn hash() -> Vec<u8> {
        unimplemented!()
    }

    pub fn bip9(&self) -> bool {
        self.version >> 29 == 0b001
    }

    pub fn bip91(&self) -> bool {
        self.version >> 4 & 1 == 1
    }

    pub fn bip141(&self) -> bool {
        self.version >> 1 & 1 == 1
    }

    pub fn difficulty(&self) -> BigInt {
        let target = bits_to_target(&self.bits);
        let difficulty = BigInt::from(0xffffu16) * BigInt::from(256u32).pow(0x1du32 - 3) / target;
        
        difficulty
    }

    /// check if this Block is valid
    pub fn check_pow(&self) -> bool {
        let serialization = self.serialize();
        let hash_of_block_header = hash256(&serialization);
        let proof = little_endian_to_int(&hash_of_block_header);
        let target = bits_to_target(&self.bits);
        
        proof < target
    }

    pub fn validate_merkle_root(&self) -> bool {
        let mut tx_hashes: Vec<Vec<u8>> = vec![];
        for h in &self.tx_hashes {
            // little-endian
            tx_hashes.push(h.clone().into_iter().rev().collect());
        }

        let mut calculated_merkle_root = merkle_root(&tx_hashes);
        calculated_merkle_root = calculated_merkle_root.into_iter().rev().collect();

        calculated_merkle_root == self.merkle_root
    }
}
