//! ✏️ Schema-erased, typed snapshot editing shared by every stdio artifact editor.

use crate::{kernel, pack, value_derive};
use kernel::{ArtifactDsl, DslValue, FromValue, OpBinary, ToValue};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ArtifactRetainedWorkCapacity, BoundedArtifactCommandWork};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, Fault, FaultCode, FaultOrigin, LocalizedLabel};
use semio_framework_plugin::{
    AppOperationContext, ArtifactBoundedFirstStepProof, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, Dialect, EditorApp, Emit,
    InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
};
use std::collections::{BTreeMap, BTreeSet};

#[path = "🪟️details/🦀️.rs"]
pub mod details;
pub use details::{
    render_file_source_editor, render_snapshot_details, render_snapshot_details_provider, snapshot_details_split_layout, snapshot_details_window_definition, DslSnapshotDetailsProvider, SnapshotDetailPathSegment,
    SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailsProvider, SNAPSHOT_DETAILS_BODY_KEY, SNAPSHOT_DETAILS_WINDOW_KIND_ID,
};

pub const SET_SNAPSHOT_VALUE_ACTION_ID: &str = "setSnapshotValue";
pub const INSERT_SNAPSHOT_VALUE_ACTION_ID: &str = "insertSnapshotValue";
pub const REMOVE_SNAPSHOT_VALUE_ACTION_ID: &str = "removeSnapshotValue";
pub const MOVE_SNAPSHOT_VALUE_ACTION_ID: &str = "moveSnapshotValue";
pub const RENAME_SNAPSHOT_KEY_ACTION_ID: &str = "renameSnapshotKey";
pub const REPLACE_SNAPSHOT_SOURCE_ACTION_ID: &str = "replaceSnapshotSource";
pub const SNAPSHOT_EDIT_ACTION_IDS: &[&str] = &[
    SET_SNAPSHOT_VALUE_ACTION_ID,
    INSERT_SNAPSHOT_VALUE_ACTION_ID,
    REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    MOVE_SNAPSHOT_VALUE_ACTION_ID,
    RENAME_SNAPSHOT_KEY_ACTION_ID,
    REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];

pub fn is_snapshot_edit_action(action: &str) -> bool {
    SNAPSHOT_EDIT_ACTION_IDS.contains(&action)
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
pub enum SnapshotEditEvent {
    SetValue { path: String, value: DslValue },
    InsertValue { path: String, value: DslValue },
    RemoveValue { path: String },
    MoveValue { from: String, path: String },
    RenameKey { path: String, key: String },
    ReplaceSource { source: String },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "channel", rename_all = "camelCase", deny_unknown_fields)]
pub enum SnapshotEditingCommand<C> {
    Native(C),
    Edit(SnapshotEditEvent),
}

pub trait SnapshotEditingNativeCommand: kernel::OpBinary {
    const ALL_TOOL_JOB_IDS: &'static [&'static str];
}

impl<C: kernel::OpBinary> kernel::OpText for SnapshotEditingCommand<C> {
    fn print_op(&self) -> String {
        match self {
            Self::Native(command) => format!("native {}", hex_encode(&command.encode_op().unwrap_or_default())),
            Self::Edit(event) => format!("edit {}", hex_encode(&<SnapshotEditEvent as kernel::OpBinary>::encode_op(event).unwrap_or_default())),
        }
    }

    fn parse_op(line: &str) -> Result<Self, kernel::TextError> {
        let (channel, payload) = line.split_once(' ').ok_or_else(|| kernel::TextError::new("snapshot editing command requires a channel", kernel::TextSpan::at(1, 1)))?;
        let bytes = hex_decode(payload).map_err(|error| kernel::TextError::new(error, kernel::TextSpan::at(1, 1)))?;
        match channel {
            "native" => C::decode_op(&bytes).map(Self::Native).map_err(|error| kernel::TextError::new(error.to_string(), kernel::TextSpan::at(1, 1))),
            "edit" => <SnapshotEditEvent as kernel::OpBinary>::decode_op(&bytes).map(Self::Edit).map_err(|error| kernel::TextError::new(error.to_string(), kernel::TextSpan::at(1, 1))),
            _ => Err(kernel::TextError::new(format!("unknown snapshot editing command channel '{channel}'"), kernel::TextSpan::at(1, 1))),
        }
    }
}

impl<C: SnapshotEditingNativeCommand> kernel::OpBinary for SnapshotEditingCommand<C> {
    const TOOL_JOB_IDS: &'static [&'static str] = C::ALL_TOOL_JOB_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> {
        let (channel, payload) = match self {
            Self::Native(command) => (0, command.encode_op()?),
            Self::Edit(event) => (1, <SnapshotEditEvent as kernel::OpBinary>::encode_op(event)?),
        };
        let mut encoded = Vec::with_capacity(payload.len() + 1);
        encoded.push(channel);
        encoded.extend(payload);
        Ok(encoded)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, kernel::ProtocolError> {
        let Some((&channel, payload)) = bytes.split_first() else {
            return Err(kernel::ProtocolError::Malformed { what: "snapshot editing command", offset: 0, detail: "missing channel".into() });
        };
        match channel {
            0 => C::decode_op(payload).map(Self::Native),
            1 => <SnapshotEditEvent as kernel::OpBinary>::decode_op(payload).map(Self::Edit),
            _ => Err(kernel::ProtocolError::Malformed { what: "snapshot editing command", offset: 0, detail: format!("unknown channel {channel}") }),
        }
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(DIGITS[(byte >> 4) as usize] as char);
        encoded.push(DIGITS[(byte & 15) as usize] as char);
    }
    encoded
}

fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
    if text.len() % 2 != 0 {
        return Err("hex payload has an odd length".into());
    }
    let nibble = |byte: u8| match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(format!("invalid hex digit '{}'", byte as char)),
    };
    text.as_bytes().chunks_exact(2).map(|pair| Ok((nibble(pair[0])? << 4) | nibble(pair[1])?)).collect()
}

#[macro_export]
macro_rules! snapshot_editing_command_roster {
    ($native:ty, [$($native_tool:expr),* $(,)?]) => {
        impl $crate::editing::SnapshotEditingNativeCommand for $native {
            const ALL_TOOL_JOB_IDS: &'static [&'static str] = &[
                $($native_tool,)*
                $crate::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
                $crate::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
                $crate::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
                $crate::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
                $crate::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
                $crate::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
            ];
        }
    };
}

impl SnapshotEditEvent {
    pub fn action_id(&self) -> &'static str {
        match self {
            Self::SetValue { .. } => SET_SNAPSHOT_VALUE_ACTION_ID,
            Self::InsertValue { .. } => INSERT_SNAPSHOT_VALUE_ACTION_ID,
            Self::RemoveValue { .. } => REMOVE_SNAPSHOT_VALUE_ACTION_ID,
            Self::MoveValue { .. } => MOVE_SNAPSHOT_VALUE_ACTION_ID,
            Self::RenameKey { .. } => RENAME_SNAPSHOT_KEY_ACTION_ID,
            Self::ReplaceSource { .. } => REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
        }
    }
}

impl kernel::OpText for SnapshotEditEvent {
    fn print_op(&self) -> String {
        pack::json::to_json_string(self)
    }

    fn parse_op(line: &str) -> Result<Self, kernel::TextError> {
        pack::json::from_json_str(line).map_err(|error| kernel::TextError::new(error.to_string(), kernel::TextSpan::at(1, 1)))
    }
}

impl kernel::OpBinary for SnapshotEditEvent {
    const TOOL_JOB_IDS: &'static [&'static str] = SNAPSHOT_EDIT_ACTION_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> {
        Ok(<Self as kernel::OpText>::print_op(self).into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, kernel::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|error| kernel::ProtocolError::Malformed { what: "snapshot edit utf8", offset: 0, detail: error.to_string() })?;
        <Self as kernel::OpText>::parse_op(line).map_err(|error| kernel::ProtocolError::Malformed { what: "snapshot edit", offset: 0, detail: error.to_string() })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotEditError {
    pub code: &'static str,
    pub path: String,
    pub message: String,
}

impl SnapshotEditError {
    fn new(code: &'static str, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code, path: path.into(), message: message.into() }
    }
}

impl std::fmt::Display for SnapshotEditError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.path.is_empty() {
            write!(formatter, "{}", self.message)
        } else {
            write!(formatter, "{} at {}", self.message, self.path)
        }
    }
}

impl std::error::Error for SnapshotEditError {}

fn decode_pointer(path: &str) -> Result<Vec<String>, SnapshotEditError> {
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

fn object_index(entries: &[(String, DslValue)], key: &str, path: &str) -> Result<usize, SnapshotEditError> {
    let mut matches = entries.iter().enumerate().filter(|(_, (candidate, _))| candidate == key);
    let Some((index, _)) = matches.next() else {
        return Err(SnapshotEditError::new("snapshot-edit.path-missing", path, format!("object key '{key}' does not exist")));
    };
    if matches.next().is_some() {
        return Err(SnapshotEditError::new("snapshot-edit.ambiguous-object", path, format!("object key '{key}' occurs more than once")));
    }
    Ok(index)
}

fn array_index(segment: &str, length: usize, path: &str, insert: bool) -> Result<usize, SnapshotEditError> {
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

fn remove_value(root: &mut DslValue, path: &str) -> Result<DslValue, SnapshotEditError> {
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

fn insert_value(root: &mut DslValue, path: &str, value: DslValue) -> Result<(), SnapshotEditError> {
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

fn set_value(root: &mut DslValue, path: &str, value: DslValue) -> Result<(), SnapshotEditError> {
    let segments = decode_pointer(path)?;
    if segments.is_empty() {
        *root = value;
        return Ok(());
    }
    *value_at_mut(root, &segments, path)? = value;
    Ok(())
}

fn move_value(root: &mut DslValue, from: &str, path: &str) -> Result<(), SnapshotEditError> {
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

fn rename_key(root: &mut DslValue, path: &str, key: &str) -> Result<(), SnapshotEditError> {
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

/// 🧾️ Prints every typed snapshot detail without passing through a native file encoder.
pub fn snapshot_edit_source<S: ToValue>(snapshot: &S) -> String {
    pack::json::to_string_pretty(&pack::json::from_dsl_value(&snapshot.to_value()))
}

fn validate_source_keys(source: &str) -> Result<(), SnapshotEditError> {
    use pack::json::Token;
    let mut lexer = pack::json::Lexer::new(source);
    let mut scopes: Vec<Option<(bool, BTreeSet<String>)>> = Vec::new();
    while let Some(token) = lexer.next_token().map_err(|error| SnapshotEditError::new("snapshot-edit.invalid-source", "", error.to_string()))? {
        match token {
            Token::ObjectStart => scopes.push(Some((true, BTreeSet::new()))),
            Token::ArrayStart => scopes.push(None),
            Token::ObjectEnd | Token::ArrayEnd => { scopes.pop(); }
            Token::Comma => {
                if let Some(Some((key, _))) = scopes.last_mut() { *key = true; }
            }
            Token::String(name) => {
                if let Some(Some((key, names))) = scopes.last_mut() {
                    if *key && !names.insert(name.clone()) {
                        return Err(SnapshotEditError::new("snapshot-edit.ambiguous-object", "", format!("source repeats object key '{name}'")));
                    }
                    *key = false;
                }
            }
            _ => {}
        }
        if scopes.len() > 128 { return Err(SnapshotEditError::new("snapshot-edit.depth-exceeded", "", "snapshot nesting exceeds 128 levels")); }
    }
    Ok(())
}

/// 🔬️ Decodes complete snapshot JSON and refuses any normalized or discarded detail.
pub fn snapshot_from_edit_source<S: FromValue + ToValue>(source: &str) -> Result<S, SnapshotEditError> {
    validate_source_keys(source)?;
    let value: DslValue = pack::json::from_json_str(source).map_err(|error| SnapshotEditError::new("snapshot-edit.invalid-source", "", error.to_string()))?;
    validate_value(&value, "")?;
    let decoded = S::from_value(value.clone()).map_err(|error| SnapshotEditError::new("snapshot-edit.schema-invalid", "", error.to_string()))?;
    if !values_equivalent(&decoded.to_value(), &value) {
        return Err(SnapshotEditError::new("snapshot-edit.lossy-conversion", "", "the typed snapshot would normalize or discard part of the source"));
    }
    Ok(decoded)
}

pub fn apply_snapshot_edit<S>(snapshot: &S, event: &SnapshotEditEvent) -> Result<S, SnapshotEditError>
where
    S: ArtifactDsl + ToValue + FromValue,
{
    let original = snapshot.to_value();
    let next = apply_snapshot_edit_unvalidated(snapshot, event)?;
    validate_registered_snapshot_schema(&original, &next.to_value())?;
    Ok(next)
}

/// 🛡️ Applies one edit and atomically enforces an explicitly supplied snapshot schema.
pub fn apply_snapshot_edit_with_schema<S>(snapshot: &S, event: &SnapshotEditEvent, schema: &str) -> Result<S, SnapshotEditError>
where
    S: ArtifactDsl + ToValue + FromValue,
{
    let next = apply_snapshot_edit_unvalidated(snapshot, event)?;
    validate_snapshot_value_against_schema(&next.to_value(), schema)?;
    Ok(next)
}

/// 🧬️ Applies one edit against the exact schema selected by the editor dialect.
pub fn apply_snapshot_edit_for_dialect<S>(snapshot: &S, event: &SnapshotEditEvent, dialect: Dialect) -> Result<S, SnapshotEditError>
where
    S: ArtifactDsl + ToValue + FromValue,
{
    let original = snapshot.to_value();
    validate_snapshot_schema_for_dialect(&original, dialect)?;
    let next = apply_snapshot_edit_unvalidated(snapshot, event)?;
    let value = next.to_value();
    if snapshot_schema_id(&value) != snapshot_schema_id(&original) {
        return Err(SnapshotEditError::new("snapshot-edit.schema-identity", "$.schema", "an edit cannot change the registered snapshot schema identity"));
    }
    let descriptor_id = snapshot_schema_descriptor_for_dialect(dialect)?;
    let validator = semio_framework_schema::structural_validator_for(&descriptor_id, "snapshot")
        .map_err(|error| SnapshotEditError::new("snapshot-edit.invalid-schema-contract", "", format!("{descriptor_id}: {error}")))?;
    validate_snapshot_value_with_validator(&value, &validator)?;
    Ok(next)
}

fn apply_snapshot_edit_unvalidated<S>(snapshot: &S, event: &SnapshotEditEvent) -> Result<S, SnapshotEditError>
where
    S: ArtifactDsl + ToValue + FromValue,
{
    if let SnapshotEditEvent::ReplaceSource { source } = event {
        return snapshot_from_edit_source(source);
    }
    let mut value = snapshot.to_value();
    match event {
        SnapshotEditEvent::SetValue { path, value: next } => set_value(&mut value, path, next.clone())?,
        SnapshotEditEvent::InsertValue { path, value: next } => insert_value(&mut value, path, next.clone())?,
        SnapshotEditEvent::RemoveValue { path } => {
            remove_value(&mut value, path)?;
        }
        SnapshotEditEvent::MoveValue { from, path } => move_value(&mut value, from, path)?,
        SnapshotEditEvent::RenameKey { path, key } => rename_key(&mut value, path, key)?,
        SnapshotEditEvent::ReplaceSource { .. } => unreachable!(),
    }
    validate_value(&value, "")?;
    let next = S::from_value(value.clone()).map_err(|error| SnapshotEditError::new("snapshot-edit.schema-invalid", "", error.to_string()))?;
    if !values_equivalent(&next.to_value(), &value) {
        return Err(SnapshotEditError::new("snapshot-edit.lossy-conversion", "", "the typed snapshot would normalize or discard part of the edit"));
    }
    Ok(next)
}

/// ✅ Validates a typed snapshot projection against its normative JSON Schema constraints.
pub fn validate_snapshot_value_against_schema(value: &DslValue, schema: &str) -> Result<(), SnapshotEditError> {
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(schema)
        .map_err(|error| SnapshotEditError::new("snapshot-edit.invalid-schema-contract", "", error.to_string()))?;
    validate_snapshot_value_with_validator(value, &validator)
}

fn validate_snapshot_value_with_validator(value: &DslValue, validator: &semio_framework_schema::OwnedJsonSchemaValidator) -> Result<(), SnapshotEditError> {
    let source = pack::json::to_string(&pack::json::from_dsl_value(value));
    validator.validate_json(&source).map(|_| ()).map_err(|error| {
        let message = match error {
            semio_framework_schema::SchemaError::Validation(message) => message,
            other => other.to_string(),
        };
        let path = message.split_once(':').map_or("", |(path, _)| path).trim().to_string();
        SnapshotEditError::new("snapshot-edit.constraint-invalid", path, message)
    })
}

static SNAPSHOT_SCHEMA_VALIDATORS: std::sync::OnceLock<std::sync::RwLock<std::collections::HashMap<String, semio_framework_schema::OwnedJsonSchemaValidator>>> = std::sync::OnceLock::new();

fn snapshot_schema_id(value: &DslValue) -> Option<String> {
    let DslValue::Object(entries) = value else { return None };
    let DslValue::String(schema) = entries.iter().find(|(key, _)| key == "schema").map(|(_, value)| value)? else { return None };
    Some(if schema.starts_with("s.") { schema.clone() } else { format!("s.{schema}") })
}

/// 🧬 Refuses retained editing when an editor's authoritative snapshot identity has no exact dialect schema contract.
pub fn validate_snapshot_schema_for_dialect(value: &DslValue, dialect: Dialect) -> Result<(), SnapshotEditError> {
    let actual = snapshot_schema_id(value);
    let expected = snapshot_schema_descriptor_for_dialect(dialect)?;
    if actual.as_deref() == Some(dialect.artifact_kind) || actual.as_deref() == Some(expected.as_str()) {
        return Ok(());
    }
    Err(SnapshotEditError::new(
        "snapshot-edit.schema-identity",
        "$.schema",
        format!("snapshot schema identity '{}' does not match registered editor schema '{expected}'", actual.as_deref().unwrap_or("<missing>")),
    ))
}

fn snapshot_schema_descriptor_for_dialect(dialect: Dialect) -> Result<String, SnapshotEditError> {
    semio_framework_schema::with_artifact_schema_registry(|artifacts| {
        let explicit = format!("{}.{}.{}", dialect.artifact_kind, dialect.standard.0, dialect.subset.0);
        if dialect.subset.0 != "*" && artifacts.get(&explicit).is_some() {
            return Ok(explicit);
        }
        if dialect.subset.0 == "*" {
            for subset in ["base", "any"] {
                let candidate = format!("{}.{}.{}", dialect.artifact_kind, dialect.standard.0, subset);
                if artifacts.get(&candidate).is_some() {
                    return Ok(candidate);
                }
            }
        }
        Err(SnapshotEditError::new(
            "snapshot-edit.schema-unregistered",
            "$.schema",
            format!("no snapshot schema is registered for {}@{}/{}", dialect.artifact_kind, dialect.standard.0, dialect.subset.0),
        ))
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
    semio_framework_schema::with_artifact_schema_registry(|artifacts| {
        if artifacts.get(&id).is_none() {
            return Ok(());
        }
        if candidate_schema != Some(&DslValue::String(schema.clone())) {
            return Err(SnapshotEditError::new("snapshot-edit.schema-identity", "$.schema", "an edit cannot change the registered snapshot schema identity"));
        }
        let validator = semio_framework_schema::structural_validator_for(&id, "snapshot")
            .map_err(|error| SnapshotEditError::new("snapshot-edit.invalid-schema-contract", "", format!("{id}: {error}")))?;
        let result = validate_snapshot_value_with_validator(value, &validator);
        validators.write().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(id, validator);
        result
    })
}

fn validate_value(value: &DslValue, path: &str) -> Result<(), SnapshotEditError> {
    validate_value_at_depth(value, path, 0)
}

fn validate_value_at_depth(value: &DslValue, path: &str, depth: usize) -> Result<(), SnapshotEditError> {
    if depth > 128 {
        return Err(SnapshotEditError::new("snapshot-edit.depth-exceeded", path, "snapshot nesting exceeds 128 levels"));
    }
    match value {
        DslValue::Number(kernel::Number::Float(value)) if !value.is_finite() => Err(SnapshotEditError::new("snapshot-edit.non-finite-number", path, "snapshot numbers must be finite")),
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

fn values_equivalent(left: &DslValue, right: &DslValue) -> bool {
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
        (DslValue::Number(kernel::Number::Float(float)), DslValue::Number(kernel::Number::UInt(integer)))
        | (DslValue::Number(kernel::Number::UInt(integer)), DslValue::Number(kernel::Number::Float(float))) => *integer <= 9_007_199_254_740_991 && *float == *integer as f64,
        (DslValue::Number(kernel::Number::Float(float)), DslValue::Number(kernel::Number::Int(integer)))
        | (DslValue::Number(kernel::Number::Int(integer)), DslValue::Number(kernel::Number::Float(float))) => integer.unsigned_abs() <= 9_007_199_254_740_991 && *float == *integer as f64,
        _ => left == right,
    }
}

fn edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

fn argument<'a>(args: Option<&'a DslValue>, key: &str) -> Result<&'a DslValue, Fault> {
    let Some(DslValue::Object(entries)) = args else {
        return Err(edit_fault("snapshot-edit.arguments-required", "snapshot edit arguments must be an object"));
    };
    let mut matches = entries.iter().filter(|(candidate, _)| candidate == key);
    let value = matches.next().map(|(_, value)| value).ok_or_else(|| edit_fault("snapshot-edit.argument-missing", format!("snapshot edit argument '{key}' is required")))?;
    if matches.next().is_some() {
        return Err(edit_fault("snapshot-edit.argument-duplicate", format!("snapshot edit argument '{key}' occurs more than once")));
    }
    Ok(value)
}

fn text_argument(args: Option<&DslValue>, key: &str) -> Result<String, Fault> {
    match argument(args, key)? {
        DslValue::String(value) => Ok(value.clone()),
        _ => Err(edit_fault("snapshot-edit.argument-type", format!("snapshot edit argument '{key}' must be text"))),
    }
}

fn optional_argument<'a>(args: Option<&'a DslValue>, key: &str) -> Result<Option<&'a DslValue>, Fault> {
    let Some(DslValue::Object(entries)) = args else {
        return Err(edit_fault("snapshot-edit.arguments-required", "snapshot edit arguments must be an object"));
    };
    let mut matches = entries.iter().filter(|(candidate, _)| candidate == key);
    let value = matches.next().map(|(_, value)| value);
    if matches.next().is_some() {
        return Err(edit_fault("snapshot-edit.argument-duplicate", format!("snapshot edit argument '{key}' occurs more than once")));
    }
    Ok(value)
}

fn pointer_argument(args: Option<&DslValue>, direct: &str, chunks: &str) -> Result<String, Fault> {
    match (optional_argument(args, direct)?, optional_argument(args, chunks)?) {
        (Some(DslValue::String(path)), None) => Ok(path.clone()),
        (None, Some(DslValue::Array(parts))) => {
            let mut bytes = 0usize;
            for part in parts {
                let DslValue::String(part) = part else {
                    return Err(edit_fault("snapshot-edit.argument-type", format!("snapshot edit argument '{chunks}' must contain only text")));
                };
                bytes = bytes.checked_add(part.len()).filter(|bytes| *bytes <= SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES).ok_or_else(|| {
                    edit_fault("snapshot-edit.path-too-large", format!("snapshot edit argument '{chunks}' exceeds the bounded path extent"))
                })?;
            }
            let mut path = String::with_capacity(bytes);
            for part in parts {
                let DslValue::String(part) = part else { unreachable!("validated path chunk") };
                path.push_str(part);
            }
            Ok(path)
        }
        (Some(_), None) => Err(edit_fault("snapshot-edit.argument-type", format!("snapshot edit argument '{direct}' must be text"))),
        (None, Some(_)) => Err(edit_fault("snapshot-edit.argument-type", format!("snapshot edit argument '{chunks}' must be a text list"))),
        (Some(_), Some(_)) => Err(edit_fault("snapshot-edit.path-shape", format!("snapshot edit arguments must contain exactly one of '{direct}' or '{chunks}'"))),
        (None, None) => Err(edit_fault("snapshot-edit.argument-missing", format!("snapshot edit argument '{direct}' or '{chunks}' is required"))),
    }
}

pub fn snapshot_edit_event_from_action(action: &str, args: Option<&DslValue>) -> Result<Option<SnapshotEditEvent>, Fault> {
    let json_value = || -> Result<DslValue, Fault> {
        let value = argument(args, "value")?;
        if matches!(argument(args, "valueEncoding"), Ok(DslValue::String(encoding)) if encoding == "json") {
            let DslValue::String(source) = value else { return Err(edit_fault("snapshot-edit.argument-type", "a JSON-encoded control value must be text")) };
            validate_source_keys(source).map_err(|error| edit_fault(error.code, error.to_string()))?;
            return pack::json::from_json_str(source).map_err(|error| edit_fault("snapshot-edit.value-json-invalid", error.to_string()));
        }
        Ok(value.clone())
    };
    let event = match action {
        SET_SNAPSHOT_VALUE_ACTION_ID => SnapshotEditEvent::SetValue { path: pointer_argument(args, "path", "pathChunks")?, value: json_value()? },
        INSERT_SNAPSHOT_VALUE_ACTION_ID => SnapshotEditEvent::InsertValue { path: pointer_argument(args, "path", "pathChunks")?, value: json_value()? },
        REMOVE_SNAPSHOT_VALUE_ACTION_ID => SnapshotEditEvent::RemoveValue { path: pointer_argument(args, "path", "pathChunks")? },
        MOVE_SNAPSHOT_VALUE_ACTION_ID => SnapshotEditEvent::MoveValue { from: pointer_argument(args, "from", "fromChunks")?, path: pointer_argument(args, "path", "pathChunks")? },
        RENAME_SNAPSHOT_KEY_ACTION_ID => SnapshotEditEvent::RenameKey { path: pointer_argument(args, "path", "pathChunks")?, key: text_argument(args, "value")? },
        REPLACE_SNAPSHOT_SOURCE_ACTION_ID => SnapshotEditEvent::ReplaceSource { source: text_argument(args, "value")? },
        _ => return Ok(None),
    };
    Ok(Some(event))
}

pub fn snapshot_editing_command_from_action<C>(action: &str, args: Option<&DslValue>, native: impl FnOnce(&str, Option<&DslValue>) -> Result<C, Fault>) -> Result<SnapshotEditingCommand<C>, Fault> {
    match snapshot_edit_event_from_action(action, args)? {
        Some(event) => Ok(SnapshotEditingCommand::Edit(event)),
        None => native(action, args).map(SnapshotEditingCommand::Native),
    }
}

pub fn snapshot_editing_command_id<C>(command: &SnapshotEditingCommand<C>, native: impl FnOnce(&C) -> &'static str) -> &'static str {
    match command {
        SnapshotEditingCommand::Native(command) => native(command),
        SnapshotEditingCommand::Edit(event) => event.action_id(),
    }
}

pub fn snapshot_edit_actions() -> Vec<ActionDefinition> {
    let path = || ActionArgDef::text("path", LocalizedLabel::native("Path", "Pfad")).min_length(0);
    let path_chunks = || ActionArgDef::text_list("pathChunks", LocalizedLabel::native("Path Chunks", "Pfadsegmente"));
    let value = || ActionArgDef::any("value", LocalizedLabel::native("Value", "Wert")).required();
    let encoding = || ActionArgDef::text("valueEncoding", LocalizedLabel::native("Value Encoding", "Wertkodierung"));
    let mutation = |id, en, de, args| ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args);
    vec![
        mutation(SET_SNAPSHOT_VALUE_ACTION_ID, "Set Value", "Wert setzen", vec![path(), path_chunks(), value(), encoding()]),
        mutation(INSERT_SNAPSHOT_VALUE_ACTION_ID, "Insert Value", "Wert einfügen", vec![path(), path_chunks(), value(), encoding()]),
        mutation(REMOVE_SNAPSHOT_VALUE_ACTION_ID, "Remove Value", "Wert entfernen", vec![path(), path_chunks()]),
        mutation(MOVE_SNAPSHOT_VALUE_ACTION_ID, "Move Value", "Wert verschieben", vec![ActionArgDef::text("from", LocalizedLabel::native("From", "Von")).min_length(0), ActionArgDef::text_list("fromChunks", LocalizedLabel::native("From Chunks", "Von-Segmente")), path(), path_chunks()]),
        mutation(RENAME_SNAPSHOT_KEY_ACTION_ID, "Rename Key", "Schlüssel umbenennen", vec![path(), path_chunks(), ActionArgDef::text("value", LocalizedLabel::native("Key", "Schlüssel")).min_length(0).required()]),
        mutation(REPLACE_SNAPSHOT_SOURCE_ACTION_ID, "Replace Source", "Quelltext ersetzen", vec![ActionArgDef::text("value", LocalizedLabel::native("Source", "Quelltext")).required()]),
    ]
}

/// 🧰 Adds the complete snapshot-edit action catalog to an artifact editor manifest.
pub fn snapshot_edit_actions_with(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    snapshot_edit_actions().into_iter().fold(builder, |builder, action| {
        let id = action.id.clone();
        builder.action_with(action).action_interactive_job(id, InteractiveJobClassification::Migrated)
    })
}

const SNAPSHOT_EDIT_PAYLOAD_SCHEMA: &str = "semio.stdio.snapshot-edit-command.v1";
pub const SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES: usize = 16 * 1_024 * 1_024;
pub const SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES: usize = 65_536;

pub fn snapshot_edit_source_is_admitted(source: &str) -> bool {
    <SnapshotEditEvent as OpBinary>::encode_op(&SnapshotEditEvent::ReplaceSource { source: source.to_owned() }).is_ok_and(|encoded| encoded.len() <= SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES)
}
const SNAPSHOT_EDIT_WORK_CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(1);
const SNAPSHOT_EDIT_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: SET_SNAPSHOT_VALUE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: INSERT_SNAPSHOT_VALUE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: REMOVE_SNAPSHOT_VALUE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: MOVE_SNAPSHOT_VALUE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: RENAME_SNAPSHOT_KEY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: REPLACE_SNAPSHOT_SOURCE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

pub trait SnapshotEditingEditor: ArtifactEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent>;
    fn snapshot_edit_is_admitted(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> bool;
    fn snapshot_edit_emit(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault>;
}

pub fn snapshot_edit_value_is_admitted<S: ToValue>(_event: &SnapshotEditEvent, snapshot: &S) -> bool {
    fn count(value: &DslValue, remaining: &mut usize, depth: usize) -> bool {
        if *remaining == 0 || depth > 128 {
            return false;
        }
        *remaining -= 1;
        match value {
            DslValue::Array(items) => items.iter().all(|item| count(item, remaining, depth + 1)),
            DslValue::Object(entries) => entries.iter().all(|(_, item)| count(item, remaining, depth + 1)),
            _ => true,
        }
    }
    let mut remaining = SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES;
    count(&snapshot.to_value(), &mut remaining, 0)
}

pub fn snapshot_edit_set_snapshot<S, M, C, D>(event: &SnapshotEditEvent, snapshot: &S, wrap: fn(S) -> M) -> Result<Emit<M, C, D>, Fault>
where
    S: ArtifactDsl + ToValue + FromValue,
{
    let next = apply_snapshot_edit(snapshot, event).map_err(|error| edit_fault(error.code, error.to_string()))?;
    Ok(Emit { artifact_mutations: vec![wrap(next)], description: Some("Edit document details".into()), ..Default::default() })
}

pub fn snapshot_edit_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES, 4_096, 1, SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES, 7_500)
}

pub struct SnapshotEditToolJobFactory<E: SnapshotEditingEditor> {
    keys: Vec<ToolFactoryKey>,
    marker: std::marker::PhantomData<fn() -> E>,
}

impl<E: SnapshotEditingEditor> SnapshotEditToolJobFactory<E> {
    fn new(controller_id: &str) -> Self {
        Self { keys: SNAPSHOT_EDIT_ACTION_IDS.iter().map(|tool| ToolFactoryKey::new(controller_id, *tool)).collect(), marker: std::marker::PhantomData }
    }
}

impl<E: SnapshotEditingEditor> ToolJobFactory for SnapshotEditToolJobFactory<E> {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<E>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<E>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SNAPSHOT_EDIT_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        snapshot_edit_execution_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("snapshot edit rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl<E: SnapshotEditingEditor> ArtifactOwnedToolJobFactory for SnapshotEditToolJobFactory<E> {
    type Owner = EditorApp<E>;
    const TOOL_IDS: &'static [&'static str] = SNAPSHOT_EDIT_ACTION_IDS;
    const DOCUMENT_SCHEMA: &'static str = E::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = SNAPSHOT_EDIT_PUBLICATION_CONTRACTS;
}

pub fn register_snapshot_edit_tool_factory<E: SnapshotEditingEditor>(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<E>>) -> Result<(), Fault> {
    let controller = registry.controller_id().to_string();
    registry.register(SnapshotEditToolJobFactory::<E>::new(&controller))
}

fn snapshot_edit_command_id<E: SnapshotEditingEditor>(command: &E::Command) -> &'static str {
    E::snapshot_edit_event(command).map_or("unknownSnapshotEdit", SnapshotEditEvent::action_id)
}

fn snapshot_edit_extent<E: SnapshotEditingEditor>(command: &E::Command, _snapshot: &E::Snapshot, _interaction: &kernel::InteractionState) -> Option<usize> {
    let event = E::snapshot_edit_event(command)?;
    E::snapshot_edit_is_admitted(event, _snapshot).then(|| SNAPSHOT_EDIT_WORK_CAPACITY.rows_for_items(1)).flatten()
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn snapshot_edit_reduce<E: SnapshotEditingEditor>(
    command: &E::Command,
    snapshot: &E::Snapshot,
    _config: &E::Config,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &kernel::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<E::Mutation, E::ConfigMutation, E::DraftMutation>, Fault> {
    let event = E::snapshot_edit_event(command).ok_or_else(|| edit_fault("snapshot-edit.command-mismatch", "snapshot edit factory received a native command"))?;
    apply_snapshot_edit_for_dialect(snapshot, event, E::DIALECT).map_err(|error| edit_fault(error.code, error.to_string()))?;
    E::snapshot_edit_emit(event, snapshot)
}

pub fn build_snapshot_edit_tool_job<E: SnapshotEditingEditor>(request: ArtifactOwnedToolJobRequest<EditorApp<E>>) -> Result<Option<ToolOperationSpec>, Fault> {
    if !SNAPSHOT_EDIT_ACTION_IDS.contains(&request.tool_id.as_str()) {
        return Ok(None);
    }
    let Some(event) = E::snapshot_edit_event(&request.command) else {
        return Err(edit_fault("snapshot-edit.command-mismatch", "snapshot edit tool received a native command"));
    };
    if event.action_id() != request.tool_id {
        return Err(edit_fault("snapshot-edit.tool-mismatch", "snapshot edit command does not match its addressed tool"));
    }
    let tool_id = event.action_id();
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id,
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
    };
    let payload = ArtifactRetainedCommandPayload::try_new(
        ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation,
            completion: request.completion,
        },
        snapshot_edit_command_id::<E>,
        SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES,
        SNAPSHOT_EDIT_WORK_CAPACITY.work_items(),
        Box::new(BoundedArtifactCommandWork::new(tool_id, snapshot_edit_reduce::<E>, snapshot_edit_extent::<E>)),
    )?;
    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}

pub fn snapshot_edit_bounded_first_step_proofs<E: SnapshotEditingEditor>(owner_file: &'static str, controller_id: &'static str, artifact_schema: &'static str) -> Vec<ArtifactBoundedFirstStepProof> {
    let factory_type = std::any::type_name::<SnapshotEditToolJobFactory<E>>();
    let factory = factory_type.rsplit("::").next().unwrap_or(factory_type);
    SNAPSHOT_EDIT_ACTION_IDS
        .iter()
        .map(|tool| {
            ArtifactBoundedFirstStepProof::new::<EditorApp<E>>(owner_file, controller_id, factory, tool, artifact_schema, snapshot_edit_execution_contract())
                .with_factory_type::<EditorApp<E>, SnapshotEditToolJobFactory<E>>()
        })
        .collect()
}

/// 🧩 Declares native retained-tool proofs and appends the shared snapshot-edit proofs.
#[macro_export]
macro_rules! snapshot_editing_bounded_first_step_tool_proofs {
    (
        owner: $owner:ty,
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        factory: $factory:literal,
        $(factory_type: $factory_type:ty,)?
        contract: $contract:expr,
        tools: [$($tool:literal),+ $(,)?]
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: semio_framework_plugin::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            let mut proofs = vec![$(
                qualify(semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<$owner>(
                    $owner_file,
                    $controller,
                    $factory,
                    $tool,
                    $artifact_schema,
                    $contract,
                ))
            ),+];
            proofs.extend($crate::editing::snapshot_edit_bounded_first_step_proofs::<Self>($owner_file, $controller, $artifact_schema));
            proofs
        }
    };
    (
        owner: $owner:ty,
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        factory: $factory:literal,
        $(factory_type: $factory_type:ty,)?
        tools: { $($tool:literal => $contract:expr),+ $(,)? }
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: semio_framework_plugin::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            let mut proofs = vec![$(
                qualify(semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<$owner>(
                    $owner_file,
                    $controller,
                    $factory,
                    $tool,
                    $artifact_schema,
                    $contract,
                ))
            ),+];
            proofs.extend($crate::editing::snapshot_edit_bounded_first_step_proofs::<Self>($owner_file, $controller, $artifact_schema));
            proofs
        }
    };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
