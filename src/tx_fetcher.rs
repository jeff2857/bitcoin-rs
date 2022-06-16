use std::{collections::HashMap, lazy::Lazy, sync::{Mutex, Arc}, rc::Rc, cell::RefCell};

use hex::ToHex;
use log::info;
use num_bigint::BigInt;
use serde::{Serialize, Deserialize};

use crate::{transaction::{Tx, TxIn, TxOut}, vec_with_init_val, script::Script};


/*
static CACHE: Lazy<Mutex<HashMap<Vec<u8>, Tx>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});
*/


pub struct TxFetcher {
}

impl TxFetcher {

    pub fn get_url(testnet: bool) -> String {
        if testnet {
            return "https://blockstream.info/testnet/api/tx/".to_string();
        } else {
            return "https://blockstream.info/api/tx/".to_string();
        }
    }

    pub fn fetch(tx_id: &[u8], testnet: bool, fresh: bool) -> Tx {
        let url = format!("{}{}", TxFetcher::get_url(testnet), tx_id.encode_hex::<String>());
        let resp = reqwest::blocking::get(url).unwrap();
        let resp = resp.json::<TxResponse>().unwrap();

        info!("resp: {:?}", resp);

        let mut tx_ins: Vec<TxIn> = vec![];
        for tx_in in resp.vin {
            let prev_tx = vec_with_init_val!(0u8; 32);
            let script_sig = Script::parse(&hex::decode(&tx_in.scriptsig).unwrap());
            tx_ins.push(
                TxIn::new(prev_tx, 0u32, Some(script_sig), tx_in.sequence)
            );
        }

        let mut tx_outs: Vec<TxOut> = vec![];
        for tx_out in resp.vout {
            let script_pub_key = Script::parse(&hex::decode(&tx_out.scriptpubkey).unwrap());
            tx_outs.push(
                TxOut::new(&BigInt::from(tx_out.value), &script_pub_key)
            );
        }

        Tx::new(resp.version, tx_ins, tx_outs, BigInt::from(resp.locktime), testnet)
    }
}


// -- Tx Response --

#[derive(Serialize, Deserialize, Debug)]
struct TxResponse {
    fee: u128,
    locktime: u128,
    size: u128,
    version: u32,
    vin: Vec<TxInResponse>,
    vout: Vec<TxOutResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TxInResponse {
    is_coinbase: bool,
    prevout: TxOutResponse,
    scriptsig: String,
    scriptsig_asm: String,
    sequence: u32,
    txid: String,
    vout: u128,
}

#[derive(Serialize, Deserialize, Debug)]
struct TxOutResponse {
    scriptpubkey: String,
    scriptpubkey_address: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    value: u128,
}


#[cfg(test)]
mod tests_tx_fetcher {
    use super::TxFetcher;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_fetch() {
        init();

        let tx_id = "3cac64f933aaa5d2a80ec6d50a774309284b7c68d7d026cb68c7ccc4cd56a07f".to_string();
        TxFetcher::fetch(&hex::decode(tx_id).unwrap(), false, true);
    }
}
