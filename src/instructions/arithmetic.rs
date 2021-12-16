use crate::{state::*, Revision, StatusCode};
use ethereum_types::U512;
use ethnum::U256;
use i256::I256;

pub(crate) fn add(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();
    stack.push(a.overflowing_add(b).0);
}

pub(crate) fn mul(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();
    stack.push(a.overflowing_mul(b).0);
}

pub(crate) fn sub(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();
    stack.push(a.overflowing_sub(b).0);
}

pub(crate) fn div(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();
    stack.push(if b == 0 { U256::ZERO } else { a / b });
}

pub(crate) fn sdiv(stack: &mut Stack) {
    let a = I256::from(stack.pop());
    let b = I256::from(stack.pop());
    let v = a / b;
    stack.push(v.into());
}

pub(crate) fn modulo(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();
    let v = if b == 0 { U256::ZERO } else { a % b };
    stack.push(v);
}

pub(crate) fn smod(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();

    let v = if b == 0 {
        U256::ZERO
    } else {
        let v = I256::from(a) % I256::from(b);
        v.into()
    };

    stack.push(v);
}

pub(crate) fn addmod(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();
    let c = stack.pop();

    let v = if c == 0 {
        U256::ZERO
    } else {
        let v = ethereum_types::U256::try_from(
            (U512::from_big_endian(&a.to_be_bytes()) + U512::from_big_endian(&b.to_be_bytes()))
                % U512::from_big_endian(&c.to_be_bytes()),
        )
        .unwrap();
        let mut arr = [0; 32];
        v.to_big_endian(&mut arr);
        U256::from_be_bytes(arr)
    };

    stack.push(v);
}

pub(crate) fn mulmod(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();
    let c = stack.pop();

    let v = if c == 0 {
        U256::ZERO
    } else {
        let v = ethereum_types::U256::try_from(
            (U512::from_big_endian(&a.to_be_bytes()) * U512::from_big_endian(&b.to_be_bytes()))
                % U512::from_big_endian(&c.to_be_bytes()),
        )
        .unwrap();
        let mut arr = [0; 32];
        v.to_big_endian(&mut arr);
        U256::from_be_bytes(arr)
    };

    stack.push(v);
}

fn log2floor(value: U256) -> u64 {
    assert!(value != 0);
    let mut l: u64 = 256;
    for i in 0..=1 {
        let i = 1 - i;
        if value.0[i] == 0 {
            l -= 128;
        } else {
            l -= value.0[i].leading_zeros() as u64;
            if l == 0 {
                return l;
            } else {
                return l - 1;
            }
        }
    }
    l
}

pub(crate) fn exp(state: &mut ExecutionState) -> Result<(), StatusCode> {
    let mut base = state.stack.pop();
    let mut power = state.stack.pop();

    if power > 0 {
        let additional_gas = if state.evm_revision >= Revision::Spurious {
            50
        } else {
            10
        } * (log2floor(power) / 8 + 1);

        state.gas_left -= additional_gas as i64;

        if state.gas_left < 0 {
            return Err(StatusCode::OutOfGas);
        }
    }

    let mut v = U256::ONE;

    while power > 0 {
        if (power & 1) != 0 {
            v = v.overflowing_mul(base).0;
        }
        power >>= 1;
        base = base.overflowing_mul(base).0;
    }

    state.stack.push(v);

    Ok(())
}

pub(crate) fn signextend(stack: &mut Stack) {
    let a = stack.pop();
    let b = stack.pop();

    let v = if a < 32 {
        let bit_index = (8 * a.as_u32() + 7) as usize;
        let (hi, lo) = b.into_words();
        let bit = if bit_index > 0x7f { hi } else { lo } & (1 << (bit_index % 128)) != 0;
        let mask = (U256::ONE << bit_index) - U256::ONE;
        if bit {
            b | !mask
        } else {
            b & mask
        }
    } else {
        b
    };

    stack.push(v);
}
