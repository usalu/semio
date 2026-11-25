//! 🧮️ Numeric instruction integration with the core operand stack.

use super::*;

pub(super) fn execute_numeric(opcode:u8,machine:&mut Machine)->Result<(),CoreError>{let count=numeric_arity(opcode).map_err(|error|CoreError::Trap(error.message().into()))?;let start=machine.values.len().checked_sub(count).ok_or_else(||CoreError::Trap(NumericError::Arity.message().into()))?;let value=execute_numeric_fixed(opcode,&machine.values[start..]).map_err(|error|CoreError::Trap(error.message().into()))?;machine.values.truncate(start);machine.values.push(value);Ok(())}
pub(super) fn map_top(machine:&mut Machine,operation:impl FnOnce(Value)->Result<Value,CoreError>)->Result<(),CoreError>{let input=super::pop(machine)?;let output=operation(input)?;machine.values.push(output);Ok(())}

