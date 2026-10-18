//! 🧮️ Numeric execution: the numeric instruction dispatch and the unary, binary and comparison helpers. See [`super`].

use super::*;

//#region 🧮️NumericExecution
pub(super) fn execute_numeric(opcode: u8, machine: &mut Machine) -> Result<(), CoreError> {
    match opcode {
        0x45 => unary_i32(machine, |value| i32::from(value == 0))?,
        0x46 => compare_i32(machine, |left, right| left == right)?,
        0x47 => compare_i32(machine, |left, right| left != right)?,
        0x48 => compare_i32(machine, |left, right| left < right)?,
        0x49 => compare_u32(machine, |left, right| left < right)?,
        0x4a => compare_i32(machine, |left, right| left > right)?,
        0x4b => compare_u32(machine, |left, right| left > right)?,
        0x4c => compare_i32(machine, |left, right| left <= right)?,
        0x4d => compare_u32(machine, |left, right| left <= right)?,
        0x4e => compare_i32(machine, |left, right| left >= right)?,
        0x4f => compare_u32(machine, |left, right| left >= right)?,
        0x50 => unary_i64_to_i32(machine, |value| i32::from(value == 0))?,
        0x51 => compare_i64(machine, |left, right| left == right)?,
        0x52 => compare_i64(machine, |left, right| left != right)?,
        0x53 => compare_i64(machine, |left, right| left < right)?,
        0x54 => compare_u64(machine, |left, right| left < right)?,
        0x55 => compare_i64(machine, |left, right| left > right)?,
        0x56 => compare_u64(machine, |left, right| left > right)?,
        0x57 => compare_i64(machine, |left, right| left <= right)?,
        0x58 => compare_u64(machine, |left, right| left <= right)?,
        0x59 => compare_i64(machine, |left, right| left >= right)?,
        0x5a => compare_u64(machine, |left, right| left >= right)?,
        0x5b => compare_f32(machine, |left, right| left == right)?,
        0x5c => compare_f32(machine, |left, right| left != right)?,
        0x5d => compare_f32(machine, |left, right| left < right)?,
        0x5e => compare_f32(machine, |left, right| left > right)?,
        0x5f => compare_f32(machine, |left, right| left <= right)?,
        0x60 => compare_f32(machine, |left, right| left >= right)?,
        0x61 => compare_f64(machine, |left, right| left == right)?,
        0x62 => compare_f64(machine, |left, right| left != right)?,
        0x63 => compare_f64(machine, |left, right| left < right)?,
        0x64 => compare_f64(machine, |left, right| left > right)?,
        0x65 => compare_f64(machine, |left, right| left <= right)?,
        0x66 => compare_f64(machine, |left, right| left >= right)?,
        0x67 => unary_i32(machine, |value| value.leading_zeros() as i32)?,
        0x68 => unary_i32(machine, |value| value.trailing_zeros() as i32)?,
        0x69 => unary_i32(machine, |value| value.count_ones() as i32)?,
        0x6a => binary_i32(machine, i32::wrapping_add)?,
        0x6b => binary_i32(machine, i32::wrapping_sub)?,
        0x6c => binary_i32(machine, i32::wrapping_mul)?,
        0x6d => binary_i32_result(machine, signed_div_i32)?,
        0x6e => binary_u32_result(machine, |left, right| left.checked_div(right).ok_or_else(|| CoreError::Trap("integer divide by zero".into())))?,
        0x6f => binary_i32_result(machine, |left, right| {
            if right == 0 {
                Err(CoreError::Trap("integer divide by zero".into()))
            } else if left == i32::MIN && right == -1 {
                Ok(0)
            } else {
                Ok(left % right)
            }
        })?,
        0x70 => binary_u32_result(machine, |left, right| if right == 0 { Err(CoreError::Trap("integer divide by zero".into())) } else { Ok(left % right) })?,
        0x71 => binary_i32(machine, |left, right| left & right)?,
        0x72 => binary_i32(machine, |left, right| left | right)?,
        0x73 => binary_i32(machine, |left, right| left ^ right)?,
        0x74 => binary_i32(machine, |left, right| left.wrapping_shl(right as u32 & 31))?,
        0x75 => binary_i32(machine, |left, right| left.wrapping_shr(right as u32 & 31))?,
        0x76 => binary_u32(machine, |left, right| left.wrapping_shr(right & 31))?,
        0x77 => binary_i32(machine, |left, right| left.rotate_left(right as u32 & 31))?,
        0x78 => binary_i32(machine, |left, right| left.rotate_right(right as u32 & 31))?,
        0x79 => unary_i64(machine, |value| value.leading_zeros() as i64)?,
        0x7a => unary_i64(machine, |value| value.trailing_zeros() as i64)?,
        0x7b => unary_i64(machine, |value| value.count_ones() as i64)?,
        0x7c => binary_i64(machine, i64::wrapping_add)?,
        0x7d => binary_i64(machine, i64::wrapping_sub)?,
        0x7e => binary_i64(machine, i64::wrapping_mul)?,
        0x7f => binary_i64_result(machine, signed_div_i64)?,
        0x80 => binary_u64_result(machine, |left, right| left.checked_div(right).ok_or_else(|| CoreError::Trap("integer divide by zero".into())))?,
        0x81 => binary_i64_result(machine, |left, right| {
            if right == 0 {
                Err(CoreError::Trap("integer divide by zero".into()))
            } else if left == i64::MIN && right == -1 {
                Ok(0)
            } else {
                Ok(left % right)
            }
        })?,
        0x82 => binary_u64_result(machine, |left, right| if right == 0 { Err(CoreError::Trap("integer divide by zero".into())) } else { Ok(left % right) })?,
        0x83 => binary_i64(machine, |left, right| left & right)?,
        0x84 => binary_i64(machine, |left, right| left | right)?,
        0x85 => binary_i64(machine, |left, right| left ^ right)?,
        0x86 => binary_i64(machine, |left, right| left.wrapping_shl(right as u32 & 63))?,
        0x87 => binary_i64(machine, |left, right| left.wrapping_shr(right as u32 & 63))?,
        0x88 => binary_u64(machine, |left, right| left.wrapping_shr(right as u32 & 63))?,
        0x89 => binary_i64(machine, |left, right| left.rotate_left(right as u32 & 63))?,
        0x8a => binary_i64(machine, |left, right| left.rotate_right(right as u32 & 63))?,
        0x8b => unary_f32(machine, f32::abs)?,
        0x8c => unary_f32(machine, |value| -value)?,
        0x8d => unary_f32(machine, f32::ceil)?,
        0x8e => unary_f32(machine, f32::floor)?,
        0x8f => unary_f32(machine, f32::trunc)?,
        0x90 => unary_f32(machine, round_ties_even_f32)?,
        0x91 => unary_f32(machine, f32::sqrt)?,
        0x92 => binary_f32(machine, |left, right| left + right)?,
        0x93 => binary_f32(machine, |left, right| left - right)?,
        0x94 => binary_f32(machine, |left, right| left * right)?,
        0x95 => binary_f32(machine, |left, right| left / right)?,
        0x96 => binary_f32(machine, wasm_min_f32)?,
        0x97 => binary_f32(machine, wasm_max_f32)?,
        0x98 => binary_f32(machine, f32::copysign)?,
        0x99 => unary_f64(machine, f64::abs)?,
        0x9a => unary_f64(machine, |value| -value)?,
        0x9b => unary_f64(machine, f64::ceil)?,
        0x9c => unary_f64(machine, f64::floor)?,
        0x9d => unary_f64(machine, f64::trunc)?,
        0x9e => unary_f64(machine, round_ties_even_f64)?,
        0x9f => unary_f64(machine, f64::sqrt)?,
        0xa0 => binary_f64(machine, |left, right| left + right)?,
        0xa1 => binary_f64(machine, |left, right| left - right)?,
        0xa2 => binary_f64(machine, |left, right| left * right)?,
        0xa3 => binary_f64(machine, |left, right| left / right)?,
        0xa4 => binary_f64(machine, wasm_min_f64)?,
        0xa5 => binary_f64(machine, wasm_max_f64)?,
        0xa6 => binary_f64(machine, f64::copysign)?,
        0xa7 => map_top(machine, |value| Ok(Value::I32(value.as_i64()? as i32)))?,
        0xa8 => map_top(machine, |value| Ok(Value::I32(trunc_i32_from_f32(value.as_f32()?, true)?)))?,
        0xa9 => map_top(machine, |value| Ok(Value::I32(trunc_i32_from_f32(value.as_f32()?, false)?)))?,
        0xaa => map_top(machine, |value| Ok(Value::I32(trunc_i32_from_f64(value.as_f64()?, true)?)))?,
        0xab => map_top(machine, |value| Ok(Value::I32(trunc_i32_from_f64(value.as_f64()?, false)?)))?,
        0xac => map_top(machine, |value| Ok(Value::I64(value.as_i32()? as i64)))?,
        0xad => map_top(machine, |value| Ok(Value::I64(i64::from(value.as_i32()? as u32))))?,
        0xae => map_top(machine, |value| Ok(Value::I64(trunc_i64_from_f32(value.as_f32()?, true)?)))?,
        0xaf => map_top(machine, |value| Ok(Value::I64(trunc_i64_from_f32(value.as_f32()?, false)?)))?,
        0xb0 => map_top(machine, |value| Ok(Value::I64(trunc_i64_from_f64(value.as_f64()?, true)?)))?,
        0xb1 => map_top(machine, |value| Ok(Value::I64(trunc_i64_from_f64(value.as_f64()?, false)?)))?,
        0xb2 => map_top(machine, |value| Ok(Value::f32(value.as_i32()? as f32)))?,
        0xb3 => map_top(machine, |value| Ok(Value::f32(value.as_i32()? as u32 as f32)))?,
        0xb4 => map_top(machine, |value| Ok(Value::f32(value.as_i64()? as f32)))?,
        0xb5 => map_top(machine, |value| Ok(Value::f32(value.as_i64()? as u64 as f32)))?,
        0xb6 => map_top(machine, |value| Ok(Value::f32(value.as_f64()? as f32)))?,
        0xb7 => map_top(machine, |value| Ok(Value::f64(value.as_i32()? as f64)))?,
        0xb8 => map_top(machine, |value| Ok(Value::f64(value.as_i32()? as u32 as f64)))?,
        0xb9 => map_top(machine, |value| Ok(Value::f64(value.as_i64()? as f64)))?,
        0xba => map_top(machine, |value| Ok(Value::f64(value.as_i64()? as u64 as f64)))?,
        0xbb => map_top(machine, |value| Ok(Value::f64(value.as_f32()? as f64)))?,
        0xbc => match pop(machine)? {
            Value::F32(bits) => machine.values.push(Value::I32(bits as i32)),
            _ => return Err(CoreError::Trap("i32.reinterpret_f32 operand is not f32".into())),
        },
        0xbd => match pop(machine)? {
            Value::F64(bits) => machine.values.push(Value::I64(bits as i64)),
            _ => return Err(CoreError::Trap("i64.reinterpret_f64 operand is not f64".into())),
        },
        0xbe => map_top(machine, |value| Ok(Value::F32(value.as_i32()? as u32)))?,
        0xbf => map_top(machine, |value| Ok(Value::F64(value.as_i64()? as u64)))?,
        0xc0 => unary_i32(machine, |value| value as i8 as i32)?,
        0xc1 => unary_i32(machine, |value| value as i16 as i32)?,
        0xc2 => unary_i64(machine, |value| value as i8 as i64)?,
        0xc3 => unary_i64(machine, |value| value as i16 as i64)?,
        0xc4 => unary_i64(machine, |value| value as i32 as i64)?,
        _ => return Err(CoreError::Trap(format!("unsupported numeric opcode 0x{opcode:02x}"))),
    }
    Ok(())
}

pub(super) fn map_top(machine: &mut Machine, operation: impl FnOnce(Value) -> Result<Value, CoreError>) -> Result<(), CoreError> {
    let input = pop(machine)?;
    let output = operation(input)?;
    machine.values.push(output);
    Ok(())
}

fn unary_i32(machine: &mut Machine, operation: impl FnOnce(i32) -> i32) -> Result<(), CoreError> {
    let value = pop(machine)?.as_i32()?;
    machine.values.push(Value::I32(operation(value)));
    Ok(())
}

fn unary_i64(machine: &mut Machine, operation: impl FnOnce(i64) -> i64) -> Result<(), CoreError> {
    let value = pop(machine)?.as_i64()?;
    machine.values.push(Value::I64(operation(value)));
    Ok(())
}

fn unary_i64_to_i32(machine: &mut Machine, operation: impl FnOnce(i64) -> i32) -> Result<(), CoreError> {
    let value = pop(machine)?.as_i64()?;
    machine.values.push(Value::I32(operation(value)));
    Ok(())
}

fn unary_f32(machine: &mut Machine, operation: impl FnOnce(f32) -> f32) -> Result<(), CoreError> {
    let value = pop(machine)?.as_f32()?;
    machine.values.push(Value::f32(operation(value)));
    Ok(())
}

fn unary_f64(machine: &mut Machine, operation: impl FnOnce(f64) -> f64) -> Result<(), CoreError> {
    let value = pop(machine)?.as_f64()?;
    machine.values.push(Value::f64(operation(value)));
    Ok(())
}

fn binary_i32(machine: &mut Machine, operation: impl FnOnce(i32, i32) -> i32) -> Result<(), CoreError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(operation(left, right)));
    Ok(())
}

fn binary_i32_result(machine: &mut Machine, operation: impl FnOnce(i32, i32) -> Result<i32, CoreError>) -> Result<(), CoreError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(operation(left, right)?));
    Ok(())
}

fn binary_u32(machine: &mut Machine, operation: impl FnOnce(u32, u32) -> u32) -> Result<(), CoreError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(operation(left as u32, right as u32) as i32));
    Ok(())
}

fn binary_u32_result(machine: &mut Machine, operation: impl FnOnce(u32, u32) -> Result<u32, CoreError>) -> Result<(), CoreError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(operation(left as u32, right as u32)? as i32));
    Ok(())
}

fn binary_i64(machine: &mut Machine, operation: impl FnOnce(i64, i64) -> i64) -> Result<(), CoreError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I64(operation(left, right)));
    Ok(())
}

fn binary_i64_result(machine: &mut Machine, operation: impl FnOnce(i64, i64) -> Result<i64, CoreError>) -> Result<(), CoreError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I64(operation(left, right)?));
    Ok(())
}

fn binary_u64(machine: &mut Machine, operation: impl FnOnce(u64, u64) -> u64) -> Result<(), CoreError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I64(operation(left as u64, right as u64) as i64));
    Ok(())
}

fn binary_u64_result(machine: &mut Machine, operation: impl FnOnce(u64, u64) -> Result<u64, CoreError>) -> Result<(), CoreError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I64(operation(left as u64, right as u64)? as i64));
    Ok(())
}

fn binary_f32(machine: &mut Machine, operation: impl FnOnce(f32, f32) -> f32) -> Result<(), CoreError> {
    let right = pop(machine)?.as_f32()?;
    let left = pop(machine)?.as_f32()?;
    machine.values.push(Value::f32(operation(left, right)));
    Ok(())
}

fn binary_f64(machine: &mut Machine, operation: impl FnOnce(f64, f64) -> f64) -> Result<(), CoreError> {
    let right = pop(machine)?.as_f64()?;
    let left = pop(machine)?.as_f64()?;
    machine.values.push(Value::f64(operation(left, right)));
    Ok(())
}

fn compare_i32(machine: &mut Machine, operation: impl FnOnce(i32, i32) -> bool) -> Result<(), CoreError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(i32::from(operation(left, right))));
    Ok(())
}

fn compare_u32(machine: &mut Machine, operation: impl FnOnce(u32, u32) -> bool) -> Result<(), CoreError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(i32::from(operation(left as u32, right as u32))));
    Ok(())
}

fn compare_i64(machine: &mut Machine, operation: impl FnOnce(i64, i64) -> bool) -> Result<(), CoreError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I32(i32::from(operation(left, right))));
    Ok(())
}

fn compare_u64(machine: &mut Machine, operation: impl FnOnce(u64, u64) -> bool) -> Result<(), CoreError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I32(i32::from(operation(left as u64, right as u64))));
    Ok(())
}

fn compare_f32(machine: &mut Machine, operation: impl FnOnce(f32, f32) -> bool) -> Result<(), CoreError> {
    let right = pop(machine)?.as_f32()?;
    let left = pop(machine)?.as_f32()?;
    machine.values.push(Value::I32(i32::from(operation(left, right))));
    Ok(())
}

fn compare_f64(machine: &mut Machine, operation: impl FnOnce(f64, f64) -> bool) -> Result<(), CoreError> {
    let right = pop(machine)?.as_f64()?;
    let left = pop(machine)?.as_f64()?;
    machine.values.push(Value::I32(i32::from(operation(left, right))));
    Ok(())
}

fn pop_i32_pair(machine: &mut Machine) -> Result<(i32, i32), CoreError> {
    let right = pop(machine)?.as_i32()?;
    let left = pop(machine)?.as_i32()?;
    Ok((left, right))
}

fn pop_i64_pair(machine: &mut Machine) -> Result<(i64, i64), CoreError> {
    let right = pop(machine)?.as_i64()?;
    let left = pop(machine)?.as_i64()?;
    Ok((left, right))
}

fn signed_div_i32(left: i32, right: i32) -> Result<i32, CoreError> {
    if right == 0 {
        Err(CoreError::Trap("integer divide by zero".into()))
    } else if left == i32::MIN && right == -1 {
        Err(CoreError::Trap("integer overflow".into()))
    } else {
        Ok(left / right)
    }
}

fn signed_div_i64(left: i64, right: i64) -> Result<i64, CoreError> {
    if right == 0 {
        Err(CoreError::Trap("integer divide by zero".into()))
    } else if left == i64::MIN && right == -1 {
        Err(CoreError::Trap("integer overflow".into()))
    } else {
        Ok(left / right)
    }
}
//#endregion 🧮️NumericExecution
