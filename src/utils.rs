pub fn u8_slice_to_string(a: &[u8]) -> String {
    let a = a.to_owned();
    let mut s = String::with_capacity(2 * a.len());

    for byte in a {
        s.push_str(&format!("{:02x?}", &byte));
    }

    s
}
