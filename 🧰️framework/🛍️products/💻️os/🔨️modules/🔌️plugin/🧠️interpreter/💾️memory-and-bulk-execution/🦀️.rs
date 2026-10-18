//! 💾️ Memory and bulk execution: the memory and table instructions, the bulk memory operations and the little endian loads and stores. See [`super`].

use super::*;

//#region 💾️MemoryAndBulkExecution
impl CoreInstance {
    pub(super) fn execute_memory(&mut self, opcode: u8, decoder: &mut Decoder<'_>, machine: &mut Machine) -> Result<(), CoreError> {
        let _alignment = decoder.u32()?;
        let offset = decoder.u64()?;
        let memory64 = self.memories.first().is_some_and(|memory| memory.memory64);
        if opcode <= 0x35 {
            let base = if memory64 { pop(machine)?.as_i64()? as u64 } else { pop(machine)?.as_i32()? as u32 as u64 };
            let address = effective_address(base, offset)?;
            let memory = self.memories.first().ok_or_else(|| CoreError::Trap("memory instruction has no memory".into()))?;
            let value = match opcode {
                0x28 => Value::I32(read_u32(memory, address)? as i32),
                0x29 => Value::I64(read_u64(memory, address)? as i64),
                0x2a => Value::F32(read_u32(memory, address)?),
                0x2b => Value::F64(read_u64(memory, address)?),
                0x2c => Value::I32(read_u8(memory, address)? as i8 as i32),
                0x2d => Value::I32(i32::from(read_u8(memory, address)?)),
                0x2e => Value::I32(read_u16(memory, address)? as i16 as i32),
                0x2f => Value::I32(i32::from(read_u16(memory, address)?)),
                0x30 => Value::I64(read_u8(memory, address)? as i8 as i64),
                0x31 => Value::I64(i64::from(read_u8(memory, address)?)),
                0x32 => Value::I64(read_u16(memory, address)? as i16 as i64),
                0x33 => Value::I64(i64::from(read_u16(memory, address)?)),
                0x34 => Value::I64(read_u32(memory, address)? as i32 as i64),
                0x35 => Value::I64(i64::from(read_u32(memory, address)?)),
                _ => unreachable!(),
            };
            machine.values.push(value);
            return Ok(());
        }
        let value = pop(machine)?;
        let base = if memory64 { pop(machine)?.as_i64()? as u64 } else { pop(machine)?.as_i32()? as u32 as u64 };
        let address = effective_address(base, offset)?;
        let memory = self.memories.first_mut().ok_or_else(|| CoreError::Trap("memory instruction has no memory".into()))?;
        match opcode {
            0x36 => write_bytes(memory, address, &value.as_i32()?.to_le_bytes()),
            0x37 => write_bytes(memory, address, &value.as_i64()?.to_le_bytes()),
            0x38 => match value {
                Value::F32(bits) => write_bytes(memory, address, &bits.to_le_bytes()),
                _ => Err(CoreError::Trap("f32.store value is not f32".into())),
            },
            0x39 => match value {
                Value::F64(bits) => write_bytes(memory, address, &bits.to_le_bytes()),
                _ => Err(CoreError::Trap("f64.store value is not f64".into())),
            },
            0x3a => write_bytes(memory, address, &[value.as_i32()? as u8]),
            0x3b => write_bytes(memory, address, &(value.as_i32()? as u16).to_le_bytes()),
            0x3c => write_bytes(memory, address, &[value.as_i64()? as u8]),
            0x3d => write_bytes(memory, address, &(value.as_i64()? as u16).to_le_bytes()),
            0x3e => write_bytes(memory, address, &(value.as_i64()? as u32).to_le_bytes()),
            _ => unreachable!(),
        }
    }

    pub fn grow_memory(&mut self, memory_index: u32, delta: u64) -> Option<u64> {
        let memory = self.memories.get_mut(memory_index as usize)?;
        let old_pages = (memory.bytes.len() / WASM_PAGE_BYTES) as u64;
        let new_pages = old_pages.checked_add(delta)?;
        if memory.maximum_pages.is_some_and(|maximum| new_pages > maximum) {
            return None;
        }
        let new_length = pages_to_bytes(new_pages).ok()?;
        memory.bytes.try_reserve(new_length.saturating_sub(memory.bytes.len())).ok()?;
        memory.bytes.resize(new_length, 0);
        Some(old_pages)
    }

    pub(super) fn execute_fc(&mut self, decoder: &mut Decoder<'_>, machine: &mut Machine) -> Result<(), CoreError> {
        match decoder.u32()? {
            0 => map_top(machine, |value| Ok(Value::I32(saturating_i32_from_f32(value.as_f32()?, true))))?,
            1 => map_top(machine, |value| Ok(Value::I32(saturating_i32_from_f32(value.as_f32()?, false))))?,
            2 => map_top(machine, |value| Ok(Value::I32(saturating_i32_from_f64(value.as_f64()?, true))))?,
            3 => map_top(machine, |value| Ok(Value::I32(saturating_i32_from_f64(value.as_f64()?, false))))?,
            4 => map_top(machine, |value| Ok(Value::I64(saturating_i64_from_f32(value.as_f32()?, true))))?,
            5 => map_top(machine, |value| Ok(Value::I64(saturating_i64_from_f32(value.as_f32()?, false))))?,
            6 => map_top(machine, |value| Ok(Value::I64(saturating_i64_from_f64(value.as_f64()?, true))))?,
            7 => map_top(machine, |value| Ok(Value::I64(saturating_i64_from_f64(value.as_f64()?, false))))?,
            8 => {
                let data_index = decoder.u32()?;
                let memory_index = decoder.u32()?;
                let length = pop(machine)?.as_i32()? as u32 as usize;
                let source = pop(machine)?.as_i32()? as u32 as usize;
                let destination = pop(machine)?.as_i32()? as u32 as usize;
                let data = self.data.get(data_index as usize).and_then(Option::as_ref).ok_or_else(|| CoreError::Trap("memory.init uses a dropped data segment".into()))?;
                let source_end = checked_end(source, length, data.len(), "memory.init source")?;
                let memory = self.memories.get_mut(memory_index as usize).ok_or_else(|| CoreError::Trap("memory.init memory is out of bounds".into()))?;
                let destination_end = checked_end(destination, length, memory.bytes.len(), "memory.init destination")?;
                memory.bytes[destination..destination_end].copy_from_slice(&data[source..source_end]);
            }
            9 => {
                let data_index = decoder.u32()?;
                *self.data.get_mut(data_index as usize).ok_or_else(|| CoreError::Trap("data.drop index is out of bounds".into()))? = None;
            }
            10 => {
                let destination_memory = decoder.u32()?;
                let source_memory = decoder.u32()?;
                let length = pop(machine)?.as_i32()? as u32 as usize;
                let source = pop(machine)?.as_i32()? as u32 as usize;
                let destination = pop(machine)?.as_i32()? as u32 as usize;
                if destination_memory != source_memory {
                    let source_index = source_memory as usize;
                    let destination_index = destination_memory as usize;
                    let source_bound = self.memories.get(source_index).ok_or_else(|| CoreError::Trap("memory.copy source memory is out of bounds".into()))?.bytes.len();
                    let destination_bound = self.memories.get(destination_index).ok_or_else(|| CoreError::Trap("memory.copy destination memory is out of bounds".into()))?.bytes.len();
                    let source_end = checked_end(source, length, source_bound, "memory.copy source")?;
                    let destination_end = checked_end(destination, length, destination_bound, "memory.copy destination")?;
                    if source_index < destination_index {
                        let (source_memories, destination_memories) = self.memories.split_at_mut(destination_index);
                        destination_memories[0].bytes[destination..destination_end].copy_from_slice(&source_memories[source_index].bytes[source..source_end]);
                    } else {
                        let (destination_memories, source_memories) = self.memories.split_at_mut(source_index);
                        destination_memories[destination_index].bytes[destination..destination_end].copy_from_slice(&source_memories[0].bytes[source..source_end]);
                    }
                } else {
                    let memory = self.memories.get_mut(source_memory as usize).ok_or_else(|| CoreError::Trap("memory.copy memory is out of bounds".into()))?;
                    checked_end(source, length, memory.bytes.len(), "memory.copy source")?;
                    checked_end(destination, length, memory.bytes.len(), "memory.copy destination")?;
                    memory.bytes.copy_within(source..source + length, destination);
                }
            }
            11 => {
                let memory_index = decoder.u32()?;
                let length = pop(machine)?.as_i32()? as u32 as usize;
                let value = pop(machine)?.as_i32()? as u8;
                let destination = pop(machine)?.as_i32()? as u32 as usize;
                let memory = self.memories.get_mut(memory_index as usize).ok_or_else(|| CoreError::Trap("memory.fill memory is out of bounds".into()))?;
                let end = checked_end(destination, length, memory.bytes.len(), "memory.fill")?;
                memory.bytes[destination..end].fill(value);
            }
            12 => {
                let element_index = decoder.u32()?;
                let table_index = decoder.u32()?;
                let length = pop(machine)?.as_i32()? as u32 as usize;
                let source = pop(machine)?.as_i32()? as u32 as usize;
                let destination = pop(machine)?.as_i32()? as u32 as usize;
                let elements = self.elements.get(element_index as usize).and_then(Option::as_ref).ok_or_else(|| CoreError::Trap("table.init uses a dropped element segment".into()))?;
                let source_end = checked_end(source, length, elements.len(), "table.init source")?;
                let table = self.tables.get_mut(table_index as usize).ok_or_else(|| CoreError::Trap("table.init table is out of bounds".into()))?;
                let destination_end = checked_end(destination, length, table.values.len(), "table.init destination")?;
                for (target, value) in table.values[destination..destination_end].iter_mut().zip(&elements[source..source_end]) {
                    *target = value.map(u64::from);
                }
            }
            13 => {
                let element_index = decoder.u32()?;
                *self.elements.get_mut(element_index as usize).ok_or_else(|| CoreError::Trap("elem.drop index is out of bounds".into()))? = None;
            }
            14 => {
                let destination_table = decoder.u32()?;
                let source_table = decoder.u32()?;
                let length = pop(machine)?.as_i32()? as u32 as usize;
                let source = pop(machine)?.as_i32()? as u32 as usize;
                let destination = pop(machine)?.as_i32()? as u32 as usize;
                let source_values = self.tables.get(source_table as usize).ok_or_else(|| CoreError::Trap("table.copy source is out of bounds".into()))?;
                let source_end = checked_end(source, length, source_values.values.len(), "table.copy source")?;
                let copied = source_values.values[source..source_end].to_vec();
                let target = self.tables.get_mut(destination_table as usize).ok_or_else(|| CoreError::Trap("table.copy destination is out of bounds".into()))?;
                let destination_end = checked_end(destination, length, target.values.len(), "table.copy destination")?;
                target.values[destination..destination_end].copy_from_slice(&copied);
            }
            15 => {
                let table_index = decoder.u32()?;
                let delta = pop(machine)?.as_i32()? as u32 as u64;
                let value = pop(machine)?;
                let table = self.tables.get_mut(table_index as usize).ok_or_else(|| CoreError::Trap("table.grow table is out of bounds".into()))?;
                expect_type(value, table.element)?;
                let encoded = reference_bits(value)?;
                let old = table.values.len() as u64;
                let new = old.checked_add(delta);
                let grown = new.filter(|new| table.maximum.is_none_or(|maximum| *new <= maximum)).and_then(|new| usize::try_from(new).ok()).and_then(|new| table.values.try_reserve(new.saturating_sub(table.values.len())).ok().map(|_| new));
                if let Some(new) = grown {
                    table.values.resize(new, encoded);
                    machine.values.push(Value::I32(old as i32));
                } else {
                    machine.values.push(Value::I32(-1));
                }
            }
            16 => {
                let table_index = decoder.u32()?;
                let table = self.tables.get(table_index as usize).ok_or_else(|| CoreError::Trap("table.size table is out of bounds".into()))?;
                machine.values.push(Value::I32(table.values.len() as i32));
            }
            17 => {
                let table_index = decoder.u32()?;
                let length = pop(machine)?.as_i32()? as u32 as usize;
                let value = pop(machine)?;
                let destination = pop(machine)?.as_i32()? as u32 as usize;
                let table = self.tables.get_mut(table_index as usize).ok_or_else(|| CoreError::Trap("table.fill table is out of bounds".into()))?;
                expect_type(value, table.element)?;
                let end = checked_end(destination, length, table.values.len(), "table.fill")?;
                table.values[destination..end].fill(reference_bits(value)?);
            }
            subopcode => return Err(CoreError::Trap(format!("unsupported 0xfc instruction {subopcode}"))),
        }
        Ok(())
    }
}

fn effective_address(base: u64, offset: u64) -> Result<usize, CoreError> {
    usize::try_from(base.checked_add(offset).ok_or_else(|| CoreError::Trap("memory address overflow".into()))?).map_err(|_| CoreError::Trap("memory address exceeds host usize".into()))
}

fn read_u8(memory: &MemoryState, address: usize) -> Result<u8, CoreError> {
    memory.bytes.get(address).copied().ok_or_else(|| CoreError::Trap("out-of-bounds memory read".into()))
}

fn read_u16(memory: &MemoryState, address: usize) -> Result<u16, CoreError> {
    Ok(u16::from_le_bytes(read_array(memory, address)?))
}

fn read_u32(memory: &MemoryState, address: usize) -> Result<u32, CoreError> {
    Ok(u32::from_le_bytes(read_array(memory, address)?))
}

fn read_u64(memory: &MemoryState, address: usize) -> Result<u64, CoreError> {
    Ok(u64::from_le_bytes(read_array(memory, address)?))
}

fn read_array<const N: usize>(memory: &MemoryState, address: usize) -> Result<[u8; N], CoreError> {
    let end = checked_end(address, N, memory.bytes.len(), "memory read")?;
    Ok(memory.bytes[address..end].try_into().expect("checked exact array length"))
}

fn write_bytes(memory: &mut MemoryState, address: usize, bytes: &[u8]) -> Result<(), CoreError> {
    let end = checked_end(address, bytes.len(), memory.bytes.len(), "memory write")?;
    memory.bytes[address..end].copy_from_slice(bytes);
    Ok(())
}

fn reference_bits(value: Value) -> Result<Option<u64>, CoreError> {
    match value {
        Value::FuncRef(value) => Ok(value.map(u64::from)),
        Value::ExternRef(value) => Ok(value),
        _ => Err(CoreError::Trap("value is not a reference".into())),
    }
}
//#endregion 💾️MemoryAndBulkExecution
