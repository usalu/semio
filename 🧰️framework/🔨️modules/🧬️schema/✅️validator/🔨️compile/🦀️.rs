//! 🔨️ Schema compilation: `$ref` scopes, keyword checks and fragment-context validation.

use crate::SchemaError;
use semio_framework_pack_json::{parse as parse_json, Object, Value};
use semio_framework_value::DslValue;
use super::{PatternMatcher, SchemaFragmentContextRefusal, SchemaFragmentError, SchemaFragmentOperation, SchemaFragmentPathSegment, SchemaFragmentValueShape, SchemaIndex, Traversal, pattern_matches, validate_value, values_equal};

//#region 🧬️Compile

/// 🔎️ Resolution context: the document a local `#/...` pointer walks, plus every sibling document a
/// cross-scope `$ref` may name by `$id`.
#[derive(Clone, Copy)]
pub(super) struct Scope<'a> {
    pub(super) root: &'a Value,
    pub(super) base: &'a Value,
    pub(super) documents: &'a SchemaIndex<Value>,
    pub(super) patterns: &'a SchemaIndex<PatternMatcher>,
}

pub(super) fn index_documents(documents: &[&str]) -> Result<SchemaIndex<Value>, SchemaError> {
    let mut indexed = SchemaIndex::new();
    for body in documents {
        let document = parse_json(body, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| SchemaError::Validation(format!("invalid sibling schema JSON: {error}")))?;
        let id = document.get("$id").and_then(Value::as_str).ok_or_else(|| SchemaError::Validation("sibling schema document requires an `$id`".to_string()))?;
        let id = resolve_schema_document_reference(id, None);
        if indexed.insert(id.clone(), document).is_some() {
            return Err(SchemaError::Validation(format!("duplicate sibling schema `$id` {id}")));
        }
    }
    Ok(indexed)
}

pub(super) fn schema_object<'a>(schema: &'a Value, path: &str) -> Result<&'a Object, SchemaError> {
    schema.as_object().ok_or_else(|| SchemaError::Validation(format!("{path}: schema must be an object or boolean")))
}

pub(super) fn validate_schema_node(scope: Scope<'_>, schema: &Value, path: &str, traversal: &mut Traversal<'_>) -> Result<(), SchemaError> {
    traversal.visit()?;
    if !traversal.compiled_schemas.insert((scope.base as *const Value as usize, schema as *const Value as usize)) {
        return Ok(());
    }
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
                let (base, referenced) = resolve_reference(scope, reference).map_err(|message| SchemaError::Validation(format!("{keyword_path}: {message}")))?;
                validate_schema_node(Scope { base, ..scope }, referenced, &keyword_path, traversal)?;
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

pub(super) fn schema_usize(value: &Value, path: &str) -> Result<usize, SchemaError> {
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

#[derive(Clone, Copy)]
struct UriReference<'a> {
    scheme: Option<&'a str>,
    authority: Option<&'a str>,
    path: &'a str,
    query: Option<&'a str>,
}

fn uri_reference(value: &str) -> UriReference<'_> {
    let value = value.split_once('#').map_or(value, |(head, _)| head);
    let (hierarchy, query) = value.split_once('?').map_or((value, None), |(head, query)| (head, Some(query)));
    let (scheme, rest) = match hierarchy.find(':') {
        Some(index) if hierarchy.as_bytes().first().is_some_and(u8::is_ascii_alphabetic) && hierarchy[..index].bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.')) => (Some(&hierarchy[..index]), &hierarchy[index + 1..]),
        _ => (None, hierarchy),
    };
    let (authority, path) = match rest.strip_prefix("//") {
        Some(rest) => rest.find('/').map_or((Some(rest), ""), |index| (Some(&rest[..index]), &rest[index..])),
        None => (None, rest),
    };
    UriReference { scheme, authority, path, query }
}

fn uri_path_without_dots(mut path: &str) -> String {
    let mut output = String::with_capacity(path.len());
    while !path.is_empty() {
        if let Some(tail) = path.strip_prefix("../").or_else(|| path.strip_prefix("./")) {
            path = tail;
        } else if path.starts_with("/./") {
            path = &path[2..];
        } else if path == "/." {
            path = "/";
        } else if path.starts_with("/../") || path == "/.." {
            path = if path == "/.." { "/" } else { &path[3..] };
            output.truncate(output.rfind('/').unwrap_or(0));
        } else if path == "." || path == ".." {
            path = "";
        } else {
            let start = usize::from(path.starts_with('/'));
            let end = path[start..].find('/').map_or(path.len(), |index| index + start);
            output.push_str(&path[..end]);
            path = &path[end..];
        }
    }
    output
}

/// 🧭️ Resolves a document URI reference against its declared base without filesystem or network access.
/// @see <https://www.rfc-editor.org/rfc/rfc3986#section-5.2>
pub fn resolve_schema_document_reference(reference: &str, base: Option<&str>) -> String {
    let reference = uri_reference(reference);
    let base = uri_reference(base.unwrap_or(""));
    let scheme = reference.scheme.or(base.scheme);
    let authority = if reference.scheme.is_some() { reference.authority } else { reference.authority.or(base.authority) };
    let (path, query) = if reference.scheme.is_some() || reference.authority.is_some() {
        (uri_path_without_dots(reference.path), reference.query)
    } else if reference.path.is_empty() {
        (uri_path_without_dots(base.path), reference.query.or(base.query))
    } else if reference.path.starts_with('/') {
        (uri_path_without_dots(reference.path), reference.query)
    } else {
        let prefix = if base.authority.is_some() && base.path.is_empty() { "/" } else { &base.path[..base.path.rfind('/').map_or(0, |index| index + 1)] };
        (uri_path_without_dots(&format!("{prefix}{}", reference.path)), reference.query)
    };
    let mut result = String::new();
    if let Some(scheme) = scheme { result.push_str(scheme); result.push(':'); }
    if let Some(authority) = authority { result.push_str("//"); result.push_str(authority); }
    result.push_str(&path);
    if let Some(query) = query { result.push('?'); result.push_str(query); }
    result
}

fn decoded_uri_fragment(fragment: &str) -> Result<String, String> {
    let mut output = Vec::with_capacity(fragment.len());
    let bytes = fragment.as_bytes();
    let mut position = 0;
    while position < bytes.len() {
        if bytes[position] == b'%' {
            let pair = bytes.get(position + 1..position + 3).filter(|pair| pair.iter().all(u8::is_ascii_hexdigit)).ok_or_else(|| "invalid percent encoding in schema fragment".to_string())?;
            let digit = |byte: u8| if byte.is_ascii_digit() { byte - b'0' } else { byte.to_ascii_lowercase() - b'a' + 10 };
            output.push(digit(pair[0]) * 16 + digit(pair[1]));
            position += 3;
        } else {
            output.push(bytes[position]);
            position += 1;
        }
    }
    String::from_utf8(output).map_err(|_| "invalid UTF8 in schema fragment".to_string())
}

/// 🔗 Resolves `$ref` to `(document it lives in, referenced node)` — `#`/`#/…` and the document's
/// own `$id` inside the current document, another `<$id>`/`<$id>#/…` into a registered sibling.
pub(super) fn resolve_reference<'a>(scope: Scope<'a>, reference: &str) -> Result<(&'a Value, &'a Value), String> {
    let (document_id, fragment) = match reference.split_once('#') {
        Some((document_id, fragment)) => (document_id, fragment),
        None => (reference, ""),
    };
    let resolved = resolve_schema_document_reference(document_id, scope.base.get("$id").and_then(Value::as_str));
    let document_id = resolved.as_str();
    let base = if reference.starts_with('#') || reference.is_empty() || scope.base.get("$id").and_then(Value::as_str).is_some_and(|id| resolve_schema_document_reference(id, None) == document_id) {
        scope.base
    } else if scope.root.get("$id").and_then(Value::as_str).is_some_and(|id| resolve_schema_document_reference(id, None) == document_id) {
        scope.root
    } else {
        scope.documents.get(document_id).ok_or_else(|| format!("unresolved cross-document reference `{reference}`"))?
    };
    if fragment.is_empty() {
        return Ok((base, base));
    }
    let decoded = decoded_uri_fragment(fragment)?;
    let pointer = decoded.strip_prefix('/').ok_or_else(|| format!("only JSON Pointer fragments are supported, found `{reference}`"))?;
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

#[derive(Clone, Copy)]
pub(super) enum FragmentSchemaCursor<'a> {
    Any,
    Reject,
    Node { scope: Scope<'a>, schema: &'a Value },
}

pub(super) fn fragment_path(path: &[SchemaFragmentPathSegment]) -> String {
    let mut pointer = String::new();
    for segment in path {
        pointer.push('/');
        match segment {
            SchemaFragmentPathSegment::Key(key) => pointer.push_str(&key.replace('~', "~0").replace('/', "~1")),
            SchemaFragmentPathSegment::Index(index) => pointer.push_str(&index.to_string()),
            SchemaFragmentPathSegment::Append => pointer.push('-'),
        }
    }
    pointer
}

fn fragment_error(error: SchemaError, path: &[SchemaFragmentPathSegment]) -> SchemaFragmentError {
    SchemaFragmentError::new("schema.fragment.invalid", fragment_path(path), error.to_string())
}

fn schema_has_observing_siblings(schema: &Object) -> bool {
    schema.iter().any(|(key, _)| !key.starts_with("x-") && !matches!(key, "$ref" | "$id" | "$schema" | "$comment" | "title" | "description" | "default" | "examples" | "readOnly" | "writeOnly" | "deprecated" | "definitions" | "$defs"))
}

pub(super) fn normalize_fragment_cursor<'a>(mut cursor: FragmentSchemaCursor<'a>, path: &[SchemaFragmentPathSegment]) -> Result<FragmentSchemaCursor<'a>, SchemaFragmentError> {
    for _ in 0..64 {
        let FragmentSchemaCursor::Node { scope, schema } = cursor else { return Ok(cursor) };
        if let Some(allowed) = schema.as_bool() {
            return Ok(if allowed { FragmentSchemaCursor::Any } else { FragmentSchemaCursor::Reject });
        }
        let object = schema_object(schema, &fragment_path(path)).map_err(|error| fragment_error(error, path))?;
        let Some(reference) = object.get("$ref").and_then(Value::as_str) else { return Ok(cursor) };
        if schema_has_observing_siblings(object) {
            return Ok(cursor);
        }
        let (base, referenced) = resolve_reference(scope, reference)
            .map_err(|message| SchemaFragmentError::new("schema.fragment.reference", fragment_path(path), message))?;
        cursor = FragmentSchemaCursor::Node { scope: Scope { base, ..scope }, schema: referenced };
    }
    Err(SchemaFragmentError::new(
        "schema.fragment.reference-depth",
        fragment_path(path),
        "schema reference depth exceeds 64",
    ))
}

pub(super) fn validate_fragment_array_structure<G>(
    cursor: FragmentSchemaCursor<'_>,
    path: &[SchemaFragmentPathSegment],
    operation: SchemaFragmentOperation,
    shape: &mut G,
    traversal: &mut Traversal<'_>,
) -> Result<bool, SchemaFragmentError>
where
    G: FnMut(&[SchemaFragmentPathSegment]) -> Result<SchemaFragmentValueShape, SchemaFragmentContextRefusal>,
{
    if !matches!(operation, SchemaFragmentOperation::Insert | SchemaFragmentOperation::Remove) {
        return Ok(false);
    }
    let FragmentSchemaCursor::Node { schema, .. } = cursor else { return Ok(matches!(cursor, FragmentSchemaCursor::Any)) };
    let schema = schema_object(schema, &fragment_path(path)).map_err(|error| fragment_error(error, path))?;
    let is_array = matches!(schema.get("type"), Some(Value::String(kind)) if kind == "array") || schema.contains_key("items");
    if !is_array
        || schema.get("items").is_some_and(|items| items.as_array().is_some())
        || schema.get("uniqueItems").and_then(Value::as_bool) == Some(true)
        || ["contains", "enum", "const", "allOf", "anyOf", "oneOf", "not", "if", "then", "else"].iter().any(|keyword| schema.contains_key(*keyword))
    {
        return Ok(false);
    }
    let len = match shape(path).map_err(|refusal| {
        SchemaFragmentError::new(
            "schema.fragment.context-required",
            fragment_path(path),
            format!("array structure validation requires its post-edit length: {}", refusal.reason),
        )
    })? {
        SchemaFragmentValueShape::Array { len } => len,
        _ => return Ok(false),
    };
    traversal.visit().map_err(|error| fragment_error(error, path))?;
    if let Some(minimum) = schema.get("minItems") {
        let minimum = schema_usize(minimum, &format!("{}.minItems", fragment_path(path))).map_err(|error| fragment_error(error, path))?;
        if len < minimum {
            return Err(SchemaFragmentError::new(
                "schema.fragment.invalid",
                fragment_path(path),
                format!("array length {len} is below minItems {minimum}"),
            ));
        }
    }
    if let Some(maximum) = schema.get("maxItems") {
        let maximum = schema_usize(maximum, &format!("{}.maxItems", fragment_path(path))).map_err(|error| fragment_error(error, path))?;
        if len > maximum {
            return Err(SchemaFragmentError::new(
                "schema.fragment.invalid",
                fragment_path(path),
                format!("array length {len} exceeds maxItems {maximum}"),
            ));
        }
    }
    Ok(true)
}

pub(super) fn fragment_ancestor_needs_context(cursor: FragmentSchemaCursor<'_>, segment: &SchemaFragmentPathSegment, operation: SchemaFragmentOperation, is_parent: bool) -> bool {
    let FragmentSchemaCursor::Node { schema, .. } = cursor else { return false };
    let Some(schema) = schema.as_object() else { return false };
    if schema.get("$ref").is_some() && schema_has_observing_siblings(schema) {
        return true;
    }
    if ["allOf", "anyOf", "oneOf", "not", "if", "then", "else", "dependencies", "patternProperties"]
        .iter()
        .any(|keyword| schema.contains_key(*keyword))
    {
        return true;
    }
    if operation != SchemaFragmentOperation::Set && is_parent {
        return [
            "required",
            "minProperties",
            "maxProperties",
            "propertyNames",
            "minItems",
            "maxItems",
            "contains",
            "uniqueItems",
        ]
        .iter()
        .any(|keyword| schema.contains_key(*keyword));
    }
    matches!(segment, SchemaFragmentPathSegment::Index(_) | SchemaFragmentPathSegment::Append)
        && ["contains", "uniqueItems"].iter().any(|keyword| schema.contains_key(*keyword))
}

fn fragment_union_branches<'a>(schema: &'a Object) -> Option<&'a [Value]> {
    let union = schema.get("oneOf").or_else(|| schema.get("anyOf"))?.as_array()?;
    (!union.is_empty()).then_some(union)
}

fn fragment_union_is_standalone(schema: &Object) -> bool {
    schema.iter().all(|(key, _)| key.starts_with("x-") || matches!(key, "oneOf" | "anyOf" | "$id" | "$schema" | "$comment" | "title" | "description" | "default" | "examples" | "readOnly" | "writeOnly" | "deprecated" | "definitions" | "$defs"))
}

fn fragment_branch_object<'a>(cursor: FragmentSchemaCursor<'a>, path: &[SchemaFragmentPathSegment]) -> Result<Option<(Scope<'a>, &'a Object)>, SchemaFragmentError> {
    let cursor = normalize_fragment_cursor(cursor, path)?;
    let FragmentSchemaCursor::Node { scope, schema } = cursor else { return Ok(None) };
    Ok(schema.as_object().map(|schema| (scope, schema)))
}

fn fragment_discriminator_schema<'a>(schema: &'a Object, key: &str) -> Option<&'a Value> {
    schema.get("properties")?.as_object()?.get(key)
}

fn fragment_discriminator_matches(schema: &Value, value: &Value) -> bool {
    let Some(schema) = schema.as_object() else { return false };
    schema.get("const").is_some_and(|expected| values_equal(expected, value))
        || schema.get("enum").and_then(Value::as_array).is_some_and(|expected| expected.iter().any(|expected| values_equal(expected, value)))
}

pub(super) fn select_fragment_union_branch<'a, F>(
    cursor: FragmentSchemaCursor<'a>,
    path: &[SchemaFragmentPathSegment],
    context: &mut F,
) -> Result<Option<(FragmentSchemaCursor<'a>, String)>, SchemaFragmentError>
where
    F: FnMut(&[SchemaFragmentPathSegment]) -> Result<DslValue, SchemaFragmentContextRefusal>,
{
    let FragmentSchemaCursor::Node { scope, schema } = cursor else { return Ok(None) };
    let Some(schema) = schema.as_object() else { return Ok(None) };
    let Some(branches) = fragment_union_branches(schema) else { return Ok(None) };
    if !fragment_union_is_standalone(schema) {
        return Ok(None);
    }
    let branch_objects = branches
        .iter()
        .map(|branch| fragment_branch_object(FragmentSchemaCursor::Node { scope, schema: branch }, path))
        .collect::<Result<Vec<_>, _>>()?;
    let Some((_, first)) = branch_objects.first().and_then(Option::as_ref) else { return Ok(None) };
    let Some(properties) = first.get("properties").and_then(Value::as_object) else { return Ok(None) };
    for (key, _) in properties {
        if branch_objects.iter().any(|branch| {
            branch
                .as_ref()
                .and_then(|(_, branch)| fragment_discriminator_schema(branch, key))
                .and_then(Value::as_object)
                .map_or(true, |schema| !schema.contains_key("const") && !schema.contains_key("enum"))
        }) {
            continue;
        }
        let mut discriminator_path = path.to_vec();
        discriminator_path.push(SchemaFragmentPathSegment::Key(key.to_owned()));
        let discriminator = context(&discriminator_path).map_err(|refusal| {
            SchemaFragmentError::new(
                "schema.fragment.context-required",
                fragment_path(&discriminator_path),
                format!("the discriminated union requires its tag value: {}", refusal.reason),
            )
        })?;
        let discriminator = semio_framework_pack_json::from_dsl_value(&discriminator);
        let matching = branch_objects
            .iter()
            .enumerate()
            .filter(|(_, branch)| {
                branch
                    .as_ref()
                    .and_then(|(_, branch)| fragment_discriminator_schema(branch, key))
                    .is_some_and(|schema| fragment_discriminator_matches(schema, &discriminator))
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if let [index] = matching.as_slice() {
            let selected = normalize_fragment_cursor(FragmentSchemaCursor::Node { scope, schema: &branches[*index] }, path)?;
            return Ok(Some((selected, key.to_owned())));
        }
    }
    Ok(None)
}

pub(super) fn descend_fragment_cursor<'a>(
    cursor: FragmentSchemaCursor<'a>,
    segment: &SchemaFragmentPathSegment,
    path: &[SchemaFragmentPathSegment],
) -> Result<FragmentSchemaCursor<'a>, SchemaFragmentError> {
    let FragmentSchemaCursor::Node { scope, schema } = cursor else { return Ok(cursor) };
    let schema = schema_object(schema, &fragment_path(path)).map_err(|error| fragment_error(error, path))?;
    match segment {
        SchemaFragmentPathSegment::Key(key) => {
            let properties = schema.get("properties").and_then(Value::as_object);
            let exact = properties.and_then(|properties| properties.get(key));
            let mut patterns = Vec::new();
            if let Some(pattern_properties) = schema.get("patternProperties").and_then(Value::as_object) {
                for (pattern, child) in pattern_properties {
                    if pattern_matches(scope, pattern, key, &format!("{}.patternProperties", fragment_path(path))).map_err(|error| fragment_error(error, path))? {
                        patterns.push(child);
                    }
                }
            }
            if exact.is_some() && !patterns.is_empty() || patterns.len() > 1 {
                return Err(SchemaFragmentError::new(
                    "schema.fragment.context-required",
                    fragment_path(path),
                    "overlapping property schemas require the parent object projection",
                ));
            }
            let child = exact.or_else(|| patterns.first().copied()).or_else(|| schema.get("additionalProperties"));
            Ok(match child {
                Some(Value::Bool(true)) | None => FragmentSchemaCursor::Any,
                Some(Value::Bool(false)) => FragmentSchemaCursor::Reject,
                Some(child) => FragmentSchemaCursor::Node { scope, schema: child },
            })
        }
        SchemaFragmentPathSegment::Index(index) => {
            let child = match schema.get("items") {
                Some(Value::Array(items)) => items.get(*index).or_else(|| schema.get("additionalItems")),
                Some(items) => Some(items),
                None => None,
            };
            Ok(match child {
                Some(Value::Bool(true)) | None => FragmentSchemaCursor::Any,
                Some(Value::Bool(false)) => FragmentSchemaCursor::Reject,
                Some(child) => FragmentSchemaCursor::Node { scope, schema: child },
            })
        }
        SchemaFragmentPathSegment::Append => {
            let child = match schema.get("items") {
                Some(Value::Array(_)) => schema.get("additionalItems"),
                Some(items) => Some(items),
                None => None,
            };
            Ok(match child {
                Some(Value::Bool(true)) | None => FragmentSchemaCursor::Any,
                Some(Value::Bool(false)) => FragmentSchemaCursor::Reject,
                Some(child) => FragmentSchemaCursor::Node { scope, schema: child },
            })
        }
    }
}

pub(super) fn validate_fragment_context<F>(
    cursor: FragmentSchemaCursor<'_>,
    path: &[SchemaFragmentPathSegment],
    context: &mut F,
    traversal: &mut Traversal<'_>,
) -> Result<(), SchemaFragmentError>
where
    F: FnMut(&[SchemaFragmentPathSegment]) -> Result<DslValue, SchemaFragmentContextRefusal>,
{
    let projected = context(path).map_err(|refusal| {
        SchemaFragmentError::new(
            "schema.fragment.context-required",
            fragment_path(path),
            format!("the validation frontier requires this post-edit container projection: {}", refusal.reason),
        )
    })?;
    validate_fragment_candidate(cursor, &projected, path, traversal)
}

pub(super) fn validate_fragment_candidate(
    cursor: FragmentSchemaCursor<'_>,
    candidate: &DslValue,
    path: &[SchemaFragmentPathSegment],
    traversal: &mut Traversal<'_>,
) -> Result<(), SchemaFragmentError> {
    match cursor {
        FragmentSchemaCursor::Any => Ok(()),
        FragmentSchemaCursor::Reject => Err(SchemaFragmentError::new(
            "schema.fragment.invalid",
            fragment_path(path),
            "candidate is rejected by a false schema",
        )),
        FragmentSchemaCursor::Node { scope, schema } => {
            let candidate = semio_framework_pack_json::from_dsl_value(candidate);
            validate_value(scope, schema, &candidate, &fragment_path(path), traversal).map_err(|error| fragment_error(error, path))
        }
    }
}

//#endregion 🧬️Compile
