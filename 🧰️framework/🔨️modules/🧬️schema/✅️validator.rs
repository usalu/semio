use crate::SchemaError;
use pack::json::{parse as parse_json, Number, Object, Value};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

struct Traversal<'a> {
    control: &'a ValidationControl,
    visited_nodes: usize,
    active: Vec<(usize, usize)>,
    patterns: HashMap<String, PatternMatcher>,
}

impl<'a> Traversal<'a> {
    fn new(control: &'a ValidationControl) -> Self {
        Self { control, visited_nodes: 0, active: Vec::new(), patterns: HashMap::new() }
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

/// ✅️ Boundary interface for structural validation of serialized input — application boundary code
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
#[derive(Clone)]
pub struct OwnedJsonSchemaValidator {
    schema: Value,
    documents: HashMap<String, Value>,
    patterns: HashMap<String, PatternMatcher>,
}

impl OwnedJsonSchemaValidator {
    /// 📥 Compiles the product-supported draft-07 subset from an owned string boundary.
    pub fn compile(schema_json: &str) -> Result<Self, SchemaError> {
        Self::compile_with_control(schema_json, &ValidationControl::default()).map(|(validator, _)| validator)
    }

    /// 📊 Compiles with cooperative cancellation and deterministic node progress.
    pub fn compile_with_control(schema_json: &str, control: &ValidationControl) -> Result<(Self, ValidationProgress), SchemaError> {
        Self::compile_with_documents_and_control(schema_json, &[], control)
    }

    /// 🔗 Compiles a document whose `$ref`s may cross into `documents`, each keyed by its own `$id`.
    pub fn compile_with_documents(schema_json: &str, documents: &[&str]) -> Result<Self, SchemaError> {
        Self::compile_with_documents_and_control(schema_json, documents, &ValidationControl::default()).map(|(validator, _)| validator)
    }

    /// 📊 Cross-document compilation with cooperative cancellation and deterministic node progress.
    pub fn compile_with_documents_and_control(schema_json: &str, documents: &[&str], control: &ValidationControl) -> Result<(Self, ValidationProgress), SchemaError> {
        let schema = parse_json(schema_json).map_err(|error| SchemaError::Validation(format!("invalid schema JSON: {error}")))?;
        let documents = index_documents(documents)?;
        let mut traversal = Traversal::new(control);
        validate_schema_node(Scope { base: &schema, documents: &documents, patterns: &HashMap::new() }, &schema, "$", &mut traversal)?;
        let progress = traversal.progress();
        Ok((Self { schema, documents, patterns: std::mem::take(&mut traversal.patterns) }, progress))
    }

    pub(crate) fn new(schema: &Value) -> Result<Self, SchemaError> {
        let control = ValidationControl::default();
        let documents = HashMap::new();
        let mut traversal = Traversal::new(&control);
        validate_schema_node(Scope { base: schema, documents: &documents, patterns: &HashMap::new() }, schema, "$", &mut traversal)?;
        Ok(Self { schema: schema.clone(), documents, patterns: std::mem::take(&mut traversal.patterns) })
    }

    /// ✅ Validates one JSON string and reports deterministic traversal progress.
    pub fn validate_json(&self, value_json: &str) -> Result<ValidationProgress, SchemaError> {
        self.validate_json_with_control(value_json, &ValidationControl::default())
    }

    /// 📊 Validates with cooperative cancellation and a bounded node traversal.
    pub fn validate_json_with_control(&self, value_json: &str, control: &ValidationControl) -> Result<ValidationProgress, SchemaError> {
        let value = parse_json(value_json).map_err(|error| SchemaError::Validation(format!("invalid instance JSON: {error}")))?;
        let mut traversal = Traversal::new(control);
        validate_value(Scope { base: &self.schema, documents: &self.documents, patterns: &self.patterns }, &self.schema, &value, "$", &mut traversal)?;
        Ok(traversal.progress())
    }

    /// 🔎 Reports validity without exposing the internal JSON representation.
    pub fn is_valid_json(&self, value_json: &str) -> bool {
        self.validate_json(value_json).is_ok()
    }

    pub(crate) fn validate(&self, value: &Value) -> Result<(), SchemaError> {
        let control = ValidationControl::default();
        let mut traversal = Traversal::new(&control);
        validate_value(Scope { base: &self.schema, documents: &self.documents, patterns: &self.patterns }, &self.schema, value, "$", &mut traversal)
    }
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

//#region 🧬️Compile

/// 🔗 Resolution context: the document a local `#/...` pointer walks, plus every sibling document a
/// cross-scope `$ref` may name by `$id`.
#[derive(Clone, Copy)]
struct Scope<'a> {
    base: &'a Value,
    documents: &'a HashMap<String, Value>,
    patterns: &'a HashMap<String, PatternMatcher>,
}

fn index_documents(documents: &[&str]) -> Result<HashMap<String, Value>, SchemaError> {
    let mut indexed = HashMap::new();
    for body in documents {
        let document = parse_json(body).map_err(|error| SchemaError::Validation(format!("invalid sibling schema JSON: {error}")))?;
        let id = document.get("$id").and_then(Value::as_str).ok_or_else(|| SchemaError::Validation("sibling schema document requires an `$id`".to_string()))?.to_string();
        if indexed.insert(id.clone(), document).is_some() {
            return Err(SchemaError::Validation(format!("duplicate sibling schema `$id` {id}")));
        }
    }
    Ok(indexed)
}

fn schema_object<'a>(schema: &'a Value, path: &str) -> Result<&'a Object, SchemaError> {
    schema.as_object().ok_or_else(|| SchemaError::Validation(format!("{path}: schema must be an object or boolean")))
}

fn validate_schema_node(scope: Scope<'_>, schema: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    traversal.visit()?;
    if schema.as_bool().is_some() {
        return Ok(());
    }
    let object = schema_object(schema, path)?;
    for (keyword, value) in object {
        let keyword_path = format!("{path}.{keyword}");
        match keyword {
            "$id" | "$schema" | "$anchor" | "$comment" | "title" | "description" | "contentMediaType" | "contentEncoding" => {
                let _ = require_string(value, &keyword_path)?;
            }
            "format" => {
                let _ = require_string(value, &keyword_path)?;
            }
            "$ref" => {
                let reference = require_string(value, &keyword_path)?;
                resolve_reference(scope, reference).map_err(|message| SchemaError::Validation(format!("{keyword_path}: {message}")))?;
            }
            "$defs" | "definitions" | "properties" => {
                let entries = value.as_object().ok_or_else(|| SchemaError::Validation(format!("{keyword_path}: expected object")))?;
                for (name, child) in entries {
                    validate_schema_node(scope, child, &format!("{keyword_path}.{name}"), traversal)?;
                }
            }
            "type" => validate_type_keyword(value, &keyword_path)?,
            "required" => validate_string_set(value, &keyword_path)?,
            "additionalProperties" | "additionalItems" | "not" | "if" | "then" | "else" | "propertyNames" | "contains" => validate_schema_node(scope, value, &keyword_path, traversal)?,
            "patternProperties" => {
                let entries = value.as_object().ok_or_else(|| SchemaError::Validation(format!("{keyword_path}: expected object")))?;
                for (pattern, child) in entries {
                    let child_path = format!("{keyword_path}.{pattern}");
                    let matcher = PatternMatcher::compile(pattern).map_err(|reason| SchemaError::Validation(format!("{child_path}: {reason}")))?;
                    traversal.patterns.insert(pattern.to_string(), matcher);
                    validate_schema_node(scope, child, &child_path, traversal)?;
                }
            }
            "dependencies" => {
                let entries = value.as_object().ok_or_else(|| SchemaError::Validation(format!("{keyword_path}: expected object")))?;
                for (name, child) in entries {
                    let child_path = format!("{keyword_path}.{name}");
                    match child.as_array() {
                        Some(_) => validate_string_set(child, &child_path)?,
                        None => validate_schema_node(scope, child, &child_path, traversal)?,
                    }
                }
            }
            "items" => match value.as_array() {
                Some(entries) => {
                    for (index, entry) in entries.iter().enumerate() {
                        validate_schema_node(scope, entry, &format!("{keyword_path}[{index}]"), traversal)?;
                    }
                }
                None => validate_schema_node(scope, value, &keyword_path, traversal)?,
            },
            "enum" => validate_enum_keyword(value, &keyword_path)?,
            "const" | "default" => {}
            "pattern" => {
                let pattern = require_string(value, &keyword_path)?;
                let matcher = PatternMatcher::compile(pattern).map_err(|reason| SchemaError::Validation(format!("{keyword_path}: {reason}")))?;
                traversal.patterns.insert(pattern.to_string(), matcher);
            }
            "examples" => {
                if value.as_array().is_none() {
                    return Err(SchemaError::Validation(format!("{keyword_path}: expected array")));
                }
            }
            "allOf" | "anyOf" | "oneOf" => {
                let branches = value.as_array().filter(|branches| !branches.is_empty()).ok_or_else(|| SchemaError::Validation(format!("{keyword_path}: expected non-empty array")))?;
                for (index, branch) in branches.iter().enumerate() {
                    validate_schema_node(scope, branch, &format!("{keyword_path}[{index}]"), traversal)?;
                }
            }
            "minItems" | "maxItems" | "minLength" | "maxLength" | "minProperties" | "maxProperties" => {
                let _ = schema_usize(value, &keyword_path)?;
            }
            "uniqueItems" | "readOnly" | "writeOnly" | "deprecated" => {
                if value.as_bool().is_none() {
                    return Err(SchemaError::Validation(format!("{keyword_path}: expected boolean")));
                }
            }
            "minimum" | "maximum" | "exclusiveMinimum" | "exclusiveMaximum" => {
                let _ = schema_number(value, &keyword_path)?;
            }
            "multipleOf" => {
                if schema_number(value, &keyword_path)? <= 0.0 {
                    return Err(SchemaError::Validation(format!("{keyword_path}: expected a positive number")));
                }
            }
            extension if extension.starts_with("x-") => {}
            unsupported => return Err(SchemaError::Validation(format!("{path}: unsupported JSON Schema keyword `{unsupported}`"))),
        }
    }
    Ok(())
}

fn require_string<'a>(value: &'a Value, path: &str) -> Result<&'a str, SchemaError> {
    value.as_str().ok_or_else(|| SchemaError::Validation(format!("{path}: expected string")))
}

fn schema_usize(value: &Value, path: &str) -> Result<usize, SchemaError> {
    value.as_u64().and_then(|value| usize::try_from(value).ok()).ok_or_else(|| SchemaError::Validation(format!("{path}: expected non-negative integer")))
}

fn schema_number(value: &Value, path: &str) -> Result<f64, SchemaError> {
    value.as_f64().filter(|value| value.is_finite()).ok_or_else(|| SchemaError::Validation(format!("{path}: expected finite number")))
}

fn validate_type_keyword(value: &Value, path: &str) -> Result<(), SchemaError> {
    let valid = |name: &str| matches!(name, "null" | "boolean" | "object" | "array" | "string" | "integer" | "number");
    if value.as_str().is_some_and(valid) || value.as_array().is_some_and(|types| !types.is_empty() && types.iter().all(|entry| entry.as_str().is_some_and(valid))) {
        return Ok(());
    }
    Err(SchemaError::Validation(format!("{path}: unsupported JSON Schema type")))
}

fn validate_string_set(value: &Value, path: &str) -> Result<(), SchemaError> {
    let entries = value.as_array().ok_or_else(|| SchemaError::Validation(format!("{path}: expected array")))?;
    let mut seen = std::collections::HashSet::new();
    for entry in entries {
        let entry = entry.as_str().ok_or_else(|| SchemaError::Validation(format!("{path}: expected only strings")))?;
        if !seen.insert(entry) {
            return Err(SchemaError::Validation(format!("{path}: duplicate string `{entry}`")));
        }
    }
    Ok(())
}

fn validate_enum_keyword(value: &Value, path: &str) -> Result<(), SchemaError> {
    let entries = value.as_array().filter(|entries| !entries.is_empty()).ok_or_else(|| SchemaError::Validation(format!("{path}: expected non-empty array")))?;
    for (index, entry) in entries.iter().enumerate() {
        if entries[..index].iter().any(|previous| values_equal(previous, entry)) {
            return Err(SchemaError::Validation(format!("{path}: duplicate enum value")));
        }
    }
    Ok(())
}

/// 🔗 Resolves `$ref` to `(document it lives in, referenced node)` — `#`/`#/…` and the document's
/// own `$id` inside the current document, another `<$id>`/`<$id>#/…` into a registered sibling.
fn resolve_reference<'a>(scope: Scope<'a>, reference: &str) -> Result<(&'a Value, &'a Value), String> {
    let (document_id, fragment) = match reference.split_once('#') {
        Some((document_id, fragment)) => (document_id, fragment),
        None => (reference, ""),
    };
    let base = if document_id.is_empty() || scope.base.get("$id").and_then(Value::as_str) == Some(document_id) {
        scope.base
    } else {
        scope.documents.get(document_id).ok_or_else(|| format!("unresolved cross-document reference `{reference}`"))?
    };
    if fragment.is_empty() {
        return Ok((base, base));
    }
    let pointer = fragment.strip_prefix('/').ok_or_else(|| format!("only JSON Pointer fragments are supported, found `{reference}`"))?;
    let mut current = base;
    for raw in pointer.split('/') {
        let segment = raw.replace("~1", "/").replace("~0", "~");
        current = match current {
            Value::Object(object) => object.get(&segment),
            Value::Array(items) => segment.parse::<usize>().ok().and_then(|index| items.get(index)),
            _ => None,
        }
        .ok_or_else(|| format!("unresolved reference `{reference}`"))?;
    }
    Ok((base, current))
}

//#endregion 🧬️Compile

//#region ✅️Validate

fn validate_value(scope: Scope<'_>, schema: &Value, value: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    traversal.visit()?;
    if let Some(allowed) = schema.as_bool() {
        return if allowed { Ok(()) } else { Err(SchemaError::Validation(format!("{path}: rejected by false schema"))) };
    }
    let key = (schema as *const Value as usize, value as *const Value as usize);
    if traversal.active.contains(&key) {
        return Ok(());
    }
    traversal.active.push(key);
    let result = validate_value_inner(scope, schema_object(schema, path)?, value, path, traversal);
    traversal.active.pop();
    result
}

fn validate_value_inner(scope: Scope<'_>, schema: &Object, value: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        let (base, referenced) = resolve_reference(scope, reference).map_err(|message| SchemaError::Validation(format!("{path}.$ref: {message}")))?;
        validate_value(Scope { base, documents: scope.documents, patterns: scope.patterns }, referenced, value, path, traversal)?;
    }
    if schema.get("enum").and_then(Value::as_array).is_some_and(|allowed| !allowed.iter().any(|allowed| values_equal(allowed, value))) {
        return Err(SchemaError::Validation(format!("{path}: value is not in enum")));
    }
    if schema.get("const").is_some_and(|expected| !values_equal(expected, value)) {
        return Err(SchemaError::Validation(format!("{path}: value does not equal const")));
    }
    if let Some(expected) = schema.get("type") {
        let matched = expected.as_str().is_some_and(|expected| value_matches_type(value, expected)) || expected.as_array().is_some_and(|types| types.iter().filter_map(Value::as_str).any(|expected| value_matches_type(value, expected)));
        if !matched {
            let label = expected.as_str().map_or_else(|| "one of the declared types".to_string(), str::to_string);
            return Err(SchemaError::Validation(format!("{path}: expected {label}")));
        }
    }
    if let Some(branches) = schema.get("allOf").and_then(Value::as_array) {
        for branch in branches {
            validate_value(scope, branch, value, path, traversal)?;
        }
    }
    if let Some(branches) = schema.get("anyOf").and_then(Value::as_array) {
        let mut matched = false;
        for branch in branches {
            matched |= branch_matches(validate_value(scope, branch, value, path, traversal))?;
        }
        if !matched {
            return Err(SchemaError::Validation(format!("{path}: no anyOf branch matched")));
        }
    }
    if let Some(branches) = schema.get("oneOf").and_then(Value::as_array) {
        let mut matches = 0usize;
        for branch in branches {
            matches += usize::from(branch_matches(validate_value(scope, branch, value, path, traversal))?);
        }
        if matches != 1 {
            return Err(SchemaError::Validation(format!("{path}: expected exactly one matching oneOf branch, found {matches}")));
        }
    }
    if let Some(disallowed) = schema.get("not") {
        if branch_matches(validate_value(scope, disallowed, value, path, traversal))? {
            return Err(SchemaError::Validation(format!("{path}: matched disallowed schema")));
        }
    }
    if let Some(condition) = schema.get("if") {
        let taken = if branch_matches(validate_value(scope, condition, value, path, traversal))? { schema.get("then") } else { schema.get("else") };
        if let Some(branch) = taken {
            validate_value(scope, branch, value, path, traversal)?;
        }
    }
    validate_object(scope, schema, value, path, traversal)?;
    validate_array(scope, schema, value, path, traversal)?;
    validate_string(scope, schema, value, path)?;
    validate_number(schema, value, path)?;
    Ok(())
}

fn branch_matches(result: Result<(), SchemaError>) -> Result<bool, SchemaError> {
    match result {
        Ok(()) => Ok(true),
        Err(SchemaError::Validation(_)) => Ok(false),
        Err(error) => Err(error),
    }
}

fn value_matches_type(value: &Value, expected: &str) -> bool {
    match expected {
        "null" => value.is_null(),
        "boolean" => value.as_bool().is_some(),
        "object" => value.as_object().is_some(),
        "array" => value.as_array().is_some(),
        "string" => value.as_str().is_some(),
        "integer" => value.as_number().is_some_and(|number| match number {
            Number::UInt(_) | Number::Int(_) => true,
            Number::Float(number) => number.is_finite() && number.fract() == 0.0,
        }),
        "number" => value.as_number().is_some(),
        _ => false,
    }
}

fn validate_object(scope: Scope<'_>, schema: &Object, value: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    let Some(object) = value.as_object() else { return Ok(()) };
    let properties = schema.get("properties").and_then(Value::as_object);
    let pattern_properties = schema.get("patternProperties").and_then(Value::as_object);
    if let Some(minimum) = schema.get("minProperties") {
        let minimum = schema_usize(minimum, &format!("{path}.minProperties"))?;
        if object.len() < minimum {
            return Err(SchemaError::Validation(format!("{path}: expected at least {minimum} properties")));
        }
    }
    if let Some(maximum) = schema.get("maxProperties") {
        let maximum = schema_usize(maximum, &format!("{path}.maxProperties"))?;
        if object.len() > maximum {
            return Err(SchemaError::Validation(format!("{path}: expected at most {maximum} properties")));
        }
    }
    if let Some(required) = schema.get("required").and_then(Value::as_array) {
        for key in required.iter().filter_map(Value::as_str) {
            if !object.contains_key(key) {
                return Err(SchemaError::Validation(format!("{path}: missing required property `{key}`")));
            }
        }
    }
    if let Some(names) = schema.get("propertyNames") {
        for (key, _) in object {
            if !branch_matches(validate_value(scope, names, &Value::String(key.to_string()), path, traversal))? {
                return Err(SchemaError::Validation(format!("{path}: property name `{key}` is rejected by propertyNames")));
            }
        }
    }
    if let Some(properties) = properties {
        for (key, property_schema) in properties {
            if let Some(property_value) = object.get(key) {
                validate_value(scope, property_schema, property_value, &format!("{path}.{key}"), traversal)?;
            }
        }
    }
    if let Some(pattern_properties) = pattern_properties {
        for (pattern, property_schema) in pattern_properties {
            for (key, property_value) in object {
                if pattern_matches(scope, pattern, key, &format!("{path}.patternProperties"))? {
                    validate_value(scope, property_schema, property_value, &format!("{path}.{key}"), traversal)?;
                }
            }
        }
    }
    if let Some(additional) = schema.get("additionalProperties") {
        for (key, property_value) in object {
            if properties.is_some_and(|properties| properties.contains_key(key)) {
                continue;
            }
            let matched_pattern = match pattern_properties {
                Some(pattern_properties) => {
                    let mut matched = false;
                    for (pattern, _) in pattern_properties {
                        matched |= pattern_matches(scope, pattern, key, &format!("{path}.patternProperties"))?;
                    }
                    matched
                }
                None => false,
            };
            if matched_pattern {
                continue;
            }
            match additional.as_bool() {
                Some(true) => {}
                Some(false) => return Err(SchemaError::Validation(format!("{path}: additional property `{key}` is not allowed"))),
                None => validate_value(scope, additional, property_value, &format!("{path}.{key}"), traversal)?,
            }
        }
    }
    if let Some(dependencies) = schema.get("dependencies").and_then(Value::as_object) {
        for (key, dependency) in dependencies {
            if !object.contains_key(key) {
                continue;
            }
            match dependency.as_array() {
                Some(names) => {
                    for name in names.iter().filter_map(Value::as_str) {
                        if !object.contains_key(name) {
                            return Err(SchemaError::Validation(format!("{path}: property `{key}` requires `{name}`")));
                        }
                    }
                }
                None => validate_value(scope, dependency, value, path, traversal)?,
            }
        }
    }
    Ok(())
}

fn validate_array(scope: Scope<'_>, schema: &Object, value: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    let Some(items) = value.as_array() else { return Ok(()) };
    if let Some(minimum) = schema.get("minItems") {
        let minimum = schema_usize(minimum, &format!("{path}.minItems"))?;
        if items.len() < minimum {
            return Err(SchemaError::Validation(format!("{path}: expected at least {minimum} items")));
        }
    }
    if let Some(maximum) = schema.get("maxItems") {
        let maximum = schema_usize(maximum, &format!("{path}.maxItems"))?;
        if items.len() > maximum {
            return Err(SchemaError::Validation(format!("{path}: expected at most {maximum} items")));
        }
    }
    match schema.get("items").map(|item_schema| (item_schema.as_array(), item_schema)) {
        Some((Some(tuple), _)) => {
            for (index, item) in items.iter().enumerate() {
                match tuple.get(index) {
                    Some(item_schema) => validate_value(scope, item_schema, item, &format!("{path}[{index}]"), traversal)?,
                    None => match schema.get("additionalItems") {
                        Some(additional) if additional.as_bool() == Some(false) => return Err(SchemaError::Validation(format!("{path}[{index}]: additional item is not allowed"))),
                        Some(additional) if additional.as_bool().is_none() => validate_value(scope, additional, item, &format!("{path}[{index}]"), traversal)?,
                        _ => {}
                    },
                }
            }
        }
        Some((None, item_schema)) => {
            for (index, item) in items.iter().enumerate() {
                validate_value(scope, item_schema, item, &format!("{path}[{index}]"), traversal)?;
            }
        }
        None => {}
    }
    if let Some(contained) = schema.get("contains") {
        let mut matched = false;
        for (index, item) in items.iter().enumerate() {
            matched |= branch_matches(validate_value(scope, contained, item, &format!("{path}[{index}]"), traversal))?;
        }
        if !matched {
            return Err(SchemaError::Validation(format!("{path}: no item matches contains")));
        }
    }
    if schema.get("uniqueItems").and_then(Value::as_bool) == Some(true) {
        for index in 0..items.len() {
            for previous in &items[..index] {
                traversal.visit()?;
                if values_equal(previous, &items[index]) {
                    return Err(SchemaError::Validation(format!("{path}: array items must be unique")));
                }
            }
        }
    }
    Ok(())
}

fn validate_string(scope: Scope<'_>, schema: &Object, value: &Value, path: &str) -> Result<(), SchemaError> {
    let Some(value) = value.as_str() else { return Ok(()) };
    let length = value.chars().count();
    for (keyword, violated) in
        [("minLength", schema.get("minLength").and_then(Value::as_u64).is_some_and(|minimum| length < minimum as usize)), ("maxLength", schema.get("maxLength").and_then(Value::as_u64).is_some_and(|maximum| length > maximum as usize))]
    {
        if violated {
            return Err(SchemaError::Validation(format!("{path}: string violates {keyword}")));
        }
    }
    if let Some(pattern) = schema.get("pattern").and_then(Value::as_str) {
        if !pattern_matches(scope, pattern, value, &format!("{path}.pattern"))? {
            return Err(SchemaError::Validation(format!("{path}: string does not match pattern")));
        }
    }
    if let Some(format) = schema.get("format").and_then(Value::as_str) {
        if string_format_matches(format, value) == Some(false) {
            return Err(SchemaError::Validation(format!("{path}: string is not a valid {format}")));
        }
    }
    Ok(())
}

fn pattern_matches(scope: Scope<'_>, pattern: &str, text: &str, path: &str) -> Result<bool, SchemaError> {
    match scope.patterns.get(pattern) {
        Some(matcher) => Ok(matcher.is_match(text)),
        None => PatternMatcher::compile(pattern).map(|matcher| matcher.is_match(text)).map_err(|reason| SchemaError::Validation(format!("{path}: {reason}"))),
    }
}

fn validate_number(schema: &Object, value: &Value, path: &str) -> Result<(), SchemaError> {
    let Some(number) = value.as_f64() else { return Ok(()) };
    for (keyword, accepted) in [
        ("minimum", schema.get("minimum").map(|bound| number >= bound.as_f64().unwrap_or(f64::INFINITY))),
        ("maximum", schema.get("maximum").map(|bound| number <= bound.as_f64().unwrap_or(f64::NEG_INFINITY))),
        ("exclusiveMinimum", schema.get("exclusiveMinimum").map(|bound| number > bound.as_f64().unwrap_or(f64::INFINITY))),
        ("exclusiveMaximum", schema.get("exclusiveMaximum").map(|bound| number < bound.as_f64().unwrap_or(f64::NEG_INFINITY))),
    ] {
        if accepted == Some(false) {
            return Err(SchemaError::Validation(format!("{path}: number violates {keyword}")));
        }
    }
    if let Some(divisor) = schema.get("multipleOf").and_then(Value::as_f64) {
        let quotient = number / divisor;
        let tolerance = f64::EPSILON * quotient.abs().max(1.0) * 8.0;
        if (quotient - quotient.round()).abs() > tolerance {
            return Err(SchemaError::Validation(format!("{path}: number is not a multiple of {divisor}")));
        }
    }
    Ok(())
}

//#endregion ✅️Validate

//#region 🏷️Format

/// 🏷️ The pinned `format` policy of this validator. Exactly these seven values are **assertions**,
/// implemented in-house and verified against the `ajv-formats` "full" oracle by the shared vector
/// corpus. Every other `format` value — including the `double`, `float`, `int64`, `uint32` and
/// `base64` spellings the repo's proto-derived modules carry — is an **annotation** that never
/// rejects an instance.
///
/// `regex` is asserted against the owned [`PatternMatcher`] subset, which is deliberately narrower
/// than ECMA-262: backreferences, lookbehind and `\b` are rejected instead of accepted.
/// @see <https://datatracker.ietf.org/doc/html/rfc3339>
/// @see <https://ajv.js.org/guide/formats.html>
pub const ASSERTED_STRING_FORMATS: [&str; 7] = ["date", "date-time", "email", "regex", "time", "uri", "uuid"];

/// 🔎 Applies the pinned policy to one string: `Some(verdict)` for an asserted format, `None` when
/// the format is annotation-only and therefore carries no constraint at all.
pub fn string_format_matches(format: &str, value: &str) -> Option<bool> {
    match format {
        "date" => Some(is_rfc3339_date(value)),
        "date-time" => Some(is_rfc3339_date_time(value)),
        "time" => Some(is_rfc3339_time(value)),
        "email" => Some(is_email(value)),
        "uri" => Some(is_uri(value)),
        "uuid" => Some(is_uuid(value)),
        "regex" => Some(PatternMatcher::compile(value).is_ok()),
        _ => None,
    }
}

fn split_digits(value: &str, width: usize) -> Option<(u32, &str)> {
    let head = value.get(..width)?;
    if !head.chars().all(|entry| entry.is_ascii_digit()) {
        return None;
    }
    Some((head.parse::<u32>().ok()?, &value[width..]))
}

fn days_in_month(year: u32, month: u32) -> u32 {
    const DAYS: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if month == 2 && year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) {
        29
    } else {
        DAYS[month as usize]
    }
}

fn is_rfc3339_date(value: &str) -> bool {
    let Some((year, rest)) = split_digits(value, 4) else { return false };
    let Some(rest) = rest.strip_prefix('-') else { return false };
    let Some((month, rest)) = split_digits(rest, 2) else { return false };
    let Some(rest) = rest.strip_prefix('-') else { return false };
    let Some((day, rest)) = split_digits(rest, 2) else { return false };
    rest.is_empty() && (1..=12).contains(&month) && day >= 1 && day <= days_in_month(year, month)
}

fn is_rfc3339_time(value: &str) -> bool {
    let Some((hour, rest)) = split_digits(value, 2) else { return false };
    let Some(rest) = rest.strip_prefix(':') else { return false };
    let Some((minute, rest)) = split_digits(rest, 2) else { return false };
    let Some(rest) = rest.strip_prefix(':') else { return false };
    let Some((second, rest)) = split_digits(rest, 2) else { return false };
    let rest = match rest.strip_prefix('.') {
        Some(fraction) => {
            let digits = fraction.chars().take_while(|entry| entry.is_ascii_digit()).count();
            if digits == 0 {
                return false;
            }
            &fraction[digits..]
        }
        None => rest,
    };
    let (offset_hour, offset_minute, sign) = if rest.eq_ignore_ascii_case("z") {
        (0, 0, 1i64)
    } else {
        let (sign, rest) = match rest.as_bytes().first() {
            Some(b'+') => (1i64, &rest[1..]),
            Some(b'-') => (-1i64, &rest[1..]),
            _ => return false,
        };
        let Some((offset_hour, rest)) = split_digits(rest, 2) else { return false };
        if rest.is_empty() {
            (offset_hour, 0, sign)
        } else {
            let Some((offset_minute, tail)) = split_digits(rest.strip_prefix(':').unwrap_or(rest), 2) else { return false };
            if !tail.is_empty() {
                return false;
            }
            (offset_hour, offset_minute, sign)
        }
    };
    if offset_hour > 23 || offset_minute > 59 {
        return false;
    }
    if hour <= 23 && minute <= 59 && second < 60 {
        return true;
    }
    let utc_minute = i64::from(minute) - i64::from(offset_minute) * sign;
    let utc_hour = i64::from(hour) - i64::from(offset_hour) * sign - i64::from(utc_minute < 0);
    (utc_hour == 23 || utc_hour == -1) && (utc_minute == 59 || utc_minute == -1) && second < 61
}

fn is_rfc3339_date_time(value: &str) -> bool {
    let separators: Vec<(usize, char)> = value.char_indices().filter(|(_, entry)| *entry == 't' || *entry == 'T' || entry.is_whitespace()).collect();
    let [(index, separator)] = separators[..] else { return false };
    is_rfc3339_date(&value[..index]) && is_rfc3339_time(&value[index + separator.len_utf8()..])
}

const EMAIL_ATOM_EXTRA: &str = "!#$%&'*+/=?^_`{|}~-";

fn is_email_atom(atom: &str) -> bool {
    !atom.is_empty() && atom.chars().all(|entry| entry.is_ascii_alphanumeric() || EMAIL_ATOM_EXTRA.contains(entry))
}

fn is_domain_label(label: &str) -> bool {
    let boundaries_are_alphanumeric = label.chars().next().is_some_and(|entry| entry.is_ascii_alphanumeric()) && label.chars().next_back().is_some_and(|entry| entry.is_ascii_alphanumeric());
    boundaries_are_alphanumeric && label.chars().all(|entry| entry.is_ascii_alphanumeric() || entry == '-')
}

fn is_email(value: &str) -> bool {
    let Some((local, domain)) = value.split_once('@') else { return false };
    let labels: Vec<&str> = domain.split('.').collect();
    local.split('.').all(is_email_atom) && labels.len() >= 2 && labels.iter().all(|label| is_domain_label(label))
}

fn is_uuid(value: &str) -> bool {
    let value = match value.get(..9) {
        Some(prefix) if prefix.eq_ignore_ascii_case("urn:uuid:") => &value[9..],
        _ => value,
    };
    let groups: Vec<&str> = value.split('-').collect();
    groups.len() == 5 && [8usize, 4, 4, 4, 12].iter().zip(&groups).all(|(width, group)| group.len() == *width && group.chars().all(|entry| entry.is_ascii_hexdigit()))
}

fn is_pct_encoded_set(text: &str, extra: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '%' {
            if index + 2 >= chars.len() || !chars[index + 1].is_ascii_hexdigit() || !chars[index + 2].is_ascii_hexdigit() {
                return false;
            }
            index += 3;
            continue;
        }
        if !chars[index].is_ascii_alphanumeric() && !extra.contains(chars[index]) {
            return false;
        }
        index += 1;
    }
    true
}

fn is_uri_scheme(scheme: &str) -> bool {
    let mut chars = scheme.chars();
    chars.next().is_some_and(|first| first.is_ascii_alphabetic()) && chars.all(|entry| entry.is_ascii_alphanumeric() || matches!(entry, '+' | '-' | '.'))
}

fn is_uri_authority(authority: &str) -> bool {
    let (userinfo, host) = match authority.rsplit_once('@') {
        Some((userinfo, host)) => (Some(userinfo), host),
        None => (None, authority),
    };
    if userinfo.is_some_and(|userinfo| !is_pct_encoded_set(userinfo, "-._~!$&'()*+,;=:")) {
        return false;
    }
    let host = match host.rsplit_once(':') {
        Some((head, port)) if port.chars().all(|entry| entry.is_ascii_digit()) => head,
        _ => host,
    };
    match host.strip_prefix('[') {
        Some(literal) => literal.ends_with(']') && literal[..literal.len() - 1].chars().all(|entry| entry.is_ascii_hexdigit() || matches!(entry, ':' | '.')),
        None => is_pct_encoded_set(host, "-._~!$&'()*+,;="),
    }
}

/// 🔗 RFC 3986 absolute URI with an optional fragment — a scheme is mandatory, which is exactly the
/// distinction draft-07 draws between `uri` and `uri-reference`.
fn is_uri(value: &str) -> bool {
    let Some((scheme, rest)) = value.split_once(':') else { return false };
    if !is_uri_scheme(scheme) {
        return false;
    }
    let (rest, fragment) = match rest.split_once('#') {
        Some((rest, fragment)) => (rest, Some(fragment)),
        None => (rest, None),
    };
    let (rest, query) = match rest.split_once('?') {
        Some((rest, query)) => (rest, Some(query)),
        None => (rest, None),
    };
    if fragment.is_some_and(|fragment| !is_pct_encoded_set(fragment, "-._~!$&'()*+,;=:@/?")) || query.is_some_and(|query| !is_pct_encoded_set(query, "-._~!$&'()*+,;=:@/?")) {
        return false;
    }
    let path_is_valid = |path: &str| path.split('/').all(|segment| is_pct_encoded_set(segment, "-._~!$&'()*+,;=:@"));
    match rest.strip_prefix("//") {
        Some(rest) => {
            let (authority, path) = match rest.find('/') {
                Some(index) => rest.split_at(index),
                None => (rest, ""),
            };
            is_uri_authority(authority) && path_is_valid(path)
        }
        None => !rest.is_empty() && path_is_valid(rest),
    }
}

//#endregion 🏷️Format

//#region 🔤️Pattern

/// 🔤 Owned backtracking matcher for the ECMA-262 subset JSON Schema `pattern` needs — anchors,
/// classes, groups, lookahead, alternation and quantifiers over `char`s.
///
/// Not reusing `semio-framework-math`'s `🎯️sampling` DFA: that engine is byte-level with no anchors
/// and no `\d`/`\w`/`\s` shorthand, exposes an `async` per-byte `step`, and would invert the layering
/// by making the boundary schema module depend on the sampling crate.
/// See <https://262.ecma-international.org/#sec-patterns>.
#[derive(Clone, Debug)]
pub struct PatternMatcher {
    node: PatternNode,
}

#[derive(Clone, Debug)]
enum PatternNode {
    Empty,
    Literal(char),
    Any,
    Class { negated: bool, items: Vec<ClassItem> },
    Start,
    End,
    Concat(Vec<PatternNode>),
    Alternate(Vec<PatternNode>),
    Repeat(Box<Repeat>),
    Look { negated: bool, node: Box<PatternNode> },
}

#[derive(Clone, Debug)]
struct Repeat {
    node: PatternNode,
    min: usize,
    max: Option<usize>,
    greedy: bool,
}

#[derive(Clone, Copy, Debug)]
enum ClassItem {
    Literal(char),
    Range(char, char),
    Digit(bool),
    Word(bool),
    Space(bool),
}

impl PatternMatcher {
    /// 📥 Compiles one `pattern` string, rejecting constructs outside the supported subset.
    pub fn compile(pattern: &str) -> Result<Self, String> {
        let chars: Vec<char> = pattern.chars().collect();
        let mut parser = PatternParser { chars: &chars, position: 0 };
        let node = parser.parse_alternate()?;
        if parser.position != chars.len() {
            return Err(format!("unbalanced pattern at offset {}", parser.position));
        }
        Ok(Self { node })
    }

    /// 🔎 Whether the pattern matches anywhere in `text`, the unanchored JSON Schema semantics.
    pub fn is_match(&self, text: &str) -> bool {
        let input: Vec<char> = text.chars().collect();
        (0..=input.len()).any(|start| match_node(&self.node, &input, start, &mut |_| true))
    }
}

struct PatternParser<'a> {
    chars: &'a [char],
    position: usize,
}

impl PatternParser<'_> {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn parse_alternate(&mut self) -> Result<PatternNode, String> {
        let mut branches = vec![self.parse_concat()?];
        while self.peek() == Some('|') {
            self.position += 1;
            branches.push(self.parse_concat()?);
        }
        Ok(if branches.len() == 1 { branches.remove(0) } else { PatternNode::Alternate(branches) })
    }

    fn parse_concat(&mut self) -> Result<PatternNode, String> {
        let mut nodes = Vec::new();
        while !matches!(self.peek(), None | Some('|') | Some(')')) {
            nodes.push(self.parse_quantified()?);
        }
        Ok(match nodes.len() {
            0 => PatternNode::Empty,
            1 => nodes.remove(0),
            _ => PatternNode::Concat(nodes),
        })
    }

    fn parse_quantified(&mut self) -> Result<PatternNode, String> {
        let node = self.parse_atom()?;
        let (min, max) = match self.peek() {
            Some('*') => {
                self.position += 1;
                (0, None)
            }
            Some('+') => {
                self.position += 1;
                (1, None)
            }
            Some('?') => {
                self.position += 1;
                (0, Some(1))
            }
            Some('{') => match self.parse_bounds()? {
                Some(bounds) => bounds,
                None => return Ok(node),
            },
            _ => return Ok(node),
        };
        let greedy = if self.peek() == Some('?') {
            self.position += 1;
            false
        } else {
            true
        };
        Ok(PatternNode::Repeat(Box::new(Repeat { node, min, max, greedy })))
    }

    fn parse_bounds(&mut self) -> Result<Option<(usize, Option<usize>)>, String> {
        let open = self.position;
        let Some(close) = self.chars[open..].iter().position(|entry| *entry == '}').map(|offset| open + offset) else {
            return Ok(None);
        };
        let body: String = self.chars[open + 1..close].iter().collect();
        if body.is_empty() || !body.chars().all(|entry| entry.is_ascii_digit() || entry == ',') {
            return Ok(None);
        }
        let bounds = match body.split_once(',') {
            None => {
                let exact = body.parse::<usize>().map_err(|_| format!("invalid repeat bound `{body}`"))?;
                (exact, Some(exact))
            }
            Some((minimum, "")) => (minimum.parse::<usize>().map_err(|_| format!("invalid repeat bound `{body}`"))?, None),
            Some((minimum, maximum)) => (minimum.parse::<usize>().map_err(|_| format!("invalid repeat bound `{body}`"))?, Some(maximum.parse::<usize>().map_err(|_| format!("invalid repeat bound `{body}`"))?)),
        };
        if bounds.1.is_some_and(|maximum| maximum < bounds.0) {
            return Err(format!("descending repeat bound `{body}`"));
        }
        self.position = close + 1;
        Ok(Some(bounds))
    }

    fn parse_atom(&mut self) -> Result<PatternNode, String> {
        let Some(current) = self.peek() else { return Err("unexpected end of pattern".to_string()) };
        self.position += 1;
        match current {
            '^' => Ok(PatternNode::Start),
            '$' => Ok(PatternNode::End),
            '.' => Ok(PatternNode::Any),
            '(' => {
                let look = match (self.peek(), self.chars.get(self.position + 1)) {
                    (Some('?'), Some(':')) => {
                        self.position += 2;
                        None
                    }
                    (Some('?'), Some('=')) => {
                        self.position += 2;
                        Some(false)
                    }
                    (Some('?'), Some('!')) => {
                        self.position += 2;
                        Some(true)
                    }
                    (Some('?'), _) => return Err("lookbehind and named groups are outside the supported subset".to_string()),
                    _ => None,
                };
                let inner = self.parse_alternate()?;
                if self.peek() != Some(')') {
                    return Err("unclosed group".to_string());
                }
                self.position += 1;
                Ok(match look {
                    Some(negated) => PatternNode::Look { negated, node: Box::new(inner) },
                    None => inner,
                })
            }
            '[' => self.parse_class(),
            '\\' => self.parse_escape(),
            ')' | ']' => Err(format!("unbalanced `{current}`")),
            '*' | '+' | '?' => Err(format!("quantifier `{current}` has nothing to repeat")),
            literal => Ok(PatternNode::Literal(literal)),
        }
    }

    fn parse_class(&mut self) -> Result<PatternNode, String> {
        let negated = if self.peek() == Some('^') {
            self.position += 1;
            true
        } else {
            false
        };
        let mut items = Vec::new();
        loop {
            let Some(current) = self.peek() else { return Err("unclosed character class".to_string()) };
            self.position += 1;
            if current == ']' {
                break;
            }
            let item = if current == '\\' {
                match self.parse_escape()? {
                    PatternNode::Literal(literal) => ClassItem::Literal(literal),
                    PatternNode::Class { negated: shorthand_negated, items: shorthand } => {
                        items.extend(shorthand.into_iter().map(|item| match (item, shorthand_negated) {
                            (ClassItem::Digit(polarity), true) => ClassItem::Digit(!polarity),
                            (ClassItem::Word(polarity), true) => ClassItem::Word(!polarity),
                            (ClassItem::Space(polarity), true) => ClassItem::Space(!polarity),
                            (item, _) => item,
                        }));
                        continue;
                    }
                    _ => return Err("unsupported escape inside a character class".to_string()),
                }
            } else {
                ClassItem::Literal(current)
            };
            let ClassItem::Literal(low) = item else {
                items.push(item);
                continue;
            };
            if self.peek() == Some('-') && self.chars.get(self.position + 1).is_some_and(|next| *next != ']') {
                self.position += 1;
                let Some(high) = self.peek() else { return Err("unclosed character class".to_string()) };
                self.position += 1;
                let high = if high == '\\' {
                    match self.parse_escape()? {
                        PatternNode::Literal(literal) => literal,
                        _ => return Err("unsupported escape as a class range bound".to_string()),
                    }
                } else {
                    high
                };
                if high < low {
                    return Err(format!("descending class range `{low}-{high}`"));
                }
                items.push(ClassItem::Range(low, high));
                continue;
            }
            items.push(ClassItem::Literal(low));
        }
        Ok(PatternNode::Class { negated, items })
    }

    fn parse_escape(&mut self) -> Result<PatternNode, String> {
        let Some(current) = self.peek() else { return Err("dangling escape".to_string()) };
        self.position += 1;
        let shorthand = |item: ClassItem| Ok(PatternNode::Class { negated: false, items: vec![item] });
        match current {
            'd' => shorthand(ClassItem::Digit(true)),
            'D' => shorthand(ClassItem::Digit(false)),
            'w' => shorthand(ClassItem::Word(true)),
            'W' => shorthand(ClassItem::Word(false)),
            's' => shorthand(ClassItem::Space(true)),
            'S' => shorthand(ClassItem::Space(false)),
            'n' => Ok(PatternNode::Literal('\n')),
            'r' => Ok(PatternNode::Literal('\r')),
            't' => Ok(PatternNode::Literal('\t')),
            'f' => Ok(PatternNode::Literal('\u{c}')),
            'v' => Ok(PatternNode::Literal('\u{b}')),
            '0' => Ok(PatternNode::Literal('\0')),
            'x' => self.parse_code_point(2),
            'u' => self.parse_code_point(4),
            digit if digit.is_ascii_digit() => Err("backreferences are outside the supported subset".to_string()),
            'b' | 'B' => Err("word boundaries are outside the supported subset".to_string()),
            literal => Ok(PatternNode::Literal(literal)),
        }
    }

    fn parse_code_point(&mut self, width: usize) -> Result<PatternNode, String> {
        let end = self.position + width;
        if end > self.chars.len() {
            return Err("truncated code point escape".to_string());
        }
        let digits: String = self.chars[self.position..end].iter().collect();
        let code = u32::from_str_radix(&digits, 16).map_err(|_| format!("invalid code point escape `{digits}`"))?;
        self.position = end;
        char::from_u32(code).map(PatternNode::Literal).ok_or_else(|| format!("invalid code point escape `{digits}`"))
    }
}

fn class_item_matches(item: ClassItem, value: char) -> bool {
    match item {
        ClassItem::Literal(literal) => literal == value,
        ClassItem::Range(low, high) => low <= value && value <= high,
        ClassItem::Digit(polarity) => value.is_ascii_digit() == polarity,
        ClassItem::Word(polarity) => (value.is_ascii_alphanumeric() || value == '_') == polarity,
        ClassItem::Space(polarity) => value.is_whitespace() == polarity,
    }
}

fn match_node(node: &PatternNode, input: &[char], position: usize, next: &mut dyn FnMut(usize) -> bool) -> bool {
    match node {
        PatternNode::Empty => next(position),
        PatternNode::Literal(literal) => input.get(position) == Some(literal) && next(position + 1),
        PatternNode::Any => input.get(position).is_some_and(|value| *value != '\n') && next(position + 1),
        PatternNode::Class { negated, items } => input.get(position).is_some_and(|value| items.iter().any(|item| class_item_matches(*item, *value)) != *negated) && next(position + 1),
        PatternNode::Start => position == 0 && next(position),
        PatternNode::End => position == input.len() && next(position),
        PatternNode::Concat(nodes) => match_concat(nodes, input, position, next),
        PatternNode::Alternate(branches) => branches.iter().any(|branch| match_node(branch, input, position, next)),
        PatternNode::Repeat(repeat) => match_repeat(repeat, input, position, 0, next),
        PatternNode::Look { negated, node } => match_node(node, input, position, &mut |_| true) != *negated && next(position),
    }
}

fn match_concat(nodes: &[PatternNode], input: &[char], position: usize, next: &mut dyn FnMut(usize) -> bool) -> bool {
    match nodes.split_first() {
        None => next(position),
        Some((head, rest)) => {
            let mut continuation = |resumed: usize| match_concat(rest, input, resumed, next);
            match_node(head, input, position, &mut continuation)
        }
    }
}

fn match_repeat(repeat: &Repeat, input: &[char], position: usize, count: usize, next: &mut dyn FnMut(usize) -> bool) -> bool {
    let can_stop = count >= repeat.min;
    let can_repeat = match repeat.max {
        Some(maximum) => count < maximum,
        None => true,
    };
    if !repeat.greedy && can_stop && next(position) {
        return true;
    }
    if can_repeat {
        let mut continuation = |resumed: usize| {
            if resumed == position {
                return count + 1 >= repeat.min && next(position);
            }
            match_repeat(repeat, input, resumed, count + 1, next)
        };
        if match_node(&repeat.node, input, position, &mut continuation) {
            return true;
        }
    }
    repeat.greedy && can_stop && next(position)
}

//#endregion 🔤️Pattern

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
