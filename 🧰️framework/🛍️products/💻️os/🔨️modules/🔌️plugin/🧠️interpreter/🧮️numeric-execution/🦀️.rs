//! 🧮️ Numeric execution: the numeric instruction dispatch and the unary, binary and comparison helpers. See [`super`].

use super::*;
/// 🧮️ Numeric failures remain fixed scalar facts until an ordinary caller chooses diagnostic text.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(super) enum NumericError{Arity,Type,Division,Overflow,Conversion,Unsupported}
impl NumericError{fn message(self)->&'static str{match self{Self::Arity=>"numeric operand stack underflow",Self::Type=>"numeric operand type mismatch",Self::Division=>"integer divide by zero",Self::Overflow=>"integer overflow",Self::Conversion=>"invalid conversion to integer",Self::Unsupported=>"unsupported numeric opcode"}}}
struct NumericValues{items:[Value;2],count:usize}
impl NumericValues{fn push(&mut self,value:Value){assert!(self.count<2);self.items[self.count]=value;self.count+=1;}fn pop(&mut self)->Option<Value>{if self.count==0{return None}self.count-=1;Some(self.items[self.count])}}
struct NumericMachine{values:NumericValues}
fn pop(machine:&mut NumericMachine)->Result<Value,NumericError>{machine.values.pop().ok_or(NumericError::Arity)}
trait NumericScalar{fn numeric_i32(self)->Result<i32,NumericError>;fn numeric_i64(self)->Result<i64,NumericError>;fn numeric_f32(self)->Result<f32,NumericError>;fn numeric_f64(self)->Result<f64,NumericError>;}
impl NumericScalar for Value{fn numeric_i32(self)->Result<i32,NumericError>{match self{Value::I32(value)=>Ok(value),_=>Err(NumericError::Type)}}fn numeric_i64(self)->Result<i64,NumericError>{match self{Value::I64(value)=>Ok(value),_=>Err(NumericError::Type)}}fn numeric_f32(self)->Result<f32,NumericError>{match self{Value::F32(value)=>Ok(f32::from_bits(value)),_=>Err(NumericError::Type)}}fn numeric_f64(self)->Result<f64,NumericError>{match self{Value::F64(value)=>Ok(f64::from_bits(value)),_=>Err(NumericError::Type)}}}
pub(super) fn numeric_arity(opcode:u8)->Result<usize,NumericError>{if !(0x45..=0xc4).contains(&opcode){return Err(NumericError::Unsupported)}Ok(if matches!(opcode,0x46..=0x4f|0x51..=0x66|0x6a..=0x78|0x7c..=0x8a|0x92..=0x98|0xa0..=0xa6){2}else{1})}
/// 🔢️ The shared numeric evaluator uses two stack scalars and returns a fixed failure without heap effects.
pub(super) fn execute_numeric_fixed(opcode:u8,operands:&[Value])->Result<Value,NumericError>{if operands.len()!=numeric_arity(opcode)?{return Err(NumericError::Arity)}let mut machine=NumericMachine{values:NumericValues{items:[Value::I64(0);2],count:0}};for value in operands{machine.values.push(*value)}dispatch_numeric(opcode,&mut machine)?;if machine.values.count!=1{return Err(NumericError::Arity)}Ok(machine.values.items[0])}
pub(super) fn execute_numeric(opcode:u8,machine:&mut Machine)->Result<(),CoreError>{let count=numeric_arity(opcode).map_err(|error|CoreError::Trap(error.message().into()))?;let start=machine.values.len().checked_sub(count).ok_or_else(||CoreError::Trap(NumericError::Arity.message().into()))?;let value=execute_numeric_fixed(opcode,&machine.values[start..]).map_err(|error|CoreError::Trap(error.message().into()))?;machine.values.truncate(start);machine.values.push(value);Ok(())}
pub(super) fn map_top(machine:&mut Machine,operation:impl FnOnce(Value)->Result<Value,CoreError>)->Result<(),CoreError>{let input=super::pop(machine)?;let output=operation(input)?;machine.values.push(output);Ok(())}


//#region 🧮️NumericExecution
fn dispatch_numeric(opcode: u8, machine: &mut NumericMachine) -> Result<(), NumericError> {
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
        0x6e => binary_u32_result(machine, |left, right| left.checked_div(right).ok_or_else(|| NumericError::Division))?,
        0x6f => binary_i32_result(machine, |left, right| {
            if right == 0 {
                Err(NumericError::Division)
            } else if left == i32::MIN && right == -1 {
                Ok(0)
            } else {
                Ok(left % right)
            }
        })?,
        0x70 => binary_u32_result(machine, |left, right| if right == 0 { Err(NumericError::Division) } else { Ok(left % right) })?,
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
        0x80 => binary_u64_result(machine, |left, right| left.checked_div(right).ok_or_else(|| NumericError::Division))?,
        0x81 => binary_i64_result(machine, |left, right| {
            if right == 0 {
                Err(NumericError::Division)
            } else if left == i64::MIN && right == -1 {
                Ok(0)
            } else {
                Ok(left % right)
            }
        })?,
        0x82 => binary_u64_result(machine, |left, right| if right == 0 { Err(NumericError::Division) } else { Ok(left % right) })?,
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
        0xa7 => map_numeric(machine, |value| Ok(Value::I32(value.numeric_i64()? as i32)))?,
        0xa8 => map_numeric(machine, |value| Ok(Value::I32(trunc_i32_from_f32(value.numeric_f32()?, true)?)))?,
        0xa9 => map_numeric(machine, |value| Ok(Value::I32(trunc_i32_from_f32(value.numeric_f32()?, false)?)))?,
        0xaa => map_numeric(machine, |value| Ok(Value::I32(trunc_i32_from_f64(value.numeric_f64()?, true)?)))?,
        0xab => map_numeric(machine, |value| Ok(Value::I32(trunc_i32_from_f64(value.numeric_f64()?, false)?)))?,
        0xac => map_numeric(machine, |value| Ok(Value::I64(value.numeric_i32()? as i64)))?,
        0xad => map_numeric(machine, |value| Ok(Value::I64(i64::from(value.numeric_i32()? as u32))))?,
        0xae => map_numeric(machine, |value| Ok(Value::I64(trunc_i64_from_f32(value.numeric_f32()?, true)?)))?,
        0xaf => map_numeric(machine, |value| Ok(Value::I64(trunc_i64_from_f32(value.numeric_f32()?, false)?)))?,
        0xb0 => map_numeric(machine, |value| Ok(Value::I64(trunc_i64_from_f64(value.numeric_f64()?, true)?)))?,
        0xb1 => map_numeric(machine, |value| Ok(Value::I64(trunc_i64_from_f64(value.numeric_f64()?, false)?)))?,
        0xb2 => map_numeric(machine, |value| Ok(Value::f32(value.numeric_i32()? as f32)))?,
        0xb3 => map_numeric(machine, |value| Ok(Value::f32(value.numeric_i32()? as u32 as f32)))?,
        0xb4 => map_numeric(machine, |value| Ok(Value::f32(value.numeric_i64()? as f32)))?,
        0xb5 => map_numeric(machine, |value| Ok(Value::f32(value.numeric_i64()? as u64 as f32)))?,
        0xb6 => map_numeric(machine, |value| Ok(Value::f32(value.numeric_f64()? as f32)))?,
        0xb7 => map_numeric(machine, |value| Ok(Value::f64(value.numeric_i32()? as f64)))?,
        0xb8 => map_numeric(machine, |value| Ok(Value::f64(value.numeric_i32()? as u32 as f64)))?,
        0xb9 => map_numeric(machine, |value| Ok(Value::f64(value.numeric_i64()? as f64)))?,
        0xba => map_numeric(machine, |value| Ok(Value::f64(value.numeric_i64()? as u64 as f64)))?,
        0xbb => map_numeric(machine, |value| Ok(Value::f64(value.numeric_f32()? as f64)))?,
        0xbc => match pop(machine)? {
            Value::F32(bits) => machine.values.push(Value::I32(bits as i32)),
            _ => return Err(NumericError::Type),
        },
        0xbd => match pop(machine)? {
            Value::F64(bits) => machine.values.push(Value::I64(bits as i64)),
            _ => return Err(NumericError::Type),
        },
        0xbe => map_numeric(machine, |value| Ok(Value::F32(value.numeric_i32()? as u32)))?,
        0xbf => map_numeric(machine, |value| Ok(Value::F64(value.numeric_i64()? as u64)))?,
        0xc0 => unary_i32(machine, |value| value as i8 as i32)?,
        0xc1 => unary_i32(machine, |value| value as i16 as i32)?,
        0xc2 => unary_i64(machine, |value| value as i8 as i64)?,
        0xc3 => unary_i64(machine, |value| value as i16 as i64)?,
        0xc4 => unary_i64(machine, |value| value as i32 as i64)?,
        _ => return Err(NumericError::Unsupported),
    }
    Ok(())
}

fn map_numeric(machine: &mut NumericMachine, operation: impl FnOnce(Value) -> Result<Value, NumericError>) -> Result<(), NumericError> {
    let input = pop(machine)?;
    let output = operation(input)?;
    machine.values.push(output);
    Ok(())
}

fn unary_i32(machine: &mut NumericMachine, operation: impl FnOnce(i32) -> i32) -> Result<(), NumericError> {
    let value = pop(machine)?.numeric_i32()?;
    machine.values.push(Value::I32(operation(value)));
    Ok(())
}

fn unary_i64(machine: &mut NumericMachine, operation: impl FnOnce(i64) -> i64) -> Result<(), NumericError> {
    let value = pop(machine)?.numeric_i64()?;
    machine.values.push(Value::I64(operation(value)));
    Ok(())
}

fn unary_i64_to_i32(machine: &mut NumericMachine, operation: impl FnOnce(i64) -> i32) -> Result<(), NumericError> {
    let value = pop(machine)?.numeric_i64()?;
    machine.values.push(Value::I32(operation(value)));
    Ok(())
}

fn unary_f32(machine: &mut NumericMachine, operation: impl FnOnce(f32) -> f32) -> Result<(), NumericError> {
    let value = pop(machine)?.numeric_f32()?;
    machine.values.push(Value::f32(operation(value)));
    Ok(())
}

fn unary_f64(machine: &mut NumericMachine, operation: impl FnOnce(f64) -> f64) -> Result<(), NumericError> {
    let value = pop(machine)?.numeric_f64()?;
    machine.values.push(Value::f64(operation(value)));
    Ok(())
}

fn binary_i32(machine: &mut NumericMachine, operation: impl FnOnce(i32, i32) -> i32) -> Result<(), NumericError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(operation(left, right)));
    Ok(())
}

fn binary_i32_result(machine: &mut NumericMachine, operation: impl FnOnce(i32, i32) -> Result<i32, NumericError>) -> Result<(), NumericError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(operation(left, right)?));
    Ok(())
}

fn binary_u32(machine: &mut NumericMachine, operation: impl FnOnce(u32, u32) -> u32) -> Result<(), NumericError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(operation(left as u32, right as u32) as i32));
    Ok(())
}

fn binary_u32_result(machine: &mut NumericMachine, operation: impl FnOnce(u32, u32) -> Result<u32, NumericError>) -> Result<(), NumericError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(operation(left as u32, right as u32)? as i32));
    Ok(())
}

fn binary_i64(machine: &mut NumericMachine, operation: impl FnOnce(i64, i64) -> i64) -> Result<(), NumericError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I64(operation(left, right)));
    Ok(())
}

fn binary_i64_result(machine: &mut NumericMachine, operation: impl FnOnce(i64, i64) -> Result<i64, NumericError>) -> Result<(), NumericError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I64(operation(left, right)?));
    Ok(())
}

fn binary_u64(machine: &mut NumericMachine, operation: impl FnOnce(u64, u64) -> u64) -> Result<(), NumericError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I64(operation(left as u64, right as u64) as i64));
    Ok(())
}

fn binary_u64_result(machine: &mut NumericMachine, operation: impl FnOnce(u64, u64) -> Result<u64, NumericError>) -> Result<(), NumericError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I64(operation(left as u64, right as u64)? as i64));
    Ok(())
}

fn binary_f32(machine: &mut NumericMachine, operation: impl FnOnce(f32, f32) -> f32) -> Result<(), NumericError> {
    let right = pop(machine)?.numeric_f32()?;
    let left = pop(machine)?.numeric_f32()?;
    machine.values.push(Value::f32(operation(left, right)));
    Ok(())
}

fn binary_f64(machine: &mut NumericMachine, operation: impl FnOnce(f64, f64) -> f64) -> Result<(), NumericError> {
    let right = pop(machine)?.numeric_f64()?;
    let left = pop(machine)?.numeric_f64()?;
    machine.values.push(Value::f64(operation(left, right)));
    Ok(())
}

fn compare_i32(machine: &mut NumericMachine, operation: impl FnOnce(i32, i32) -> bool) -> Result<(), NumericError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(i32::from(operation(left, right))));
    Ok(())
}

fn compare_u32(machine: &mut NumericMachine, operation: impl FnOnce(u32, u32) -> bool) -> Result<(), NumericError> {
    let (left, right) = pop_i32_pair(machine)?;
    machine.values.push(Value::I32(i32::from(operation(left as u32, right as u32))));
    Ok(())
}

fn compare_i64(machine: &mut NumericMachine, operation: impl FnOnce(i64, i64) -> bool) -> Result<(), NumericError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I32(i32::from(operation(left, right))));
    Ok(())
}

fn compare_u64(machine: &mut NumericMachine, operation: impl FnOnce(u64, u64) -> bool) -> Result<(), NumericError> {
    let (left, right) = pop_i64_pair(machine)?;
    machine.values.push(Value::I32(i32::from(operation(left as u64, right as u64))));
    Ok(())
}

fn compare_f32(machine: &mut NumericMachine, operation: impl FnOnce(f32, f32) -> bool) -> Result<(), NumericError> {
    let right = pop(machine)?.numeric_f32()?;
    let left = pop(machine)?.numeric_f32()?;
    machine.values.push(Value::I32(i32::from(operation(left, right))));
    Ok(())
}

fn compare_f64(machine: &mut NumericMachine, operation: impl FnOnce(f64, f64) -> bool) -> Result<(), NumericError> {
    let right = pop(machine)?.numeric_f64()?;
    let left = pop(machine)?.numeric_f64()?;
    machine.values.push(Value::I32(i32::from(operation(left, right))));
    Ok(())
}

fn pop_i32_pair(machine: &mut NumericMachine) -> Result<(i32, i32), NumericError> {
    let right = pop(machine)?.numeric_i32()?;
    let left = pop(machine)?.numeric_i32()?;
    Ok((left, right))
}

fn pop_i64_pair(machine: &mut NumericMachine) -> Result<(i64, i64), NumericError> {
    let right = pop(machine)?.numeric_i64()?;
    let left = pop(machine)?.numeric_i64()?;
    Ok((left, right))
}

fn signed_div_i32(left: i32, right: i32) -> Result<i32, NumericError> {
    if right == 0 {
        Err(NumericError::Division)
    } else if left == i32::MIN && right == -1 {
        Err(NumericError::Overflow)
    } else {
        Ok(left / right)
    }
}

fn signed_div_i64(left: i64, right: i64) -> Result<i64, NumericError> {
    if right == 0 {
        Err(NumericError::Division)
    } else if left == i64::MIN && right == -1 {
        Err(NumericError::Overflow)
    } else {
        Ok(left / right)
    }
}
//#endregion 🧮️NumericExecution

#[cfg(test)]
mod original_numeric_tests{
 use super::*;
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 use semio_framework_actor::{RetainedTurnInput,OriginalGuestExecutionBudget};
 use semio_framework_value::RetainedCloneGrant;
 fn clock()->Option<u64>{Some(1000)}
 fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:2_000_000,maximum_capacity_bytes:2_000_000,maximum_release_bytes:2_000_000,maximum_depth:64}}
 fn binary(opcode:u8,count:usize)->Vec<u8>{let mut bytes=b"\0asm\x01\0\0\0".to_vec();let mut section=|id:u8,payload:Vec<u8>|{bytes.push(id);bytes.push(payload.len()as u8);bytes.extend(payload)};let mut ty=vec![1,0x60,count as u8];ty.extend(std::iter::repeat_n(0x7f,count));ty.extend([1,0x7f]);section(1,ty);section(3,vec![1,0]);section(7,vec![1,3,b'r',b'u',b'n',0,0]);let mut body=vec![0];for index in 0..count{body.extend([0x20,index as u8])}body.extend([opcode,0x0b]);let mut code=vec![1,body.len()as u8];code.extend(body);section(10,code);bytes}
 /// 🧮️ The independent VM agrees with the shared fixed evaluator and actual original instruction machine, including zero-heap traps.
 #[test]
 fn original_numeric_primitives_preserve_measured_heap_and_match_independent_vm(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
  for row in fixture["cases"].as_array().unwrap(){
   let args:Vec<Value>=row["args"].as_array().unwrap().iter().map(|value|Value::I32(value.as_i64().unwrap()as i32)).collect();let opcode=row["opcode"].as_u64().unwrap()as u8;
   let(fixed,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||execute_numeric_fixed(opcode,&args));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let bytes=binary(opcode,args.len());let engine=wasmtime::Engine::default();let module=wasmtime::Module::new(&engine,&bytes).unwrap();let mut store=wasmtime::Store::new(&engine,());let oracle=wasmtime::Instance::new(&mut store,&module,&[]).unwrap();let argument=|index:usize|match args[index]{Value::I32(value)=>value,_=>unreachable!()};let actual=if args.len()==1{oracle.get_typed_func::<i32,i32>(&mut store,"run").unwrap().call(&mut store,argument(0))}else{oracle.get_typed_func::<(i32,i32),i32>(&mut store,"run").unwrap().call(&mut store,(argument(0),argument(1)))};
   assert_eq!(actual.is_err(),row["trap"].as_bool().unwrap_or(false));if let Ok(value)=actual{assert_eq!(fixed,Ok(Value::I32(value)));assert_eq!(i64::from(value),row["expected"].as_i64().unwrap());}else{assert!(fixed.is_err())}
   let module=Arc::new(CoreModule::parse(&bytes).unwrap());let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(91),Generation(5),stock,clock,1000000,1000000);let mut epoch=1;let mut owner=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;CoreInstanceOpening::begin(&module,cx)}).unwrap().unwrap();for _ in 0..100{if owner.resources_ready(){break}epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.advance(&module,cx)}).unwrap();}epoch+=1;assert!(caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.begin_function_step(&module,0,&args,cx)}).unwrap());
   let mut refused=false;for _ in 0..100{epoch+=1;let(call,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;let input=RetainedTurnInput::borrow_original(cx)?;owner.advance_scalar(&module,OriginalGuestExecutionBudget{input,fuel:1,deadline_us:cx.deadline_us()},cx)}).unwrap());assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),(heap.requested_bytes,heap.released_bytes));assert!(call.execution.spent_fuel.unwrap()<=1);match call.result{Ok(CoreOriginalInvocationStatus::Complete)=>break,Err(_)=>{refused=true;break},_=>{}}}
   assert_eq!(refused,row["trap"].as_bool().unwrap_or(false));if !refused{assert_eq!(owner.invocation_values(),Some(&[fixed.unwrap()][..]));}
   caller.cancel().cancel_now();for _ in 0..100{if owner.terminal_is_empty(){break}epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.close_step(&module,cx)}).unwrap();}assert!(owner.terminal_is_empty());caller.retire(stock);
  }
 }
}
