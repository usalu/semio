//! 🧠️ Repository-owned WebAssembly core interpreter for the Semio plugin sandbox.
//!
//! The interpreter is deliberately execution-engine neutral: component canonical-ABI lowering is
//! an adapter above this module, while every guest core instruction is decoded, fuelled, cancelled,
//! and checkpointed here. Browser hosts keep using the browser's WebAssembly engine.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

//#region 🧬️CoreTypes

const WASM_PAGE_BYTES: usize = 65_536;
const CHECKPOINT_MAGIC: &[u8; 8] = b"SEMIOWCP";
const CHECKPOINT_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    FuncRef,
    ExternRef,
}

impl ValueType {
    fn parse(byte: u8) -> Result<Self, CoreError> {
        match byte {
            0x7f => Ok(Self::I32),
            0x7e => Ok(Self::I64),
            0x7d => Ok(Self::F32),
            0x7c => Ok(Self::F64),
            0x70 => Ok(Self::FuncRef),
            0x6f => Ok(Self::ExternRef),
            _ => Err(CoreError::Decode(format!("unsupported value type 0x{byte:02x}"))),
        }
    }

    fn zero(self) -> Value {
        match self {
            Self::I32 => Value::I32(0),
            Self::I64 => Value::I64(0),
            Self::F32 => Value::F32(0),
            Self::F64 => Value::F64(0),
            Self::FuncRef => Value::FuncRef(None),
            Self::ExternRef => Value::ExternRef(None),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
    FuncRef(Option<u32>),
    ExternRef(Option<u64>),
}

impl Value {
    pub fn f32(value: f32) -> Self {
        Self::F32(canonical_f32(value).to_bits())
    }

    pub fn f64(value: f64) -> Self {
        Self::F64(canonical_f64(value).to_bits())
    }

    pub fn value_type(self) -> ValueType {
        match self {
            Self::I32(_) => ValueType::I32,
            Self::I64(_) => ValueType::I64,
            Self::F32(_) => ValueType::F32,
            Self::F64(_) => ValueType::F64,
            Self::FuncRef(_) => ValueType::FuncRef,
            Self::ExternRef(_) => ValueType::ExternRef,
        }
    }

    fn as_i32(self) -> Result<i32, CoreError> {
        match self {
            Self::I32(value) => Ok(value),
            _ => Err(CoreError::Trap("expected i32".into())),
        }
    }

    fn as_i64(self) -> Result<i64, CoreError> {
        match self {
            Self::I64(value) => Ok(value),
            _ => Err(CoreError::Trap("expected i64".into())),
        }
    }

    fn as_f32(self) -> Result<f32, CoreError> {
        match self {
            Self::F32(value) => Ok(f32::from_bits(value)),
            _ => Err(CoreError::Trap("expected f32".into())),
        }
    }

    fn as_f64(self) -> Result<f64, CoreError> {
        match self {
            Self::F64(value) => Ok(f64::from_bits(value)),
            _ => Err(CoreError::Trap("expected f64".into())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoreError {
    Decode(String),
    Validation(String),
    Trap(String),
    State(String),
    Host(String),
}

impl Display for CoreError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(message) => write!(formatter, "wasm decode: {message}"),
            Self::Validation(message) => write!(formatter, "wasm validation: {message}"),
            Self::Trap(message) => write!(formatter, "wasm trap: {message}"),
            Self::State(message) => write!(formatter, "wasm state: {message}"),
            Self::Host(message) => write!(formatter, "wasm host: {message}"),
        }
    }
}

impl std::error::Error for CoreError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostCall {
    pub id: u64,
    pub module: String,
    pub name: String,
    pub arguments: Vec<Value>,
    pub results: Vec<ValueType>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoreStepOutcome {
    Yield { fuel_used: u64 },
    HostCall { fuel_used: u64, call: HostCall },
    Complete { fuel_used: u64, values: Vec<Value> },
    Cancelled { fuel_used: u64 },
    Fault { fuel_used: u64, error: CoreError },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StepControl {
    pub cancelled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionType {
    pub parameters: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryLimits {
    pub minimum_pages: u64,
    pub maximum_pages: Option<u64>,
    pub memory64: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableLimits {
    pub element: ValueType,
    pub minimum: u64,
    pub maximum: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExportKind {
    Function(u32),
    Table(u32),
    Memory(u32),
    Global(u32),
}

//#endregion 🧬️CoreTypes

//#region 📦️ModuleModel

#[derive(Clone, Debug)]
pub struct CoreModule {
    bytes_fingerprint: u64,
    types: Vec<FunctionType>,
    functions: Vec<FunctionDecl>,
    tables: Vec<TableLimits>,
    memories: Vec<MemoryLimits>,
    globals: Vec<GlobalDecl>,
    exports: BTreeMap<String, ExportKind>,
    start: Option<u32>,
    elements: Vec<ElementDecl>,
    data: Vec<DataDecl>,
}

#[derive(Clone, Debug)]
enum FunctionDecl {
    Import { module: String, name: String, type_index: u32 },
    Defined { type_index: u32, locals: Vec<ValueType>, body: Arc<[u8]>, controls: Arc<BTreeMap<usize, ControlBounds>> },
}

impl FunctionDecl {
    fn type_index(&self) -> u32 {
        match self {
            Self::Import { type_index, .. } | Self::Defined { type_index, .. } => *type_index,
        }
    }
}

#[derive(Clone, Debug)]
struct GlobalDecl {
    value_type: ValueType,
    mutable: bool,
    initializer: ConstExpr,
}

#[derive(Clone, Debug)]
enum ConstExpr {
    Value(Value),
    Global(u32),
    RefFunction(u32),
}

#[derive(Clone, Debug)]
struct ElementDecl {
    mode: SegmentMode,
    table: u32,
    offset: Option<ConstExpr>,
    values: Vec<Option<u32>>,
}

#[derive(Clone, Debug)]
struct DataDecl {
    mode: SegmentMode,
    memory: u32,
    offset: Option<ConstExpr>,
    bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SegmentMode {
    Active,
    Passive,
    Declarative,
}

#[derive(Clone, Copy, Debug)]
struct ControlBounds {
    else_pc: Option<usize>,
    end_pc: usize,
}

impl CoreModule {
    pub fn parse(bytes: &[u8]) -> Result<Self, CoreError> {
        ModuleParser::parse(bytes)
    }

    pub fn function_type(&self, index: u32) -> Result<&FunctionType, CoreError> {
        let declaration = self.functions.get(index as usize).ok_or_else(|| CoreError::Validation(format!("function index {index} is out of bounds")))?;
        self.types.get(declaration.type_index() as usize).ok_or_else(|| CoreError::Validation(format!("function {index} has an invalid type index")))
    }

    pub fn export(&self, name: &str) -> Option<&ExportKind> {
        self.exports.get(name)
    }

    pub fn imports(&self) -> impl Iterator<Item = (&str, &str, &FunctionType)> {
        self.functions.iter().filter_map(|function| match function {
            FunctionDecl::Import { module, name, type_index } => self.types.get(*type_index as usize).map(|function_type| (module.as_str(), name.as_str(), function_type)),
            FunctionDecl::Defined { .. } => None,
        })
    }

    pub fn exports(&self) -> impl Iterator<Item = (&str, &ExportKind)> {
        self.exports.iter().map(|(name, export)| (name.as_str(), export))
    }
}

//#endregion 📦️ModuleModel

//#region 🧩️ComponentArtifact

const COMPONENT_VERSION: [u8; 4] = [0x0d, 0x00, 0x01, 0x00];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WasmArtifactKind {
    Core,
    Component,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentSectionKind {
    Custom,
    CoreModule,
    CoreInstance,
    CoreType,
    Component,
    Instance,
    Alias,
    Type,
    Canonical,
    Start,
    Import,
    Export,
}

impl ComponentSectionKind {
    fn parse(id: u8) -> Result<Self, CoreError> {
        match id {
            0 => Ok(Self::Custom),
            1 => Ok(Self::CoreModule),
            2 => Ok(Self::CoreInstance),
            3 => Ok(Self::CoreType),
            4 => Ok(Self::Component),
            5 => Ok(Self::Instance),
            6 => Ok(Self::Alias),
            7 => Ok(Self::Type),
            8 => Ok(Self::Canonical),
            9 => Ok(Self::Start),
            10 => Ok(Self::Import),
            11 => Ok(Self::Export),
            _ => Err(CoreError::Decode(format!("unknown component section {id}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentSection {
    pub kind: ComponentSectionKind,
    pub offset: usize,
    pub length: usize,
    pub custom_name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentLimits {
    pub maximum_bytes: usize,
    pub maximum_nesting: u32,
    pub maximum_core_modules: usize,
}

impl Default for ComponentLimits {
    fn default() -> Self {
        Self { maximum_bytes: 512 * 1024 * 1024, maximum_nesting: 32, maximum_core_modules: 1_024 }
    }
}

#[derive(Clone, Debug)]
pub struct ComponentArtifact {
    bytes_fingerprint: u64,
    sections: Vec<ComponentSection>,
    core_modules: Vec<Arc<CoreModule>>,
    nested_components: Vec<Arc<ComponentArtifact>>,
}

impl ComponentArtifact {
    pub fn parse(bytes: &[u8]) -> Result<Self, CoreError> {
        Self::parse_with_limits(bytes, ComponentLimits::default())
    }

    pub fn parse_with_limits(bytes: &[u8], limits: ComponentLimits) -> Result<Self, CoreError> {
        Self::parse_nested(bytes, limits, 0)
    }

    fn parse_nested(bytes: &[u8], limits: ComponentLimits, nesting: u32) -> Result<Self, CoreError> {
        if bytes.len() > limits.maximum_bytes {
            return Err(CoreError::Validation(format!("component size {} exceeds limit {}", bytes.len(), limits.maximum_bytes)));
        }
        if nesting > limits.maximum_nesting {
            return Err(CoreError::Validation(format!("component nesting {nesting} exceeds limit {}", limits.maximum_nesting)));
        }
        let mut decoder = Decoder::new(bytes);
        if decoder.bytes(4)? != b"\0asm" || decoder.bytes(4)? != COMPONENT_VERSION {
            return Err(CoreError::Decode("expected WebAssembly component version 13.1".into()));
        }
        let mut sections = Vec::new();
        let mut core_modules = Vec::new();
        let mut nested_components = Vec::new();
        while !decoder.done() {
            let section_offset = decoder.position;
            let id = decoder.byte()?;
            let kind = ComponentSectionKind::parse(id)?;
            let length = decoder.u32()? as usize;
            let payload_offset = decoder.position;
            let payload = decoder.bytes(length)?;
            let custom_name = if kind == ComponentSectionKind::Custom {
                let mut custom = Decoder::new(payload);
                Some(custom.name()?)
            } else {
                None
            };
            match kind {
                ComponentSectionKind::CoreModule => {
                    if core_modules.len() == limits.maximum_core_modules {
                        return Err(CoreError::Validation(format!("component core module count exceeds limit {}", limits.maximum_core_modules)));
                    }
                    core_modules.push(Arc::new(CoreModule::parse(payload).map_err(|error| CoreError::Decode(format!("core module {}: {error}", core_modules.len())))?));
                }
                ComponentSectionKind::Component => nested_components.push(Arc::new(Self::parse_nested(payload, limits, nesting + 1)?)),
                _ => {}
            }
            sections.push(ComponentSection { kind, offset: section_offset, length: decoder.position - payload_offset, custom_name });
        }
        Ok(Self { bytes_fingerprint: stable_fingerprint(bytes), sections, core_modules, nested_components })
    }

    pub fn fingerprint(&self) -> u64 {
        self.bytes_fingerprint
    }

    pub fn sections(&self) -> &[ComponentSection] {
        &self.sections
    }

    pub fn core_modules(&self) -> &[Arc<CoreModule>] {
        &self.core_modules
    }

    pub fn nested_components(&self) -> &[Arc<ComponentArtifact>] {
        &self.nested_components
    }
}

#[derive(Clone, Debug)]
pub enum WasmArtifact {
    Core(Arc<CoreModule>),
    Component(Arc<ComponentArtifact>),
}

impl WasmArtifact {
    pub fn parse(bytes: &[u8]) -> Result<Self, CoreError> {
        if bytes.get(0..4) != Some(b"\0asm") {
            return Err(CoreError::Decode("expected WebAssembly magic".into()));
        }
        match bytes.get(4..8) {
            Some([1, 0, 0, 0]) => Ok(Self::Core(Arc::new(CoreModule::parse(bytes)?))),
            Some(version) if version == COMPONENT_VERSION => Ok(Self::Component(Arc::new(ComponentArtifact::parse(bytes)?))),
            _ => Err(CoreError::Decode("unsupported WebAssembly encoding version".into())),
        }
    }

    pub fn kind(&self) -> WasmArtifactKind {
        match self {
            Self::Core(_) => WasmArtifactKind::Core,
            Self::Component(_) => WasmArtifactKind::Component,
        }
    }
}

pub fn wasm_execution_binary(bytes: &[u8]) -> Result<Vec<u8>, CoreError> {
    normalize_wasm_binary(bytes, 0)
}

fn normalize_wasm_binary(bytes: &[u8], nesting: u32) -> Result<Vec<u8>, CoreError> {
    if nesting > ComponentLimits::default().maximum_nesting {
        return Err(CoreError::Validation("execution binary component nesting exceeds limit".into()));
    }
    if bytes.get(..4) != Some(b"\0asm") {
        return Err(CoreError::Decode("expected WebAssembly magic".into()));
    }
    let version = bytes.get(4..8).ok_or_else(|| CoreError::Decode("truncated WebAssembly version".into()))?;
    let component = version == COMPONENT_VERSION;
    if !component && version != [1, 0, 0, 0] {
        return Err(CoreError::Decode("unsupported WebAssembly encoding version".into()));
    }
    let mut decoder = Decoder::at(bytes, 8);
    let mut output = bytes[..8].to_vec();
    while !decoder.done() {
        let id = decoder.byte()?;
        let length = decoder.u32()? as usize;
        let payload = decoder.bytes(length)?;
        if id == 0 {
            continue;
        }
        let normalized = if component && id == 1 {
            normalize_wasm_binary(payload, nesting)?
        } else if component && id == 4 {
            normalize_wasm_binary(payload, nesting + 1)?
        } else {
            payload.to_vec()
        };
        output.push(id);
        encode_unsigned_leb(normalized.len() as u64, &mut output);
        output.extend_from_slice(&normalized);
    }
    Ok(output)
}

fn encode_unsigned_leb(mut value: u64, output: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output.push(byte);
        if value == 0 {
            return;
        }
    }
}

//#endregion 🧩️ComponentArtifact

//#region 🔌️SemioActorBoundary

const SEMIO_DESCRIBE_EXPORT: &str = "[async-lift]semio:framework/describe@1.0.0#describe";
const SEMIO_DESCRIBE_RETURN_MODULE: &str = "[export]semio:framework/describe@1.0.0";
const SEMIO_DESCRIBE_RETURN_NAME: &str = "[task-return]describe";
const SEMIO_ACTOR_CHECKPOINT_MAGIC: &[u8; 8] = b"SEMIOACT";
const SEMIO_ACTOR_CHECKPOINT_VERSION: u8 = 1;
const SEMIO_DESCRIBE_CHECKPOINT_MAGIC: &[u8; 8] = b"SEMIODSC";
const SEMIO_DESCRIBE_CHECKPOINT_VERSION: u8 = 1;

pub const SEMIO_OWNED_ALLOC_EXPORT: &str = "semio_owned_alloc_v1";
pub const SEMIO_OWNED_DEALLOC_EXPORT: &str = "semio_owned_dealloc_v1";
pub const SEMIO_OWNED_CHECKPOINT_EXPORT: &str = "semio_owned_checkpoint_v1";
pub const SEMIO_OWNED_RESTORE_EXPORT: &str = "semio_owned_restore_v1";
pub const SEMIO_OWNED_DESCRIBE_EXPORT: &str = "semio_owned_describe_v1";
pub const SEMIO_OWNED_CANCEL_JOB_EXPORT: &str = "semio_owned_cancel_job_v1";
pub const SEMIO_OWNED_START_JOB_EXPORT: &str = "semio_owned_start_job_v1";
pub const SEMIO_OWNED_STEP_JOB_EXPORT: &str = "semio_owned_step_job_v1";
pub const SEMIO_OWNED_POLL_EXPORT: &str = "semio_owned_poll_v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemioActorExport {
    Checkpoint,
    Restore,
    Describe,
    CancelJob,
    StartJob,
    StepJob,
    Poll,
}

impl SemioActorExport {
    pub const ALL: [Self; 7] = [Self::Checkpoint, Self::Restore, Self::Describe, Self::CancelJob, Self::StartJob, Self::StepJob, Self::Poll];

    pub fn core_name(self) -> &'static str {
        match self {
            Self::Checkpoint => "[async-lift]semio:framework/checkpoint@1.0.0#checkpoint",
            Self::Restore => "[async-lift]semio:framework/checkpoint@1.0.0#restore",
            Self::Describe => SEMIO_DESCRIBE_EXPORT,
            Self::CancelJob => "[async-lift]semio:framework/jobs@1.0.0#cancel-job",
            Self::StartJob => "[async-lift]semio:framework/jobs@1.0.0#start-job",
            Self::StepJob => "[async-lift]semio:framework/jobs@1.0.0#step-job",
            Self::Poll => "[async-lift]semio:framework/reactor@1.0.0#poll",
        }
    }

    fn core_type(self) -> FunctionType {
        let parameters = match self {
            Self::Checkpoint | Self::Describe => vec![],
            Self::Restore => vec![ValueType::I32, ValueType::I32],
            Self::CancelJob => vec![ValueType::I64],
            Self::StartJob => vec![ValueType::I64, ValueType::I32, ValueType::I32, ValueType::I32, ValueType::I32],
            Self::StepJob => vec![ValueType::I64, ValueType::I64, ValueType::I32],
            Self::Poll => vec![ValueType::I32, ValueType::I32, ValueType::I64, ValueType::I32, ValueType::I32, ValueType::I32, ValueType::I32],
        };
        FunctionType { parameters, results: vec![ValueType::I32] }
    }
}

//#region 🧠️OwnedSemioBoundary

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OwnedSemioExport {
    Allocate,
    Deallocate,
    Checkpoint,
    Restore,
    Describe,
    CancelJob,
    StartJob,
    StepJob,
    Poll,
}

impl OwnedSemioExport {
    pub const ALL: [Self; 9] = [Self::Allocate, Self::Deallocate, Self::Checkpoint, Self::Restore, Self::Describe, Self::CancelJob, Self::StartJob, Self::StepJob, Self::Poll];

    pub fn core_name(self) -> &'static str {
        match self {
            Self::Allocate => SEMIO_OWNED_ALLOC_EXPORT,
            Self::Deallocate => SEMIO_OWNED_DEALLOC_EXPORT,
            Self::Checkpoint => SEMIO_OWNED_CHECKPOINT_EXPORT,
            Self::Restore => SEMIO_OWNED_RESTORE_EXPORT,
            Self::Describe => SEMIO_OWNED_DESCRIBE_EXPORT,
            Self::CancelJob => SEMIO_OWNED_CANCEL_JOB_EXPORT,
            Self::StartJob => SEMIO_OWNED_START_JOB_EXPORT,
            Self::StepJob => SEMIO_OWNED_STEP_JOB_EXPORT,
            Self::Poll => SEMIO_OWNED_POLL_EXPORT,
        }
    }

    fn core_type(self) -> FunctionType {
        match self {
            Self::Allocate => FunctionType { parameters: vec![ValueType::I32], results: vec![ValueType::I32] },
            Self::Deallocate => FunctionType { parameters: vec![ValueType::I32, ValueType::I32], results: vec![] },
            Self::Checkpoint | Self::Describe => FunctionType { parameters: vec![], results: vec![ValueType::I64] },
            Self::Restore | Self::CancelJob | Self::StartJob | Self::StepJob | Self::Poll => FunctionType { parameters: vec![ValueType::I32, ValueType::I32], results: vec![ValueType::I64] },
        }
    }
}

#[derive(Clone, Debug)]
pub struct OwnedSemioArtifact {
    component: Arc<ComponentArtifact>,
    actor_module: Arc<CoreModule>,
}

impl OwnedSemioArtifact {
    pub fn parse(bytes: &[u8]) -> Result<Self, CoreError> {
        Self::from_component(Arc::new(ComponentArtifact::parse(bytes)?))
    }

    pub fn from_component(component: Arc<ComponentArtifact>) -> Result<Self, CoreError> {
        let mut candidates = component.core_modules.iter().filter(|module| {
            OwnedSemioExport::ALL.iter().all(|export| match module.export(export.core_name()) {
                Some(ExportKind::Function(function)) => module.function_type(*function).is_ok_and(|function_type| function_type == &export.core_type()),
                _ => false,
            })
        });
        let actor_module = Arc::clone(candidates.next().ok_or_else(|| CoreError::Validation("component has no core module implementing the owned Semio actor ABI".into()))?);
        if candidates.next().is_some() {
            return Err(CoreError::Validation("component has multiple core modules implementing the owned Semio actor ABI".into()));
        }
        if !matches!(actor_module.export("memory"), Some(ExportKind::Memory(_))) {
            return Err(CoreError::Validation("owned Semio actor core module does not export memory".into()));
        }
        Ok(Self { component, actor_module })
    }

    pub fn fingerprint(&self) -> u64 {
        self.component.fingerprint()
    }

    pub fn instantiate(&self) -> Result<OwnedSemioInstance, CoreError> {
        Ok(OwnedSemioInstance { component_fingerprint: self.fingerprint(), core: CoreInstance::instantiate(Arc::clone(&self.actor_module))? })
    }

    pub fn restore(&self, checkpoint: &[u8]) -> Result<OwnedSemioInstance, CoreError> {
        if checkpoint.get(..8) != Some(SEMIO_ACTOR_CHECKPOINT_MAGIC) || checkpoint.get(8) != Some(&SEMIO_ACTOR_CHECKPOINT_VERSION) {
            return Err(CoreError::State("invalid owned Semio actor checkpoint header".into()));
        }
        let component_fingerprint = checkpoint.get(9..17).and_then(|bytes| bytes.try_into().ok()).map(u64::from_le_bytes).ok_or_else(|| CoreError::State("truncated owned Semio actor checkpoint fingerprint".into()))?;
        if component_fingerprint != self.fingerprint() {
            return Err(CoreError::State("owned Semio actor checkpoint component fingerprint mismatch".into()));
        }
        let core_length =
            checkpoint.get(17..25).and_then(|bytes| bytes.try_into().ok()).map(u64::from_le_bytes).and_then(|length| usize::try_from(length).ok()).ok_or_else(|| CoreError::State("invalid owned Semio actor core checkpoint length".into()))?;
        let core_checkpoint = checkpoint.get(25..).filter(|bytes| bytes.len() == core_length).ok_or_else(|| CoreError::State("owned Semio actor core checkpoint length mismatch".into()))?;
        Ok(OwnedSemioInstance { component_fingerprint, core: CoreInstance::restore(Arc::clone(&self.actor_module), core_checkpoint)? })
    }
}

#[derive(Clone, Debug)]
pub struct OwnedSemioInstance {
    component_fingerprint: u64,
    core: CoreInstance,
}

impl OwnedSemioInstance {
    pub fn startup_active(&self) -> bool {
        self.core.active()
    }

    pub fn begin(&mut self, export: OwnedSemioExport, arguments: Vec<Value>) -> Result<(), CoreError> {
        self.core.begin_export(export.core_name(), arguments)
    }

    pub fn step(&mut self, fuel: u64, control: StepControl) -> CoreStepOutcome {
        self.core.step(fuel, control)
    }

    pub fn resume_host(&mut self, call_id: u64, result: Result<Vec<Value>, String>) -> Result<(), CoreError> {
        self.core.resume_host(call_id, result)
    }

    pub fn memory(&self) -> Option<&[u8]> {
        let Some(ExportKind::Memory(memory)) = self.core.module.export("memory") else { return None };
        self.core.memory(*memory)
    }

    pub fn memory_mut(&mut self) -> Option<&mut [u8]> {
        let memory = match self.core.module.export("memory") {
            Some(ExportKind::Memory(memory)) => *memory,
            _ => return None,
        };
        self.core.memory_mut(memory)
    }

    pub fn read_bytes_result(&self, values: &[Value], maximum_bytes: usize) -> Result<Vec<u8>, CoreError> {
        let [Value::I64(pair)] = values else { return Err(CoreError::Host("owned Semio export returned an invalid pointer/length pair".into())) };
        let pair = *pair as u64;
        let start = pair as u32 as usize;
        let length = (pair >> 32) as u32 as usize;
        if length > maximum_bytes {
            return Err(CoreError::Host(format!("owned Semio export length {length} exceeds limit {maximum_bytes}")));
        }
        let memory = self.memory().ok_or_else(|| CoreError::State("owned Semio actor memory is unavailable".into()))?;
        let end = checked_end(start, length, memory.len(), "owned Semio export")?;
        Ok(memory[start..end].to_vec())
    }

    pub fn checkpoint(&self) -> Vec<u8> {
        let core = self.core.checkpoint();
        let mut checkpoint = Vec::with_capacity(25 + core.len());
        checkpoint.extend_from_slice(SEMIO_ACTOR_CHECKPOINT_MAGIC);
        checkpoint.push(SEMIO_ACTOR_CHECKPOINT_VERSION);
        checkpoint.extend_from_slice(&self.component_fingerprint.to_le_bytes());
        checkpoint.extend_from_slice(&(core.len() as u64).to_le_bytes());
        checkpoint.extend_from_slice(&core);
        checkpoint
    }
}

//#endregion 🧠️OwnedSemioBoundary

#[derive(Clone, Debug)]
pub struct SemioActorArtifact {
    component: Arc<ComponentArtifact>,
    actor_module: Arc<CoreModule>,
}

impl SemioActorArtifact {
    pub fn parse(bytes: &[u8]) -> Result<Self, CoreError> {
        Self::from_component(Arc::new(ComponentArtifact::parse(bytes)?))
    }

    pub fn from_component(component: Arc<ComponentArtifact>) -> Result<Self, CoreError> {
        let mut candidates = component.core_modules.iter().filter(|module| {
            SemioActorExport::ALL.iter().all(|export| match module.export(export.core_name()) {
                Some(ExportKind::Function(function)) => module.function_type(*function).is_ok_and(|function_type| function_type == &export.core_type()),
                _ => false,
            })
        });
        let actor_module = Arc::clone(candidates.next().ok_or_else(|| CoreError::Validation("component has no core module implementing the Semio actor export ABI".into()))?);
        if candidates.next().is_some() {
            return Err(CoreError::Validation("component has multiple core modules implementing the Semio actor export ABI".into()));
        }
        if !matches!(actor_module.export("memory"), Some(ExportKind::Memory(_))) {
            return Err(CoreError::Validation("Semio actor core module does not export canonical memory".into()));
        }
        if !matches!(actor_module.export("cabi_realloc"), Some(ExportKind::Function(_))) {
            return Err(CoreError::Validation("Semio actor core module does not export cabi_realloc".into()));
        }
        let describe_return = FunctionType { parameters: vec![ValueType::I32, ValueType::I32], results: vec![] };
        if !actor_module.imports().any(|(module, name, function_type)| module == SEMIO_DESCRIBE_RETURN_MODULE && name == SEMIO_DESCRIBE_RETURN_NAME && function_type == &describe_return) {
            return Err(CoreError::Validation("Semio actor core module does not import the describe task-return ABI".into()));
        }
        Ok(Self { component, actor_module })
    }

    pub fn fingerprint(&self) -> u64 {
        self.component.fingerprint()
    }

    pub fn module(&self) -> &Arc<CoreModule> {
        &self.actor_module
    }

    pub fn instantiate(&self) -> Result<SemioActorInstance, CoreError> {
        Ok(SemioActorInstance { component_fingerprint: self.fingerprint(), core: CoreInstance::instantiate(Arc::clone(&self.actor_module))? })
    }

    pub fn restore(&self, checkpoint: &[u8]) -> Result<SemioActorInstance, CoreError> {
        if checkpoint.get(..8) != Some(SEMIO_ACTOR_CHECKPOINT_MAGIC) || checkpoint.get(8) != Some(&SEMIO_ACTOR_CHECKPOINT_VERSION) {
            return Err(CoreError::State("invalid Semio actor checkpoint header".into()));
        }
        let component_fingerprint = checkpoint.get(9..17).and_then(|bytes| bytes.try_into().ok()).map(u64::from_le_bytes).ok_or_else(|| CoreError::State("truncated Semio actor checkpoint fingerprint".into()))?;
        if component_fingerprint != self.fingerprint() {
            return Err(CoreError::State("Semio actor checkpoint component fingerprint mismatch".into()));
        }
        let core_length = checkpoint.get(17..25).and_then(|bytes| bytes.try_into().ok()).map(u64::from_le_bytes).and_then(|length| usize::try_from(length).ok()).ok_or_else(|| CoreError::State("invalid Semio actor core checkpoint length".into()))?;
        let core_checkpoint = checkpoint.get(25..).filter(|bytes| bytes.len() == core_length).ok_or_else(|| CoreError::State("Semio actor core checkpoint length mismatch".into()))?;
        Ok(SemioActorInstance { component_fingerprint, core: CoreInstance::restore(Arc::clone(&self.actor_module), core_checkpoint)? })
    }
}

#[derive(Clone, Debug)]
pub struct SemioActorInstance {
    component_fingerprint: u64,
    core: CoreInstance,
}

impl SemioActorInstance {
    pub fn component_fingerprint(&self) -> u64 {
        self.component_fingerprint
    }

    pub fn startup_active(&self) -> bool {
        self.core.active()
    }

    pub fn begin(&mut self, export: SemioActorExport, arguments: Vec<Value>) -> Result<(), CoreError> {
        self.core.begin_export(export.core_name(), arguments)
    }

    pub fn begin_describe(&mut self) -> Result<(), CoreError> {
        self.begin(SemioActorExport::Describe, Vec::new())
    }

    pub fn step(&mut self, fuel: u64, control: StepControl) -> CoreStepOutcome {
        self.core.step(fuel, control)
    }

    pub fn pending_host_call(&self) -> Option<&HostCall> {
        self.core.pending_host_call()
    }

    pub fn resume_host(&mut self, call_id: u64, result: Result<Vec<Value>, String>) -> Result<(), CoreError> {
        self.core.resume_host(call_id, result)
    }

    pub fn memory(&self) -> Option<&[u8]> {
        let Some(ExportKind::Memory(memory)) = self.core.module.export("memory") else { return None };
        self.core.memory(*memory)
    }

    pub fn memory_mut(&mut self) -> Option<&mut [u8]> {
        let memory = match self.core.module.export("memory") {
            Some(ExportKind::Memory(memory)) => *memory,
            _ => return None,
        };
        self.core.memory_mut(memory)
    }

    pub fn describe_task_result(&self, call: &HostCall, maximum_bytes: usize) -> Result<Option<Vec<u8>>, CoreError> {
        if call.module != SEMIO_DESCRIBE_RETURN_MODULE || call.name != SEMIO_DESCRIBE_RETURN_NAME {
            return Ok(None);
        }
        let [Value::I32(pointer), Value::I32(length)] = call.arguments.as_slice() else {
            return Err(CoreError::Host("describe task-return has an invalid canonical argument shape".into()));
        };
        let start = *pointer as u32 as usize;
        let length = *length as u32 as usize;
        if length > maximum_bytes {
            return Err(CoreError::Host(format!("describe task-return length {length} exceeds limit {maximum_bytes}")));
        }
        let memory = self.memory().ok_or_else(|| CoreError::State("Semio actor canonical memory is unavailable".into()))?;
        let end = checked_end(start, length, memory.len(), "describe task-return")?;
        Ok(Some(memory[start..end].to_vec()))
    }

    pub fn checkpoint(&self) -> Vec<u8> {
        let core = self.core.checkpoint();
        let mut checkpoint = Vec::with_capacity(25 + core.len());
        checkpoint.extend_from_slice(SEMIO_ACTOR_CHECKPOINT_MAGIC);
        checkpoint.push(SEMIO_ACTOR_CHECKPOINT_VERSION);
        checkpoint.extend_from_slice(&self.component_fingerprint.to_le_bytes());
        checkpoint.extend_from_slice(&(core.len() as u64).to_le_bytes());
        checkpoint.extend_from_slice(&core);
        checkpoint
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemioDescribeReply {
    pub results: Vec<Value>,
    pub descriptor: Option<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct SemioDescribeHost {
    context: i32,
    next_resource: i32,
    maximum_descriptor_bytes: usize,
}

impl SemioDescribeHost {
    pub fn new(maximum_descriptor_bytes: usize) -> Self {
        Self { context: 0, next_resource: 1, maximum_descriptor_bytes }
    }

    pub fn reply(&mut self, actor: &mut SemioActorInstance, call: &HostCall) -> Result<SemioDescribeReply, CoreError> {
        let descriptor = actor.describe_task_result(call, self.maximum_descriptor_bytes)?;
        let results = match (call.module.as_str(), call.name.as_str()) {
            (SEMIO_DESCRIBE_RETURN_MODULE, SEMIO_DESCRIBE_RETURN_NAME) => vec![],
            ("semio:framework/pure@1.0.0", "now-ms") | ("wasi:clocks/monotonic-clock@0.2.0", "now") => vec![Value::I64(0)],
            ("$root", "[context-get-0]") => vec![Value::I32(self.context)],
            ("$root", "[context-set-0]") => {
                let [Value::I32(context)] = call.arguments.as_slice() else { return Err(CoreError::Host("context-set-0 has an invalid argument shape".into())) };
                self.context = *context;
                vec![]
            }
            ("$root", "[waitable-set-new]")
            | ("wasi:io/streams@0.2.0", "[method]output-stream.subscribe")
            | ("wasi:clocks/monotonic-clock@0.2.0", "subscribe-duration")
            | ("wasi:cli/stdin@0.2.0", "get-stdin")
            | ("wasi:cli/stdout@0.2.0", "get-stdout")
            | ("wasi:cli/stderr@0.2.0", "get-stderr") => vec![Value::I32(self.take_resource())],
            ("$root", "[waitable-set-poll]") => vec![Value::I32(0)],
            ("$root", "[waitable-join]") | ("$root", "[waitable-set-drop]") | ("[export]$root", "[task-cancel]") | ("wasi:io/poll@0.2.0", "[method]pollable.block") => vec![],
            ("wasi:random/insecure-seed@0.2.9", "insecure-seed") => {
                self.write_zeroes(actor, argument_i32(call, 0)?, 16)?;
                vec![]
            }
            ("wasi:cli/environment@0.2.0", "get-environment") => {
                self.write_zeroes(actor, argument_i32(call, 0)?, 8)?;
                vec![]
            }
            ("wasi:clocks/wall-clock@0.2.0", "now") => {
                self.write_zeroes(actor, argument_i32(call, 0)?, 16)?;
                vec![]
            }
            ("wasi:cli/terminal-stdin@0.2.0", "get-terminal-stdin") | ("wasi:cli/terminal-stdout@0.2.0", "get-terminal-stdout") | ("wasi:cli/terminal-stderr@0.2.0", "get-terminal-stderr") => {
                self.write_zeroes(actor, argument_i32(call, 0)?, 8)?;
                vec![]
            }
            ("wasi:io/poll@0.2.0", "poll") => {
                self.write_zeroes(actor, argument_i32(call, 2)?, 8)?;
                vec![]
            }
            ("wasi:io/streams@0.2.0", "[method]output-stream.check-write") => {
                let pointer = argument_i32(call, 1)?;
                self.write_zeroes(actor, pointer, 16)?;
                self.write_memory(actor, pointer.wrapping_add(8), &65_536u64.to_le_bytes())?;
                vec![]
            }
            ("wasi:io/streams@0.2.0", "[method]output-stream.write") => {
                self.write_zeroes(actor, argument_i32(call, 3)?, 16)?;
                vec![]
            }
            ("wasi:io/streams@0.2.0", "[method]output-stream.blocking-flush") => {
                self.write_zeroes(actor, argument_i32(call, 1)?, 16)?;
                vec![]
            }
            ("wasi:io/error@0.2.0", "[resource-drop]error")
            | ("wasi:io/poll@0.2.0", "[resource-drop]pollable")
            | ("wasi:io/streams@0.2.0", "[resource-drop]input-stream")
            | ("wasi:io/streams@0.2.0", "[resource-drop]output-stream")
            | ("wasi:cli/terminal-input@0.2.0", "[resource-drop]terminal-input")
            | ("wasi:cli/terminal-output@0.2.0", "[resource-drop]terminal-output") => vec![],
            _ => return Err(CoreError::Host(format!("import {}::{} is unavailable in the owned pure describe host", call.module, call.name))),
        };
        check_values(&results, &call.results)?;
        Ok(SemioDescribeReply { results, descriptor })
    }

    fn take_resource(&mut self) -> i32 {
        let resource = self.next_resource;
        self.next_resource = self.next_resource.wrapping_add(1).max(1);
        resource
    }

    fn write_zeroes(&self, actor: &mut SemioActorInstance, pointer: i32, length: usize) -> Result<(), CoreError> {
        self.write_memory(actor, pointer, &vec![0; length])
    }

    fn write_memory(&self, actor: &mut SemioActorInstance, pointer: i32, bytes: &[u8]) -> Result<(), CoreError> {
        let start = pointer as u32 as usize;
        let memory = actor.memory_mut().ok_or_else(|| CoreError::State("Semio actor canonical memory is unavailable".into()))?;
        let end = checked_end(start, bytes.len(), memory.len(), "describe host result")?;
        memory[start..end].copy_from_slice(bytes);
        Ok(())
    }
}

fn argument_i32(call: &HostCall, index: usize) -> Result<i32, CoreError> {
    match call.arguments.get(index) {
        Some(Value::I32(value)) => Ok(*value),
        _ => Err(CoreError::Host(format!("{}::{} argument {index} is not i32", call.module, call.name))),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemioDescribeStepOutcome {
    Yield { fuel_used: u64 },
    Complete { fuel_used: u64, descriptor: Vec<u8> },
    Cancelled { fuel_used: u64 },
    Fault { fuel_used: u64, error: CoreError },
}

#[derive(Clone, Debug)]
pub struct SemioDescribeSession {
    actor: SemioActorInstance,
    host: SemioDescribeHost,
    descriptor: Option<Vec<u8>>,
}

impl SemioDescribeSession {
    pub fn start(artifact: &SemioActorArtifact, maximum_descriptor_bytes: usize) -> Result<Self, CoreError> {
        let mut actor = artifact.instantiate()?;
        if actor.startup_active() {
            return Err(CoreError::Validation("Semio actor has a start function that must complete before canonical export invocation".into()));
        }
        actor.begin_describe()?;
        Ok(Self { actor, host: SemioDescribeHost::new(maximum_descriptor_bytes), descriptor: None })
    }

    pub fn restore(artifact: &SemioActorArtifact, checkpoint: &[u8]) -> Result<Self, CoreError> {
        let mut reader = CheckpointReader::new(checkpoint);
        if reader.bytes(8)? != SEMIO_DESCRIBE_CHECKPOINT_MAGIC || reader.byte()? != SEMIO_DESCRIBE_CHECKPOINT_VERSION {
            return Err(CoreError::State("invalid Semio describe checkpoint header".into()));
        }
        let context = reader.u32()? as i32;
        let next_resource = reader.u32()? as i32;
        let maximum_descriptor_bytes = reader.usize()?;
        let descriptor = reader.option(CheckpointReader::sized_bytes)?;
        let actor_checkpoint = reader.sized_bytes()?;
        if !reader.done() {
            return Err(CoreError::State("trailing bytes in Semio describe checkpoint".into()));
        }
        let actor = artifact.restore(&actor_checkpoint)?;
        Ok(Self { actor, host: SemioDescribeHost { context, next_resource, maximum_descriptor_bytes }, descriptor })
    }

    pub fn step(&mut self, fuel: u64, control: StepControl) -> SemioDescribeStepOutcome {
        let mut used = 0;
        while used < fuel {
            match self.actor.step(fuel - used, control) {
                CoreStepOutcome::Yield { fuel_used } => return SemioDescribeStepOutcome::Yield { fuel_used: used + fuel_used },
                CoreStepOutcome::HostCall { fuel_used, call } => {
                    used += fuel_used;
                    let reply = match self.host.reply(&mut self.actor, &call) {
                        Ok(reply) => reply,
                        Err(error) => return SemioDescribeStepOutcome::Fault { fuel_used: used, error },
                    };
                    self.descriptor = reply.descriptor.or_else(|| self.descriptor.take());
                    if let Err(error) = self.actor.resume_host(call.id, Ok(reply.results)) {
                        return SemioDescribeStepOutcome::Fault { fuel_used: used, error };
                    }
                }
                CoreStepOutcome::Complete { fuel_used, .. } => {
                    used += fuel_used;
                    return match self.descriptor.take() {
                        Some(descriptor) => SemioDescribeStepOutcome::Complete { fuel_used: used, descriptor },
                        None => SemioDescribeStepOutcome::Fault { fuel_used: used, error: CoreError::State("Semio describe core export completed without task-return".into()) },
                    };
                }
                CoreStepOutcome::Cancelled { fuel_used } => return SemioDescribeStepOutcome::Cancelled { fuel_used: used + fuel_used },
                CoreStepOutcome::Fault { fuel_used, error } => return SemioDescribeStepOutcome::Fault { fuel_used: used + fuel_used, error },
            }
        }
        SemioDescribeStepOutcome::Yield { fuel_used: used }
    }

    pub fn checkpoint(&self) -> Vec<u8> {
        let actor = self.actor.checkpoint();
        let mut writer = CheckpointWriter::default();
        writer.bytes(SEMIO_DESCRIBE_CHECKPOINT_MAGIC);
        writer.byte(SEMIO_DESCRIBE_CHECKPOINT_VERSION);
        writer.u32(self.host.context as u32);
        writer.u32(self.host.next_resource as u32);
        writer.usize(self.host.maximum_descriptor_bytes);
        writer.option(self.descriptor.as_deref(), CheckpointWriter::sized_bytes);
        writer.sized_bytes(&actor);
        writer.output
    }
}

//#endregion 🔌️SemioActorBoundary

//#region 🔎️BinaryDecoder

#[derive(Clone)]
struct Decoder<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Decoder<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn at(bytes: &'a [u8], position: usize) -> Self {
        Self { bytes, position }
    }

    fn done(&self) -> bool {
        self.position == self.bytes.len()
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    fn byte(&mut self) -> Result<u8, CoreError> {
        let byte = self.bytes.get(self.position).copied().ok_or_else(|| CoreError::Decode("unexpected end of input".into()))?;
        self.position += 1;
        Ok(byte)
    }

    fn bytes(&mut self, length: usize) -> Result<&'a [u8], CoreError> {
        let end = self.position.checked_add(length).ok_or_else(|| CoreError::Decode("byte length overflow".into()))?;
        let result = self.bytes.get(self.position..end).ok_or_else(|| CoreError::Decode("byte range exceeds input".into()))?;
        self.position = end;
        Ok(result)
    }

    fn u32(&mut self) -> Result<u32, CoreError> {
        let value = self.u64()?;
        u32::try_from(value).map_err(|_| CoreError::Decode("unsigned LEB128 exceeds u32".into()))
    }

    fn u64(&mut self) -> Result<u64, CoreError> {
        let mut value = 0u64;
        let mut shift = 0u32;
        loop {
            let byte = self.byte()?;
            let payload = u64::from(byte & 0x7f);
            if shift >= 64 && payload != 0 {
                return Err(CoreError::Decode("unsigned LEB128 overflow".into()));
            }
            value |= payload.checked_shl(shift).unwrap_or(0);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
            shift += 7;
            if shift > 70 {
                return Err(CoreError::Decode("unsigned LEB128 is too long".into()));
            }
        }
    }

    fn i32(&mut self) -> Result<i32, CoreError> {
        Ok(self.signed(32)? as i32)
    }

    fn i64(&mut self) -> Result<i64, CoreError> {
        self.signed(64)
    }

    fn signed(&mut self, bits: u32) -> Result<i64, CoreError> {
        let mut value = 0i64;
        let mut shift = 0u32;
        let mut byte;
        loop {
            byte = self.byte()?;
            value |= i64::from(byte & 0x7f).checked_shl(shift).unwrap_or(0);
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
            if shift > 70 {
                return Err(CoreError::Decode("signed LEB128 is too long".into()));
            }
        }
        if shift < bits && byte & 0x40 != 0 {
            value |= (!0i64) << shift;
        }
        Ok(value)
    }

    fn name(&mut self) -> Result<String, CoreError> {
        let length = self.u32()? as usize;
        String::from_utf8(self.bytes(length)?.to_vec()).map_err(|_| CoreError::Decode("name is not UTF-8".into()))
    }

    fn vector<T>(&mut self, mut decode: impl FnMut(&mut Decoder<'a>) -> Result<T, CoreError>) -> Result<Vec<T>, CoreError> {
        let count = self.u32()? as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(decode(self)?);
        }
        Ok(values)
    }

    fn section(&mut self) -> Result<(u8, Decoder<'a>), CoreError> {
        let id = self.byte()?;
        let length = self.u32()? as usize;
        Ok((id, Decoder::new(self.bytes(length)?)))
    }
}

//#endregion 🔎️BinaryDecoder

//#region 🧱️ModuleParser

type FunctionBody = (Vec<ValueType>, Vec<u8>, BTreeMap<usize, ControlBounds>);

struct ModuleParser {
    types: Vec<FunctionType>,
    imported_functions: Vec<FunctionDecl>,
    defined_types: Vec<u32>,
    code: Vec<FunctionBody>,
    tables: Vec<TableLimits>,
    memories: Vec<MemoryLimits>,
    globals: Vec<GlobalDecl>,
    exports: BTreeMap<String, ExportKind>,
    start: Option<u32>,
    elements: Vec<ElementDecl>,
    data: Vec<DataDecl>,
    data_count: Option<usize>,
    seen: BTreeSet<u8>,
    last_non_custom_section: u8,
}

impl ModuleParser {
    fn parse(bytes: &[u8]) -> Result<CoreModule, CoreError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.bytes(4)? != b"\0asm" || decoder.bytes(4)? != [1, 0, 0, 0] {
            return Err(CoreError::Decode("expected WebAssembly core module version 1".into()));
        }
        let mut parser = Self {
            types: Vec::new(),
            imported_functions: Vec::new(),
            defined_types: Vec::new(),
            code: Vec::new(),
            tables: Vec::new(),
            memories: Vec::new(),
            globals: Vec::new(),
            exports: BTreeMap::new(),
            start: None,
            elements: Vec::new(),
            data: Vec::new(),
            data_count: None,
            seen: BTreeSet::new(),
            last_non_custom_section: 0,
        };
        while !decoder.done() {
            let (id, mut section) = decoder.section()?;
            if id != 0 {
                if !parser.seen.insert(id) {
                    return Err(CoreError::Validation(format!("duplicate section {id}")));
                }
                let order = core_section_order(id)?;
                if order < parser.last_non_custom_section {
                    return Err(CoreError::Validation(format!("section {id} is out of order")));
                }
                parser.last_non_custom_section = order;
            }
            parser.parse_section(id, &mut section)?;
            if !section.done() {
                return Err(CoreError::Decode(format!("section {id} has {} trailing bytes", section.remaining())));
            }
        }
        parser.finish(bytes)
    }

    fn parse_section(&mut self, id: u8, section: &mut Decoder<'_>) -> Result<(), CoreError> {
        match id {
            0 => {
                let _ = section.name()?;
                let remaining = section.remaining();
                section.bytes(remaining)?;
            }
            1 => self.types = section.vector(parse_function_type)?,
            2 => self.parse_imports(section)?,
            3 => self.defined_types = section.vector(|decoder| decoder.u32())?,
            4 => self.tables.extend(section.vector(parse_table_type)?),
            5 => self.memories.extend(section.vector(parse_memory_type)?),
            6 => self.globals.extend(section.vector(parse_global)?),
            7 => self.parse_exports(section)?,
            8 => self.start = Some(section.u32()?),
            9 => self.elements = section.vector(parse_element)?,
            10 => self.code = section.vector(parse_code)?,
            11 => self.data = section.vector(parse_data)?,
            12 => {
                let declared = section.u32()? as usize;
                self.data_count = Some(declared);
            }
            13 => return Err(CoreError::Validation("exception tags are outside the owned Semio ABI subset".into())),
            _ => return Err(CoreError::Decode(format!("unknown core section {id}"))),
        }
        Ok(())
    }

    fn parse_imports(&mut self, section: &mut Decoder<'_>) -> Result<(), CoreError> {
        let count = section.u32()?;
        for _ in 0..count {
            let module = section.name()?;
            let name = section.name()?;
            match section.byte()? {
                0x00 => self.imported_functions.push(FunctionDecl::Import { module, name, type_index: section.u32()? }),
                0x01 => self.tables.push(parse_table_type(section)?),
                0x02 => self.memories.push(parse_memory_type(section)?),
                0x03 => {
                    let value_type = ValueType::parse(section.byte()?)?;
                    let mutable = match section.byte()? {
                        0 => false,
                        1 => true,
                        other => return Err(CoreError::Decode(format!("invalid global mutability {other}"))),
                    };
                    self.globals.push(GlobalDecl { value_type, mutable, initializer: ConstExpr::Value(value_type.zero()) });
                }
                0x04 => return Err(CoreError::Validation(format!("tag import {module}.{name} is unsupported"))),
                kind => return Err(CoreError::Decode(format!("unknown import kind {kind}"))),
            }
        }
        Ok(())
    }

    fn parse_exports(&mut self, section: &mut Decoder<'_>) -> Result<(), CoreError> {
        let count = section.u32()?;
        for _ in 0..count {
            let name = section.name()?;
            let kind = match section.byte()? {
                0x00 => ExportKind::Function(section.u32()?),
                0x01 => ExportKind::Table(section.u32()?),
                0x02 => ExportKind::Memory(section.u32()?),
                0x03 => ExportKind::Global(section.u32()?),
                other => return Err(CoreError::Decode(format!("unsupported export kind {other}"))),
            };
            if self.exports.insert(name.clone(), kind).is_some() {
                return Err(CoreError::Validation(format!("duplicate export {name}")));
            }
        }
        Ok(())
    }

    fn finish(self, bytes: &[u8]) -> Result<CoreModule, CoreError> {
        if self.defined_types.len() != self.code.len() {
            return Err(CoreError::Validation(format!("function section declares {} bodies but code has {}", self.defined_types.len(), self.code.len())));
        }
        if self.data_count.is_some_and(|declared| declared != self.data.len()) {
            return Err(CoreError::Validation("data-count section disagrees with data section".into()));
        }
        let mut functions = self.imported_functions;
        for (type_index, (locals, body, controls)) in self.defined_types.into_iter().zip(self.code) {
            if type_index as usize >= self.types.len() {
                return Err(CoreError::Validation(format!("function type index {type_index} is out of bounds")));
            }
            functions.push(FunctionDecl::Defined { type_index, locals, body: body.into(), controls: Arc::new(controls) });
        }
        for export in self.exports.values() {
            match *export {
                ExportKind::Function(index) if index as usize >= functions.len() => return Err(CoreError::Validation(format!("exported function {index} is out of bounds"))),
                ExportKind::Table(index) if index as usize >= self.tables.len() => return Err(CoreError::Validation(format!("exported table {index} is out of bounds"))),
                ExportKind::Memory(index) if index as usize >= self.memories.len() => return Err(CoreError::Validation(format!("exported memory {index} is out of bounds"))),
                ExportKind::Global(index) if index as usize >= self.globals.len() => return Err(CoreError::Validation(format!("exported global {index} is out of bounds"))),
                _ => {}
            }
        }
        Ok(CoreModule {
            bytes_fingerprint: stable_fingerprint(bytes),
            types: self.types,
            functions,
            tables: self.tables,
            memories: self.memories,
            globals: self.globals,
            exports: self.exports,
            start: self.start,
            elements: self.elements,
            data: self.data,
        })
    }
}

fn core_section_order(id: u8) -> Result<u8, CoreError> {
    match id {
        1..=9 => Ok(id),
        12 => Ok(10),
        10 => Ok(11),
        11 => Ok(12),
        13 => Ok(13),
        _ => Err(CoreError::Decode(format!("unknown core section {id}"))),
    }
}

fn parse_function_type(decoder: &mut Decoder<'_>) -> Result<FunctionType, CoreError> {
    if decoder.byte()? != 0x60 {
        return Err(CoreError::Decode("expected function type".into()));
    }
    Ok(FunctionType { parameters: decoder.vector(|decoder| ValueType::parse(decoder.byte()?))?, results: decoder.vector(|decoder| ValueType::parse(decoder.byte()?))? })
}

fn parse_limits(decoder: &mut Decoder<'_>) -> Result<(u64, Option<u64>, bool, bool), CoreError> {
    let flags = decoder.u32()?;
    if flags & !0x07 != 0 || flags & 0x04 != 0 && flags & 0x02 == 0 {
        return Err(CoreError::Decode(format!("invalid limits flags {flags}")));
    }
    let memory64 = flags & 0x04 != 0;
    let minimum = if memory64 { decoder.u64()? } else { u64::from(decoder.u32()?) };
    let maximum = if flags & 0x01 != 0 { Some(if memory64 { decoder.u64()? } else { u64::from(decoder.u32()?) }) } else { None };
    if maximum.is_some_and(|maximum| maximum < minimum) {
        return Err(CoreError::Validation("limits maximum is below minimum".into()));
    }
    Ok((minimum, maximum, flags & 0x02 != 0, memory64))
}

fn parse_memory_type(decoder: &mut Decoder<'_>) -> Result<MemoryLimits, CoreError> {
    let (minimum_pages, maximum_pages, shared, memory64) = parse_limits(decoder)?;
    if shared {
        return Err(CoreError::Validation("shared memory is outside the single-threaded Semio plugin sandbox".into()));
    }
    Ok(MemoryLimits { minimum_pages, maximum_pages, memory64 })
}

fn parse_table_type(decoder: &mut Decoder<'_>) -> Result<TableLimits, CoreError> {
    let element = ValueType::parse(decoder.byte()?)?;
    if !matches!(element, ValueType::FuncRef | ValueType::ExternRef) {
        return Err(CoreError::Validation("table element is not a reference type".into()));
    }
    let (minimum, maximum, shared, memory64) = parse_limits(decoder)?;
    if shared || memory64 {
        return Err(CoreError::Validation("table limits cannot be shared or memory64".into()));
    }
    Ok(TableLimits { element, minimum, maximum })
}

fn parse_global(decoder: &mut Decoder<'_>) -> Result<GlobalDecl, CoreError> {
    let value_type = ValueType::parse(decoder.byte()?)?;
    let mutable = match decoder.byte()? {
        0 => false,
        1 => true,
        other => return Err(CoreError::Decode(format!("invalid global mutability {other}"))),
    };
    Ok(GlobalDecl { value_type, mutable, initializer: parse_const_expr(decoder)? })
}

fn parse_const_expr(decoder: &mut Decoder<'_>) -> Result<ConstExpr, CoreError> {
    let expression = match decoder.byte()? {
        0x23 => ConstExpr::Global(decoder.u32()?),
        0x41 => ConstExpr::Value(Value::I32(decoder.i32()?)),
        0x42 => ConstExpr::Value(Value::I64(decoder.i64()?)),
        0x43 => ConstExpr::Value(Value::F32(u32::from_le_bytes(decoder.bytes(4)?.try_into().expect("four bytes")))),
        0x44 => ConstExpr::Value(Value::F64(u64::from_le_bytes(decoder.bytes(8)?.try_into().expect("eight bytes")))),
        0xd0 => {
            let reference = ValueType::parse(decoder.byte()?)?;
            ConstExpr::Value(reference.zero())
        }
        0xd2 => ConstExpr::RefFunction(decoder.u32()?),
        opcode => return Err(CoreError::Decode(format!("unsupported constant opcode 0x{opcode:02x}"))),
    };
    if decoder.byte()? != 0x0b {
        return Err(CoreError::Decode("constant expression has no end".into()));
    }
    Ok(expression)
}

fn parse_code(decoder: &mut Decoder<'_>) -> Result<FunctionBody, CoreError> {
    let size = decoder.u32()? as usize;
    let mut body = Decoder::new(decoder.bytes(size)?);
    let groups = body.u32()?;
    let mut locals = Vec::new();
    for _ in 0..groups {
        let count = body.u32()? as usize;
        let value_type = ValueType::parse(body.byte()?)?;
        let new_length = locals.len().checked_add(count).ok_or_else(|| CoreError::Validation("local count overflow".into()))?;
        locals.resize(new_length, value_type);
    }
    let code = body.bytes(body.remaining())?.to_vec();
    let controls = analyze_controls(&code)?;
    Ok((locals, code, controls))
}

fn parse_element(decoder: &mut Decoder<'_>) -> Result<ElementDecl, CoreError> {
    let flags = decoder.u32()?;
    let mode = if flags & 0x01 == 0 {
        SegmentMode::Active
    } else if flags & 0x03 == 0x03 {
        SegmentMode::Declarative
    } else {
        SegmentMode::Passive
    };
    let table = if mode == SegmentMode::Active && flags & 0x02 != 0 { decoder.u32()? } else { 0 };
    let offset = if mode == SegmentMode::Active { Some(parse_const_expr(decoder)?) } else { None };
    let expressions = flags & 0x04 != 0;
    if flags & 0x03 != 0 {
        let element_type = decoder.byte()?;
        if element_type != 0x00 && element_type != 0x70 {
            return Err(CoreError::Decode(format!("unsupported element type 0x{element_type:02x}")));
        }
    }
    let values = if expressions {
        decoder.vector(|decoder| match parse_const_expr(decoder)? {
            ConstExpr::RefFunction(index) => Ok(Some(index)),
            ConstExpr::Value(Value::FuncRef(None)) => Ok(None),
            _ => Err(CoreError::Validation("element expression is not funcref".into())),
        })?
    } else {
        decoder.vector(|decoder| decoder.u32().map(Some))?
    };
    Ok(ElementDecl { mode, table, offset, values })
}

fn parse_data(decoder: &mut Decoder<'_>) -> Result<DataDecl, CoreError> {
    let flags = decoder.u32()?;
    let mode = match flags {
        0 | 2 => SegmentMode::Active,
        1 => SegmentMode::Passive,
        _ => return Err(CoreError::Decode(format!("unsupported data segment flags {flags}"))),
    };
    let memory = if flags == 2 { decoder.u32()? } else { 0 };
    let offset = if mode == SegmentMode::Active { Some(parse_const_expr(decoder)?) } else { None };
    let length = decoder.u32()? as usize;
    Ok(DataDecl { mode, memory, offset, bytes: decoder.bytes(length)?.to_vec() })
}

//#endregion 🧱️ModuleParser

//#region 🧭️InstructionShape

fn analyze_controls(code: &[u8]) -> Result<BTreeMap<usize, ControlBounds>, CoreError> {
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

fn decode_block_type(decoder: &mut Decoder<'_>, types: &[FunctionType]) -> Result<FunctionType, CoreError> {
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

//#region 🏗️InstanceState

#[derive(Clone, Debug)]
pub struct CoreInstance {
    module: Arc<CoreModule>,
    memories: Vec<MemoryState>,
    tables: Vec<TableState>,
    globals: Vec<GlobalState>,
    data: Vec<Option<Vec<u8>>>,
    elements: Vec<Option<Vec<Option<u32>>>>,
    machine: Option<Machine>,
    next_host_call: u64,
}

#[derive(Clone, Debug)]
struct MemoryState {
    bytes: Vec<u8>,
    maximum_pages: Option<u64>,
    memory64: bool,
}

#[derive(Clone, Debug)]
struct TableState {
    values: Vec<Option<u64>>,
    element: ValueType,
    maximum: Option<u64>,
}

#[derive(Clone, Debug)]
struct GlobalState {
    value: Value,
    mutable: bool,
}

#[derive(Clone, Debug)]
struct Machine {
    values: Vec<Value>,
    frames: Vec<Frame>,
    pending_host: Option<PendingHost>,
}

#[derive(Clone, Debug)]
struct PendingHost {
    call: HostCall,
    stack_height: usize,
}

#[derive(Clone, Debug)]
struct Frame {
    function: u32,
    pc: usize,
    locals: Vec<Value>,
    stack_base: usize,
    controls: Vec<ControlFrame>,
}

#[derive(Clone, Debug)]
struct ControlFrame {
    kind: ControlKind,
    start_pc: usize,
    end_pc: usize,
    stack_height: usize,
    branch_types: Vec<ValueType>,
    result_types: Vec<ValueType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ControlKind {
    Function,
    Block,
    Loop,
    If,
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
        while used < fuel {
            if control.cancelled {
                self.machine = None;
                return CoreStepOutcome::Cancelled { fuel_used: used };
            }
            match self.execute_instruction() {
                Ok(InstructionProgress::Continue) => used += 1,
                Ok(InstructionProgress::Host(call)) => {
                    used += 1;
                    return CoreStepOutcome::HostCall { fuel_used: used, call };
                }
                Ok(InstructionProgress::Complete(values)) => {
                    used += 1;
                    self.machine = None;
                    return CoreStepOutcome::Complete { fuel_used: used, values };
                }
                Err(error) => {
                    used += 1;
                    self.machine = None;
                    return CoreStepOutcome::Fault { fuel_used: used, error };
                }
            }
        }
        CoreStepOutcome::Yield { fuel_used: used }
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
enum InstructionProgress {
    Continue,
    Host(HostCall),
    Complete(Vec<Value>),
}

//#endregion 🏗️InstanceState

//#region ⚙️InstructionExecution

impl CoreInstance {
    fn enter_function(&mut self, function: u32, arguments: Vec<Value>) -> Result<(), CoreError> {
        let mut machine = self.machine.take().ok_or_else(|| CoreError::State("no invocation is active".into()))?;
        let result = self.enter_function_on(&mut machine, function, arguments);
        self.machine = Some(machine);
        result.map(|_| ())
    }

    fn enter_function_on(&mut self, machine: &mut Machine, function: u32, arguments: Vec<Value>) -> Result<Option<HostCall>, CoreError> {
        let declaration = self.module.functions.get(function as usize).ok_or_else(|| CoreError::Trap(format!("function {function} is out of bounds")))?.clone();
        let function_type = self.module.function_type(function)?.clone();
        check_values(&arguments, &function_type.parameters)?;
        match declaration {
            FunctionDecl::Import { module, name, .. } => {
                let call = HostCall { id: self.next_host_call, module, name, arguments, results: function_type.results };
                self.next_host_call = self.next_host_call.wrapping_add(1).max(1);
                let pending = PendingHost { call: call.clone(), stack_height: machine.values.len() };
                machine.pending_host = Some(pending);
                Ok(Some(call))
            }
            FunctionDecl::Defined { locals, body, .. } => {
                let stack_base = machine.values.len();
                let mut all_locals = arguments;
                all_locals.extend(locals.into_iter().map(ValueType::zero));
                let end_pc = body.len().checked_sub(1).ok_or_else(|| CoreError::Validation("empty function body".into()))?;
                machine.frames.push(Frame {
                    function,
                    pc: 0,
                    locals: all_locals,
                    stack_base,
                    controls: vec![ControlFrame { kind: ControlKind::Function, start_pc: 0, end_pc, stack_height: stack_base, branch_types: function_type.results.clone(), result_types: function_type.results }],
                });
                Ok(None)
            }
        }
    }

    fn execute_instruction(&mut self) -> Result<InstructionProgress, CoreError> {
        let mut machine = self.machine.take().ok_or_else(|| CoreError::State("no invocation is active".into()))?;
        let result = self.execute_machine(&mut machine);
        if !matches!(result, Ok(InstructionProgress::Complete(_))) {
            self.machine = Some(machine);
        }
        result
    }

    fn execute_machine(&mut self, machine: &mut Machine) -> Result<InstructionProgress, CoreError> {
        let frame_index = machine.frames.len().checked_sub(1).ok_or_else(|| CoreError::State("active invocation has no call frame".into()))?;
        let function = machine.frames[frame_index].function;
        let (body, controls) = match self.module.functions.get(function as usize) {
            Some(FunctionDecl::Defined { body, controls, .. }) => (body.clone(), controls.clone()),
            _ => return Err(CoreError::State("call frame points at an import".into())),
        };
        let instruction_pc = machine.frames[frame_index].pc;
        let mut decoder = Decoder::at(&body, instruction_pc);
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
                let control = machine.frames[frame_index].controls.last().cloned().ok_or_else(|| CoreError::Trap("else has no control frame".into()))?;
                if control.kind != ControlKind::If {
                    return Err(CoreError::Trap("else is not inside an if".into()));
                }
                close_control(machine, &control)?;
                machine.frames[frame_index].controls.pop();
                decoder.position = control.end_pc + 1;
            }
            0x0b => {
                let control = machine.frames[frame_index].controls.last().cloned().ok_or_else(|| CoreError::Trap("end has no control frame".into()))?;
                if control.kind == ControlKind::Function {
                    return self.return_frame(machine);
                }
                close_control(machine, &control)?;
                machine.frames[frame_index].controls.pop();
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
        let results = self.module.function_type(frame.function)?.results.clone();
        let values = take_results(machine, frame.stack_base, &results)?;
        if machine.frames.is_empty() {
            return Ok(InstructionProgress::Complete(values));
        }
        machine.values.extend(values);
        Ok(InstructionProgress::Continue)
    }

    fn branch(&mut self, machine: &mut Machine, depth: u32) -> Result<InstructionProgress, CoreError> {
        let frame_index = machine.frames.len().checked_sub(1).ok_or_else(|| CoreError::Trap("branch has no frame".into()))?;
        let control_count = machine.frames[frame_index].controls.len();
        let target_index = control_count.checked_sub(depth as usize + 1).ok_or_else(|| CoreError::Trap(format!("branch depth {depth} is out of bounds")))?;
        let target = machine.frames[frame_index].controls[target_index].clone();
        let values = take_results(machine, target.stack_height, &target.branch_types)?;
        machine.values.extend(values);
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

fn close_control(machine: &mut Machine, control: &ControlFrame) -> Result<(), CoreError> {
    let values = take_results(machine, control.stack_height, &control.result_types)?;
    machine.values.extend(values);
    Ok(())
}

fn pop(machine: &mut Machine) -> Result<Value, CoreError> {
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

fn check_values(values: &[Value], types: &[ValueType]) -> Result<(), CoreError> {
    if values.len() != types.len() {
        return Err(CoreError::Trap(format!("expected {} values, received {}", types.len(), values.len())));
    }
    for (value, expected) in values.iter().zip(types) {
        expect_type(*value, *expected)?;
    }
    Ok(())
}

fn expect_type(value: Value, expected: ValueType) -> Result<(), CoreError> {
    if value.value_type() == expected {
        Ok(())
    } else {
        Err(CoreError::Trap(format!("expected {expected:?}, received {:?}", value.value_type())))
    }
}

//#endregion ⚙️InstructionExecution

//#region 💾️MemoryAndBulkExecution

impl CoreInstance {
    fn execute_memory(&mut self, opcode: u8, decoder: &mut Decoder<'_>, machine: &mut Machine) -> Result<(), CoreError> {
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

    fn grow_memory(&mut self, memory_index: u32, delta: u64) -> Option<u64> {
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

    fn execute_fc(&mut self, decoder: &mut Decoder<'_>, machine: &mut Machine) -> Result<(), CoreError> {
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
                    let source_bytes = self.memories.get(source_memory as usize).ok_or_else(|| CoreError::Trap("memory.copy source memory is out of bounds".into()))?;
                    let source_end = checked_end(source, length, source_bytes.bytes.len(), "memory.copy source")?;
                    let copied = source_bytes.bytes[source..source_end].to_vec();
                    let target = self.memories.get_mut(destination_memory as usize).ok_or_else(|| CoreError::Trap("memory.copy destination memory is out of bounds".into()))?;
                    let destination_end = checked_end(destination, length, target.bytes.len(), "memory.copy destination")?;
                    target.bytes[destination..destination_end].copy_from_slice(&copied);
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

//#region 🧮️NumericExecution

fn execute_numeric(opcode: u8, machine: &mut Machine) -> Result<(), CoreError> {
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

fn map_top(machine: &mut Machine, operation: impl FnOnce(Value) -> Result<Value, CoreError>) -> Result<(), CoreError> {
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

//#region 🔢️NumericSemantics

fn canonical_f32(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(0x7fc0_0000)
    } else {
        value
    }
}

fn canonical_f64(value: f64) -> f64 {
    if value.is_nan() {
        f64::from_bits(0x7ff8_0000_0000_0000)
    } else {
        value
    }
}

fn round_ties_even_f32(value: f32) -> f32 {
    value.round_ties_even()
}

fn round_ties_even_f64(value: f64) -> f64 {
    value.round_ties_even()
}

fn wasm_min_f32(left: f32, right: f32) -> f32 {
    if left.is_nan() || right.is_nan() {
        return f32::NAN;
    }
    if left == right {
        if left == 0.0 && (left.is_sign_negative() || right.is_sign_negative()) {
            -0.0
        } else {
            left
        }
    } else if left < right {
        left
    } else {
        right
    }
}

fn wasm_max_f32(left: f32, right: f32) -> f32 {
    if left.is_nan() || right.is_nan() {
        return f32::NAN;
    }
    if left == right {
        if left == 0.0 && (!left.is_sign_negative() || !right.is_sign_negative()) {
            0.0
        } else {
            left
        }
    } else if left > right {
        left
    } else {
        right
    }
}

fn wasm_min_f64(left: f64, right: f64) -> f64 {
    if left.is_nan() || right.is_nan() {
        return f64::NAN;
    }
    if left == right {
        if left == 0.0 && (left.is_sign_negative() || right.is_sign_negative()) {
            -0.0
        } else {
            left
        }
    } else if left < right {
        left
    } else {
        right
    }
}

fn wasm_max_f64(left: f64, right: f64) -> f64 {
    if left.is_nan() || right.is_nan() {
        return f64::NAN;
    }
    if left == right {
        if left == 0.0 && (!left.is_sign_negative() || !right.is_sign_negative()) {
            0.0
        } else {
            left
        }
    } else if left > right {
        left
    } else {
        right
    }
}

fn trunc_i32_from_f32(value: f32, signed: bool) -> Result<i32, CoreError> {
    trunc_i32_from_f64(f64::from(value), signed)
}

fn trunc_i32_from_f64(value: f64, signed: bool) -> Result<i32, CoreError> {
    if value.is_nan() {
        return Err(CoreError::Trap("invalid conversion to integer".into()));
    }
    let value = value.trunc();
    if signed {
        if !(-2_147_483_648.0..2_147_483_648.0).contains(&value) {
            return Err(CoreError::Trap("integer overflow".into()));
        }
        Ok(value as i32)
    } else {
        if !(0.0..4_294_967_296.0).contains(&value) {
            return Err(CoreError::Trap("integer overflow".into()));
        }
        Ok(value as u32 as i32)
    }
}

fn trunc_i64_from_f32(value: f32, signed: bool) -> Result<i64, CoreError> {
    trunc_i64_from_f64(f64::from(value), signed)
}

fn trunc_i64_from_f64(value: f64, signed: bool) -> Result<i64, CoreError> {
    if value.is_nan() {
        return Err(CoreError::Trap("invalid conversion to integer".into()));
    }
    let value = value.trunc();
    if signed {
        if value < -9_223_372_036_854_775_808.0 || value >= 9_223_372_036_854_775_808.0 {
            return Err(CoreError::Trap("integer overflow".into()));
        }
        Ok(value as i64)
    } else {
        if value < 0.0 || value >= 18_446_744_073_709_551_616.0 {
            return Err(CoreError::Trap("integer overflow".into()));
        }
        Ok(value as u64 as i64)
    }
}

fn saturating_i32_from_f32(value: f32, signed: bool) -> i32 {
    saturating_i32_from_f64(f64::from(value), signed)
}

fn saturating_i32_from_f64(value: f64, signed: bool) -> i32 {
    if value.is_nan() {
        return 0;
    }
    if signed {
        value.trunc().clamp(i32::MIN as f64, i32::MAX as f64) as i32
    } else {
        value.trunc().clamp(0.0, u32::MAX as f64) as u32 as i32
    }
}

fn saturating_i64_from_f32(value: f32, signed: bool) -> i64 {
    saturating_i64_from_f64(f64::from(value), signed)
}

fn saturating_i64_from_f64(value: f64, signed: bool) -> i64 {
    if value.is_nan() {
        return 0;
    }
    if signed {
        value.trunc().clamp(i64::MIN as f64, i64::MAX as f64) as i64
    } else {
        value.trunc().clamp(0.0, u64::MAX as f64) as u64 as i64
    }
}

//#endregion 🔢️NumericSemantics

//#region 💾️CheckpointCodec

impl CoreInstance {
    pub fn checkpoint(&self) -> Vec<u8> {
        let mut writer = CheckpointWriter::default();
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
        writer.output
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

#[derive(Default)]
struct CheckpointWriter {
    output: Vec<u8>,
}

impl CheckpointWriter {
    fn byte(&mut self, value: u8) {
        self.output.push(value);
    }

    fn bytes(&mut self, bytes: &[u8]) {
        self.output.extend_from_slice(bytes);
    }

    fn u32(&mut self, value: u32) {
        self.output.extend_from_slice(&value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.output.extend_from_slice(&value.to_le_bytes());
    }

    fn usize(&mut self, value: usize) {
        self.u64(value as u64);
    }

    fn sized_bytes(&mut self, bytes: &[u8]) {
        self.usize(bytes.len());
        self.bytes(bytes);
    }

    fn list<T>(&mut self, values: &[T], mut write: impl FnMut(&mut Self, &T)) {
        self.usize(values.len());
        for value in values {
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

    fn value(&mut self, value: Value) {
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

    fn value_type(&mut self, value: ValueType) {
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

    fn machine(&mut self, machine: &Machine) {
        self.list(&machine.values, |writer, value| writer.value(*value));
        self.list(&machine.frames, Self::frame);
        self.option(machine.pending_host.as_ref(), Self::pending_host);
    }

    fn frame(&mut self, frame: &Frame) {
        self.u32(frame.function);
        self.usize(frame.pc);
        self.usize(frame.stack_base);
        self.list(&frame.locals, |writer, value| writer.value(*value));
        self.list(&frame.controls, Self::control);
    }

    fn control(&mut self, control: &ControlFrame) {
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

    fn pending_host(&mut self, pending: &PendingHost) {
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
    fn new(bytes: &'a [u8]) -> Self {
        Self(Decoder::new(bytes))
    }

    fn done(&self) -> bool {
        self.0.done()
    }

    fn byte(&mut self) -> Result<u8, CoreError> {
        self.0.byte().map_err(|error| CoreError::State(error.to_string()))
    }

    fn bytes(&mut self, length: usize) -> Result<&'a [u8], CoreError> {
        self.0.bytes(length).map_err(|error| CoreError::State(error.to_string()))
    }

    fn u32(&mut self) -> Result<u32, CoreError> {
        Ok(u32::from_le_bytes(self.bytes(4)?.try_into().expect("four bytes")))
    }

    fn u64(&mut self) -> Result<u64, CoreError> {
        Ok(u64::from_le_bytes(self.bytes(8)?.try_into().expect("eight bytes")))
    }

    fn usize(&mut self) -> Result<usize, CoreError> {
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

    fn value(&mut self) -> Result<Value, CoreError> {
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

    fn value_type(&mut self) -> Result<ValueType, CoreError> {
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

    fn machine(&mut self) -> Result<Machine, CoreError> {
        Ok(Machine { values: self.list(Self::value)?, frames: self.list(Self::frame)?, pending_host: self.option(Self::pending_host)? })
    }

    fn frame(&mut self) -> Result<Frame, CoreError> {
        Ok(Frame { function: self.u32()?, pc: self.usize()?, stack_base: self.usize()?, locals: self.list(Self::value)?, controls: self.list(Self::control)? })
    }

    fn control(&mut self) -> Result<ControlFrame, CoreError> {
        let kind = match self.byte()? {
            0 => ControlKind::Function,
            1 => ControlKind::Block,
            2 => ControlKind::Loop,
            3 => ControlKind::If,
            tag => return Err(CoreError::State(format!("checkpoint control tag {tag} is invalid"))),
        };
        Ok(ControlFrame { kind, start_pc: self.usize()?, end_pc: self.usize()?, stack_height: self.usize()?, branch_types: self.list(Self::value_type)?, result_types: self.list(Self::value_type)? })
    }

    fn pending_host(&mut self) -> Result<PendingHost, CoreError> {
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

//#region 🧰️Utilities

fn pages_to_bytes(pages: u64) -> Result<usize, CoreError> {
    let bytes = pages.checked_mul(WASM_PAGE_BYTES as u64).ok_or_else(|| CoreError::Validation("memory page count overflows bytes".into()))?;
    usize::try_from(bytes).map_err(|_| CoreError::Validation("memory size exceeds host usize".into()))
}

fn checked_end(start: usize, length: usize, bound: usize, subject: &str) -> Result<usize, CoreError> {
    let end = start.checked_add(length).ok_or_else(|| CoreError::Trap(format!("{subject} range overflow")))?;
    if end > bound {
        Err(CoreError::Trap(format!("{subject} is out of bounds")))
    } else {
        Ok(end)
    }
}

fn stable_fingerprint(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

//#endregion 🧰️Utilities

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn module(sections: &[(u8, Vec<u8>)]) -> Vec<u8> {
        let mut bytes = b"\0asm\x01\0\0\0".to_vec();
        for (id, payload) in sections {
            bytes.push(*id);
            leb(payload.len() as u32, &mut bytes);
            bytes.extend(payload);
        }
        bytes
    }

    fn leb(mut value: u32, output: &mut Vec<u8>) {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            output.push(byte);
            if value == 0 {
                return;
            }
        }
    }

    fn function_type(parameters: &[u8], results: &[u8]) -> Vec<u8> {
        let mut payload = vec![1, 0x60, parameters.len() as u8];
        payload.extend(parameters);
        payload.push(results.len() as u8);
        payload.extend(results);
        payload
    }

    fn one_function(body: &[u8]) -> Vec<u8> {
        let mut payload = vec![1];
        leb(body.len() as u32, &mut payload);
        payload.extend(body);
        payload
    }

    fn exported_function(name: &str, index: u8) -> Vec<u8> {
        let mut payload = vec![1, name.len() as u8];
        payload.extend(name.as_bytes());
        payload.extend([0, index]);
        payload
    }

    fn add_module() -> Vec<u8> {
        module(&[(1, function_type(&[0x7f, 0x7f], &[0x7f])), (3, vec![1, 0]), (7, exported_function("add", 0)), (10, one_function(&[0, 0x20, 0, 0x20, 1, 0x6a, 0x0b]))])
    }

    fn loop_module() -> Vec<u8> {
        let body = [1, 1, 0x7f, 0x41, 0, 0x21, 1, 0x02, 0x40, 0x03, 0x40, 0x20, 0, 0x45, 0x0d, 1, 0x20, 1, 0x20, 0, 0x6a, 0x21, 1, 0x20, 0, 0x41, 1, 0x6b, 0x21, 0, 0x0c, 0, 0x0b, 0x0b, 0x20, 1, 0x0b];
        module(&[(1, function_type(&[0x7f], &[0x7f])), (3, vec![1, 0]), (7, exported_function("sum", 0)), (10, one_function(&body))])
    }

    fn host_module() -> Vec<u8> {
        let mut imports = vec![1, 3];
        imports.extend(b"env");
        imports.push(6);
        imports.extend(b"double");
        imports.extend([0, 0]);
        module(&[(1, function_type(&[0x7f], &[0x7f])), (2, imports), (3, vec![1, 0]), (7, exported_function("call", 1)), (10, one_function(&[0, 0x20, 0, 0x10, 0, 0x0b]))])
    }

    fn memory_module() -> Vec<u8> {
        let mut exports = vec![2, 6];
        exports.extend(b"memory");
        exports.extend([2, 0, 5]);
        exports.extend(b"round");
        exports.extend([0, 0]);
        module(&[(1, function_type(&[0x7f], &[0x7f])), (3, vec![1, 0]), (5, vec![1, 1, 1, 2]), (7, exports), (10, one_function(&[0, 0x20, 0, 0x41, 0xfb, 0, 0x36, 2, 0, 0x20, 0, 0x28, 2, 0, 0x0b]))])
    }

    fn passive_data_module() -> Vec<u8> {
        module(&[
            (1, function_type(&[], &[0x7f])),
            (3, vec![1, 0]),
            (5, vec![1, 0, 1]),
            (7, exported_function("load", 0)),
            (12, vec![1]),
            (10, one_function(&[0, 0x41, 0, 0x41, 0, 0x41, 4, 0xfc, 8, 0, 0, 0xfc, 9, 0, 0x41, 0, 0x28, 2, 0, 0x0b])),
            (11, vec![1, 1, 4, 1, 2, 3, 4]),
        ])
    }

    fn drive(instance: &mut CoreInstance, fuel: u64) -> Result<(Vec<Value>, u64), CoreError> {
        let mut total = 0;
        loop {
            match instance.step(fuel, StepControl::default()) {
                CoreStepOutcome::Yield { fuel_used } => total += fuel_used,
                CoreStepOutcome::Complete { fuel_used, values } => return Ok((values, total + fuel_used)),
                CoreStepOutcome::Fault { error, .. } => return Err(error),
                other => return Err(CoreError::State(format!("unexpected outcome {other:?}"))),
            }
        }
    }

    #[test]
    fn add_executes_one_instruction_per_unit_of_fuel() {
        let module = Arc::new(CoreModule::parse(&add_module()).expect("parse add"));
        let mut instance = CoreInstance::instantiate(module).expect("instantiate add");
        instance.begin_export("add", vec![Value::I32(20), Value::I32(22)]).expect("begin add");
        let (values, fuel) = drive(&mut instance, 1).expect("drive add");
        assert_eq!(values, vec![Value::I32(42)]);
        assert_eq!(fuel, 4);
    }

    #[test]
    fn loop_checkpoint_resumes_byte_identically_at_every_instruction() {
        let module = Arc::new(CoreModule::parse(&loop_module()).expect("parse loop"));
        let mut baseline = CoreInstance::instantiate(Arc::clone(&module)).expect("instantiate loop");
        baseline.begin_export("sum", vec![Value::I32(100)]).expect("begin loop");
        let mut checkpoints = Vec::new();
        loop {
            checkpoints.push(baseline.checkpoint());
            match baseline.step(1, StepControl::default()) {
                CoreStepOutcome::Yield { .. } => {}
                CoreStepOutcome::Complete { values, .. } => {
                    assert_eq!(values, vec![Value::I32(5050)]);
                    break;
                }
                other => panic!("unexpected loop outcome {other:?}"),
            }
        }
        for checkpoint in checkpoints {
            let mut resumed = CoreInstance::restore(Arc::clone(&module), &checkpoint).expect("restore loop");
            assert_eq!(drive(&mut resumed, 7).expect("finish resumed loop").0, vec![Value::I32(5050)]);
        }
    }

    #[test]
    fn host_call_is_an_explicit_resumable_boundary() {
        let module = Arc::new(CoreModule::parse(&host_module()).expect("parse host module"));
        let mut instance = CoreInstance::instantiate(Arc::clone(&module)).expect("instantiate host module");
        instance.begin_export("call", vec![Value::I32(21)]).expect("begin host call");
        let call = loop {
            match instance.step(1, StepControl::default()) {
                CoreStepOutcome::Yield { .. } => {}
                CoreStepOutcome::HostCall { call, .. } => break call,
                other => panic!("unexpected pre-host outcome {other:?}"),
            }
        };
        assert_eq!((call.module.as_str(), call.name.as_str(), call.arguments.as_slice()), ("env", "double", [Value::I32(21)].as_slice()));
        let checkpoint = instance.checkpoint();
        let mut resumed = CoreInstance::restore(module, &checkpoint).expect("restore pending host call");
        assert_eq!(resumed.pending_host_call(), Some(&call));
        resumed.resume_host(call.id, Ok(vec![Value::I32(42)])).expect("resume host");
        assert_eq!(drive(&mut resumed, 1).expect("finish host call").0, vec![Value::I32(42)]);
    }

    #[test]
    fn memory_access_and_growth_are_bounded() {
        let module = Arc::new(CoreModule::parse(&memory_module()).expect("parse memory module"));
        let mut instance = CoreInstance::instantiate(module).expect("instantiate memory module");
        instance.begin_export("round", vec![Value::I32(64)]).expect("begin roundtrip");
        assert_eq!(drive(&mut instance, 2).expect("drive roundtrip").0, vec![Value::I32(123)]);
        assert_eq!(&instance.memory(0).expect("memory")[64..68], &[123, 0, 0, 0]);
        assert_eq!(instance.grow_memory(0, 1), Some(1));
        assert_eq!(instance.grow_memory(0, 1), None);
    }

    #[test]
    fn semio_describe_host_preserves_context_backpressure_and_bounded_result() {
        let module = Arc::new(CoreModule::parse(&memory_module()).expect("parse memory module"));
        let core = CoreInstance::instantiate(module).expect("instantiate memory module");
        let mut actor = SemioActorInstance { component_fingerprint: 1, core };
        let mut host = SemioDescribeHost::new(4);
        let set = HostCall { id: 1, module: "$root".into(), name: "[context-set-0]".into(), arguments: vec![Value::I32(77)], results: vec![] };
        assert_eq!(host.reply(&mut actor, &set).expect("set context").results, vec![]);
        let get = HostCall { id: 2, module: "$root".into(), name: "[context-get-0]".into(), arguments: vec![], results: vec![ValueType::I32] };
        assert_eq!(host.reply(&mut actor, &get).expect("get context").results, vec![Value::I32(77)]);
        let check_write = HostCall { id: 3, module: "wasi:io/streams@0.2.0".into(), name: "[method]output-stream.check-write".into(), arguments: vec![Value::I32(1), Value::I32(0)], results: vec![] };
        host.reply(&mut actor, &check_write).expect("check write");
        assert_eq!(&actor.memory().expect("memory")[8..16], &65_536u64.to_le_bytes());
        actor.memory_mut().expect("memory")[16..20].copy_from_slice(b"test");
        let returned = HostCall { id: 4, module: SEMIO_DESCRIBE_RETURN_MODULE.into(), name: SEMIO_DESCRIBE_RETURN_NAME.into(), arguments: vec![Value::I32(16), Value::I32(4)], results: vec![] };
        assert_eq!(host.reply(&mut actor, &returned).expect("describe result").descriptor, Some(b"test".to_vec()));
    }

    #[test]
    fn passive_data_initialization_and_drop_execute_in_section_order() {
        let module = Arc::new(CoreModule::parse(&passive_data_module()).expect("parse passive data module"));
        let mut instance = CoreInstance::instantiate(module).expect("instantiate passive data module");
        instance.begin_export("load", Vec::new()).expect("begin passive load");
        assert_eq!(drive(&mut instance, 1).expect("drive passive load").0, vec![Value::I32(0x0403_0201)]);
        assert!(instance.data[0].is_none());
    }

    #[test]
    fn cancellation_discards_the_active_machine_before_an_instruction_runs() {
        let module = Arc::new(CoreModule::parse(&loop_module()).expect("parse loop"));
        let mut instance = CoreInstance::instantiate(module).expect("instantiate loop");
        instance.begin_export("sum", vec![Value::I32(1_000_000)]).expect("begin loop");
        assert_eq!(instance.step(50, StepControl { cancelled: true }), CoreStepOutcome::Cancelled { fuel_used: 0 });
        assert!(!instance.active());
    }

    #[test]
    fn checkpoints_are_deterministic_and_module_bound() {
        let module = Arc::new(CoreModule::parse(&add_module()).expect("parse add"));
        let mut instance = CoreInstance::instantiate(Arc::clone(&module)).expect("instantiate add");
        instance.begin_export("add", vec![Value::I32(1), Value::I32(2)]).expect("begin add");
        instance.step(2, StepControl::default());
        assert_eq!(instance.checkpoint(), instance.checkpoint());
        let other = Arc::new(CoreModule::parse(&loop_module()).expect("parse other"));
        assert!(CoreInstance::restore(other, &instance.checkpoint()).is_err());
    }

    #[test]
    fn component_artifact_owns_nested_core_modules_without_platform_wasm() {
        let core = add_module();
        let mut component = b"\0asm\x0d\0\x01\0".to_vec();
        component.push(1);
        leb(core.len() as u32, &mut component);
        component.extend(&core);
        component.extend([0, 5, 4]);
        component.extend(b"test");
        let artifact = WasmArtifact::parse(&component).expect("parse component");
        assert_eq!(artifact.kind(), WasmArtifactKind::Component);
        let WasmArtifact::Component(component) = artifact else { unreachable!() };
        assert_eq!(component.core_modules().len(), 1);
        assert_eq!(component.sections().len(), 2);
        assert_eq!(component.sections()[0].kind, ComponentSectionKind::CoreModule);
        assert_eq!(component.sections()[1].custom_name.as_deref(), Some("test"));
        let mut instance = CoreInstance::instantiate(Arc::clone(&component.core_modules()[0])).expect("instantiate nested core");
        instance.begin_export("add", vec![Value::I32(7), Value::I32(8)]).expect("begin nested add");
        assert_eq!(drive(&mut instance, 1).expect("drive nested add").0, vec![Value::I32(15)]);
    }

    #[test]
    fn component_limits_reject_oversize_before_section_allocation() {
        let bytes = b"\0asm\x0d\0\x01\0";
        let limits = ComponentLimits { maximum_bytes: 7, maximum_nesting: 0, maximum_core_modules: 0 };
        assert!(matches!(ComponentArtifact::parse_with_limits(bytes, limits), Err(CoreError::Validation(_))));
    }

    #[test]
    fn execution_binary_recursively_removes_only_custom_sections() {
        let raw_core = add_module();
        let mut core = raw_core[..8].to_vec();
        core.extend([0, 6, 1, b'c', 1, 2, 3, 4]);
        core.extend_from_slice(&raw_core[8..]);
        let mut component = b"\0asm\x0d\0\x01\0".to_vec();
        component.push(1);
        leb(core.len() as u32, &mut component);
        component.extend_from_slice(&core);
        component.extend([0, 6, 1, b't', 1, 2, 3, 4]);
        let normalized = wasm_execution_binary(&component).expect("normalize component");
        assert!(normalized.len() < component.len());
        let artifact = ComponentArtifact::parse(&normalized).expect("parse normalized component");
        assert_eq!(artifact.sections().len(), 1);
        let mut instance = CoreInstance::instantiate(Arc::clone(&artifact.core_modules()[0])).expect("instantiate normalized core");
        instance.begin_export("add", vec![Value::I32(19), Value::I32(23)]).expect("begin normalized add");
        assert_eq!(drive(&mut instance, 1).expect("drive normalized add").0, vec![Value::I32(42)]);
    }

    #[test]
    fn configured_component_fixture_is_owned_parseable() {
        let Some(path) = std::env::var_os("SEMIO_OWNED_COMPONENT_FIXTURE") else { return };
        let bytes = std::fs::read(path).expect("read configured component fixture");
        let WasmArtifact::Component(component) = WasmArtifact::parse(&bytes).expect("parse configured component fixture") else { panic!("configured fixture is not a component") };
        assert!(!component.core_modules().is_empty(), "configured component has no executable core module");
        let actor = SemioActorArtifact::from_component(component).expect("validate configured Semio actor ABI");
        assert!(actor.module().export(SEMIO_DESCRIBE_EXPORT).is_some());
        let execution = wasm_execution_binary(&bytes).expect("normalize configured component fixture");
        assert!(execution.len() < bytes.len());
        SemioActorArtifact::parse(&execution).expect("validate normalized Semio actor ABI");
    }

    mod long {
        use super::*;

        #[test]
        fn configured_component_fixture_owned_describe_runs_bounded_fuel() {
            let Some(path) = std::env::var_os("SEMIO_OWNED_COMPONENT_FIXTURE") else { return };
            let bytes = std::fs::read(path).expect("read configured component fixture");
            let artifact = SemioActorArtifact::parse(&bytes).expect("parse configured Semio actor");
            let mut actor = artifact.instantiate().expect("instantiate configured Semio actor");
            assert!(!actor.startup_active(), "configured Semio actor has an undriven start function");
            actor.begin_describe().expect("begin owned describe");
            let mut host = SemioDescribeHost::new(64 * 1024 * 1024);
            let mut task_return = None;
            let mut fuel_used = 0;
            let mut host_calls = 0;
            while fuel_used < 100_000_000 {
                let grant = (100_000_000 - fuel_used).min(100_000);
                match actor.step(grant, StepControl::default()) {
                    CoreStepOutcome::Yield { fuel_used: used } => fuel_used += used,
                    CoreStepOutcome::HostCall { fuel_used: used, call } => {
                        fuel_used += used;
                        host_calls += 1;
                        let reply = host.reply(&mut actor, &call).expect("serve owned describe host call");
                        task_return = reply.descriptor.or(task_return);
                        actor.resume_host(call.id, Ok(reply.results)).expect("resume owned describe host call");
                    }
                    CoreStepOutcome::Complete { fuel_used: used, .. } => {
                        fuel_used += used;
                        break;
                    }
                    CoreStepOutcome::Cancelled { .. } => panic!("owned describe cancelled"),
                    CoreStepOutcome::Fault { error, .. } => panic!("owned describe fault: {error}"),
                }
            }
            assert_eq!(fuel_used, 100_000_000);
            assert!(host_calls > 0);
            assert!(task_return.is_none(), "stdio fixture unexpectedly completed inside the documented probe budget");
        }
    }
}

//#endregion 🧪️Tests
