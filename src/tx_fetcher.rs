use std::{collections::HashMap, lazy::Lazy, sync::{Mutex, Arc}, rc::Rc, cell::RefCell};

use hex::ToHex;

use crate::transaction::Tx;


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
            return String::from("http://testnet.programmingbitcoin.com");
        } else {
            return String::from("http://mainnet.programmingbitcoin.com");
        }
    }

    pub fn fetch(tx_id: &[u8], testnet: bool, fresh: bool) -> Tx {
        /*
        let cache = CACHE.lock().unwrap();
        if fresh || cache.borrow().contains_key(tx_id) {
            let url = format!("{}/tx/{}.hex", Self::get_url(testnet), tx_id.encode_hex::<String>());
            // todo: do http request
        }
        */

        unimplemented!()
    }
}
