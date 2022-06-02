use num_bigint::BigInt;
use private_key::PrivateKey;

use crate::utils::u8_slice_to_string;

mod field_element;
mod elliptic_curve;
mod signature;
mod s256field;
mod s256point;
mod private_key;
mod utils;


fn main() {
    //let x1 = Rc::new(S256Field::new(BigInt::from(15i32)));
    //let y1 = Rc::new(S256Field::new(BigInt::from(86i32)));
    //let x2 = Rc::new(S256Field::new(BigInt::from(17i32)));
    //let y2 = Rc::new(S256Field::new(BigInt::from(56i32)));
    //let p1 = S256Point::new(Some(x1), Some(y1));
    //let p2 = S256Point::new(Some(x2), Some(y2));

    //let gx = BigInt::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
    //let gy = BigInt::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();

    //let x = Rc::new(S256Field::new(gx));
    //let y = Rc::new(S256Field::new(gy));

    //let g = S256Point::new(Some(x), Some(y));
    //println!("G: {:?}", &g);

    //let n = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();
    //let n_g = g.multi(n);
    //println!("n * G: {:?}", &n_g);

    //let secret = BigInt::from(5000i32);
    let secret = BigInt::parse_bytes(b"deadbeef12345", 16).unwrap();
    let message = String::from("my message");

    let private_key = PrivateKey::new(secret);
    //let signature = private_key.sign(message);

    //println!("{:?}", signature);

    let sec_pub_key = private_key.get_pub_key().sec();
    println!("{}", u8_slice_to_string(&sec_pub_key));
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use num_bigint::BigInt;

    use crate::{field_element::FieldElement, elliptic_curve::Point, s256field::S256Field, s256point::S256Point, signature::Signature, private_key::PrivateKey, utils::u8_slice_to_string};

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

    #[test]
    fn test_signature_valid() {
        let pub_x = Rc::new(S256Field::new(BigInt::parse_bytes(b"887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c", 16).unwrap()));
        let pub_y = Rc::new(S256Field::new(BigInt::parse_bytes(b"61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34", 16).unwrap()));

        let pub_key = S256Point::new(Some(pub_x), Some(pub_y));
        let z = BigInt::parse_bytes(b"ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60", 16).unwrap();
        let r = BigInt::parse_bytes(b"ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395", 16).unwrap();
        let s = BigInt::parse_bytes(b"68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4", 16).unwrap();
        let signature = Signature::new(r, s);

        assert!(signature.is_valid(&z, &pub_key));
    }

    #[test]
    fn test_signature_valid_2() {
        let pub_x = Rc::new(S256Field::new(BigInt::parse_bytes(b"887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c", 16).unwrap()));
        let pub_y = Rc::new(S256Field::new(BigInt::parse_bytes(b"61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34", 16).unwrap()));

        let pub_key = S256Point::new(Some(pub_x), Some(pub_y));
        let z = BigInt::parse_bytes(b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d", 16).unwrap();
        let r = BigInt::parse_bytes(b"eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c", 16).unwrap();
        let s = BigInt::parse_bytes(b"c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6", 16).unwrap();
        let signature = Signature::new(r, s);

        assert!(signature.is_valid(&z, &pub_key));
    }

    #[test]
    fn test_sec_format() {
        let secret = BigInt::parse_bytes(b"deadbeef12345", 16).unwrap();
        let private_key = PrivateKey::new(secret);

        let sec_pub_key = private_key.get_pub_key().sec();

        assert_eq!(
            String::from("04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd59961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121"),
            u8_slice_to_string(&sec_pub_key)
        );

        let secret = BigInt::from(5000i32);
        let private_key = PrivateKey::new(secret);

        let sec_pub_key = private_key.get_pub_key().sec();

        assert_eq!(
            String::from("04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10"),
            u8_slice_to_string(&sec_pub_key)
        );
    }
}
