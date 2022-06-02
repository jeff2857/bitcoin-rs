use std::fmt::Debug;

use num_bigint::BigInt;


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

impl Debug for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_formatted = format!("Signature {{ r: 0x{:0>64}, s: 0x{:0>64} }}", self.r.to_str_radix(16), self.s.to_str_radix(16));
        write!(f, "{}", self_formatted)
    }
}
