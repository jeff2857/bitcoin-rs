use num_bigint::BigInt;
use hex::ToHex;
use private_key::PrivateKey;
use signature::Signature;
use utils::encode_base58;

use crate::utils::{u8_slice_to_string, u8_slice_base58_to_string};

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
    let secret = BigInt::from(5003i32);

    let private_key = PrivateKey::new(secret);
    //let signature = private_key.sign(message);

    //println!("{:?}", signature);

    let wif = private_key.wif(true, true);
    
    println!("{}", u8_slice_base58_to_string(&wif));
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use num_bigint::BigInt;

    use hex::ToHex;

    use crate::{field_element::FieldElement, elliptic_curve::Point, s256field::S256Field, s256point::{S256Point, self}, signature::Signature, private_key::PrivateKey, utils::{u8_slice_to_string, u8_slice_base58_to_string, encode_base58}};

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

        let sec_pub_key = private_key.get_pub_key().sec(false);

        assert_eq!(
            String::from("04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd59961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121"),
            u8_slice_to_string(&sec_pub_key)
        );

        let secret = BigInt::from(5000i32);
        let private_key = PrivateKey::new(secret);

        let sec_pub_key = private_key.get_pub_key().sec(false);

        assert_eq!(
            String::from("04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10"),
            u8_slice_to_string(&sec_pub_key)
        );
    }

    #[test]
    fn test_parse_sec_pubkey() {
        let secret = BigInt::parse_bytes(b"deadbeef12345", 16).unwrap();
        let private_key = PrivateKey::new(secret);

        let pub_key = private_key.get_pub_key();

        let sec_pub_key = (&pub_key).sec(true);

        let sec = sec_pub_key.encode_hex::<String>();
        let sec = hex::decode(sec).unwrap();
        let parsed_pub_key = s256point::S256Point::parse(sec);

        assert_eq!(pub_key, parsed_pub_key);
    }

    #[test]
    fn test_der_format() {
        let signature = Signature::new(
            BigInt::parse_bytes(b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6", 16).unwrap(),
            BigInt::parse_bytes(b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec", 16).unwrap(),
        );

        let der = signature.der();

        assert_eq!(
            String::from("3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec"),
            u8_slice_to_string(&der),
        );
    }

    #[test]
    fn test_encode_base58() {
        let a = String::from("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d");
        let a_base58 = encode_base58(&hex::decode(a).unwrap());

        assert_eq!(
            String::from("9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6"),
            u8_slice_base58_to_string(&a_base58),
        );

        let a = String::from("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c");
        let a_base58 = encode_base58(&hex::decode(a).unwrap());

        assert_eq!(
            String::from("4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd"),
            u8_slice_base58_to_string(&a_base58),
        );

        let a = String::from("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6");
        let a_base58 = encode_base58(&hex::decode(a).unwrap());

        assert_eq!(
            String::from("EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7"),
            u8_slice_base58_to_string(&a_base58),
        );
    }

    #[test]
    fn test_address() {
        let secret = BigInt::parse_bytes(b"12345deadbeef", 16).unwrap();
        let private_key = PrivateKey::new(secret);
        let pub_key = private_key.get_pub_key();
        let address = pub_key.address(true, false);
     
        assert_eq!(
            String::from("1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1"),
            u8_slice_base58_to_string(&address),
        );

        let secret = BigInt::from(5002i32);
        let private_key = PrivateKey::new(secret);
        let pub_key = private_key.get_pub_key();
        let address = pub_key.address(false, true);
     
        assert_eq!(
            String::from("mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA"),
            u8_slice_base58_to_string(&address),
        );

        let secret = BigInt::from(2020i32).pow(5);
        let private_key = PrivateKey::new(secret);
        let pub_key = private_key.get_pub_key();
        let address = pub_key.address(true, true);
     
        assert_eq!(
            String::from("mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH"),
            u8_slice_base58_to_string(&address),
        );
    }

    #[test]
    fn test_wif() {
        let secret = BigInt::from(5003i32);
        let private_key = PrivateKey::new(secret);
        let wif = private_key.wif(true, true);

        assert_eq!(
            String::from("cMahea7zqjxrtgAbB7LSGbcQUr1uX1ojuat9jZodMN8rFTv2sfUK"),
            u8_slice_base58_to_string(&wif),
        );

        let secret = BigInt::from(2021i32).pow(5);
        let private_key = PrivateKey::new(secret);
        let wif = private_key.wif(false, true);

        assert_eq!(
            String::from("91avARGdfge8E4tZfYLoxeJ5sGBdNJQH4kvjpWAxgzczjbCwxic"),
            u8_slice_base58_to_string(&wif),
        );

        let secret = BigInt::parse_bytes(b"54321deadbeef", 16).unwrap();
        let private_key = PrivateKey::new(secret);
        let wif = private_key.wif(true, false);

        assert_eq!(
            String::from("KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgiuQJv1h8Ytr2S53a"),
            u8_slice_base58_to_string(&wif),
        );
    }
}
