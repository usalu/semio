//! 🏛️ Instance state: the core instance, its memory, table and global state, the machine, frames and control frames and the instance construction and call entry. See [`super`].

use super::*;

//#region 🏗️InstanceState
#[derive(Clone, Debug)]
pub struct CoreInstance {
    pub(super) module: Arc<CoreModule>,
    pub(super) memories: Vec<MemoryState>,
    pub(super) tables: Vec<TableState>,
    pub(super) globals: Vec<GlobalState>,
    pub data: Vec<Option<Vec<u8>>>,
    pub(super) elements: Vec<Option<Vec<Option<u32>>>>,
    pub(super) machine: Option<Machine>,
    pub(super) next_host_call: u64,
}

#[derive(Clone, Debug)]
pub(super) struct MemoryState {
    pub(super) bytes: Vec<u8>,
    pub(super) maximum_pages: Option<u64>,
    pub(super) memory64: bool,
}

#[derive(Clone, Debug)]
pub(super) struct TableState {
    pub(super) values: Vec<Option<u64>>,
    pub(super) element: ValueType,
    pub(super) maximum: Option<u64>,
}

#[derive(Clone, Debug)]
pub(super) struct GlobalState {
    pub(super) value: Value,
    pub(super) mutable: bool,
}

#[derive(Clone, Debug)]
pub(super) struct Machine {
    pub(super) values: Vec<Value>,
    pub(super) frames: Vec<Frame>,
    pub(super) pending_host: Option<PendingHost>,
}

#[derive(Clone, Debug)]
pub(super) struct PendingHost {
    pub(super) call: HostCall,
    pub(super) stack_height: usize,
}

#[derive(Clone, Debug)]
pub(super) struct Frame {
    pub(super) function: u32,
    pub(super) pc: usize,
    pub(super) locals: Vec<Value>,
    pub(super) stack_base: usize,
    pub(super) controls: Vec<ControlFrame>,
}

#[derive(Clone, Debug)]
pub(super) struct ControlFrame {
    pub(super) kind: ControlKind,
    pub(super) start_pc: usize,
    pub(super) end_pc: usize,
    pub(super) stack_height: usize,
    pub(super) branch_types: Vec<ValueType>,
    pub(super) result_types: Vec<ValueType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ControlKind {
    Function,
    Block,
    Loop,
    If,
}

impl CoreModule{
    /// ▶️ The actual declared start function, without opening an instance or cloning its body.
    pub fn original_start_function(&self)->Option<u32>{self.start}
}

impl CoreInstance {
    pub fn instantiate(module: Arc<CoreModule>) -> Result<Self, CoreError> {
        let mut instance = Self {
            memories: module
                .memories
                .iter()
                .map(|limits| {
                    let length = pages_to_bytes(limits.minimum_pages)?;
                    Ok(MemoryState { bytes: vec![0; length], maximum_pages: limits.maximum_pages, memory64: limits.memory64 })
                })
                .collect::<Result<Vec<_>, CoreError>>()?,
            tables: module
                .tables
                .iter()
                .map(|limits| {
                    let length = usize::try_from(limits.minimum).map_err(|_| CoreError::Validation("table minimum exceeds host usize".into()))?;
                    Ok(TableState { values: vec![None; length], element: limits.element, maximum: limits.maximum })
                })
                .collect::<Result<Vec<_>, CoreError>>()?,
            globals: Vec::with_capacity(module.globals.len()),
            data: module.data.iter().map(|segment| Some(segment.bytes.clone())).collect(),
            elements: module.elements.iter().map(|segment| Some(segment.values.clone())).collect(),
            module,
            machine: None,
            next_host_call: 1,
        };
        for declaration in &instance.module.globals {
            let value = instance.evaluate_const(&declaration.initializer)?;
            expect_type(value, declaration.value_type)?;
            instance.globals.push(GlobalState { value, mutable: declaration.mutable });
        }
        instance.initialize_segments()?;
        if let Some(start) = instance.module.start {
            instance.begin_function(start, Vec::new())?;
        }
        Ok(instance)
    }

    pub fn begin_export(&mut self, name: &str, arguments: Vec<Value>) -> Result<(), CoreError> {
        if self.machine.is_some() {
            return Err(CoreError::State("an invocation is already active".into()));
        }
        let function = match self.module.exports.get(name) {
            Some(ExportKind::Function(function)) => *function,
            Some(_) => return Err(CoreError::State(format!("export {name} is not a function"))),
            None => return Err(CoreError::State(format!("function export {name} does not exist"))),
        };
        self.begin_function(function, arguments)
    }

    pub fn begin_function(&mut self, function: u32, arguments: Vec<Value>) -> Result<(), CoreError> {
        if self.machine.is_some() {
            return Err(CoreError::State("an invocation is already active".into()));
        }
        self.machine = Some(Machine { values: Vec::new(), frames: Vec::new(), pending_host: None });
        if let Err(error) = self.enter_function(function, arguments) {
            self.machine = None;
            return Err(error);
        }
        Ok(())
    }

    pub fn active(&self) -> bool {
        self.machine.is_some()
    }

    pub fn pending_host_call(&self) -> Option<&HostCall> {
        self.machine.as_ref()?.pending_host.as_ref().map(|pending| &pending.call)
    }

    pub fn resume_host(&mut self, call_id: u64, result: Result<Vec<Value>, String>) -> Result<(), CoreError> {
        let machine = self.machine.as_mut().ok_or_else(|| CoreError::State("no invocation is active".into()))?;
        let pending = machine.pending_host.take().ok_or_else(|| CoreError::State("no host call is pending".into()))?;
        if pending.call.id != call_id {
            machine.pending_host = Some(pending);
            return Err(CoreError::State(format!("host reply {call_id} does not match pending call")));
        }
        let values = result.map_err(CoreError::Host)?;
        check_values(&values, &pending.call.results)?;
        machine.values.truncate(pending.stack_height);
        machine.values.extend(values);
        Ok(())
    }

    pub fn step(&mut self, fuel: u64, control: StepControl) -> CoreStepOutcome {
        let mut used = 0u64;
        if self.machine.is_none() {
            return CoreStepOutcome::Fault { fuel_used: used, error: CoreError::State("no invocation is active".into()) };
        }
        if let Some(call) = self.pending_host_call().cloned() {
            return CoreStepOutcome::HostCall { fuel_used: used, call };
        }
        let Some(mut machine) = self.machine.take() else { return CoreStepOutcome::Fault { fuel_used: used, error: CoreError::State("no invocation is active".into()) } };
        let module = Arc::clone(&self.module);
        while used < fuel {
            if control.cancelled {
                return CoreStepOutcome::Cancelled { fuel_used: used };
            }
            match self.execute_machine(&module, &mut machine) {
                Ok(InstructionProgress::Continue) => used += 1,
                Ok(InstructionProgress::Host(call)) => {
                    used += 1;
                    self.machine = Some(machine);
                    return CoreStepOutcome::HostCall { fuel_used: used, call };
                }
                Ok(InstructionProgress::Complete(values)) => {
                    used += 1;
                    return CoreStepOutcome::Complete { fuel_used: used, values };
                }
                Err(error) => {
                    used += 1;
                    self.machine = Some(machine);
                    let error = self.diagnose_fault(error);
                    self.machine = None;
                    return CoreStepOutcome::Fault { fuel_used: used, error };
                }
            }
        }
        self.machine = Some(machine);
        CoreStepOutcome::Yield { fuel_used: used }
    }

    /// 🧯️ Names a shadow-stack underflow for what it is. A `--stack-first` guest keeps its shadow
    /// stack in `[0, __stack_pointer_init)` and grows it down, so an overrun addresses below zero and
    /// surfaces as an ordinary out-of-bounds access at a huge unsigned address — indistinguishable,
    /// in the bare trap text, from a wild pointer. Reported once, at the only place that still holds
    /// the guest's call depth and globals (ticket 26/09/18 slice A2: a 1 MiB-linked component
    /// overflowing at call depth 26 read as "runaway recursion" for a full slice).
    fn diagnose_fault(&self, error: CoreError) -> CoreError {
        let CoreError::Trap(message) = &error else { return error };
        if !message.contains("is out of bounds: start=4294") {
            return error;
        }
        let Some(Value::I32(pointer)) = self.globals.first().filter(|global| global.mutable).map(|global| global.value) else { return error };
        if pointer >= 0 {
            return error;
        }
        let depth = self.machine.as_ref().map_or(0, |machine| machine.frames.len());
        CoreError::Trap(format!("{message} — the guest's shadow stack underflowed at call depth {depth} (stack pointer {pointer}); link the component with a larger `-zstack-size`"))
    }

    pub fn memory(&self, index: u32) -> Option<&[u8]> {
        self.memories.get(index as usize).map(|memory| memory.bytes.as_slice())
    }

    pub fn memory_mut(&mut self, index: u32) -> Option<&mut [u8]> {
        self.memories.get_mut(index as usize).map(|memory| memory.bytes.as_mut_slice())
    }

    fn evaluate_const(&self, expression: &ConstExpr) -> Result<Value, CoreError> {
        match expression {
            ConstExpr::Value(value) => Ok(*value),
            ConstExpr::Global(index) => self.globals.get(*index as usize).map(|global| global.value).ok_or_else(|| CoreError::Validation(format!("constant global {index} is out of bounds"))),
            ConstExpr::RefFunction(index) => Ok(Value::FuncRef(Some(*index))),
        }
    }

    fn initialize_segments(&mut self) -> Result<(), CoreError> {
        for index in 0..self.module.data.len() {
            let declaration = &self.module.data[index];
            if declaration.mode != SegmentMode::Active {
                continue;
            }
            let offset = self.evaluate_const(declaration.offset.as_ref().ok_or_else(|| CoreError::Validation("active data segment has no offset".into()))?)?.as_i32()? as u32 as usize;
            let memory = self.memories.get_mut(declaration.memory as usize).ok_or_else(|| CoreError::Validation(format!("data memory {} is out of bounds", declaration.memory)))?;
            let end = checked_end(offset, declaration.bytes.len(), memory.bytes.len(), "active data segment")?;
            memory.bytes[offset..end].copy_from_slice(&declaration.bytes);
            self.data[index] = None;
        }
        for index in 0..self.module.elements.len() {
            let declaration = &self.module.elements[index];
            if declaration.mode == SegmentMode::Declarative {
                self.elements[index] = None;
                continue;
            }
            if declaration.mode != SegmentMode::Active {
                continue;
            }
            let offset = self.evaluate_const(declaration.offset.as_ref().ok_or_else(|| CoreError::Validation("active element segment has no offset".into()))?)?.as_i32()? as u32 as usize;
            let table = self.tables.get_mut(declaration.table as usize).ok_or_else(|| CoreError::Validation(format!("element table {} is out of bounds", declaration.table)))?;
            let end = checked_end(offset, declaration.values.len(), table.values.len(), "active element segment")?;
            for (target, source) in table.values[offset..end].iter_mut().zip(&declaration.values) {
                *target = source.map(u64::from);
            }
            self.elements[index] = None;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) enum InstructionProgress {
    Continue,
    Host(HostCall),
    Complete(Vec<Value>),
}
//#endregion 🏗️InstanceState
