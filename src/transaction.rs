use std::fmt::Display;

use num_bigint::BigInt;
use hex::ToHex;

use crate::{utils::{hash256, int_to_little_endian, encode_varint, little_endian_to_int, read_varint}, script::Script, tx_fetcher::TxFetcher};


#[derive(Clone)]
pub struct Tx {
    /// transaction version, 4 bytes, LE
    version: u32,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
    locktime: BigInt,
    testnet: bool,
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxIn>, tx_outs: Vec<TxOut>, locktime: BigInt, testnet: bool) -> Self {
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
        }
    }

    /// Parse transaction serialization into Tx struct
    pub fn parse(serialization: &[u8], testnet: bool) -> Self {
        let mut bytes_read = 0;
        // version is encoded in 4 bytes little-endian
        let version = little_endian_to_int(&serialization[bytes_read..4]);
        let version = version.to_u32_digits().1[0];
        bytes_read += 4;

        // inputs
        let tx_ins = TxIn::parse(&serialization, &mut bytes_read);

        // outputs
        let tx_outs = TxOut::parse(&serialization, &mut bytes_read);

        // locktime, 4 bytes; if sequence is ffffffff, locktime will be ignored
        let locktime = BigInt::from_bytes_le(num_bigint::Sign::Plus, &serialization[bytes_read..(bytes_read + 4)]);

        Self {
            version,
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
        let mut result = int_to_little_endian(&BigInt::from(self.version), 4);

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

    pub fn is_coinbase(&self) -> bool {
        // coinbase transaction must have exactly one input
        let is_one_input = self.tx_ins.len() == 1;
        if !is_one_input {
            return false;
        }

        // the one input must have a previous transaction of 32 bytes of 00
        let prev_tx = &self.tx_ins[0].prev_tx;
        for b in prev_tx {
            if *b != 0x00u8 {
                return false;
            }
        }

        // the one input must have a previous index of ffffffff
        return self.tx_ins[0].prev_index == 0xffffffff;
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

#[derive(Clone)]
pub struct TxIn {
    /// previous transaction id, 32 bytes, LE
    pub prev_tx: Vec<u8>,
    /// previous transaction index, 4 bytes, LE
    pub prev_index: u32,
    pub script_sig: Script,
    /// sequence, 4 bytes, LE
    pub sequence: u32,
}

impl TxIn {
    pub fn new(prev_tx: Vec<u8>, prev_index: u32, script_sig: Option<Script>, sequence: u32) -> Self {
        let script_sig = match script_sig {
            Some(sig) => sig,
            None => Script::new(None),
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
            let prev_index = prev_index.to_u32_digits().1[0];
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
            let sequence = sequence.to_u32_digits().1[0];
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
        result.extend_from_slice(&int_to_little_endian(&BigInt::from(self.prev_index), 4));
        result.extend_from_slice(&self.script_sig.serialize());
        result.extend_from_slice(&int_to_little_endian(&BigInt::from(self.sequence), 4));

        result
    }

    /// fetch transaction from http request
    pub fn fetch_tx(&self, testnet: bool) -> Tx {
        TxFetcher::fetch(&self.prev_tx, testnet, true)
    }

    // fetch transaction from http request, and get output amount
    pub fn value(&self, testnet: bool) -> BigInt {
        let tx = self.fetch_tx(testnet);
        tx.tx_outs[self.prev_index as usize].amount.clone()
    }

    /// fetch transaction from http request, and get script
    pub fn script_pubkey(&self, testnet: bool) -> Script {
        let tx = self.fetch_tx(testnet);
        tx.tx_outs[self.prev_index as usize].script_pub_key.clone()
    }
}

impl Display for TxIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.prev_tx.encode_hex::<String>(), self.prev_index)
    }
}

// -- TxOut --

#[derive(Clone)]
pub struct TxOut {
    pub amount: BigInt,
    pub script_pub_key: Script,
}

impl TxOut {
    pub fn new(amount: &BigInt, script_pub_key: &Script) -> Self {
        Self {
            amount: amount.clone(),
            script_pub_key: script_pub_key.clone(),
        }
    }

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


#[cfg(test)]
mod tests_tx {
    use log::info;

    use super::Tx;

    #[test]
    fn test_parse_tx() {
        env_logger::init();

        let serialization = hex::decode("010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010\
        000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951\
        c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0\
        da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4\
        038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a473044022078\
        99531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b84\
        61cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba\
        1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c35\
        6efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da\
        6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c3\
        4210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49\
        abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd\
        04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea833\
        1ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c\
        2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20df\
        e7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948\
        a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46\
        430600".to_string()).unwrap();

        let tx = Tx::parse(&serialization, true);
        info!("{}", tx);
    }
}
