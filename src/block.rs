use num_bigint::BigInt;

use crate::utils::{little_endian_to_int, bits_to_target, hash256, merkle_root, int_to_little_endian};

pub struct Block {
    /// version 4 bytes, little-endian
    pub version: u32,
    /// previous block hash, 32 bytes, LE
    pub prev_block: Vec<u8>,
    /// merkle root, 32 bytes, LE
    pub merkle_root: Vec<u8>,
    /// timestamp, 4 bytes, LE
    pub timestamp: u32,
    /// difficulty target, 4 bytes
    pub bits: Vec<u8>,
    /// nonce, 4 bytes
    pub nonce: u32,
    /// hashes of transactions
    pub tx_hashes: Vec<Vec<u8>>,
}

impl Block {
    pub fn new(version: u32, prev_block: &[u8], merkle_root: &[u8], timestamp: u32, bits: &[u8], nonce: u32, tx_hashes: &[Vec<u8>]) -> Self {
        Self {
            version,
            prev_block: prev_block.to_owned(),
            merkle_root: merkle_root.to_owned(),
            timestamp,
            bits: bits.to_owned(),
            nonce,
            tx_hashes: tx_hashes.to_owned(),
        }
    }

    pub fn parse(serialization: &[u8]) -> Self {
        let mut bytes_read = 0usize;
        let version = little_endian_to_int(&serialization[0..4]).to_u32_digits().1[0];
        bytes_read += 4;

        let prev_block = serialization[bytes_read..(bytes_read + 32)].to_owned();
        bytes_read += 32;

        let merkle_root = serialization[bytes_read..(bytes_read + 32)].to_owned();
        bytes_read += 32;

        let timestamp = little_endian_to_int(&serialization[bytes_read..(bytes_read + 4)]).to_u32_digits().1[0];
        bytes_read += 4;

        let bits = serialization[bytes_read..(bytes_read + 4)].to_owned();
        bytes_read += 4;

        let nonce = little_endian_to_int(&serialization[bytes_read..(bytes_read + 4)]).to_u32_digits().1[0];
        bytes_read += 4;

        Self {
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce,
            tx_hashes: vec![],
        }
    }

}

impl Block {
    pub fn serialize(&self) -> Vec<u8> {
        let mut serialization: Vec<u8> = vec![];

        let version = int_to_little_endian(&BigInt::from(self.version), 4);
        serialization.extend_from_slice(&version);

        serialization.extend_from_slice(&self.prev_block);

        serialization.extend_from_slice(&self.merkle_root);

        let timestamp = int_to_little_endian(&BigInt::from(self.timestamp), 4);
        serialization.extend_from_slice(&timestamp);

        serialization.extend_from_slice(&self.bits);

        let nonce = int_to_little_endian(&BigInt::from(self.nonce), 4);
        serialization.extend_from_slice(&nonce);

        serialization
    }

    pub fn hash(&self) -> Vec<u8> {
        let serialization = self.serialize();
        hash256(&serialization)
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
