use crate::utils::{hash256, hash160};

pub fn op_dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    stack.push(stack[stack.len() - 1].clone());
    return true;
}

pub fn op_hash256(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let element = stack.pop().unwrap();
    stack.push(hash256(&element));
    return true;
}

pub fn op_hash160(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let element = stack.pop().unwrap();
    stack.push(hash160(&element));
    return true;
}
