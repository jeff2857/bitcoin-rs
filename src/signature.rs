use std::{fmt::Debug, rc::Rc};

use num_bigint::BigInt;

use crate::{s256point::S256Point, s256field::S256Field};


pub struct Signature {
    pub r: BigInt,
    pub s: BigInt,
}

impl Signature {
    pub fn new(r: BigInt, s: BigInt) -> Self {
        Self {
            r,
            s,
        }
    }
}

impl Signature {
    pub fn is_valid(&self, z: &BigInt, pub_key: &S256Point) -> bool {
        let gx = BigInt::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
        let gy = BigInt::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();

        let x = Rc::new(S256Field::new(gx));
        let y = Rc::new(S256Field::new(gy));

        let g = S256Point::new(Some(x), Some(y));

        let n = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();

        let z = z.clone();
        let s = self.s.clone();
        let r = self.r.clone();

        let u = z * s.clone().modpow(&(n.clone() - BigInt::from(2i32)), &n) % n.clone();
        let v = r.clone() * s.modpow(&(n.clone() - BigInt::from(2i32)), &n) % n.clone();

        let k_g = g.multi(u) + pub_key.multi(v);

        let rx = &k_g.x.as_ref().unwrap().num;

        return *rx == r;
    }
}

impl Debug for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_formatted = format!("Signature {{ r: 0x{:0>64}, s: 0x{:0>64} }}", self.r.to_str_radix(16), self.s.to_str_radix(16));
        write!(f, "{}", self_formatted)
    }
}
