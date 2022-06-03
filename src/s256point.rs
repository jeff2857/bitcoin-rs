use std::{ops::Add, rc::Rc, fmt::Debug};

use num_bigint::BigInt;

use hex::ToHex;

use crate::{s256field::S256Field, utils::{hash160, encode_base58_checksum, u8_slice_to_string}};


#[derive(Clone)]
pub struct S256Point {
    pub a: Rc<S256Field>,
    pub b: Rc<S256Field>,
    pub x: Option<Rc<S256Field>>,
    pub y: Option<Rc<S256Field>>,
}

impl S256Point {
    pub fn new(x: Option<Rc<S256Field>>, y: Option<Rc<S256Field>>) -> Self {
        let a = Rc::new(S256Field::new(BigInt::from(0i32)));
        let b = Rc::new(S256Field::new(BigInt::from(7i32)));

        if x.is_none() && y.is_none() {
            return Self {
                a: a.clone(),
                b: b.clone(),
                x,
                y,
            };
        }

        if x.is_none() || y.is_none() {
            panic!("({:?}, {:?}) is not on the curve", x, y);
        }

        let x = x.unwrap().clone();
        let y = y.unwrap().clone();

        assert!(y.pow(&BigInt::from(2i32)) == (x.pow(&BigInt::from(3i32)) + (*a).clone() * (*x).clone() + (*b).clone()), "({:?}, {:?}) is not on the curve", x, y);

        Self {
            a,
            b,
            x: Some(x),
            y: Some(y),
        }
    }

    pub fn parse(sec_bin: Vec<u8>) -> Self {
        if sec_bin[0] == 4 {
            let x = BigInt::from_bytes_be(num_bigint::Sign::Plus, &sec_bin[1..33]);
            let y = BigInt::from_bytes_be(num_bigint::Sign::Plus, &sec_bin[33..65]);

            let px = Rc::new(S256Field::new(x));
            let py = Rc::new(S256Field::new(y));
            return Self::new(Some(px), Some(py));
        }

        let is_even = sec_bin[0]== 2;
        let x = S256Field::new(BigInt::from_bytes_be(num_bigint::Sign::Plus, &sec_bin[1..]));
        let b = S256Field::new(BigInt::from(7i32));
        let alpha = x.pow(&BigInt::from(3i32)) + b;
        let beta = alpha.sqrt();

        let t = BigInt::from(2i32);
        let p = t.clone().pow(256u32) - t.pow(32u32) - BigInt::from(977i32);

        let even_beta;
        let odd_beta;

        if beta.num.clone() % BigInt::from(2i32) == BigInt::from(0i32) {
            even_beta = beta.clone();
            odd_beta = S256Field::new(p - beta.num.clone());
        } else {
            even_beta = S256Field::new(p - beta.num.clone());
            odd_beta = beta;
        }

        if is_even {
            let px = Rc::new(x);
            let py = Rc::new(even_beta);
            return Self::new(Some(px), Some(py));
        } else {
            let px = Rc::new(x);
            let py = Rc::new(odd_beta);
            return Self::new(Some(px), Some(py));
        }
    }
}

impl S256Point {
    pub fn multi(&self, coefficient: BigInt) -> Self {
        // n is specified for s256
        let n = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();

        let mut coef = coefficient.clone() % n;
        let mut current = self.clone();
        let mut result = Self::new(None, None);

        while coef != BigInt::from(0i32) {
            if (coef.clone() & BigInt::from(1i32)) != BigInt::from(0i32) {
                result = result + current.clone();
            }
            let current_1 = current.clone();
            let current_2 = current.clone();
            let temp = current_1 + current_2;
            current = temp;
            coef >>= 1;
        }

        result
    }

    /// returns the binary version of the SEC format
    pub fn sec(&self, compressed: bool) -> Vec<u8> {
        let mut s: Vec<u8> = Vec::new();
        let mut x_bytes = self.x.as_ref().unwrap().num.to_bytes_be().1;
        let mut y_bytes = self.y.as_ref().unwrap().num.to_bytes_be().1;
        
        if x_bytes.len() < 32 {
            for _ in 0..(32 - x_bytes.len()) {
                x_bytes.insert(0, b'\x00');
            }
        }

        if y_bytes.len() < 32 {
            for _ in 0..(32 - y_bytes.len()) {
                y_bytes.insert(0, b'\x00');
            }
        }

        if compressed {
            if self.y.as_ref().unwrap().num.clone() % BigInt::from(2i32) == BigInt::from(0i32) {
                s.push(b'\x02');
            } else {
                s.push(b'\x03');
            }
            s.extend_from_slice(&x_bytes);
        } else {
            s.push(b'\x04');
            s.extend_from_slice(&x_bytes);
            s.extend_from_slice(&y_bytes);
        }

        s
    }

    pub fn hash160(&self, compressed: bool) -> Vec<u8> {
        let h = hash160(&self.sec(compressed));
        h
    }

    pub fn address(&self, compressed: bool, testnet: bool) -> Vec<u8> {
        let h160 = self.hash160(compressed);
        let prefix;
        if testnet {
            prefix = b'\x6f';
        } else {
            prefix = b'\x00';
        }

        let mut s: Vec<u8> = vec![prefix];
        s.extend_from_slice(&h160);

        encode_base58_checksum(&s)
    }
}

impl PartialEq for S256Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

impl Add for S256Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, rhs);
        }

        // if one of the points are Infinite, return the other
        if self.x.is_none() {
            return rhs;
        }
        if rhs.x.is_none() {
            return self;
        }

        let x1 = self.x.as_ref().unwrap().clone();
        let y1 = self.y.as_ref().unwrap().clone();
        let x2 = rhs.x.as_ref().unwrap().clone();
        let y2 = rhs.y.as_ref().unwrap().clone();

        // two pints are on the same coordinate and y = 0
        if self == rhs && *y1 == S256Field::new(BigInt::from(0i32)) {
            return Self {
                x: None,
                y: None,
                a: self.a.clone(),
                b: self.b.clone(),
            }
        }

        let s;
        if *x1 == *x2 {
            // if the two points are on the vertical line
            if *y1 != *y2 {
                return Self {
                    x: None,
                    y: None,
                    a: self.a.clone(),
                    b: self.b.clone(),
                }
            } else {
                // two points are on the same coordinate
                let temp1 = x1.pow(&BigInt::from(2i32));
                let temp2 = S256Field::new(3i32 * temp1.num % &x1.prime);
                let temp3 = S256Field::new(2i32 * &y1.num % &x1.prime);
                let a = self.a.clone();
                s = (temp2 + (*a).clone()) / (temp3);
            }
        } else {
            s = ((*y2).clone() - (*y1).clone()) / ((*x2).clone() - (*x1).clone());
        }

        let x3 = s.pow(&BigInt::from(2i32)) - (*x1).clone() - (*x2).clone();
        let y3 = s * ((*x1).clone() - x3.clone()) - (*y1).clone();

        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            x: Some(Rc::new(x3)),
            y: Some(Rc::new(y3)),
        }
    }
}

impl Debug for S256Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = if let Some(x) = &self.x {
            format!("{:0>64}", x.num.to_str_radix(16))
        } else {
            String::from("None")
        };

        let y = if let Some(y) = &self.y {
            format!("{:0>64}", y.num.to_str_radix(16))
        } else {
            String::from("None")
        };

        let self_formatted = format!("S256Point {{ x: 0x{:0>64}, y: 0x{:0>64} }}", x, y);
        write!(f, "{}", self_formatted)
    }
}
