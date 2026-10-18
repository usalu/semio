//! 🧭️ Instruction shape: the control analysis of function bodies, the immediate skippers and the block type decoder. See [`super`].

use super::*;

//#region 🧭️InstructionShape
pub(super) fn analyze_controls(code: &[u8]) -> Result<BTreeMap<usize, ControlBounds>, CoreError> {
    let mut decoder = Decoder::new(code);
    let mut open: Vec<(usize, Option<usize>)> = Vec::new();
    let mut controls = BTreeMap::new();
    while !decoder.done() {
        let instruction_pc = decoder.position;
        let opcode = decoder.byte()?;
        match opcode {
            0x02..=0x04 => {
                skip_block_type(&mut decoder)?;
                open.push((instruction_pc, None));
            }
            0x05 => {
                let Some((_, else_pc)) = open.last_mut() else {
                    return Err(CoreError::Validation("else without an open if".into()));
                };
                if else_pc.replace(instruction_pc).is_some() {
                    return Err(CoreError::Validation("if has more than one else".into()));
                }
            }
            0x0b => {
                if let Some((start, else_pc)) = open.pop() {
                    controls.insert(start, ControlBounds { else_pc, end_pc: instruction_pc });
                } else if !decoder.done() {
                    return Err(CoreError::Validation("function end is followed by instructions".into()));
                }
            }
            _ => skip_immediate(opcode, &mut decoder)?,
        }
    }
    if !open.is_empty() {
        return Err(CoreError::Validation("unterminated structured control instruction".into()));
    }
    if code.last() != Some(&0x0b) {
        return Err(CoreError::Validation("function body has no final end".into()));
    }
    Ok(controls)
}

fn skip_block_type(decoder: &mut Decoder<'_>) -> Result<(), CoreError> {
    let first = decoder.byte()?;
    if first == 0x40 || ValueType::parse(first).is_ok() {
        return Ok(());
    }
    if first & 0x80 == 0 {
        return Ok(());
    }
    while decoder.byte()? & 0x80 != 0 {}
    Ok(())
}

fn skip_memarg(decoder: &mut Decoder<'_>) -> Result<(), CoreError> {
    decoder.u32()?;
    decoder.u64()?;
    Ok(())
}

fn skip_immediate(opcode: u8, decoder: &mut Decoder<'_>) -> Result<(), CoreError> {
    match opcode {
        0x0c | 0x0d | 0x10 | 0x12 | 0x20..=0x26 | 0xd2 => {
            decoder.u32()?;
        }
        0x0e => {
            let count = decoder.u32()?;
            for _ in 0..=count {
                decoder.u32()?;
            }
        }
        0x11 | 0x13 => {
            decoder.u32()?;
            decoder.u32()?;
        }
        0x1c => {
            let types = decoder.u32()?;
            for _ in 0..types {
                ValueType::parse(decoder.byte()?)?;
            }
        }
        0x28..=0x3e => skip_memarg(decoder)?,
        0x3f | 0x40 => {
            decoder.u32()?;
        }
        0x41 => {
            decoder.i32()?;
        }
        0x42 => {
            decoder.i64()?;
        }
        0x43 => {
            decoder.bytes(4)?;
        }
        0x44 => {
            decoder.bytes(8)?;
        }
        0xd0 => {
            ValueType::parse(decoder.byte()?)?;
        }
        0xfc => skip_fc(decoder)?,
        0xfd => return Err(CoreError::Validation("SIMD is outside the current Semio guest target feature set".into())),
        0xfe => return Err(CoreError::Validation("threads/atomics are outside the Semio plugin sandbox".into())),
        _ => {}
    }
    Ok(())
}

fn skip_fc(decoder: &mut Decoder<'_>) -> Result<(), CoreError> {
    match decoder.u32()? {
        0..=7 => {}
        8 => {
            decoder.u32()?;
            decoder.u32()?;
        }
        9 => {
            decoder.u32()?;
        }
        10 => {
            decoder.u32()?;
            decoder.u32()?;
        }
        11 => {
            decoder.u32()?;
        }
        12 => {
            decoder.u32()?;
            decoder.u32()?;
        }
        13 => {
            decoder.u32()?;
        }
        14 => {
            decoder.u32()?;
            decoder.u32()?;
        }
        15..=17 => {
            decoder.u32()?;
        }
        subopcode => return Err(CoreError::Validation(format!("unsupported 0xfc instruction {subopcode}"))),
    }
    Ok(())
}

pub(super) fn decode_block_type(decoder: &mut Decoder<'_>, types: &[FunctionType]) -> Result<FunctionType, CoreError> {
    let first = decoder.byte()?;
    if first == 0x40 {
        return Ok(FunctionType { parameters: Vec::new(), results: Vec::new() });
    }
    if let Ok(result) = ValueType::parse(first) {
        return Ok(FunctionType { parameters: Vec::new(), results: vec![result] });
    }
    let mut value = i64::from(first & 0x7f);
    let mut shift = 7u32;
    let mut byte = first;
    while byte & 0x80 != 0 {
        byte = decoder.byte()?;
        value |= i64::from(byte & 0x7f).checked_shl(shift).unwrap_or(0);
        shift += 7;
    }
    if byte & 0x40 != 0 && shift < 64 {
        value |= (!0i64) << shift;
    }
    if value < 0 {
        return Err(CoreError::Validation(format!("unsupported negative block type {value}")));
    }
    types.get(value as usize).cloned().ok_or_else(|| CoreError::Validation(format!("block type index {value} is out of bounds")))
}
//#endregion 🧭️InstructionShape
