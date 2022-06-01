use std::ops::{Add, Sub, Neg, Mul, Div};

use num_bigint::BigInt;


#[derive(Clone)]
#[derive(Debug)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt,
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);

        Self {
            num: (self.num + rhs.num) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

impl Sub for FieldElement {
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

impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FieldElement::sub(FieldElement {
            num: BigInt::from(0i32),
            prime: self.prime.clone(),
        }, self)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);

        Self {
            num: self.num * rhs.num % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

impl Div for FieldElement {
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

impl FieldElement {
    pub fn new(num: BigInt, prime: &BigInt) -> Self {
        let prime = prime.clone();
        if num >= prime || num < BigInt::from(0i32) {
            panic!("Num {} not in field range 0 to {}", num, prime);
        }

        FieldElement {
            num,
            prime,
        }
    }

    pub fn pow(&self, exponent: &BigInt) -> Self {
        let mut expo = BigInt::from(exponent.clone());
        if exponent < &BigInt::from(0i32) {
            expo = &self.prime - BigInt::from(1u32) + expo;
        }

        Self {
            //num: self.num.pow(expo.to_u32_digits().1[0]) % &self.prime,
            num: self.num.modpow(&expo, &self.prime),
            prime: self.prime.clone(),
        }
    }
}
