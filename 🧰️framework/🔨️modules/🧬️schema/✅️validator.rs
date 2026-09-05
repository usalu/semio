use crate::SchemaError;
use pack::json::{parse as parse_json, Number, Object, Value};
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
}

impl<'a> Traversal<'a> {
    fn new(control: &'a ValidationControl) -> Self {
        Self { control, visited_nodes: 0, active: Vec::new() }
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

#[derive(Clone)]
pub struct OwnedJsonSchemaValidator {
    schema: Value,
}

impl OwnedJsonSchemaValidator {
    /// 📥 Compiles the product-supported JSON Schema subset from an owned string boundary.
    pub fn compile(schema_json: &str) -> Result<Self, SchemaError> {
        Self::compile_with_control(schema_json, &ValidationControl::default()).map(|(validator, _)| validator)
    }

    /// 📊 Compiles with cooperative cancellation and deterministic node progress.
    pub fn compile_with_control(schema_json: &str, control: &ValidationControl) -> Result<(Self, ValidationProgress), SchemaError> {
        let schema = parse_json(schema_json).map_err(|error| SchemaError::Validation(format!("invalid schema JSON: {error}")))?;
        let mut traversal = Traversal::new(control);
        validate_schema_node(&schema, &schema, "$", &mut traversal)?;
        Ok((Self { schema }, traversal.progress()))
    }

    pub(crate) fn new(schema: &Value) -> Result<Self, SchemaError> {
        let control = ValidationControl::default();
        let mut traversal = Traversal::new(&control);
        validate_schema_node(schema, schema, "$", &mut traversal)?;
        Ok(Self { schema: schema.clone() })
    }

    /// ✅ Validates one JSON string and reports deterministic traversal progress.
    pub fn validate_json(&self, value_json: &str) -> Result<ValidationProgress, SchemaError> {
        self.validate_json_with_control(value_json, &ValidationControl::default())
    }

    /// 📊 Validates with cooperative cancellation and a bounded node traversal.
    pub fn validate_json_with_control(&self, value_json: &str, control: &ValidationControl) -> Result<ValidationProgress, SchemaError> {
        let value = parse_json(value_json).map_err(|error| SchemaError::Validation(format!("invalid instance JSON: {error}")))?;
        let mut traversal = Traversal::new(control);
        validate_value(&self.schema, &self.schema, &value, "$", &mut traversal)?;
        Ok(traversal.progress())
    }

    /// 🔎 Reports validity without exposing the internal JSON representation.
    pub fn is_valid_json(&self, value_json: &str) -> bool {
        self.validate_json(value_json).is_ok()
    }

    pub(crate) fn validate(&self, value: &Value) -> Result<(), SchemaError> {
        let control = ValidationControl::default();
        let mut traversal = Traversal::new(&control);
        validate_value(&self.schema, &self.schema, value, "$", &mut traversal)
    }
}

//#endregion 📋️Validator

//#region 🧬️Compile

fn schema_object<'a>(schema: &'a Value, path: &str) -> Result<&'a Object, SchemaError> {
    schema.as_object().ok_or_else(|| SchemaError::Validation(format!("{path}: schema must be an object or boolean")))
}

fn validate_schema_node(root: &Value, schema: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    traversal.visit()?;
    if schema.as_bool().is_some() {
        return Ok(());
    }
    let object = schema_object(schema, path)?;
    for (keyword, value) in object {
        let keyword_path = format!("{path}.{keyword}");
        match keyword {
            "$id" | "$schema" | "$anchor" | "$comment" | "title" | "description" | "format" => {
                let _ = require_string(value, &keyword_path)?;
            }
            "$ref" => {
                let reference = require_string(value, &keyword_path)?;
                resolve_local_ref(root, reference).map_err(|message| SchemaError::Validation(format!("{keyword_path}: {message}")))?;
            }
            "$defs" | "definitions" | "properties" => {
                let entries = value.as_object().ok_or_else(|| SchemaError::Validation(format!("{keyword_path}: expected object")))?;
                for (name, child) in entries {
                    validate_schema_node(root, child, &format!("{keyword_path}.{name}"), traversal)?;
                }
            }
            "type" => validate_type_keyword(value, &keyword_path)?,
            "required" => validate_string_set(value, &keyword_path)?,
            "additionalProperties" | "items" | "not" => validate_schema_node(root, value, &keyword_path, traversal)?,
            "enum" => validate_enum_keyword(value, &keyword_path)?,
            "const" | "default" => {}
            "examples" => {
                if value.as_array().is_none() {
                    return Err(SchemaError::Validation(format!("{keyword_path}: expected array")));
                }
            }
            "allOf" | "anyOf" | "oneOf" => {
                let branches = value.as_array().filter(|branches| !branches.is_empty()).ok_or_else(|| SchemaError::Validation(format!("{keyword_path}: expected non-empty array")))?;
                for (index, branch) in branches.iter().enumerate() {
                    validate_schema_node(root, branch, &format!("{keyword_path}[{index}]"), traversal)?;
                }
            }
            "minItems" | "maxItems" | "minLength" | "maxLength" => {
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

fn resolve_local_ref<'a>(root: &'a Value, reference: &str) -> Result<&'a Value, String> {
    if reference == "#" {
        return Ok(root);
    }
    let pointer = reference.strip_prefix("#/").ok_or_else(|| "only local JSON Pointer references are supported".to_string())?;
    let mut current = root;
    for raw in pointer.split('/') {
        let segment = raw.replace("~1", "/").replace("~0", "~");
        current = match current {
            Value::Object(object) => object.get(&segment),
            Value::Array(items) => segment.parse::<usize>().ok().and_then(|index| items.get(index)),
            _ => None,
        }
        .ok_or_else(|| format!("unresolved local reference `{reference}`"))?;
    }
    Ok(current)
}

//#endregion 🧬️Compile

//#region ✅️Validate

fn validate_value(root: &Value, schema: &Value, value: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    traversal.visit()?;
    if let Some(allowed) = schema.as_bool() {
        return if allowed { Ok(()) } else { Err(SchemaError::Validation(format!("{path}: rejected by false schema"))) };
    }
    let key = (schema as *const Value as usize, value as *const Value as usize);
    if traversal.active.contains(&key) {
        return Ok(());
    }
    traversal.active.push(key);
    let result = validate_value_inner(root, schema_object(schema, path)?, value, path, traversal);
    traversal.active.pop();
    result
}

fn validate_value_inner(root: &Value, schema: &Object, value: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        let referenced = resolve_local_ref(root, reference).map_err(|message| SchemaError::Validation(format!("{path}.$ref: {message}")))?;
        validate_value(root, referenced, value, path, traversal)?;
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
            validate_value(root, branch, value, path, traversal)?;
        }
    }
    if let Some(branches) = schema.get("anyOf").and_then(Value::as_array) {
        let mut matched = false;
        for branch in branches {
            matched |= branch_matches(validate_value(root, branch, value, path, traversal))?;
        }
        if !matched {
            return Err(SchemaError::Validation(format!("{path}: no anyOf branch matched")));
        }
    }
    if let Some(branches) = schema.get("oneOf").and_then(Value::as_array) {
        let mut matches = 0usize;
        for branch in branches {
            matches += usize::from(branch_matches(validate_value(root, branch, value, path, traversal))?);
        }
        if matches != 1 {
            return Err(SchemaError::Validation(format!("{path}: expected exactly one matching oneOf branch, found {matches}")));
        }
    }
    if let Some(disallowed) = schema.get("not") {
        if branch_matches(validate_value(root, disallowed, value, path, traversal))? {
            return Err(SchemaError::Validation(format!("{path}: matched disallowed schema")));
        }
    }
    validate_object(root, schema, value, path, traversal)?;
    validate_array(root, schema, value, path, traversal)?;
    validate_string(schema, value, path)?;
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

fn validate_object(root: &Value, schema: &Object, value: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    let Some(object) = value.as_object() else { return Ok(()) };
    let properties = schema.get("properties").and_then(Value::as_object);
    if let Some(required) = schema.get("required").and_then(Value::as_array) {
        for key in required.iter().filter_map(Value::as_str) {
            if !object.contains_key(key) {
                return Err(SchemaError::Validation(format!("{path}: missing required property `{key}`")));
            }
        }
    }
    if let Some(properties) = properties {
        for (key, property_schema) in properties {
            if let Some(property_value) = object.get(key) {
                validate_value(root, property_schema, property_value, &format!("{path}.{key}"), traversal)?;
            }
        }
    }
    if let Some(additional) = schema.get("additionalProperties") {
        for (key, property_value) in object {
            if properties.is_some_and(|properties| properties.contains_key(key)) {
                continue;
            }
            match additional.as_bool() {
                Some(true) => {}
                Some(false) => return Err(SchemaError::Validation(format!("{path}: additional property `{key}` is not allowed"))),
                None => validate_value(root, additional, property_value, &format!("{path}.{key}"), traversal)?,
            }
        }
    }
    Ok(())
}

fn validate_array(root: &Value, schema: &Object, value: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
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
    if let Some(item_schema) = schema.get("items") {
        for (index, item) in items.iter().enumerate() {
            validate_value(root, item_schema, item, &format!("{path}[{index}]"), traversal)?;
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

fn validate_string(schema: &Object, value: &Value, path: &str) -> Result<(), SchemaError> {
    let Some(value) = value.as_str() else { return Ok(()) };
    let length = value.chars().count();
    for (keyword, violated) in
        [("minLength", schema.get("minLength").and_then(Value::as_u64).is_some_and(|minimum| length < minimum as usize)), ("maxLength", schema.get("maxLength").and_then(Value::as_u64).is_some_and(|maximum| length > maximum as usize))]
    {
        if violated {
            return Err(SchemaError::Validation(format!("{path}: string violates {keyword}")));
        }
    }
    Ok(())
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
