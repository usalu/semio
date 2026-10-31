//! 🔎️ Instance validation of compiled schemas: objects, arrays, strings and numbers.

use crate::SchemaError;
use semio_framework_pack_json::{Number, Object, Value};
use super::{PatternMatcher, Scope, Traversal, resolve_reference, schema_object, schema_usize, string_format_matches, values_equal};

//#region ✅️Validate

pub(super) fn validate_value(scope: Scope<'_>, schema: &Value, value: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
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
        validate_value(Scope { base, ..scope }, referenced, value, path, traversal)?;
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

pub(super) fn pattern_matches(scope: Scope<'_>, pattern: &str, text: &str, path: &str) -> Result<bool, SchemaError> {
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
