use std::fmt::Display;

use num_traits::ToPrimitive;

use crate::utils::{read_varint, little_endian_to_int};

#[derive(Clone)]
pub struct Script {
    pub cmds: Vec<Vec<u8>>,
}

impl Script {
    pub fn new(cmds: Option<&[Vec<u8>]>) -> Self {
        let cmds = match cmds {
            Some(c) => c.to_owned(),
            None => vec![],
        };

        Self {
            cmds,
        }
    }

    pub fn parse(s: &[u8]) -> Self {
        let (len, _) = read_varint(s);
        let len = len.to_u32_digits().1[0] as usize;
        let mut cmds: Vec<Vec<u8>> = vec![];
        let mut count = 0usize;

        while count < len {
            let current = s[0];
            count += 1;
            let current_byte = current;

            if current_byte >= 1 && current_byte <= 75 { // next n bytes are element
                let n = current_byte as usize;
                let mut op: Vec<u8> = vec![];
                op.extend_from_slice(&s[count..(count + n)]);
                cmds.push(op);
                count += n;
            } else if current_byte == 76 { // OP_PUSHDATA1, next 1 byte implys how many bytes to
                                           // read
                let data_len = little_endian_to_int(&s[count..(count + 1)]).to_u32_digits().1[0] as usize;
                count += 1;
                let mut op: Vec<u8> = vec![];
                op.extend_from_slice(&s[count..(count + data_len)]);
                cmds.push(op);
                count += data_len;
            } else if current_byte == 77 { // OP_PUSHDATA2, next 2 bytes implys how many bytes to
                                           // read
                let data_len = little_endian_to_int(&s[count..(count + 2)]).to_u32_digits().1[0] as usize;
                count += 2;
                let mut op: Vec<u8> = vec![];
                op.extend_from_slice(&s[count..(count + data_len)]);
                cmds.push(op);
                count += data_len;
            } else { // op_code
                cmds.push(vec![current_byte]);
            }
        }

        if count != len {
            panic!("parsing script failed");
        }

        Self {
            cmds,
        }
    }
}

impl Script {
    pub fn serialize(&self) -> Vec<u8> {
        // todo: unimplemented
        vec![]
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo: unimplemented
        write!(f, "")
    }
}
