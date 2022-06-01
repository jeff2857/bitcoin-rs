use std::{ops::{Add, Sub, Neg, Mul, Div}, rc::Rc};

use num_bigint::BigInt;
use num_traits::Pow;


#[derive(Clone)]
#[derive(Debug)]
struct FieldElement {
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

#[derive(Debug)]
#[derive(Clone)]
struct Point {
    pub a: Rc<FieldElement>,
    pub b: Rc<FieldElement>,
    pub x: Option<Rc<FieldElement>>,
    pub y: Option<Rc<FieldElement>>,
}

impl Point {
    pub fn new(x: Option<Rc<FieldElement>>, y: Option<Rc<FieldElement>>, a: Rc<FieldElement>, b: Rc<FieldElement>) -> Self {
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
        let mut coef = coefficient.clone();
        let mut current = self.clone();
        let mut result = Self::new(None, None, self.a.clone(), self.b.clone());

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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

impl Add for Point {
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
        if self == rhs && *y1 == FieldElement::new(BigInt::from(0i32), &x1.prime) {
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
                let temp2 = FieldElement::new(3i32 * temp1.num % &x1.prime, &x1.prime);
                let temp3 = FieldElement::new(2i32 * &y1.num % &x1.prime, &x1.prime);
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

fn main() {
    let prime = BigInt::from(223i32);
    let a = Rc::new(FieldElement::new(BigInt::from(0i32), &prime));
    let b = Rc::new(FieldElement::new(BigInt::from(7i32), &prime));
    let x1 = Rc::new(FieldElement::new(BigInt::from(15i32), &prime));
    let y1 = Rc::new(FieldElement::new(BigInt::from(86i32), &prime));
    let x2 = Rc::new(FieldElement::new(BigInt::from(17i32), &prime));
    let y2 = Rc::new(FieldElement::new(BigInt::from(56i32), &prime));
    let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone());
    let p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone());

    let gx = BigInt::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
    let gy = BigInt::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();

    let p = BigInt::from(2i32);
    let t = p.clone().pow(256u32) - p.pow(32u32) - BigInt::from(977i32);
    let n = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();

    let a = Rc::new(FieldElement::new(BigInt::from(0i32), &t));
    let b = Rc::new(FieldElement::new(BigInt::from(7i32), &t));
    let x = Rc::new(FieldElement::new(gx, &t));
    let y = Rc::new(FieldElement::new(gy, &t));

    let G = Point::new(Some(x), Some(y), a, b);
    println!("G: {:?}", &G);

    let nG = G.multi(n);
    println!("n * G: {:?}", &nG);
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use num_bigint::BigInt;

    use crate::{FieldElement, Point};

    #[test]
    fn test_on_curve() {
        let prime = BigInt::from(223i32);
        let a = Rc::new(FieldElement::new(BigInt::from(0i32), &prime));
        let b = Rc::new(FieldElement::new(BigInt::from(7i32), &prime));

        let valid_points = [
            (BigInt::from(192i32), BigInt::from(105i32)),
            (BigInt::from(17i32), BigInt::from(56i32)),
            (BigInt::from(1i32), BigInt::from(193i32)),
        ];

        for (x_raw, y_raw) in valid_points {
            let x = Rc::new(FieldElement::new(x_raw, &prime));
            let y = Rc::new(FieldElement::new(y_raw, &prime));
            Point::new(Some(x), Some(y), a.clone(), b.clone());
        }
    }

    #[test]
    #[should_panic]
    fn test_not_on_curve() {
        let prime = BigInt::from(223i32);
        let a = Rc::new(FieldElement::new(BigInt::from(0i32), &prime));
        let b = Rc::new(FieldElement::new(BigInt::from(0i32), &prime));

        let invalid_points = [
            (BigInt::from(200i32), BigInt::from(119i32)),
            (BigInt::from(42i32), BigInt::from(99i32)),
        ];

        for (x_raw, y_raw) in invalid_points {
            let x = Rc::new(FieldElement::new(x_raw, &prime));
            let y = Rc::new(FieldElement::new(y_raw, &prime));
            Point::new(Some(x), Some(y), a.clone(), b.clone());
        }
    }

    #[test]
    fn test_addition() {
        let prime = BigInt::from(223i32);
        let a = Rc::new(FieldElement::new(BigInt::from(0i32), &prime));
        let b = Rc::new(FieldElement::new(BigInt::from(7i32), &prime));

        let x1 = Rc::new(FieldElement::new(BigInt::from(192i32), &prime));
        let y1 = Rc::new(FieldElement::new(BigInt::from(105i32), &prime));
        let x2 = Rc::new(FieldElement::new(BigInt::from(17i32), &prime));
        let y2 = Rc::new(FieldElement::new(BigInt::from(56i32), &prime));

        let p1 = Point::new(Some(x1.clone()), Some(y1.clone()), a.clone(), b.clone());
        let p2 = Point::new(Some(x2.clone()), Some(y2.clone()), a.clone(), b.clone());

        let expected_x = Rc::new(FieldElement::new(BigInt::from(170i32), &prime));
        let expected_y = Rc::new(FieldElement::new(BigInt::from(142i32), &prime));

        let expected_point = Point::new(
            Some(expected_x),
            Some(expected_y),
            a,
            b,
        );
        assert_eq!(expected_point, p1 + p2);
    }

    #[test]
    fn test_scalar_multi() {
        let prime = BigInt::from(223i32);
        let a = Rc::new(FieldElement::new(BigInt::from(0i32), &prime));
        let b = Rc::new(FieldElement::new(BigInt::from(7i32), &prime));
        let x1 = Rc::new(FieldElement::new(BigInt::from(15i32), &prime));
        let y1 = Rc::new(FieldElement::new(BigInt::from(86i32), &prime));
        let p1 = Point::new(Some(x1.clone()), Some(y1.clone()), a.clone(), b.clone());

        let expected_point = Point::new(
            None,
            None,
            a.clone(),
            b.clone(),
        );

        assert_eq!(expected_point, p1.multi(BigInt::from(7i32)));
    }
}
