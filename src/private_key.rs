use std::rc::Rc;

use hmac::{Hmac, Mac};
use num_bigint::BigInt;
use sha2::{Sha256, Digest};
use hex::ToHex;

use crate::{signature::Signature, s256point::S256Point, s256field::S256Field, utils::encode_base58_checksum};


type HmacSha256 = Hmac<Sha256>;

pub struct PrivateKey {
    pub secret: BigInt,
}

impl PrivateKey {
    pub fn new(secret: BigInt) -> Self {
        Self {
            secret,
        }
    }
}

impl PrivateKey {
    pub fn hex(&self) -> String {
        format!("{:0>64}", self.secret)
    }

    pub fn sign(&self, z: String) -> Signature {
        let gx = BigInt::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
        let gy = BigInt::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();

        let x = Rc::new(S256Field::new(gx));
        let y = Rc::new(S256Field::new(gy));

        let g = S256Point::new(Some(x), Some(y));

        let n = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();

        let z_hashed = self.hash256(z.as_bytes());
        let z = BigInt::from_bytes_be(num_bigint::Sign::Plus, &z_hashed);

        //let k = BigInt::from(1234567890i128);
        let k = self.deterministic_k(&z, &n);
        let r = g.multi(*k.clone()).x.unwrap().num.clone();
        let k_inv = k.modpow(&(&n - &BigInt::from(2i32)), &n);

        let secret_hashed = self.hash256(&self.secret.to_bytes_be().1);
        let secret = BigInt::from_bytes_be(num_bigint::Sign::Plus, &secret_hashed);

        let mut s = (z + r.clone() * secret) * k_inv % n.clone();

        if s > n.clone() / 2 {
            s = n - s;
        }

        Signature::new(r, s)
    }

    pub fn get_pub_key(&self) -> S256Point {
        let gx = BigInt::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
        let gy = BigInt::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();

        let x = Rc::new(S256Field::new(gx));
        let y = Rc::new(S256Field::new(gy));

        let g = S256Point::new(Some(x), Some(y));

        let pub_point = g.multi(self.secret.clone());

        pub_point
    }

    /// two rounds of sha256
    fn hash256(&self, s: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(s);
        let s1 = hasher.finalize();
        let mut hasher = Sha256::new();
        hasher.update(s1.as_slice());
        let s2 = hasher.finalize();
        s2.as_slice().to_owned()
    }

    fn deterministic_k(&self, z: &BigInt, n: &BigInt) -> Box<BigInt> {
        let mut z = z.clone();
        let mut k = Vec::new();
        for _ in 0..32 {
            k.push(b'\x00');
        }

        let mut v = Vec::new();
        for _ in 0..32 {
            v.push(b'\x00');
        }

        if z > *n {
            z = z - n.clone();
        }

        let mut z_bytes = z.to_bytes_be().1;
        // pre-complete with b'\x00' to make it's length at 32
        if z_bytes.len() < 32 {
            for _ in 0..(32 - z_bytes.len()) {
                z_bytes.insert(0, b'\x00');
            }
        }

        let mut secret_bytes = self.secret.to_bytes_be().1;
        // pre-complete with b'\x00' to make it's length at 32
        if secret_bytes.len() < 32 {
            for _ in 0..(32 - secret_bytes.len()) {
                secret_bytes.insert(0, b'\x00');
            }
        }
       
        let mut mac = HmacSha256::new_from_slice(&k).unwrap();
        let mut msg = v.clone();
        msg.push(b'\x00');
        msg.extend_from_slice(&secret_bytes);
        msg.extend_from_slice(&z_bytes);
        mac.update(&msg);
        let k = &mac.finalize().into_bytes()[..];

        let mut mac = HmacSha256::new_from_slice(k).unwrap();
        mac.update(&v);
        let v = &mac.finalize().into_bytes()[..];

        let mut mac = HmacSha256::new_from_slice(&k).unwrap();
        let mut msg = v.clone().to_owned();
        msg.push(b'\x01');
        msg.extend_from_slice(&secret_bytes);
        msg.extend_from_slice(&z_bytes);
        mac.update(&msg);
        let mut k = mac.finalize().into_bytes();

        let mut mac = HmacSha256::new_from_slice(&k[..]).unwrap();
        mac.update(v);
        let mut v = mac.finalize().into_bytes();

        loop {
            let mut mac = HmacSha256::new_from_slice(&k[..]).unwrap();
            mac.update(&v[..]);
            v = mac.finalize().into_bytes();
            
            let candidate = BigInt::from_bytes_be(num_bigint::Sign::Plus, &v[..]);
            if candidate >= BigInt::from(1i32) && candidate < n.clone() {
                return Box::new(candidate);
            }

            let mut mac = HmacSha256::new_from_slice(&k[..]).unwrap();
            let mut msg = (&v[..]).clone().to_owned();
            msg.push(b'\x00');
            mac.update(&msg);
            k = mac.finalize().into_bytes();

            let mut mac = HmacSha256::new_from_slice(&k[..]).unwrap();
            mac.update(&v[..]);
            v = mac.finalize().into_bytes();
        }
    }

    pub fn wif(&self, compressed: bool, testnet: bool) -> Vec<u8> {
        let mut secret_bytes = self.secret.to_bytes_be().1;
        if secret_bytes.len() < 32 {
            for _ in 0..(32 - secret_bytes.len()) {
                secret_bytes.insert(0, b'\x00');
            }
        }

        let prefix;
        if testnet {
            prefix = b'\xef';
        } else {
            prefix = b'\x80';
        }

        let mut s: Vec<u8> = vec![prefix];
        s.extend_from_slice(&secret_bytes);
        if compressed {
            s.push(b'\x01');
        }

        encode_base58_checksum(&s)
    }
}
