use std::ops::{Add, Sub, Neg, Mul, Rem};

#[derive(Clone, Copy)]
#[derive(Debug)]
struct FieldElement {
    pub num: i32,
    pub prime: i32,
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
            num: (self.num + rhs.num) % self.prime,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);

        let mut num = self.num - rhs.num;
        if num >= 0 {
            num = num % self.prime;
        } else {
            num = self.prime - (-num) % self.prime;
        }

        Self {
            num,
            prime: self.prime,
        }
    }
}

impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FieldElement::sub(FieldElement {
            num: 0,
            prime: self.prime,
        }, self)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);

        Self {
            num: (self.num * rhs.num) % self.prime,
            prime: self.prime,
        }
    }
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Self {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime);
        }

        FieldElement {
            num,
            prime,
        }
    }

    pub fn pow(&self, exponent: u32) -> Self {
        Self {
            num: self.num.pow(exponent) % self.prime,
            prime: self.prime,
        }
    }
}

fn main() {
    let a = FieldElement::new(3, 13);
    let b = FieldElement::new(1, 13);
    let c = FieldElement::new(10, 13);

    println!("{}", a.pow(3) == b);
}
