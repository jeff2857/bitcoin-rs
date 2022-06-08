use std::fmt::{Display, write};

use num_bigint::BigInt;
use hex::ToHex;

use crate::utils::{hash256, int_to_little_endian, encode_varint, little_endian_to_int};


pub struct Tx {
    version: BigInt,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
    locktime: BigInt,
    testnet: bool,
}

impl Tx {
    pub fn new(version: BigInt, tx_ins: Vec<TxIn>, tx_outs: Vec<TxOut>, locktime: BigInt, testnet: bool) -> Self {
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
        }
    }

    pub fn parse(serialization: &[u8]) -> Self {
        // todo: change parameter to stream
        let mut bytes_read = 0;
        // version is encoded in 4 bytes little-endian
        let version = &serialization[bytes_read..4];
        let version_parsed = little_endian_to_int(&version);
        bytes_read += 4;

        // inputs
        let tx_ins = TxIn::parse(&serialization[bytes_read..], &mut bytes_read);
        println!("{}", &bytes_read);

        // todo: parse script
        let tx_outs = TxOut::parse(serialization);

        Self {
            version: version_parsed,
            tx_ins,
            tx_outs,
            locktime: BigInt::from(0i32),
            testnet: true,
        }
    }
}

impl Tx {
    /// binary hash of the legacy serialization
    pub fn hash(&self) -> Vec<u8> {
        hash256(&self.serialize().into_iter().rev().collect::<Vec<u8>>())
    }

    /// human-readable hexadecimal of the transaction hash
    pub fn id(&self) -> String {
        self.hash().encode_hex::<String>()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = int_to_little_endian(&self.version, 4);

        result.extend_from_slice(&encode_varint(&BigInt::from(self.tx_ins.len())));
        for tx_in in &self.tx_ins {
            result.extend_from_slice(&tx_in.serialize());
        }

        result.extend_from_slice(&encode_varint(&BigInt::from(self.tx_outs.len())));
        for tx_out in &self.tx_outs {
            result.extend_from_slice(&tx_out.serialize());
        }

        result.extend_from_slice(&int_to_little_endian(&self.locktime, 4));

        result
    }

    pub fn fee(&self) -> BigInt {
        let mut input_sum = BigInt::from(0i32);
        let mut output_sum = BigInt::from(0i32);

        for tx_in in &self.tx_ins {
            input_sum = input_sum + tx_in.value(self.testnet);
        }
        for tx_out in &self.tx_outs {
            output_sum = output_sum + tx_out.amount.clone();
        }

        input_sum - output_sum
    }
}

impl Display for Tx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tx_ins = String::new();
        for tx_in in &self.tx_ins {
            tx_ins += &format!("{}", &tx_in);
        }
        let mut tx_outs = String::new();
        for tx_out in &self.tx_outs {
            tx_outs += &format!("{}", &tx_out);
        }

        write!(
            f,
            "{{tx: {}\nversion: {}\ntx_ins:\n{}tx_outs:\n{}locktime: {}}}",
            self.id(),
            self.version,
            tx_ins,
            tx_outs,
            self.locktime,
        )
    }
}


// -- TxIn --

pub struct TxIn {
    pub prev_tx: Vec<u8>,
    pub prev_index: BigInt,
    pub script_sig: Vec<u8>,
    pub sequence: BigInt,
}

impl TxIn {
    pub fn new(prev_tx: Vec<u8>, prev_index: BigInt, script_sig: Vec<u8>, sequence: BigInt) -> Self {
        Self {
            prev_tx,
            prev_index,
            script_sig,
            sequence,
        }
    }

    pub fn parse(serialization: &[u8], bytes_read: &mut usize) -> Vec<Self> {
        vec![]
    }
}

impl TxIn {
    /// returns the byte serialization of the transaction input
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = self.prev_tx.clone().into_iter().rev().collect();
        result.extend_from_slice(&int_to_little_endian(&self.prev_index, 4));
        //result.extend_from_slice(self.script_sig.serialize());
        result.extend_from_slice(&int_to_little_endian(&self.sequence, 4));
        result
    }

    // todo: fetch transaction from web and get amout

    pub fn value(&self, testnet: bool) -> BigInt {
        BigInt::from(0i32)
    }
}

impl Display for TxIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.prev_tx.encode_hex::<String>(), self.prev_index)
    }
}

// -- TxOut --

pub struct TxOut {
    pub amount: BigInt,
    pub script_pub_key: Vec<u8>,
}

impl TxOut {
    pub fn parse(serialization: &[u8]) -> Vec<Self> {
        vec![]
    }
}

impl TxOut {
    /// returns the byte serialization of the transaction output
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = int_to_little_endian(&self.amount, 8);
        //result.extend_from_slice(&self.script_pub_key.serialize());
        result
    }
}

impl Display for TxOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.amount, self.script_pub_key.encode_hex::<String>())
    }
}

// todo: fetch transaction from web
pub struct TxFetcher {

}
