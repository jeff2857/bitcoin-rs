use std::{ops::{Add, Sub, Neg, Mul, Div}, fmt::Debug};

use num_bigint::BigInt;


#[derive(Clone)]
pub struct S256Field {
    pub num: BigInt,
    pub prime: BigInt,
}

impl PartialEq for S256Field {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Add for S256Field {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);

        Self {
            num: (self.num + rhs.num) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

impl Sub for S256Field {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);

        let mut num = self.num - rhs.num;
        if num >= BigInt::from(0i32) {
            num = num % &self.prime;
        } else {
            num = &self.prime - (-num) % &self.prime;
        }

        Self {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl Neg for S256Field {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::sub(Self {
            num: BigInt::from(0i32),
            prime: self.prime.clone(),
        }, self)
    }
}

impl Mul for S256Field {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);

        Self {
            num: self.num * rhs.num % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

impl Div for S256Field {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let temp = Self {
            num: rhs.num.modpow(&(&rhs.prime - BigInt::from(2u32)), &self.prime),
            prime: rhs.prime.clone(),
        };

        Self {
            num: (temp * self.clone()).num,
            prime: self.prime,
        }
    }
}

impl S256Field {
    pub fn new(num: BigInt) -> Self {
        let t = BigInt::from(2i32);
        let p = t.clone().pow(256u32) - t.pow(32u32) - BigInt::from(977i32);

        if num >= p || num < BigInt::from(0i32) {
            panic!("Num {} not in field range 0 to {}", num, &p);
        }

        Self {
            num,
            prime: p,
        }
    }

    pub fn pow(&self, exponent: &BigInt) -> Self {
        let mut expo = BigInt::from(exponent.clone());
        if exponent < &BigInt::from(0i32) {
            expo = &self.prime - BigInt::from(1u32) + expo;
        }

        Self {
            num: self.num.modpow(&expo, &self.prime),
            prime: self.prime.clone(),
        }
    }
}

impl Debug for S256Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_formatted = format!("S256Field {{ num: 0x{:0>64} }}", self.num.to_str_radix(16));
        write!(f, "{}", self_formatted)
    }
}
