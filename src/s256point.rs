use std::{ops::Add, rc::Rc};

use num_bigint::BigInt;

use crate::s256field::S256Field;


#[derive(Debug)]
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
