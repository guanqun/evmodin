use crate::{common::*, state::*};

pub(crate) fn push(stack: &mut Stack, code: &[u8], push_len: usize) {
    stack.push(u256_from_slice(&code[..push_len]));
}

pub(crate) fn dup(stack: &mut Stack, height: usize) {
    stack.push(*stack.get(height - 1));
}

pub(crate) fn swap(stack: &mut Stack, height: usize) {
    stack.swap_top(height);
}

pub(crate) fn pop(stack: &mut Stack) {
    stack.pop();
}
