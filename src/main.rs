use std::ops::{Add, Sub, Neg};

#[derive(Clone, Copy)]
#[derive(Debug)]
struct FieldElement {
    num: i32,
    prime: i32,
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
        if num < 0 {
            num = self.prime - (-num) % self.prime;
        } else {
            num = num % self.prime;
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
}

fn main() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(7, 13);

    let d = -a;

    println!("{}", a + b == c);
    println!("{:?}", d);
    println!("{:?}", -d);
}
