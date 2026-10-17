//! 🧬️ Schema bundling, normalization, union selection and schema navigation by path.

use super::*;

pub(super) fn object_field<'a>(value: &'a DslValue, key: &str) -> Option<&'a DslValue> {
    let DslValue::Object(entries) = value else { return None };
    entries.iter().find(|(candidate, _)| candidate == key).map(|(_, value)| value)
}

fn local_schema_ref<'a>(root: &'a DslValue, reference: &str) -> Option<&'a DslValue> {
    if reference == "#" {
        return Some(root);
    }
    let mut value = root;
    for segment in reference.strip_prefix("#/")?.split('/') {
        let key = segment.replace("~1", "/").replace("~0", "~");
        value = object_field(value, &key)?;
    }
    Some(value)
}

pub(super) const RESOLVED_DOCUMENTS_KEY: &str = "x-semio-resolved-documents";

pub(super) fn schema_id(value: &DslValue) -> Option<&str> {
    match object_field(value, "$id") {
        Some(DslValue::String(id)) => Some(id),
        _ => None,
    }
}

fn collect_external_schema_references(value: &DslValue, references: &mut Vec<String>) {
    match value {
        DslValue::Array(values) => values.iter().for_each(|value| collect_external_schema_references(value, references)),
        DslValue::Object(entries) => {
            for (key, value) in entries {
                if key == "$ref" {
                    if let DslValue::String(reference) = value {
                        let document = reference.split_once('#').map_or(reference.as_str(), |(document, _)| document);
                        if !document.is_empty() && !references.iter().any(|existing| existing == document) {
                            references.push(document.to_string());
                        }
                    }
                } else {
                    collect_external_schema_references(value, references);
                }
            }
        }
        _ => {}
    }
}

fn rewrite_schema_references(value: DslValue, current: &str, locations: &HashMap<String, String>) -> DslValue {
    match value {
        DslValue::Array(values) => DslValue::Array(values.into_iter().map(|value| rewrite_schema_references(value, current, locations)).collect()),
        DslValue::Object(entries) => DslValue::Object(
            entries
                .into_iter()
                .map(|(key, value)| {
                    if key != "$ref" {
                        return (key, rewrite_schema_references(value, current, locations));
                    }
                    let DslValue::String(reference) = value else { return (key, value) };
                    let (document, fragment) = reference.split_once('#').map_or((reference.as_str(), ""), |(document, fragment)| (document, fragment));
                    let location = if document.is_empty() { Some(current) } else { locations.get(document).map(String::as_str) };
                    let Some(location) = location else { return (key, DslValue::String(reference)) };
                    let reference = if fragment.is_empty() { location.to_string() } else { format!("{location}{fragment}") };
                    (key, DslValue::String(reference))
                })
                .collect(),
        ),
        value => value,
    }
}

fn schema_array_union(left: &mut Vec<DslValue>, right: Vec<DslValue>) {
    for value in right {
        if !left.contains(&value) {
            left.push(value);
        }
    }
}

fn merge_schema_object(left: &mut Vec<(String, DslValue)>, right: Vec<(String, DslValue)>) {
    for (key, right) in right {
        let Some((_, left_value)) = left.iter_mut().find(|(candidate, _)| candidate == &key) else {
            left.push((key, right));
            continue;
        };
        if *left_value == right {
            continue;
        }
        match (key.as_str(), left_value, right) {
            ("required", DslValue::Array(left), DslValue::Array(right)) => schema_array_union(left, right),
            ("properties" | "$defs" | "definitions", DslValue::Object(left), DslValue::Object(right)) => {
                for (field, schema) in right {
                    if let Some((_, existing)) = left.iter_mut().find(|(candidate, _)| candidate == &field) {
                        if let (DslValue::Object(existing), DslValue::Object(schema)) = (existing, schema) {
                            merge_schema_object(existing, schema);
                        }
                    } else {
                        left.push((field, schema));
                    }
                }
            }
            ("enum", DslValue::Array(left), DslValue::Array(right)) => left.retain(|value| right.contains(value)),
            ("additionalProperties", DslValue::Bool(left), DslValue::Bool(right)) => *left = *left && right,
            ("additionalProperties", left @ DslValue::Bool(true), right @ DslValue::Object(_)) => *left = right,
            ("additionalProperties", left @ DslValue::Object(_), DslValue::Bool(false)) => *left = DslValue::Bool(false),
            ("additionalProperties" | "items", DslValue::Object(left), DslValue::Object(right)) => merge_schema_object(left, right),
            ("minItems" | "minLength" | "minProperties" | "minimum" | "exclusiveMinimum", DslValue::Number(left), DslValue::Number(right)) => {
                if number_as_f64(right) > number_as_f64(*left) {
                    *left = right;
                }
            }
            ("maxItems" | "maxLength" | "maxProperties" | "maximum" | "exclusiveMaximum", DslValue::Number(left), DslValue::Number(right)) => {
                if number_as_f64(right) < number_as_f64(*left) {
                    *left = right;
                }
            }
            _ => {}
        }
    }
}

fn normalize_schema_map(root: &DslValue, value: DslValue, depth: usize) -> DslValue {
    let DslValue::Object(entries) = value else { return value };
    DslValue::Object(entries.into_iter().map(|(key, value)| (key, normalize_schema_node(root, value, depth + 1))).collect())
}

fn normalize_schema_node(root: &DslValue, schema: DslValue, depth: usize) -> DslValue {
    if depth >= 64 {
        return schema;
    }
    let DslValue::Object(entries) = schema else { return schema };
    let all_of = entries.iter().find(|(key, _)| key == "allOf").map(|(_, value)| value.clone());
    let mut normalized = Vec::with_capacity(entries.len());
    for (key, value) in entries {
        if key == "allOf" {
            continue;
        }
        let value = match key.as_str() {
            "properties" | "$defs" | "definitions" | "patternProperties" => normalize_schema_map(root, value, depth),
            "items" | "oneOf" | "anyOf" | "not" | "if" | "then" | "else" | "contains" | "additionalProperties" => match value {
                DslValue::Array(values) => DslValue::Array(values.into_iter().map(|value| normalize_schema_node(root, value, depth + 1)).collect()),
                value @ DslValue::Object(_) => normalize_schema_node(root, value, depth + 1),
                value => value,
            },
            _ => value,
        };
        normalized.push((key, value));
    }
    if let Some(DslValue::Array(members)) = all_of {
        for member in members {
            let member = match &member {
                DslValue::Object(_) => match object_field(&member, "$ref") {
                    Some(DslValue::String(reference)) => local_schema_ref(root, reference).cloned().unwrap_or_else(|| member.clone()),
                    _ => member,
                },
                _ => member,
            };
            if let DslValue::Object(member) = normalize_schema_node(root, member, depth + 1) {
                merge_schema_object(&mut normalized, member);
            }
        }
    }
    DslValue::Object(normalized)
}

pub(super) fn bundle_snapshot_schema(root: DslValue, available: Vec<DslValue>) -> DslValue {
    let root_id = schema_id(&root).map(str::to_string);
    let mut available = available
        .into_iter()
        .filter_map(|schema| {
            let id = schema_id(&schema)?.to_string();
            Some((id, schema))
        })
        .collect::<HashMap<_, _>>();
    if let Some(root_id) = root_id.as_ref() {
        available.remove(root_id);
    }
    let mut selected = Vec::new();
    collect_external_schema_references(&root, &mut selected);
    let mut visited = HashSet::new();
    let mut cursor = 0;
    while cursor < selected.len() {
        let id = selected[cursor].clone();
        cursor += 1;
        if !visited.insert(id.clone()) {
            continue;
        }
        if let Some(schema) = available.get(&id) {
            collect_external_schema_references(schema, &mut selected);
        }
    }
    selected.retain(|id| available.contains_key(id));
    selected.sort();
    selected.dedup();
    let mut locations = selected.iter().enumerate().map(|(index, id)| (id.clone(), format!("#/{RESOLVED_DOCUMENTS_KEY}/d{index}"))).collect::<HashMap<_, _>>();
    if let Some(root_id) = root_id {
        locations.insert(root_id, "#".into());
    }
    let rewritten_root = rewrite_schema_references(root, "#", &locations);
    let rewritten_documents = selected
        .iter()
        .enumerate()
        .filter_map(|(index, id)| {
            let location = format!("#/{RESOLVED_DOCUMENTS_KEY}/d{index}");
            available.remove(id).map(|schema| (format!("d{index}"), rewrite_schema_references(schema, &location, &locations)))
        })
        .collect::<Vec<_>>();
    let mut raw = rewritten_root;
    if let DslValue::Object(entries) = &mut raw {
        entries.push((RESOLVED_DOCUMENTS_KEY.into(), DslValue::Object(rewritten_documents)));
    }
    let mut normalized = normalize_schema_node(&raw, raw.clone(), 0);
    if let DslValue::Object(entries) = &mut normalized {
        if let Some((_, DslValue::Object(documents))) = entries.iter_mut().find(|(key, _)| key == RESOLVED_DOCUMENTS_KEY) {
            for (_, document) in documents {
                *document = normalize_schema_node(&raw, document.clone(), 0);
            }
        }
    }
    normalized
}

pub(super) fn resolved_schema<'a>(root: &'a DslValue, schema: &'a DslValue, depth: usize) -> Option<&'a DslValue> {
    if depth >= 64 {
        return None;
    }
    match object_field(schema, "$ref") {
        Some(DslValue::String(reference)) => resolved_schema(root, local_schema_ref(root, reference)?, depth + 1),
        _ => Some(schema),
    }
}

pub(super) fn union_members(schema: &DslValue) -> Option<&[DslValue]> {
    match object_field(schema, "oneOf").or_else(|| object_field(schema, "anyOf")) {
        Some(DslValue::Array(members)) => Some(members),
        _ => None,
    }
}

fn schema_enum_constraint(schema: &DslValue) -> Option<Vec<&DslValue>> {
    if let Some(value) = object_field(schema, "const") {
        return Some(vec![value]);
    }
    match object_field(schema, "enum") {
        Some(DslValue::Array(values)) if !values.is_empty() => Some(values.iter().collect()),
        _ => None,
    }
}

fn schema_discriminator_constraint(schema: &DslValue) -> Option<Vec<&DslValue>> {
    if let Some(value) = object_field(schema, "const") {
        return Some(vec![value]);
    }
    match object_field(schema, "enum") {
        Some(DslValue::Array(values)) if values.len() == 1 => Some(values.iter().collect()),
        _ => None,
    }
}

pub(super) fn schema_accepts_shape(root: &DslValue, schema: &DslValue, shape: &ValueShape) -> bool {
    let Some(schema) = resolved_schema(root, schema, 0) else { return false };
    let accepts = |kind: &str| match kind {
        "null" => matches!(shape, ValueShape::Null),
        "boolean" => matches!(shape, ValueShape::Bool),
        "integer" | "number" => matches!(shape, ValueShape::Number),
        "string" => matches!(shape, ValueShape::String),
        "array" => matches!(shape, ValueShape::Array { .. } | ValueShape::Bytes { .. }),
        "object" => matches!(shape, ValueShape::Object { .. }),
        _ => false,
    };
    match object_field(schema, "type") {
        Some(DslValue::String(kind)) => accepts(kind),
        Some(DslValue::Array(kinds)) => kinds.iter().any(|kind| matches!(kind, DslValue::String(kind) if accepts(kind))),
        _ if object_field(schema, "properties").is_some() => matches!(shape, ValueShape::Object { .. }),
        _ if object_field(schema, "items").is_some() => matches!(shape, ValueShape::Array { .. } | ValueShape::Bytes { .. }),
        _ => true,
    }
}

pub(super) fn selected_union_member<'a>(
    root: &'a DslValue,
    members: &'a [DslValue],
    path: &[SnapshotDetailPathSegment],
    read: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<DslValue>,
    shape: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<ValueShape>,
) -> Option<&'a DslValue> {
    let current_shape = shape(path)?;
    let candidates = members.iter().enumerate().filter(|(_, member)| schema_accepts_shape(root, member, &current_shape)).collect::<Vec<_>>();
    if let [(index, _)] = candidates.as_slice() {
        return members.get(*index);
    }
    let current_value = matches!(current_shape, ValueShape::Null | ValueShape::Bool | ValueShape::Number | ValueShape::String).then(|| read(path)).flatten();
    if let Some(current_value) = current_value.as_ref() {
        let matching = candidates.iter().filter_map(|(index, member)| schema_enum_constraint(resolved_schema(root, member, 0)?).is_some_and(|expected| expected.iter().any(|expected| *expected == current_value)).then_some(*index)).collect::<Vec<_>>();
        if let [index] = matching.as_slice() {
            return members.get(*index);
        }
    }
    let (_, first) = candidates.first()?;
    let first = resolved_schema(root, first, 0)?;
    let DslValue::Object(properties) = object_field(first, "properties")? else { return None };
    for (key, _) in properties {
        let constraints = candidates
            .iter()
            .map(|(_, member)| {
                let member = resolved_schema(root, member, 0)?;
                schema_discriminator_constraint(object_field(object_field(member, "properties")?, key)?)
            })
            .collect::<Option<Vec<_>>>();
        let Some(constraints) = constraints else { continue };
        let mut discriminator_path = path.to_vec();
        discriminator_path.push(SnapshotDetailPathSegment::Key(key.clone()));
        let Some(value) = read(&discriminator_path) else { continue };
        let matching = constraints.iter().enumerate().filter_map(|(candidate_index, expected)| expected.iter().any(|expected| *expected == &value).then_some(candidates[candidate_index].0)).collect::<Vec<_>>();
        if let [index] = matching.as_slice() {
            return members.get(*index);
        }
    }
    None
}

fn selected_schema<'a>(
    root: &'a DslValue,
    schema: &'a DslValue,
    path: &[SnapshotDetailPathSegment],
    read: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<DslValue>,
    shape: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<ValueShape>,
) -> Option<&'a DslValue> {
    let schema = resolved_schema(root, schema, 0)?;
    let Some(members) = union_members(schema) else { return Some(schema) };
    let selected = selected_union_member(root, members, path, read, shape)?;
    selected_schema(root, selected, path, read, shape)
}

pub(super) fn object_schema<'a>(
    root: &'a DslValue,
    schema: &'a DslValue,
    path: &[SnapshotDetailPathSegment],
    read: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<DslValue>,
    shape: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<ValueShape>,
) -> Option<&'a DslValue> {
    let schema = selected_schema(root, schema, path, read, shape)?;
    let is_object = matches!(object_field(schema, "type"), Some(DslValue::String(kind)) if kind == "object") || object_field(schema, "properties").is_some();
    is_object.then_some(schema)
}

fn schema_for_segment<'a>(
    root: &'a DslValue,
    schema: &'a DslValue,
    path: &[SnapshotDetailPathSegment],
    segment: &SnapshotDetailPathSegment,
    read: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<DslValue>,
    shape: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<ValueShape>,
) -> Option<&'a DslValue> {
    let resolved = resolved_schema(root, schema, 0)?;
    let schema = if let Some(members) = union_members(resolved) {
        match selected_union_member(root, members, path, read, shape) {
            Some(selected) => selected_schema(root, selected, path, read, shape)?,
            None => {
                let candidates = members.iter().filter_map(|member| schema_for_segment(root, member, path, segment, read, shape)).collect::<Vec<_>>();
                let first = *candidates.first()?;
                return candidates.iter().all(|candidate| *candidate == first).then_some(first);
            }
        }
    } else {
        resolved
    };
    match segment {
        SnapshotDetailPathSegment::Key(key) => object_field(object_field(schema, "properties")?, key).or_else(|| match object_field(schema, "additionalProperties") {
            Some(value @ DslValue::Object(_)) => Some(value),
            _ => None,
        }),
        SnapshotDetailPathSegment::Index(index) => match object_field(schema, "items")? {
            DslValue::Array(items) => items.get(*index).or_else(|| items.last()),
            items => Some(items),
        },
    }
}

pub(super) fn schema_at_path<'a>(
    root: &'a DslValue,
    path: &[SnapshotDetailPathSegment],
    read: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<DslValue>,
    shape: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<ValueShape>,
) -> Option<&'a DslValue> {
    let mut schema = root;
    let mut current = Vec::with_capacity(path.len());
    for segment in path {
        schema = schema_for_segment(root, schema, &current, segment, read, shape)?;
        current.push(segment.clone());
    }
    selected_schema(root, schema, &current, read, shape)
}

pub(super) fn creation_schema_at_path<'a>(
    root: &'a DslValue,
    path: &[SnapshotDetailPathSegment],
    read: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<DslValue>,
    shape: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<ValueShape>,
) -> Option<&'a DslValue> {
    let mut schema = root;
    let mut current = Vec::with_capacity(path.len());
    for segment in path {
        schema = schema_for_segment(root, schema, &current, segment, read, shape)?;
        current.push(segment.clone());
    }
    resolved_schema(root, schema, 0)
}

pub(super) fn schema_is_null(root: &DslValue, schema: &DslValue) -> bool {
    matches!(resolved_schema(root, schema, 0).and_then(|schema| object_field(schema, "type")), Some(DslValue::String(kind)) if kind == "null")
}

pub(super) fn schema_number(value: Option<&DslValue>) -> Option<Number> {
    match value {
        Some(DslValue::Number(number)) => Some(*number),
        _ => None,
    }
}

pub(super) fn schema_usize(value: Option<&DslValue>) -> Option<usize> {
    match schema_number(value)? {
        Number::UInt(value) => usize::try_from(value).ok(),
        Number::Int(value) if value >= 0 => usize::try_from(value).ok(),
        _ => None,
    }
}

pub(super) fn schema_requires_property(schema: &DslValue, key: &str) -> bool {
    matches!(
        object_field(schema, "required"),
        Some(DslValue::Array(required)) if required.iter().any(|required| matches!(required, DslValue::String(required) if required == key))
    )
}

pub(super) fn schema_accepts_scalar(root: &DslValue, schema: &DslValue, shape: &ValueShape, value: &DslValue) -> bool {
    if !schema_accepts_shape(root, schema, shape) {
        return false;
    }
    schema_enum_constraint(schema).is_none_or(|expected| expected.into_iter().any(|expected| expected == value))
}

pub(super) fn union_discriminator_key(root: &DslValue, schema: &DslValue) -> Option<String> {
    let members = union_members(resolved_schema(root, schema, 0)?)?;
    let first = resolved_schema(root, members.first()?, 0)?;
    let DslValue::Object(properties) = object_field(first, "properties")? else { return None };
    properties.iter().find_map(|(key, _)| {
        members
            .iter()
            .map(|member| {
                let member = resolved_schema(root, member, 0)?;
                schema_enum_constraint(object_field(object_field(member, "properties")?, key)?)
            })
            .collect::<Option<Vec<_>>>()
            .filter(|constraints| constraints.iter().enumerate().all(|(index, values)| constraints.iter().skip(index + 1).all(|other| values.iter().all(|value| !other.contains(value)))))
            .map(|_| key.clone())
    })
}

pub(super) fn variant_label(root: &DslValue, schema: &DslValue, locale: Locale) -> Option<String> {
    let schema = resolved_schema(root, schema, 0)?;
    if let Some(label) = localized_schema_annotation(schema, "title", locale) {
        return Some(label);
    }
    let DslValue::Object(properties) = object_field(schema, "properties")? else { return None };
    properties.iter().find_map(|(_, property)| {
        schema_enum_constraint(property)?.into_iter().find_map(|value| match value {
            DslValue::String(value) => Some(value.clone()),
            _ => None,
        })
    })
}

pub(super) fn number_as_f64(value: Number) -> f64 {
    match value {
        Number::UInt(value) => value as f64,
        Number::Int(value) => value as f64,
        Number::Float(value) => value,
    }
}
