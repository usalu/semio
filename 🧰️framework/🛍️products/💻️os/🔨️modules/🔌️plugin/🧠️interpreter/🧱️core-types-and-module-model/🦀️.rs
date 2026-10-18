//! 🧱️ Core types and module model: the value types and values, the core error, host calls and step outcomes, the module declarations, limits and control bounds and the core module record. See [`super`].

use super::*;

//#region 🧬️CoreTypes
pub(super) const WASM_PAGE_BYTES: usize = 65_536;

pub(super) const CHECKPOINT_MAGIC: &[u8; 8] = b"SEMIOWCP";

pub(super) const CHECKPOINT_VERSION: u8 = 1;

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
    pub(super) fn parse(byte: u8) -> Result<Self, CoreError> {
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

    pub(super) fn zero(self) -> Value {
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

    pub(super) fn as_i32(self) -> Result<i32, CoreError> {
        match self {
            Self::I32(value) => Ok(value),
            _ => Err(CoreError::Trap("expected i32".into())),
        }
    }

    pub(super) fn as_i64(self) -> Result<i64, CoreError> {
        match self {
            Self::I64(value) => Ok(value),
            _ => Err(CoreError::Trap("expected i64".into())),
        }
    }

    pub(super) fn as_f32(self) -> Result<f32, CoreError> {
        match self {
            Self::F32(value) => Ok(f32::from_bits(value)),
            _ => Err(CoreError::Trap("expected f32".into())),
        }
    }

    pub(super) fn as_f64(self) -> Result<f64, CoreError> {
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
    pub(super) bytes_fingerprint: u64,
    pub(super) types: Vec<FunctionType>,
    pub(super) functions: Vec<FunctionDecl>,
    pub(super) tables: Vec<TableLimits>,
    pub(super) memories: Vec<MemoryLimits>,
    pub(super) globals: Vec<GlobalDecl>,
    pub(super) exports: BTreeMap<String, ExportKind>,
    pub(super) start: Option<u32>,
    pub(super) elements: Vec<ElementDecl>,
    pub(super) data: Vec<DataDecl>,
}

#[derive(Clone, Debug)]
pub(super) enum FunctionDecl {
    Import { module: String, name: String, type_index: u32 },
    Defined { type_index: u32, locals: Vec<ValueType>, body: Arc<[u8]>, controls: Arc<BTreeMap<usize, ControlBounds>> },
}

impl FunctionDecl {
    pub(super) fn type_index(&self) -> u32 {
        match self {
            Self::Import { type_index, .. } | Self::Defined { type_index, .. } => *type_index,
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct GlobalDecl {
    pub(super) value_type: ValueType,
    pub(super) mutable: bool,
    pub(super) initializer: ConstExpr,
}

#[derive(Clone, Debug)]
pub(super) enum ConstExpr {
    Value(Value),
    Global(u32),
    RefFunction(u32),
}

#[derive(Clone, Debug)]
pub(super) struct ElementDecl {
    pub(super) mode: SegmentMode,
    pub(super) table: u32,
    pub(super) offset: Option<ConstExpr>,
    pub(super) values: Vec<Option<u32>>,
}

#[derive(Clone, Debug)]
pub(super) struct DataDecl {
    pub(super) mode: SegmentMode,
    pub(super) memory: u32,
    pub(super) offset: Option<ConstExpr>,
    pub(super) bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SegmentMode {
    Active,
    Passive,
    Declarative,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ControlBounds {
    pub(super) else_pc: Option<usize>,
    pub(super) end_pc: usize,
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

    pub fn exports(&self) -> impl Iterator<Item = (&str, &ExportKind)> {
        self.exports.iter().map(|(name, export)| (name.as_str(), export))
    }
}
//#endregion 📦️ModuleModel
