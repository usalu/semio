//! 🪡️ Pointer operations on snapshot values, source parsing and schema validation.

use super::*;

pub fn decode_pointer(path: &str) -> Result<Vec<String>, SnapshotEditError> {
    if path.is_empty() {
        return Ok(Vec::new());
    }
    if !path.starts_with('/') {
        return Err(SnapshotEditError::new("snapshot-edit.invalid-pointer", path, "an RFC 6901 pointer must be empty or start with '/'"));
    }
    path[1..]
        .split('/')
        .map(|raw| {
            let mut decoded = String::with_capacity(raw.len());
            let mut chars = raw.chars();
            while let Some(character) = chars.next() {
                if character != '~' {
                    decoded.push(character);
                    continue;
                }
                match chars.next() {
                    Some('0') => decoded.push('~'),
                    Some('1') => decoded.push('/'),
                    _ => return Err(SnapshotEditError::new("snapshot-edit.invalid-pointer", path, "an RFC 6901 escape must be '~0' or '~1'")),
                }
            }
            Ok(decoded)
        })
        .collect()
}

pub(super) fn object_index(entries: &[(String, DslValue)], key: &str, path: &str) -> Result<usize, SnapshotEditError> {
    let mut matches = entries.iter().enumerate().filter(|(_, (candidate, _))| candidate == key);
    let Some((index, _)) = matches.next() else {
        return Err(SnapshotEditError::new("snapshot-edit.path-missing", path, format!("object key '{key}' does not exist")));
    };
    if matches.next().is_some() {
        return Err(SnapshotEditError::new("snapshot-edit.ambiguous-object", path, format!("object key '{key}' occurs more than once")));
    }
    Ok(index)
}

pub fn array_index(segment: &str, length: usize, path: &str, insert: bool) -> Result<usize, SnapshotEditError> {
    if insert && segment == "-" {
        return Ok(length);
    }
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(SnapshotEditError::new("snapshot-edit.invalid-index", path, format!("'{segment}' is not a canonical array index")));
    }
    let index = segment.parse::<usize>().map_err(|_| SnapshotEditError::new("snapshot-edit.invalid-index", path, format!("array index '{segment}' is too large")))?;
    let admitted = if insert { index <= length } else { index < length };
    if !admitted {
        return Err(SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("array index {index} exceeds length {length}")));
    }
    Ok(index)
}

fn value_at_mut<'a>(root: &'a mut DslValue, segments: &[String], path: &str) -> Result<&'a mut DslValue, SnapshotEditError> {
    let mut current = root;
    for segment in segments {
        current = match current {
            DslValue::Object(entries) => {
                let index = object_index(entries, segment, path)?;
                &mut entries[index].1
            }
            DslValue::Array(items) => {
                let index = array_index(segment, items.len(), path, false)?;
                &mut items[index]
            }
            _ => return Err(SnapshotEditError::new("snapshot-edit.not-container", path, format!("path segment '{segment}' has a scalar parent"))),
        };
    }
    Ok(current)
}

fn split_parent<'a>(segments: &'a [String], path: &str) -> Result<(&'a [String], &'a str), SnapshotEditError> {
    segments.split_last().map(|(last, parent)| (parent, last.as_str())).ok_or_else(|| SnapshotEditError::new("snapshot-edit.root-operation", path, "this operation cannot address the document root"))
}

pub(super) fn remove_value(root: &mut DslValue, path: &str) -> Result<DslValue, SnapshotEditError> {
    let segments = decode_pointer(path)?;
    let (parent, last) = split_parent(&segments, path)?;
    match value_at_mut(root, parent, path)? {
        DslValue::Object(entries) => {
            let index = object_index(entries, last, path)?;
            Ok(entries.remove(index).1)
        }
        DslValue::Array(items) => {
            let index = array_index(last, items.len(), path, false)?;
            Ok(items.remove(index))
        }
        _ => Err(SnapshotEditError::new("snapshot-edit.not-container", path, "the addressed parent is a scalar")),
    }
}

pub(super) fn insert_value(root: &mut DslValue, path: &str, value: DslValue) -> Result<(), SnapshotEditError> {
    let segments = decode_pointer(path)?;
    let (parent, last) = split_parent(&segments, path)?;
    match value_at_mut(root, parent, path)? {
        DslValue::Object(entries) => {
            if entries.iter().any(|(key, _)| key == last) {
                return Err(SnapshotEditError::new("snapshot-edit.key-exists", path, format!("object key '{last}' already exists")));
            }
            entries.push((last.to_string(), value));
            Ok(())
        }
        DslValue::Array(items) => {
            let index = array_index(last, items.len(), path, true)?;
            items.insert(index, value);
            Ok(())
        }
        _ => Err(SnapshotEditError::new("snapshot-edit.not-container", path, "the addressed parent is a scalar")),
    }
}

pub(super) fn set_value(root: &mut DslValue, path: &str, value: DslValue) -> Result<(), SnapshotEditError> {
    let segments = decode_pointer(path)?;
    if segments.is_empty() {
        *root = value;
        return Ok(());
    }
    *value_at_mut(root, &segments, path)? = value;
    Ok(())
}

pub(super) fn move_value(root: &mut DslValue, from: &str, path: &str) -> Result<(), SnapshotEditError> {
    let source = decode_pointer(from)?;
    let destination = decode_pointer(path)?;
    if source.is_empty() || destination.is_empty() {
        return Err(SnapshotEditError::new("snapshot-edit.root-operation", path, "move cannot address the document root"));
    }
    if destination.len() > source.len() && destination[..source.len()] == source {
        return Err(SnapshotEditError::new("snapshot-edit.descendant-move", path, "a value cannot move into its own descendant"));
    }
    if from == path {
        return Ok(());
    }
    let value = remove_value(root, from)?;
    insert_value(root, path, value)
}

pub(super) fn rename_key(root: &mut DslValue, path: &str, key: &str) -> Result<(), SnapshotEditError> {
    let segments = decode_pointer(path)?;
    let (parent, old_key) = split_parent(&segments, path)?;
    let DslValue::Object(entries) = value_at_mut(root, parent, path)? else {
        return Err(SnapshotEditError::new("snapshot-edit.not-object", path, "only an object key can be renamed"));
    };
    let index = object_index(entries, old_key, path)?;
    if old_key != key && entries.iter().any(|(candidate, _)| candidate == key) {
        return Err(SnapshotEditError::new("snapshot-edit.key-exists", path, format!("object key '{key}' already exists")));
    }
    entries[index].0 = key.to_string();
    Ok(())
}

fn json_error_offset(error: &semio_framework_pack_json::JsonError, source_len: usize) -> Option<usize> {
    match error {
        semio_framework_pack_json::JsonError::UnexpectedEof => Some(source_len),
        semio_framework_pack_json::JsonError::UnexpectedByte { offset, .. }
        | semio_framework_pack_json::JsonError::ControlCharacterInString { offset, .. }
        | semio_framework_pack_json::JsonError::DuplicateMember { offset, .. } => Some(*offset),
        semio_framework_pack_json::JsonError::InvalidNumber(offset)
        | semio_framework_pack_json::JsonError::InvalidEscape(offset)
        | semio_framework_pack_json::JsonError::InvalidUnicodeEscape(offset)
        | semio_framework_pack_json::JsonError::UnpairedSurrogate(offset)
        | semio_framework_pack_json::JsonError::TrailingData(offset) => Some(*offset),
        semio_framework_pack_json::JsonError::Native(_) | semio_framework_pack_json::JsonError::InvalidUtf8 | semio_framework_pack_json::JsonError::MaxDepthExceeded(_) => None,
    }
}

fn source_span(source: &str, offset: usize) -> semio_framework_diagnostic::TextSpan {
    let mut end = offset.min(source.len());
    while end > 0 && !source.is_char_boundary(end) {
        end -= 1;
    }
    let prefix = &source[..end];
    let line = u32::try_from(prefix.bytes().filter(|byte| *byte == b'\n').count().saturating_add(1)).unwrap_or(u32::MAX);
    let column = u32::try_from(prefix.rsplit_once('\n').map_or(prefix, |(_, tail)| tail).chars().count().saturating_add(1)).unwrap_or(u32::MAX);
    semio_framework_diagnostic::TextSpan::with_length(line, column, u32::from(end < source.len()))
}

pub(super) fn parse_snapshot_json(source: &str) -> Result<DslValue, SnapshotEditError> {
    semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject)
        .map(|value| semio_framework_pack_json::to_dsl_value(&value))
        .map_err(|error| {
            let span = json_error_offset(&error, source.len()).map(|offset| source_span(source, offset));
            let code = if matches!(&error, semio_framework_pack_json::JsonError::DuplicateMember { .. }) { "snapshot-edit.ambiguous-object" } else { "snapshot-edit.invalid-source" };
            let error = SnapshotEditError::new(code, "", error.to_string());
            span.map_or(error.clone(), |span| error.with_span(span))
        })
}

/// 🔬️ Decodes complete snapshot JSON and refuses any normalized or discarded detail.
pub fn snapshot_from_edit_source<S: FromValue + ToValue>(source: &str) -> Result<S, SnapshotEditError> {
    let value = parse_snapshot_json(source)?;
    validate_value(&value, "")?;
    let decoded = S::from_value(value.clone()).map_err(|error| SnapshotEditError::new("snapshot-edit.schema-invalid", "", error.to_string()))?;
    if !values_equivalent(&decoded.to_value(), &value) {
        return Err(SnapshotEditError::new("snapshot-edit.lossy-conversion", "", "the typed snapshot would normalize or discard part of the source"));
    }
    Ok(decoded)
}

/// ✅ Validates a typed snapshot projection against its normative JSON Schema constraints.
pub fn validate_snapshot_value_against_schema(value: &DslValue, schema: &str) -> Result<(), SnapshotEditError> {
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(schema).map_err(|error| SnapshotEditError::new("snapshot-edit.invalid-schema-contract", "", error.to_string()))?;
    validate_snapshot_value_with_validator(value, &validator)
}

fn validate_snapshot_value_with_validator(value: &DslValue, validator: &semio_framework_schema::OwnedJsonSchemaValidator) -> Result<(), SnapshotEditError> {
    let source = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(value));
    validator.validate_json(&source).map(|_| ()).map_err(|error| {
        let message = match error {
            semio_framework_schema::SchemaError::Validation(message) => message,
            other => other.to_string(),
        };
        let path = message.split_once(':').map_or("", |(path, _)| path).trim().to_string();
        SnapshotEditError::new("snapshot-edit.constraint-invalid", path, message)
    })
}

pub static SNAPSHOT_SCHEMA_VALIDATORS: std::sync::OnceLock<std::sync::RwLock<std::collections::HashMap<String, semio_framework_schema::OwnedJsonSchemaValidator>>> = std::sync::OnceLock::new();

fn snapshot_schema_id(value: &DslValue) -> Option<String> {
    let DslValue::Object(entries) = value else { return None };
    let DslValue::String(schema) = entries.iter().find(|(key, _)| key == "schema").map(|(_, value)| value)? else { return None };
    Some(if schema.starts_with("s.") { schema.clone() } else { format!("s.{schema}") })
}

/// 🧬 Refuses retained editing when an editor's authoritative snapshot identity has no exact dialect schema contract.
pub fn validate_snapshot_schema_for_dialect(value: &DslValue, dialect: Dialect, document_schema: &str) -> Result<(), SnapshotEditError> {
    let actual = snapshot_schema_id(value);
    let expected = snapshot_schema_descriptor_for_dialect(dialect, document_schema)?;
    if actual.as_deref() == Some(expected.as_str()) {
        return Ok(());
    }
    Err(SnapshotEditError::new("snapshot-edit.schema-identity", "$.schema", format!("snapshot schema identity '{}' does not match registered editor schema '{expected}'", actual.as_deref().unwrap_or("<missing>"))))
}

pub fn snapshot_schema_descriptor_for_dialect(dialect: Dialect, document_schema: &str) -> Result<String, SnapshotEditError> {
    let descriptor_id = if document_schema.starts_with("s.") { document_schema.to_string() } else { format!("s.{document_schema}") };
    if descriptor_id != dialect.artifact_kind && !descriptor_id.strip_prefix(dialect.artifact_kind).is_some_and(|suffix| suffix.starts_with('.')) {
        return Err(SnapshotEditError::new("snapshot-edit.schema-owner", "$.schema", format!("native document schema '{descriptor_id}' is not owned by editor artifact '{}'", dialect.artifact_kind)));
    }
    semio_framework_schema_registry::with_artifact_schema_registry(|artifacts| {
        if artifacts.get(&descriptor_id).is_some() {
            return Ok(descriptor_id);
        }
        Err(SnapshotEditError::new("snapshot-edit.schema-unregistered", "$.schema", format!("native document schema '{descriptor_id}' selected by {}@{}/{} is not registered", dialect.artifact_kind, dialect.standard.0, dialect.subset.0)))
    })
}

fn validate_registered_snapshot_schema(original: &DslValue, value: &DslValue) -> Result<(), SnapshotEditError> {
    let DslValue::Object(entries) = original else { return Ok(()) };
    let Some(DslValue::String(schema)) = entries.iter().find(|(key, _)| key == "schema").map(|(_, value)| value) else {
        return Ok(());
    };
    let id = if schema.starts_with("s.") { schema.clone() } else { format!("s.{schema}") };
    let candidate_schema = match value {
        DslValue::Object(entries) => entries.iter().find(|(key, _)| key == "schema").map(|(_, value)| value),
        _ => None,
    };
    let validators = SNAPSHOT_SCHEMA_VALIDATORS.get_or_init(|| std::sync::RwLock::new(std::collections::HashMap::new()));
    {
        let cached = validators.read().unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(validator) = cached.get(&id) {
            if candidate_schema != Some(&DslValue::String(schema.clone())) {
                return Err(SnapshotEditError::new("snapshot-edit.schema-identity", "$.schema", "an edit cannot change the registered snapshot schema identity"));
            }
            return validate_snapshot_value_with_validator(value, validator);
        }
    }
    semio_framework_schema_registry::with_artifact_schema_registry(|artifacts| {
        if artifacts.get(&id).is_none() {
            return Ok(());
        }
        if candidate_schema != Some(&DslValue::String(schema.clone())) {
            return Err(SnapshotEditError::new("snapshot-edit.schema-identity", "$.schema", "an edit cannot change the registered snapshot schema identity"));
        }
        let validator = semio_framework_schema::structural_validator_for(&id, "snapshot").map_err(|error| SnapshotEditError::new("snapshot-edit.invalid-schema-contract", "", format!("{id}: {error}")))?;
        let result = validate_snapshot_value_with_validator(value, &validator);
        validators.write().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(id, validator);
        result
    })
}

pub fn validate_value(value: &DslValue, path: &str) -> Result<(), SnapshotEditError> {
    validate_value_at_depth(value, path, 0)
}

fn validate_value_at_depth(value: &DslValue, path: &str, depth: usize) -> Result<(), SnapshotEditError> {
    if depth > 128 {
        return Err(SnapshotEditError::new("snapshot-edit.depth-exceeded", path, "snapshot nesting exceeds 128 levels"));
    }
    match value {
        DslValue::Number(semio_framework_value::Number::Float(value)) if !value.is_finite() => Err(SnapshotEditError::new("snapshot-edit.non-finite-number", path, "snapshot numbers must be finite")),
        DslValue::Array(items) => items.iter().enumerate().try_for_each(|(index, item)| validate_value_at_depth(item, &format!("{path}/{index}"), depth + 1)),
        DslValue::Object(entries) => {
            let mut keys = BTreeSet::new();
            for (key, item) in entries {
                if !keys.insert(key.as_str()) {
                    return Err(SnapshotEditError::new("snapshot-edit.ambiguous-object", path, format!("object key '{key}' occurs more than once")));
                }
                let escaped = key.replace('~', "~0").replace('/', "~1");
                validate_value_at_depth(item, &format!("{path}/{escaped}"), depth + 1)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// 🟰️ Whether two snapshot values denote the same JSON document: object members are unordered, integral numbers equal their exact float, and an octet string equals the JSON array of its octets (the only form JSON can spell it in).
pub fn values_equivalent(left: &DslValue, right: &DslValue) -> bool {
    match (left, right) {
        (DslValue::Object(left), DslValue::Object(right)) => {
            fn index(entries: &[(String, DslValue)]) -> Option<BTreeMap<&str, &DslValue>> {
                let mut values = BTreeMap::new();
                for (key, value) in entries {
                    if values.insert(key.as_str(), value).is_some() {
                        return None;
                    }
                }
                Some(values)
            }
            match (index(left), index(right)) {
                (Some(left), Some(right)) => left.len() == right.len() && left.iter().all(|(key, value)| right.get(key).is_some_and(|other| values_equivalent(value, other))),
                _ => false,
            }
        }
        (DslValue::Array(left), DslValue::Array(right)) => left.len() == right.len() && left.iter().zip(right).all(|(left, right)| values_equivalent(left, right)),
        (DslValue::Bytes(bytes), DslValue::Array(items)) | (DslValue::Array(items), DslValue::Bytes(bytes)) => {
            bytes.len() == items.len() && bytes.iter().zip(items).all(|(byte, item)| values_equivalent(&DslValue::Number(semio_framework_value::Number::UInt(u64::from(*byte))), item))
        }
        (DslValue::Number(semio_framework_value::Number::Float(float)), DslValue::Number(semio_framework_value::Number::UInt(integer))) | (DslValue::Number(semio_framework_value::Number::UInt(integer)), DslValue::Number(semio_framework_value::Number::Float(float))) => {
            *integer <= 9_007_199_254_740_991 && *float == *integer as f64
        }
        (DslValue::Number(semio_framework_value::Number::Float(float)), DslValue::Number(semio_framework_value::Number::Int(integer))) | (DslValue::Number(semio_framework_value::Number::Int(integer)), DslValue::Number(semio_framework_value::Number::Float(float))) => {
            integer.unsigned_abs() <= 9_007_199_254_740_991 && *float == *integer as f64
        }
        _ => left == right,
    }
}
