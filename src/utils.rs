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

/// convert BigInt to little-endian bytes
pub fn int_to_little_endian(n: &BigInt, len: usize) -> Vec<u8> {
    let n = n.clone();
    let mut n_bytes = n.to_bytes_le().1;
    if n_bytes.len() < len {
        for _ in 0..(len - n_bytes.len()) {
            n_bytes.push(b'\x00');
        }
    }

    n_bytes
}

/// convert little-endian bytes to BigInt
pub fn little_endian_to_int(b: &[u8]) -> BigInt {
    BigInt::from_bytes_le(num_bigint::Sign::Plus, b)
}

/// reads a variable integer and decode to BigInt
pub fn read_varint(s: &[u8]) -> (BigInt, usize) {
    let i = s[0];
    // number between 253 and 2^16 - 1, start with 253(fd) and the number in 2 bytes in little-endian
    if i == 0xfd {
        return (little_endian_to_int(&s[1..3]), 2);
    }
    // number between 2^16 and 2^32 - 1, start with 254(fe) and the number in 4 bytes in little-endian
    if i == 0xfe {
        return (little_endian_to_int(&s[1..5]), 4);
    }
    // number between 2^32 and 2^64 - 1, start with 255(ff) and the number in 8 bytes in little-endian
    if i == 0xff {
        return (little_endian_to_int(&s[1..9]), 8);
    }
    // number below 253, 1 single byte
    return (little_endian_to_int(&[i]), 1);
}

/// encode BigInt to varint
pub fn encode_varint(i: &BigInt) -> Vec<u8> {
    let i = i.clone();
    if i < BigInt::parse_bytes(b"fd", 16).unwrap() {
        return i.to_bytes_le().1;
    }
    if i < BigInt::parse_bytes(b"10000", 16).unwrap() {
        let mut result: Vec<u8> = vec![b'\xfd'];
        result.extend_from_slice(&int_to_little_endian(&i, 2));
        return result;
    }
    if i < BigInt::parse_bytes(b"100000000", 16).unwrap() {
        let mut result: Vec<u8> = vec![b'\xfe'];
        result.extend_from_slice(&int_to_little_endian(&i, 4));
        return result;
    }
    if i < BigInt::parse_bytes(b"10000000000000000", 16).unwrap() {
        let mut result: Vec<u8> = vec![b'\xff'];
        result.extend_from_slice(&int_to_little_endian(&i, 8));
        return result;
    }

    panic!("integer too large: {}", i);
}

/// parse block target bits to BigInt, 4 bytes
pub fn bits_to_target(s: &[u8]) -> BigInt {
    let exponent = s[3] as u32;
    let coefficient = little_endian_to_int(&s[0..3]);
    let target = coefficient * BigInt::from(256usize).pow(exponent - 3);

    target
}

pub fn merkle_parent(hash_l: &[u8], hash_r: &[u8]) -> Vec<u8> {
    let mut hashes = hash_l.to_owned();
    hashes.extend_from_slice(hash_r);
    let parent = hash256(&hashes);

    parent
}

pub fn merkle_parent_level(hashes: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut even_hashes = hashes.to_owned();
    if hashes.len() % 2 == 1 {
        even_hashes.push(hashes[hashes.len() - 1].clone());
    }

    let mut parent_level = vec![];
    for i in 0..(even_hashes.len() / 2) {
        let parent = merkle_parent(&even_hashes[i * 2], &even_hashes[i * 2 + 1]);
        parent_level.push(parent);
    }

    parent_level
}

pub fn merkle_root(hashes: &[Vec<u8>]) -> Vec<u8> {
    let mut current_hashes = hashes.to_owned();
    while current_hashes.len() > 1 {
        current_hashes = merkle_parent_level(&current_hashes);
    }

    current_hashes[0].clone()
}

#[macro_export]
macro_rules! vec_with_init_val {
    ( $x: expr; $y: expr) => {
        {
            let mut v = Vec::new();
            for _ in 0..$y {
                v.push($x);
            }

            v
        }
    };
}
