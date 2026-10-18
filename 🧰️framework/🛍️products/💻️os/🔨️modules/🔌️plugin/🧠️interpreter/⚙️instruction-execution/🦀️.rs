//! ⚙️ Instruction execution: the instruction stepping loop, the branch targets, the stack helpers and the typed stack checks. See [`super`].

use super::*;

//#region ⚙️InstructionExecution
impl CoreInstance {
    pub(super) fn enter_function(&mut self, function: u32, arguments: Vec<Value>) -> Result<(), CoreError> {
        let mut machine = self.machine.take().ok_or_else(|| CoreError::State("no invocation is active".into()))?;
        let result = self.enter_function_on(&mut machine, function, arguments);
        self.machine = Some(machine);
        result.map(|_| ())
    }

    fn enter_function_on(&mut self, machine: &mut Machine, function: u32, arguments: Vec<Value>) -> Result<Option<HostCall>, CoreError> {
        let code = Arc::clone(&self.module);
        let declaration = code.functions.get(function as usize).ok_or_else(|| CoreError::Trap(format!("function {function} is out of bounds")))?;
        let function_type = code.function_type(function)?;
        check_values(&arguments, &function_type.parameters)?;
        match declaration {
            FunctionDecl::Import { module, name, .. } => {
                let call = HostCall { id: self.next_host_call, module: module.clone(), name: name.clone(), arguments, results: function_type.results.clone() };
                self.next_host_call = self.next_host_call.wrapping_add(1).max(1);
                let pending = PendingHost { call: call.clone(), stack_height: machine.values.len() };
                machine.pending_host = Some(pending);
                Ok(Some(call))
            }
            FunctionDecl::Defined { locals, body, .. } => {
                let stack_base = machine.values.len();
                let mut all_locals = arguments;
                all_locals.extend(locals.iter().copied().map(ValueType::zero));
                let end_pc = body.len().checked_sub(1).ok_or_else(|| CoreError::Validation("empty function body".into()))?;
                machine.frames.push(Frame {
                    function,
                    pc: 0,
                    locals: all_locals,
                    stack_base,
                    controls: vec![ControlFrame { kind: ControlKind::Function, start_pc: 0, end_pc, stack_height: stack_base, branch_types: function_type.results.clone(), result_types: function_type.results.clone() }],
                });
                Ok(None)
            }
        }
    }

    pub(super) fn execute_machine(&mut self, code: &CoreModule, machine: &mut Machine) -> Result<InstructionProgress, CoreError> {
        let frame_index = machine.frames.len().checked_sub(1).ok_or_else(|| CoreError::State("active invocation has no call frame".into()))?;
        let function = machine.frames[frame_index].function;
        let (body, controls) = match code.functions.get(function as usize) {
            Some(FunctionDecl::Defined { body, controls, .. }) => (&body[..], &**controls),
            _ => return Err(CoreError::State("call frame points at an import".into())),
        };
        let instruction_pc = machine.frames[frame_index].pc;
        let mut decoder = Decoder::at(body, instruction_pc);
        let opcode = decoder.byte()?;
        match opcode {
            0x00 => return Err(CoreError::Trap("unreachable executed".into())),
            0x01 => {}
            0x02..=0x04 => {
                let block_type = decode_block_type(&mut decoder, &self.module.types)?;
                let condition = if opcode == 0x04 { Some(pop(machine)?.as_i32()? != 0) } else { None };
                let stack_height = machine.values.len().checked_sub(block_type.parameters.len()).ok_or_else(|| CoreError::Trap("block parameters underflow the operand stack".into()))?;
                check_stack_tail(&machine.values, &block_type.parameters)?;
                let bounds = *controls.get(&instruction_pc).ok_or_else(|| CoreError::Validation(format!("missing control bounds at {instruction_pc}")))?;
                let kind = match opcode {
                    0x02 => ControlKind::Block,
                    0x03 => ControlKind::Loop,
                    _ => ControlKind::If,
                };
                let branch_types = if kind == ControlKind::Loop { block_type.parameters.clone() } else { block_type.results.clone() };
                machine.frames[frame_index].controls.push(ControlFrame { kind, start_pc: decoder.position, end_pc: bounds.end_pc, stack_height, branch_types, result_types: block_type.results });
                match condition {
                    Some(false) if bounds.else_pc.is_some() => decoder.position = bounds.else_pc.expect("checked") + 1,
                    Some(false) => {
                        machine.frames[frame_index].controls.pop();
                        decoder.position = bounds.end_pc + 1;
                    }
                    _ => {}
                }
            }
            0x05 => {
                let kind = machine.frames[frame_index].controls.last().map(|control| control.kind).ok_or_else(|| CoreError::Trap("else has no control frame".into()))?;
                if kind != ControlKind::If {
                    return Err(CoreError::Trap("else is not inside an if".into()));
                }
                let control = machine.frames[frame_index].controls.pop().expect("checked");
                keep_results(machine, control.stack_height, &control.result_types)?;
                decoder.position = control.end_pc + 1;
            }
            0x0b => {
                let kind = machine.frames[frame_index].controls.last().map(|control| control.kind).ok_or_else(|| CoreError::Trap("end has no control frame".into()))?;
                if kind == ControlKind::Function {
                    return self.return_frame(machine);
                }
                let control = machine.frames[frame_index].controls.pop().expect("checked");
                keep_results(machine, control.stack_height, &control.result_types)?;
            }
            0x0c => {
                let depth = decoder.u32()?;
                machine.frames[frame_index].pc = decoder.position;
                return self.branch(machine, depth);
            }
            0x0d => {
                let depth = decoder.u32()?;
                if pop(machine)?.as_i32()? != 0 {
                    machine.frames[frame_index].pc = decoder.position;
                    return self.branch(machine, depth);
                }
            }
            0x0e => {
                let count = decoder.u32()?;
                let mut targets = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    targets.push(decoder.u32()?);
                }
                let default = decoder.u32()?;
                let selector = pop(machine)?.as_i32()? as u32;
                machine.frames[frame_index].pc = decoder.position;
                return self.branch(machine, targets.get(selector as usize).copied().unwrap_or(default));
            }
            0x0f => return self.return_frame(machine),
            0x10 => {
                let callee = decoder.u32()?;
                let arguments = pop_arguments(machine, &self.module.function_type(callee)?.parameters)?;
                machine.frames[frame_index].pc = decoder.position;
                if let Some(call) = self.enter_function_on(machine, callee, arguments)? {
                    return Ok(InstructionProgress::Host(call));
                }
                return Ok(InstructionProgress::Continue);
            }
            0x11 => {
                let type_index = decoder.u32()?;
                let table_index = decoder.u32()?;
                let element = pop(machine)?.as_i32()? as u32;
                let table = self.tables.get(table_index as usize).ok_or_else(|| CoreError::Trap(format!("table {table_index} is out of bounds")))?;
                let callee = table.values.get(element as usize).copied().flatten().ok_or_else(|| CoreError::Trap("indirect call targets null or out-of-bounds table entry".into()))? as u32;
                if self.module.functions.get(callee as usize).map(FunctionDecl::type_index) != Some(type_index) {
                    return Err(CoreError::Trap("indirect call type mismatch".into()));
                }
                let function_type = self.module.types.get(type_index as usize).ok_or_else(|| CoreError::Trap(format!("call_indirect type {type_index} is out of bounds")))?.clone();
                let arguments = pop_arguments(machine, &function_type.parameters)?;
                machine.frames[frame_index].pc = decoder.position;
                if let Some(call) = self.enter_function_on(machine, callee, arguments)? {
                    return Ok(InstructionProgress::Host(call));
                }
                return Ok(InstructionProgress::Continue);
            }
            0x1a => {
                pop(machine)?;
            }
            0x1b => {
                let condition = pop(machine)?.as_i32()?;
                let right = pop(machine)?;
                let left = pop(machine)?;
                if left.value_type() != right.value_type() {
                    return Err(CoreError::Trap("select operands have different types".into()));
                }
                machine.values.push(if condition != 0 { left } else { right });
            }
            0x1c => {
                let types = decoder.vector(|decoder| ValueType::parse(decoder.byte()?))?;
                if types.len() != 1 {
                    return Err(CoreError::Validation("typed select must name exactly one result type".into()));
                }
                let condition = pop(machine)?.as_i32()?;
                let right = pop(machine)?;
                let left = pop(machine)?;
                expect_type(left, types[0])?;
                expect_type(right, types[0])?;
                machine.values.push(if condition != 0 { left } else { right });
            }
            0x20 => {
                let local = decoder.u32()?;
                let value = *machine.frames[frame_index].locals.get(local as usize).ok_or_else(|| CoreError::Trap(format!("local {local} is out of bounds")))?;
                machine.values.push(value);
            }
            0x21 => {
                let local = decoder.u32()?;
                let value = pop(machine)?;
                let target = machine.frames[frame_index].locals.get_mut(local as usize).ok_or_else(|| CoreError::Trap(format!("local {local} is out of bounds")))?;
                expect_type(value, target.value_type())?;
                *target = value;
            }
            0x22 => {
                let local = decoder.u32()?;
                let value = *machine.values.last().ok_or_else(|| CoreError::Trap("operand stack underflow".into()))?;
                let target = machine.frames[frame_index].locals.get_mut(local as usize).ok_or_else(|| CoreError::Trap(format!("local {local} is out of bounds")))?;
                expect_type(value, target.value_type())?;
                *target = value;
            }
            0x23 => {
                let global = decoder.u32()?;
                machine.values.push(self.globals.get(global as usize).ok_or_else(|| CoreError::Trap(format!("global {global} is out of bounds")))?.value);
            }
            0x24 => {
                let global = decoder.u32()?;
                let value = pop(machine)?;
                let target = self.globals.get_mut(global as usize).ok_or_else(|| CoreError::Trap(format!("global {global} is out of bounds")))?;
                if !target.mutable {
                    return Err(CoreError::Trap(format!("global {global} is immutable")));
                }
                expect_type(value, target.value.value_type())?;
                target.value = value;
            }
            0x25 => {
                let table = decoder.u32()?;
                let index = pop(machine)?.as_i32()? as u32 as usize;
                let table = self.tables.get(table as usize).ok_or_else(|| CoreError::Trap("table index is out of bounds".into()))?;
                let value = table.values.get(index).copied().ok_or_else(|| CoreError::Trap("table access is out of bounds".into()))?;
                machine.values.push(match table.element {
                    ValueType::FuncRef => Value::FuncRef(value.map(|value| value as u32)),
                    ValueType::ExternRef => Value::ExternRef(value),
                    _ => return Err(CoreError::State("table has non-reference element type".into())),
                });
            }
            0x26 => {
                let table = decoder.u32()?;
                let value = pop(machine)?;
                let index = pop(machine)?.as_i32()? as u32 as usize;
                let table = self.tables.get_mut(table as usize).ok_or_else(|| CoreError::Trap("table index is out of bounds".into()))?;
                expect_type(value, table.element)?;
                let encoded = match value {
                    Value::FuncRef(value) => value.map(u64::from),
                    Value::ExternRef(value) => value,
                    _ => return Err(CoreError::Trap("table.set value is not a reference".into())),
                };
                *table.values.get_mut(index).ok_or_else(|| CoreError::Trap("table access is out of bounds".into()))? = encoded;
            }
            0x28..=0x3e => self.execute_memory(opcode, &mut decoder, machine)?,
            0x3f => {
                let memory = decoder.u32()?;
                let memory = self.memories.get(memory as usize).ok_or_else(|| CoreError::Trap("memory index is out of bounds".into()))?;
                let pages = memory.bytes.len() / WASM_PAGE_BYTES;
                machine.values.push(if memory.memory64 { Value::I64(pages as i64) } else { Value::I32(pages as i32) });
            }
            0x40 => {
                let memory_index = decoder.u32()?;
                let delta = if self.memories.get(memory_index as usize).is_some_and(|memory| memory.memory64) { pop(machine)?.as_i64()? as u64 } else { pop(machine)?.as_i32()? as u32 as u64 };
                let result = self.grow_memory(memory_index, delta);
                let memory64 = self.memories.get(memory_index as usize).is_some_and(|memory| memory.memory64);
                machine.values.push(if memory64 { Value::I64(result.map_or(-1, |pages| pages as i64)) } else { Value::I32(result.map_or(-1, |pages| pages as i32)) });
            }
            0x41 => machine.values.push(Value::I32(decoder.i32()?)),
            0x42 => machine.values.push(Value::I64(decoder.i64()?)),
            0x43 => machine.values.push(Value::F32(u32::from_le_bytes(decoder.bytes(4)?.try_into().expect("four bytes")))),
            0x44 => machine.values.push(Value::F64(u64::from_le_bytes(decoder.bytes(8)?.try_into().expect("eight bytes")))),
            0x45..=0xc4 => execute_numeric(opcode, machine)?,
            0xd0 => {
                let reference = ValueType::parse(decoder.byte()?)?;
                machine.values.push(reference.zero());
            }
            0xd1 => {
                let value = pop(machine)?;
                let is_null = matches!(value, Value::FuncRef(None) | Value::ExternRef(None));
                if !matches!(value, Value::FuncRef(_) | Value::ExternRef(_)) {
                    return Err(CoreError::Trap("ref.is_null operand is not a reference".into()));
                }
                machine.values.push(Value::I32(i32::from(is_null)));
            }
            0xd2 => machine.values.push(Value::FuncRef(Some(decoder.u32()?))),
            0xfc => self.execute_fc(&mut decoder, machine)?,
            0xfd => return Err(CoreError::Trap("SIMD instruction reached without the SIMD guest feature".into())),
            0xfe => return Err(CoreError::Trap("atomic instruction reached in a single-threaded plugin".into())),
            _ => return Err(CoreError::Trap(format!("unsupported opcode 0x{opcode:02x}"))),
        }
        machine.frames[frame_index].pc = decoder.position;
        Ok(InstructionProgress::Continue)
    }

    fn return_frame(&mut self, machine: &mut Machine) -> Result<InstructionProgress, CoreError> {
        let frame = machine.frames.pop().ok_or_else(|| CoreError::Trap("return has no frame".into()))?;
        let module = Arc::clone(&self.module);
        keep_results(machine, frame.stack_base, &module.function_type(frame.function)?.results)?;
        if machine.frames.is_empty() {
            return Ok(InstructionProgress::Complete(machine.values.split_off(frame.stack_base)));
        }
        Ok(InstructionProgress::Continue)
    }

    fn branch(&mut self, machine: &mut Machine, depth: u32) -> Result<InstructionProgress, CoreError> {
        let frame_index = machine.frames.len().checked_sub(1).ok_or_else(|| CoreError::Trap("branch has no frame".into()))?;
        let control_count = machine.frames[frame_index].controls.len();
        let target_index = control_count.checked_sub(depth as usize + 1).ok_or_else(|| CoreError::Trap(format!("branch depth {depth} is out of bounds")))?;
        let control = &mut machine.frames[frame_index].controls[target_index];
        let (kind, stack_height, start_pc, end_pc) = (control.kind, control.stack_height, control.start_pc, control.end_pc);
        let branch_types = std::mem::take(&mut control.branch_types);
        let kept = keep_results(machine, stack_height, &branch_types);
        machine.frames[frame_index].controls[target_index].branch_types = branch_types;
        kept?;
        let target = BranchTargetV1 { kind, start_pc, end_pc };
        match target.kind {
            ControlKind::Loop => {
                machine.frames[frame_index].controls.truncate(target_index + 1);
                machine.frames[frame_index].pc = target.start_pc;
                Ok(InstructionProgress::Continue)
            }
            ControlKind::Function => self.return_frame(machine),
            ControlKind::Block | ControlKind::If => {
                machine.frames[frame_index].controls.truncate(target_index);
                machine.frames[frame_index].pc = target.end_pc + 1;
                Ok(InstructionProgress::Continue)
            }
        }
    }
}

/// 🎯️ What a taken branch needs of its target once the results are in place.
struct BranchTargetV1 {
    pub(super) kind: ControlKind,
    pub(super) start_pc: usize,
    pub(super) end_pc: usize,
}

/// 📥️ Leaves exactly `types` on top of the operand stack at `stack_height`: the same checks, in the same
/// order and with the same traps, as [`take_results`] followed by re-pushing its values — without the
/// intermediate copy, which every block end, branch and return used to allocate.
fn keep_results(machine: &mut Machine, stack_height: usize, types: &[ValueType]) -> Result<(), CoreError> {
    if stack_height > machine.values.len() {
        return Err(CoreError::Trap("control stack height exceeds operand stack".into()));
    }
    let start = machine.values.len().checked_sub(types.len()).ok_or_else(|| CoreError::Trap("control results underflow the operand stack".into()))?;
    if start < stack_height {
        return Err(CoreError::Trap("control results overlap the outer operand stack".into()));
    }
    check_values(&machine.values[start..], types)?;
    machine.values.copy_within(start.., stack_height);
    machine.values.truncate(stack_height + types.len());
    Ok(())
}

pub(super) fn pop(machine: &mut Machine) -> Result<Value, CoreError> {
    machine.values.pop().ok_or_else(|| CoreError::Trap("operand stack underflow".into()))
}

fn pop_arguments(machine: &mut Machine, types: &[ValueType]) -> Result<Vec<Value>, CoreError> {
    let start = machine.values.len().checked_sub(types.len()).ok_or_else(|| CoreError::Trap("call arguments underflow the operand stack".into()))?;
    let arguments = machine.values.split_off(start);
    check_values(&arguments, types)?;
    Ok(arguments)
}

fn take_results(machine: &mut Machine, stack_height: usize, types: &[ValueType]) -> Result<Vec<Value>, CoreError> {
    if stack_height > machine.values.len() {
        return Err(CoreError::Trap("control stack height exceeds operand stack".into()));
    }
    let start = machine.values.len().checked_sub(types.len()).ok_or_else(|| CoreError::Trap("control results underflow the operand stack".into()))?;
    if start < stack_height {
        return Err(CoreError::Trap("control results overlap the outer operand stack".into()));
    }
    let values = machine.values[start..].to_vec();
    check_values(&values, types)?;
    machine.values.truncate(stack_height);
    Ok(values)
}

fn check_stack_tail(values: &[Value], types: &[ValueType]) -> Result<(), CoreError> {
    let start = values.len().checked_sub(types.len()).ok_or_else(|| CoreError::Trap("operand stack does not contain block parameters".into()))?;
    check_values(&values[start..], types)
}

pub(super) fn check_values(values: &[Value], types: &[ValueType]) -> Result<(), CoreError> {
    if values.len() != types.len() {
        return Err(CoreError::Trap(format!("expected {} values, received {}", types.len(), values.len())));
    }
    for (value, expected) in values.iter().zip(types) {
        expect_type(*value, *expected)?;
    }
    Ok(())
}

pub(super) fn expect_type(value: Value, expected: ValueType) -> Result<(), CoreError> {
    if value.value_type() == expected {
        Ok(())
    } else {
        Err(CoreError::Trap(format!("expected {expected:?}, received {:?}", value.value_type())))
    }
}
//#endregion ⚙️InstructionExecution
