//! 📥️ Retained typed Pack and SPR loading for one exact window-config partition.

use super::{PluginCloseStep, WindowConfigOwner, WindowConfigPack, WindowConfigPartition};
use crate::store;
use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::mem::ManuallyDrop;

type Mounted = store::mounted_pack_rt::RetainedValueToken;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowConfigPackLoadGrant {
    pub maximum_items: usize,
    pub maximum_bytes: usize,
}

impl WindowConfigPackLoadGrant {
    pub const fn one_page() -> Self {
        Self { maximum_items: 1, maximum_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowConfigPackLoadDiagnostic {
    EnvelopeIdentity,
    Pack,
    TypedState,
    History,
    InnerIdentity,
    Replay,
    Capacity,
    Stale,
    Cancelled,
    Retirement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowConfigPackLoadPhase {
    EnvelopeIdentity,
    PackIngress,
    PackReplay,
    PackRetirement,
    HistoryReplay,
    InputRetirement,
    StoreHydration,
    Ready,
    RetiringRejectedCandidate,
    RetiringDisplacedStore,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowConfigPackLoadProgress {
    pub phase: WindowConfigPackLoadPhase,
    pub completed_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowConfigPackLoadStep {
    Pending(WindowConfigPackLoadProgress),
    Ready,
    Rejected(WindowConfigPackLoadDiagnostic),
    Complete,
}

pub(super) trait ErasedWindowConfigPackLoad: Send {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn window_id(&self) -> &str;
    fn window_kind_id(&self) -> &str;
    fn registry_lifetime(&self) -> u64;
    fn partition_generation(&self) -> Option<u64>;
    fn phase(&self) -> WindowConfigPackLoadPhase;
    fn progress(&self) -> WindowConfigPackLoadProgress;
    fn diagnostic(&self) -> Option<WindowConfigPackLoadDiagnostic>;
    fn advance(&mut self, grant: WindowConfigPackLoadGrant) -> WindowConfigPackLoadStep;
    fn request_cancel(&mut self);
    fn reject_stale(&mut self) -> WindowConfigPackLoadStep;
    fn close_step(&mut self, grant: WindowConfigPackLoadGrant) -> Result<PluginCloseStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct WindowConfigPackLoad {
    pub(super) inner: Box<dyn ErasedWindowConfigPackLoad>,
}

impl WindowConfigPackLoad {
    pub fn phase(&self) -> WindowConfigPackLoadPhase {
        self.inner.phase()
    }

    pub fn progress(&self) -> WindowConfigPackLoadProgress {
        self.inner.progress()
    }

    pub fn diagnostic(&self) -> Option<WindowConfigPackLoadDiagnostic> {
        self.inner.diagnostic()
    }

    pub fn request_cancel(&mut self) {
        self.inner.request_cancel();
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.inner.terminal_is_empty()
    }
}

impl Drop for WindowConfigPackLoad {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.inner.terminal_is_empty(), "window config Pack load reached Drop before terminal-empty handoff or retirement");
    }
}

#[derive(Clone)]
struct ExpectedValue {
    shape: Option<store::mounted_pack_rt::Shape>,
    dsl: bool,
}

impl ExpectedValue {
    fn field(shape: Option<store::mounted_pack_rt::Shape>) -> Self {
        Self { shape, dsl: false }
    }

    fn dsl() -> Self {
        Self { shape: None, dsl: true }
    }
}

enum BuiltValue {
    Field(store::mounted_pack_rt::FieldValue),
    Dsl(store::mounted_pack_rt::DslValue),
}

enum ValueFrame {
    Record {
        kind: store::mounted_pack_rt::RetainedValueContainer,
        spec: Option<store::mounted_pack_rt::RecordSpec>,
        fields: HashMap<u16, store::mounted_pack_rt::FieldValue>,
        field: Option<u16>,
    },
    Sequence {
        kind: store::mounted_pack_rt::RetainedValueContainer,
        element: ExpectedValue,
        values: Vec<BuiltValue>,
    },
    Map {
        dsl: bool,
        element: ExpectedValue,
        values: Vec<(String, BuiltValue)>,
        key: Option<String>,
    },
    Statements {
        variants: Vec<(String, fn() -> store::mounted_pack_rt::RecordSpec)>,
        values: Vec<(String, store::mounted_pack_rt::RecordValue)>,
        keyword: Option<String>,
    },
    Bytes {
        values: Vec<u8>,
        remaining: usize,
    },
}

#[derive(Clone, Copy)]
enum ValueWrapper {
    Block,
    Dynamic,
}

enum StringTarget {
    Value(ExpectedValue),
    MapKey,
    StatementKeyword,
}

struct RetainedString {
    target: StringTarget,
    value: String,
    remaining: Option<u64>,
    symbol: Option<(u64, usize, usize)>,
}

struct RetainedWindowConfigTypedState<O: WindowConfigOwner> {
    spec: store::mounted_pack_rt::RecordSpec,
    stack: Vec<ValueFrame>,
    wrappers: Vec<ValueWrapper>,
    string: Option<RetainedString>,
    tag: Option<u8>,
    root: Option<O::State>,
    complete: bool,
    handed_back: bool,
}

impl<O: WindowConfigOwner> RetainedWindowConfigTypedState<O> {
    fn new() -> Result<Self, WindowConfigPackLoadDiagnostic> {
        let spec = <O::State as store::ArtifactPack>::record_spec().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
        let mut stack = Vec::new();
        let mut wrappers = Vec::new();
        stack.try_reserve_exact(64).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        wrappers.try_reserve_exact(64).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        Ok(Self { spec, stack, wrappers, string: None, tag: None, root: None, complete: false, handed_back: false })
    }

    fn expected(&self) -> Result<ExpectedValue, WindowConfigPackLoadDiagnostic> {
        if let Some(wrapper) = self.wrappers.last() {
            return Ok(match wrapper {
                ValueWrapper::Block => {
                    let outer = self.parent_expected()?;
                    match outer.shape {
                        Some(store::mounted_pack_rt::Shape::Block(inner)) => ExpectedValue::field(Some(*inner)),
                        _ => ExpectedValue::field(None),
                    }
                }
                ValueWrapper::Dynamic => ExpectedValue::dsl(),
            });
        }
        self.parent_expected()
    }

    fn parent_expected(&self) -> Result<ExpectedValue, WindowConfigPackLoadDiagnostic> {
        match self.stack.last() {
            None if self.root.is_none() => Ok(ExpectedValue::field(Some(store::mounted_pack_rt::Shape::Record(Self::root_spec)))),
            Some(ValueFrame::Record { spec, field: Some(field), .. }) => Ok(ExpectedValue::field(spec.as_ref().and_then(|spec| spec.fields.iter().find(|candidate| candidate.id == *field)).map(|field| field.shape.clone()))),
            Some(ValueFrame::Sequence { element, .. }) => Ok(element.clone()),
            Some(ValueFrame::Map { element, key: Some(_), .. }) => Ok(element.clone()),
            Some(ValueFrame::Statements { variants, keyword: Some(keyword), .. }) => Ok(ExpectedValue::field(
                variants.iter().find(|(candidate, _)| candidate == keyword).map(|(_, spec)| store::mounted_pack_rt::Shape::Record(*spec)),
            )),
            _ => Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
    }

    fn root_spec() -> store::mounted_pack_rt::RecordSpec {
        <O::State as store::ArtifactPack>::record_spec().expect("retained window config owner was preflighted with a record spec")
    }

    fn child_record_spec(expected: &ExpectedValue) -> Option<store::mounted_pack_rt::RecordSpec> {
        match expected.shape.as_ref() {
            Some(store::mounted_pack_rt::Shape::Record(spec)) | Some(store::mounted_pack_rt::Shape::Table(spec)) => Some(spec()),
            _ => None,
        }
    }

    fn child_element(expected: &ExpectedValue) -> ExpectedValue {
        let shape = match expected.shape.as_ref() {
            Some(store::mounted_pack_rt::Shape::Tuple(inner, _)) | Some(store::mounted_pack_rt::Shape::List(inner)) | Some(store::mounted_pack_rt::Shape::Map(inner)) => Some(inner.as_ref().clone()),
            Some(store::mounted_pack_rt::Shape::Table(spec)) => Some(store::mounted_pack_rt::Shape::Record(*spec)),
            _ => None,
        };
        ExpectedValue { shape, dsl: expected.dsl }
    }

    fn string_target(&self) -> Result<StringTarget, WindowConfigPackLoadDiagnostic> {
        match self.stack.last() {
            Some(ValueFrame::Map { key: None, .. }) => Ok(StringTarget::MapKey),
            Some(ValueFrame::Statements { keyword: None, .. }) => Ok(StringTarget::StatementKeyword),
            _ => Ok(StringTarget::Value(self.expected()?)),
        }
    }

    fn begin_string(&mut self) -> Result<(), WindowConfigPackLoadDiagnostic> {
        if self.string.is_some() {
            return Err(WindowConfigPackLoadDiagnostic::TypedState);
        }
        self.string = Some(RetainedString { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    fn begin_symbol(&mut self, symbol: u64, catalog: &store::mounted_pack_rt::RetainedPackCatalogCursor) -> Result<(), WindowConfigPackLoadDiagnostic> {
        self.begin_string()?;
        let chars = catalog.symbol_chars(symbol).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        let owner = self.string.as_mut().expect("retained config string was just created");
        owner.value.try_reserve_exact(chars.saturating_mul(4)).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        owner.symbol = Some((symbol, 0, chars));
        if chars == 0 {
            self.finish_string()?;
        }
        Ok(())
    }

    fn grant_symbol(&mut self, catalog: &store::mounted_pack_rt::RetainedPackCatalogCursor) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, chars)) = owner.symbol else { return Ok(false) };
        let value = catalog.symbol_char(symbol, index).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?.ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        owner.value.push(value);
        if index + 1 == chars {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("retained config symbol remains owned").symbol = Some((symbol, index + 1, chars));
        }
        Ok(true)
    }

    fn finish_string(&mut self) -> Result<(), WindowConfigPackLoadDiagnostic> {
        let owner = self.string.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
        match owner.target {
            StringTarget::MapKey => match self.stack.last_mut() {
                Some(ValueFrame::Map { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            StringTarget::StatementKeyword => match self.stack.last_mut() {
                Some(ValueFrame::Statements { keyword, .. }) if keyword.is_none() => *keyword = Some(owner.value),
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            StringTarget::Value(expected) => {
                let value = if expected.dsl {
                    BuiltValue::Dsl(store::mounted_pack_rt::DslValue::String(owner.value))
                } else {
                    BuiltValue::Field(store::mounted_pack_rt::FieldValue::Text(owner.value))
                };
                self.emit(value)?;
            }
        }
        Ok(())
    }

    fn into_dsl(value: BuiltValue) -> Result<store::mounted_pack_rt::DslValue, WindowConfigPackLoadDiagnostic> {
        match value {
            BuiltValue::Dsl(value) => Ok(value),
            _ => Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
    }

    fn into_field(value: BuiltValue) -> Result<store::mounted_pack_rt::FieldValue, WindowConfigPackLoadDiagnostic> {
        match value {
            BuiltValue::Field(value) => Ok(value),
            _ => Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
    }

    fn emit(&mut self, mut value: BuiltValue) -> Result<(), WindowConfigPackLoadDiagnostic> {
        while let Some(wrapper) = self.wrappers.pop() {
            value = match wrapper {
                ValueWrapper::Block => BuiltValue::Field(store::mounted_pack_rt::FieldValue::Block(Box::new(Self::into_field(value)?))),
                ValueWrapper::Dynamic => BuiltValue::Field(store::mounted_pack_rt::FieldValue::Value(Self::into_dsl(value)?)),
            };
        }
        match self.stack.last_mut() {
            Some(ValueFrame::Record { fields, field, .. }) => {
                let id = field.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                if fields.insert(id, Self::into_field(value)?).is_some() {
                    return Err(WindowConfigPackLoadDiagnostic::TypedState);
                }
            }
            Some(ValueFrame::Sequence { values, .. }) => values.push(value),
            Some(ValueFrame::Map { values, key, .. }) => values.push((key.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?, value)),
            Some(ValueFrame::Statements { values, keyword, .. }) => {
                let keyword = keyword.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                let record = match Self::into_field(value)? {
                    store::mounted_pack_rt::FieldValue::Record(record) => record,
                    _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
                };
                values.push((keyword, record));
            }
            Some(ValueFrame::Bytes { .. }) => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            None => {
                let field = Self::into_field(value)?;
                let state = <O::State as store::mounted_pack_rt::DslField>::from_value(&field).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?;
                if self.root.replace(state).is_some() {
                    return Err(WindowConfigPackLoadDiagnostic::TypedState);
                }
            }
        }
        Ok(())
    }

    fn begin_container(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer, count: u64) -> Result<(), WindowConfigPackLoadDiagnostic> {
        let expected = self.expected()?;
        let count = usize::try_from(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        match kind {
            store::mounted_pack_rt::RetainedValueContainer::Record => {
                let spec = if self.stack.is_empty() && self.root.is_none() { Some(self.spec.clone()) } else { Self::child_record_spec(&expected) };
                let mut fields = HashMap::new();
                fields.try_reserve(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Record { kind, spec, fields, field: None });
            }
            store::mounted_pack_rt::RetainedValueContainer::Tuple
            | store::mounted_pack_rt::RetainedValueContainer::List
            | store::mounted_pack_rt::RetainedValueContainer::PackedF64
            | store::mounted_pack_rt::RetainedValueContainer::PackedVarint => {
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Sequence { kind, element: Self::child_element(&expected), values });
            }
            store::mounted_pack_rt::RetainedValueContainer::Map => {
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Map { dsl: expected.dsl, element: Self::child_element(&expected), values, key: None });
            }
            store::mounted_pack_rt::RetainedValueContainer::Statements => {
                let variants = match expected.shape {
                    Some(store::mounted_pack_rt::Shape::Statements(variants)) => variants,
                    _ => Vec::new(),
                };
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Statements { variants, values, keyword: None });
            }
            store::mounted_pack_rt::RetainedValueContainer::ChunkedBytes
            | store::mounted_pack_rt::RetainedValueContainer::Table
            | store::mounted_pack_rt::RetainedValueContainer::Wire => return Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
        Ok(())
    }

    fn end_container(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer) -> Result<(), WindowConfigPackLoadDiagnostic> {
        let frame = self.stack.pop().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
        let value = match frame {
            ValueFrame::Record { kind: expected, spec, mut fields, field: None } if expected == kind => {
                if let Some(spec) = spec {
                    for field in spec.fields {
                        fields.entry(field.id).or_insert(store::mounted_pack_rt::FieldValue::Absent);
                    }
                }
                BuiltValue::Field(store::mounted_pack_rt::FieldValue::Record(store::mounted_pack_rt::RecordValue { fields }))
            }
            ValueFrame::Sequence { kind: expected, values, .. } if expected == kind => {
                let values = values.into_iter().map(Self::into_field).collect::<Result<Vec<_>, _>>()?;
                let tuple = matches!(kind, store::mounted_pack_rt::RetainedValueContainer::Tuple)
                    || matches!(self.expected()?.shape, Some(store::mounted_pack_rt::Shape::Tuple(_, _)));
                BuiltValue::Field(if tuple { store::mounted_pack_rt::FieldValue::Tuple(values) } else { store::mounted_pack_rt::FieldValue::List(values) })
            }
            ValueFrame::Map { dsl: true, values, key: None, .. } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => {
                BuiltValue::Dsl(store::mounted_pack_rt::DslValue::Object(
                    values.into_iter().map(|(key, value)| Self::into_dsl(value).map(|value| (key, value))).collect::<Result<Vec<_>, _>>()?,
                ))
            }
            ValueFrame::Map { dsl: false, values, key: None, .. } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => {
                BuiltValue::Field(store::mounted_pack_rt::FieldValue::Map(
                    values.into_iter().map(|(key, value)| Self::into_field(value).map(|value| (key, value))).collect::<Result<Vec<_>, _>>()?,
                ))
            }
            ValueFrame::Statements { values, keyword: None, .. } if kind == store::mounted_pack_rt::RetainedValueContainer::Statements => {
                BuiltValue::Field(store::mounted_pack_rt::FieldValue::Statements(values))
            }
            _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
        };
        self.emit(value)
    }

    fn scalar_signed(&self, value: i64) -> BuiltValue {
        let expected = self.expected().ok();
        if expected.as_ref().is_some_and(|expected| expected.dsl) {
            BuiltValue::Dsl(store::mounted_pack_rt::DslValue::int(value))
        } else if expected.as_ref().and_then(|expected| expected.shape.as_ref()).is_some_and(|shape| matches!(shape, store::mounted_pack_rt::Shape::UInt | store::mounted_pack_rt::Shape::Count)) && value >= 0 {
            BuiltValue::Field(store::mounted_pack_rt::FieldValue::UInt(value as u64))
        } else if let Some(store::mounted_pack_rt::Shape::Enum(_)) = expected.as_ref().and_then(|expected| expected.shape.as_ref()) {
            BuiltValue::Field(store::mounted_pack_rt::FieldValue::Enum(value as u32))
        } else {
            BuiltValue::Field(store::mounted_pack_rt::FieldValue::Int(value))
        }
    }

    fn accept(&mut self, token: Mounted, catalog: &store::mounted_pack_rt::RetainedPackCatalogCursor) -> Result<(), WindowConfigPackLoadDiagnostic> {
        use store::mounted_pack_rt::{RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Tag { value: tag @ (0x00..=0x02), .. } => {
                let expected = self.expected()?;
                let value = match (tag, expected.dsl) {
                    (0x00, false) => BuiltValue::Field(store::mounted_pack_rt::FieldValue::Absent),
                    (0x01, false) => BuiltValue::Field(store::mounted_pack_rt::FieldValue::Bool(false)),
                    (0x02, false) => BuiltValue::Field(store::mounted_pack_rt::FieldValue::Bool(true)),
                    (0x01, true) => BuiltValue::Dsl(store::mounted_pack_rt::DslValue::Bool(false)),
                    (0x02, true) => BuiltValue::Dsl(store::mounted_pack_rt::DslValue::Bool(true)),
                    _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
                };
                self.emit(value)?;
            }
            Token::Tag { value: 0x12, .. } if self.expected()?.dsl => self.emit(BuiltValue::Dsl(store::mounted_pack_rt::DslValue::Null))?,
            Token::Tag { value: 0x0e, .. } => self.wrappers.push(ValueWrapper::Block),
            Token::Tag { value: 0x11, .. } => self.wrappers.push(ValueWrapper::Dynamic),
            Token::Tag { value, .. } if matches!(value, 0x03..=0x0d | 0x0f..=0x10 | 0x15..=0x17) => self.tag = Some(value),
            Token::Begin { kind, count } => {
                self.tag.take();
                self.begin_container(kind, count)?;
            }
            Token::Unsigned { role: Role::FieldId, value } => match self.stack.last_mut() {
                Some(ValueFrame::Record { field, .. }) if field.is_none() && value <= u16::MAX as u64 => *field = Some(value as u16),
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            Token::Unsigned { role: Role::Symbol, value } => {
                self.tag.take();
                self.begin_symbol(value, catalog)?;
            }
            Token::Unsigned { role: Role::StringLength, value } => {
                self.tag.take();
                self.begin_string()?;
                let owner = self.string.as_mut().expect("retained inline string was just created");
                owner.value.try_reserve_exact(usize::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::BytesLength, value } => {
                self.tag.take();
                let mut bytes = Vec::new();
                bytes.try_reserve_exact(usize::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                if value == 0 {
                    self.emit(BuiltValue::Field(store::mounted_pack_rt::FieldValue::Bytes64(bytes)))?;
                } else {
                    self.stack.push(ValueFrame::Bytes { values: bytes, remaining: usize::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)? });
                }
            }
            Token::StringChar(value) => {
                let owner = self.string.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                owner.value.push(value);
                let remaining = owner.remaining.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                *remaining = remaining.checked_sub(value.len_utf8() as u64).ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Byte(value) => match self.stack.last_mut() {
                Some(ValueFrame::Bytes { values, remaining }) => {
                    values.push(value);
                    *remaining = remaining.checked_sub(1).ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                    if *remaining == 0 {
                        let ValueFrame::Bytes { values, remaining: 0 } = self.stack.pop().expect("retained byte owner remains topmost") else { unreachable!() };
                        self.emit(BuiltValue::Field(store::mounted_pack_rt::FieldValue::Bytes64(values)))?;
                    }
                }
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            Token::Signed(value) => {
                self.tag.take();
                let scalar = self.scalar_signed(value);
                self.emit(scalar)?;
            }
            Token::Unsigned { role: Role::Unsigned | Role::Integer | Role::Enum, value } => {
                let expected = self.expected()?;
                let built = if expected.dsl {
                    BuiltValue::Dsl(store::mounted_pack_rt::DslValue::uint(value))
                } else if matches!(expected.shape, Some(store::mounted_pack_rt::Shape::Enum(_))) || self.tag == Some(0x0a) {
                    BuiltValue::Field(store::mounted_pack_rt::FieldValue::Enum(u32::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?))
                } else {
                    BuiltValue::Field(store::mounted_pack_rt::FieldValue::UInt(value))
                };
                self.tag.take();
                self.emit(built)?;
            }
            Token::F64(bits) => {
                self.tag.take();
                let expected = self.expected()?;
                let built = if expected.dsl {
                    BuiltValue::Dsl(store::mounted_pack_rt::DslValue::float(f64::from_bits(bits)))
                } else {
                    BuiltValue::Field(store::mounted_pack_rt::FieldValue::Float(f64::from_bits(bits)))
                };
                self.emit(built)?;
            }
            Token::End(kind) => self.end_container(kind)?,
            Token::Complete { .. } => {
                if !self.stack.is_empty() || !self.wrappers.is_empty() || self.string.is_some() || self.root.is_none() {
                    return Err(WindowConfigPackLoadDiagnostic::TypedState);
                }
                self.complete = true;
            }
            Token::Unsigned { role: Role::TableRows | Role::TableField | Role::Chunk, .. }
            | Token::WirePresence(_)
            | Token::WireNodePresence(_)
            | Token::WireLabelPresence(_)
            | Token::TablePresence { .. }
            | Token::TableBitmap { .. }
            | Token::Tag { .. } => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            Token::Unsigned { role: Role::Count | Role::StringLength | Role::BytesLength | Role::Symbol | Role::FieldId, .. } => return Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
        Ok(())
    }

    fn take(&mut self) -> Option<O::State> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.root.take()
    }

    fn close_step(&mut self) -> bool {
        self.string = None;
        if self.stack.pop().is_some() || self.wrappers.pop().is_some() {
            return false;
        }
        self.root = None;
        self.handed_back = true;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.root.is_none() && self.stack.is_empty() && self.wrappers.is_empty() && self.string.is_none()
    }
}

impl<O: WindowConfigOwner> Drop for RetainedWindowConfigTypedState<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained typed window config reached Drop before handoff or terminal-empty retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedStatePhase {
    Envelope,
    Ingress,
    Replay,
    Ready,
    Closing,
    Closed,
}

struct RetainedWindowConfigStateDecode<O: WindowConfigOwner> {
    phase: RetainedStatePhase,
    inner_start: usize,
    admitted: usize,
    document_byte: Option<(u64, u8)>,
    source: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackSourceCursor>>,
    anchor: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackAnchorCursor>>,
    segment: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackSegmentCursor>>,
    catalog: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackCatalogCursor>>,
    value: ManuallyDrop<Option<store::mounted_pack_rt::RetainedValueCursor>>,
    typed: ManuallyDrop<Option<RetainedWindowConfigTypedState<O>>>,
    catalog_value: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackCatalog>>,
    source_complete: bool,
    anchor_ready: bool,
    segment_complete: bool,
    catalog_complete: bool,
    value_sealed: bool,
    value_complete: bool,
    state: ManuallyDrop<Option<O::State>>,
    digest: Option<[u8; 32]>,
    hasher: semio_framework_hash::Hasher,
}

impl<O: WindowConfigOwner> RetainedWindowConfigStateDecode<O> {
    fn new() -> Self {
        Self {
            phase: RetainedStatePhase::Envelope,
            inner_start: 0,
            admitted: 0,
            document_byte: None,
            source: ManuallyDrop::new(None),
            anchor: ManuallyDrop::new(None),
            segment: ManuallyDrop::new(None),
            catalog: ManuallyDrop::new(None),
            value: ManuallyDrop::new(None),
            typed: ManuallyDrop::new(None),
            catalog_value: ManuallyDrop::new(None),
            source_complete: false,
            anchor_ready: false,
            segment_complete: false,
            catalog_complete: false,
            value_sealed: false,
            value_complete: false,
            state: ManuallyDrop::new(None),
            digest: None,
            hasher: semio_framework_hash::Hasher::new(),
        }
    }

    fn validate_envelope(&mut self, pack: &[u8], maximum_bytes: usize) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let envelope_id = <O::State as store::ArtifactDsl>::envelope_id();
        let expected_token_bytes = envelope_id.len().checked_add(".pack v1".len()).ok_or(WindowConfigPackLoadDiagnostic::EnvelopeIdentity)?;
        let header_bytes = 12usize.checked_add(expected_token_bytes).ok_or(WindowConfigPackLoadDiagnostic::EnvelopeIdentity)?;
        if maximum_bytes < header_bytes {
            return Ok(false);
        }
        if pack.len() <= header_bytes
            || pack.get(..8) != Some(store::semio_format::BINARY_MAGIC.as_slice())
            || pack.get(8..12).and_then(|bytes| <[u8; 4]>::try_from(bytes).ok()).map(u32::from_le_bytes) != Some(expected_token_bytes as u32)
        {
            return Err(WindowConfigPackLoadDiagnostic::EnvelopeIdentity);
        }
        let token = pack.get(12..header_bytes).ok_or(WindowConfigPackLoadDiagnostic::EnvelopeIdentity)?;
        let id = envelope_id.as_bytes();
        if token.get(..id.len()) != Some(id) || token.get(id.len()..) != Some(b".pack v1") {
            return Err(WindowConfigPackLoadDiagnostic::EnvelopeIdentity);
        }
        let inner_len = pack.len() - header_bytes;
        if inner_len > store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES {
            return Err(WindowConfigPackLoadDiagnostic::Capacity);
        }
        let pages = inner_len.div_ceil(store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES);
        if pages == 0 || pages > store::mounted_pack_rt::RETAINED_PACK_MAXIMUM_PAGES {
            return Err(WindowConfigPackLoadDiagnostic::Capacity);
        }
        let maximum_items = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
        let maximum_allocation = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES;
        let limits = || store::mounted_pack_rt::PackLimits {
            max_file_len: inner_len as u64,
            max_segment_len: inner_len as u64,
            max_symbols: maximum_items as u32,
            max_depth: 64,
            max_items: maximum_items as u64,
            max_total_alloc: maximum_allocation as u64,
        };
        *self.source = Some(store::mounted_pack_rt::RetainedPackSourceCursor::try_new(pages, inner_len, maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        *self.anchor = Some(store::mounted_pack_rt::RetainedPackAnchorCursor::new());
        *self.segment = Some(store::mounted_pack_rt::RetainedPackSegmentCursor::try_new(limits(), maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        *self.catalog = Some(
            store::mounted_pack_rt::RetainedPackCatalogCursor::try_new(limits(), maximum_items, inner_len, inner_len, maximum_items, maximum_allocation)
                .map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?,
        );
        *self.value = Some(store::mounted_pack_rt::RetainedValueCursor::try_new(limits(), maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        *self.typed = Some(RetainedWindowConfigTypedState::<O>::new()?);
        self.hasher.update(&pack[..header_bytes]);
        self.inner_start = header_bytes;
        self.phase = RetainedStatePhase::Ingress;
        Ok(true)
    }

    fn retained_allocated_bytes(&self) -> usize {
        self.source.as_ref().map_or(0, store::mounted_pack_rt::RetainedPackSourceCursor::allocated_bytes)
            + self.segment.as_ref().map_or(0, store::mounted_pack_rt::RetainedPackSegmentCursor::allocated_bytes)
            + self.catalog.as_ref().map_or(0, store::mounted_pack_rt::RetainedPackCatalogCursor::allocated_bytes)
            + self.value.as_ref().map_or(0, store::mounted_pack_rt::RetainedValueCursor::allocated_bytes)
    }

    fn reserve_next(&mut self, maximum_bytes: usize) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let allocated = self.retained_allocated_bytes();
        let remaining = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES.saturating_sub(allocated);
        if self.phase == RetainedStatePhase::Ingress {
            let source = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
            if source.has_reserved_page() {
                return Ok(false);
            }
            let requested = source.next_allocation_bytes().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            if requested > maximum_bytes || requested > remaining {
                return Ok(false);
            }
            let step = source.reserve_page(maximum_bytes.min(remaining)).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
            return Ok(step.progressed);
        }
        if self.phase != RetainedStatePhase::Replay {
            return Ok(false);
        }
        let segment = self.segment.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if let Some(requested) = segment.next_allocation_bytes() {
            if requested > maximum_bytes || requested > remaining {
                return Ok(false);
            }
            return segment.reserve_allocation(maximum_bytes.min(remaining)).map(|step| step.progressed).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity);
        }
        let value = self.value.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if let Some(requested) = value.next_allocation_bytes().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
            if requested > maximum_bytes || requested > remaining {
                return Ok(false);
            }
            return value.reserve_allocation(maximum_bytes.min(remaining)).map(|step| step.progressed).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity);
        }
        let catalog = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if let Some(requested) = catalog.next_allocation_bytes().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
            if requested > maximum_bytes || requested > remaining {
                return Ok(false);
            }
            return catalog.reserve_allocation(maximum_bytes.min(remaining)).map(|step| step.progressed).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity);
        }
        Ok(false)
    }

    fn ingress(&mut self, pack: &[u8], maximum_bytes: usize) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        if self.reserve_next(maximum_bytes)? {
            return Ok(true);
        }
        let source = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if !source.has_reserved_page() {
            return Ok(false);
        }
        let start = self.inner_start + self.admitted;
        let remaining = pack.len().saturating_sub(start);
        if remaining == 0 {
            source.seal().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            self.phase = RetainedStatePhase::Replay;
            return Ok(true);
        }
        let len = remaining.min(store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES).min(maximum_bytes);
        if len == 0 {
            return Ok(false);
        }
        source.preflight_page(len).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        let mut bytes = [0; store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES];
        bytes[..len].copy_from_slice(&pack[start..start + len]);
        self.hasher.update(&pack[start..start + len]);
        let page = store::mounted_pack_rt::RetainedPackPage::try_from_array(bytes, len).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        source.admit_page(page).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        self.admitted += len;
        Ok(true)
    }

    fn replay(&mut self, maximum_bytes: usize) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        if self.reserve_next(maximum_bytes)? {
            return Ok(true);
        }
        if self.typed.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?.grant_symbol(self.catalog.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?)? {
            return Ok(true);
        }
        if let Some((index, byte)) = self.document_byte {
            if self.value.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.ingress_ready() {
                self.document_byte = None;
                self.value.as_mut().expect("retained config value owner remains").admit_byte(index, byte).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                return Ok(true);
            }
        }
        if !self.value_complete {
            if let Some(token) = self.value.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                self.value_complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
                self.typed.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?.accept(token, self.catalog.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?)?;
                return Ok(true);
            }
        }
        if self.catalog_complete && !self.value_sealed {
            let bytes = self.catalog.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.document_bytes();
            self.value.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.seal(bytes).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            self.value_sealed = true;
            return Ok(true);
        }
        if self.catalog.as_ref().is_some_and(store::mounted_pack_rt::RetainedPackCatalogCursor::has_pending_input) {
            if let Some(event) = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                match event {
                    store::mounted_pack_rt::RetainedPackCatalogEvent::DocumentByte { index, value, .. } => self.document_byte = Some((index, value)),
                    store::mounted_pack_rt::RetainedPackCatalogEvent::Complete => self.catalog_complete = true,
                    _ => {}
                }
            }
            return Ok(true);
        }
        let segment_can_admit = self.segment.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.preflight().is_ok();
        if self.document_byte.is_none() && !self.segment_complete && (!segment_can_admit || self.source_complete) {
            if let Some(event) = self.segment.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                self.segment_complete = matches!(event, store::mounted_pack_rt::RetainedPackSegmentEvent::PackComplete { .. });
                let catalog = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
                catalog.admit(event).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                if let Some(event) = catalog.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                    match event {
                        store::mounted_pack_rt::RetainedPackCatalogEvent::DocumentByte { index, value, .. } => self.document_byte = Some((index, value)),
                        store::mounted_pack_rt::RetainedPackCatalogEvent::Complete => self.catalog_complete = true,
                        _ => {}
                    }
                }
                return Ok(true);
            }
        }
        if !self.source_complete && segment_can_admit {
            if let Some(event) = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                self.source_complete = matches!(event, store::mounted_pack_rt::RetainedPackSourceEvent::Complete { .. });
                self.anchor.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant(Some(event)).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                self.segment.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.admit(event).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                return Ok(true);
            }
        }
        if self.source_complete && !self.anchor_ready {
            self.anchor_ready = self.anchor.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant(None).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            return Ok(true);
        }
        if self.anchor_ready && self.catalog_complete && self.value_complete && self.catalog_value.is_none() {
            let superblock = self.anchor.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.take().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
            *self.catalog_value = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.take(superblock).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            *self.state = self.typed.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?.take();
            if self.state.is_none() {
                return Err(WindowConfigPackLoadDiagnostic::TypedState);
            }
            self.digest = Some(*self.hasher.finalize().as_bytes());
            self.phase = RetainedStatePhase::Ready;
            return Ok(true);
        }
        Ok(false)
    }

    fn advance(&mut self, pack: &[u8], grant: WindowConfigPackLoadGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        if grant.maximum_items == 0 {
            return Ok(false);
        }
        match self.phase {
            RetainedStatePhase::Envelope => self.validate_envelope(pack, grant.maximum_bytes),
            RetainedStatePhase::Ingress => self.ingress(pack, grant.maximum_bytes),
            RetainedStatePhase::Replay => self.replay(grant.maximum_bytes),
            RetainedStatePhase::Ready => Ok(true),
            RetainedStatePhase::Closing | RetainedStatePhase::Closed => Ok(false),
        }
    }

    fn take_ready(&mut self) -> Option<(O::State, [u8; 32])> {
        if self.phase != RetainedStatePhase::Ready {
            return None;
        }
        Some((self.state.take()?, self.digest.take()?))
    }

    fn request_cancel(&mut self) {
        if let Some(source) = self.source.as_mut() {
            source.request_cancel();
        }
        self.phase = RetainedStatePhase::Closing;
    }

    fn next_release_allocation_bytes(&self) -> Option<usize> {
        if self.document_byte.is_some() || self.catalog_value.is_some() || self.typed.is_some() || self.state.is_some() {
            return None;
        }
        if let Some(value) = self.value.as_ref() {
            return value.next_release_allocation_bytes();
        }
        if let Some(catalog) = self.catalog.as_ref() {
            return catalog.next_release_allocation_bytes().ok().flatten();
        }
        if let Some(segment) = self.segment.as_ref() {
            return segment.next_release_allocation_bytes();
        }
        self.source.as_ref()?.next_release_allocation_bytes().ok()
    }

    fn close_step(&mut self, grant: WindowConfigPackLoadGrant) -> Result<PluginCloseStep, String> {
        if grant.maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.request_cancel();
        if self.document_byte.take().is_some() || self.catalog_value.take().is_some() || self.state.take().is_some() {
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(typed) = self.typed.as_mut() {
            if !typed.close_step() {
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.typed.take();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(value) = self.value.as_mut() {
            return match value.close_step(1, grant.maximum_bytes).map_err(|error| error.to_string())? {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete if value.terminal_is_empty() => {
                    self.value.take();
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => Err("retained config value returned false terminal".into()),
            };
        }
        if let Some(catalog) = self.catalog.as_mut() {
            return match catalog.close_step(1, grant.maximum_bytes).map_err(str::to_string)? {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete if catalog.terminal_is_empty() => {
                    self.catalog.take();
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => Err("retained config catalog returned false terminal".into()),
            };
        }
        if let Some(segment) = self.segment.as_mut() {
            return match segment.close_step(1, grant.maximum_bytes) {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete if segment.terminal_is_empty() => {
                    self.segment.take();
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => Err("retained config segment returned false terminal".into()),
            };
        }
        if let Some(anchor) = self.anchor.as_mut() {
            anchor.close_step();
            self.anchor.take();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(source) = self.source.as_mut() {
            return match source.close_step(1, grant.maximum_bytes).map_err(|error| error.to_string())? {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete if source.terminal_is_empty() => {
                    self.source.take();
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => Err("retained config source returned false terminal".into()),
            };
        }
        self.phase = RetainedStatePhase::Closed;
        Ok(PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == RetainedStatePhase::Closed
            && self.document_byte.is_none()
            && self.source.is_none()
            && self.anchor.is_none()
            && self.segment.is_none()
            && self.catalog.is_none()
            && self.value.is_none()
            && self.typed.is_none()
            && self.catalog_value.is_none()
            && self.state.is_none()
    }
}

impl<O: WindowConfigOwner> Drop for RetainedWindowConfigStateDecode<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained window config state decoder reached Drop before terminal-empty retirement");
    }
}

struct TypedWindowConfigPackLoad<O: WindowConfigOwner> {
    window_id: ManuallyDrop<Option<String>>,
    window_kind_id: ManuallyDrop<Option<String>>,
    expected_id: ManuallyDrop<Option<String>>,
    files: ManuallyDrop<Option<store::ArtifactPackFiles>>,
    state_decode: ManuallyDrop<Option<RetainedWindowConfigStateDecode<O>>>,
    initial: ManuallyDrop<Option<O::State>>,
    validation: ManuallyDrop<Option<O::State>>,
    current: ManuallyDrop<Option<O::State>>,
    initial_digest: Option<[u8; 32]>,
    history_decode: ManuallyDrop<Option<store::RetainedHistoryDecode>>,
    history: ManuallyDrop<Option<store::HistoryLog>>,
    hydration: ManuallyDrop<Option<store::RetainedConfigStoreHydration<O::State, O::Mutation>>>,
    owners: ManuallyDrop<Option<store::DocumentStoreOwners<O::State, O::Mutation>>>,
    candidate: ManuallyDrop<Option<WindowConfigPartition<O>>>,
    displaced: ManuallyDrop<Option<WindowConfigPartition<O>>>,
    active: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    registry_lifetime: u64,
    partition_generation: Option<u64>,
    phase: WindowConfigPackLoadPhase,
    diagnostic: Option<WindowConfigPackLoadDiagnostic>,
    completed_bytes: u64,
    total_bytes: u64,
    decoded_states: u8,
    committed: bool,
    terminal: bool,
}

impl<O: WindowConfigOwner> TypedWindowConfigPackLoad<O> {
    fn new(registry_lifetime: u64, partition_generation: Option<u64>, pack: WindowConfigPack) -> Self {
        let WindowConfigPack { window_id, window_kind_id, files } = pack;
        let total_bytes = files.pack.len().saturating_mul(3).saturating_add(files.spr.len()).saturating_add(files.ops.len()) as u64;
        let expected_id = format!("window-config:{}:{window_id}", O::WINDOW_KIND_ID);
        Self {
            window_id: ManuallyDrop::new(Some(window_id)),
            window_kind_id: ManuallyDrop::new(Some(window_kind_id)),
            expected_id: ManuallyDrop::new(Some(expected_id)),
            files: ManuallyDrop::new(Some(files)),
            state_decode: ManuallyDrop::new(Some(RetainedWindowConfigStateDecode::new())),
            initial: ManuallyDrop::new(None),
            validation: ManuallyDrop::new(None),
            current: ManuallyDrop::new(None),
            initial_digest: None,
            history_decode: ManuallyDrop::new(None),
            history: ManuallyDrop::new(None),
            hydration: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(Some(O::build_store_owners())),
            candidate: ManuallyDrop::new(None),
            displaced: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            registry_lifetime,
            partition_generation,
            phase: WindowConfigPackLoadPhase::EnvelopeIdentity,
            diagnostic: None,
            completed_bytes: 0,
            total_bytes,
            decoded_states: 0,
            committed: false,
            terminal: false,
        }
    }

    fn progress_now(&self) -> WindowConfigPackLoadProgress {
        WindowConfigPackLoadProgress { phase: self.phase, completed_bytes: self.completed_bytes.min(self.total_bytes), total_bytes: self.total_bytes }
    }

    fn pending(&self) -> WindowConfigPackLoadStep {
        WindowConfigPackLoadStep::Pending(self.progress_now())
    }

    fn reject(&mut self, diagnostic: WindowConfigPackLoadDiagnostic) -> WindowConfigPackLoadStep {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = WindowConfigPackLoadPhase::RetiringRejectedCandidate;
        WindowConfigPackLoadStep::Rejected(self.diagnostic.unwrap())
    }

    fn map_hydration_diagnostic(diagnostic: store::ConfigStoreHydrationDiagnostic) -> WindowConfigPackLoadDiagnostic {
        match diagnostic {
            store::ConfigStoreHydrationDiagnostic::Identity => WindowConfigPackLoadDiagnostic::InnerIdentity,
            store::ConfigStoreHydrationDiagnostic::Malformed => WindowConfigPackLoadDiagnostic::History,
            store::ConfigStoreHydrationDiagnostic::Replay => WindowConfigPackLoadDiagnostic::Replay,
            store::ConfigStoreHydrationDiagnostic::Capacity => WindowConfigPackLoadDiagnostic::Capacity,
            store::ConfigStoreHydrationDiagnostic::Initialization => WindowConfigPackLoadDiagnostic::Replay,
            store::ConfigStoreHydrationDiagnostic::Cancelled => WindowConfigPackLoadDiagnostic::Cancelled,
        }
    }

    fn advance_decode(&mut self, grant: WindowConfigPackLoadGrant) -> WindowConfigPackLoadStep {
        let pack = &self.files.as_ref().expect("retained window config input remains").pack;
        let decoder = self.state_decode.as_mut().expect("retained window config state decoder remains");
        let before = decoder.admitted;
        match decoder.advance(pack, grant) {
            Ok(_) => {
                self.completed_bytes = self.completed_bytes.saturating_add(decoder.admitted.saturating_sub(before) as u64);
                self.phase = match decoder.phase {
                    RetainedStatePhase::Envelope => WindowConfigPackLoadPhase::EnvelopeIdentity,
                    RetainedStatePhase::Ingress => WindowConfigPackLoadPhase::PackIngress,
                    RetainedStatePhase::Replay => WindowConfigPackLoadPhase::PackReplay,
                    RetainedStatePhase::Ready => {
                        let (state, digest) = decoder.take_ready().expect("ready retained window config state remains");
                        if self.initial_digest.is_some_and(|expected| expected != digest) {
                            return self.reject(WindowConfigPackLoadDiagnostic::TypedState);
                        }
                        self.initial_digest.get_or_insert(digest);
                        match self.decoded_states {
                            0 => *self.initial = Some(state),
                            1 => *self.validation = Some(state),
                            2 => *self.current = Some(state),
                            _ => return self.reject(WindowConfigPackLoadDiagnostic::TypedState),
                        }
                        self.decoded_states += 1;
                        WindowConfigPackLoadPhase::PackRetirement
                    }
                    RetainedStatePhase::Closing | RetainedStatePhase::Closed => return self.reject(WindowConfigPackLoadDiagnostic::Pack),
                };
                self.pending()
            }
            Err(diagnostic) => self.reject(diagnostic),
        }
    }

    fn advance_pack_retirement(&mut self, grant: WindowConfigPackLoadGrant) -> WindowConfigPackLoadStep {
        let decoder = self.state_decode.as_mut().expect("retained window config state decoder remains during close");
        match decoder.close_step(grant) {
            Ok(PluginCloseStep::Complete) if decoder.terminal_is_empty() => {
                self.state_decode.take();
                if self.decoded_states < 3 {
                    *self.state_decode = Some(RetainedWindowConfigStateDecode::new());
                    self.phase = WindowConfigPackLoadPhase::EnvelopeIdentity;
                    return self.pending();
                }
                let spr_len = self.files.as_ref().expect("retained window config SPR input remains").spr.len();
                let limits = store::RetainedSprLimits {
                    file_bytes: spr_len as u64,
                    frame_body_bytes: spr_len as u64,
                    records: store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES as u64,
                };
                match store::RetainedHistoryDecode::new_persisted_document(spr_len, limits) {
                    Ok(decoder) => {
                        *self.history_decode = Some(decoder);
                        self.phase = WindowConfigPackLoadPhase::HistoryReplay;
                        self.pending()
                    }
                    Err(_) => self.reject(WindowConfigPackLoadDiagnostic::History),
                }
            }
            Ok(PluginCloseStep::Complete) => self.reject(WindowConfigPackLoadDiagnostic::Retirement),
            Ok(_) => self.pending(),
            Err(_) => self.reject(WindowConfigPackLoadDiagnostic::Retirement),
        }
    }

    fn advance_history(&mut self, grant: WindowConfigPackLoadGrant) -> WindowConfigPackLoadStep {
        let bytes = &self.files.as_ref().expect("retained window config SPR input remains").spr;
        let decoder = self.history_decode.as_mut().expect("retained window config history decoder remains");
        match decoder.step(bytes, grant.maximum_bytes, grant.maximum_items.min(1)) {
            Ok(store::RetainedHistoryDecodeStep::Pending { completed_bytes, .. }) => {
                self.completed_bytes = (self.files.as_ref().expect("retained window config input remains").pack.len() as u64).saturating_add(completed_bytes);
                self.pending()
            }
            Ok(store::RetainedHistoryDecodeStep::Ready) => {
                let history = decoder.take_ready().expect("ready retained window config history remains");
                let auxiliary = decoder.take_auxiliary_owners();
                if !decoder.terminal_is_empty() {
                    return self.reject(WindowConfigPackLoadDiagnostic::Retirement);
                }
                self.history_decode.take();
                *self.history = Some(history);
                *self.active = Some(store::retirement::owned_retirement(auxiliary));
                self.phase = WindowConfigPackLoadPhase::InputRetirement;
                self.pending()
            }
            Err(_) => self.reject(WindowConfigPackLoadDiagnostic::History),
        }
    }

    fn drive_active(&mut self, grant: WindowConfigPackLoadGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(grant.maximum_items.min(1), grant.maximum_bytes) {
            Ok(store::SnapshotRetirementStep::Complete) if active.terminal_is_empty() => {
                self.active.take();
                Ok(true)
            }
            Ok(store::SnapshotRetirementStep::Complete) => Err(WindowConfigPackLoadDiagnostic::Retirement),
            Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) if released_items <= grant.maximum_items.min(1) && released_bytes <= grant.maximum_bytes => Ok(true),
            _ => Err(WindowConfigPackLoadDiagnostic::Retirement),
        }
    }

    fn truncate_string(value: &mut String, maximum_bytes: usize) -> usize {
        if maximum_bytes == 0 || value.is_empty() {
            return 0;
        }
        let mut start = value.len().saturating_sub(maximum_bytes);
        while !value.is_char_boundary(start) {
            start += 1;
        }
        let released = value.len() - start;
        value.truncate(start);
        released
    }

    fn retire_files(&mut self, grant: WindowConfigPackLoadGrant) -> Option<(usize, usize)> {
        let Some(files) = self.files.as_mut() else { return None };
        if !files.pack.is_empty() {
            if grant.maximum_bytes == 0 {
                return None;
            }
            let released = files.pack.len().min(grant.maximum_bytes);
            files.pack.truncate(files.pack.len() - released);
            return Some((0, released));
        }
        if !files.spr.is_empty() {
            if grant.maximum_bytes == 0 {
                return None;
            }
            let released = files.spr.len().min(grant.maximum_bytes);
            files.spr.truncate(files.spr.len() - released);
            return Some((0, released));
        }
        if !files.ops.is_empty() {
            if grant.maximum_bytes == 0 {
                return None;
            }
            let released = Self::truncate_string(&mut files.ops, grant.maximum_bytes);
            return (released != 0).then_some((0, released));
        }
        if grant.maximum_items != 0 {
            self.files.take();
            return Some((1, 0));
        }
        None
    }

    fn advance_input_retirement(&mut self, grant: WindowConfigPackLoadGrant) -> WindowConfigPackLoadStep {
        match self.drive_active(grant) {
            Ok(true) => return self.pending(),
            Ok(false) => {}
            Err(diagnostic) => return self.reject(diagnostic),
        }
        if self.retire_files(grant).is_some() {
            return self.pending();
        }
        if self.files.is_some() {
            return self.pending();
        }
        let initial = self.initial.take().expect("decoded initial window config state remains");
        let validation = self.validation.take().expect("decoded validation window config state remains");
        let current = self.current.take().expect("decoded current window config state remains");
        let history = self.history.take().expect("decoded window config history remains");
        let expected_id = self.expected_id.take().expect("exact window config partition id remains");
        let owners = self.owners.take().expect("window config store owners remain");
        *self.hydration = Some(store::RetainedConfigStoreHydration::from_snapshots(
            initial,
            validation,
            current,
            history,
            expected_id,
            O::SCHEMA.to_string(),
            self.initial_digest.take().expect("decoded window config digest remains"),
            owners,
            self.partition_generation.map_or(0, |generation| generation.saturating_add(1)),
            O::MAXIMUM_PUBLICATION_BYTES,
        ));
        self.phase = WindowConfigPackLoadPhase::StoreHydration;
        self.pending()
    }

    fn advance_hydration(&mut self, grant: WindowConfigPackLoadGrant) -> WindowConfigPackLoadStep {
        let hydration = self.hydration.as_mut().expect("retained window config hydration remains");
        match hydration.advance(grant.maximum_items.min(1), grant.maximum_bytes) {
            store::ConfigStoreHydrationStep::Pending(_) => self.pending(),
            store::ConfigStoreHydrationStep::Rejected(diagnostic) => self.reject(Self::map_hydration_diagnostic(diagnostic)),
            store::ConfigStoreHydrationStep::Ready(store) => {
                self.hydration.take();
                *self.candidate = Some(WindowConfigPartition { store: *store, disposer: Some(O::build_store_disposer()) });
                self.phase = WindowConfigPackLoadPhase::Ready;
                WindowConfigPackLoadStep::Ready
            }
        }
    }

    fn advance_inner(&mut self, grant: WindowConfigPackLoadGrant) -> WindowConfigPackLoadStep {
        if let Some(diagnostic) = self.diagnostic {
            return WindowConfigPackLoadStep::Rejected(diagnostic);
        }
        if grant.maximum_items == 0 {
            return self.pending();
        }
        match self.phase {
            WindowConfigPackLoadPhase::EnvelopeIdentity | WindowConfigPackLoadPhase::PackIngress | WindowConfigPackLoadPhase::PackReplay => self.advance_decode(grant),
            WindowConfigPackLoadPhase::PackRetirement => self.advance_pack_retirement(grant),
            WindowConfigPackLoadPhase::HistoryReplay => self.advance_history(grant),
            WindowConfigPackLoadPhase::InputRetirement => self.advance_input_retirement(grant),
            WindowConfigPackLoadPhase::StoreHydration => self.advance_hydration(grant),
            WindowConfigPackLoadPhase::Ready => WindowConfigPackLoadStep::Ready,
            WindowConfigPackLoadPhase::RetiringRejectedCandidate => WindowConfigPackLoadStep::Rejected(self.diagnostic.unwrap_or(WindowConfigPackLoadDiagnostic::Cancelled)),
            WindowConfigPackLoadPhase::RetiringDisplacedStore => self.pending(),
            WindowConfigPackLoadPhase::Complete => WindowConfigPackLoadStep::Complete,
        }
    }

    fn reject_stale(&mut self) -> WindowConfigPackLoadStep {
        self.reject(WindowConfigPackLoadDiagnostic::Stale)
    }

    fn install(&mut self, partitions: &mut BTreeMap<String, WindowConfigPartition<O>>, registry_lifetime: u64) -> WindowConfigPackLoadStep {
        if self.phase != WindowConfigPackLoadPhase::Ready || self.registry_lifetime != registry_lifetime {
            return self.reject_stale();
        }
        let window_id = self.window_id.as_ref().expect("ready window config load retains its address");
        let live_generation = partitions.get(window_id).map(|partition| partition.store.generation());
        if live_generation != self.partition_generation {
            return self.reject_stale();
        }
        let candidate = self.candidate.take().expect("ready window config candidate remains");
        *self.displaced = partitions.insert(window_id.clone(), candidate);
        self.committed = true;
        self.phase = WindowConfigPackLoadPhase::RetiringDisplacedStore;
        self.pending()
    }

    fn close_partition(partition: &mut WindowConfigPartition<O>, grant: WindowConfigPackLoadGrant) -> Result<PluginCloseStep, String> {
        let disposer = partition.disposer.as_mut().ok_or_else(|| "retained window config partition lost its disposer".to_string())?;
        disposer.close_step(&mut partition.store, grant.maximum_items.min(1), grant.maximum_bytes).map_err(|fault| fault.message)
    }

    fn close_metadata(&mut self, grant: WindowConfigPackLoadGrant) -> Option<(usize, usize)> {
        for value in [&mut *self.expected_id, &mut *self.window_kind_id, &mut *self.window_id] {
            if value.as_ref().is_some_and(|text| !text.is_empty()) {
                if grant.maximum_bytes == 0 {
                    return None;
                }
                let released = Self::truncate_string(value.as_mut().expect("retained metadata remains"), grant.maximum_bytes);
                return (released != 0).then_some((0, released));
            }
            if value.is_some() {
                if grant.maximum_items != 0 {
                    *value = None;
                    return Some((1, 0));
                }
            }
        }
        None
    }

    fn close_inner(&mut self, grant: WindowConfigPackLoadGrant) -> Result<PluginCloseStep, String> {
        if self.terminal {
            return Ok(PluginCloseStep::Complete);
        }
        if grant.maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.diagnostic.get_or_insert(WindowConfigPackLoadDiagnostic::Cancelled);
        if let Some(decoder) = self.state_decode.as_mut() {
            let step = decoder.close_step(grant)?;
            if step == PluginCloseStep::Complete {
                if !decoder.terminal_is_empty() {
                    return Err("retained window config state decoder reported false terminal".into());
                }
                self.state_decode.take();
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(decoder) = self.history_decode.as_mut() {
            let history = decoder.take_partial();
            let auxiliary = decoder.take_auxiliary_owners();
            if !decoder.terminal_is_empty() {
                return Err("retained window config history decoder retained untransferred owners".into());
            }
            self.history_decode.take();
            *self.active = Some(store::retirement::owned_retirement((history, auxiliary)));
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(hydration) = self.hydration.as_mut() {
            return match store::ErasedSnapshotRetirement::close_step(hydration, grant.maximum_items.min(1), grant.maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if store::ErasedSnapshotRetirement::terminal_is_empty(hydration) => {
                    self.hydration.take();
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("retained window config hydration reported false terminal".into()),
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }),
            };
        }
        match self.drive_active(grant) {
            Ok(true) => return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(false) => {}
            Err(_) => return Err("retained window config nested retirement failed".into()),
        }
        if let Some(partition) = self.displaced.as_mut() {
            let step = Self::close_partition(partition, grant)?;
            if step == PluginCloseStep::Complete {
                let disposer = partition.disposer.as_ref().ok_or_else(|| "retained displaced window config lost its disposer".to_string())?;
                if !disposer.terminal_is_empty(&partition.store) {
                    return Err("retained displaced window config reported false terminal".into());
                }
                partition.disposer = None;
                self.displaced.take();
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(partition) = self.candidate.as_mut() {
            let step = Self::close_partition(partition, grant)?;
            if step == PluginCloseStep::Complete {
                let disposer = partition.disposer.as_ref().ok_or_else(|| "retained rejected window config lost its disposer".to_string())?;
                if !disposer.terminal_is_empty(&partition.store) {
                    return Err("retained rejected window config reported false terminal".into());
                }
                partition.disposer = None;
                self.candidate.take();
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(state) = self.current.take().or_else(|| self.validation.take()).or_else(|| self.initial.take()) {
            let owners = self.owners.as_ref().ok_or_else(|| "retained typed window config lost its owner catalog".to_string())?;
            *self.active = Some(owners.retire_initial_snapshot_owned(state));
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(history) = self.history.take() {
            *self.active = Some(store::retirement::owned_retirement(history));
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some((released_items, released_bytes)) = self.retire_files(grant) {
            return Ok(PluginCloseStep::Pending { released_items, released_bytes });
        }
        if let Some(owners) = self.owners.as_mut() {
            return match owners.close_uninstalled_owners_step(grant.maximum_items.min(1))? {
                store::SnapshotRetirementStep::Complete if owners.uninstalled_owners_terminal_is_empty() => {
                    self.owners.take();
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("retained window config owner catalog reported false terminal".into()),
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }),
            };
        }
        if let Some((released_items, released_bytes)) = self.close_metadata(grant) {
            return Ok(PluginCloseStep::Pending { released_items, released_bytes });
        }
        self.initial_digest = None;
        self.terminal = true;
        self.phase = WindowConfigPackLoadPhase::Complete;
        Ok(PluginCloseStep::Complete)
    }

    fn ownership_is_empty(&self) -> bool {
        self.terminal
            && self.window_id.is_none()
            && self.window_kind_id.is_none()
            && self.expected_id.is_none()
            && self.files.is_none()
            && self.state_decode.is_none()
            && self.initial.is_none()
            && self.validation.is_none()
            && self.current.is_none()
            && self.initial_digest.is_none()
            && self.history_decode.is_none()
            && self.history.is_none()
            && self.hydration.is_none()
            && self.owners.is_none()
            && self.candidate.is_none()
            && self.displaced.is_none()
            && self.active.is_none()
    }
}

impl<O: WindowConfigOwner> ErasedWindowConfigPackLoad for TypedWindowConfigPackLoad<O> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn window_id(&self) -> &str {
        self.window_id.as_deref().unwrap_or("")
    }

    fn window_kind_id(&self) -> &str {
        self.window_kind_id.as_deref().unwrap_or("")
    }

    fn registry_lifetime(&self) -> u64 {
        self.registry_lifetime
    }

    fn partition_generation(&self) -> Option<u64> {
        self.partition_generation
    }

    fn phase(&self) -> WindowConfigPackLoadPhase {
        self.phase
    }

    fn progress(&self) -> WindowConfigPackLoadProgress {
        self.progress_now()
    }

    fn diagnostic(&self) -> Option<WindowConfigPackLoadDiagnostic> {
        self.diagnostic
    }

    fn advance(&mut self, grant: WindowConfigPackLoadGrant) -> WindowConfigPackLoadStep {
        self.advance_inner(grant)
    }

    fn request_cancel(&mut self) {
        self.reject(WindowConfigPackLoadDiagnostic::Cancelled);
    }

    fn reject_stale(&mut self) -> WindowConfigPackLoadStep {
        TypedWindowConfigPackLoad::reject_stale(self)
    }

    fn close_step(&mut self, grant: WindowConfigPackLoadGrant) -> Result<PluginCloseStep, String> {
        self.close_inner(grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.ownership_is_empty()
    }
}

impl<O: WindowConfigOwner> Drop for TypedWindowConfigPackLoad<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "typed window config Pack load reached Drop before terminal-empty handoff or retirement");
    }
}

pub(super) fn begin_typed_window_config_pack_load<O: WindowConfigOwner>(registry_lifetime: u64, partition_generation: Option<u64>, pack: WindowConfigPack) -> WindowConfigPackLoad {
    WindowConfigPackLoad { inner: Box::new(TypedWindowConfigPackLoad::<O>::new(registry_lifetime, partition_generation, pack)) }
}

pub(super) fn commit_typed_window_config_pack_load<O: WindowConfigOwner>(
    registry_lifetime: u64,
    partitions: &mut BTreeMap<String, WindowConfigPartition<O>>,
    load: &mut dyn ErasedWindowConfigPackLoad,
) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic> {
    let typed = load.as_any_mut().downcast_mut::<TypedWindowConfigPackLoad<O>>().ok_or(WindowConfigPackLoadDiagnostic::Stale)?;
    Ok(typed.install(partitions, registry_lifetime))
}
