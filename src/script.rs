use std::fmt::Display;

#[derive(Clone)]
pub struct Script {

}

impl Script {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn parse(s: &[u8]) -> Self {
        Self {

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
