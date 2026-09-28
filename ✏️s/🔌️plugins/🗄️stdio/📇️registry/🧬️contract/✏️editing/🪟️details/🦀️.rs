//! 🪟️ Bounded, localized typed snapshot details shared by stdio artifact editors.

use super::{snapshot_edit_actions, INSERT_SNAPSHOT_VALUE_ACTION_ID, MOVE_SNAPSHOT_VALUE_ACTION_ID, REMOVE_SNAPSHOT_VALUE_ACTION_ID, RENAME_SNAPSHOT_KEY_ACTION_ID, REPLACE_SNAPSHOT_SOURCE_ACTION_ID, SET_SNAPSHOT_VALUE_ACTION_ID};
use crate::kernel::{ArtifactDsl, DslValue, Number, ToValue, ValueShape};
use semio_framework_plugin::app::{TextDraftView, TextView, TextWindowKit, TreeWindowKit, WindowKit};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::{
    tree_window_indexed_section, ActionId, Buildable, BuiltNode, HasBase, HasChildren, InteractiveJobClassification, Locale, LocalizedLabel, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiListBuilder, UiMapBuilder, UiText, UiValue,
    WindowKindDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode,
};
use semio_framework_ui_contract as ui_contract;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

pub const SNAPSHOT_DETAILS_WINDOW_KIND_ID: &str = "s.stdio.window.snapshot-details";
pub const SNAPSHOT_DETAILS_BODY_KEY: &str = SNAPSHOT_DETAILS_WINDOW_KIND_ID;
const ROOT_ID: &str = "stdio-snapshot-details";
const ROOT_SECTION_ID: &str = "stdio-snapshot-details-fields";
const ROOT_PATH_ID: &str = "stdio-snapshot-details-root";
const SOURCE_ID: &str = "stdio-snapshot-details-source";
const SOURCE_ROW_ID: &str = "stdio-snapshot-details-source-row";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotDetailPathSegment {
    Key(String),
    Index(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SnapshotDetailValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array,
    Object,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotDetailPresentation {
    pub label: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotDetailVariant {
    pub label: String,
    pub value: DslValue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapshotDetailItemCapabilities {
    pub edit: bool,
    pub rename: bool,
    pub remove: bool,
    pub reorder: bool,
}

impl SnapshotDetailItemCapabilities {
    fn for_path(path: &[SnapshotDetailPathSegment]) -> Self {
        Self { edit: true, rename: matches!(path.last(), Some(SnapshotDetailPathSegment::Key(_))), remove: !path.is_empty(), reorder: matches!(path.last(), Some(SnapshotDetailPathSegment::Index(_))) }
    }
}

pub trait SnapshotDetailsProvider {
    fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue>;
    fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize;
    fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String>;
    fn has_source(&self) -> bool {
        false
    }
    fn source(&self) -> Option<String> {
        None
    }
    fn creation_template(&self, _path: &[SnapshotDetailPathSegment], _collection_item: bool) -> Option<DslValue> {
        None
    }
    fn missing_property_templates(&self, _path: &[SnapshotDetailPathSegment]) -> Vec<(String, DslValue)> {
        Vec::new()
    }
    fn missing_property_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
        self.missing_property_templates(path).len()
    }
    fn missing_property_template(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<(String, DslValue)> {
        self.missing_property_templates(path).into_iter().nth(index)
    }
    fn allows_untyped_creation(&self, _path: &[SnapshotDetailPathSegment]) -> bool {
        true
    }
    fn enum_values(&self, _path: &[SnapshotDetailPathSegment]) -> Vec<DslValue> {
        Vec::new()
    }
    fn presentation(&self, _path: &[SnapshotDetailPathSegment], _locale: Locale) -> Option<SnapshotDetailPresentation> {
        None
    }
    fn item_capabilities(&self, path: &[SnapshotDetailPathSegment]) -> SnapshotDetailItemCapabilities {
        SnapshotDetailItemCapabilities::for_path(path)
    }
    fn allows_collection_insert(&self, _path: &[SnapshotDetailPathSegment]) -> bool {
        true
    }
    fn collection_insertion_key(&self, _path: &[SnapshotDetailPathSegment]) -> Option<String> {
        None
    }
    fn variants(&self, _path: &[SnapshotDetailPathSegment], _locale: Locale) -> Vec<SnapshotDetailVariant> {
        Vec::new()
    }
    fn variant_count(&self, path: &[SnapshotDetailPathSegment], locale: Locale) -> usize {
        self.variants(path, locale).len()
    }
    fn variant(&self, path: &[SnapshotDetailPathSegment], locale: Locale, index: usize) -> Option<SnapshotDetailVariant> {
        self.variants(path, locale).into_iter().nth(index)
    }
}

fn object_field<'a>(value: &'a DslValue, key: &str) -> Option<&'a DslValue> {
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

const RESOLVED_DOCUMENTS_KEY: &str = "x-semio-resolved-documents";

fn schema_id(value: &DslValue) -> Option<&str> {
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

fn bundle_snapshot_schema(root: DslValue, available: Vec<DslValue>) -> DslValue {
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

fn resolved_schema<'a>(root: &'a DslValue, schema: &'a DslValue, depth: usize) -> Option<&'a DslValue> {
    if depth >= 64 {
        return None;
    }
    match object_field(schema, "$ref") {
        Some(DslValue::String(reference)) => resolved_schema(root, local_schema_ref(root, reference)?, depth + 1),
        _ => Some(schema),
    }
}

fn union_members(schema: &DslValue) -> Option<&[DslValue]> {
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

fn schema_accepts_shape(root: &DslValue, schema: &DslValue, shape: &ValueShape) -> bool {
    let Some(schema) = resolved_schema(root, schema, 0) else { return false };
    let accepts = |kind: &str| match kind {
        "null" => matches!(shape, ValueShape::Null),
        "boolean" => matches!(shape, ValueShape::Bool),
        "integer" | "number" => matches!(shape, ValueShape::Number),
        "string" => matches!(shape, ValueShape::String),
        "array" => matches!(shape, ValueShape::Array { .. }),
        "object" => matches!(shape, ValueShape::Object { .. }),
        _ => false,
    };
    match object_field(schema, "type") {
        Some(DslValue::String(kind)) => accepts(kind),
        Some(DslValue::Array(kinds)) => kinds.iter().any(|kind| matches!(kind, DslValue::String(kind) if accepts(kind))),
        _ if object_field(schema, "properties").is_some() => matches!(shape, ValueShape::Object { .. }),
        _ if object_field(schema, "items").is_some() => matches!(shape, ValueShape::Array { .. }),
        _ => true,
    }
}

fn selected_union_member<'a>(
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

fn object_schema<'a>(
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

fn schema_at_path<'a>(
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

fn creation_schema_at_path<'a>(
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

fn schema_is_null(root: &DslValue, schema: &DslValue) -> bool {
    matches!(resolved_schema(root, schema, 0).and_then(|schema| object_field(schema, "type")), Some(DslValue::String(kind)) if kind == "null")
}

fn schema_number(value: Option<&DslValue>) -> Option<Number> {
    match value {
        Some(DslValue::Number(number)) => Some(*number),
        _ => None,
    }
}

fn schema_usize(value: Option<&DslValue>) -> Option<usize> {
    match schema_number(value)? {
        Number::UInt(value) => usize::try_from(value).ok(),
        Number::Int(value) if value >= 0 => usize::try_from(value).ok(),
        _ => None,
    }
}

fn schema_requires_property(schema: &DslValue, key: &str) -> bool {
    matches!(
        object_field(schema, "required"),
        Some(DslValue::Array(required)) if required.iter().any(|required| matches!(required, DslValue::String(required) if required == key))
    )
}

fn schema_accepts_scalar(root: &DslValue, schema: &DslValue, shape: &ValueShape, value: &DslValue) -> bool {
    if !schema_accepts_shape(root, schema, shape) {
        return false;
    }
    schema_enum_constraint(schema).is_none_or(|expected| expected.into_iter().any(|expected| expected == value))
}

fn union_discriminator_key(root: &DslValue, schema: &DslValue) -> Option<String> {
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

fn variant_label(root: &DslValue, schema: &DslValue, locale: Locale) -> Option<String> {
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

fn number_as_f64(value: Number) -> f64 {
    match value {
        Number::UInt(value) => value as f64,
        Number::Int(value) => value as f64,
        Number::Float(value) => value,
    }
}

fn template_candidate_is_valid(root: &DslValue, schema: &DslValue, value: &DslValue, depth: usize) -> bool {
    if depth >= 64 {
        return false;
    }
    let Some(schema) = resolved_schema(root, schema, depth) else { return false };
    if let Some(members) = union_members(schema) {
        let matches = members.iter().filter(|member| template_candidate_is_valid(root, member, value, depth + 1)).count();
        return if object_field(schema, "oneOf").is_some() { matches == 1 } else { matches > 0 };
    }
    if let Some(expected) = object_field(schema, "const") {
        if expected != value {
            return false;
        }
    }
    if let Some(DslValue::Array(values)) = object_field(schema, "enum") {
        if !values.contains(value) {
            return false;
        }
    }
    let shape = match value {
        DslValue::Null => ValueShape::Null,
        DslValue::Bool(_) => ValueShape::Bool,
        DslValue::Number(_) => ValueShape::Number,
        DslValue::String(_) => ValueShape::String,
        DslValue::Array(values) => ValueShape::Array { len: values.len() },
        DslValue::Object(values) => ValueShape::Object { len: values.len() },
    };
    if !schema_accepts_shape(root, schema, &shape) {
        return false;
    }
    match value {
        DslValue::String(value) => {
            let length = value.chars().count();
            if schema_usize(object_field(schema, "minLength")).is_some_and(|minimum| length < minimum) || schema_usize(object_field(schema, "maxLength")).is_some_and(|maximum| length > maximum) {
                return false;
            }
        }
        DslValue::Number(value) => {
            let value = number_as_f64(*value);
            if schema_number(object_field(schema, "minimum")).is_some_and(|minimum| value < number_as_f64(minimum)) || schema_number(object_field(schema, "maximum")).is_some_and(|maximum| value > number_as_f64(maximum)) {
                return false;
            }
        }
        DslValue::Array(values) => {
            if schema_usize(object_field(schema, "minItems")).is_some_and(|minimum| values.len() < minimum) || schema_usize(object_field(schema, "maxItems")).is_some_and(|maximum| values.len() > maximum) {
                return false;
            }
            if let Some(items) = object_field(schema, "items") {
                for (index, value) in values.iter().enumerate() {
                    let item = match items {
                        DslValue::Array(items) => items.get(index).or_else(|| items.last()),
                        item => Some(item),
                    };
                    if item.is_some_and(|item| !template_candidate_is_valid(root, item, value, depth + 1)) {
                        return false;
                    }
                }
            }
        }
        DslValue::Object(values) => {
            if schema_usize(object_field(schema, "minProperties")).is_some_and(|minimum| values.len() < minimum) || schema_usize(object_field(schema, "maxProperties")).is_some_and(|maximum| values.len() > maximum) {
                return false;
            }
            if let Some(DslValue::Array(required)) = object_field(schema, "required") {
                if required
                    .iter()
                    .filter_map(|value| match value {
                        DslValue::String(value) => Some(value),
                        _ => None,
                    })
                    .any(|required| !values.iter().any(|(key, _)| key == required))
                {
                    return false;
                }
            }
            for (key, value) in values {
                let property = object_field(schema, "properties").and_then(|properties| object_field(properties, key));
                let property = property.or_else(|| match object_field(schema, "additionalProperties") {
                    Some(value @ DslValue::Object(_)) => Some(value),
                    Some(DslValue::Bool(false)) => return None,
                    _ => None,
                });
                if property.is_some_and(|property| !template_candidate_is_valid(root, property, value, depth + 1)) {
                    return false;
                }
                if property.is_none() && matches!(object_field(schema, "additionalProperties"), Some(DslValue::Bool(false))) {
                    return false;
                }
            }
        }
        _ => {}
    }
    true
}

fn template_validation_schema(root: &DslValue, schema: &DslValue) -> Option<DslValue> {
    let DslValue::Object(mut entries) = resolved_schema(root, schema, 0)?.clone() else { return None };
    for key in ["$schema", "$defs", "definitions", RESOLVED_DOCUMENTS_KEY] {
        if entries.iter().all(|(candidate, _)| candidate != key) {
            if let Some(value) = object_field(root, key) {
                entries.push((key.into(), value.clone()));
            }
        }
    }
    Some(DslValue::Object(entries))
}

fn template_candidate_is_authoritatively_valid(root: &DslValue, schema: &DslValue, value: &DslValue) -> bool {
    static CACHE: OnceLock<Mutex<HashMap<String, semio_framework_schema::OwnedJsonSchemaValidator>>> = OnceLock::new();
    let Some(schema) = template_validation_schema(root, schema) else { return false };
    let schema = crate::pack::json::to_string(&crate::pack::json::from_dsl_value(&schema));
    let validator = CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().ok().and_then(|cache| cache.get(&schema).cloned()).or_else(|| {
        let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(&schema).ok()?;
        CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().ok()?.insert(schema.clone(), validator.clone());
        Some(validator)
    });
    let Some(validator) = validator else { return false };
    let value = crate::pack::json::to_string(&crate::pack::json::from_dsl_value(value));
    validator.is_valid_json(&value)
}

fn template_candidate_can_be_offered(root: &DslValue, schema: &DslValue, value: &DslValue, depth: usize) -> bool {
    template_candidate_is_valid(root, schema, value, depth) && template_candidate_is_authoritatively_valid(root, schema, value)
}

fn template_from_schema(root: &DslValue, schema: &DslValue, depth: usize) -> Option<DslValue> {
    if depth >= 64 {
        return None;
    }
    let schema = resolved_schema(root, schema, depth)?;
    for keyword in ["default", "const", "examples", "enum"] {
        match object_field(schema, keyword) {
            Some(DslValue::Array(values)) if matches!(keyword, "examples" | "enum") => {
                if let Some(value) = values.iter().find(|value| template_candidate_can_be_offered(root, schema, value, depth + 1)) {
                    return Some(value.clone());
                }
            }
            Some(value) if !matches!(keyword, "examples" | "enum") && template_candidate_can_be_offered(root, schema, value, depth + 1) => return Some(value.clone()),
            _ => {}
        }
    }
    if let Some(members) = union_members(schema) {
        return members.iter().filter(|member| !schema_is_null(root, member)).find_map(|member| template_from_schema(root, member, depth + 1)).or_else(|| members.iter().find_map(|member| template_from_schema(root, member, depth + 1)));
    }
    if let Some(DslValue::Array(members)) = object_field(schema, "allOf") {
        return members.iter().find_map(|member| template_from_schema(root, member, depth + 1));
    }
    let kind = match object_field(schema, "type") {
        Some(DslValue::String(kind)) => Some(kind.as_str()),
        Some(DslValue::Array(kinds)) => kinds.iter().find_map(|kind| match kind {
            DslValue::String(kind) if kind != "null" => Some(kind.as_str()),
            _ => None,
        }),
        _ if object_field(schema, "properties").is_some() => Some("object"),
        _ if object_field(schema, "items").is_some() => Some("array"),
        _ => None,
    }?;
    match kind {
        "object" => {
            let properties = object_field(schema, "properties");
            let required = match object_field(schema, "required") {
                Some(DslValue::Array(required)) => required.as_slice(),
                _ => &[],
            };
            let mut fields = Vec::with_capacity(required.len());
            for required in required {
                let DslValue::String(key) = required else { return None };
                let property = object_field(properties?, key)?;
                fields.push((key.clone(), template_from_schema(root, property, depth + 1)?));
            }
            let minimum = schema_usize(object_field(schema, "minProperties")).unwrap_or(0);
            if fields.len() < minimum {
                let DslValue::Object(properties) = properties? else { return None };
                for (key, property) in properties {
                    if fields.iter().any(|(field, _)| field == key) {
                        continue;
                    }
                    if let Some(value) = template_from_schema(root, property, depth + 1) {
                        fields.push((key.clone(), value));
                    }
                    if fields.len() >= minimum {
                        break;
                    }
                }
            }
            let candidate = DslValue::Object(fields);
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "array" => {
            let count = match schema_number(object_field(schema, "minItems")) {
                Some(Number::UInt(value)) => usize::try_from(value).ok()?,
                Some(Number::Int(value)) if value >= 0 => usize::try_from(value).ok()?,
                _ => 0,
            };
            let item = object_field(schema, "items");
            let mut items = Vec::with_capacity(count);
            for _ in 0..count {
                items.push(template_from_schema(root, item?, depth + 1)?);
            }
            let candidate = DslValue::Array(items);
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "string" => {
            let length = match schema_number(object_field(schema, "minLength")) {
                Some(Number::UInt(value)) => usize::try_from(value).ok()?,
                Some(Number::Int(value)) if value >= 0 => usize::try_from(value).ok()?,
                _ => 0,
            };
            let candidate = DslValue::String("x".repeat(length));
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "integer" => {
            let candidate = DslValue::Number(schema_number(object_field(schema, "minimum")).or_else(|| schema_number(object_field(schema, "maximum"))).unwrap_or(Number::Int(0)));
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "number" => {
            let candidate = DslValue::Number(schema_number(object_field(schema, "minimum")).or_else(|| schema_number(object_field(schema, "maximum"))).unwrap_or(Number::Float(0.0)));
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "boolean" => template_candidate_can_be_offered(root, schema, &DslValue::Bool(false), depth + 1).then_some(DslValue::Bool(false)),
        "null" => template_candidate_can_be_offered(root, schema, &DslValue::Null, depth + 1).then_some(DslValue::Null),
        _ => None,
    }
}

fn schema_enum_values(root: &DslValue, schema: &DslValue, depth: usize, values: &mut Vec<DslValue>) {
    if depth >= 64 {
        return;
    }
    let Some(schema) = resolved_schema(root, schema, depth) else { return };
    match object_field(schema, "enum") {
        Some(DslValue::Array(options)) => {
            for option in options {
                if !values.contains(option) {
                    values.push(option.clone());
                }
            }
        }
        _ => {}
    }
    if let Some(option) = object_field(schema, "const") {
        if !values.contains(option) {
            values.push(option.clone());
        }
    }
    if let Some(members) = union_members(schema) {
        for member in members {
            schema_enum_values(root, member, depth + 1, values);
        }
    }
}

fn localized_schema_annotation(schema: &DslValue, key: &str, locale: Locale) -> Option<String> {
    let language = match locale {
        Locale::En => "en",
        Locale::De => "de",
    };
    for annotation in [format!("x-semio-{key}"), key.to_string()] {
        match object_field(schema, &annotation) {
            Some(DslValue::Object(entries)) => {
                if let Some(DslValue::String(value)) = entries.iter().find(|(candidate, _)| candidate == language).map(|(_, value)| value) {
                    return Some(value.clone());
                }
            }
            Some(DslValue::String(value)) if locale == Locale::En => return Some(value.clone()),
            _ => {}
        }
    }
    None
}

fn humanized_schema_key(key: &str, locale: Locale) -> String {
    let mut words = String::with_capacity(key.len() + 8);
    let mut previous_lowercase = false;
    for character in key.chars() {
        if matches!(character, '_' | '-') {
            if !words.ends_with(' ') {
                words.push(' ');
            }
            previous_lowercase = false;
            continue;
        }
        if character.is_uppercase() && previous_lowercase {
            words.push(' ');
        }
        words.extend(character.to_lowercase());
        previous_lowercase = character.is_lowercase();
    }
    let words = words.trim();
    let mut result = String::with_capacity(words.len());
    let mut characters = words.chars();
    if let Some(first) = characters.next() {
        result.extend(first.to_uppercase());
        result.extend(characters);
    }
    match (locale, key) {
        (Locale::De, "schema") => "Schemakennung".into(),
        (Locale::De, "description") => "Beschreibung".into(),
        (Locale::De, "title") => "Titel".into(),
        _ => result,
    }
}

fn snapshot_schema(schema_id: &str) -> Option<Arc<DslValue>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Arc<DslValue>>>> = OnceLock::new();
    let base_id = if schema_id.starts_with("s.") { schema_id.to_string() } else { format!("s.{schema_id}") };
    if let Some(schema) = CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().ok()?.get(&base_id).cloned() {
        return Some(schema);
    }
    let (source, documents) = semio_framework_schema::with_artifact_schema_registry(|registry| {
        let source = registry.get(&base_id)?.snapshot.json_schema;
        Some((source, registry.iter().map(|descriptor| descriptor.snapshot.json_schema).collect::<Vec<_>>()))
    })?;
    let root = crate::pack::json::from_json_str(source).ok()?;
    let documents = documents.into_iter().filter_map(|source| crate::pack::json::from_json_str(source).ok()).collect();
    let schema = Arc::new(bundle_snapshot_schema(root, documents));
    CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().ok()?.insert(base_id, schema.clone());
    Some(schema)
}

enum DslSnapshotDetailsValue<'a, S> {
    Snapshot(&'a S),
    Materialized(DslValue),
}

pub struct DslSnapshotDetailsProvider<'a, S> {
    value: DslSnapshotDetailsValue<'a, S>,
    schema: Option<Arc<DslValue>>,
}

impl<'a, S: ArtifactDsl + ToValue> DslSnapshotDetailsProvider<'a, S> {
    pub fn new(snapshot: &'a S) -> Self {
        Self::new_with_schema_hint(snapshot, None)
    }

    pub fn new_with_schema_hint(snapshot: &'a S, _controller_id: Option<&str>) -> Self {
        let schema = match snapshot.value_at_path(&["schema"]) {
            Ok(DslValue::String(schema_id)) => snapshot_schema(&schema_id),
            _ => None,
        };
        Self { value: DslSnapshotDetailsValue::Snapshot(snapshot), schema }
    }

    pub fn from_value(value: DslValue) -> Self {
        let schema = match value.value_at_path(&["schema"]) {
            Ok(DslValue::String(schema_id)) => snapshot_schema(&schema_id),
            _ => None,
        };
        Self { value: DslSnapshotDetailsValue::Materialized(value), schema }
    }

    pub fn from_value_and_schema(value: DslValue, schema: &str) -> Self {
        Self { value: DslSnapshotDetailsValue::Materialized(value), schema: crate::pack::json::from_json_str(schema).ok().map(|schema| Arc::new(bundle_snapshot_schema(schema, Vec::new()))) }
    }

    fn path(path: &[SnapshotDetailPathSegment]) -> Vec<String> {
        path.iter()
            .map(|segment| match segment {
                SnapshotDetailPathSegment::Key(key) => key.clone(),
                SnapshotDetailPathSegment::Index(index) => index.to_string(),
            })
            .collect()
    }

    fn with_path<R>(&self, path: &[SnapshotDetailPathSegment], read: impl FnOnce(&dyn ToValue, &[&str]) -> Result<R, crate::kernel::ValueError>) -> Option<R> {
        let path = Self::path(path);
        let path = path.iter().map(String::as_str).collect::<Vec<_>>();
        match &self.value {
            DslSnapshotDetailsValue::Snapshot(snapshot) => read(*snapshot, &path).ok(),
            DslSnapshotDetailsValue::Materialized(value) => read(value, &path).ok(),
        }
    }

    fn shape(&self, path: &[SnapshotDetailPathSegment]) -> Option<ValueShape> {
        self.with_path(path, |value, path| value.value_shape_at_path(path))
    }

    fn scalar(&self, path: &[SnapshotDetailPathSegment]) -> Option<DslValue> {
        self.with_path(path, |value, path| value.value_at_path(path))
    }

    fn schema_at_path<'b>(&'b self, root: &'b DslValue, path: &[SnapshotDetailPathSegment]) -> Option<&'b DslValue> {
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        schema_at_path(root, path, &mut read, &mut shape)
    }

    fn object_schema_at_path<'b>(&'b self, root: &'b DslValue, path: &[SnapshotDetailPathSegment]) -> Option<&'b DslValue> {
        let schema = self.schema_at_path(root, path)?;
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        object_schema(root, schema, path, &mut read, &mut shape)
    }

    fn creation_schema_at_path<'b>(&'b self, root: &'b DslValue, path: &[SnapshotDetailPathSegment]) -> Option<&'b DslValue> {
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        creation_schema_at_path(root, path, &mut read, &mut shape)
    }

    fn variant_from_member(&self, root: &DslValue, current: Option<&DslValue>, member: &DslValue, path: &[SnapshotDetailPathSegment], locale: Locale) -> Option<SnapshotDetailVariant> {
        let target = resolved_schema(root, member, 0)?;
        let mut value = template_from_schema(root, target, 0)?;
        let DslValue::Object(fields) = &mut value else { return None };
        let DslValue::Object(target_properties) = object_field(target, "properties")? else { return None };
        let current_properties = current.and_then(|schema| resolved_schema(root, schema, 0)).and_then(|schema| object_field(schema, "properties"));
        if let Some(DslValue::Object(current_properties)) = current_properties {
            for (key, current_schema) in current_properties {
                let Some((_, target_schema)) = target_properties.iter().find(|(target, _)| target == key) else { continue };
                if resolved_schema(root, current_schema, 0) != resolved_schema(root, target_schema, 0) {
                    continue;
                }
                let mut field_path = path.to_vec();
                field_path.push(SnapshotDetailPathSegment::Key(key.clone()));
                let Some(field_shape) = self.shape(&field_path) else { continue };
                if !matches!(field_shape, ValueShape::Null | ValueShape::Bool | ValueShape::Number | ValueShape::String) {
                    continue;
                }
                let Some(field_value) = self.scalar(&field_path) else { continue };
                if !schema_accepts_scalar(root, target_schema, &field_shape, &field_value) {
                    continue;
                }
                if let Some((_, value)) = fields.iter_mut().find(|(field, _)| field == key) {
                    *value = field_value;
                } else {
                    fields.push((key.clone(), field_value));
                }
            }
        }
        if !template_candidate_can_be_offered(root, target, &value, 0) {
            return None;
        }
        Some(SnapshotDetailVariant { label: variant_label(root, target, locale)?, value })
    }
}

impl<S: ArtifactDsl + ToValue> SnapshotDetailsProvider for DslSnapshotDetailsProvider<'_, S> {
    fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
        Some(match self.shape(path)? {
            ValueShape::Null => SnapshotDetailValue::Null,
            ValueShape::Bool => match self.scalar(path)? {
                DslValue::Bool(value) => SnapshotDetailValue::Bool(value),
                _ => return None,
            },
            ValueShape::Number => match self.scalar(path)? {
                DslValue::Number(value) => SnapshotDetailValue::Number(value),
                _ => return None,
            },
            ValueShape::String => match self.scalar(path)? {
                DslValue::String(value) => SnapshotDetailValue::String(value),
                _ => return None,
            },
            ValueShape::Array { .. } => SnapshotDetailValue::Array,
            ValueShape::Object { .. } => SnapshotDetailValue::Object,
        })
    }

    fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
        match self.shape(path) {
            Some(ValueShape::Array { len } | ValueShape::Object { len }) => len,
            _ => 0,
        }
    }

    fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
        self.with_path(path, |value, path| value.value_key_at_path(path, index))
    }

    fn source(&self) -> Option<String> {
        match &self.value {
            DslSnapshotDetailsValue::Snapshot(snapshot) => Some(super::snapshot_edit_source(*snapshot)),
            DslSnapshotDetailsValue::Materialized(_) => None,
        }
    }

    fn has_source(&self) -> bool {
        matches!(&self.value, DslSnapshotDetailsValue::Snapshot(_))
    }

    fn creation_template(&self, path: &[SnapshotDetailPathSegment], collection_item: bool) -> Option<DslValue> {
        let root = self.schema.as_ref()?;
        let mut schema = self.creation_schema_at_path(root, path)?;
        if collection_item {
            let child_keyword = match self.shape(path)? {
                ValueShape::Array { .. } => "items",
                ValueShape::Object { .. } => "additionalProperties",
                _ => return None,
            };
            schema = if let Some(members) = union_members(schema) { members.iter().find_map(|member| resolved_schema(root, member, 0).and_then(|member| object_field(member, child_keyword)))? } else { object_field(schema, child_keyword)? };
            schema = resolved_schema(root, schema, 0)?;
        }
        template_from_schema(root, schema, 0)
    }

    fn missing_property_templates(&self, path: &[SnapshotDetailPathSegment]) -> Vec<(String, DslValue)> {
        (0..self.missing_property_count(path)).filter_map(|index| self.missing_property_template(path, index)).collect()
    }

    fn missing_property_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
        let Some(root) = self.schema.as_ref() else { return 0 };
        let Some(schema) = self.object_schema_at_path(root, path) else { return 0 };
        let Some(DslValue::Object(properties)) = object_field(schema, "properties") else { return 0 };
        properties
            .iter()
            .filter(|(key, schema)| {
                let mut property = path.to_vec();
                property.push(SnapshotDetailPathSegment::Key(key.clone()));
                self.shape(&property).is_none() && template_from_schema(root, schema, 0).is_some()
            })
            .count()
    }

    fn missing_property_template(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<(String, DslValue)> {
        let root = self.schema.as_ref()?;
        let schema = self.object_schema_at_path(root, path)?;
        let DslValue::Object(properties) = object_field(schema, "properties")? else { return None };
        properties
            .iter()
            .filter_map(|(key, schema)| {
                let mut property = path.to_vec();
                property.push(SnapshotDetailPathSegment::Key(key.clone()));
                self.shape(&property).is_none().then(|| template_from_schema(root, schema, 0).map(|value| (key.clone(), value))).flatten()
            })
            .nth(index)
    }

    fn allows_untyped_creation(&self, path: &[SnapshotDetailPathSegment]) -> bool {
        let Some(root) = self.schema.as_ref() else { return true };
        let Some(schema) = self.schema_at_path(root, path) else { return true };
        let Some(schema) = resolved_schema(root, schema, 0) else { return true };
        match self.shape(path) {
            Some(ValueShape::Array { .. }) => object_field(schema, "items").is_none(),
            Some(ValueShape::Object { .. }) => match object_field(schema, "additionalProperties") {
                Some(DslValue::Bool(allowed)) => *allowed,
                Some(DslValue::Object(_)) => false,
                None => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn enum_values(&self, path: &[SnapshotDetailPathSegment]) -> Vec<DslValue> {
        let Some(root) = self.schema.as_ref() else { return Vec::new() };
        let Some(schema) = self.schema_at_path(root, path) else { return Vec::new() };
        let mut values = Vec::new();
        schema_enum_values(root, schema, 0, &mut values);
        values
    }

    fn presentation(&self, path: &[SnapshotDetailPathSegment], locale: Locale) -> Option<SnapshotDetailPresentation> {
        let (SnapshotDetailPathSegment::Key(key), parent) = path.split_last()? else { return None };
        let root = self.schema.as_ref()?;
        let parent_schema = self.object_schema_at_path(root, parent)?;
        let properties = object_field(parent_schema, "properties")?;
        let schema = object_field(properties, key)?;
        let friendly = localized_schema_annotation(schema, "title", locale).unwrap_or_else(|| humanized_schema_key(key, locale));
        Some(SnapshotDetailPresentation { label: format!("{friendly} [{key}]"), description: localized_schema_annotation(schema, "description", locale) })
    }

    fn item_capabilities(&self, path: &[SnapshotDetailPathSegment]) -> SnapshotDetailItemCapabilities {
        let mut capabilities = SnapshotDetailItemCapabilities::for_path(path);
        if matches!(path.last(), Some(SnapshotDetailPathSegment::Key(key)) if key == "schema") {
            return SnapshotDetailItemCapabilities { edit: false, rename: false, remove: false, reorder: false };
        }
        let Some((segment, parent)) = path.split_last() else {
            capabilities.rename = false;
            capabilities.remove = false;
            capabilities.reorder = false;
            return capabilities;
        };
        let Some(root) = self.schema.as_ref() else { return capabilities };
        match segment {
            SnapshotDetailPathSegment::Key(key) => {
                let Some(schema) = self.object_schema_at_path(root, parent) else { return capabilities };
                if matches!(object_field(schema, "properties"), Some(properties) if object_field(properties, key).is_some()) {
                    capabilities.rename = false;
                }
                if object_field(schema, "properties").and_then(|properties| object_field(properties, key)).and_then(|property| resolved_schema(root, property, 0)).and_then(|property| object_field(property, "const")).is_some() {
                    capabilities.edit = false;
                }
                if schema_requires_property(schema, key) {
                    capabilities.remove = false;
                }
                if schema_usize(object_field(schema, "minProperties")).is_some_and(|minimum| self.child_count(parent) <= minimum) {
                    capabilities.remove = false;
                }
                let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
                let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
                if creation_schema_at_path(root, parent, &mut read, &mut shape).and_then(|schema| union_discriminator_key(root, schema)).as_deref() == Some(key) {
                    capabilities.edit = false;
                }
                capabilities.reorder = false;
            }
            SnapshotDetailPathSegment::Index(_) => {
                let Some(schema) = self.schema_at_path(root, parent) else { return capabilities };
                let length = self.child_count(parent);
                capabilities.rename = false;
                capabilities.remove = schema_usize(object_field(schema, "minItems")).is_none_or(|minimum| length > minimum);
            }
        }
        capabilities
    }

    fn allows_collection_insert(&self, path: &[SnapshotDetailPathSegment]) -> bool {
        let Some(root) = self.schema.as_ref() else { return true };
        let Some(schema) = self.schema_at_path(root, path) else { return true };
        let length = self.child_count(path);
        match self.shape(path) {
            Some(ValueShape::Array { .. }) => schema_usize(object_field(schema, "maxItems")).is_none_or(|maximum| length < maximum),
            Some(ValueShape::Object { .. }) => schema_usize(object_field(schema, "maxProperties")).is_none_or(|maximum| length < maximum),
            _ => false,
        }
    }

    fn collection_insertion_key(&self, path: &[SnapshotDetailPathSegment]) -> Option<String> {
        if !matches!(self.shape(path), Some(ValueShape::Object { .. })) {
            return None;
        }
        let length = self.child_count(path);
        for ordinal in [length.saturating_add(1), length.saturating_add(2), length.saturating_add(3), length.saturating_add(4)] {
            let key = format!("new{ordinal}");
            let mut candidate = path.to_vec();
            candidate.push(SnapshotDetailPathSegment::Key(key.clone()));
            if self.shape(&candidate).is_none() {
                return Some(key);
            }
        }
        None
    }

    fn variants(&self, path: &[SnapshotDetailPathSegment], locale: Locale) -> Vec<SnapshotDetailVariant> {
        (0..self.variant_count(path, locale)).filter_map(|index| self.variant(path, locale, index)).collect()
    }

    fn variant_count(&self, path: &[SnapshotDetailPathSegment], locale: Locale) -> usize {
        let Some(root) = self.schema.as_ref() else { return 0 };
        let Some(schema) = self.creation_schema_at_path(root, path) else { return 0 };
        let Some(members) = union_members(schema) else { return 0 };
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        let current = selected_union_member(root, members, path, &mut read, &mut shape);
        members.iter().filter(|member| current.is_none_or(|current| !std::ptr::eq(*member, current))).filter(|member| self.variant_from_member(root, current, member, path, locale).is_some()).count()
    }

    fn variant(&self, path: &[SnapshotDetailPathSegment], locale: Locale, index: usize) -> Option<SnapshotDetailVariant> {
        let root = self.schema.as_ref()?;
        let schema = self.creation_schema_at_path(root, path)?;
        let members = union_members(schema)?;
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        let current = selected_union_member(root, members, path, &mut read, &mut shape);
        members.iter().filter(|member| current.is_none_or(|current| !std::ptr::eq(*member, current))).filter_map(|member| self.variant_from_member(root, current, member, path, locale)).nth(index)
    }
}

#[derive(Clone, Copy)]
struct Labels {
    locale: Locale,
    details: &'static str,
    source: &'static str,
    apply: &'static str,
    discard: &'static str,
    conflict: &'static str,
    applying: &'static str,
    cancel: &'static str,
    failed: &'static str,
    add: &'static str,
    add_text: &'static str,
    add_number: &'static str,
    add_true: &'static str,
    add_false: &'static str,
    add_object: &'static str,
    add_list: &'static str,
    add_null: &'static str,
    create: &'static str,
    choose: &'static str,
    switch_to: &'static str,
    remove: &'static str,
    move_up: &'static str,
    move_down: &'static str,
    key: &'static str,
    item: &'static str,
    value: &'static str,
}

fn labels(locale: Locale) -> Labels {
    if locale == Locale::De {
        Labels {
            locale,
            details: "Details",
            source: "Quelltext",
            apply: "Anwenden",
            discard: "Verwerfen",
            conflict: "Das Dokument wurde geändert. Kopieren Sie Ihren Entwurf oder verwerfen Sie ihn vor dem Anwenden.",
            applying: "Entwurf wird vorbereitet",
            cancel: "Abbrechen",
            failed: "Entwurf konnte nicht angewendet werden",
            add: "Hinzufügen",
            add_text: "Text hinzufügen",
            add_number: "Zahl hinzufügen",
            add_true: "Wahr hinzufügen",
            add_false: "Falsch hinzufügen",
            add_object: "Objekt hinzufügen",
            add_list: "Liste hinzufügen",
            add_null: "Leerwert hinzufügen",
            create: "Gültigen Eintrag erstellen",
            choose: "Auswählen",
            switch_to: "Wechseln zu",
            remove: "Entfernen",
            move_up: "Nach oben",
            move_down: "Nach unten",
            key: "Schlüssel",
            item: "Eintrag",
            value: "Wert",
        }
    } else {
        Labels {
            locale,
            details: "Details",
            source: "Source",
            apply: "Apply",
            discard: "Discard",
            conflict: "The document changed. Copy your draft or discard it before applying.",
            applying: "Preparing draft",
            cancel: "Cancel",
            failed: "The draft could not be applied",
            add: "Add",
            add_text: "Add text",
            add_number: "Add number",
            add_true: "Add true",
            add_false: "Add false",
            add_object: "Add object",
            add_list: "Add list",
            add_null: "Add null",
            create: "Create valid item",
            choose: "Choose",
            switch_to: "Switch to",
            remove: "Remove",
            move_up: "Move up",
            move_down: "Move down",
            key: "Key",
            item: "Item",
            value: "Value",
        }
    }
}

fn error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "snapshot details UI admission failed")
}

fn text(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| error("ui.snapshot-details.text"))
}

fn label(value: &str) -> UiAssemblyResult<ui::Label> {
    Ok(ui::Label(UiText::clipped(value)))
}

fn ui_args(entries: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut map = UiMapBuilder::try_new().ok_or_else(|| error("ui.snapshot-details.arguments"))?;
    for (key, value) in entries {
        map.try_insert(key.to_string(), value).map_err(|_| error("ui.snapshot-details.argument"))?;
    }
    Ok(UiValue::Map(map.finish()))
}

fn ui_text_value(value: &str) -> UiAssemblyResult<UiValue> {
    Ok(UiValue::Text(text(value)?))
}

fn action(controller_id: &str, name: &str) -> UiAssemblyResult<ActionId> {
    ActionId::try_v1(controller_id, name).ok_or_else(|| error("ui.snapshot-details.action-id"))
}

fn bind<B: HasBase>(builder: B, trigger: ui::Trigger, controller_id: &str, action_id: &str, args: UiValue) -> UiAssemblyResult<B> {
    builder.try_on_with(trigger, action(controller_id, action_id)?, args).map_err(|_| error("ui.snapshot-details.action-binding"))
}

fn button(id: &str, title: &str, icon: &str, controller_id: &str, action_id: &str, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    let builder = ui::button(label(title)?).icon(text(icon)?).try_id(id).map_err(|_| error("ui.snapshot-details.button-id"))?;
    bind(builder, ui::Trigger::Activate, controller_id, action_id, args)?.try_build().map_err(|_| error("ui.snapshot-details.button"))
}

fn control_group(id: &str, controls: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<BuiltNode> {
    let mut builder = ui::row().try_id(id).map_err(|_| error("ui.snapshot-details.controls-id"))?;
    for control in controls {
        builder = builder.try_child(control?).map_err(|_| error("ui.snapshot-details.control"))?;
    }
    builder.try_build().map_err(|_| error("ui.snapshot-details.controls"))
}

fn pointer(path: &[SnapshotDetailPathSegment]) -> String {
    let mut pointer = String::new();
    for segment in path {
        pointer.push('/');
        match segment {
            SnapshotDetailPathSegment::Key(key) => pointer.push_str(&key.replace('~', "~0").replace('/', "~1")),
            SnapshotDetailPathSegment::Index(index) => pointer.push_str(&index.to_string()),
        }
    }
    pointer
}

fn path_id(path: &[SnapshotDetailPathSegment]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in pointer(path).bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("sd-{hash:016x}")
}

fn static_value_args(path: &str, value_json: &str) -> UiAssemblyResult<UiValue> {
    let value = match value_json {
        "null" => UiValue::Null,
        "true" => UiValue::Bool(true),
        "false" => UiValue::Bool(false),
        "0" => UiValue::Number(0.0),
        "\"\"" => ui_text_value("")?,
        "[]" => UiValue::List(Default::default()),
        "{}" => UiValue::Map(Default::default()),
        _ => return Err(error("ui.snapshot-details.static-value")),
    };
    let path = pointer_argument(path, "path", "pathChunks")?;
    ui_args([path, ("value", value)])
}

fn path_args(path: &str) -> UiAssemblyResult<UiValue> {
    ui_args([pointer_argument(path, "path", "pathChunks")?])
}

fn json_encoded_path_args(path: &str) -> UiAssemblyResult<UiValue> {
    ui_args([pointer_argument(path, "path", "pathChunks")?, ("valueEncoding", ui_text_value("json")?)])
}

fn pointer_argument(path: &str, direct: &'static str, chunks: &'static str) -> UiAssemblyResult<(&'static str, UiValue)> {
    if let Some(path) = UiText::try_from_str(path) {
        return Ok((direct, UiValue::Text(path)));
    }
    let mut values = UiListBuilder::try_new().ok_or_else(|| error("ui.snapshot-details.path-chunks"))?;
    let mut start = 0;
    while start < path.len() {
        let mut end = (start + ui_contract::UI_TEXT_MAX_BYTES).min(path.len());
        while !path.is_char_boundary(end) {
            end -= 1;
        }
        if end == start {
            return Err(error("ui.snapshot-details.path-chunk-boundary"));
        }
        values.push(UiValue::Text(UiText::try_from_str(&path[start..end]).ok_or_else(|| error("ui.snapshot-details.path-chunk"))?)).map_err(|_| error("ui.snapshot-details.path-chunks-capacity"))?;
        start = end;
    }
    Ok((chunks, UiValue::List(values.finish())))
}

fn path_is_bindable(path: &str) -> bool {
    if UiText::try_from_str(path).is_some() {
        return true;
    }
    let mut start = 0;
    let mut count = 0;
    while start < path.len() {
        let mut end = (start + ui_contract::UI_TEXT_MAX_BYTES).min(path.len());
        while !path.is_char_boundary(end) {
            end -= 1;
        }
        if end == start {
            return false;
        }
        count += 1;
        if count > ui_contract::UI_VALUE_MAX_ITEMS {
            return false;
        }
        start = end;
    }
    true
}

fn read_only_value(id: &str, value: SnapshotDetailValue) -> UiAssemblyResult<BuiltNode> {
    let value = match value {
        SnapshotDetailValue::Null => "null".to_string(),
        SnapshotDetailValue::Bool(value) => value.to_string(),
        SnapshotDetailValue::Number(Number::UInt(value)) => value.to_string(),
        SnapshotDetailValue::Number(Number::Int(value)) => value.to_string(),
        SnapshotDetailValue::Number(Number::Float(value)) => value.to_string(),
        SnapshotDetailValue::String(value) => value,
        SnapshotDetailValue::Array => "[]".to_string(),
        SnapshotDetailValue::Object => "{}".to_string(),
    };
    readonly_long_text(id, value)
}

fn set_input(id: &str, path: &str, kind: ui::InputKind, value: &str, title: &str, controller_id: &str, json_encoded: bool) -> UiAssemblyResult<BuiltNode> {
    let builder = ui::input(kind).value(text(value)?).commit(text("blur")?).try_label(title).map_err(|_| error("ui.snapshot-details.input-label"))?.try_id(id).map_err(|_| error("ui.snapshot-details.input-id"))?;
    let args = if json_encoded { json_encoded_path_args(path)? } else { path_args(path)? };
    bind(builder, ui::Trigger::Commit, controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, args)?.try_build().map_err(|_| error("ui.snapshot-details.input"))
}

fn readonly_long_text(id: &str, value: String) -> UiAssemblyResult<BuiltNode> {
    let surface = TextWindowKit::render_read_only(id, &TextView { text: value, language: None, read_only: true })?;
    ui::column().try_id(id).map_err(|_| error("ui.snapshot-details.long-value-id"))?.try_child(surface).map_err(|_| error("ui.snapshot-details.long-value"))?.try_build().map_err(|_| error("ui.snapshot-details.long-value"))
}

fn text_draft(id: &str, value: String, path: Option<&str>, labels: Labels) -> UiAssemblyResult<BuiltNode> {
    let language = path.is_none().then(|| "json".to_string());
    let (action_id, arguments) = match path {
        Some(path) => (SET_SNAPSHOT_VALUE_ACTION_ID, Some(path_args(path)?)),
        None => (REPLACE_SNAPSHOT_SOURCE_ACTION_ID, None),
    };
    action_text_draft(id, value, language, action_id, arguments, labels)
}

fn action_text_draft(id: &str, value: String, language: Option<String>, action_id: &str, arguments: Option<UiValue>, labels: Labels) -> UiAssemblyResult<BuiltNode> {
    let surface = TextWindowKit::render_draft(&TextDraftView {
        surface_id: id.into(),
        text: value,
        language,
        action_id: action_id.into(),
        argument: "value".into(),
        arguments,
        apply_label: labels.apply.into(),
        discard_label: labels.discard.into(),
        conflict_label: labels.conflict.into(),
        applying_label: labels.applying.into(),
        cancel_label: labels.cancel.into(),
        failed_label: labels.failed.into(),
    })?;
    ui::column().try_id(id).map_err(|_| error("ui.snapshot-details.draft-id"))?.try_child(surface).map_err(|_| error("ui.snapshot-details.draft"))?.try_build().map_err(|_| error("ui.snapshot-details.draft"))
}

fn template_control(id: &str, title: &str, path: &str, template: DslValue, insert: bool, labels: Labels, controller_id: &str) -> UiAssemblyResult<BuiltNode> {
    let action_id = if insert { INSERT_SNAPSHOT_VALUE_ACTION_ID } else { SET_SNAPSHOT_VALUE_ACTION_ID };
    let source = super::snapshot_edit_source(&template);
    if let Some(source) = UiText::try_from_str(&source) {
        return button(id, title, "plus", controller_id, action_id, ui_args([pointer_argument(path, "path", "pathChunks")?, ("value", UiValue::Text(source)), ("valueEncoding", ui_text_value("json")?)])?);
    }
    action_text_draft(id, source, Some("json".into()), action_id, Some(json_encoded_path_args(path)?), labels)
}

fn variant_control(id: &str, title: &str, path: &str, template: DslValue, labels: Labels, controller_id: &str) -> UiAssemblyResult<BuiltNode> {
    let source = super::snapshot_edit_source(&template);
    if let Some(source) = UiText::try_from_str(&source) {
        return button(id, title, "replace", controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, ui_args([pointer_argument(path, "path", "pathChunks")?, ("value", UiValue::Text(source)), ("valueEncoding", ui_text_value("json")?)])?);
    }
    action_text_draft(id, source, None, SET_SNAPSHOT_VALUE_ACTION_ID, Some(json_encoded_path_args(path)?), labels)
}

/// 📝️ Renders an artifact's natural file source beside its structured canvas, with an explicit
/// localized Apply/Discard draft routed through the artifact-owned whole-document action.
pub fn render_file_source_editor(surface_id: &str, source: String, language: &str, action_id: &str, root_node_id: &str, revision: &str, locale: Locale, body: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let labels = labels(locale);
    let source = TextWindowKit::render_draft(&TextDraftView {
        surface_id: surface_id.into(),
        text: source,
        language: Some(language.into()),
        action_id: action_id.into(),
        argument: "value".into(),
        arguments: Some(ui_args([("nodeId", ui_text_value(root_node_id)?), ("revision", ui_text_value(revision)?)])?),
        apply_label: labels.apply.into(),
        discard_label: labels.discard.into(),
        conflict_label: labels.conflict.into(),
        applying_label: labels.applying.into(),
        cancel_label: labels.cancel.into(),
        failed_label: labels.failed.into(),
    })?;
    ui::column().try_id(format!("{surface_id}-layout")).map_err(|_| error("ui.file-source.layout-id"))?.try_children(vec![source, body]).map_err(|_| error("ui.file-source.layout-children"))?.try_build().map_err(|_| error("ui.file-source.layout"))
}

fn enum_control(id: &str, path: &str, current: &SnapshotDetailValue, values: &[DslValue], labels: Labels, controller_id: &str) -> UiAssemblyResult<BuiltNode> {
    if values.len() <= ui_contract::UI_VALUE_MAX_ITEMS {
        if let SnapshotDetailValue::String(current) = current {
            if UiText::try_from_str(current).is_some() && values.iter().all(|value| matches!(value, DslValue::String(value) if UiText::try_from_str(value).is_some())) {
                let mut builder = ui::select(text(current)?).try_label(labels.value).map_err(|_| error("ui.snapshot-details.enum-label"))?.try_id(format!("{id}-enum")).map_err(|_| error("ui.snapshot-details.enum-id"))?;
                for value in values {
                    let DslValue::String(value) = value else { unreachable!("checked string enum") };
                    builder = builder.try_item(text(value)?, label(value)?).map_err(|_| error("ui.snapshot-details.enum-option"))?;
                }
                return bind(builder, ui::Trigger::Change, controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, path_args(path)?)?.try_build().map_err(|_| error("ui.snapshot-details.enum"));
            }
        }
    }
    let controls = values.iter().enumerate().map(|(index, value)| {
        let source = super::snapshot_edit_source(value);
        template_control(&format!("{id}-enum-{index}"), &format!("{} {source}", labels.choose), path, value.clone(), false, labels, controller_id)
    });
    control_group(&format!("{id}-enum"), controls)
}

fn scalar_control(id: &str, path: &str, value: SnapshotDetailValue, template: Option<DslValue>, enum_values: &[DslValue], allows_untyped_creation: bool, labels: Labels, controller_id: &str) -> UiAssemblyResult<BuiltNode> {
    if !enum_values.is_empty() {
        return enum_control(id, path, &value, enum_values, labels, controller_id);
    }
    match value {
        SnapshotDetailValue::Bool(value) => {
            let builder = ui::toggle(value).try_label(labels.value).map_err(|_| error("ui.snapshot-details.toggle-label"))?.try_id(format!("{id}-toggle")).map_err(|_| error("ui.snapshot-details.toggle-id"))?;
            bind(builder, ui::Trigger::Change, controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, path_args(path)?)?.try_build().map_err(|_| error("ui.snapshot-details.toggle"))
        }
        SnapshotDetailValue::Number(value) => {
            let value = super::snapshot_edit_source(&DslValue::Number(value));
            set_input(&format!("{id}-number"), path, ui::InputKind::Text, &value, labels.value, controller_id, true)
        }
        SnapshotDetailValue::String(value) if UiText::try_from_str(&value).is_some() => set_input(&format!("{id}-text"), path, ui::InputKind::Text, &value, labels.value, controller_id, false),
        SnapshotDetailValue::String(value) => text_draft(&format!("{id}-draft"), value, Some(path), labels),
        SnapshotDetailValue::Null => {
            let candidates = [(labels.add_text, "\"\""), (labels.add_number, "0"), (labels.add_true, "true"), (labels.add_false, "false"), (labels.add_object, "{}"), (labels.add_list, "[]")];
            let mut controls = Vec::new();
            if let Some(template) = template.filter(|template| !matches!(template, DslValue::Null)) {
                controls.push(template_control(&format!("{id}-null-template"), labels.create, path, template, false, labels, controller_id)?);
            }
            if allows_untyped_creation {
                for (index, (title, value)) in candidates.into_iter().enumerate() {
                    controls.push(button(&format!("{id}-null-{index}"), title, "plus", controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, static_value_args(path, value)?)?);
                }
            }
            control_group(&format!("{id}-null"), controls.into_iter().map(Ok))
        }
        SnapshotDetailValue::Array | SnapshotDetailValue::Object => Err(error("ui.snapshot-details.scalar-kind")),
    }
}

fn rename_control(id: &str, path: &str, key: &str, labels: Labels, controller_id: &str) -> UiAssemblyResult<BuiltNode> {
    if UiText::try_from_str(key).is_none() {
        return action_text_draft(&format!("{id}-key-draft"), key.into(), None, RENAME_SNAPSHOT_KEY_ACTION_ID, Some(path_args(path)?), labels);
    }
    let builder = ui::input(ui::InputKind::Text).value(text(key)?).commit(text("blur")?).try_label(labels.key).map_err(|_| error("ui.snapshot-details.key-label"))?.try_id(format!("{id}-key")).map_err(|_| error("ui.snapshot-details.key-id"))?;
    bind(builder, ui::Trigger::Commit, controller_id, RENAME_SNAPSHOT_KEY_ACTION_ID, path_args(path)?)?.try_build().map_err(|_| error("ui.snapshot-details.key"))
}

fn collection_insert_path<P: SnapshotDetailsProvider + ?Sized>(provider: &P, path: &[SnapshotDetailPathSegment], value: &SnapshotDetailValue) -> Option<String> {
    let base = pointer(path);
    if matches!(value, SnapshotDetailValue::Array) {
        return Some(format!("{base}/-"));
    }
    provider.collection_insertion_key(path).map(|key| format!("{base}/{}", key.replace('~', "~0").replace('/', "~1")))
}

fn collection_controls<P: SnapshotDetailsProvider + ?Sized>(provider: &P, path: &[SnapshotDetailPathSegment], value: &SnapshotDetailValue, id: &str, labels: Labels, controller_id: &str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let insert_path = collection_insert_path(provider, path, value);
    let candidates = [(labels.add_text, "\"\""), (labels.add_number, "0"), (labels.add_true, "true"), (labels.add_false, "false"), (labels.add_object, "{}"), (labels.add_list, "[]"), (labels.add_null, "null")];
    let mut builder = ui::column().try_id(format!("{id}-collection-controls")).map_err(|_| error("ui.snapshot-details.collection-controls-id"))?;
    let variant_count = provider.variant_count(path, labels.locale);
    if variant_count > 0 {
        let variants_id = format!("{id}-variants");
        let variants = tree_window_indexed_section(windows, &variants_id, label(labels.switch_to)?, true, variant_count, |index| {
            let variant = provider.variant(path, labels.locale, index).ok_or_else(|| error("ui.snapshot-details.variant"))?;
            let title = format!("{} {}", labels.switch_to, variant.label);
            variant_control(&format!("{id}-variant-{index}"), &title, &pointer(path), variant.value, labels, controller_id)
        })?;
        builder = builder.try_child(variants).map_err(|_| error("ui.snapshot-details.variant-controls"))?;
    }
    let mut controls = Vec::new();
    if provider.allows_collection_insert(path) {
        if let (Some(insert_path), Some(template)) = (insert_path.as_deref().filter(|path| path_is_bindable(path)), provider.creation_template(path, true)) {
            controls.push(template_control(&format!("{id}-add-template"), labels.create, insert_path, template, true, labels, controller_id)?);
        }
        let property_count = provider.missing_property_count(path);
        if property_count > 0 {
            let properties_id = format!("{id}-missing-properties");
            let properties = tree_window_indexed_section(windows, &properties_id, label(labels.add)?, true, property_count, |index| {
                let (key, template) = provider.missing_property_template(path, index).ok_or_else(|| error("ui.snapshot-details.missing-property"))?;
                let mut property = path.to_vec();
                property.push(SnapshotDetailPathSegment::Key(key.clone()));
                let property_path = pointer(&property);
                if !path_is_bindable(&property_path) {
                    return text_draft(&format!("{id}-add-property-source-{index}"), provider.source().ok_or_else(|| error("ui.snapshot-details.unbindable-property-without-source"))?, None, labels);
                }
                let property_label = provider.presentation(&property, labels.locale).map_or_else(|| key.clone(), |presentation| presentation.label);
                let title = format!("{} {property_label}", labels.add);
                template_control(&format!("{id}-add-property-{index}"), &title, &property_path, template, true, labels, controller_id)
            })?;
            builder = builder.try_child(properties).map_err(|_| error("ui.snapshot-details.missing-property-controls"))?;
        }
    }
    if provider.allows_collection_insert(path) && provider.allows_untyped_creation(path) && insert_path.as_deref().is_some_and(path_is_bindable) {
        let insert_path = insert_path.as_deref().expect("checked bindable insertion path");
        for (index, (title, value)) in candidates.into_iter().enumerate() {
            controls.push(button(&format!("{id}-add-{index}"), title, "plus", controller_id, INSERT_SNAPSHOT_VALUE_ACTION_ID, static_value_args(insert_path, value)?)?);
        }
    }
    if !controls.is_empty() {
        builder = builder.try_child(control_group(&format!("{id}-add"), controls.into_iter().map(Ok))?).map_err(|_| error("ui.snapshot-details.add-controls"))?;
    }
    builder.try_build().map_err(|_| error("ui.snapshot-details.collection-controls"))
}

fn item_controls(
    id: &str,
    path: &str,
    key: Option<&str>,
    array_index: Option<(usize, usize)>,
    value: Option<SnapshotDetailValue>,
    template: Option<DslValue>,
    enum_values: &[DslValue],
    allows_untyped_creation: bool,
    capabilities: SnapshotDetailItemCapabilities,
    labels: Labels,
    controller_id: &str,
) -> UiAssemblyResult<BuiltNode> {
    let mut controls = Vec::new();
    if capabilities.rename {
        let key = key.ok_or_else(|| error("ui.snapshot-details.rename-without-key"))?;
        controls.push(rename_control(id, path, key, labels, controller_id));
    }
    if capabilities.edit {
        if let Some(value) = value {
            controls.push(scalar_control(id, path, value, template, enum_values, allows_untyped_creation, labels, controller_id));
        }
    }
    if capabilities.reorder {
        let (index, length) = array_index.ok_or_else(|| error("ui.snapshot-details.reorder-without-index"))?;
        if index > 0 {
            let parent = path.rsplit_once('/').map_or("", |(parent, _)| parent);
            let destination = format!("{parent}/{}", index - 1);
            if path_is_bindable(&destination) {
                controls.push(button(&format!("{id}-up"), labels.move_up, "arrow-up", controller_id, MOVE_SNAPSHOT_VALUE_ACTION_ID, ui_args([pointer_argument(path, "from", "fromChunks")?, pointer_argument(&destination, "path", "pathChunks")?])?));
            }
        }
        if index + 1 < length {
            let parent = path.rsplit_once('/').map_or("", |(parent, _)| parent);
            let destination = format!("{parent}/{}", index + 1);
            if path_is_bindable(&destination) {
                controls.push(button(
                    &format!("{id}-down"),
                    labels.move_down,
                    "arrow-down",
                    controller_id,
                    MOVE_SNAPSHOT_VALUE_ACTION_ID,
                    ui_args([pointer_argument(path, "from", "fromChunks")?, pointer_argument(&destination, "path", "pathChunks")?])?,
                ));
            }
        }
    }
    if capabilities.remove {
        controls.push(button(&format!("{id}-remove"), labels.remove, "trash-2", controller_id, REMOVE_SNAPSHOT_VALUE_ACTION_ID, path_args(path)?));
    }
    control_group(&format!("{id}-controls"), controls)
}

fn detail_node<P: SnapshotDetailsProvider + ?Sized>(
    provider: &P,
    path: &[SnapshotDetailPathSegment],
    title: &str,
    key: Option<&str>,
    array_index: Option<(usize, usize)>,
    labels: Labels,
    controller_id: &str,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode> {
    let value = provider.value(path).ok_or_else(|| error("ui.snapshot-details.value-missing"))?;
    let id = if path.is_empty() { ROOT_PATH_ID.to_string() } else { path_id(path) };
    let path_pointer = pointer(path);
    let is_collection = matches!(value, SnapshotDetailValue::Array | SnapshotDetailValue::Object);
    let scalar = (!is_collection).then(|| value.clone());
    let template = provider.creation_template(path, false);
    let enum_values = provider.enum_values(path);
    let allows_untyped_creation = provider.allows_untyped_creation(path);
    let capabilities = provider.item_capabilities(path);
    let controls = if path_is_bindable(&path_pointer) {
        item_controls(&id, &path_pointer, key, array_index, scalar, template, &enum_values, allows_untyped_creation, capabilities, labels, controller_id)?
    } else {
        text_draft(&format!("{id}-source-fallback"), provider.source().ok_or_else(|| error("ui.snapshot-details.unbindable-path-without-source"))?, None, labels)?
    };
    let presentation = provider.presentation(path, labels.locale);
    let title = presentation.as_ref().map_or(title, |presentation| presentation.label.as_str());
    let mut builder = ui_contract::tree_item(label(title)?).default_open(is_collection).try_id(&id).map_err(|_| error("ui.snapshot-details.item-id"))?.try_child(controls).map_err(|_| error("ui.snapshot-details.item-controls"))?;
    if let Some(description) = presentation.and_then(|presentation| presentation.description) {
        builder = builder.description(UiText::clipped(&description));
    }
    if is_collection {
        if path_is_bindable(&path_pointer) {
            builder = builder.try_child(collection_controls(provider, path, &value, &id, labels, controller_id, windows)?).map_err(|_| error("ui.snapshot-details.collection-controls"))?;
        }
        let count = provider.child_count(path);
        let children_id = format!("{id}-children");
        let children_label = if labels.locale == Locale::De { "Einträge" } else { "Items" };
        let children = tree_window_indexed_section(windows, &children_id, label(children_label)?, false, count, |index| detail_child(provider, path, &value, index, count, labels, controller_id, windows))?;
        builder = builder.try_child(children).map_err(|_| error("ui.snapshot-details.item-children"))?;
    }
    builder.try_build().map_err(|_| error("ui.snapshot-details.item"))
}

fn detail_child<P: SnapshotDetailsProvider + ?Sized>(
    provider: &P,
    path: &[SnapshotDetailPathSegment],
    value: &SnapshotDetailValue,
    index: usize,
    count: usize,
    labels: Labels,
    controller_id: &str,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode> {
    let (segment, title, key, array_index) = if matches!(value, SnapshotDetailValue::Object) {
        let key = provider.object_key(path, index).ok_or_else(|| error("ui.snapshot-details.object-key"))?;
        (SnapshotDetailPathSegment::Key(key.clone()), key.clone(), Some(key), None)
    } else {
        (SnapshotDetailPathSegment::Index(index), format!("{} {}", labels.item, index + 1), None, Some((index, count)))
    };
    let mut child_path = path.to_vec();
    child_path.push(segment);
    detail_node(provider, &child_path, &title, key.as_deref(), array_index, labels, controller_id, windows)
}

fn source_row(source: String, labels: Labels) -> UiAssemblyResult<BuiltNode> {
    let controls = text_draft("stdio-snapshot-details-source-draft", source, None, labels)?;
    ui_contract::tree_item(label(labels.source)?)
        .try_id(SOURCE_ROW_ID)
        .map_err(|_| error("ui.snapshot-details.source-id"))?
        .try_child(controls)
        .map_err(|_| error("ui.snapshot-details.source-controls"))?
        .try_build()
        .map_err(|_| error("ui.snapshot-details.source"))
}

pub fn snapshot_details_window_definition() -> WindowKindDefinition {
    let mut definition = TreeWindowKit::window_kind();
    definition.id = SNAPSHOT_DETAILS_WINDOW_KIND_ID.into();
    definition.body_key = SNAPSHOT_DETAILS_BODY_KEY.into();
    definition.label = LocalizedLabel::native("Details", "Details");
    definition.icon_id = "list-tree".into();
    definition.actions = snapshot_edit_actions();
    for action in &mut definition.actions {
        action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    }
    definition
}

pub fn snapshot_details_split_layout(main_window_kind_id: &str, main_title: &str) -> WindowLayout {
    let stack = |size, window_kind_id: &str, title: &str| {
        WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: Some(size),
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
        })
    };
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![stack(0.68, main_window_kind_id, main_title), stack(0.32, SNAPSHOT_DETAILS_WINDOW_KIND_ID, "Details")] }) }
}

pub fn render_snapshot_details_provider<P: SnapshotDetailsProvider + ?Sized>(provider: &P, locale: Locale, controller_id: &str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let labels = labels(locale);
    let mut builder = ui_contract::tree().try_id(ROOT_ID).map_err(|_| error("ui.snapshot-details.root-id"))?;
    if provider.has_source() {
        let source = semio_framework_plugin::tree_window_section(windows, SOURCE_ID, label(labels.source)?, false, &[()], |_| {
            let source = provider.source().ok_or_else(|| error("ui.snapshot-details.source-unavailable"))?;
            source_row(source, labels)
        })?;
        builder = builder.try_child(source).map_err(|_| error("ui.snapshot-details.source-section"))?;
    }
    let value = provider.value(&[]).ok_or_else(|| error("ui.snapshot-details.root-value"))?;
    let fields = if matches!(value, SnapshotDetailValue::Object | SnapshotDetailValue::Array) {
        let controls = provider.variant_count(&[], locale) > 0
            || (provider.allows_collection_insert(&[])
                && (provider.missing_property_count(&[]) > 0 || (collection_insert_path(provider, &[], &value).is_some() && (provider.allows_untyped_creation(&[]) || provider.creation_template(&[], true).is_some()))));
        let count = provider.child_count(&[]);
        tree_window_indexed_section(windows, ROOT_SECTION_ID, label(labels.details)?, true, count + usize::from(controls), |index| {
            if controls && index == 0 {
                let controls = collection_controls(provider, &[], &value, ROOT_PATH_ID, labels, controller_id, windows)?;
                return ui_contract::tree_item(label(labels.details)?)
                    .default_open(true)
                    .try_id(ROOT_PATH_ID)
                    .map_err(|_| error("ui.snapshot-details.root-controls-id"))?
                    .try_child(controls)
                    .map_err(|_| error("ui.snapshot-details.root-controls"))?
                    .try_build()
                    .map_err(|_| error("ui.snapshot-details.root-controls"));
            }
            detail_child(provider, &[], &value, index - usize::from(controls), count, labels, controller_id, windows)
        })?
    } else {
        semio_framework_plugin::tree_window_section(windows, ROOT_SECTION_ID, label(labels.details)?, true, &[()], |_| detail_node(provider, &[], labels.details, None, None, labels, controller_id, windows))?
    };
    builder.try_child(fields).map_err(|_| error("ui.snapshot-details.fields"))?.try_build().map_err(|_| error("ui.snapshot-details.root"))
}

pub fn render_snapshot_details<S: ArtifactDsl + ToValue>(snapshot: &S, locale: Locale, controller_id: &str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    render_snapshot_details_provider(&DslSnapshotDetailsProvider::new_with_schema_hint(snapshot, Some(controller_id)), locale, controller_id, windows)
}

#[macro_export]
macro_rules! snapshot_details_editor_support {
    (
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        preparation: $preparation:literal
        $(, bounded_native: $bounded_native:tt)?
        $(, native: {
            factory_name: $native_factory_name:literal,
            factory_type: $native_factory_type:ty,
            contract: $native_contract:expr,
            build: $native_build:path,
            tools: [$($native_tool:literal),+ $(,)?]
        })?
        $(,)?
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
            let mut proofs = $crate::editing::snapshot_edit_bounded_first_step_proofs::<Self>($owner_file, $controller, $artifact_schema);
            $(
                let _ = stringify!($bounded_native);
                proofs.extend($crate::editing::bounded_native_edit_bounded_first_step_proofs::<Self>($owner_file, $controller, $artifact_schema));
            )?
            $($(
                proofs.push(
                    semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<semio_framework_plugin::EditorApp<Self>>(
                        $owner_file,
                        $controller,
                        $native_factory_name,
                        $native_tool,
                        $artifact_schema,
                        $native_contract,
                    )
                    .with_factory_type::<semio_framework_plugin::EditorApp<Self>, $native_factory_type>(),
                );
            )+)?
            proofs
        }

        fn register_tool_job_factories(
            registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>,
        ) -> Result<(), semio_framework_plugin::Fault> {
            $(
                let _ = stringify!($bounded_native);
                $crate::editing::register_bounded_native_edit_tool_factory::<Self>(registry)?;
            )?
            $(
                let controller = registry.controller_id().to_string();
                registry.register(<$native_factory_type>::new(&controller))?;
            )?
            $crate::editing::register_snapshot_edit_tool_factory::<Self>(registry)
        }

        fn build_tool_job(
            request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>,
        ) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, semio_framework_plugin::Fault> {
            if $crate::editing::is_snapshot_edit_action(&request.tool_id) {
                return $crate::editing::build_snapshot_edit_tool_job::<Self>(request);
            }
            $(
                let _ = stringify!($bounded_native);
                if <$crate::editing::BoundedNativeEditToolJobFactory<Self> as semio_framework_plugin::ArtifactOwnedToolJobFactory>::TOOL_IDS.contains(&request.tool_id.as_str()) {
                    return $crate::editing::build_bounded_native_edit_tool_job::<Self>(request);
                }
            )?
            $(return $native_build(request);)?
            Ok(None)
        }

        fn build_document_store_owners() -> Option<semio_framework_plugin::plugin_app_close_prelude::store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
            Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
        }

        fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
            Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
        }

        fn build_artifact_store_one_item_preparation_factory(
        ) -> Option<std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
            let factory = semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>(
                $preparation,
                semio_framework_plugin::plugin_app_close_prelude::store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES,
            );
            $(
                let _ = stringify!($bounded_native);
                let factory = $crate::editing::routed_native_edit_preparation_factory(
                    <Self as $crate::editing::BoundedNativeEditingEditor>::native_edit_preparation_route($preparation),
                    factory,
                );
            )?
            Some(factory)
        }

        fn build_config_store_owners() -> Option<semio_framework_plugin::plugin_app_close_prelude::store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
            Some(semio_framework_plugin::no_config_store_owners())
        }

        fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<semio_framework_plugin::plugin_app_close_prelude::store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
            Some(semio_framework_plugin::no_config_store_disposer())
        }

        fn build_draft_store_owners() -> Option<semio_framework_plugin::plugin_app_close_prelude::store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
            Some(semio_framework_plugin::no_draft_store_owners())
        }

        fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<semio_framework_plugin::plugin_app_close_prelude::store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
            Some(semio_framework_plugin::no_draft_store_disposer())
        }

        fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<semio_framework_plugin::plugin_app_close_prelude::store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
            Some(semio_framework_plugin::no_presence_store_disposer())
        }

        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
        }

        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(semio_framework_plugin::no_presence_peer_retirement_factory())
        }

        fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<semio_framework_plugin::plugin_app_close_prelude::store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
            Some(semio_framework_plugin::no_transient_store_disposer())
        }

        fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::SnapshotRetirementFactory<Self::Transient>>> {
            Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
        }
    };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
