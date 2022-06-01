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
