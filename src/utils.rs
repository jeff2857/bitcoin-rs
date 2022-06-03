use num_bigint::BigInt;
use num_traits::ToPrimitive;
use ripemd::Ripemd160;
use sha2::{Sha256, Digest};
use hex::ToHex;

pub fn u8_slice_to_string(a: &[u8]) -> String {
    let a = a.to_owned();
    let mut s = String::with_capacity(2 * a.len());

    for byte in a {
        s.push_str(&format!("{:02x?}", &byte));
    }

    s
}

pub fn u8_slice_base58_to_string(a: &[u8]) -> String {
    let a = a.to_owned();
    let s = String::from_utf8(a).unwrap();
    s
}

pub fn encode_base58(s: &[u8]) -> Vec<u8> {
    let base58_alphabet = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    let mut count = 0;
    for c in s {
        if *c == 0u8 {
            count += 1;
        } else {
            break;
        }
    }

    let s = s.encode_hex::<String>();
    let mut num = BigInt::parse_bytes(s.as_bytes(), 16).unwrap();
    let mut prefix: Vec<u8> = vec![];
    for _ in 0..count {
        prefix.push(b'1');
    }

    let mut result: Vec<u8> = vec![];
    while num > BigInt::from(0i32) {
        let n = num.clone() / BigInt::from(58i32);
        let r = num.clone() % BigInt::from(58i32);
        num = n;
        result.insert(0, base58_alphabet[r.to_i64().unwrap() as usize]);
    }

    prefix.extend_from_slice(&result);

    prefix
}

pub fn encode_base58_checksum(b: &[u8]) -> Vec<u8> {
    let mut b = b.to_owned();
    let a = hash256(&b);
    b.extend_from_slice(&(hash256(&b)[..4]));
    encode_base58(&b)
}

/// two rounds of sha256
pub fn hash256(s: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(s);
    let s1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(s1.as_slice());
    let s2 = hasher.finalize();
    s2.as_slice().to_owned()
}

pub fn hash160(s: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(s);
    let s1 = hasher.finalize();
    let s1 = s1.as_slice();

    let mut hasher = Ripemd160::new();
    hasher.update(s1);
    let s1 = hasher.finalize();
    s1.as_slice().to_owned()
}
