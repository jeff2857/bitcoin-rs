use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Clone, Copy)]
#[derive(Debug)]
struct FieldElement {
    pub num: i64,
    pub prime: i64,
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
            num: self.num * rhs.num % self.prime,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let temp = Self {
            num: rhs.num.pow((rhs.prime - 2) as u32),
            prime: rhs.prime,
        };

        Self {
            num: (self * temp).num,
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
            num: num as i64,
            prime: prime as i64,
        }
    }

    pub fn pow(&self, mut exponent: i64) -> Self {
        if exponent < 0 {
            exponent = self.prime - 1 + exponent;
        }

        Self {
            num: self.num.pow(exponent as u32) % self.prime,
            prime: self.prime,
        }
    }
}

fn main() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(8, 13);
    let c = FieldElement::new(10, 13);

    println!("{:?}", a.pow(-3));
    //println!("{:?}", b.pow((b.prime - 2) as u32));
    //println!("{:?}", b.pow(12));
    //println!("{:?}", c.pow(12));
}
