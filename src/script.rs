use std::fmt::Display;

use num_bigint::BigInt;
use num_traits::ToPrimitive;

use crate::utils::{read_varint, little_endian_to_int, int_to_little_endian, encode_varint};


#[derive(Clone)]
pub enum ScriptCmd {
    OpCode(u8),
    Cmd(Vec<u8>),
}


#[derive(Clone)]
pub struct Script {
    pub cmds: Vec<ScriptCmd>,
}

impl Script {
    pub fn new(cmds: Option<&[ScriptCmd]>) -> Self {
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
        let mut cmds: Vec<ScriptCmd> = vec![];
        let mut count = 0usize;

        while count < len {
            let current = s[count];
            count += 1;
            let current_byte = current;

            if current_byte >= 1 && current_byte <= 75 { // next n bytes are element
                let n = current_byte as usize;
                let mut op: Vec<u8> = vec![];
                op.extend_from_slice(&s[count..(count + n)]);
                cmds.push(ScriptCmd::Cmd(op));
                count += n;
            } else if current_byte == 76 { // OP_PUSHDATA1, next 1 byte implys how many bytes to
                                           // read
                let data_len = little_endian_to_int(&s[count..(count + 1)]).to_u32_digits().1[0] as usize;
                count += 1;
                let mut op: Vec<u8> = vec![];
                op.extend_from_slice(&s[count..(count + data_len)]);
                cmds.push(ScriptCmd::Cmd(op));
                count += data_len;
            } else if current_byte == 77 { // OP_PUSHDATA2, next 2 bytes implys how many bytes to
                                           // read
                let data_len = little_endian_to_int(&s[count..(count + 2)]).to_u32_digits().1[0] as usize;
                count += 2;
                let mut op: Vec<u8> = vec![];
                op.extend_from_slice(&s[count..(count + data_len)]);
                cmds.push(ScriptCmd::Cmd(op));
                count += data_len;
            } else { // op_code
                cmds.push(ScriptCmd::OpCode(current_byte));
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
        let result = self.raw_serialize();
        let total = result.len();
        let mut total = encode_varint(&BigInt::from(total));
        total.extend_from_slice(&result);

        total
    }
    
    pub fn raw_serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for cmd in &self.cmds {
            match cmd {
                ScriptCmd::OpCode(op_code) => {
                    result.extend_from_slice(&int_to_little_endian(&BigInt::from(*op_code as i32), 1));
                },
                ScriptCmd::Cmd(c) => {
                    let len = c.len();
                    if len < 75 {
                        result.extend_from_slice(&int_to_little_endian(&BigInt::from(len), 1));
                    } else if len > 75 && len < 0x100 {
                        result.extend_from_slice(&int_to_little_endian(&BigInt::from(76i32), 1));
                        result.extend_from_slice(&int_to_little_endian(&BigInt::from(len), 1));
                    } else if len >= 0x100 && len <= 520 {
                        result.extend_from_slice(&int_to_little_endian(&BigInt::from(77i32), 1));
                        result.extend_from_slice(&int_to_little_endian(&BigInt::from(len), 2));
                    } else {
                        panic!("too long a cmd");
                    }
                    result.extend_from_slice(c);
                }
            }
        }

        result
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo: unimplemented
        write!(f, "")
    }
}
