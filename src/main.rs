use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::Pow;

mod field_element;
mod elliptic_curve;

use crate::field_element::FieldElement;
use crate::elliptic_curve::Point;


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
