use std::fmt::{Display, write};

use num_bigint::BigInt;
use hex::ToHex;

use crate::{utils::{hash256, int_to_little_endian, encode_varint, little_endian_to_int, read_varint}, script::Script};


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

    pub fn parse(serialization: &[u8], testnet: bool) -> Self {
        // todo: change parameter to stream
        let mut bytes_read = 0;
        // version is encoded in 4 bytes little-endian
        let version = &serialization[bytes_read..4];
        let version_parsed = little_endian_to_int(&version);
        bytes_read += 4;

        // inputs
        let tx_ins = TxIn::parse(&serialization, &mut bytes_read);

        // outputs
        let tx_outs = TxOut::parse(&serialization, &mut bytes_read);

        // locktime, 4 bytes; if sequence is ffffffff, locktime will be ignored
        let locktime = BigInt::from_bytes_le(num_bigint::Sign::Plus, &serialization[bytes_read..(bytes_read + 4)]);

        Self {
            version: version_parsed,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
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

    /// returns the byte serialization of the transaction
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
    pub script_sig: Script,
    pub sequence: BigInt,
}

impl TxIn {
    pub fn new(prev_tx: Vec<u8>, prev_index: BigInt, script_sig: Option<Script>, sequence: BigInt) -> Self {
        let script_sig = match script_sig {
            Some(sig) => sig,
            None => Script::new(),
        };

        Self {
            prev_tx,
            prev_index,
            script_sig,
            sequence,
        }
    }

    pub fn parse(serialization: &[u8], bytes_read: &mut usize) -> Vec<Self> {
        let (num, b_read) = read_varint(&serialization[*bytes_read..]);
        *bytes_read += b_read;

        let num = num.to_u32_digits().1[0];
        let mut tx_ins: Vec<Self> = vec![];

        for _ in 0..num  {
            // previous transaction id, 32 bytes
            let prev_tx_id = &serialization[*bytes_read..(*bytes_read + 32)];
            *bytes_read += 32;

            // previous transaction index, 4 bytes
            let prev_index = &serialization[*bytes_read..(*bytes_read + 4)];
            let prev_index = BigInt::from_bytes_le(num_bigint::Sign::Plus, prev_index);
            *bytes_read += 4;

            // script sig, variant length, preceded by a varint
            let (script_sig_len, b_read) = read_varint(&serialization[*bytes_read..]);
            *bytes_read += b_read;
            let script_sig_len = script_sig_len.to_u32_digits().1[0] as usize;
            let script_sig = &serialization[*bytes_read..(*bytes_read + script_sig_len)];
            *bytes_read += script_sig_len;

            // sequence, 4 bytes
            let sequence = &serialization[*bytes_read..(*bytes_read + 4)];
            let sequence = BigInt::from_bytes_le(num_bigint::Sign::Plus, sequence);
            *bytes_read += 4;

            let tx_in = Self {
                prev_tx: prev_tx_id.to_owned(),
                prev_index,
                script_sig: Script::parse(script_sig),
                sequence,
            };

            tx_ins.push(tx_in);
        }
        
        tx_ins
    }
}

impl TxIn {
    /// returns the byte serialization of the transaction input
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = self.prev_tx.clone().into_iter().rev().collect();
        result.extend_from_slice(&int_to_little_endian(&self.prev_index, 4));
        result.extend_from_slice(&self.script_sig.serialize());
        result.extend_from_slice(&int_to_little_endian(&self.sequence, 4));

        result
    }

    // todo: fetch transaction from web and get amount

    pub fn value(&self, testnet: bool) -> BigInt {
        // todo: unimplemented
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
    pub script_pub_key: Script,
}

impl TxOut {
    pub fn parse(serialization: &[u8], bytes_read: &mut usize) -> Vec<Self> {
        let (num, b_read) = read_varint(&serialization[*bytes_read..]);
        *bytes_read += b_read;

        let num = num.to_u32_digits().1[0];
        let mut tx_outs: Vec<Self> = vec![];

        for _ in 0..num {
            // amount, 8 bytes
            let amount = &serialization[*bytes_read..(*bytes_read + 8)];
            let amount = BigInt::from_bytes_le(num_bigint::Sign::Plus, amount);
            *bytes_read += 8;

            // script pub key, varint
            let (script_pub_key_len, b_read) = read_varint(&serialization[*bytes_read..]);
            let script_pub_key_len = script_pub_key_len.to_u32_digits().1[0] as usize;
            *bytes_read += b_read;

            let script_pub_key = &serialization[*bytes_read..(*bytes_read + script_pub_key_len)];
            *bytes_read += script_pub_key_len;

            let tx_out = Self {
                amount,
                script_pub_key: Script::parse(script_pub_key),
            };
            tx_outs.push(tx_out);
        }

        tx_outs
    }
}

impl TxOut {
    /// returns the byte serialization of the transaction output
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = int_to_little_endian(&self.amount, 8);
        result.extend_from_slice(&self.script_pub_key.serialize());

        result
    }
}

impl Display for TxOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.amount, self.script_pub_key)
    }
}

// todo: fetch transaction from web
pub struct TxFetcher {

}
