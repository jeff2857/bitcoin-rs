use std::fmt::Display;

use log::info;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use hex::ToHex;

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
        info!("script serialization: {}", s.encode_hex::<String>());

        let (len, bytes_read) = read_varint(s);
        let len = len.to_u32_digits().1[0] as usize;
        info!("script length: {}", len);

        let mut cmds: Vec<ScriptCmd> = vec![];
        let mut count = 0usize;

        let s = &s[bytes_read..];

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
                let data_len = little_endian_to_int(&[s[count]]).to_u32_digits().1[0] as usize;
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

        info!("count: {}, len: {}", count, len);
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
                    result.extend_from_slice(&int_to_little_endian(&BigInt::from(*op_code), 1));
                },
                ScriptCmd::Cmd(c) => {
                    let len = c.len();
                    if len < 75 {
                        result.extend_from_slice(&int_to_little_endian(&BigInt::from(len), 1));
                    } else if len > 75 && len < 0x100 {
                        result.extend_from_slice(&int_to_little_endian(&BigInt::from(76u8), 1));
                        result.extend_from_slice(&int_to_little_endian(&BigInt::from(len), 1));
                    } else if len >= 0x100 && len <= 520 {
                        result.extend_from_slice(&int_to_little_endian(&BigInt::from(77u8), 1));
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


#[cfg(test)]
mod tests_script {
    use log::info;
    use hex::ToHex;

    use crate::script::ScriptCmd;

    use super::Script;

    #[test]
    fn test_parse() {
        env_logger::init();

        let serialization = hex::decode("6b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccf\
        cf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8\
        e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278\
        a".to_string()).unwrap();

        let script = Script::parse(&serialization);
        info!("{}", script);

        match &script.cmds[0] {
            ScriptCmd::Cmd(cmd) => {
                info!("cmd: {}", cmd.encode_hex::<String>());
            },
            ScriptCmd::OpCode(op) => {
                info!("op: {:02x?}", op);
            }
        }
    }

    #[test]
    fn test_serialize() {
        env_logger::init();

        let serialization = hex::decode("6b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccf\
        cf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8\
        e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278\
        a".to_string()).unwrap();

        let script = Script::parse(&serialization);

        let script_serialized = script.serialize();
        info!("{}", script_serialized.encode_hex::<String>());

        assert_eq!(serialization, script_serialized);
    }
}
