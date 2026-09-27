//! 🪟️ Bounded, localized typed snapshot details shared by stdio artifact editors.

use super::{
    snapshot_edit_actions, INSERT_SNAPSHOT_VALUE_ACTION_ID, MOVE_SNAPSHOT_VALUE_ACTION_ID, REMOVE_SNAPSHOT_VALUE_ACTION_ID, RENAME_SNAPSHOT_KEY_ACTION_ID, REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
    SET_SNAPSHOT_VALUE_ACTION_ID,
};
use crate::kernel::{ArtifactDsl, DslValue, Number, ToValue};
use semio_framework_plugin::app::{TextDraftView, TextView, TextWindowKit, TreeWindowKit, WindowKit};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_ui_contract as ui_contract;
use semio_framework_plugin::{
    tree_window_indexed_section, ActionId, Buildable, BuiltNode, HasBase, HasChildren, InteractiveJobClassification, Locale, LocalizedLabel, PanelTreeBuilder, PluginAssemblyError, TreeWindows,
    UiAssemblyResult, UiListBuilder, UiMapBuilder, UiText, UiValue, WindowKindDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode,
};

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
    fn allows_untyped_creation(&self, _path: &[SnapshotDetailPathSegment]) -> bool {
        true
    }
    fn enum_values(&self, _path: &[SnapshotDetailPathSegment]) -> Vec<DslValue> {
        Vec::new()
    }
    fn presentation(&self, _path: &[SnapshotDetailPathSegment], _locale: Locale) -> Option<SnapshotDetailPresentation> {
        None
    }
}

fn object_field<'a>(value: &'a DslValue, key: &str) -> Option<&'a DslValue> {
    let DslValue::Object(entries) = value else { return None };
    entries.iter().find(|(candidate, _)| candidate == key).map(|(_, value)| value)
}

fn local_schema_ref<'a>(root: &'a DslValue, reference: &str) -> Option<&'a DslValue> {
    let mut value = root;
    for segment in reference.strip_prefix("#/")?.split('/') {
        let key = segment.replace("~1", "/").replace("~0", "~");
        value = object_field(value, &key)?;
    }
    Some(value)
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

fn object_schema<'a>(root: &'a DslValue, schema: &'a DslValue) -> Option<&'a DslValue> {
    let schema = resolved_schema(root, schema, 0)?;
    if let Some(members) = union_members(schema) {
        return members.iter().find_map(|member| object_schema(root, member));
    }
    let is_object = matches!(object_field(schema, "type"), Some(DslValue::String(kind)) if kind == "object") || object_field(schema, "properties").is_some();
    is_object.then_some(schema)
}

fn schema_for_segment<'a>(root: &'a DslValue, schema: &'a DslValue, segment: &SnapshotDetailPathSegment) -> Option<&'a DslValue> {
    let schema = resolved_schema(root, schema, 0)?;
    if let Some(members) = union_members(schema) {
        return members.iter().find_map(|member| schema_for_segment(root, member, segment));
    }
    match segment {
        SnapshotDetailPathSegment::Key(key) => object_field(object_field(schema, "properties")?, key)
            .or_else(|| match object_field(schema, "additionalProperties") {
                Some(value @ DslValue::Object(_)) => Some(value),
                _ => None,
            }),
        SnapshotDetailPathSegment::Index(index) => match object_field(schema, "items")? {
            DslValue::Array(items) => items.get(*index).or_else(|| items.last()),
            items => Some(items),
        },
    }
}

fn schema_at_path<'a>(root: &'a DslValue, path: &[SnapshotDetailPathSegment]) -> Option<&'a DslValue> {
    let mut schema = root;
    for segment in path {
        schema = schema_for_segment(root, schema, segment)?;
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

fn template_from_schema(root: &DslValue, schema: &DslValue, depth: usize) -> Option<DslValue> {
    if depth >= 64 {
        return None;
    }
    let schema = resolved_schema(root, schema, depth)?;
    for keyword in ["default", "const", "examples", "enum"] {
        match object_field(schema, keyword) {
            Some(DslValue::Array(values)) if matches!(keyword, "examples" | "enum") => {
                if let Some(value) = values.first() {
                    return Some(value.clone());
                }
            }
            Some(value) if !matches!(keyword, "examples" | "enum") => return Some(value.clone()),
            _ => {}
        }
    }
    if let Some(members) = union_members(schema) {
        return members
            .iter()
            .filter(|member| !schema_is_null(root, member))
            .find_map(|member| template_from_schema(root, member, depth + 1))
            .or_else(|| members.iter().find_map(|member| template_from_schema(root, member, depth + 1)));
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
            Some(DslValue::Object(fields))
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
            Some(DslValue::Array(items))
        }
        "string" => {
            let length = match schema_number(object_field(schema, "minLength")) {
                Some(Number::UInt(value)) => usize::try_from(value).ok()?,
                Some(Number::Int(value)) if value >= 0 => usize::try_from(value).ok()?,
                _ => 0,
            };
            Some(DslValue::String("x".repeat(length)))
        }
        "integer" => Some(DslValue::Number(schema_number(object_field(schema, "minimum")).or_else(|| schema_number(object_field(schema, "maximum"))).unwrap_or(Number::Int(0)))),
        "number" => Some(DslValue::Number(schema_number(object_field(schema, "minimum")).or_else(|| schema_number(object_field(schema, "maximum"))).unwrap_or(Number::Float(0.0)))),
        "boolean" => Some(DslValue::Bool(false)),
        "null" => Some(DslValue::Null),
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

fn snapshot_schema(value: &DslValue, controller_id: Option<&str>) -> Option<DslValue> {
    let DslValue::String(schema_id) = object_field(value, "schema")? else { return None };
    let base_id = if schema_id.starts_with("s.") { schema_id.clone() } else { format!("s.{schema_id}") };
    let dialect = controller_id.and_then(controller_schema_descriptor_ids);
    let source = semio_framework_schema::with_artifact_schema_registry(|registry| {
        registry
            .get(&base_id)
            .or_else(|| {
                let (primary, fallback) = dialect.as_ref()?;
                registry.get(primary).or_else(|| fallback.as_deref().and_then(|id| registry.get(id)))
            })
            .map(|descriptor| descriptor.snapshot.json_schema)
    })?;
    crate::pack::json::from_json_str(source).ok()
}

fn controller_schema_descriptor_ids(controller: &str) -> Option<(String, Option<String>)> {
    let (artifact, dialect) = controller.split_once('@')?;
    let dialect = dialect.split('#').next()?;
    let (standard, subset) = dialect.split_once('/')?;
    if subset == "*" {
        return Some((format!("{artifact}.{standard}.base"), Some(format!("{artifact}.{standard}.any"))));
    }
    Some((format!("{artifact}.{standard}.{subset}"), None))
}

pub struct DslSnapshotDetailsProvider<'a, S> {
    value: DslValue,
    snapshot: Option<&'a S>,
    schema: Option<DslValue>,
}

impl<'a, S: ArtifactDsl + ToValue> DslSnapshotDetailsProvider<'a, S> {
    pub fn new(snapshot: &'a S) -> Self {
        Self::new_with_schema_hint(snapshot, None)
    }

    pub fn new_with_schema_hint(snapshot: &'a S, controller_id: Option<&str>) -> Self {
        let value = snapshot.to_value();
        let schema = snapshot_schema(&value, controller_id);
        Self { value, snapshot: Some(snapshot), schema }
    }

    pub fn from_value(value: DslValue) -> Self {
        let schema = snapshot_schema(&value, None);
        Self { value, snapshot: None, schema }
    }

    pub fn from_value_and_schema(value: DslValue, schema: &str) -> Self {
        Self { value, snapshot: None, schema: crate::pack::json::from_json_str(schema).ok() }
    }

    fn at(&self, path: &[SnapshotDetailPathSegment]) -> Option<&DslValue> {
        let mut value = &self.value;
        for segment in path {
            value = match (value, segment) {
                (DslValue::Object(entries), SnapshotDetailPathSegment::Key(key)) => entries.iter().find(|(candidate, _)| candidate == key).map(|(_, value)| value)?,
                (DslValue::Array(items), SnapshotDetailPathSegment::Index(index)) => items.get(*index)?,
                _ => return None,
            };
        }
        Some(value)
    }
}

impl<S: ArtifactDsl + ToValue> SnapshotDetailsProvider for DslSnapshotDetailsProvider<'_, S> {
    fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
        Some(match self.at(path)? {
            DslValue::Null => SnapshotDetailValue::Null,
            DslValue::Bool(value) => SnapshotDetailValue::Bool(*value),
            DslValue::Number(value) => SnapshotDetailValue::Number(*value),
            DslValue::String(value) => SnapshotDetailValue::String(value.clone()),
            DslValue::Array(_) => SnapshotDetailValue::Array,
            DslValue::Object(_) => SnapshotDetailValue::Object,
        })
    }

    fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
        match self.at(path) {
            Some(DslValue::Array(items)) => items.len(),
            Some(DslValue::Object(entries)) => entries.len(),
            _ => 0,
        }
    }

    fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
        match self.at(path)? {
            DslValue::Object(entries) => entries.get(index).map(|(key, _)| key.clone()),
            _ => None,
        }
    }

    fn source(&self) -> Option<String> {
        self.snapshot.map(super::snapshot_edit_source)
    }

    fn has_source(&self) -> bool {
        self.snapshot.is_some()
    }

    fn creation_template(&self, path: &[SnapshotDetailPathSegment], collection_item: bool) -> Option<DslValue> {
        let root = self.schema.as_ref()?;
        let mut schema = schema_at_path(root, path)?;
        if collection_item {
            let child_keyword = match self.at(path)? {
                DslValue::Array(_) => "items",
                DslValue::Object(_) => "additionalProperties",
                _ => return None,
            };
            schema = if let Some(members) = union_members(schema) {
                members.iter().find_map(|member| resolved_schema(root, member, 0).and_then(|member| object_field(member, child_keyword)))?
            } else {
                object_field(schema, child_keyword)?
            };
            schema = resolved_schema(root, schema, 0)?;
        }
        template_from_schema(root, schema, 0)
    }

    fn missing_property_templates(&self, path: &[SnapshotDetailPathSegment]) -> Vec<(String, DslValue)> {
        let Some(root) = self.schema.as_ref() else { return Vec::new() };
        let Some(schema) = schema_at_path(root, path) else { return Vec::new() };
        let Some(schema) = object_schema(root, schema) else { return Vec::new() };
        let Some(DslValue::Object(properties)) = object_field(schema, "properties") else { return Vec::new() };
        let Some(DslValue::Object(current)) = self.at(path) else { return Vec::new() };
        properties
            .iter()
            .filter(|(key, _)| !current.iter().any(|(present, _)| present == key))
            .filter_map(|(key, schema)| template_from_schema(root, schema, 0).map(|value| (key.clone(), value)))
            .collect()
    }

    fn allows_untyped_creation(&self, path: &[SnapshotDetailPathSegment]) -> bool {
        let Some(root) = self.schema.as_ref() else { return true };
        let Some(schema) = schema_at_path(root, path) else { return true };
        let Some(schema) = resolved_schema(root, schema, 0) else { return true };
        match self.at(path) {
            Some(DslValue::Array(_)) => object_field(schema, "items").is_none(),
            Some(DslValue::Object(_)) => match object_field(schema, "additionalProperties") {
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
        let Some(schema) = schema_at_path(root, path) else { return Vec::new() };
        let mut values = Vec::new();
        schema_enum_values(root, schema, 0, &mut values);
        values
    }

    fn presentation(&self, path: &[SnapshotDetailPathSegment], locale: Locale) -> Option<SnapshotDetailPresentation> {
        let (SnapshotDetailPathSegment::Key(key), parent) = path.split_last()? else { return None };
        let root = self.schema.as_ref()?;
        let parent_schema = object_schema(root, schema_at_path(root, parent)?)?;
        let properties = object_field(parent_schema, "properties")?;
        let schema = object_field(properties, key)?;
        let friendly = localized_schema_annotation(schema, "title", locale).unwrap_or_else(|| humanized_schema_key(key, locale));
        Some(SnapshotDetailPresentation {
            label: format!("{friendly} [{key}]"),
            description: localized_schema_annotation(schema, "description", locale),
        })
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
        values
            .push(UiValue::Text(UiText::try_from_str(&path[start..end]).ok_or_else(|| error("ui.snapshot-details.path-chunk"))?))
            .map_err(|_| error("ui.snapshot-details.path-chunks-capacity"))?;
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
    let builder = ui::input(kind)
        .value(text(value)?)
        .commit(text("blur")?)
        .try_label(title)
        .map_err(|_| error("ui.snapshot-details.input-label"))?
        .try_id(id)
        .map_err(|_| error("ui.snapshot-details.input-id"))?;
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
        return button(
            id,
            title,
            "plus",
            controller_id,
            action_id,
            ui_args([pointer_argument(path, "path", "pathChunks")?, ("value", UiValue::Text(source)), ("valueEncoding", ui_text_value("json")?)])?,
        );
    }
    action_text_draft(
        id,
        source,
        Some("json".into()),
        action_id,
        Some(json_encoded_path_args(path)?),
        labels,
    )
}

/// 📝️ Renders an artifact's natural file source beside its structured canvas, with an explicit
/// localized Apply/Discard draft routed through the artifact-owned whole-document action.
pub fn render_file_source_editor(
    surface_id: &str,
    source: String,
    language: &str,
    action_id: &str,
    root_node_id: &str,
    locale: Locale,
    body: BuiltNode,
) -> UiAssemblyResult<BuiltNode> {
    let labels = labels(locale);
    let source = TextWindowKit::render_draft(&TextDraftView {
        surface_id: surface_id.into(),
        text: source,
        language: Some(language.into()),
        action_id: action_id.into(),
        argument: "value".into(),
        arguments: Some(ui_args([("nodeId", ui_text_value(root_node_id)?)] )?),
        apply_label: labels.apply.into(),
        discard_label: labels.discard.into(),
        conflict_label: labels.conflict.into(),
        applying_label: labels.applying.into(),
        cancel_label: labels.cancel.into(),
        failed_label: labels.failed.into(),
    })?;
    ui::column()
        .try_id(format!("{surface_id}-layout"))
        .map_err(|_| error("ui.file-source.layout-id"))?
        .try_children(vec![source, body])
        .map_err(|_| error("ui.file-source.layout-children"))?
        .try_build()
        .map_err(|_| error("ui.file-source.layout"))
}

fn enum_control(id: &str, path: &str, current: &SnapshotDetailValue, values: &[DslValue], labels: Labels, controller_id: &str) -> UiAssemblyResult<BuiltNode> {
    if values.len() <= ui_contract::UI_VALUE_MAX_ITEMS {
        if let SnapshotDetailValue::String(current) = current {
            if UiText::try_from_str(current).is_some() && values.iter().all(|value| matches!(value, DslValue::String(value) if UiText::try_from_str(value).is_some())) {
                let mut builder = ui::select(text(current)?)
                    .try_label(labels.value)
                    .map_err(|_| error("ui.snapshot-details.enum-label"))?
                    .try_id(format!("{id}-enum"))
                    .map_err(|_| error("ui.snapshot-details.enum-id"))?;
                for value in values {
                    let DslValue::String(value) = value else { unreachable!("checked string enum") };
                    builder = builder.try_item(text(value)?, label(value)?).map_err(|_| error("ui.snapshot-details.enum-option"))?;
                }
                return bind(builder, ui::Trigger::Change, controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, path_args(path)?)?
                    .try_build()
                    .map_err(|_| error("ui.snapshot-details.enum"));
            }
        }
    }
    let controls = values.iter().enumerate().map(|(index, value)| {
        let source = super::snapshot_edit_source(value);
        template_control(&format!("{id}-enum-{index}"), &format!("{} {source}", labels.choose), path, value.clone(), false, labels, controller_id)
    });
    control_group(&format!("{id}-enum"), controls)
}

fn scalar_control(
    id: &str,
    path: &str,
    value: SnapshotDetailValue,
    template: Option<DslValue>,
    enum_values: &[DslValue],
    allows_untyped_creation: bool,
    labels: Labels,
    controller_id: &str,
) -> UiAssemblyResult<BuiltNode> {
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
    let builder = ui::input(ui::InputKind::Text)
        .value(text(key)?)
        .commit(text("blur")?)
        .try_label(labels.key)
        .map_err(|_| error("ui.snapshot-details.key-label"))?
        .try_id(format!("{id}-key"))
        .map_err(|_| error("ui.snapshot-details.key-id"))?;
    bind(builder, ui::Trigger::Commit, controller_id, RENAME_SNAPSHOT_KEY_ACTION_ID, path_args(path)?)?.try_build().map_err(|_| error("ui.snapshot-details.key"))
}

fn collection_insert_path<P: SnapshotDetailsProvider + ?Sized>(provider: &P, path: &[SnapshotDetailPathSegment], value: &SnapshotDetailValue) -> String {
    let base = pointer(path);
    if matches!(value, SnapshotDetailValue::Array) {
        return format!("{base}/-");
    }
    let mut ordinal = 1usize;
    loop {
        let key = if ordinal == 1 { "new".to_string() } else { format!("new{ordinal}") };
        if !(0..provider.child_count(path)).any(|index| provider.object_key(path, index).as_deref() == Some(key.as_str())) {
            return format!("{base}/{}", key.replace('~', "~0").replace('/', "~1"));
        }
        ordinal += 1;
    }
}

fn collection_controls<P: SnapshotDetailsProvider + ?Sized>(provider: &P, path: &[SnapshotDetailPathSegment], value: &SnapshotDetailValue, id: &str, labels: Labels, controller_id: &str) -> UiAssemblyResult<BuiltNode> {
    let insert_path = collection_insert_path(provider, path, value);
    let candidates = [
        (labels.add_text, "\"\""),
        (labels.add_number, "0"),
        (labels.add_true, "true"),
        (labels.add_false, "false"),
        (labels.add_object, "{}"),
        (labels.add_list, "[]"),
        (labels.add_null, "null"),
    ];
    let mut controls = Vec::new();
    if let Some(template) = provider.creation_template(path, true) {
        controls.push(template_control(&format!("{id}-add-template"), labels.create, &insert_path, template, true, labels, controller_id)?);
    }
    for (index, (key, template)) in provider.missing_property_templates(path).into_iter().enumerate() {
        let property_path = format!("{}/{}", pointer(path), key.replace('~', "~0").replace('/', "~1"));
        let title = format!("{} {key}", labels.add);
        controls.push(template_control(&format!("{id}-add-property-{index}"), &title, &property_path, template, true, labels, controller_id)?);
    }
    if provider.allows_untyped_creation(path) {
        for (index, (title, value)) in candidates.into_iter().enumerate() {
            controls.push(button(&format!("{id}-add-{index}"), title, "plus", controller_id, INSERT_SNAPSHOT_VALUE_ACTION_ID, static_value_args(&insert_path, value)?)?);
        }
    }
    control_group(&format!("{id}-add"), controls.into_iter().map(Ok))
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
    labels: Labels,
    controller_id: &str,
) -> UiAssemblyResult<BuiltNode> {
    let mut controls = Vec::new();
    if let Some(key) = key {
        controls.push(rename_control(id, path, key, labels, controller_id));
    }
    if let Some(value) = value {
        controls.push(scalar_control(id, path, value, template, enum_values, allows_untyped_creation, labels, controller_id));
    }
    if let Some((index, length)) = array_index {
        if index > 0 {
            let parent = path.rsplit_once('/').map_or("", |(parent, _)| parent);
            controls.push(button(
                &format!("{id}-up"),
                labels.move_up,
                "arrow-up",
                controller_id,
                MOVE_SNAPSHOT_VALUE_ACTION_ID,
                ui_args([
                    pointer_argument(path, "from", "fromChunks")?,
                    pointer_argument(&format!("{parent}/{}", index - 1), "path", "pathChunks")?,
                ])?,
            ));
        }
        if index + 1 < length {
            let parent = path.rsplit_once('/').map_or("", |(parent, _)| parent);
            controls.push(button(
                &format!("{id}-down"),
                labels.move_down,
                "arrow-down",
                controller_id,
                MOVE_SNAPSHOT_VALUE_ACTION_ID,
                ui_args([
                    pointer_argument(path, "from", "fromChunks")?,
                    pointer_argument(&format!("{parent}/{}", index + 1), "path", "pathChunks")?,
                ])?,
            ));
        }
    }
    if !path.is_empty() {
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
    let controls = if path_is_bindable(&path_pointer) {
        item_controls(&id, &path_pointer, key, array_index, scalar, template, &enum_values, allows_untyped_creation, labels, controller_id)?
    } else {
        text_draft(
            &format!("{id}-source-fallback"),
            provider.source().ok_or_else(|| error("ui.snapshot-details.unbindable-path-without-source"))?,
            None,
            labels,
        )?
    };
    let presentation = provider.presentation(path, labels.locale);
    let title = presentation.as_ref().map_or(title, |presentation| presentation.label.as_str());
    let mut builder = ui_contract::tree_item(label(title)?)
        .default_open(is_collection)
        .try_id(&id)
        .map_err(|_| error("ui.snapshot-details.item-id"))?
        .try_child(controls)
        .map_err(|_| error("ui.snapshot-details.item-controls"))?;
    if let Some(description) = presentation.and_then(|presentation| presentation.description) {
        builder = builder.description(UiText::clipped(&description));
    }
    if is_collection {
        if path_is_bindable(&path_pointer) {
            builder = builder.try_child(collection_controls(provider, path, &value, &id, labels, controller_id)?).map_err(|_| error("ui.snapshot-details.collection-controls"))?;
        }
        let count = provider.child_count(path);
        let children_id = format!("{id}-children");
        let children = tree_window_indexed_section(windows, &children_id, Default::default(), true, count, |index| {
            let (segment, title, key, array_index) = if matches!(value, SnapshotDetailValue::Object) {
                let key = provider.object_key(path, index).ok_or_else(|| error("ui.snapshot-details.object-key"))?;
                (SnapshotDetailPathSegment::Key(key.clone()), key.clone(), Some(key), None)
            } else {
                (SnapshotDetailPathSegment::Index(index), format!("{} {}", labels.item, index + 1), None, Some((index, count)))
            };
            let mut child_path = path.to_vec();
            child_path.push(segment);
            detail_node(provider, &child_path, &title, key.as_deref(), array_index, labels, controller_id, windows)
        })?;
        builder = builder.try_child(children).map_err(|_| error("ui.snapshot-details.item-children"))?;
    }
    builder.try_build().map_err(|_| error("ui.snapshot-details.item"))
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
            children: vec![WindowLayoutWindowNode {
                kind: "window".into(),
                window_kind_id: window_kind_id.into(),
                title: Some(title.into()),
                instance_id: None,
                template_id: None,
                corner: None,
            }],
        })
    };
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![stack(0.68, main_window_kind_id, main_title), stack(0.32, SNAPSHOT_DETAILS_WINDOW_KIND_ID, "Details")],
        }),
    }
}

pub fn render_snapshot_details_provider<P: SnapshotDetailsProvider + ?Sized>(provider: &P, locale: Locale, controller_id: &str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let labels = labels(locale);
    let mut builder = PanelTreeBuilder::new(ROOT_ID)?;
    if provider.has_source() {
        builder = builder.window_section(windows, SOURCE_ID, Some(label(labels.source)?), false, &[()], |_| {
            let source = provider.source().ok_or_else(|| error("ui.snapshot-details.source-unavailable"))?;
            source_row(source, labels)
        })?;
    }
    builder = builder.window_section(windows, ROOT_SECTION_ID, Some(label(labels.details)?), true, &[()], |_| {
        detail_node(provider, &[], labels.details, None, None, labels, controller_id, windows)
    })?;
    builder.build()
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
            Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>(
                $preparation,
                semio_framework_plugin::plugin_app_close_prelude::store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES,
            ))
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
