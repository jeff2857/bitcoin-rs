use std::ops::{Add, Sub, Neg, Mul, Div};

use num_bigint::BigInt;


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
        let mut temp = BigInt::from(rhs.num);
        temp = temp.pow((rhs.prime - 2) as u32);
        let temp = Self {
            num: (temp % self.prime).to_u64_digits().1[0] as i64,
            prime: rhs.prime,
        };

        Self {
            num: (self * temp).num,
            prime: self.prime,
        }
    }
}

impl FieldElement {
    pub fn new(num: i64, prime: i64) -> Self {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime);
        }

        FieldElement {
            num,
            prime,
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

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Point {
    pub a: FieldElement,
    pub b: FieldElement,
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
}

impl Point {
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Self {
       let point = Self {
            a,
            b,
            x,
            y,
        };
 
        if x.is_none() && y.is_none() {
            return point;
        }
        if x.is_none() || y.is_none() {
            panic!("({:?}, {:?}) is not on the curve", x, y);
        }

        let x = x.unwrap();
        let y = y.unwrap();

        assert!(y.pow(2) == (x.pow(3) + a * x + b), "({:?}, {:?}) is not on the curve", x, y);
        point
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


        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = rhs.x.unwrap();
        let y2 = rhs.y.unwrap();

        // two pints are on the same coordinate and y = 0
        if self == rhs && self.y == Some(FieldElement::new(0, x1.prime)) {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            }
        }

        let s;
        if x1 == x2 {
            // if the two points are on the vertical line
            if y1 != y2 {
                return Self {
                    x: None,
                    y: None,
                    a: self.a,
                    b: self.b,
                }
            } else {
                // two points are on the same coordinate
                let temp1 = x1.pow(2);
                let temp2 = FieldElement::new(3 * temp1.num, x1.prime);
                let temp3 = FieldElement::new(2 * y1.num, x1.prime);
                s = (temp2 + self.a) / (temp3);
            }
        } else {
            s = (y2 - y1) / (x2 - x1);
        }

        let x3 = s.pow(2) - x1 - x2;
        let y3 = s * (x1 - x3) - y1;
        Self {
            a: self.a,
            b: self.b,
            x: Some(x3),
            y: Some(y3),
        }
    }
}

fn main() {
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);
    let x1 = FieldElement::new(192, prime);
    let y1 = FieldElement::new(105, prime);
    let x2 = FieldElement::new(17, prime);
    let y2 = FieldElement::new(56, prime);
    let p1 = Point::new(Some(x1), Some(y1), a, b);
    let p2 = Point::new(Some(x2), Some(y2), a, b);
    let p = p1 + p2;
    println!("{:?}", p);
}


#[cfg(test)]
mod tests {
    use crate::{FieldElement, Point};

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let valid_points = [(192, 105), (17, 56), (1, 193)];

        for (x_raw, y_raw) in valid_points {
            let x = FieldElement::new(x_raw, prime);
            let y = FieldElement::new(y_raw, prime);
            Point::new(Some(x), Some(y), a, b);
        }
    }

    #[test]
    #[should_panic]
    fn test_not_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(0, prime);

        let invalid_points = [(200, 119), (42, 99)];

        for (x_raw, y_raw) in invalid_points {
            let x = FieldElement::new(x_raw, prime);
            let y = FieldElement::new(y_raw, prime);
            Point::new(Some(x), Some(y), a, b);
        }
    }

    #[test]
    fn test_addition() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let x1 = FieldElement::new(192, prime);
        let y1 = FieldElement::new(105, prime);
        let x2 = FieldElement::new(17, prime);
        let y2 = FieldElement::new(56, prime);

        let p1 = Point::new(Some(x1), Some(y1), a, b);
        let p2 = Point::new(Some(x2), Some(y2), a, b);

        let expected_point = Point::new(
            Some(FieldElement::new(170, prime)),
            Some(FieldElement::new(142, prime)),
            a,
            b,
        );
        assert_eq!(expected_point, p1 + p2);
    }
}
