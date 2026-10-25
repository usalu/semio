//! 📸️ Checkpoint codec: the borrowed checkpoint output retirement, the checkpoint writer and reader and the machine validation. See [`super`].

use super::*;

//#region 💾️CheckpointCodec
impl CoreInstance {
    /// 👓️ Visits original borrowed checkpoint fields without creating intermediate storage.
    pub fn visit_checkpoint(&self,visitor:&mut dyn FnMut(&[u8])->Result<(),semio_framework_value::ValueError>)->Result<(),semio_framework_value::ValueError>{
        let mut writer=CheckpointWriter{sink:Some(visitor),..Default::default()};self.write_checkpoint(&mut writer);writer.error.map_or(Ok(()),Err)
    }

    pub fn checkpoint(&self) -> Vec<u8> {
        let mut writer = CheckpointWriter::default();
        self.write_checkpoint(&mut writer);
        writer.output
    }

    fn write_checkpoint(&self,writer:&mut CheckpointWriter<'_>){
        writer.bytes(CHECKPOINT_MAGIC);
        writer.byte(CHECKPOINT_VERSION);
        writer.u64(self.module.bytes_fingerprint);
        writer.u64(self.next_host_call);
        writer.list(&self.memories, |writer, memory| writer.sized_bytes(&memory.bytes));
        writer.list(&self.tables, |writer, table| writer.list(&table.values, |writer, value| writer.option(*value, CheckpointWriter::u64)));
        writer.list(&self.globals, |writer, global| writer.value(global.value));
        writer.list(&self.data, |writer, data| writer.option(data.as_ref(), |writer, bytes| writer.sized_bytes(bytes)));
        writer.list(&self.elements, |writer, elements| writer.option(elements.as_ref(), |writer, values| writer.list(values, |writer, value| writer.option(*value, |writer, value| writer.u32(value)))));
        writer.option(self.machine.as_ref(), |writer, machine| writer.machine(machine));
    }

    /// 📤️ Measures borrowed state before original allocation and returns every failure owner to its caller slot.
    pub fn checkpoint_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Vec<u8>,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        control.scoped_stage(|control|{
            control.begin_stage(0)?;let mut count=0usize;
            {let mut sink=|bytes:&[u8]|{count=count.checked_add(bytes.len()).filter(|count|*count<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"owned checkpoint extent exceeds address space"))?;control.advance(bytes.len())};let mut writer=CheckpointWriter{sink:Some(&mut sink),..Default::default()};self.write_checkpoint(&mut writer);if let Some(error)=writer.error{return Err(error);}}
            control.begin_stage(count)?;
            control.with_retirement_owner(size_of::<CheckpointOutputRetirement>(),|control|{
                let mut owner=Box::new(CheckpointOutputRetirement{bytes:std::mem::ManuallyDrop::new(None)});
                let result=(||{owner.bytes.replace(control.allocate_vec(count)?);let output=owner.bytes.as_mut().unwrap();let mut sink=|bytes:&[u8]|{let next=output.len().checked_add(bytes.len()).filter(|next|*next<=count).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"checkpoint changed after measured reservation"))?;control.advance(bytes.len())?;output.extend_from_slice(bytes);debug_assert_eq!(output.len(),next);Ok(())};let mut writer=CheckpointWriter{sink:Some(&mut sink),..Default::default()};self.write_checkpoint(&mut writer);if let Some(error)=writer.error{return Err(error);}if owner.bytes.as_ref().unwrap().len()!=count{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"checkpoint changed after measured reservation"));}Ok(owner.bytes.take().unwrap())})();
                (result,Some(owner as Box<dyn semio_framework_value::ErasedSnapshotRetirement>))
            })
        })
    }

    pub fn restore(module: Arc<CoreModule>, bytes: &[u8]) -> Result<Self, CoreError> {
        let mut reader = CheckpointReader::new(bytes);
        if reader.bytes(CHECKPOINT_MAGIC.len())? != CHECKPOINT_MAGIC || reader.byte()? != CHECKPOINT_VERSION {
            return Err(CoreError::State("checkpoint header/version mismatch".into()));
        }
        if reader.u64()? != module.bytes_fingerprint {
            return Err(CoreError::State("checkpoint belongs to a different module".into()));
        }
        let next_host_call = reader.u64()?;
        let memory_bytes = reader.list(|reader| reader.sized_bytes())?;
        let table_values = reader.list(|reader| reader.list(|reader| reader.option(CheckpointReader::u64)))?;
        let global_values = reader.list(CheckpointReader::value)?;
        let data = reader.list(|reader| reader.option(CheckpointReader::sized_bytes))?;
        let elements = reader.list(|reader| reader.option(|reader| reader.list(|reader| reader.option(CheckpointReader::u32))))?;
        let machine = reader.option(CheckpointReader::machine)?;
        if !reader.done() {
            return Err(CoreError::State("checkpoint has trailing bytes".into()));
        }
        if memory_bytes.len() != module.memories.len() || table_values.len() != module.tables.len() || global_values.len() != module.globals.len() || data.len() != module.data.len() || elements.len() != module.elements.len() {
            return Err(CoreError::State("checkpoint state vector count differs from module".into()));
        }
        let memories = memory_bytes
            .into_iter()
            .zip(&module.memories)
            .map(|(bytes, limits)| {
                if bytes.len() % WASM_PAGE_BYTES != 0 || limits.maximum_pages.is_some_and(|maximum| bytes.len() / WASM_PAGE_BYTES > maximum as usize) {
                    return Err(CoreError::State("checkpoint memory violates module limits".into()));
                }
                Ok(MemoryState { bytes, maximum_pages: limits.maximum_pages, memory64: limits.memory64 })
            })
            .collect::<Result<Vec<_>, CoreError>>()?;
        let tables = table_values
            .into_iter()
            .zip(&module.tables)
            .map(|(values, limits)| {
                if limits.maximum.is_some_and(|maximum| values.len() > maximum as usize) {
                    return Err(CoreError::State("checkpoint table violates module limits".into()));
                }
                Ok(TableState { values, element: limits.element, maximum: limits.maximum })
            })
            .collect::<Result<Vec<_>, CoreError>>()?;
        let mut globals = Vec::with_capacity(global_values.len());
        for (value, declaration) in global_values.into_iter().zip(&module.globals) {
            expect_type(value, declaration.value_type)?;
            globals.push(GlobalState { value, mutable: declaration.mutable });
        }
        validate_machine(&module, machine.as_ref())?;
        Ok(Self { module, memories, tables, globals, data, elements, machine, next_host_call })
    }
}

struct CheckpointOutputRetirement{pub(super) bytes:std::mem::ManuallyDrop<Option<Vec<u8>>>}

impl semio_framework_value::ErasedSnapshotRetirement for CheckpointOutputRetirement{
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let _=maximum_body_bytes;Ok(semio_framework_value::RetirementDemand{copy_bytes:if self.bytes.is_some(){size_of::<Option<Vec<u8>>>()}else{0},release_bytes:self.bytes.as_ref().map_or(0,Vec::capacity),depth:usize::from(self.bytes.is_some()),..Default::default()})}

    fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};let empty=RetainedCloneProgress::default();if self.bytes.is_none(){return Ok(RetainedCloneStep::Complete(empty));}let release=self.bytes.as_ref().unwrap().capacity();let copy=size_of::<Option<Vec<u8>>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release{return Ok(RetainedCloneStep::Progress(empty));}if grant.maximum_depth==0{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"checkpoint owner release requires original depth"));}drop(self.bytes.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..empty}))}
    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if self.bytes.is_some(){size_of::<Option<Vec<u8>>>()}else{0})}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.bytes.as_ref().map_or(0,Vec::capacity))}
    fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(usize::from(self.bytes.is_some()))}
    fn terminal_is_empty(&self)->bool{self.bytes.is_none()}
}

impl Drop for CheckpointOutputRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.bytes.is_none(),"checkpoint original output reached Drop before admitted release");if self.bytes.is_none(){unsafe{std::mem::ManuallyDrop::drop(&mut self.bytes);}}}}

#[derive(Default)]
struct CheckpointWriter<'a> {
    pub(super) output: Vec<u8>,
    sink: Option<&'a mut dyn FnMut(&[u8])->Result<(),semio_framework_value::ValueError>>,
    pub(super) error: Option<semio_framework_value::ValueError>,
}

impl CheckpointWriter<'_> {
    pub(super) fn byte(&mut self, value: u8) {
        self.bytes(&[value]);
    }

    pub(super) fn bytes(&mut self, bytes: &[u8]) {
        if self.error.is_some(){return;}
        if let Some(sink)=self.sink.as_mut(){if let Err(error)=sink(bytes){self.error=Some(error);}}else{self.output.extend_from_slice(bytes);}
    }

    pub(super) fn u32(&mut self, value: u32) {
        self.bytes(&value.to_le_bytes());
    }

    pub(super) fn u64(&mut self, value: u64) {
        self.bytes(&value.to_le_bytes());
    }

    pub(super) fn usize(&mut self, value: usize) {
        self.u64(value as u64);
    }

    fn sized_bytes(&mut self, bytes: &[u8]) {
        self.usize(bytes.len());
        self.bytes(bytes);
    }

    fn list<T>(&mut self, values: &[T], mut write: impl FnMut(&mut Self, &T)) {
        self.usize(values.len());
        for value in values {
            if self.error.is_some(){break;}
            write(self, value);
        }
    }

    fn option<T>(&mut self, value: Option<T>, write: impl FnOnce(&mut Self, T)) {
        match value {
            Some(value) => {
                self.byte(1);
                write(self, value);
            }
            None => self.byte(0),
        }
    }

    pub(super) fn value(&mut self, value: Value) {
        match value {
            Value::I32(value) => {
                self.byte(0);
                self.u32(value as u32);
            }
            Value::I64(value) => {
                self.byte(1);
                self.u64(value as u64);
            }
            Value::F32(value) => {
                self.byte(2);
                self.u32(value);
            }
            Value::F64(value) => {
                self.byte(3);
                self.u64(value);
            }
            Value::FuncRef(value) => {
                self.byte(4);
                self.option(value, Self::u32);
            }
            Value::ExternRef(value) => {
                self.byte(5);
                self.option(value, Self::u64);
            }
        }
    }

    pub(super) fn value_type(&mut self, value: ValueType) {
        self.byte(match value {
            ValueType::I32 => 0,
            ValueType::I64 => 1,
            ValueType::F32 => 2,
            ValueType::F64 => 3,
            ValueType::FuncRef => 4,
            ValueType::ExternRef => 5,
        });
    }

    fn string(&mut self, value: &str) {
        self.sized_bytes(value.as_bytes());
    }

    pub(super) fn machine(&mut self, machine: &Machine) {
        self.list(&machine.values, |writer, value| writer.value(*value));
        self.list(&machine.frames, Self::frame);
        self.option(machine.pending_host.as_ref(), Self::pending_host);
    }

    pub(super) fn frame(&mut self, frame: &Frame) {
        self.u32(frame.function);
        self.usize(frame.pc);
        self.usize(frame.stack_base);
        self.list(&frame.locals, |writer, value| writer.value(*value));
        self.list(&frame.controls, Self::control);
    }

    pub(super) fn control(&mut self, control: &ControlFrame) {
        self.byte(match control.kind {
            ControlKind::Function => 0,
            ControlKind::Block => 1,
            ControlKind::Loop => 2,
            ControlKind::If => 3,
        });
        self.usize(control.start_pc);
        self.usize(control.end_pc);
        self.usize(control.stack_height);
        self.list(&control.branch_types, |writer, value| writer.value_type(*value));
        self.list(&control.result_types, |writer, value| writer.value_type(*value));
    }

    pub(super) fn pending_host(&mut self, pending: &PendingHost) {
        self.u64(pending.call.id);
        self.string(&pending.call.module);
        self.string(&pending.call.name);
        self.list(&pending.call.arguments, |writer, value| writer.value(*value));
        self.list(&pending.call.results, |writer, value| writer.value_type(*value));
        self.usize(pending.stack_height);
    }
}

struct CheckpointReader<'a>(Decoder<'a>);

impl<'a> CheckpointReader<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self(Decoder::new(bytes))
    }

    pub(super) fn done(&self) -> bool {
        self.0.done()
    }

    pub(super) fn byte(&mut self) -> Result<u8, CoreError> {
        self.0.byte().map_err(|error| CoreError::State(error.to_string()))
    }

    pub(super) fn bytes(&mut self, length: usize) -> Result<&'a [u8], CoreError> {
        self.0.bytes(length).map_err(|error| CoreError::State(error.to_string()))
    }

    pub(super) fn u32(&mut self) -> Result<u32, CoreError> {
        Ok(u32::from_le_bytes(self.bytes(4)?.try_into().expect("four bytes")))
    }

    pub(super) fn u64(&mut self) -> Result<u64, CoreError> {
        Ok(u64::from_le_bytes(self.bytes(8)?.try_into().expect("eight bytes")))
    }

    pub(super) fn usize(&mut self) -> Result<usize, CoreError> {
        usize::try_from(self.u64()?).map_err(|_| CoreError::State("checkpoint length exceeds host usize".into()))
    }

    fn sized_bytes(&mut self) -> Result<Vec<u8>, CoreError> {
        let length = self.usize()?;
        Ok(self.bytes(length)?.to_vec())
    }

    fn list<T>(&mut self, mut read: impl FnMut(&mut Self) -> Result<T, CoreError>) -> Result<Vec<T>, CoreError> {
        let count = self.usize()?;
        let mut values = Vec::new();
        values.try_reserve(count).map_err(|_| CoreError::State("checkpoint vector allocation failed".into()))?;
        for _ in 0..count {
            values.push(read(self)?);
        }
        Ok(values)
    }

    fn option<T>(&mut self, read: impl FnOnce(&mut Self) -> Result<T, CoreError>) -> Result<Option<T>, CoreError> {
        match self.byte()? {
            0 => Ok(None),
            1 => read(self).map(Some),
            other => Err(CoreError::State(format!("checkpoint option tag {other} is invalid"))),
        }
    }

    pub(super) fn value(&mut self) -> Result<Value, CoreError> {
        match self.byte()? {
            0 => Ok(Value::I32(self.u32()? as i32)),
            1 => Ok(Value::I64(self.u64()? as i64)),
            2 => Ok(Value::F32(self.u32()?)),
            3 => Ok(Value::F64(self.u64()?)),
            4 => Ok(Value::FuncRef(self.option(Self::u32)?)),
            5 => Ok(Value::ExternRef(self.option(Self::u64)?)),
            tag => Err(CoreError::State(format!("checkpoint value tag {tag} is invalid"))),
        }
    }

    pub(super) fn value_type(&mut self) -> Result<ValueType, CoreError> {
        match self.byte()? {
            0 => Ok(ValueType::I32),
            1 => Ok(ValueType::I64),
            2 => Ok(ValueType::F32),
            3 => Ok(ValueType::F64),
            4 => Ok(ValueType::FuncRef),
            5 => Ok(ValueType::ExternRef),
            tag => Err(CoreError::State(format!("checkpoint value-type tag {tag} is invalid"))),
        }
    }

    fn string(&mut self) -> Result<String, CoreError> {
        String::from_utf8(self.sized_bytes()?).map_err(|_| CoreError::State("checkpoint string is not UTF-8".into()))
    }

    pub(super) fn machine(&mut self) -> Result<Machine, CoreError> {
        Ok(Machine { values: self.list(Self::value)?, frames: self.list(Self::frame)?, pending_host: self.option(Self::pending_host)? })
    }

    pub(super) fn frame(&mut self) -> Result<Frame, CoreError> {
        Ok(Frame { function: self.u32()?, pc: self.usize()?, stack_base: self.usize()?, locals: self.list(Self::value)?, controls: self.list(Self::control)? })
    }

    pub(super) fn control(&mut self) -> Result<ControlFrame, CoreError> {
        let kind = match self.byte()? {
            0 => ControlKind::Function,
            1 => ControlKind::Block,
            2 => ControlKind::Loop,
            3 => ControlKind::If,
            tag => return Err(CoreError::State(format!("checkpoint control tag {tag} is invalid"))),
        };
        Ok(ControlFrame { kind, start_pc: self.usize()?, end_pc: self.usize()?, stack_height: self.usize()?, branch_types: self.list(Self::value_type)?, result_types: self.list(Self::value_type)? })
    }

    pub(super) fn pending_host(&mut self) -> Result<PendingHost, CoreError> {
        let id = self.u64()?;
        let module = self.string()?;
        let name = self.string()?;
        let arguments = self.list(Self::value)?;
        let results = self.list(Self::value_type)?;
        let stack_height = self.usize()?;
        Ok(PendingHost { call: HostCall { id, module, name, arguments, results }, stack_height })
    }
}

fn validate_machine(module: &CoreModule, machine: Option<&Machine>) -> Result<(), CoreError> {
    let Some(machine) = machine else { return Ok(()) };
    if machine.frames.is_empty() {
        return Err(CoreError::State("checkpoint machine has no frames".into()));
    }
    for frame in &machine.frames {
        let declaration = module.functions.get(frame.function as usize).ok_or_else(|| CoreError::State("checkpoint frame function is out of bounds".into()))?;
        let (body, function_type, local_types) = match declaration {
            FunctionDecl::Defined { locals, body, .. } => (body, module.function_type(frame.function)?, locals),
            FunctionDecl::Import { .. } => return Err(CoreError::State("checkpoint frame points at an import".into())),
        };
        if frame.pc > body.len() || frame.stack_base > machine.values.len() || frame.locals.len() != function_type.parameters.len() + local_types.len() {
            return Err(CoreError::State("checkpoint frame shape is invalid".into()));
        }
        for (value, value_type) in frame.locals.iter().zip(function_type.parameters.iter().chain(local_types)) {
            expect_type(*value, *value_type)?;
        }
        if frame.controls.is_empty() || frame.controls.iter().any(|control| control.end_pc >= body.len() || control.stack_height > machine.values.len()) {
            return Err(CoreError::State("checkpoint control frame shape is invalid".into()));
        }
    }
    if machine.pending_host.as_ref().is_some_and(|pending| pending.stack_height > machine.values.len()) {
        return Err(CoreError::State("checkpoint pending host stack height is invalid".into()));
    }
    Ok(())
}
//#endregion 💾️CheckpointCodec
