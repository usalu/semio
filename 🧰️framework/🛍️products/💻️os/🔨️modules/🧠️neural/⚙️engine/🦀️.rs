//! 🧠️ Headless neural engine: dictionary in, dictionary out.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::mem::ManuallyDrop;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// 🔮️ Test-only: production moved to `ToValue`/`FromValue` below (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass — see `📓️orderedmap-tenth-seam.md`).
#[cfg(test)]
use serde::{Deserialize, Serialize};
use protocol::value::ordered::OrderedMap;
use protocol::value::{DslValue, FromValue, Number, ToValue, ValueError};

#[path = "🧵️retirement/🦀️.rs"]
pub mod retirement;
pub use retirement::{ColdDictionaryBuilder, ColdValueOwner, ValueRetirement, ValueRetirementStep};

#[path = "🧊️cold/🦀️.rs"]
pub mod cold;
pub use cold::{ColdOwner, ColdRetire};

#[path = "📔️registry/🦀️.rs"]
pub mod registry;
pub use registry::{RegistryRetirement, SharedRegistry};

// #region 🔖️Dictionary
/// 📚️ Immutable, unordered, collision-free key-value collection. `serde` is TEST-ONLY
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass — see
/// `📓️orderedmap-tenth-seam.md`): production now routes through the hand-written `ToValue`/
/// `FromValue` below, mirroring `OrderedMap<V>`'s own `#[cfg(test)]`-gated `Serialize` in
/// `🌱️value/🗂️ordered/🦀️.rs`. All three former blockers moved off serde in this pass — the `Value`
/// enum's derive, `Neuron`'s derive, and the `serde_json::to_string(&merged)` call in the
/// evaluator's pending-extension branch (now `pack::json::to_json_string(&merged.to_value())`).
/// No `#[derive(Serialize)]` here (not even test-gated): `OrderedMap<Value>`'s own `Serialize` is
/// `#[cfg(test)]`-gated inside `replication`'s OWN compilation unit — cfg(test) never crosses a
/// crate boundary, so it stays invisible when `replication` is pulled in as this crate's ordinary
/// (non-test) dependency, even while THIS crate's tests run. The oracle `Serialize` below is
/// hand-written instead, over `Value: Serialize` (local to this crate, genuinely test-cfg'd here).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dictionary {
    pairs: OrderedMap<Value>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_schema(schema: impl Into<String>) -> Self {
        Self::new().insert(SCHEMA_KEY, Value::Atom(Atom::String(schema.into())))
    }

    /// 🧊️ Explicit synchronous dictionary construction; retained callers use immutable sharing and typed update cursors.
    pub fn insert(self, key: impl Into<String>, value: Value) -> Self {
        let mut builder = ColdDictionaryBuilder::from_dictionary(self);
        builder.insert(key.into(), value);
        builder.finish()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.pairs.get(key)
    }

    pub fn schema(&self) -> Option<&str> {
        self.get(SCHEMA_KEY).and_then(|v| v.as_atom()).and_then(|a| a.as_str())
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.pairs.keys()
    }

    /// 🔎️ Borrows ordered entries without cloning nested values.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&String, &Value)> + ExactSizeIterator {
        self.pairs.iter()
    }

    /// 📤️ Moves exact dictionary ownership into nested byte-aware retirement without cloning values.
    pub fn into_retirement(self) -> ValueRetirement { ValueRetirement::from_dictionary(self) }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn merge(&self, other: &Dictionary) -> Dictionary {
        let mut builder = ColdDictionaryBuilder::from_dictionary(self.clone());
        for (k, v) in &other.pairs {
            builder.insert(k.clone(), v.clone());
        }
        builder.finish()
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        if let Err(_retirement) = std::mem::take(&mut self.pairs).release_shared() {
            assert!(std::thread::panicking(), "final Dictionary ownership must be explicitly retired or owned by a cold boundary");
        }
    }
}

/// 🧊️ Test-only oracle mirror of the hand-written `Deserialize` below; manual (not derived) for the
/// same reason `Deserialize` is hand-written rather than delegating to `OrderedMap`'s own — see
/// `Dictionary`'s struct docstring above. Serializes each entry directly over `Value: Serialize`.
#[cfg(test)]
impl Serialize for Dictionary {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (key, value) in self.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

/// 🧊️ Cold decoding stages every replacement and partial failure in a domain-aware builder.
/// 🔮️ Test-only — production decoding routes through `FromValue` below.
#[cfg(test)]
impl<'de> Deserialize<'de> for Dictionary {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Dictionary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("a neural dictionary") }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut builder = ColdDictionaryBuilder::new();
                while let Some((key, value)) = access.next_entry::<String, Value>()? { builder.insert(key, value); }
                Ok(builder.finish())
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

/// 🔁️ First-party analog of the hand-written `Deserialize` above — routes through the same
/// `ColdDictionaryBuilder` retirement contract, never constructs `Dictionary` fields directly.
impl ToValue for Dictionary {
    fn to_value(&self) -> DslValue {
        DslValue::Object(self.iter().map(|(key, value)| (key.clone(), value.to_value())).collect())
    }
}

impl FromValue for Dictionary {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(entries) = value else { return Err(ValueError::new("expected an object for Dictionary")) };
        let mut builder = ColdDictionaryBuilder::new();
        for (key, entry) in entries {
            builder.insert(key, Value::from_value(entry)?);
        }
        Ok(builder.finish())
    }
}

/// 🔑️ Dot-separated camelCase segment path.
pub type Key = String;

/// 💎️ Atom or nested dictionary. `serde` is TEST-ONLY — see `Dictionary`'s docstring above.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(untagged))]
pub enum Value {
    Atom(Atom),
    Dictionary(Dictionary),
}

impl Value {
    pub fn null() -> Self {
        Self::Atom(Atom::Null)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Atom(Atom::Null))
    }

    pub fn as_atom(&self) -> Option<&Atom> {
        match self {
            Value::Atom(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_dictionary(&self) -> Option<&Dictionary> {
        match self {
            Value::Dictionary(d) => Some(d),
            _ => None,
        }
    }
}

/// 🔁️ Mirrors `#[serde(untagged)]` above: an object decodes as `Dictionary`, anything else as `Atom`.
impl ToValue for Value {
    fn to_value(&self) -> DslValue {
        match self {
            Value::Atom(atom) => atom.to_value(),
            Value::Dictionary(dictionary) => dictionary.to_value(),
        }
    }
}

impl FromValue for Value {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            object @ DslValue::Object(_) => Ok(Value::Dictionary(Dictionary::from_value(object)?)),
            other => Ok(Value::Atom(Atom::from_value(other)?)),
        }
    }
}

/// ⚛️ Immutable non-dictionary value. `serde` is TEST-ONLY — see `Dictionary`'s docstring above.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(untagged))]
pub enum Atom {
    Null,
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    String(String),
}

impl Atom {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Atom::Boolean(b) => Some(*b),
            Atom::Integer(i) => Some(*i != 0),
            Atom::Decimal(d) => Some(*d != 0.0),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Atom::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            Atom::Integer(i) => Some(*i as f64),
            Atom::Decimal(d) => Some(*d),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Atom::String(s) => Some(s),
            _ => None,
        }
    }
}

/// 🔁️ `DslValue::Number`'s `Int`/`UInt` variants round-trip straight to `Atom::Integer`; only a
/// `Float` still needs the whole-valued check, recovering `Integer` for a fractionless value and
/// `Decimal` otherwise — the same convention `pack::json::Number` uses.
impl ToValue for Atom {
    fn to_value(&self) -> DslValue {
        match self {
            Atom::Null => DslValue::Null,
            Atom::Boolean(b) => DslValue::Bool(*b),
            Atom::Integer(i) => DslValue::Number(Number::Int(*i)),
            Atom::Decimal(d) => DslValue::Number(Number::Float(*d)),
            Atom::String(s) => DslValue::String(s.clone()),
        }
    }
}

impl FromValue for Atom {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => Ok(Atom::Null),
            DslValue::Bool(b) => Ok(Atom::Boolean(b)),
            DslValue::Number(Number::Int(value)) => Ok(Atom::Integer(value)),
            DslValue::Number(Number::UInt(value)) => Ok(Atom::Integer(value as i64)),
            DslValue::Number(Number::Float(value)) if value.fract() == 0.0 => Ok(Atom::Integer(value as i64)),
            DslValue::Number(Number::Float(value)) => Ok(Atom::Decimal(value)),
            DslValue::String(s) => Ok(Atom::String(s)),
            DslValue::Array(_) | DslValue::Object(_) => Err(ValueError::new("expected an atom, found an array or object")),
        }
    }
}
// #endregion 🔖️Dictionary

// #region 🔖️Schema
pub const SCHEMA_KEY: &str = "$schema";

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", tag = "kind", content = "of"))]
pub enum ValueType {
    Boolean,
    Integer,
    Decimal,
    Text,
    List(Box<ValueType>),
    Schema(String),
    #[default]
    Any,
}

impl ValueType {
    pub fn id(&self) -> String {
        match self {
            ValueType::Boolean => "boolean".into(),
            ValueType::Integer => "integer".into(),
            ValueType::Decimal => "number".into(),
            ValueType::Text => "text".into(),
            ValueType::List(_) => "list".into(),
            ValueType::Schema(id) => id.clone(),
            ValueType::Any => "value".into(),
        }
    }

    pub fn matches(&self, value: &Value) -> bool {
        if value.is_null() {
            return false;
        }
        match self {
            ValueType::Any => true,
            ValueType::Boolean => value.as_atom().is_some_and(|a| matches!(a, Atom::Boolean(_))),
            ValueType::Integer => value.as_atom().is_some_and(|a| matches!(a, Atom::Integer(_))),
            ValueType::Decimal => value.as_atom().and_then(|a| a.as_f64()).is_some(),
            ValueType::Text => value.as_atom().and_then(|a| a.as_str()).is_some(),
            ValueType::List(_) => value.as_dictionary().is_some_and(|d| d.schema() == Some("list")),
            ValueType::Schema(schema) => value.as_dictionary().is_some_and(|d| d.schema() == Some(schema.as_str())),
        }
    }
}

/// 🔁️ Mirrors `#[serde(tag = "kind", content = "of")]` above (`serde` stays unconditional here —
/// `ValueType` never touches `Dictionary`/`Value`, so it was never part of the tenth-seam blocker).
impl ToValue for ValueType {
    fn to_value(&self) -> DslValue {
        match self {
            ValueType::Boolean => DslValue::object([("kind".to_string(), "boolean".to_value())]),
            ValueType::Integer => DslValue::object([("kind".to_string(), "integer".to_value())]),
            ValueType::Decimal => DslValue::object([("kind".to_string(), "decimal".to_value())]),
            ValueType::Text => DslValue::object([("kind".to_string(), "text".to_value())]),
            ValueType::List(inner) => DslValue::object([("kind".to_string(), "list".to_value()), ("of".to_string(), inner.to_value())]),
            ValueType::Schema(id) => DslValue::object([("kind".to_string(), "schema".to_value()), ("of".to_string(), id.to_value())]),
            ValueType::Any => DslValue::object([("kind".to_string(), "any".to_value())]),
        }
    }
}

impl FromValue for ValueType {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let kind = value.get("kind").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("kind"))?;
        match kind.as_str() {
            "boolean" => Ok(ValueType::Boolean),
            "integer" => Ok(ValueType::Integer),
            "decimal" => Ok(ValueType::Decimal),
            "text" => Ok(ValueType::Text),
            "list" => Ok(ValueType::List(Box::new(value.get("of").cloned().map(ValueType::from_value).transpose()?.ok_or_else(|| ValueError::new("of"))?))),
            "schema" => Ok(ValueType::Schema(value.get("of").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("of"))?)),
            "any" => Ok(ValueType::Any),
            other => Err(ValueError::new(format!("unknown ValueType kind '{other}'"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct FieldSpec {
    pub key: String,
    pub value: ValueType,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub default: Option<Value>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,
}

impl FieldSpec {
    pub fn new(key: impl Into<String>, value: ValueType) -> Self {
        Self { key: key.into(), value, default: None, label: None }
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default.replace(default).retire_cold();
        self
    }

    pub fn decimal(key: impl Into<String>) -> Self {
        Self::new(key, ValueType::Decimal)
    }

    pub fn decimal_default(key: impl Into<String>, default: f64) -> Self {
        Self::decimal(key).with_default(Value::Atom(Atom::Decimal(default)))
    }
}

impl ToValue for FieldSpec {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("key".into(), self.key.to_value()),
            ("value".into(), self.value.to_value()),
            ("default".into(), self.default.to_value()),
            ("label".into(), self.label.to_value()),
        ])
    }
}

impl FromValue for FieldSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for FieldSpec")) };
        Ok(Self {
            key: value.get("key").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("key"))?,
            value: value.get("value").cloned().map(ValueType::from_value).transpose()?.ok_or_else(|| ValueError::new("value"))?,
            default: value.get("default").cloned().map(Option::<Value>::from_value).transpose()?.unwrap_or_default(),
            label: value.get("label").cloned().map(Option::<String>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

/// 🔮️ `serde` is TEST-ONLY — see `Dictionary`'s docstring; `fields: Vec<FieldSpec>` fans in `Value` transitively.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Schema {
    pub id: String,
    pub module: String,
    pub name: String,
    pub icon: String,
    pub summary: String,
    pub fields: Vec<FieldSpec>,
}

impl ToValue for Schema {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("module".into(), self.module.to_value()),
            ("name".into(), self.name.to_value()),
            ("icon".into(), self.icon.to_value()),
            ("summary".into(), self.summary.to_value()),
            ("fields".into(), self.fields.to_value()),
        ])
    }
}

impl FromValue for Schema {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for Schema")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            module: value.get("module").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("module"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("name"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("icon"))?,
            summary: value.get("summary").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("summary"))?,
            fields: value.get("fields").cloned().map(Vec::<FieldSpec>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

/// 🏷️ Schema id with display metadata for pickers.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SchemaRef {
    pub id: String,
    pub name: String,
    pub icon: String,
}

impl ToValue for SchemaRef {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("id".into(), self.id.to_value()), ("name".into(), self.name.to_value()), ("icon".into(), self.icon.to_value())])
    }
}

impl FromValue for SchemaRef {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for SchemaRef")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("name"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("icon"))?,
        })
    }
}

impl Schema {
    pub fn validate(&self, dictionary: &Dictionary) -> Result<(), EvalError> {
        match dictionary.schema() {
            Some(id) if id == self.id => {}
            Some(id) => return Err(EvalError::InvalidInput(format!("schema {id} does not match {}", self.id))),
            None => return Err(EvalError::MissingInput(SCHEMA_KEY.into())),
        }
        for field in &self.fields {
            let Some(value) = dictionary.get(&field.key) else {
                return Err(EvalError::MissingInput(field.key.clone()));
            };
            if !field.value.matches(value) {
                return Err(EvalError::InvalidInput(format!("field {} does not match {}", field.key, field.value.id())));
            }
        }
        Ok(())
    }

    pub fn default_dictionary(&self) -> Dictionary {
        let mut dictionary = Dictionary::with_schema(self.id.clone());
        for field in &self.fields {
            if let Some(default) = &field.default {
                dictionary = dictionary.insert(field.key.clone(), default.clone());
            }
        }
        dictionary
    }
}
// #endregion 🔖️Schema

//#region ⚠️ Errors
/// 🚨️ Schema field/channel conversion, instance-read, and cardinality-parse failures.
#[derive(Clone, Debug, PartialEq)]
pub enum NeuralEngineError {
    /// 🔢️ Atom value doesn't match the scalar type a schema field declares.
    FieldTypeMismatch { kind: &'static str, reason: &'static str },
    /// 📦️ Field value isn't a dictionary where a list/schema/any field required one.
    FieldNotDictionary,
    /// 🕳️ Channel value is null where a value was required.
    ChannelNull,
    /// 📦️ Channel value isn't the wrapping dictionary its scalar type expects.
    ChannelTypeMismatch { kind: &'static str },
    /// 📦️ Channel value isn't a dictionary where a list/schema/any channel required one.
    ChannelNotDictionary,
    /// 🕳️ Channel dictionary is missing its `value` entry.
    ChannelMissingValue { kind: &'static str },
    /// 🔍️ A schema instance or field channel is absent from the input dictionary.
    Missing(String),
    /// ⚠️ A schema instance dictionary carries the wrong `$schema` tag.
    Invalid(String),
    /// 🔍️ A declared schema field is absent from the constructed dictionary.
    MissingField(String),
    /// 🚫️ Neither an instance nor any field inputs were provided to a schema component.
    NoInputProvided,
    /// 🔢️ A cardinality symbol string didn't parse to a known cardinality.
    InvalidCardinality(String),
    /// 🧬️ The constructed/modified dictionary failed schema validation.
    Validation(EvalError),
}

impl std::fmt::Display for NeuralEngineError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldTypeMismatch { kind, reason } => write!(formatter, "{kind} field is not {reason}"),
            Self::FieldNotDictionary => formatter.write_str("field is not a dictionary"),
            Self::ChannelNull => formatter.write_str("channel value is null"),
            Self::ChannelTypeMismatch { kind } => write!(formatter, "{kind} channel is not a dictionary"),
            Self::ChannelNotDictionary => formatter.write_str("channel is not a dictionary"),
            Self::ChannelMissingValue { kind } => write!(formatter, "{kind} channel is missing value"),
            Self::Missing(detail) => write!(formatter, "missing {detail}"),
            Self::Invalid(detail) => write!(formatter, "invalid {detail}"),
            Self::MissingField(field) => write!(formatter, "missing field {field}"),
            Self::NoInputProvided => formatter.write_str("no instance or field inputs provided"),
            Self::InvalidCardinality(value) => write!(formatter, "invalid cardinality: {value}"),
            Self::Validation(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for NeuralEngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(error) => Some(error),
            _ => None,
        }
    }
}

impl From<EvalError> for NeuralEngineError {
    fn from(error: EvalError) -> Self {
        Self::Validation(error)
    }
}
//#endregion ⚠️ Errors

// #region 🔖️SchemaComponent
fn schema_component_operator_id(schema: &Schema) -> String {
    format!("{}.{}", schema.module, schema.id)
}

fn should_auto_register_schema_component(schema: &Schema) -> bool {
    schema.module != "core" && !schema.fields.is_empty() && schema.id != "list" && schema.id != "dictionary"
}

fn schema_field_input_cardinality(value: &ValueType) -> Cardinality {
    match value {
        ValueType::List(_) => Cardinality::ZeroOrMore,
        _ => Cardinality::ZeroOrOne,
    }
}

fn schema_field_output_cardinality(value: &ValueType) -> Cardinality {
    match value {
        ValueType::List(_) => Cardinality::ZeroOrMore,
        _ => Cardinality::ExactlyOne,
    }
}

fn field_channel_operators(value: &ValueType) -> Vec<String> {
    match value {
        ValueType::List(inner) => vec![inner.id()],
        _ => vec![value.id()],
    }
}

/// 🧩️ Builds construct/deconstruct/modify operator metadata for a schema.
pub fn schema_component_info(schema: &Schema) -> OperatorInfo {
    let operator_id = schema_component_operator_id(schema);
    let mut inputs = vec![ChannelSpec::requires(&schema.id, &[schema.id.as_str()]).with_cardinality(Cardinality::ZeroOrOne)];
    for field in &schema.fields {
        let operators = field_channel_operators(&field.value);
        inputs.push(ChannelSpec::requires(&field.key, &operators).with_cardinality(schema_field_input_cardinality(&field.value)));
    }
    let mut outputs = vec![ChannelSpec::provides(&schema.id, vec![schema.id.clone()])];
    for field in &schema.fields {
        let (code, abbreviation, full_name) = derive_channel_names(&field.key);
        outputs.push(ChannelSpec::named(code, abbreviation, &field.key, full_name).with_operators(field_channel_operators(&field.value)).with_cardinality(schema_field_output_cardinality(&field.value)));
    }
    outputs.push(ChannelSpec::list_output("errors", vec![]));
    OperatorInfo {
        id: operator_id,
        extension: schema.module.clone(),
        name: schema.name.clone(),
        abbreviation: schema.name.clone(),
        icon: schema.icon.clone(),
        summary: format!("Constructs, deconstructs, or modifies {}", schema.name),
        inputs,
        outputs,
        group: vec!["Schemas".into()],
        ..Default::default()
    }
}

fn schema_errors_list(messages: &[String]) -> Dictionary {
    let mut list = Dictionary::with_schema("list");
    for (index, message) in messages.iter().enumerate() {
        list = list.insert(index.to_string(), Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(message.clone())))));
    }
    list
}

fn schema_input_present(input: &Dictionary, key: &str) -> bool {
    input.get(key).is_some_and(|value| !value.is_null())
}

fn field_to_channel(value: &Value, value_type: &ValueType) -> Result<Value, NeuralEngineError> {
    match value_type {
        ValueType::Decimal => {
            let number = value.as_atom().and_then(|atom| atom.as_f64()).ok_or(NeuralEngineError::FieldTypeMismatch { kind: "decimal", reason: "numeric" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(number)))))
        }
        ValueType::Integer => {
            let number = value
                .as_atom()
                .and_then(|atom| match atom {
                    Atom::Integer(value) => Some(*value),
                    Atom::Decimal(value) => Some(value.round() as i64),
                    _ => None,
                })
                .ok_or(NeuralEngineError::FieldTypeMismatch { kind: "integer", reason: "integral" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Integer(number)))))
        }
        ValueType::Boolean => {
            let boolean = value.as_atom().and_then(|atom| atom.as_bool()).ok_or(NeuralEngineError::FieldTypeMismatch { kind: "boolean", reason: "boolean" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(boolean)))))
        }
        ValueType::Text => {
            let text = value.as_atom().and_then(|atom| atom.as_str()).ok_or(NeuralEngineError::FieldTypeMismatch { kind: "text", reason: "text" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(text.to_string())))))
        }
        ValueType::List(_) | ValueType::Schema(_) | ValueType::Any => value.as_dictionary().cloned().map(Value::Dictionary).ok_or(NeuralEngineError::FieldNotDictionary),
    }
}

fn channel_to_field(value: &Value, value_type: &ValueType) -> Result<Value, NeuralEngineError> {
    if value.is_null() {
        return Err(NeuralEngineError::ChannelNull);
    }
    match value_type {
        ValueType::Decimal => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "decimal" })?;
            let number = dictionary.get("value").and_then(|entry| entry.as_atom()).and_then(|atom| atom.as_f64()).ok_or(NeuralEngineError::ChannelMissingValue { kind: "decimal" })?;
            Ok(Value::Atom(Atom::Decimal(number)))
        }
        ValueType::Integer => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "integer" })?;
            let number = dictionary
                .get("value")
                .and_then(|entry| entry.as_atom())
                .and_then(|atom| match atom {
                    Atom::Integer(value) => Some(*value),
                    Atom::Decimal(value) => Some(value.round() as i64),
                    _ => None,
                })
                .ok_or(NeuralEngineError::ChannelMissingValue { kind: "integer" })?;
            Ok(Value::Atom(Atom::Integer(number)))
        }
        ValueType::Boolean => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "boolean" })?;
            let boolean = dictionary.get("value").and_then(|entry| entry.as_atom()).and_then(|atom| atom.as_bool()).ok_or(NeuralEngineError::ChannelMissingValue { kind: "boolean" })?;
            Ok(Value::Atom(Atom::Boolean(boolean)))
        }
        ValueType::Text => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "text" })?;
            let text = dictionary.get("value").and_then(|entry| entry.as_atom()).and_then(|atom| atom.as_str()).ok_or(NeuralEngineError::ChannelMissingValue { kind: "text" })?;
            Ok(Value::Atom(Atom::String(text.to_string())))
        }
        ValueType::List(_) | ValueType::Schema(_) | ValueType::Any => value.as_dictionary().cloned().map(Value::Dictionary).ok_or(NeuralEngineError::ChannelNotDictionary),
    }
}

fn read_schema_instance<'a>(input: &'a Dictionary, schema: &Schema) -> Result<&'a Dictionary, NeuralEngineError> {
    let instance = input.get(&schema.id).and_then(|value| value.as_dictionary()).ok_or_else(|| NeuralEngineError::Missing(schema.id.clone()))?;
    if instance.schema() != Some(schema.id.as_str()) {
        return Err(NeuralEngineError::Invalid(schema.id.clone()));
    }
    Ok(instance)
}

fn read_schema_field_input(input: &Dictionary, field: &FieldSpec) -> Result<Value, NeuralEngineError> {
    let value = input.get(&field.key).ok_or_else(|| NeuralEngineError::Missing(field.key.clone()))?;
    channel_to_field(value, &field.value)
}

/// 🧩️ Construct, deconstruct, or modify dictionaries for one schema.
pub struct SchemaComponent {
    pub schema: Schema,
}

impl SchemaComponent {
    fn construct(&self, input: &Dictionary, provided: &[&FieldSpec]) -> Result<Dictionary, NeuralEngineError> {
        let mut dictionary = ColdDictionaryBuilder::from_dictionary(self.schema.default_dictionary());
        for field in provided {
            dictionary.insert(field.key.clone(), read_schema_field_input(input, field)?);
        }
        for field in &self.schema.fields {
            if dictionary.dictionary().get(&field.key).is_none() {
                return Err(NeuralEngineError::MissingField(field.key.clone()));
            }
        }
        self.schema.validate(dictionary.dictionary())?;
        Ok(dictionary.finish())
    }

    fn deconstruct(&self, input: &Dictionary) -> Result<Dictionary, NeuralEngineError> {
        let instance = read_schema_instance(input, &self.schema)?;
        self.schema.validate(instance)?;
        Ok(instance.clone())
    }

    fn modify(&self, input: &Dictionary, provided: &[&FieldSpec]) -> Result<Dictionary, NeuralEngineError> {
        let mut dictionary = ColdDictionaryBuilder::from_dictionary(read_schema_instance(input, &self.schema)?.clone());
        for field in provided {
            dictionary.insert(field.key.clone(), read_schema_field_input(input, field)?);
        }
        self.schema.validate(dictionary.dictionary())?;
        Ok(dictionary.finish())
    }

    fn success_output(&self, instance: &Dictionary) -> Result<Dictionary, NeuralEngineError> {
        let mut output = ColdDictionaryBuilder::from_dictionary(Dictionary::new().insert(self.schema.id.clone(), Value::Dictionary(instance.clone())));
        for field in &self.schema.fields {
            // 🛡️ `instance` only reaches here after `Schema::validate` confirmed every
            // declared field.key is present, so this lookup can never miss.
            let value = instance.get(&field.key).expect("validated field");
            let channel = field_to_channel(value, &field.value)?;
            output.insert(field.key.clone(), channel);
        }
        output.insert("errors".into(), Value::Dictionary(schema_errors_list(&[])));
        Ok(output.finish())
    }

    fn error_output(&self, messages: &[String]) -> Dictionary {
        let mut output = Dictionary::new().insert(self.schema.id.clone(), Value::null()).insert("errors", Value::Dictionary(schema_errors_list(messages)));
        for field in &self.schema.fields {
            output = output.insert(field.key.clone(), Value::null());
        }
        output
    }
}

impl Operator for SchemaComponent {
        fn retire_cold(self: Box<Self>) { self.schema.retire_cold(); }

    fn retirement_is_empty(&self) -> bool {
        self.schema.id.is_empty() && self.schema.module.is_empty() && self.schema.name.is_empty() && self.schema.icon.is_empty() && self.schema.summary.is_empty() && self.schema.fields.is_empty()
    }

    fn retire_step(&mut self, maximum_items: usize, maximum_bytes: usize, values: &mut ValueRetirement) -> Result<ValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(ValueRetirementStep::Blocked); }
        if self.retirement_is_empty() { return Ok(ValueRetirementStep::Complete); }
        values.push_schema(std::mem::take(&mut self.schema));
        Ok(ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let has_instance = schema_input_present(input, &self.schema.id);
        let provided: Vec<&FieldSpec> = self.schema.fields.iter().filter(|field| schema_input_present(input, &field.key)).collect();
        let has_fields = !provided.is_empty();
        let result = match (has_instance, has_fields) {
            (false, false) => Err(NeuralEngineError::NoInputProvided),
            (false, true) => self.construct(input, &provided),
            (true, false) => self.deconstruct(input),
            (true, true) => self.modify(input, &provided),
        };
        Ok(match result.and_then(|instance| self.success_output(&ColdOwner::new(instance))) {
            Ok(output) => output,
            Err(error) => self.error_output(&[error.to_string()]),
        })
    }
}
// #endregion 🔖️SchemaComponent

// #region 🔖️Tree
/// 🌳️ Directed acyclic graph of neurons and synapses. `serde` is TEST-ONLY — see `Dictionary`'s
/// docstring; mutually recursive with `Neuron` via `Neuron.tree: Option<Box<Tree>>`, so both moved
/// off serde together.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct Tree {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
}

/// 🔵️ Neuron instance bound to a kind. `serde` is TEST-ONLY — see `Tree`'s docstring above.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct Neuron {
    pub id: String,
    pub kind: String,
    #[cfg_attr(test, serde(default))]
    pub params: Dictionary,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub tree: Option<Box<Tree>>,
}

impl Neuron {
    pub fn with_kind(id: impl Into<String>, kind: impl Into<String>, params: Dictionary) -> Self {
        Self { id: id.into(), kind: kind.into(), params, tree: None }
    }
}

impl Tree {
    /// 📜️ Derives contract input and output channels from boundary neurons.
    pub fn contract(&self) -> (Vec<ChannelSpec>, Vec<ChannelSpec>) {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for neuron in &self.neurons {
            let (channel_id, operators) = contract_channel(neuron);
            if neuron.kind == INPUT_KIND {
                inputs.push(ChannelSpec::requires(channel_id, &operators));
            } else if neuron.kind == OUTPUT_KIND {
                outputs.push(ChannelSpec::provides(channel_id, operators));
            }
        }
        (inputs, outputs)
    }
}

fn default_from_port() -> String {
    String::new()
}

fn default_to_port() -> String {
    String::new()
}

/// 🔗️ Directed connection between two port endpoints.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Synapse {
    pub id: String,
    pub from: String,
    pub to: String,
    #[cfg_attr(test, serde(default = "default_from_port"))]
    pub from_port: String,
    #[cfg_attr(test, serde(default = "default_to_port"))]
    pub to_port: String,
}

impl ToValue for Synapse {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("from".into(), self.from.to_value()),
            ("to".into(), self.to.to_value()),
            ("fromPort".into(), self.from_port.to_value()),
            ("toPort".into(), self.to_port.to_value()),
        ])
    }
}

impl FromValue for Synapse {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for Synapse")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            from: value.get("from").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("from"))?,
            to: value.get("to").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("to"))?,
            from_port: value.get("fromPort").cloned().map(String::from_value).transpose()?.unwrap_or_else(default_from_port),
            to_port: value.get("toPort").cloned().map(String::from_value).transpose()?.unwrap_or_else(default_to_port),
        })
    }
}

impl ToValue for Neuron {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("kind".into(), self.kind.to_value()),
            ("params".into(), self.params.to_value()),
            ("tree".into(), self.tree.to_value()),
        ])
    }
}

impl FromValue for Neuron {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for Neuron")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            kind: value.get("kind").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("kind"))?,
            params: value.get("params").cloned().map(Dictionary::from_value).transpose()?.unwrap_or_default(),
            tree: value.get("tree").cloned().map(Option::<Box<Tree>>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

impl ToValue for Tree {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("neurons".into(), self.neurons.to_value()), ("synapses".into(), self.synapses.to_value())])
    }
}

impl FromValue for Tree {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for Tree")) };
        Ok(Self {
            neurons: value.get("neurons").cloned().map(Vec::<Neuron>::from_value).transpose()?.unwrap_or_default(),
            synapses: value.get("synapses").cloned().map(Vec::<Synapse>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}
// #endregion 🔖️Tree

// #region 🔖️Contract
pub const INPUT_KIND: &str = "input";
pub const OUTPUT_KIND: &str = "output";
pub const CLUSTER_KIND: &str = "cluster";

fn contract_channel(neuron: &Neuron) -> (String, Vec<String>) {
    let channel_id = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
    let operators = neuron.params.get("operators").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(|raw| raw.split(',').map(str::trim).filter(|entry| !entry.is_empty()).map(str::to_string).collect()).unwrap_or_default();
    (channel_id, operators)
}

/// 🧩️ Builds operator metadata for a cluster neuron from its inner contract.
pub fn cluster_operator_info(id: &str, name: &str, tree: &Tree) -> OperatorInfo {
    let (inputs, outputs) = tree.contract();
    OperatorInfo { id: id.into(), extension: "flow".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:🧩️".into(), summary: "Nested tree operator".into(), inputs, outputs, ..Default::default() }
}
// #endregion 🔖️Contract

// #region 🔖️OperatorRecord
/// ⚙️ Eval error from an operator.
#[derive(Clone, Debug, PartialEq)]
pub enum EvalError {
    UnknownKind(String),
    MissingInput(String),
    InvalidInput(String),
    CardinalityViolation(String),
    HeterogeneousList(String),
    CycleDetected,
    PendingExtension { extension_id: String, operator_id: String, node_hash: u64 },
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::UnknownKind(k) => write!(f, "unknown kind: {k}"),
            EvalError::MissingInput(k) => write!(f, "missing input: {k}"),
            EvalError::InvalidInput(m) => write!(f, "invalid input: {m}"),
            EvalError::CardinalityViolation(m) => write!(f, "cardinality violation: {m}"),
            EvalError::HeterogeneousList(m) => write!(f, "heterogeneous list: {m}"),
            EvalError::CycleDetected => write!(f, "cycle detected"),
            EvalError::PendingExtension { extension_id, operator_id, .. } => write!(f, "pending extension {extension_id} operator {operator_id}"),
        }
    }
}

impl std::error::Error for EvalError {}

/// 📈️ Monotone progress of one [`OperatorJob`]. `units_done` never decreases; `units_total` is the
/// plan known so far and may be revised upward by a job whose later stages are only plannable once
/// an earlier one has run (a boolean cannot count its result's validation units before it has a
/// result). `phase` is the job's own domain vocabulary — the framework never interprets it, it only
/// carries it to the surface that labels it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OperatorProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub phase: &'static str,
}

/// ⏱️ Outcome of one budgeted [`OperatorJob::step`].
#[derive(Clone, Debug, PartialEq)]
pub enum OperatorJobStep {
    /// 🔁 Budget spent, work remains — call `step` again.
    Working(OperatorProgress),
    /// ✅ Terminal: the operator's out dictionary.
    Done(Dictionary),
    /// 🛑 Terminal: [`OperatorJob::cancel`] retired the job; nothing is produced.
    Cancelled(OperatorProgress),
}

/// ⏱️ A single operator evaluation split into budgetable units so a host can drive it across many
/// turns inside an interactive step ceiling, paint progress, and cancel it — the operator twin of
/// the kernel's resumable tessellation job.
///
/// Domain-NEUTRAL by construction: the framework knows only units, a phase tag and the three
/// outcomes. Which units an operator has (a face pair, a validation entity, a solver iteration) is
/// entirely the domain extension's business, and an operator that has no sub-structure simply does
/// not offer a job (see [`Operator::step_plan`]'s default) and is evaluated in one call as before
/// (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
pub trait OperatorJob: Send {
    /// ⏱️ Advances by at most `budget` units. A `budget` of zero is a legal progress probe.
    fn step(&mut self, budget: usize) -> Result<OperatorJobStep, EvalError>;
    /// 📈️ Progress right now — safe to read between steps and after termination.
    fn progress(&self) -> OperatorProgress;
    /// 🛑️ Retires the job at the next observable boundary. A job that already produced its output
    /// is never retired: supersession may only stop work still in flight.
    fn cancel(&mut self);
}

/// 🧮️ Computational unit: one dictionary to another.
pub trait Operator: Send + Sync {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError>;
    /// ⏱️ The budgeted, resumable form of this operator's evaluation, when it has one. `None` (the
    /// default, and the answer for every operator whose cost is microseconds) means "evaluate me in
    /// one call". An operator that answers `Some` MUST produce, through its job, exactly what
    /// [`Operator::evaluate`] would have produced for the same input — the stepped path IS the
    /// algorithm, never a second implementation to drift from.
    fn step_plan(&self, _input: &Dictionary) -> Result<Option<Box<dyn OperatorJob>>, EvalError> {
        Ok(None)
    }
    /// 🪶️ Only compiler-proven trivial operators are terminal without domain-specific field retirement.
    fn retirement_is_empty(&self) -> bool { !std::mem::needs_drop::<Self>() }
    /// 🧹️ Transfers or retires one granted domain frontier while the caller retains the operator itself.
    fn retire_step(&mut self, maximum_items: usize, maximum_bytes: usize, _values: &mut ValueRetirement) -> Result<ValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(ValueRetirementStep::Blocked); }
        if self.retirement_is_empty() { Ok(ValueRetirementStep::Complete) } else { Err("neural.operator-retirement-not-implemented") }
    }
    /// 🧊️ Explicit cold registry teardown; implementations owning neural domains retire those fields here.
    fn retire_cold(self: Box<Self>) { drop(self); }
}

// #region 🔖️Cardinality
/// 🔢️ Channel multiplicity: exactly one, optional, or homogeneous list collections.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Cardinality {
    #[default]
    ExactlyOne,
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Exactly(usize),
}

impl Cardinality {
    pub fn symbol(&self) -> String {
        match self {
            Self::ExactlyOne => "!".into(),
            Self::ZeroOrOne => "?".into(),
            Self::ZeroOrMore => "*".into(),
            Self::OneOrMore => "+".into(),
            Self::Exactly(count) => count.to_string(),
        }
    }

    pub fn from_symbol(raw: &str) -> Result<Self, NeuralEngineError> {
        match raw.trim() {
            "!" => Ok(Self::ExactlyOne),
            "?" => Ok(Self::ZeroOrOne),
            "*" => Ok(Self::ZeroOrMore),
            "+" => Ok(Self::OneOrMore),
            digits if digits.chars().all(|ch| ch.is_ascii_digit()) && !digits.is_empty() => digits.parse::<usize>().map(Self::Exactly).map_err(|_| NeuralEngineError::InvalidCardinality(raw.to_string())),
            other => Err(NeuralEngineError::InvalidCardinality(other.to_string())),
        }
    }

    pub fn is_collection(&self) -> bool {
        match self {
            Self::ZeroOrMore | Self::OneOrMore => true,
            Self::Exactly(count) => *count != 1,
            _ => false,
        }
    }

    pub fn accepts(&self, count: usize) -> bool {
        match self {
            Self::ExactlyOne => count == 1,
            Self::ZeroOrOne => count <= 1,
            Self::ZeroOrMore => true,
            Self::OneOrMore => count >= 1,
            Self::Exactly(expected) => count == *expected,
        }
    }

    pub fn count_range(&self) -> (usize, Option<usize>) {
        match self {
            Self::ExactlyOne => (1, Some(1)),
            Self::ZeroOrOne => (0, Some(1)),
            Self::ZeroOrMore => (0, None),
            Self::OneOrMore => (1, None),
            Self::Exactly(count) => (*count, Some(*count)),
        }
    }

    pub fn range_contains(&self, other: &Self) -> bool {
        let (min, max) = self.count_range();
        let (other_min, other_max) = other.count_range();
        if other_min < min {
            return false;
        }
        match (max, other_max) {
            (Some(limit), Some(other_limit)) => other_limit <= limit,
            (Some(_limit), None) => false,
            (None, Some(_other_limit)) => other_min >= min,
            (None, None) => true,
        }
    }
}

/// 🔮️ Test-only — `ChannelSpec.cardinality` was the only production caller and moved to `ToValue`/
/// `FromValue` below (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass).
#[cfg(test)]
impl Serialize for Cardinality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.symbol())
    }
}

#[cfg(test)]
impl<'de> Deserialize<'de> for Cardinality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::from_symbol(&raw).map_err(serde::de::Error::custom)
    }
}

impl ToValue for Cardinality {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.symbol())
    }
}

impl FromValue for Cardinality {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::String(raw) = value else { return Err(ValueError::new("expected a string for Cardinality")) };
        Self::from_symbol(&raw).map_err(|error| ValueError::new(error.to_string()))
    }
}
// #endregion 🔖️Cardinality

/// ➕️ Variadic input or output slot specification. `serde` is TEST-ONLY — see `Dictionary`'s docstring.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct VariadicSpec {
    pub slot_key: String,
    pub min: usize,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub max: Option<usize>,
}

impl ToValue for VariadicSpec {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("slotKey".into(), self.slot_key.to_value()), ("min".into(), self.min.to_value()), ("max".into(), self.max.to_value())])
    }
}

impl FromValue for VariadicSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for VariadicSpec")) };
        Ok(Self {
            slot_key: value.get("slotKey").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("slotKey"))?,
            min: value.get("min").cloned().map(usize::from_value).transpose()?.ok_or_else(|| ValueError::new("min"))?,
            max: value.get("max").cloned().map(Option::<usize>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

/// 🔌️ Declared operator channel with required/provided operator capabilities. `serde` is TEST-ONLY
/// — see `Dictionary`'s docstring; `default: Option<Value>` fans in `Value` transitively.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ChannelSpec {
    pub code: String,
    pub abbreviation: String,
    pub name: String,
    pub full_name: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub operators: Vec<String>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub default: Option<Value>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default))]
    pub cardinality: Cardinality,
}

impl ToValue for ChannelSpec {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("code".into(), self.code.to_value()),
            ("abbreviation".into(), self.abbreviation.to_value()),
            ("name".into(), self.name.to_value()),
            ("fullName".into(), self.full_name.to_value()),
            ("operators".into(), self.operators.to_value()),
            ("default".into(), self.default.to_value()),
            ("label".into(), self.label.to_value()),
            ("cardinality".into(), self.cardinality.to_value()),
        ])
    }
}

impl FromValue for ChannelSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for ChannelSpec")) };
        Ok(Self {
            code: value.get("code").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("code"))?,
            abbreviation: value.get("abbreviation").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("abbreviation"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("name"))?,
            full_name: value.get("fullName").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("fullName"))?,
            operators: value.get("operators").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
            default: value.get("default").cloned().map(Option::<Value>::from_value).transpose()?.unwrap_or_default(),
            label: value.get("label").cloned().map(Option::<String>::from_value).transpose()?.unwrap_or_default(),
            cardinality: value.get("cardinality").cloned().map(Cardinality::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

fn derive_channel_names(name: &str) -> (String, String, String) {
    let code = if name.len() <= 2 { name.to_uppercase() } else { name.chars().take(2).collect::<String>().to_uppercase() };
    let abbreviation = if name.len() <= 3 { name.to_string() } else { name.chars().take(3).collect() };
    let mut full = String::new();
    let mut capitalize = true;
    for ch in name.chars() {
        if ch == '_' {
            capitalize = true;
            continue;
        }
        if capitalize {
            full.extend(ch.to_uppercase());
            capitalize = false;
        } else {
            full.push(ch);
        }
    }
    if full.is_empty() {
        full = name.to_string();
    }
    (code, abbreviation, full)
}

impl ChannelSpec {
    pub fn named(code: impl Into<String>, abbreviation: impl Into<String>, name: impl Into<String>, full_name: impl Into<String>) -> Self {
        Self { code: code.into(), abbreviation: abbreviation.into(), name: name.into(), full_name: full_name.into(), operators: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn requires(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        let name = name.into();
        let (code, abbreviation, full_name) = derive_channel_names(&name);
        Self { code, abbreviation, name, full_name, operators: operators.iter().map(|entry| entry.as_ref().to_string()).collect(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn provides(name: impl Into<String>, operators: Vec<String>) -> Self {
        let name = name.into();
        let (code, abbreviation, full_name) = derive_channel_names(&name);
        Self { code, abbreviation, name, full_name, operators, default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn with_operators(mut self, operators: Vec<String>) -> Self {
        self.operators = operators;
        self
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default.replace(default).retire_cold();
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_cardinality(mut self, cardinality: Cardinality) -> Self {
        self.cardinality = cardinality;
        self
    }

    pub fn number(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators)
    }

    pub fn number_default(name: impl Into<String>, default: f64, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_default(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(default)))))
    }

    pub fn integer_default(name: impl Into<String>, default: i64, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_default(Value::Atom(Atom::Integer(default)))
    }

    pub fn boolean_default(name: impl Into<String>, default: bool, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_default(Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(default)))))
    }

    pub fn text_default(name: impl Into<String>, default: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_default(Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(default.into())))))
    }

    pub fn list(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_cardinality(Cardinality::ZeroOrMore)
    }

    pub fn list_output(name: impl Into<String>, operators: Vec<String>) -> Self {
        Self::provides(name, operators).with_cardinality(Cardinality::ZeroOrMore)
    }

    pub fn dictionary(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators)
    }

    pub fn any(name: impl Into<String>) -> Self {
        Self::requires(name, &[] as &[&str])
    }

    pub fn wildcard() -> Self {
        Self::any("*")
    }
}

/// 📤️ Wraps a payload dictionary under a named output channel.
pub fn channel_output(name: &str, payload: Dictionary) -> Dictionary {
    Dictionary::new().insert(name, Value::Dictionary(payload))
}

/// 📇️ Catalogue metadata for an operator. `serde` is TEST-ONLY — see `Dictionary`'s docstring;
/// `inputs`/`outputs: Vec<ChannelSpec>` fan in `Value` transitively.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct OperatorInfo {
    pub id: String,
    pub extension: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    pub inputs: Vec<ChannelSpec>,
    pub outputs: Vec<ChannelSpec>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub variadic_input: Option<VariadicSpec>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub variadic_output: Option<VariadicSpec>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub group: Vec<String>,
}

impl ToValue for OperatorInfo {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("extension".into(), self.extension.to_value()),
            ("name".into(), self.name.to_value()),
            ("abbreviation".into(), self.abbreviation.to_value()),
            ("icon".into(), self.icon.to_value()),
            ("summary".into(), self.summary.to_value()),
            ("inputs".into(), self.inputs.to_value()),
            ("outputs".into(), self.outputs.to_value()),
            ("variadicInput".into(), self.variadic_input.to_value()),
            ("variadicOutput".into(), self.variadic_output.to_value()),
            ("group".into(), self.group.to_value()),
        ])
    }
}

impl FromValue for OperatorInfo {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new("expected an object for OperatorInfo")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("id"))?,
            extension: value.get("extension").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("extension"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("name"))?,
            abbreviation: value.get("abbreviation").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("abbreviation"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("icon"))?,
            summary: value.get("summary").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new("summary"))?,
            inputs: value.get("inputs").cloned().map(Vec::<ChannelSpec>::from_value).transpose()?.unwrap_or_default(),
            outputs: value.get("outputs").cloned().map(Vec::<ChannelSpec>::from_value).transpose()?.unwrap_or_default(),
            variadic_input: value.get("variadicInput").cloned().map(Option::<VariadicSpec>::from_value).transpose()?.unwrap_or_default(),
            variadic_output: value.get("variadicOutput").cloned().map(Option::<VariadicSpec>::from_value).transpose()?.unwrap_or_default(),
            group: value.get("group").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

pub struct OperatorImpl {
    pub schemas: Vec<String>,
    pub operator: Box<dyn Operator>,
}

pub struct OperatorRecord {
    pub info: OperatorInfo,
    pub implementations: Vec<OperatorImpl>,
}

/// 📋️ Registry of schemas and operators by id.
#[derive(Default)]
pub struct Registry {
    schemas: BTreeMap<String, Schema>,
    operators: BTreeMap<String, OperatorRecord>,
    operator_produces: BTreeMap<String, Vec<String>>,
    schema_providers: BTreeMap<String, BTreeSet<String>>,
    finalized: bool,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_schema(&mut self, schema: Schema) {
        self.schemas.insert(schema.id.clone(), schema).retire_cold();
        self.finalized = false;
    }

    pub fn register_operator(&mut self, info: OperatorInfo, implementations: Vec<OperatorImpl>, produces: &[&str]) {
        let id = info.id.clone();
        for schema in produces {
            self.schema_providers.entry(schema.to_string()).or_default();
        }
        for implementation in &implementations {
            for schema in &implementation.schemas {
                self.schema_providers.entry(schema.clone()).or_default().insert(id.clone());
            }
        }
        self.operator_produces.insert(id.clone(), produces.iter().map(|entry| (*entry).to_string()).collect());
        self.operators.insert(id, OperatorRecord { info, implementations }).retire_cold();
        self.finalized = false;
    }

    pub fn finalize(&mut self) {
        if self.finalized {
            return;
        }
        let schema_ids: Vec<String> = self.schemas.keys().cloned().collect();
        for schema_id in schema_ids {
            let Some(schema) = self.schemas.get(&schema_id).cloned().map(ColdOwner::new) else { continue };
            if !should_auto_register_schema_component(&schema) {
                continue;
            }
            let operator_id = schema_component_operator_id(&schema);
            if self.operators.contains_key(&operator_id) {
                continue;
            }
            let info = schema_component_info(&schema);
            let produces = vec![schema.id.clone()];
            for produced in &produces {
                self.schema_providers.entry(produced.clone()).or_default();
            }
            self.schema_providers.entry(schema.id.clone()).or_default().insert(operator_id.clone());
            self.operator_produces.insert(operator_id.clone(), produces);
            self.operators.insert(operator_id, OperatorRecord { info, implementations: vec![OperatorImpl { schemas: vec![], operator: Box::new(SchemaComponent { schema: schema.into_inner() }) }] }).retire_cold();
        }
        let operator_produces = self.operator_produces.clone();
        let schema_providers = self.schema_providers.clone();
        for (operator_id, operator) in &mut self.operators {
            let produces = operator_produces.get(operator_id).cloned().unwrap_or_default();
            for channel in &mut operator.info.outputs {
                if !channel.operators.is_empty() {
                    continue;
                }
                let mut provided = HashSet::new();
                for schema in &produces {
                    if let Some(providers) = schema_providers.get(schema) {
                        provided.extend(providers.iter().cloned());
                    }
                }
                let mut operators: Vec<String> = provided.into_iter().collect();
                operators.sort();
                channel.operators = operators;
            }
        }
        self.finalized = true;
    }

    pub fn operators_for_schema(&self, schema_id: &str) -> Vec<String> {
        let mut operators: Vec<String> = self.schema_providers.get(schema_id).map(|entries| entries.iter().cloned().collect()).unwrap_or_default();
        operators.sort();
        operators
    }

    pub fn channel_compatible(output: &ChannelSpec, input: &ChannelSpec) -> bool {
        if input.operators.is_empty() {
            return true;
        }
        input.operators.iter().all(|required| output.operators.iter().any(|provided| provided == required))
    }

    pub fn schema(&self, schema_id: &str) -> Option<&Schema> {
        self.schemas.get(schema_id)
    }

    pub fn operator(&self, operator_id: &str) -> Option<&OperatorRecord> {
        self.operators.get(operator_id)
    }

    pub fn operator_info(&self, operator_id: &str) -> Option<&OperatorInfo> {
        self.operators.get(operator_id).map(|entry| &entry.info)
    }

    /// 📚️ Borrows current metadata in registry order without acquiring nested default-value owners.
    pub fn operator_infos(&self) -> impl DoubleEndedIterator<Item = &OperatorInfo> + ExactSizeIterator {
        self.operators.values().map(|entry| &entry.info)
    }

    pub fn schema_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.schemas.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// 🏷️ Lightweight schema metadata for pickers and catalogues.
    pub fn schema_refs(&self) -> Vec<SchemaRef> {
        self.schema_ids().into_iter().filter_map(|id| self.schemas.get(&id).map(|schema| SchemaRef { id: schema.id.clone(), name: schema.name.clone(), icon: schema.icon.clone() })).collect()
    }

    pub fn schema_catalogue(&self) -> Vec<Schema> {
        let mut items: Vec<Schema> = self.schemas.values().cloned().collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }

    pub fn operator_catalogue(&self) -> Vec<OperatorInfo> {
        let mut items: Vec<OperatorInfo> = self.operators.values().map(|entry| Self::finalize_operator_info(&entry.info, self.operator_produces.get(&entry.info.id).map(Vec::as_slice), &self.schema_providers)).collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }

    fn finalize_operator_info(info: &OperatorInfo, produces: Option<&[String]>, schema_providers: &BTreeMap<String, BTreeSet<String>>) -> OperatorInfo {
        let mut finalized = info.clone();
        let produces = produces.unwrap_or(&[]);
        for channel in &mut finalized.outputs {
            if !channel.operators.is_empty() {
                continue;
            }
            let mut provided = HashSet::new();
            for schema in produces {
                if let Some(providers) = schema_providers.get(schema) {
                    provided.extend(providers.iter().cloned());
                }
            }
            let mut operators: Vec<String> = provided.into_iter().collect();
            operators.sort();
            channel.operators = operators;
        }
        finalized
    }

    pub fn dispatch(&self, operator_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let operator = self.operator(operator_id).ok_or_else(|| EvalError::UnknownKind(operator_id.into()))?;
        validate_neuron_inputs(input, Some(&operator.info))?;
        let signature = operator_signature(&operator.info, input);
        let implementation = operator
            .implementations
            .iter()
            .find(|implementation| implementation.schemas == signature)
            .or_else(|| operator.implementations.iter().find(|implementation| implementation.schemas.is_empty()))
            .ok_or_else(|| EvalError::InvalidInput(format!("no implementation for {operator_id}({})", signature.join(", "))))?;
        let output = ColdOwner::new(implementation.operator.evaluate(input)?);
        validate_operator_outputs(&operator.info, &output)?;
        Ok(output.into_inner())
    }

    /// ⏱️ The budgeted form of [`Registry::dispatch`]: resolves the same operator and
    /// implementation, validates the same inputs, and asks the implementation for a resumable job.
    /// `Ok(None)` means this operator has no sub-structure and the caller should `dispatch` it in
    /// one call. The job's `Done` dictionary still has to pass [`validate_operator_outputs`], which
    /// is why [`Registry::finish_job`] — not the caller — closes it.
    // 🚫️async: E1 pure registry lookup mirroring `dispatch` (no I/O) — see R9
    pub fn dispatch_job(&self, operator_id: &str, input: &Dictionary) -> Result<Option<Box<dyn OperatorJob>>, EvalError> {
        let operator = self.operator(operator_id).ok_or_else(|| EvalError::UnknownKind(operator_id.into()))?;
        validate_neuron_inputs(input, Some(&operator.info))?;
        let signature = operator_signature(&operator.info, input);
        let implementation = operator
            .implementations
            .iter()
            .find(|implementation| implementation.schemas == signature)
            .or_else(|| operator.implementations.iter().find(|implementation| implementation.schemas.is_empty()))
            .ok_or_else(|| EvalError::InvalidInput(format!("no implementation for {operator_id}({})", signature.join(", "))))?;
        implementation.operator.step_plan(input)
    }

    /// ✅️ Holds a finished job's output to the SAME output contract [`Registry::dispatch`] holds a
    /// one-shot evaluation to, so a stepped answer and a one-shot answer are indistinguishable
    /// downstream.
    // 🚫️async: E1 pure registry lookup (no I/O) — see R9
    pub fn finish_job(&self, operator_id: &str, output: Dictionary) -> Result<Dictionary, EvalError> {
        let operator = self.operator(operator_id).ok_or_else(|| EvalError::UnknownKind(operator_id.into()))?;
        let output = ColdOwner::new(output);
        validate_operator_outputs(&operator.info, &output)?;
        Ok(output.into_inner())
    }
}
// #endregion 🔖️OperatorRecord

// #region 🔖️Cache
fn hash_str<H: Hasher>(hasher: &mut H, value: &str) {
    value.hash(hasher);
}

fn hash_atom<H: Hasher>(hasher: &mut H, atom: &Atom) {
    match atom {
        Atom::Null => 0u8.hash(hasher),
        Atom::Boolean(value) => value.hash(hasher),
        Atom::Integer(value) => value.hash(hasher),
        Atom::Decimal(value) => value.to_bits().hash(hasher),
        Atom::String(value) => hash_str(hasher, value),
    }
}

fn hash_value<H: Hasher>(hasher: &mut H, value: &Value) {
    match value {
        Value::Atom(atom) => hash_atom(hasher, atom),
        Value::Dictionary(dict) => {
            0u8.hash(hasher);
            hash_dictionary(hasher, dict);
        }
    }
}

fn hash_dictionary<H: Hasher>(hasher: &mut H, dictionary: &Dictionary) {
    for (key, value) in &dictionary.pairs {
        hash_str(hasher, key);
        hash_value(hasher, value);
    }
}

/// 🔑️ Content-addressable cache key from operator kind and resolved input dictionary.
pub fn node_hash(kind: &str, input: &Dictionary) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hash_str(&mut hasher, kind);
    hash_dictionary(&mut hasher, input);
    hasher.finish()
}

/// 🧠️ Epoch-bounded in-process cache for DAG node outputs.
#[derive(Default)]
pub struct NeuralCache {
    entries: ManuallyDrop<Mutex<BTreeMap<u64, (u64, Dictionary)>>>,
    retirement: ManuallyDrop<Mutex<ValueRetirement>>,
    epoch: AtomicU64,
}

struct NeuralCacheRetirementState {
    cache: Option<std::sync::Arc<NeuralCache>>,
    entries: BTreeMap<u64, (u64, Dictionary)>,
    retirement: ValueRetirement,
    terminal: bool,
}

/// 🧹️ Exact cache-root handoff and byte-aware nested retirement; u64 tree metadata has fixed machine-width height.
pub struct NeuralCacheRetirement { state: ManuallyDrop<NeuralCacheRetirementState> }

impl NeuralCacheRetirement {
    pub fn new(cache: std::sync::Arc<NeuralCache>) -> Self {
        Self { state: ManuallyDrop::new(NeuralCacheRetirementState { cache: Some(cache), entries: BTreeMap::new(), retirement: ValueRetirement::default(), terminal: false }) }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> ValueRetirementStep {
        if maximum_items == 0 || maximum_bytes == 0 { return ValueRetirementStep::Blocked; }
        let state = &mut *self.state;
        if !state.retirement.terminal_is_empty() { return state.retirement.close_step(maximum_items, maximum_bytes); }
        if let Some(cache) = state.cache.take() {
            if let Some(mut cache) = std::sync::Arc::into_inner(cache) {
                state.entries = std::mem::take(cache.entries.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner));
                state.retirement = std::mem::take(cache.retirement.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner));
            }
            return ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some((_, (_, value))) = state.entries.pop_first() {
            state.retirement.push_dictionary(value);
            return ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 };
        }
        state.terminal = true;
        ValueRetirementStep::Complete
    }

    pub fn terminal_nonopaque_is_empty(&self) -> bool {
        self.state.terminal && self.state.cache.is_none() && self.state.entries.is_empty() && self.state.retirement.terminal_is_empty()
    }
}

impl Drop for NeuralCacheRetirement {
    fn drop(&mut self) {
        if !self.terminal_nonopaque_is_empty() { assert!(std::thread::panicking(), "NeuralCacheRetirement must reach terminal-empty before release"); return; }
        unsafe { ManuallyDrop::drop(&mut self.state); }
    }
}

impl Drop for NeuralCache {
    fn drop(&mut self) {
        let empty = self.entries.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty()
            && self.retirement.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).terminal_is_empty();
        if !empty { assert!(std::thread::panicking(), "final NeuralCache must be explicitly retired"); return; }
        unsafe { ManuallyDrop::drop(&mut self.entries); ManuallyDrop::drop(&mut self.retirement); }
    }
}

impl NeuralCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_epoch(&self) {
        self.epoch.fetch_add(1, Ordering::Relaxed);
    }

    pub fn current_epoch(&self) -> u64 {
        self.epoch.load(Ordering::Relaxed)
    }

    pub fn len(&self) -> usize {
        self.entries.lock().map_or(0, |entries| entries.len())
    }

    pub fn is_empty(&self) -> bool {
        self.entries.lock().map_or(true, |entries| entries.is_empty())
    }

    /// 🔎️ Whether `key` has a cached entry (from any epoch) — a hit here means
    /// [`NeuralCache::get_or_insert_with`] would return without calling `compute`.
    pub fn contains(&self, key: u64) -> bool {
        let epoch = self.epoch.load(Ordering::Relaxed);
        if let Ok(mut entries) = self.entries.lock() {
            if let Some(entry) = entries.get_mut(&key) {
                entry.0 = epoch;
                return true;
            }
        }
        false
    }

    /// 🌱️ Pre-seeds a node output (host-mediated extension eval) so the next budgeted pass hits the cache.
    pub fn seed(&self, key: u64, value: Dictionary) {
        let epoch = self.epoch.load(Ordering::Relaxed);
        let displaced = self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(key, (epoch, value));
        if let Some((_, value)) = displaced { self.retirement.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push_dictionary(value); }
    }

    pub fn get(&self, key: u64) -> Option<Dictionary> {
        let epoch = self.epoch.load(Ordering::Relaxed);
        if let Ok(mut entries) = self.entries.lock() {
            if let Some(entry) = entries.get_mut(&key) {
                entry.0 = epoch;
                return Some(entry.1.clone());
            }
        }
        None
    }

    pub fn get_or_insert_with<F>(&self, key: u64, compute: F) -> Result<Dictionary, EvalError>
    where
        F: FnOnce() -> Result<Dictionary, EvalError>,
    {
        let epoch = self.epoch.load(Ordering::Relaxed);
        if let Ok(mut entries) = self.entries.lock() {
            if let Some(entry) = entries.get_mut(&key) {
                entry.0 = epoch;
                return Ok(entry.1.clone());
            }
        }
        let value = compute()?;
        self.seed(key, value.clone());
        Ok(value)
    }

    pub fn sweep(&self) {
        let epoch = self.epoch.load(Ordering::Relaxed);
        let mut entries = self.entries.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let expired: Vec<_> = entries.iter().filter(|(_, (entry_epoch, _))| *entry_epoch != epoch).map(|(key, _)| *key).collect();
        let mut retirement = self.retirement.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for key in expired { if let Some((_, value)) = entries.remove(&key) { retirement.push_dictionary(value); } }
    }
}

fn eval_error_dictionary(err: &EvalError) -> Dictionary {
    Dictionary::new().insert("error", Value::Atom(Atom::String(err.to_string())))
}

fn evaluate_cached_output<F>(cache: &NeuralCache, kind: &str, merged: &Dictionary, dispatch: F) -> Dictionary
where
    F: FnOnce() -> Result<Dictionary, EvalError>,
{
    let key = node_hash(kind, merged);
    match cache.get_or_insert_with(key, dispatch) {
        Ok(dict) => dict,
        Err(err) => eval_error_dictionary(&err),
    }
}
// #endregion 🔖️Cache

// #region 🔖️DirtyPropagation
fn hash_neuron_key<H: Hasher>(hasher: &mut H, neuron: &Neuron) {
    hash_str(hasher, &neuron.kind);
    hash_dictionary(hasher, &neuron.params);
    if let Some(sub_tree) = neuron.tree.as_deref() {
        1u8.hash(hasher);
        hash_subtree(hasher, sub_tree);
    } else {
        0u8.hash(hasher);
    }
}

fn hash_subtree<H: Hasher>(hasher: &mut H, tree: &Tree) {
    let mut neurons: Vec<&Neuron> = tree.neurons.iter().collect();
    neurons.sort_by(|a, b| a.id.cmp(&b.id));
    for neuron in neurons {
        hash_str(hasher, &neuron.id);
        hash_neuron_key(hasher, neuron);
    }
    let mut synapses: Vec<&Synapse> = tree.synapses.iter().collect();
    synapses.sort_by(|a, b| (&a.from, &a.from_port, &a.to, &a.to_port).cmp(&(&b.from, &b.from_port, &b.to, &b.to_port)));
    for syn in synapses {
        hash_str(hasher, &syn.from);
        hash_str(hasher, &syn.from_port);
        hash_str(hasher, &syn.to);
        hash_str(hasher, &syn.to_port);
    }
}

fn neuron_key_hash(neuron: &Neuron) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hash_neuron_key(&mut hasher, neuron);
    hasher.finish()
}

fn incoming_edges_signature(tree: &Tree, neuron_id: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let mut incoming: Vec<&Synapse> = tree.synapses.iter().filter(|syn| syn.to == neuron_id).collect();
    incoming.sort_by(|a, b| (&a.from, &a.from_port, &a.to_port).cmp(&(&b.from, &b.from_port, &b.to_port)));
    for syn in incoming {
        hash_str(&mut hasher, &syn.from);
        hash_str(&mut hasher, &syn.from_port);
        hash_str(&mut hasher, &syn.to_port);
    }
    hasher.finish()
}

/// 🧬️ Per-neuron structural/adjacency fingerprint, keyed once by id in [`TreeSnapshot`] instead
/// of duplicated across parallel maps — cuts id clones on [`TreeSnapshot::capture`] from four
/// per neuron down to one.
#[derive(Clone, Debug, Default, PartialEq)]
struct NeuronSnapshot {
    key: u64,
    incoming: u64,
    /// `[to, ...]` — who reads this neuron's output, used for forward dirty propagation.
    dependents: Vec<String>,
}

/// 📸️ Structural fingerprint of a tree+seeds pair, used by [`compute_dirty_set`] to diff two
/// evaluations without re-hashing or re-walking neurons that provably didn't change.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TreeSnapshot {
    neurons: BTreeMap<String, NeuronSnapshot>,
    seed_keys: BTreeMap<String, u64>,
}

impl TreeSnapshot {
    pub fn capture(tree: &Tree, seeds: &HashMap<String, Dictionary>) -> Self {
        let mut neurons: BTreeMap<String, NeuronSnapshot> = tree.neurons.iter().map(|neuron| (neuron.id.clone(), NeuronSnapshot { key: neuron_key_hash(neuron), incoming: incoming_edges_signature(tree, &neuron.id), dependents: Vec::new() })).collect();
        for syn in &tree.synapses {
            if !neurons.contains_key(&syn.to) {
                continue;
            }
            if let Some(source) = neurons.get_mut(&syn.from) {
                source.dependents.push(syn.to.clone());
            }
        }
        let mut seed_keys = BTreeMap::new();
        for (id, dict) in seeds {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            hash_dictionary(&mut hasher, dict);
            seed_keys.insert(id.clone(), hasher.finish());
        }
        Self { neurons, seed_keys }
    }
}

/// 🧭️ Forward-propagates dirtiness from directly-changed neurons to all descendants.
///
/// `previous == None` means "first evaluation ever" — everything is dirty. Otherwise a neuron
/// is directly dirty if it's new, its structural key (kind/params/subtree) changed, its incoming
/// synapse set changed, or its seed value changed; a surviving dependent of a *removed* neuron
/// (looked up via `previous`'s adjacency, since removed neurons vanish from `current`) is also
/// directly dirty. Every neuron reachable from the directly-dirty set via `current`'s `from -> to`
/// adjacency is dirty too — everything else is provably unaffected.
pub fn compute_dirty_set(previous: Option<&TreeSnapshot>, current: &TreeSnapshot) -> HashSet<String> {
    let Some(previous) = previous else {
        return current.neurons.keys().cloned().collect();
    };
    let mut direct: HashSet<String> = HashSet::new();
    for (id, snapshot) in &current.neurons {
        let prev = previous.neurons.get(id);
        let is_new = prev.is_none();
        let structurally_changed = prev.map(|p| p.key) != Some(snapshot.key);
        let rewired = prev.map(|p| p.incoming) != Some(snapshot.incoming);
        if is_new || structurally_changed || rewired {
            direct.insert(id.clone());
        }
    }
    let mut seed_ids: HashSet<&String> = previous.seed_keys.keys().collect();
    seed_ids.extend(current.seed_keys.keys());
    for id in seed_ids {
        if current.neurons.contains_key(id) && previous.seed_keys.get(id) != current.seed_keys.get(id) {
            direct.insert(id.clone());
        }
    }
    for (removed_id, removed_snapshot) in &previous.neurons {
        if current.neurons.contains_key(removed_id) {
            continue;
        }
        for dependent in &removed_snapshot.dependents {
            if current.neurons.contains_key(dependent) {
                direct.insert(dependent.clone());
            }
        }
    }
    let mut dirty: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = direct.into_iter().collect();
    while let Some(id) = queue.pop_front() {
        if !dirty.insert(id.clone()) {
            continue;
        }
        if let Some(snapshot) = current.neurons.get(&id) {
            for dep in &snapshot.dependents {
                if !dirty.contains(dep) {
                    queue.push_back(dep.clone());
                }
            }
        }
    }
    dirty
}
// #endregion 🔖️DirtyPropagation

// #region 🔖️Evaluator
/// ⏳️ Topo-ordered neuron ids still needing work when a budgeted walk stops at `from_index`.
fn budgeted_remaining_from(order: &[String], from_index: usize, dirty: &HashSet<String>) -> Vec<String> {
    order[from_index..].iter().filter(|id| dirty.is_empty() || dirty.contains(*id)).cloned().collect()
}

/// 📡️ Resolved neuron inputs and outputs from one evaluation pass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvalChannels {
    pub outputs: BTreeMap<String, Dictionary>,
    pub inputs: BTreeMap<String, Dictionary>,
}

/// ⏳️ Result of a budget-limited evaluation pass — `remaining` (in topo order) is empty once the
/// whole dirty set has been walked; a non-empty `remaining` means resume with another budgeted call.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetedEval {
    pub channels: EvalChannels,
    pub remaining: Vec<String>,
    pub pending_extension: Option<PendingExtensionEval>,
}

/// ⏱️ A wall-clock guard a budgeted walk consults BETWEEN neurons. Carried as a plain `fn` pointer
/// rather than a clock trait or an `Instant`: this crate is a leaf evaluator with no dependency on
/// the tracing module that owns the process clock, and `std::time::Instant` is not usable on every
/// wasm target the guest builds for.
#[derive(Clone, Copy, Debug)]
pub struct EvalStepDeadline {
    pub now_us: fn() -> Option<u64>,
    pub deadline_us: u64,
}

impl PartialEq for EvalStepDeadline {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::fn_addr_eq(self.now_us, other.now_us) && self.deadline_us == other.deadline_us
    }
}

impl EvalStepDeadline {
    /// ⌛️ Whether the walk has run past its wall-clock allowance. A clock that answers nothing (bare
    /// wasm with no host clock installed) never expires the walk — the node count stays the only cap.
    fn expired(&self) -> bool {
        (self.now_us)().is_some_and(|now_us| now_us >= self.deadline_us)
    }
}

/// ⏳️ What ONE budgeted walk may spend. `dispatches` is the historical cache-missed-neuron count;
/// `deadline` is the preemption point a NODE count alone cannot provide — one expensive operator
/// (a `brep.bool.fuse` on a complex solid) runs to completion inside a single non-preemptible
/// reactor step regardless of how few nodes it is, which is what parked the browser's main thread
/// for 3.5-18.4 s per hop (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️audit-guest-tick-cost-2026-09-12.md` §1.3). The deadline is consulted only AFTER at least one
/// dispatch, so a walk that starts already over budget still makes progress instead of re-arming
/// forever.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvalStepBudget {
    pub dispatches: usize,
    pub deadline: Option<EvalStepDeadline>,
}

impl EvalStepBudget {
    /// 🔍️ A pure probe: dispatches nothing, reports every neuron that would still need work.
    pub const PROBE: EvalStepBudget = EvalStepBudget { dispatches: 0, deadline: None };
    /// ♾️ Walks the whole dirty set in one call, however long it takes.
    pub const UNBOUNDED: EvalStepBudget = EvalStepBudget { dispatches: usize::MAX, deadline: None };

    /// 🔢️ A node-count-only budget — no wall-clock preemption.
    pub const fn dispatches(dispatches: usize) -> EvalStepBudget {
        EvalStepBudget { dispatches, deadline: None }
    }

    /// ⏱️ The same node count, preempted at `deadline_us` on `now_us`'s clock.
    pub const fn until(dispatches: usize, now_us: fn() -> Option<u64>, deadline_us: u64) -> EvalStepBudget {
        EvalStepBudget { dispatches, deadline: Some(EvalStepDeadline { now_us, deadline_us }) }
    }

    /// 🛑️ Whether the walk must yield before dispatching another cache-missed neuron.
    fn exhausted(&self, spent: usize) -> bool {
        spent >= self.dispatches || (spent != 0 && self.deadline.is_some_and(|deadline| deadline.expired()))
    }
}

/// ⏳️ One contributed operator that must be evaluated in its owning plugin before the graph can resume.
#[derive(Clone, Debug, PartialEq)]
pub struct PendingExtensionEval {
    pub extension_id: String,
    pub operator_id: String,
    pub node_hash: u64,
    pub input_json: String,
}

/// 🔄️ Topological evaluation over a neural tree.
pub struct Evaluator<'a> {
    registry: &'a Registry,
}

impl<'a> Evaluator<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    pub fn evaluate(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>) -> Result<BTreeMap<String, Dictionary>, EvalError> {
        let EvalChannels { outputs, inputs } = self.evaluate_channels(tree, seeds, &HashMap::new())?;
        inputs.retire_cold(); Ok(outputs)
    }

    pub fn evaluate_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
    ) -> Result<BTreeMap<String, Dictionary>, EvalError> {
        let EvalChannels { outputs, inputs } = self.evaluate_channels_with(tree, seeds, operator_infos, dispatch)?;
        inputs.retire_cold(); Ok(outputs)
    }

    pub fn evaluate_channels(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>, operator_infos: &HashMap<String, OperatorInfo>) -> Result<EvalChannels, EvalError> {
        self.evaluate_channels_with(tree, seeds, operator_infos, &|kind, input| self.registry.dispatch(kind, input))
    }

    pub fn evaluate_channels_sequential_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
    ) -> Result<EvalChannels, EvalError> {
        let cache = ColdOwner::new(NeuralCache::new());
        cache.begin_epoch();
        let result = self.evaluate_channels_sequential_cached(tree, seeds, operator_infos, dispatch, &cache, &HashSet::new(), None);
        cache.sweep();
        result
    }

    #[allow(clippy::too_many_arguments, reason = "incremental cache eval needs tree+seeds+infos+dispatch+cache+dirty+previous together; splitting into a params struct would ripple into flow/core/rs call sites outside this ticket's scope")]
    pub fn evaluate_channels_sequential_cached(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
        cache: &NeuralCache,
        dirty: &HashSet<String>,
        previous: Option<&EvalChannels>,
    ) -> Result<EvalChannels, EvalError> {
        self.evaluate_channels_budgeted(tree, seeds, operator_infos, dispatch, cache, dirty, previous, EvalStepBudget::UNBOUNDED).map(|budgeted| budgeted.channels)
    }

    /// ⏳️ Sequential topo walk that stops after computing `budget.dispatches` cache-missed (i.e.
    /// actually dispatched) neurons OR once `budget.deadline` has passed, returning the
    /// not-yet-computed neuron ids as `remaining` so a caller can resume with another budgeted call
    /// — used to spread a heavy evaluation across many cheap ticks instead of blocking a thread for
    /// the whole graph. [`EvalStepBudget::PROBE`] is a pure probe: nothing is dispatched,
    /// `remaining` reports every neuron that would still need work.
    #[allow(clippy::too_many_arguments, reason = "mirrors evaluate_channels_sequential_cached's params plus a budget; see that method's reason")]
    pub fn evaluate_channels_budgeted(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
        cache: &NeuralCache,
        dirty: &HashSet<String>,
        previous: Option<&EvalChannels>,
        budget: EvalStepBudget,
    ) -> Result<BudgetedEval, EvalError> {
        let order = topo_order(tree)?;
        let mut outputs = ColdOwner::new(seeds.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<String, Dictionary>>());
        let mut inputs = ColdOwner::new(BTreeMap::<String, Dictionary>::new());
        let mut spent = 0usize;
        for (index, neuron_id) in order.iter().enumerate() {
            if !dirty.contains(neuron_id) {
                if let Some(prev) = previous {
                    if let (Some(out), Some(inp)) = (prev.outputs.get(neuron_id), prev.inputs.get(neuron_id)) {
                        outputs.insert(neuron_id.clone(), out.clone()).retire_cold();
                        inputs.insert(neuron_id.clone(), inp.clone()).retire_cold();
                        continue;
                    }
                }
            }
            let neuron = tree.neurons.iter().find(|n| n.id == *neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
            let operator_info = operator_info_for_neuron(neuron, operator_infos, self.registry.operator_info(&neuron.kind));
            let input = ColdOwner::new(collect_neuron_input(tree, &outputs, neuron_id, operator_info)?);
            inputs.insert(neuron_id.clone(), input.clone()).retire_cold();
            if let Some(seed) = seeds.get(neuron_id) {
                outputs.insert(neuron_id.clone(), seed.clone()).retire_cold();
                continue;
            }
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                outputs.insert(neuron_id.clone(), input.merge(&neuron.params)).retire_cold();
                continue;
            }
            // 🚧️ A budget-exhausted cache miss (cluster or operator) stops the walk here; this
            // neuron and everything from `order[index..]` becomes `remaining`. Exhaustion is either
            // the dispatch count or the wall-clock deadline — see [`EvalStepBudget`]. Clusters have
            // no single cache key of their own (their inner neurons are cached individually), so a
            // cluster is conservatively always charged as a miss.
            if let Some(sub_tree) = neuron.tree.as_deref() {
                if budget.exhausted(spent) {
                    return Ok(BudgetedEval { channels: EvalChannels { outputs: outputs.into_inner(), inputs: inputs.into_inner() }, remaining: budgeted_remaining_from(&order, index, dirty), pending_extension: None });
                }
                let out = self.evaluate_cluster_sequential(sub_tree, &input, operator_infos, dispatch, cache)?;
                outputs.insert(neuron_id.clone(), out).retire_cold();
                spent += 1;
                continue;
            }
            let merged = ColdOwner::new(input.merge(&neuron.params));
            let key = node_hash(&neuron.kind, &merged);
            let is_miss = !cache.contains(key);
            if is_miss && budget.exhausted(spent) {
                return Ok(BudgetedEval { channels: EvalChannels { outputs: outputs.into_inner(), inputs: inputs.into_inner() }, remaining: budgeted_remaining_from(&order, index, dirty), pending_extension: None });
            }
            let out = if let Some(cached) = cache.get(key) {
                cached
            } else {
                match dispatch(&neuron.kind, &merged) {
                    Err(EvalError::PendingExtension { extension_id, operator_id, node_hash }) => {
                        let input_json = pack::json::to_json_string(&*merged);
                        return Ok(BudgetedEval {
                            channels: EvalChannels { outputs: outputs.into_inner(), inputs: inputs.into_inner() },
                            remaining: budgeted_remaining_from(&order, index, dirty),
                            pending_extension: Some(PendingExtensionEval { extension_id, operator_id, node_hash, input_json }),
                        });
                    }
                    Err(err) => eval_error_dictionary(&err),
                    Ok(dict) => {
                        cache.seed(key, dict.clone());
                        dict
                    }
                }
            };
            outputs.insert(neuron_id.clone(), out).retire_cold();
            if is_miss {
                spent += 1;
            }
        }
        Ok(BudgetedEval { channels: EvalChannels { outputs: outputs.into_inner(), inputs: inputs.into_inner() }, remaining: Vec::new(), pending_extension: None })
    }

    pub fn evaluate_channels_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
    ) -> Result<EvalChannels, EvalError> {
        let cache = ColdOwner::new(NeuralCache::new());
        cache.begin_epoch();
        let result = self.evaluate_channels_cached(tree, seeds, operator_infos, dispatch, &cache, &HashSet::new(), None);
        cache.sweep();
        result
    }

    #[allow(clippy::too_many_arguments, reason = "incremental cache eval needs tree+seeds+infos+dispatch+cache+dirty+previous together; splitting into a params struct would ripple into flow/core/rs call sites outside this ticket's scope")]
    pub fn evaluate_channels_cached(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
        cache: &NeuralCache,
        dirty: &HashSet<String>,
        previous: Option<&EvalChannels>,
    ) -> Result<EvalChannels, EvalError> {
        let levels = topo_levels(tree)?;
        let mut outputs = ColdOwner::new(seeds.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<String, Dictionary>>());
        let mut inputs = ColdOwner::new(BTreeMap::<String, Dictionary>::new());
        for level in levels {
            let mut level_inputs = ColdOwner::new(BTreeMap::<String, Dictionary>::new());
            let mut level_outputs = ColdOwner::new(BTreeMap::<String, Dictionary>::new());
            let mut deferred_clusters = ColdOwner::new(Vec::<(String, Tree, Dictionary)>::new());
            let mut compute_jobs = ColdOwner::new(Vec::<(String, String, Dictionary)>::new());

            for neuron_id in &level {
                if !dirty.contains(neuron_id) {
                    if let Some(prev) = previous {
                        if let (Some(out), Some(inp)) = (prev.outputs.get(neuron_id), prev.inputs.get(neuron_id)) {
                            level_outputs.insert(neuron_id.clone(), out.clone()).retire_cold();
                            level_inputs.insert(neuron_id.clone(), inp.clone()).retire_cold();
                            continue;
                        }
                    }
                }
                let neuron = tree.neurons.iter().find(|n| n.id == *neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
                let operator_info = operator_info_for_neuron(neuron, operator_infos, self.registry.operator_info(&neuron.kind));
                let input = ColdOwner::new(collect_neuron_input(tree, &outputs, neuron_id, operator_info)?);
                level_inputs.insert(neuron_id.clone(), input.clone()).retire_cold();
                if let Some(seed) = seeds.get(neuron_id) {
                    level_outputs.insert(neuron_id.clone(), seed.clone()).retire_cold();
                    continue;
                }
                if let Some(sub_tree) = neuron.tree.as_deref().cloned() {
                    deferred_clusters.push((neuron_id.clone(), sub_tree, input.into_inner()));
                    continue;
                }
                if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                    level_outputs.insert(neuron_id.clone(), input.merge(&neuron.params)).retire_cold();
                    continue;
                }
                compute_jobs.push((neuron_id.clone(), neuron.kind.clone(), input.merge(&neuron.params)));
            }

            for (neuron_id, kind, merged) in compute_jobs.into_inner() {
                let merged = ColdOwner::new(merged);
                let out = evaluate_cached_output(cache, &kind, &merged, || dispatch(&kind, &merged));
                level_outputs.insert(neuron_id, out).retire_cold();
            }

            for (neuron_id, sub_tree, input) in deferred_clusters.into_inner() {
                let sub_tree = ColdOwner::new(sub_tree); let input = ColdOwner::new(input);
                let out = self.evaluate_cluster(&sub_tree, &input, operator_infos, dispatch, cache)?;
                level_outputs.insert(neuron_id, out).retire_cold();
            }

            for (key, value) in level_inputs.into_inner() { inputs.insert(key, value).retire_cold(); }
            for (key, value) in level_outputs.into_inner() { outputs.insert(key, value).retire_cold(); }
        }
        Ok(EvalChannels { outputs: outputs.into_inner(), inputs: inputs.into_inner() })
    }

    /// 🧮️ Evaluates a tree as a function: in dictionary to out dictionary via boundary neurons.
    pub fn evaluate_function(&self, tree: &Tree, in_dict: &Dictionary) -> Result<Dictionary, EvalError> {
        self.evaluate_function_with(tree, in_dict, &HashMap::new(), &|kind, input| self.registry.dispatch(kind, input))
    }

    /// 🧮️ Evaluates a tree as a function with custom dispatch and operator metadata.
    pub fn evaluate_function_with(&self, tree: &Tree, in_dict: &Dictionary, operator_infos: &HashMap<String, OperatorInfo>, dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync)) -> Result<Dictionary, EvalError> {
        let seeds = ColdOwner::new(seed_input_boundaries(tree, in_dict));
        let channels = ColdOwner::new(self.evaluate_channels_with(tree, &seeds, operator_infos, dispatch)?);
        collect_output_boundaries(tree, &channels)
    }

    /// 🧮️ Evaluates a tree as a function with caching and custom dispatch.
    pub fn evaluate_function_cached(
        &self,
        tree: &Tree,
        in_dict: &Dictionary,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
        cache: &NeuralCache,
    ) -> Result<Dictionary, EvalError> {
        let seeds = ColdOwner::new(seed_input_boundaries(tree, in_dict));
        let channels = ColdOwner::new(self.evaluate_channels_cached(tree, &seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?);
        collect_output_boundaries(tree, &channels)
    }

    fn evaluate_cluster_sequential(
        &self,
        sub_tree: &Tree,
        parent_input: &Dictionary,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
        cache: &NeuralCache,
    ) -> Result<Dictionary, EvalError> {
        let sub_seeds = ColdOwner::new(seed_input_boundaries(sub_tree, parent_input));
        let sub_channels = ColdOwner::new(self.evaluate_channels_sequential_cached(sub_tree, &sub_seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?);
        collect_output_boundaries(sub_tree, &sub_channels)
    }

    fn evaluate_cluster(
        &self,
        sub_tree: &Tree,
        parent_input: &Dictionary,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
        cache: &NeuralCache,
    ) -> Result<Dictionary, EvalError> {
        let sub_seeds = ColdOwner::new(seed_input_boundaries(sub_tree, parent_input));
        // 🧩️ Nested cluster subtrees are evaluated atomically (v1 limitation): any change inside
        // re-evaluates the whole cluster rather than propagating dirtiness within it. Never stale,
        // just not maximally incremental for nested clusters.
        let sub_channels = ColdOwner::new(self.evaluate_channels_cached(sub_tree, &sub_seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?);
        collect_output_boundaries(sub_tree, &sub_channels)
    }
}

fn operator_info_for_neuron<'a>(neuron: &Neuron, operator_infos: &'a HashMap<String, OperatorInfo>, registry_info: Option<&'a OperatorInfo>) -> Option<&'a OperatorInfo> {
    if neuron.tree.is_some() {
        return None;
    }
    operator_infos.get(&neuron.kind).or(registry_info)
}

fn boundary_seed_dictionary(value: &Value) -> Dictionary {
    match value {
        Value::Dictionary(dict) => dict.clone(),
        other => Dictionary::new().insert("value", other.clone()),
    }
}

fn boundary_output_value(input: &Dictionary) -> Option<Value> {
    if input.len() == 1 {
        return input.keys().next().and_then(|key| input.get(key).cloned());
    }
    if input.is_empty() {
        return None;
    }
    Some(Value::Dictionary(input.clone()))
}

/// 🌱️ Seeds input boundary neurons from an in dictionary keyed by channel name.
pub fn seed_input_boundaries(tree: &Tree, in_dict: &Dictionary) -> HashMap<String, Dictionary> {
    let mut seeds = HashMap::new();
    for neuron in &tree.neurons {
        if neuron.kind != INPUT_KIND {
            continue;
        }
        let (channel_id, _) = contract_channel(neuron);
        let Some(value) = in_dict.get(&channel_id) else {
            continue;
        };
        seeds.insert(neuron.id.clone(), boundary_seed_dictionary(value));
    }
    seeds
}

/// 📤️ Collects output boundary neuron values into an out dictionary keyed by channel name.
pub fn collect_output_boundaries(tree: &Tree, channels: &EvalChannels) -> Result<Dictionary, EvalError> {
    let mut out = ColdDictionaryBuilder::new();
    for neuron in &tree.neurons {
        if neuron.kind != OUTPUT_KIND {
            continue;
        }
        let (channel_id, _) = contract_channel(neuron);
        let Some(neuron_input) = channels.inputs.get(&neuron.id) else {
            return Err(EvalError::MissingInput(format!("output boundary {channel_id}")));
        };
        let Some(value) = boundary_output_value(neuron_input) else {
            return Err(EvalError::MissingInput(format!("output boundary {channel_id}")));
        };
        out.insert(channel_id, value);
    }
    Ok(out.finish())
}

fn synapse_source_value(src_out: &Dictionary, from_port: &str) -> Value {
    if from_port.is_empty() || from_port == "out" {
        if from_port == "out" {
            if let Some(value) = src_out.get("out") {
                return value.clone();
            }
            if src_out.len() == 1 {
                if let Some(key) = src_out.keys().next() {
                    if let Some(value) = src_out.get(key) {
                        return value.clone();
                    }
                }
            }
        }
        return Value::Dictionary(src_out.clone());
    }
    src_out.get(from_port).cloned().unwrap_or_else(|| Value::Dictionary(Dictionary::new().insert("error", Value::Atom(Atom::String(format!("missing channel {from_port}"))))))
}

fn insert_variadic_slot(acc: Dictionary, slot_key: &str, port_id: &str, value: Value) -> Dictionary {
    let mut slots = acc.get(slot_key).and_then(|v| v.as_dictionary()).cloned().unwrap_or_default();
    slots = slots.insert(port_id.to_string(), value);
    acc.insert(slot_key.to_string(), Value::Dictionary(slots))
}

fn insert_fixed_port(acc: Dictionary, port_key: &str, value: Value) -> Dictionary {
    acc.insert(port_key.to_string(), value)
}

/// 💉️ Fills missing declared input keys from operator channel defaults.
pub fn inject_channel_defaults(acc: Dictionary, operator_info: &OperatorInfo) -> Dictionary {
    let mut acc = acc;
    for spec in &operator_info.inputs {
        if spec.name == "*" || acc.get(&spec.name).is_some() {
            continue;
        }
        if let Some(default) = &spec.default {
            acc = acc.insert(spec.name.clone(), default.clone());
        }
    }
    acc
}

fn inject_channel_defaults_for_operator(acc: Dictionary, operator_info: Option<&OperatorInfo>) -> Dictionary {
    match operator_info {
        Some(info) => inject_channel_defaults(acc, info),
        None => acc,
    }
}

fn list_item_count(list: &Dictionary) -> usize {
    list.keys().filter_map(|key| key.parse::<usize>().ok()).count()
}

fn validate_homogeneous_list(list: &Dictionary) -> Result<(), EvalError> {
    let mut expected: Option<String> = None;
    for key in list.keys().filter_map(|key| key.parse::<usize>().ok().map(|index| index.to_string())) {
        let Some(value) = list.get(&key) else { continue };
        if value.is_null() {
            continue;
        }
        let Some(item) = value.as_dictionary() else {
            return Err(EvalError::HeterogeneousList(format!("list item {key} is not a dictionary")));
        };
        let schema = item.schema().unwrap_or("").to_string();
        match &expected {
            None => expected = Some(schema),
            Some(current) if current == &schema => {}
            Some(current) => {
                return Err(EvalError::HeterogeneousList(format!("list mixes schema {current} and {schema}")));
            }
        }
    }
    Ok(())
}

fn validate_channel_value(channel: &ChannelSpec, value: Option<&Value>) -> Result<(), EvalError> {
    if channel.name == "*" {
        return Ok(());
    }
    if let Some(value) = value {
        if value.is_null() {
            return Ok(());
        }
    }
    if channel.cardinality.is_collection() {
        let count = match value {
            None => 0,
            Some(Value::Dictionary(list)) if list.schema() == Some("list") => list_item_count(list),
            Some(_) => {
                return Err(EvalError::CardinalityViolation(format!("channel {} expects a list dictionary", channel.name)));
            }
        };
        if !channel.cardinality.accepts(count) {
            return Err(EvalError::CardinalityViolation(format!("channel {} cardinality {} rejects count {count}", channel.name, channel.cardinality.symbol())));
        }
        if let Some(Value::Dictionary(list)) = value {
            validate_homogeneous_list(list)?;
        }
        return Ok(());
    }
    let count = usize::from(value.is_some());
    if !channel.cardinality.accepts(count) {
        return Err(EvalError::CardinalityViolation(format!("channel {} cardinality {} rejects count {count}", channel.name, channel.cardinality.symbol())));
    }
    Ok(())
}

fn validate_neuron_inputs(acc: &Dictionary, operator_info: Option<&OperatorInfo>) -> Result<(), EvalError> {
    let Some(info) = operator_info else {
        return Ok(());
    };
    if info.variadic_input.is_some() {
        return Ok(());
    }
    for channel in &info.inputs {
        if channel.name == "*" {
            continue;
        }
        let value = acc.get(&channel.name);
        if value.is_none() && channel.default.is_none() {
            continue;
        }
        validate_channel_value(channel, value)?;
    }
    Ok(())
}

fn validate_operator_outputs(info: &OperatorInfo, output: &Dictionary) -> Result<(), EvalError> {
    if info.variadic_output.is_some() {
        return Ok(());
    }
    for channel in &info.outputs {
        validate_channel_value(channel, output.get(&channel.name))?;
    }
    Ok(())
}

fn collect_neuron_input(tree: &Tree, outputs: &BTreeMap<String, Dictionary>, neuron_id: &str, operator_info: Option<&OperatorInfo>) -> Result<Dictionary, EvalError> {
    let mut acc = Dictionary::new();
    let variadic = operator_info.and_then(|info| info.variadic_input.as_ref());
    for syn in &tree.synapses {
        if syn.to != neuron_id {
            continue;
        }
        let Some(src_out) = outputs.get(&syn.from) else { continue };
        let value = synapse_source_value(src_out, &syn.from_port);
        if let Some(spec) = variadic {
            let port_id = if syn.to_port.is_empty() { "0" } else { syn.to_port.as_str() };
            acc = insert_variadic_slot(acc, &spec.slot_key, port_id, value);
            continue;
        }
        if syn.to_port.is_empty() {
            if let Value::Dictionary(dict) = value {
                let next = acc.merge(&dict);
                acc.retire_cold(); dict.retire_cold(); acc = next;
            }
            continue;
        }
        acc = insert_fixed_port(acc, &syn.to_port, value);
    }
    let acc = ColdOwner::new(inject_channel_defaults_for_operator(acc, operator_info));
    validate_neuron_inputs(&acc, operator_info)?;
    Ok(acc.into_inner())
}

fn channel_schema(input: &Dictionary, channel: &ChannelSpec) -> String {
    input
        .get(&channel.name)
        .and_then(|value| value.as_dictionary())
        .and_then(|dictionary| dictionary.schema())
        .map(str::to_string)
        .or_else(|| channel.default.as_ref().and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.schema()).map(str::to_string))
        .unwrap_or_default()
}

fn operator_signature(info: &OperatorInfo, input: &Dictionary) -> Vec<String> {
    if let Some(variadic) = &info.variadic_input {
        return input
            .get(&variadic.slot_key)
            .and_then(|value| value.as_dictionary())
            .map(|items| {
                let mut keys: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
                keys.sort_unstable();
                keys.into_iter().filter_map(|index| items.get(&index.to_string())).filter_map(|value| value.as_dictionary()).filter_map(|dictionary| dictionary.schema()).map(str::to_string).collect()
            })
            .unwrap_or_default();
    }
    info.inputs.iter().filter(|channel| channel.name != "*").map(|channel| channel_schema(input, channel)).collect()
}

fn topo_order(tree: &Tree) -> Result<Vec<String>, EvalError> {
    Ok(topo_levels(tree)?.into_iter().flatten().collect())
}

// 🚧️ Not migrated to `graph::algorithms::topo_levels`: doing so would create a circular crate
// dependency (`neural_engine` → `semio-framework-graph` → `graph::manifest` → `neural_engine`,
// via manifest's `PropertyValue` → `neural_engine::Value` conversion). Fixing that requires relocating
// `Value`/`Atom` out of `neural_engine` into a shared lower crate — out of this ticket's scope.
fn topo_levels(tree: &Tree) -> Result<Vec<Vec<String>>, EvalError> {
    let ids: HashSet<String> = tree.neurons.iter().map(|n| n.id.clone()).collect();
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
    for id in &ids {
        incoming.insert(id.clone(), vec![]);
    }
    for syn in &tree.synapses {
        if !ids.contains(&syn.from) || !ids.contains(&syn.to) {
            continue;
        }
        incoming.entry(syn.to.clone()).or_default().push(syn.from.clone());
    }
    let mut indegree: HashMap<String, usize> = incoming.iter().map(|(k, v)| (k.clone(), v.len())).collect();
    let mut queue: VecDeque<String> = indegree.iter().filter(|(_, &d)| d == 0).map(|(k, _)| k.clone()).collect();
    queue.make_contiguous().sort();
    let mut levels = Vec::new();
    let mut visited = 0usize;
    while !queue.is_empty() {
        let mut level: Vec<String> = queue.drain(..).collect();
        level.sort();
        visited += level.len();
        for n in &level {
            for syn in &tree.synapses {
                if syn.from != *n {
                    continue;
                }
                if let Some(d) = indegree.get_mut(&syn.to) {
                    *d = d.saturating_sub(1);
                    if *d == 0 {
                        queue.push_back(syn.to.clone());
                    }
                }
            }
        }
        levels.push(level);
    }
    if visited != ids.len() {
        return Err(EvalError::CycleDetected);
    }
    Ok(levels)
}
// #endregion 🔖️Evaluator

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
