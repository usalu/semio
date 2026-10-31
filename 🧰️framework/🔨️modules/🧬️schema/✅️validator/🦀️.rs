use crate::SchemaError;
use semio_framework_pack_json::{parse as parse_json, Number, Object, Value};
use semio_framework_value::DslValue;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[path = "🔨️compile/🦀️.rs"]
mod compile;
use compile::*;
pub use compile::resolve_schema_document_reference;
#[path = "🔎️validate/🦀️.rs"]
mod validate;
use validate::*;
#[path = "📃️string-format/🦀️.rs"]
mod string_format;
pub use string_format::*;
#[path = "🔤️pattern/🦀️.rs"]
mod pattern;
pub use pattern::*;

#[derive(Clone, semio_framework_value::RetireOwned)]
struct SchemaIndex<V> {
    entries: Vec<(String, V)>,
}

impl<V> Default for SchemaIndex<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> SchemaIndex<V> {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }

    fn get(&self, key: &str) -> Option<&V> {
        self.entries.binary_search_by(|(name, _)| name.as_str().cmp(key)).ok().map(|index| &self.entries[index].1)
    }

    fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    fn insert(&mut self, key: String, value: V) -> Option<V> {
        match self.entries.binary_search_by(|(name, _)| name.as_str().cmp(key.as_str())) {
            Ok(index) => Some(std::mem::replace(&mut self.entries[index].1, value)),
            Err(index) => {
                self.entries.insert(index, (key, value));
                None
            }
        }
    }
}

//#region 🎛️Control

#[derive(Clone, Debug)]
pub struct ValidationControl {
    cancelled: Arc<AtomicBool>,
    max_nodes: usize,
}

impl Default for ValidationControl {
    fn default() -> Self {
        Self::new(65_536)
    }
}

impl ValidationControl {
    /// 🎚️ Creates a cancellable traversal limit shared by schema compilation or validation.
    pub fn new(max_nodes: usize) -> Self {
        Self { cancelled: Arc::new(AtomicBool::new(false)), max_nodes }
    }

    /// 🛑 Requests cooperative cancellation at the next visited node.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// 🔎 Reports whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidationProgress {
    pub visited_nodes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemaFragmentPathSegment {
    Key(String),
    Index(usize),
    Append,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemaFragmentOperation {
    Set,
    Insert,
    Remove,
    Move,
    Rename,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemaFragmentValueShape {
    Null,
    Bool,
    Number,
    String,
    Array { len: usize },
    Object { len: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaFragmentContextRefusal {
    pub reason: String,
}

impl SchemaFragmentContextRefusal {
    /// 🚧️ Describes why the native snapshot could not project the requested validation frontier.
    pub fn new(reason: impl Into<String>) -> Self {
        Self { reason: reason.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaFragmentError {
    pub code: &'static str,
    pub path: String,
    pub message: String,
}

impl SchemaFragmentError {
    fn new(code: &'static str, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code, path: path.into(), message: message.into() }
    }
}

impl fmt::Display for SchemaFragmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for SchemaFragmentError {}

struct Traversal<'a> {
    control: &'a ValidationControl,
    visited_nodes: usize,
    active: Vec<(usize, usize)>,
    patterns: SchemaIndex<PatternMatcher>,
    compiled_schemas: std::collections::BTreeSet<(usize, usize)>,
}

impl<'a> Traversal<'a> {
    fn new(control: &'a ValidationControl) -> Self {
        Self { control, visited_nodes: 0, active: Vec::new(), patterns: SchemaIndex::new(), compiled_schemas: std::collections::BTreeSet::new() }
    }

    fn visit(&mut self) -> Result<(), SchemaError> {
        if self.control.is_cancelled() {
            return Err(SchemaError::Cancelled);
        }
        self.visited_nodes = self.visited_nodes.saturating_add(1);
        if self.visited_nodes > self.control.max_nodes {
            return Err(SchemaError::LimitExceeded(self.control.max_nodes));
        }
        Ok(())
    }

    fn progress(&self) -> ValidationProgress {
        ValidationProgress { visited_nodes: self.visited_nodes }
    }
}

//#endregion 🎛️Control

//#region 📋️Validator

/// 🏗️ Boundary interface for structural validation of serialized input — application boundary code
/// depends on this, never on a concrete validator type, so the owned draft-07 implementation stays
/// replaceable and nothing outside this module names a third-party validator.
pub trait StructuralValidation {
    /// ✅ Validates one serialized JSON instance before any domain rule runs.
    fn validate_structure(&self, instance_json: &str) -> Result<ValidationProgress, SchemaError>;

    /// 📊 Validates with cooperative cancellation and a bounded node traversal.
    fn validate_structure_with_control(&self, instance_json: &str, control: &ValidationControl) -> Result<ValidationProgress, SchemaError>;
}

/// 🏷️ The one JSON Schema dialect this validator implements. The compiler does not reject a
/// document declaring another `$schema` — it implements this keyword subset regardless — because
/// declaration hygiene is a repo-wide file check, not a per-instance runtime concern.
pub const JSON_SCHEMA_DRAFT_07_DIALECT: &str = "http://json-schema.org/draft-07/schema#";

/// 📋️ Owned draft-07 structural validator: no external crate, `$ref` resolved inside the compiled
/// document and across sibling documents by their `$id`.
#[derive(Clone, semio_framework_value::RetireOwned)]
pub struct OwnedJsonSchemaValidator {
    schema: Value,
    documents: SchemaIndex<Value>,
    patterns: SchemaIndex<PatternMatcher>,
}

impl OwnedJsonSchemaValidator {
    /// 📥 Compiles the product-supported draft-07 subset from an owned string boundary.
    pub fn compile(schema_json: &str) -> Result<Self, SchemaError> {
        Self::compile_with_control(schema_json, &ValidationControl::default()).map(|(validator, _)| validator)
    }

    /// 🔨️ Compiles with cooperative cancellation and deterministic node progress.
    pub fn compile_with_control(schema_json: &str, control: &ValidationControl) -> Result<(Self, ValidationProgress), SchemaError> {
        Self::compile_with_documents_and_control(schema_json, &[], control)
    }

    /// 🔗 Compiles a document whose `$ref`s may cross into `documents`, each keyed by its own `$id`.
    pub fn compile_with_documents(schema_json: &str, documents: &[&str]) -> Result<Self, SchemaError> {
        Self::compile_with_documents_and_control(schema_json, documents, &ValidationControl::default()).map(|(validator, _)| validator)
    }

    /// 📄️ Cross-document compilation with cooperative cancellation and deterministic node progress.
    pub fn compile_with_documents_and_control(schema_json: &str, documents: &[&str], control: &ValidationControl) -> Result<(Self, ValidationProgress), SchemaError> {
        let schema = parse_json(schema_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| SchemaError::Validation(format!("invalid schema JSON: {error}")))?;
        let documents = index_documents(documents)?;
        let mut traversal = Traversal::new(control);
        validate_schema_node(Scope { root: &schema, base: &schema, documents: &documents, patterns: &SchemaIndex::new() }, &schema, "$", &mut traversal)?;
        let progress = traversal.progress();
        Ok((Self { schema, documents, patterns: std::mem::take(&mut traversal.patterns) }, progress))
    }

    /// 🧱 Compiles an owned JSON schema representation without serializing the catalog input.
    pub fn compile_value(schema: &Value) -> Result<Self, SchemaError> {
        let control = ValidationControl::default();
        let documents = SchemaIndex::new();
        let mut traversal = Traversal::new(&control);
        validate_schema_node(Scope { root: schema, base: schema, documents: &documents, patterns: &SchemaIndex::new() }, schema, "$", &mut traversal)?;
        Ok(Self { schema: schema.clone(), documents, patterns: std::mem::take(&mut traversal.patterns) })
    }

    /// 🔣️ Validates one JSON string and reports deterministic traversal progress.
    pub fn validate_json(&self, value_json: &str) -> Result<ValidationProgress, SchemaError> {
        self.validate_json_with_control(value_json, &ValidationControl::default())
    }

    /// 🎛️ Validates with cooperative cancellation and a bounded node traversal.
    pub fn validate_json_with_control(&self, value_json: &str, control: &ValidationControl) -> Result<ValidationProgress, SchemaError> {
        let value = parse_json(value_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| SchemaError::Validation(format!("invalid instance JSON: {error}")))?;
        let mut traversal = Traversal::new(control);
        validate_value(Scope { root: &self.schema, base: &self.schema, documents: &self.documents, patterns: &self.patterns }, &self.schema, &value, "$", &mut traversal)?;
        Ok(traversal.progress())
    }

    /// 📗️ Reports validity without exposing the internal JSON representation.
    pub fn is_valid_json(&self, value_json: &str) -> bool {
        self.validate_json(value_json).is_ok()
    }

    /// 🩹️ Validates one candidate patch without materializing unrelated snapshot branches.
    pub fn validate_dsl_fragment_with_context<F, G>(
        &self,
        path: &[SchemaFragmentPathSegment],
        operation: SchemaFragmentOperation,
        candidate: Option<&DslValue>,
        mut context: F,
        mut shape: G,
    ) -> Result<ValidationProgress, SchemaFragmentError>
    where
        F: FnMut(&[SchemaFragmentPathSegment]) -> Result<DslValue, SchemaFragmentContextRefusal>,
        G: FnMut(&[SchemaFragmentPathSegment]) -> Result<SchemaFragmentValueShape, SchemaFragmentContextRefusal>,
    {
        let control = ValidationControl::default();
        let mut traversal = Traversal::new(&control);
        let scope = Scope { root: &self.schema, base: &self.schema, documents: &self.documents, patterns: &self.patterns };
        let mut cursor = FragmentSchemaCursor::Node { scope, schema: &self.schema };
        let mut prefix = Vec::new();
        for (index, segment) in path.iter().enumerate() {
            cursor = normalize_fragment_cursor(cursor, &prefix)?;
            if let Some((selected, discriminator)) = select_fragment_union_branch(cursor, &prefix, &mut context)? {
                cursor = selected;
                if index + 1 == path.len() && matches!(segment, SchemaFragmentPathSegment::Key(key) if key == &discriminator) {
                    validate_fragment_context(cursor, &prefix, &mut context, &mut traversal)?;
                    return Ok(traversal.progress());
                }
            }
            let is_parent = index + 1 == path.len();
            if is_parent && operation != SchemaFragmentOperation::Set {
                if validate_fragment_array_structure(cursor, &prefix, operation, &mut shape, &mut traversal)? {
                    if operation == SchemaFragmentOperation::Remove {
                        return Ok(traversal.progress());
                    }
                    cursor = descend_fragment_cursor(cursor, segment, &prefix)?;
                    prefix.push(segment.clone());
                    continue;
                }
            }
            if fragment_ancestor_needs_context(cursor, segment, operation, is_parent) {
                validate_fragment_context(cursor, &prefix, &mut context, &mut traversal)?;
                return Ok(traversal.progress());
            }
            if index + 1 == path.len() && operation != SchemaFragmentOperation::Set {
                validate_fragment_context(cursor, &prefix, &mut context, &mut traversal)?;
                return Ok(traversal.progress());
            }
            cursor = descend_fragment_cursor(cursor, segment, &prefix)?;
            prefix.push(segment.clone());
        }
        cursor = normalize_fragment_cursor(cursor, &prefix)?;
        let candidate = candidate.ok_or_else(|| {
            SchemaFragmentError::new(
                "schema.fragment.candidate-required",
                fragment_path(&prefix),
                "this fragment operation requires a candidate value",
            )
        })?;
        validate_fragment_candidate(cursor, candidate, &prefix, &mut traversal)?;
        Ok(traversal.progress())
    }

    /// 🧬️ Compiles semantic value contracts with indexed owned sibling documents.
    pub fn compile_intrinsic_with_documents(schema: &DslValue, documents: &[DslValue]) -> Result<Self, SchemaError> {
        Self::compile_intrinsic_with_documents_and_control(schema, documents, &ValidationControl::default()).map(|(validator, _)| validator)
    }
    /// 🔏️ Compiles intrinsic documents with bounded admission and cooperative cancellation.
    pub fn compile_intrinsic_with_documents_and_control(schema: &DslValue, documents: &[DslValue], control: &ValidationControl) -> Result<(Self, ValidationProgress), SchemaError> {
        let mut traversal = Traversal::new(control);
        let schema = intrinsic_json(schema, &mut traversal, 0)?;
        let mut indexed = SchemaIndex::new();
        for document in documents {
            let id = document.get("$id").and_then(DslValue::as_str).ok_or_else(||SchemaError::Validation("owned schema document requires an id".into()))?;
            if indexed.contains_key(id) { return Err(SchemaError::Validation(format!("duplicate owned schema id {id}"))); }
            indexed.insert(id.to_string(), intrinsic_json(document, &mut traversal, 0)?);
        }
        validate_schema_node(Scope { root:&schema, base:&schema, documents:&indexed, patterns:&SchemaIndex::new() },&schema,"$",&mut traversal)?;
        let progress = traversal.progress();
        Ok((Self { schema, documents:indexed, patterns:std::mem::take(&mut traversal.patterns) }, progress))
    }
    /// 🧪️ Validates a semantic value without encoding or decoding a physical document.
    pub fn validate_intrinsic(&self, value: &DslValue) -> Result<(),SchemaError> {
        self.validate_intrinsic_with_control(value, &ValidationControl::default()).map(|_| ())
    }
    /// 🧮️ Reports bounded intrinsic admission and validation work without document transport.
    pub fn validate_intrinsic_with_control(&self, value: &DslValue, control: &ValidationControl) -> Result<ValidationProgress,SchemaError> {
        let mut traversal = Traversal::new(control);
        let value = intrinsic_json(value, &mut traversal, 0)?;
        validate_value(Scope { root:&self.schema, base:&self.schema, documents:&self.documents, patterns:&self.patterns }, &self.schema, &value, "$", &mut traversal)?;
        Ok(traversal.progress())
    }

    /// 🌱️ Validates an owned JSON value without an intermediate serialized instance.
    pub fn validate_value(&self, value: &Value) -> Result<(), SchemaError> {
        let control = ValidationControl::default();
        let mut traversal = Traversal::new(&control);
        validate_value(Scope { root: &self.schema, base: &self.schema, documents: &self.documents, patterns: &self.patterns }, &self.schema, value, "$", &mut traversal)
    }
}

fn intrinsic_json(value:&DslValue, traversal:&mut Traversal<'_>, depth:usize)->Result<Value,SchemaError>{
    traversal.visit()?;
    if depth > 128 { return Err(SchemaError::Validation("intrinsic value nesting exceeds 128 levels".into())); }
    Ok(match value {
        DslValue::Null=>Value::Null,
        DslValue::Bool(value)=>Value::Bool(*value),
        DslValue::String(value)=>Value::String(value.clone()),
        DslValue::Number(value)=>Value::Number(match value {
            semio_framework_value::Number::UInt(value)=>Number::UInt(*value),
            semio_framework_value::Number::Int(value)=>Number::Int(*value),
            semio_framework_value::Number::Float(value) if value.is_finite()=>Number::Float(*value),
            _=>return Err(SchemaError::Validation("intrinsic contract requires finite numbers".into())),
        }),
        DslValue::Bytes(values)=>Value::Array(values.iter().map(|value|{traversal.visit()?;Ok(Value::Number(Number::UInt(u64::from(*value))))}).collect::<Result<_,SchemaError>>()?),
        DslValue::Array(values)=>Value::Array(values.iter().map(|value|intrinsic_json(value,traversal,depth+1)).collect::<Result<_,_>>()?),
        DslValue::Object(values)=>{
            let mut keys=std::collections::HashSet::new();
            Value::Object(values.iter().map(|(key,value)|{
                if !keys.insert(key.as_str()){return Err(SchemaError::Validation(format!("duplicate intrinsic key {key}")));}
                Ok((key.clone(),intrinsic_json(value,traversal,depth+1)?))
            }).collect::<Result<_,_>>()?)
        }
    })
}

impl StructuralValidation for OwnedJsonSchemaValidator {
    fn validate_structure(&self, instance_json: &str) -> Result<ValidationProgress, SchemaError> {
        self.validate_json(instance_json)
    }

    fn validate_structure_with_control(&self, instance_json: &str, control: &ValidationControl) -> Result<ValidationProgress, SchemaError> {
        self.validate_json_with_control(instance_json, control)
    }
}

//#endregion 📋️Validator

//#region ⚖️Equality

fn values_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(left), Value::Bool(right)) => left == right,
        (Value::String(left), Value::String(right)) => left == right,
        (Value::Number(left), Value::Number(right)) => numbers_equal(*left, *right),
        (Value::Array(left), Value::Array(right)) => left.len() == right.len() && left.iter().zip(right).all(|(left, right)| values_equal(left, right)),
        (Value::Object(left), Value::Object(right)) => left.len() == right.len() && left.iter().all(|(key, left)| right.get(key).is_some_and(|right| values_equal(left, right))),
        _ => false,
    }
}

fn numbers_equal(left: Number, right: Number) -> bool {
    match (left, right) {
        (Number::UInt(left), Number::UInt(right)) => left == right,
        (Number::Int(left), Number::Int(right)) => left == right,
        (Number::UInt(left), Number::Int(right)) | (Number::Int(right), Number::UInt(left)) => u64::try_from(right) == Ok(left),
        (Number::Float(left), Number::Float(right)) => left == right,
        (Number::UInt(integer), Number::Float(float)) | (Number::Float(float), Number::UInt(integer)) => float >= 0.0 && float < u64::MAX as f64 && float.fract() == 0.0 && float as u64 == integer,
        (Number::Int(integer), Number::Float(float)) | (Number::Float(float), Number::Int(integer)) => float >= i64::MIN as f64 && float < i64::MAX as f64 && float.fract() == 0.0 && float as i64 == integer,
    }
}

//#endregion ⚖️Equality
