//! 🧠️ Headless neural engine: dictionary in, dictionary out.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

// #region 🔖️Dictionary
/// 📚️ Immutable, unordered, collision-free key-value collection.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Dictionary {
    pairs: BTreeMap<String, Value>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_schema(schema: impl Into<String>) -> Self {
        Self::new().insert(SCHEMA_KEY, Value::Atom(Atom::String(schema.into())))
    }

    pub fn insert(mut self, key: impl Into<String>, value: Value) -> Self {
        self.pairs.insert(key.into(), value);
        self
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

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn merge(&self, other: &Dictionary) -> Dictionary {
        let mut pairs = self.pairs.clone();
        for (k, v) in &other.pairs {
            pairs.insert(k.clone(), v.clone());
        }
        Dictionary { pairs }
    }
}

/// 🔑️ Dot-separated camelCase segment path.
pub type Key = String;

/// 💎️ Atom or nested dictionary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
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

/// ⚛️ Immutable non-dictionary value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
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
// #endregion 🔖️Dictionary

// #region 🔖️Schema
pub const SCHEMA_KEY: &str = "$schema";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "of")]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldSpec {
    pub key: String,
    pub value: ValueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl FieldSpec {
    pub fn new(key: impl Into<String>, value: ValueType) -> Self {
        Self { key: key.into(), value, default: None, label: None }
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }

    pub fn decimal(key: impl Into<String>) -> Self {
        Self::new(key, ValueType::Decimal)
    }

    pub fn decimal_default(key: impl Into<String>, default: f64) -> Self {
        Self::decimal(key).with_default(Value::Atom(Atom::Decimal(default)))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub id: String,
    pub module: String,
    pub name: String,
    pub icon: String,
    pub summary: String,
    pub fields: Vec<FieldSpec>,
}

/// 🏷️ Schema id with display metadata for pickers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaRef {
    pub id: String,
    pub name: String,
    pub icon: String,
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
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum NeuralEngineError {
    /// 🔢️ Atom value doesn't match the scalar type a schema field declares.
    #[error("{kind} field is not {reason}")]
    FieldTypeMismatch { kind: &'static str, reason: &'static str },
    /// 📦️ Field value isn't a dictionary where a list/schema/any field required one.
    #[error("field is not a dictionary")]
    FieldNotDictionary,
    /// 🕳️ Channel value is null where a value was required.
    #[error("channel value is null")]
    ChannelNull,
    /// 📦️ Channel value isn't the wrapping dictionary its scalar type expects.
    #[error("{kind} channel is not a dictionary")]
    ChannelTypeMismatch { kind: &'static str },
    /// 📦️ Channel value isn't a dictionary where a list/schema/any channel required one.
    #[error("channel is not a dictionary")]
    ChannelNotDictionary,
    /// 🕳️ Channel dictionary is missing its `value` entry.
    #[error("{kind} channel is missing value")]
    ChannelMissingValue { kind: &'static str },
    /// 🔍️ A schema instance or field channel is absent from the input dictionary.
    #[error("missing {0}")]
    Missing(String),
    /// ⚠️ A schema instance dictionary carries the wrong `$schema` tag.
    #[error("invalid {0}")]
    Invalid(String),
    /// 🔍️ A declared schema field is absent from the constructed dictionary.
    #[error("missing field {0}")]
    MissingField(String),
    /// 🚫️ Neither an instance nor any field inputs were provided to a schema component.
    #[error("no instance or field inputs provided")]
    NoInputProvided,
    /// 🔢️ A cardinality symbol string didn't parse to a known cardinality.
    #[error("invalid cardinality: {0}")]
    InvalidCardinality(String),
    /// 🧬️ The constructed/modified dictionary failed schema validation.
    #[error(transparent)]
    Validation(#[from] EvalError),
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
        let mut dictionary = self.schema.default_dictionary();
        for field in provided {
            dictionary = dictionary.insert(field.key.clone(), read_schema_field_input(input, field)?);
        }
        for field in &self.schema.fields {
            if dictionary.get(&field.key).is_none() {
                return Err(NeuralEngineError::MissingField(field.key.clone()));
            }
        }
        self.schema.validate(&dictionary)?;
        Ok(dictionary)
    }

    fn deconstruct(&self, input: &Dictionary) -> Result<Dictionary, NeuralEngineError> {
        let instance = read_schema_instance(input, &self.schema)?;
        self.schema.validate(instance)?;
        Ok(instance.clone())
    }

    fn modify(&self, input: &Dictionary, provided: &[&FieldSpec]) -> Result<Dictionary, NeuralEngineError> {
        let mut dictionary = read_schema_instance(input, &self.schema)?.clone();
        for field in provided {
            dictionary = dictionary.insert(field.key.clone(), read_schema_field_input(input, field)?);
        }
        self.schema.validate(&dictionary)?;
        Ok(dictionary)
    }

    fn success_output(&self, instance: &Dictionary) -> Result<Dictionary, NeuralEngineError> {
        let mut output = Dictionary::new().insert(self.schema.id.clone(), Value::Dictionary(instance.clone()));
        for field in &self.schema.fields {
            // 🛡️ `instance` only reaches here after `Schema::validate` confirmed every
            // declared field.key is present, so this lookup can never miss.
            let value = instance.get(&field.key).expect("validated field");
            let channel = field_to_channel(value, &field.value)?;
            output = output.insert(field.key.clone(), channel);
        }
        Ok(output.insert("errors", Value::Dictionary(schema_errors_list(&[]))))
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
        Ok(match result.and_then(|instance| self.success_output(&instance)) {
            Ok(output) => output,
            Err(error) => self.error_output(&[error.to_string()]),
        })
    }
}
// #endregion 🔖️SchemaComponent

// #region 🔖️Tree
/// 🌳️ Directed acyclic graph of neurons and synapses.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Tree {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
}

/// 🔵️ Neuron instance bound to a kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Neuron {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub params: Dictionary,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Synapse {
    pub id: String,
    pub from: String,
    pub to: String,
    #[serde(default = "default_from_port")]
    pub from_port: String,
    #[serde(default = "default_to_port")]
    pub to_port: String,
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

/// 🧮️ Computational unit: one dictionary to another.
pub trait Operator: Send + Sync {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError>;
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

impl Serialize for Cardinality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.symbol())
    }
}

impl<'de> Deserialize<'de> for Cardinality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::from_symbol(&raw).map_err(serde::de::Error::custom)
    }
}
// #endregion 🔖️Cardinality

/// ➕️ Variadic input or output slot specification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariadicSpec {
    pub slot_key: String,
    pub min: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,
}

/// 🔌️ Declared operator channel with required/provided operator capabilities.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSpec {
    pub code: String,
    pub abbreviation: String,
    pub name: String,
    pub full_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operators: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub cardinality: Cardinality,
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
        self.default = Some(default);
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

/// 📇️ Catalogue metadata for an operator.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorInfo {
    pub id: String,
    pub extension: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    pub inputs: Vec<ChannelSpec>,
    pub outputs: Vec<ChannelSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_input: Option<VariadicSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_output: Option<VariadicSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<String>,
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
    schemas: HashMap<String, Schema>,
    operators: HashMap<String, OperatorRecord>,
    operator_produces: HashMap<String, Vec<String>>,
    schema_providers: HashMap<String, HashSet<String>>,
    finalized: bool,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_schema(&mut self, schema: Schema) {
        self.schemas.insert(schema.id.clone(), schema);
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
        self.operators.insert(id, OperatorRecord { info, implementations });
        self.finalized = false;
    }

    pub fn finalize(&mut self) {
        if self.finalized {
            return;
        }
        let schema_ids: Vec<String> = self.schemas.keys().cloned().collect();
        for schema_id in schema_ids {
            let Some(schema) = self.schemas.get(&schema_id).cloned() else { continue };
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
            self.operators.insert(operator_id, OperatorRecord { info, implementations: vec![OperatorImpl { schemas: vec![], operator: Box::new(SchemaComponent { schema }) }] });
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

    fn finalize_operator_info(info: &OperatorInfo, produces: Option<&[String]>, schema_providers: &HashMap<String, HashSet<String>>) -> OperatorInfo {
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
        let output = implementation.operator.evaluate(input)?;
        validate_operator_outputs(&operator.info, &output)?;
        Ok(output)
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
    entries: Mutex<HashMap<u64, (u64, Dictionary)>>,
    epoch: AtomicU64,
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
        if let Ok(mut entries) = self.entries.lock() {
            entries.insert(key, (epoch, value));
        }
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
        if let Ok(mut entries) = self.entries.lock() {
            entries.insert(key, (epoch, value.clone()));
        }
        Ok(value)
    }

    pub fn sweep(&self) {
        let epoch = self.epoch.load(Ordering::Relaxed);
        if let Ok(mut entries) = self.entries.lock() {
            entries.retain(|_, (entry_epoch, _)| *entry_epoch == epoch);
        }
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
    neurons: HashMap<String, NeuronSnapshot>,
    seed_keys: HashMap<String, u64>,
}

impl TreeSnapshot {
    pub fn capture(tree: &Tree, seeds: &HashMap<String, Dictionary>) -> Self {
        let mut neurons: HashMap<String, NeuronSnapshot> = tree.neurons.iter().map(|neuron| (neuron.id.clone(), NeuronSnapshot { key: neuron_key_hash(neuron), incoming: incoming_edges_signature(tree, &neuron.id), dependents: Vec::new() })).collect();
        for syn in &tree.synapses {
            if !neurons.contains_key(&syn.to) {
                continue;
            }
            if let Some(source) = neurons.get_mut(&syn.from) {
                source.dependents.push(syn.to.clone());
            }
        }
        let mut seed_keys = HashMap::new();
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
    pub outputs: HashMap<String, Dictionary>,
    pub inputs: HashMap<String, Dictionary>,
}

/// ⏳️ Result of a budget-limited evaluation pass — `remaining` (in topo order) is empty once the
/// whole dirty set has been walked; a non-empty `remaining` means resume with another budgeted call.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetedEval {
    pub channels: EvalChannels,
    pub remaining: Vec<String>,
    pub pending_extension: Option<PendingExtensionEval>,
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

    pub fn evaluate(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>) -> Result<HashMap<String, Dictionary>, EvalError> {
        Ok(self.evaluate_channels(tree, seeds, &HashMap::new())?.outputs)
    }

    pub fn evaluate_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
    ) -> Result<HashMap<String, Dictionary>, EvalError> {
        Ok(self.evaluate_channels_with(tree, seeds, operator_infos, dispatch)?.outputs)
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
        let cache = NeuralCache::new();
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
        self.evaluate_channels_budgeted(tree, seeds, operator_infos, dispatch, cache, dirty, previous, usize::MAX).map(|budgeted| budgeted.channels)
    }

    /// ⏳️ Sequential topo walk that stops after computing `budget` cache-missed (i.e. actually
    /// dispatched) neurons, returning the not-yet-computed neuron ids as `remaining` so a caller can
    /// resume with another budgeted call — used to spread a heavy evaluation across many cheap ticks
    /// instead of blocking a thread for the whole graph. `budget = 0` is a pure probe: nothing is
    /// dispatched, `remaining` reports every neuron that would still need work.
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
        budget: usize,
    ) -> Result<BudgetedEval, EvalError> {
        let order = topo_order(tree)?;
        let mut outputs: HashMap<String, Dictionary> = seeds.clone();
        let mut inputs: HashMap<String, Dictionary> = HashMap::new();
        let mut spent = 0usize;
        for (index, neuron_id) in order.iter().enumerate() {
            if !dirty.contains(neuron_id) {
                if let Some(prev) = previous {
                    if let (Some(out), Some(inp)) = (prev.outputs.get(neuron_id), prev.inputs.get(neuron_id)) {
                        outputs.insert(neuron_id.clone(), out.clone());
                        inputs.insert(neuron_id.clone(), inp.clone());
                        continue;
                    }
                }
            }
            let neuron = tree.neurons.iter().find(|n| n.id == *neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
            let operator_info = operator_info_for_neuron(neuron, operator_infos, self.registry.operator_info(&neuron.kind));
            let input = collect_neuron_input(tree, &outputs, neuron_id, operator_info)?;
            inputs.insert(neuron_id.clone(), input.clone());
            if let Some(seed) = seeds.get(neuron_id) {
                outputs.insert(neuron_id.clone(), seed.clone());
                continue;
            }
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                outputs.insert(neuron_id.clone(), input.merge(&neuron.params));
                continue;
            }
            // 🚧️ A budget-exhausted cache miss (cluster or operator) stops the walk here; this
            // neuron and everything from `order[index..]` becomes `remaining`. Clusters have no
            // single cache key of their own (their inner neurons are cached individually), so a
            // cluster is conservatively always charged as a miss.
            if let Some(sub_tree) = neuron.tree.as_deref() {
                if spent >= budget {
                    return Ok(BudgetedEval { channels: EvalChannels { outputs, inputs }, remaining: budgeted_remaining_from(&order, index, dirty), pending_extension: None });
                }
                let out = self.evaluate_cluster_sequential(sub_tree, &input, operator_infos, dispatch, cache)?;
                outputs.insert(neuron_id.clone(), out);
                spent += 1;
                continue;
            }
            let merged = input.merge(&neuron.params);
            let key = node_hash(&neuron.kind, &merged);
            let is_miss = !cache.contains(key);
            if is_miss && spent >= budget {
                return Ok(BudgetedEval { channels: EvalChannels { outputs, inputs }, remaining: budgeted_remaining_from(&order, index, dirty), pending_extension: None });
            }
            let out = if let Some(cached) = cache.get(key) {
                cached
            } else {
                match dispatch(&neuron.kind, &merged) {
                    Err(EvalError::PendingExtension { extension_id, operator_id, node_hash }) => {
                        let input_json = serde_json::to_string(&merged).unwrap_or_else(|_| "{}".into());
                        return Ok(BudgetedEval {
                            channels: EvalChannels { outputs, inputs },
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
            outputs.insert(neuron_id.clone(), out);
            if is_miss {
                spent += 1;
            }
        }
        Ok(BudgetedEval { channels: EvalChannels { outputs, inputs }, remaining: Vec::new(), pending_extension: None })
    }

    pub fn evaluate_channels_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HashMap<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
    ) -> Result<EvalChannels, EvalError> {
        let cache = NeuralCache::new();
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
        let mut outputs: HashMap<String, Dictionary> = seeds.clone();
        let mut inputs: HashMap<String, Dictionary> = HashMap::new();
        for level in levels {
            let mut level_inputs: HashMap<String, Dictionary> = HashMap::new();
            let mut level_outputs: HashMap<String, Dictionary> = HashMap::new();
            let mut deferred_clusters: Vec<(String, Tree, Dictionary)> = Vec::new();
            let mut compute_jobs: Vec<(String, String, Dictionary)> = Vec::new();

            for neuron_id in &level {
                if !dirty.contains(neuron_id) {
                    if let Some(prev) = previous {
                        if let (Some(out), Some(inp)) = (prev.outputs.get(neuron_id), prev.inputs.get(neuron_id)) {
                            level_outputs.insert(neuron_id.clone(), out.clone());
                            level_inputs.insert(neuron_id.clone(), inp.clone());
                            continue;
                        }
                    }
                }
                let neuron = tree.neurons.iter().find(|n| n.id == *neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
                let operator_info = operator_info_for_neuron(neuron, operator_infos, self.registry.operator_info(&neuron.kind));
                let input = collect_neuron_input(tree, &outputs, neuron_id, operator_info)?;
                level_inputs.insert(neuron_id.clone(), input.clone());
                if let Some(seed) = seeds.get(neuron_id) {
                    level_outputs.insert(neuron_id.clone(), seed.clone());
                    continue;
                }
                if let Some(sub_tree) = neuron.tree.as_deref().cloned() {
                    deferred_clusters.push((neuron_id.clone(), sub_tree, input));
                    continue;
                }
                if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                    level_outputs.insert(neuron_id.clone(), input.merge(&neuron.params));
                    continue;
                }
                compute_jobs.push((neuron_id.clone(), neuron.kind.clone(), input.merge(&neuron.params)));
            }

            #[cfg(feature = "parallel")]
            {
                use rayon::prelude::*;
                let parallel_outputs: Vec<(String, Dictionary)> = compute_jobs
                    .par_iter()
                    .map(|(neuron_id, kind, merged)| {
                        let out = evaluate_cached_output(cache, kind, merged, || dispatch(kind, merged));
                        (neuron_id.clone(), out)
                    })
                    .collect();
                for entry in parallel_outputs {
                    level_outputs.insert(entry.0, entry.1);
                }
            }
            #[cfg(not(feature = "parallel"))]
            {
                for (neuron_id, kind, merged) in compute_jobs {
                    let out = evaluate_cached_output(cache, &kind, &merged, || dispatch(&kind, &merged));
                    level_outputs.insert(neuron_id, out);
                }
            }

            for (neuron_id, sub_tree, input) in deferred_clusters {
                let out = self.evaluate_cluster(&sub_tree, &input, operator_infos, dispatch, cache)?;
                level_outputs.insert(neuron_id, out);
            }

            inputs.extend(level_inputs);
            outputs.extend(level_outputs);
        }
        Ok(EvalChannels { outputs, inputs })
    }

    /// 🧮️ Evaluates a tree as a function: in dictionary to out dictionary via boundary neurons.
    pub fn evaluate_function(&self, tree: &Tree, in_dict: &Dictionary) -> Result<Dictionary, EvalError> {
        self.evaluate_function_with(tree, in_dict, &HashMap::new(), &|kind, input| self.registry.dispatch(kind, input))
    }

    /// 🧮️ Evaluates a tree as a function with custom dispatch and operator metadata.
    pub fn evaluate_function_with(&self, tree: &Tree, in_dict: &Dictionary, operator_infos: &HashMap<String, OperatorInfo>, dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync)) -> Result<Dictionary, EvalError> {
        let seeds = seed_input_boundaries(tree, in_dict);
        let channels = self.evaluate_channels_with(tree, &seeds, operator_infos, dispatch)?;
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
        let seeds = seed_input_boundaries(tree, in_dict);
        let channels = self.evaluate_channels_cached(tree, &seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?;
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
        let sub_seeds = seed_input_boundaries(sub_tree, parent_input);
        let sub_channels = self.evaluate_channels_sequential_cached(sub_tree, &sub_seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?;
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
        let sub_seeds = seed_input_boundaries(sub_tree, parent_input);
        // 🧩️ Nested cluster subtrees are evaluated atomically (v1 limitation): any change inside
        // re-evaluates the whole cluster rather than propagating dirtiness within it. Never stale,
        // just not maximally incremental for nested clusters.
        let sub_channels = self.evaluate_channels_cached(sub_tree, &sub_seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?;
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
    let mut out = Dictionary::new();
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
        out = out.insert(channel_id, value);
    }
    Ok(out)
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
    src_out.get(from_port).cloned().unwrap_or(Value::Dictionary(Dictionary::new().insert("error", Value::Atom(Atom::String(format!("missing channel {from_port}"))))))
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

fn collect_neuron_input(tree: &Tree, outputs: &HashMap<String, Dictionary>, neuron_id: &str, operator_info: Option<&OperatorInfo>) -> Result<Dictionary, EvalError> {
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
                acc = acc.merge(&dict);
            }
            continue;
        }
        acc = insert_fixed_port(acc, &syn.to_port, value);
    }
    let acc = inject_channel_defaults_for_operator(acc, operator_info);
    validate_neuron_inputs(&acc, operator_info)?;
    Ok(acc)
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
mod tests {
    use super::*;

    struct Echo;

    impl Operator for Echo {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            let payload = input.get("x").and_then(|value| value.as_dictionary()).cloned().unwrap_or_else(|| input.clone());
            Ok(channel_output("x", payload))
        }
    }

    struct Double;

    impl Operator for Double {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            let value = input.get("number").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("number.value".into()))?;
            Ok(channel_output("doubled", number_dictionary(value * 2.0)))
        }
    }

    fn number_schema() -> Schema {
        Schema { id: "number".into(), module: "core".into(), name: "Number".into(), icon: "emoji:#".into(), summary: "Number dictionary".into(), fields: vec![FieldSpec::decimal_default("value", 0.0)] }
    }

    fn number_dictionary(value: f64) -> Dictionary {
        Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
    }

    fn echo_info() -> OperatorInfo {
        OperatorInfo {
            id: "echo".into(),
            extension: "test".into(),
            name: "Echo".into(),
            abbreviation: "Echo".into(),
            icon: "emoji:📣️".into(),
            summary: "Forwards input".into(),
            inputs: vec![ChannelSpec::any("x")],
            outputs: vec![ChannelSpec::named("X", "x", "x", "Echoed")],
            ..Default::default()
        }
    }

    fn double_info() -> OperatorInfo {
        OperatorInfo {
            id: "double".into(),
            extension: "test".into(),
            name: "Double".into(),
            abbreviation: "Dbl".into(),
            icon: "emoji:✖️".into(),
            summary: "Doubles number".into(),
            inputs: vec![ChannelSpec::number("number", &["double"])],
            outputs: vec![ChannelSpec::named("D", "Dbl", "doubled", "DoubledNumber")],
            ..Default::default()
        }
    }

    #[test]
    fn dictionary_schema_round_trip() {
        let d = number_dictionary(3.1);
        let json = serde_json::to_string(&d).unwrap();
        let back: Dictionary = serde_json::from_str(&json).unwrap();
        assert_eq!(back.schema(), Some("number"));
        assert_eq!(d, back);
    }

    #[test]
    fn schema_ids_and_refs_list_registered_schemas() {
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.finalize();
        assert_eq!(reg.schema_ids(), vec!["number".to_string()]);
        let refs = reg.schema_refs();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].id, "number");
        assert_eq!(refs[0].name, "Number");
    }

    #[test]
    fn registry_dispatches_operator() {
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        let out = reg.dispatch("echo", &Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0)))).unwrap();
        assert!(out.get("x").is_some());
    }

    #[test]
    fn registry_catalogue_lists_operators_and_schemas() {
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
        assert_eq!(reg.schema_catalogue()[0].id, "number");
        assert_eq!(reg.operator_catalogue()[0].id, "double");
        assert_eq!(reg.operator_catalogue()[1].id, "echo");
    }

    #[test]
    fn evaluate_with_custom_dispatch() {
        let tree = Tree { neurons: vec![Neuron::with_kind("b", "double", Dictionary::new().insert("number", Value::Dictionary(number_dictionary(3.0))))], synapses: vec![] };
        let out = Evaluator::new(&Registry::new())
            .evaluate_with(&tree, &HashMap::new(), &HashMap::new(), &|kind, input| {
                assert_eq!(kind, "double");
                Double.evaluate(input)
            })
            .unwrap();
        assert_eq!(out.get("b").and_then(|d| d.get("doubled")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(6.0));
    }

    #[test]
    fn two_neuron_pipeline() {
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
        };
        let out = Evaluator::new(&reg).evaluate(&tree, &HashMap::new()).unwrap();
        assert_eq!(out.get("b").and_then(|d| d.get("doubled")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.0));
    }

    #[test]
    fn evaluate_channels_returns_resolved_inputs_per_neuron() {
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
        let tree = Tree { neurons: vec![Neuron::with_kind("add", "double", Dictionary::new())], synapses: vec![Synapse { id: "s1".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "number".into() }] };
        let mut seeds = HashMap::new();
        seeds.insert("slider".into(), channel_output("number", number_dictionary(3.0)));
        let channels = Evaluator::new(&reg).evaluate_channels(&tree, &seeds, &HashMap::from([(double_info().id.clone(), double_info())])).unwrap();
        assert_eq!(channels.inputs.get("add").and_then(|d| d.get("number")).and_then(|v| v.as_dictionary()).and_then(|d| d.schema()), Some("number"));
        assert_eq!(channels.outputs.get("add").and_then(|d| d.get("doubled")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(6.0));
    }

    #[test]
    fn collect_routes_fixed_port_by_key() {
        let tree = Tree {
            neurons: vec![Neuron::with_kind("add", "math.add", Dictionary::new())],
            synapses: vec![
                Synapse { id: "s1".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "a".into() },
                Synapse { id: "s2".into(), from: "note".into(), to: "add".into(), from_port: "number".into(), to_port: "b".into() },
            ],
        };
        let mut outputs = HashMap::new();
        outputs.insert("slider".into(), channel_output("number", number_dictionary(2.0)));
        outputs.insert("note".into(), channel_output("number", number_dictionary(3.0)));
        let input = collect_neuron_input(&tree, &outputs, "add", None).unwrap();
        assert_eq!(input.get("a").and_then(|v| v.as_dictionary()).and_then(|d| d.schema()), Some("number"));
        assert_eq!(input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.schema()), Some("number"));
    }

    #[test]
    fn collect_routes_variadic_slots_in_order() {
        let operator = OperatorInfo {
            id: "dictionary.merge".into(),
            extension: "dictionary".into(),
            name: "Merge".into(),
            abbreviation: "Merge".into(),
            icon: "emoji:🔀️".into(),
            summary: "Merge".into(),
            inputs: vec![],
            outputs: vec![ChannelSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
            variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
            ..Default::default()
        };
        let tree = Tree {
            neurons: vec![Neuron::with_kind("merge", "dictionary.merge", Dictionary::new())],
            synapses: vec![
                Synapse { id: "s1".into(), from: "a".into(), to: "merge".into(), from_port: "dictionary".into(), to_port: "0".into() },
                Synapse { id: "s2".into(), from: "b".into(), to: "merge".into(), from_port: "dictionary".into(), to_port: "1".into() },
            ],
        };
        let mut outputs = HashMap::new();
        outputs.insert("a".into(), channel_output("dictionary", Dictionary::with_schema("dictionary")));
        outputs.insert("b".into(), channel_output("dictionary", Dictionary::with_schema("dictionary")));
        let input = collect_neuron_input(&tree, &outputs, "merge", Some(&operator)).unwrap();
        let items = input.get("items").and_then(|v| v.as_dictionary()).expect("items");
        assert!(items.get("0").is_some());
        assert!(items.get("1").is_some());
    }

    struct AddNumbers;

    impl Operator for AddNumbers {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            let a = input.get("a").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("a".into()))?;
            let b = input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("b".into()))?;
            Ok(channel_output("sum", number_dictionary(a + b)))
        }
    }

    fn add_info() -> OperatorInfo {
        OperatorInfo {
            id: "math.add".into(),
            extension: "math".into(),
            name: "Add".into(),
            abbreviation: "Add".into(),
            icon: "emoji:➕️".into(),
            summary: "Adds numbers".into(),
            inputs: vec![ChannelSpec::number("a", &["math.add"]), ChannelSpec::number("b", &["math.add"])],
            outputs: vec![ChannelSpec::named("S", "Sum", "sum", "Sum")],
            ..Default::default()
        }
    }

    fn input_boundary(id: &str, channel: &str) -> Neuron {
        Neuron::with_kind(id, INPUT_KIND, Dictionary::new().insert("channel", Value::Atom(Atom::String(channel.into()))).insert("operators", Value::Atom(Atom::String("math.add".into()))))
    }

    fn output_boundary(id: &str, channel: &str) -> Neuron {
        Neuron::with_kind(id, OUTPUT_KIND, Dictionary::new().insert("channel", Value::Atom(Atom::String(channel.into()))).insert("operators", Value::Atom(Atom::String("math.add".into()))))
    }

    #[test]
    fn cluster_contract_derives_channels() {
        let tree = Tree { neurons: vec![input_boundary("in_a", "a"), input_boundary("in_b", "b"), output_boundary("out_sum", "sum")], synapses: vec![] };
        let (inputs, outputs) = tree.contract();
        assert_eq!(inputs.len(), 2);
        assert_eq!(outputs.len(), 1);
        assert_eq!(inputs[0].name, "a");
        assert_eq!(outputs[0].name, "sum");
        let info = cluster_operator_info("cluster-1", "Add cluster", &tree);
        assert_eq!(info.inputs.len(), 2);
        assert_eq!(info.outputs[0].name, "sum");
    }

    #[test]
    fn cluster_runs_inner_tree() {
        let inner = Tree {
            neurons: vec![input_boundary("in_a", "a"), input_boundary("in_b", "b"), Neuron::with_kind("add", "math.add", Dictionary::new()), output_boundary("out_sum", "sum")],
            synapses: vec![
                Synapse { id: "s1".into(), from: "in_a".into(), to: "add".into(), from_port: String::new(), to_port: "a".into() },
                Synapse { id: "s2".into(), from: "in_b".into(), to: "add".into(), from_port: String::new(), to_port: "b".into() },
                Synapse { id: "s3".into(), from: "add".into(), to: "out_sum".into(), from_port: "sum".into(), to_port: String::new() },
            ],
        };
        let tree = Tree {
            neurons: vec![
                Neuron::with_kind("a_src", "core.number", Dictionary::new()),
                Neuron::with_kind("b_src", "core.number", Dictionary::new()),
                Neuron { id: "cluster".into(), kind: CLUSTER_KIND.into(), params: Dictionary::new(), tree: Some(Box::new(inner)) },
            ],
            synapses: vec![
                Synapse { id: "s_a".into(), from: "a_src".into(), to: "cluster".into(), from_port: "number".into(), to_port: "a".into() },
                Synapse { id: "s_b".into(), from: "b_src".into(), to: "cluster".into(), from_port: "number".into(), to_port: "b".into() },
            ],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(add_info(), vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(AddNumbers) }], &["number"]);
        let mut seeds = HashMap::new();
        seeds.insert("a_src".into(), channel_output("number", number_dictionary(2.0)));
        seeds.insert("b_src".into(), channel_output("number", number_dictionary(3.0)));
        let out = Evaluator::new(&reg).evaluate(&tree, &seeds).unwrap();
        assert_eq!(out.get("cluster").and_then(|d| d.get("sum")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(5.0));
    }

    #[test]
    fn evaluate_function_top_level() {
        let tree = Tree {
            neurons: vec![input_boundary("in_a", "a"), input_boundary("in_b", "b"), Neuron::with_kind("add", "math.add", Dictionary::new()), output_boundary("out_sum", "sum")],
            synapses: vec![
                Synapse { id: "s1".into(), from: "in_a".into(), to: "add".into(), from_port: String::new(), to_port: "a".into() },
                Synapse { id: "s2".into(), from: "in_b".into(), to: "add".into(), from_port: String::new(), to_port: "b".into() },
                Synapse { id: "s3".into(), from: "add".into(), to: "out_sum".into(), from_port: "sum".into(), to_port: String::new() },
            ],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(add_info(), vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(AddNumbers) }], &["number"]);
        let in_dict = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(2.0))).insert("b", Value::Dictionary(number_dictionary(3.0)));
        let out = Evaluator::new(&reg).evaluate_function(&tree, &in_dict).unwrap();
        assert_eq!(out.get("sum").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(5.0));
    }

    #[test]
    fn cluster_shakability_round_trip() {
        let inner = Tree { neurons: vec![input_boundary("in_a", "a"), output_boundary("out_a", "a")], synapses: vec![Synapse { id: "s1".into(), from: "in_a".into(), to: "out_a".into(), from_port: String::new(), to_port: String::new() }] };
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a_src", "core.number", Dictionary::new()), Neuron { id: "cluster".into(), kind: CLUSTER_KIND.into(), params: Dictionary::new(), tree: Some(Box::new(inner)) }],
            synapses: vec![Synapse { id: "s0".into(), from: "a_src".into(), to: "cluster".into(), from_port: "number".into(), to_port: "a".into() }],
        };
        let json = serde_json::to_string(&tree).unwrap();
        let back: Tree = serde_json::from_str(&json).unwrap();
        let mut seeds = HashMap::new();
        seeds.insert("a_src".into(), channel_output("number", number_dictionary(7.0)));
        let out = Evaluator::new(&Registry::new()).evaluate(&back, &seeds).unwrap();
        assert_eq!(out.get("cluster").and_then(|d| d.get("a")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(7.0));
    }

    #[test]
    fn collect_injects_declared_defaults_for_unconnected_inputs() {
        let operator = OperatorInfo {
            id: "list.get".into(),
            extension: "list".into(),
            name: "Get".into(),
            abbreviation: "Get".into(),
            icon: "emoji:🔍️".into(),
            summary: "Get".into(),
            inputs: vec![ChannelSpec::list("list", &["list.get"]), ChannelSpec::number_default("index", 0.0, &["list.get"]), ChannelSpec::boolean_default("wrap", false, &["list.get"])],
            outputs: vec![ChannelSpec::named("V", "Val", "value", "ListValue")],
            ..Default::default()
        };
        let tree = Tree { neurons: vec![Neuron::with_kind("get", "list.get", Dictionary::new())], synapses: vec![] };
        let input = collect_neuron_input(&tree, &HashMap::new(), "get", Some(&operator)).unwrap();
        assert_eq!(input.get("index").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(0.0));
        assert_eq!(input.get("wrap").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_bool()), Some(false));
    }

    #[test]
    fn node_hash_is_stable_for_identical_inputs() {
        let input = number_dictionary(3.0);
        assert_eq!(node_hash("double", &input), node_hash("double", &input));
        assert_ne!(node_hash("double", &input), node_hash("echo", &input));
    }

    #[test]
    fn cached_evaluate_skips_dispatch_on_hit() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
        let evaluator = Evaluator::new(&reg);
        let cache = NeuralCache::new();
        let calls = AtomicUsize::new(0);
        let dispatch = |kind: &str, input: &Dictionary| {
            calls.fetch_add(1, Ordering::Relaxed);
            reg.dispatch(kind, input)
        };
        cache.begin_epoch();
        evaluator.evaluate_channels_cached(&tree, &HashMap::new(), &HashMap::new(), &dispatch, &cache, &HashSet::new(), None).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 2);
        cache.begin_epoch();
        evaluator.evaluate_channels_cached(&tree, &HashMap::new(), &HashMap::new(), &dispatch, &cache, &HashSet::new(), None).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn missing_required_input_records_per_node_error() {
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "echo", number_dictionary(5.0)), Neuron::with_kind("add", "math.add", Dictionary::new())],
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "add".into(), from_port: "x".into(), to_port: "a".into() }],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        reg.register_operator(add_info(), vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(AddNumbers) }], &["number"]);
        let channels = Evaluator::new(&reg).evaluate_channels(&tree, &HashMap::new(), &HashMap::from([(add_info().id.clone(), add_info())])).unwrap();
        let add_out = channels.outputs.get("add").expect("add output");
        assert!(add_out.get("error").is_some() || add_out.get("sum").is_none());
        assert!(channels.outputs.contains_key("a"));
    }

    #[test]
    fn cached_evaluate_recomputes_only_changed_branch() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "echo", number_dictionary(5.0)), Neuron::with_kind("add", "math.add", Dictionary::new())],
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "add".into(), from_port: "x".into(), to_port: "a".into() }, Synapse { id: "s2".into(), from: "b".into(), to: "add".into(), from_port: "x".into(), to_port: "b".into() }],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        reg.register_operator(add_info(), vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(AddNumbers) }], &["number"]);
        let evaluator = Evaluator::new(&reg);
        let cache = NeuralCache::new();
        let calls = AtomicUsize::new(0);
        let dispatch = |kind: &str, input: &Dictionary| {
            calls.fetch_add(1, Ordering::Relaxed);
            reg.dispatch(kind, input)
        };
        cache.begin_epoch();
        evaluator.evaluate_channels_cached(&tree, &HashMap::new(), &HashMap::new(), &dispatch, &cache, &HashSet::new(), None).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 3);
        let mut tree_changed = tree.clone();
        tree_changed.neurons[0] = Neuron::with_kind("a", "echo", number_dictionary(3.0));
        cache.begin_epoch();
        evaluator.evaluate_channels_cached(&tree_changed, &HashMap::new(), &HashMap::new(), &dispatch, &cache, &HashSet::new(), None).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn evaluate_channels_budgeted_remaining_excludes_clean_branches() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new()), Neuron::with_kind("c", "echo", number_dictionary(9.0))],
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
        let evaluator = Evaluator::new(&reg);
        let cache = NeuralCache::new();
        let calls = AtomicUsize::new(0);
        let mut dispatch = |kind: &str, input: &Dictionary| {
            calls.fetch_add(1, Ordering::Relaxed);
            reg.dispatch(kind, input)
        };
        let dirty: HashSet<String> = ["b".to_string()].into_iter().collect();
        cache.begin_epoch();
        let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &dirty, None, 0).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        assert_eq!(result.remaining, vec!["b".to_string()], "clean branch node \"c\" must not appear in remaining");
    }

    #[test]
    fn evaluate_channels_budgeted_probe_computes_nothing() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
        let evaluator = Evaluator::new(&reg);
        let cache = NeuralCache::new();
        let calls = AtomicUsize::new(0);
        let mut dispatch = |kind: &str, input: &Dictionary| {
            calls.fetch_add(1, Ordering::Relaxed);
            reg.dispatch(kind, input)
        };
        cache.begin_epoch();
        let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, 0).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 0, "a budget-0 probe must never dispatch");
        assert_eq!(result.remaining, vec!["a".to_string(), "b".to_string()], "nothing computed yet — every neuron is still pending, in topo order");
    }

    #[test]
    fn evaluate_channels_budgeted_resumes_across_calls_until_complete() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
        let evaluator = Evaluator::new(&reg);
        let cache = NeuralCache::new();
        let calls = AtomicUsize::new(0);
        let mut dispatch = |kind: &str, input: &Dictionary| {
            calls.fetch_add(1, Ordering::Relaxed);
            reg.dispatch(kind, input)
        };
        cache.begin_epoch();
        // ⏱️ Tick 1: budget for exactly one cache miss — stops at "a", "b" hasn't run yet.
        let tick1 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, 1).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(tick1.remaining, vec!["b".to_string()]);
        // ⏱️ Tick 2: "a" is now a cache hit (free), so this budget-1 call reaches and computes "b".
        let tick2 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, 1).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 2, "resuming must not recompute the already-cached \"a\"");
        assert!(tick2.remaining.is_empty(), "the walk reached the end of the topo order");
        let doubled = tick2.channels.outputs.get("b").and_then(|dict| dict.get("doubled")).and_then(|value| value.as_dictionary()).and_then(|dict| dict.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64());
        assert_eq!(doubled, Some(4.0));
    }

    #[test]
    fn evaluate_channels_budgeted_unlimited_matches_full_evaluation() {
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
        };
        let mut reg = Registry::new();
        reg.register_schema(number_schema());
        reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
        reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
        let evaluator = Evaluator::new(&reg);
        let cache = NeuralCache::new();
        let mut dispatch = |kind: &str, input: &Dictionary| reg.dispatch(kind, input);
        cache.begin_epoch();
        let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, usize::MAX).unwrap();
        assert!(result.remaining.is_empty());
        let doubled = result.channels.outputs.get("b").and_then(|dict| dict.get("doubled")).and_then(|value| value.as_dictionary()).and_then(|dict| dict.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64());
        assert_eq!(doubled, Some(4.0));
    }

    #[test]
    fn neural_cache_get_and_contains_refresh_epoch_before_sweep() {
        let cache = NeuralCache::new();
        cache.begin_epoch();
        cache.get_or_insert_with(42, || Ok(Dictionary::new())).expect("seed cache");
        cache.begin_epoch();
        assert!(cache.contains(42), "contains must refresh the entry epoch on a hit");
        cache.sweep();
        assert!(cache.contains(42), "swept cache must retain entries touched by contains/get in the new epoch");
        cache.get(42);
        cache.sweep();
        assert_eq!(cache.len(), 1, "get must also refresh epoch so a completing eval does not evict its own hits");
    }

    #[test]
    fn cardinality_symbol_round_trips_json() {
        let channel = ChannelSpec::list("items", &["list.pack"]).with_cardinality(Cardinality::OneOrMore);
        let json = serde_json::to_string(&channel).unwrap();
        assert!(json.contains("\"cardinality\":\"+\""));
        let parsed: ChannelSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.cardinality, Cardinality::OneOrMore);
    }

    #[test]
    fn cardinality_accepts_expected_counts() {
        assert!(Cardinality::ExactlyOne.accepts(1));
        assert!(!Cardinality::ExactlyOne.accepts(0));
        assert!(Cardinality::ZeroOrMore.accepts(0));
        assert!(Cardinality::Exactly(2).accepts(2));
        assert!(!Cardinality::Exactly(2).accepts(1));
    }

    #[test]
    fn heterogeneous_list_input_is_rejected() {
        let operator = OperatorInfo {
            id: "list.size".into(),
            extension: "list".into(),
            name: "Size".into(),
            abbreviation: "Size".into(),
            icon: "emoji:📋️".into(),
            summary: "Size".into(),
            inputs: vec![ChannelSpec::list("list", &["list.size"])],
            outputs: vec![ChannelSpec::named("C", "Cnt", "count", "ListCount")],
            ..Default::default()
        };
        let tree = Tree { neurons: vec![Neuron::with_kind("size", "list.size", Dictionary::new())], synapses: vec![Synapse { id: "s1".into(), from: "src".into(), to: "size".into(), from_port: "list".into(), to_port: "list".into() }] };
        let mut outputs = HashMap::new();
        outputs.insert(
            "src".into(),
            channel_output("list", Dictionary::with_schema("list").insert("0", Value::Dictionary(number_dictionary(1.0))).insert("1", Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String("x".into())))))),
        );
        let err = collect_neuron_input(&tree, &outputs, "size", Some(&operator)).unwrap_err();
        assert!(matches!(err, EvalError::HeterogeneousList(_)));
    }

    fn point_schema() -> Schema {
        Schema {
            id: "point".into(),
            module: "math".into(),
            name: "Point".into(),
            icon: "emoji:📍️".into(),
            summary: "Point with x, y, z".into(),
            fields: vec![FieldSpec::decimal_default("x", 0.0), FieldSpec::decimal_default("y", 0.0), FieldSpec::decimal_default("z", 0.0)],
        }
    }

    #[test]
    fn null_atom_round_trips_through_json() {
        let value = Value::null();
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "null");
        let back: Value = serde_json::from_str(&json).unwrap();
        assert!(back.is_null());
    }

    #[test]
    fn schema_component_info_declares_tri_modal_ports() {
        let info = schema_component_info(&point_schema());
        assert_eq!(info.id, "math.point");
        assert_eq!(info.inputs.len(), 4);
        assert_eq!(info.outputs.len(), 5);
        assert_eq!(info.inputs[0].cardinality, Cardinality::ZeroOrOne);
        assert_eq!(info.outputs.last().expect("errors").name, "errors");
        assert_eq!(info.group, vec!["Schemas".to_string()]);
    }

    #[test]
    fn schema_component_construct_deconstruct_and_modify() {
        let mut registry = Registry::new();
        registry.register_schema(point_schema());
        registry.finalize();
        let construct = Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0))).insert("y", Value::Dictionary(number_dictionary(2.0))).insert("z", Value::Dictionary(number_dictionary(3.0)));
        let built = registry.dispatch("math.point", &construct).unwrap();
        let point = built.get("point").and_then(|value| value.as_dictionary()).expect("point");
        assert_eq!(point.get("z").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(3.0));
        let deconstructed = registry.dispatch("math.point", &Dictionary::new().insert("point", Value::Dictionary(point.clone()))).unwrap();
        assert_eq!(deconstructed.get("x").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(1.0));
        let modified = registry.dispatch("math.point", &Dictionary::new().insert("point", Value::Dictionary(point.clone())).insert("x", Value::Dictionary(number_dictionary(9.0)))).unwrap();
        assert_eq!(modified.get("point").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("x")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(9.0));
    }

    #[test]
    fn schema_component_error_emits_null_outputs_and_errors() {
        let mut registry = Registry::new();
        registry.register_schema(point_schema());
        registry.finalize();
        let output = registry.dispatch("math.point", &Dictionary::new()).unwrap();
        assert!(output.get("point").expect("point").is_null());
        assert!(output.get("x").expect("x").is_null());
        let errors = output.get("errors").and_then(|value| value.as_dictionary()).expect("errors");
        assert_eq!(errors.schema(), Some("list"));
        assert!(errors.get("0").is_some());
    }

    #[test]
    fn compute_dirty_set_is_empty_for_unchanged_snapshots() {
        let tree = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(1.0)), Neuron::with_kind("b", "double", Dictionary::new())],
            synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
        };
        let seeds = HashMap::new();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        assert!(compute_dirty_set(Some(&snapshot), &snapshot).is_empty());
    }

    #[test]
    fn compute_dirty_set_propagates_only_to_descendants_of_changed_leaf() {
        // Two independent branches: a -> b, c -> d. Changing `a`'s params should dirty `a` and
        // `b` (its descendant), but never touch `c`/`d` (a disjoint branch).
        let make_tree = |a_value: f64| Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(a_value)), Neuron::with_kind("b", "double", Dictionary::new()), Neuron::with_kind("c", "echo", number_dictionary(9.0)), Neuron::with_kind("d", "double", Dictionary::new())],
            synapses: vec![
                Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() },
                Synapse { id: "s2".into(), from: "c".into(), to: "d".into(), from_port: "x".into(), to_port: "number".into() },
            ],
        };
        let seeds = HashMap::new();
        let previous = TreeSnapshot::capture(&make_tree(1.0), &seeds);
        let current = TreeSnapshot::capture(&make_tree(2.0), &seeds);
        let dirty = compute_dirty_set(Some(&previous), &current);
        assert_eq!(dirty, HashSet::from(["a".to_string(), "b".to_string()]));
    }

    #[test]
    fn compute_dirty_set_marks_surviving_dependents_of_removed_neuron() {
        // a -> b -> c. Removing `b` and rewiring `a` directly into `c` must dirty `c`, since it
        // otherwise wouldn't be discovered as changed by iterating the *current* tree alone.
        let before = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(1.0)), Neuron::with_kind("b", "double", Dictionary::new()), Neuron::with_kind("c", "double", Dictionary::new())],
            synapses: vec![
                Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() },
                Synapse { id: "s2".into(), from: "b".into(), to: "c".into(), from_port: "x".into(), to_port: "number".into() },
            ],
        };
        let after = Tree {
            neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(1.0)), Neuron::with_kind("c", "double", Dictionary::new())],
            synapses: vec![Synapse { id: "s3".into(), from: "a".into(), to: "c".into(), from_port: "x".into(), to_port: "number".into() }],
        };
        let seeds = HashMap::new();
        let previous = TreeSnapshot::capture(&before, &seeds);
        let current = TreeSnapshot::capture(&after, &seeds);
        let dirty = compute_dirty_set(Some(&previous), &current);
        assert!(dirty.contains("c"), "surviving dependent of a removed neuron must be dirtied");
        assert!(!dirty.contains("a"), "unrelated unchanged neuron must stay clean");
    }

    #[test]
    fn compute_dirty_set_treats_seed_change_as_dirty() {
        let tree = Tree { neurons: vec![Neuron::with_kind("a", "echo", Dictionary::new())], synapses: vec![] };
        let mut before_seeds = HashMap::new();
        before_seeds.insert("a".to_string(), number_dictionary(1.0));
        let mut after_seeds = HashMap::new();
        after_seeds.insert("a".to_string(), number_dictionary(2.0));
        let previous = TreeSnapshot::capture(&tree, &before_seeds);
        let current = TreeSnapshot::capture(&tree, &after_seeds);
        let dirty = compute_dirty_set(Some(&previous), &current);
        assert_eq!(dirty, HashSet::from(["a".to_string()]));
    }
}
// #endregion 🔖️Tests
