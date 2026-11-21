//! 🧱️ Core types and module model: the value types and values, the core error, host calls and step outcomes, the module declarations, limits and control bounds and the core module record. See [`super`].

use super::*;

//#region 🧬️CoreTypes
pub(super) const WASM_PAGE_BYTES: usize = 65_536;

pub(super) const CHECKPOINT_MAGIC: &[u8; 8] = b"SEMIOWCP";

pub(super) const CHECKPOINT_VERSION: u8 = 1;

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
