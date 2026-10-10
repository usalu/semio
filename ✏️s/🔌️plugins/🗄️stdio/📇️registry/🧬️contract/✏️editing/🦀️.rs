//! ✏️ Schema-erased, typed snapshot editing shared by every stdio artifact editor.

use crate::{kernel, value_derive};
use kernel::{ArtifactDsl, Mutation, MutationDiff, OpBinary};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ArtifactRetainedWorkCapacity, BoundedArtifactCommandWork};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use {semio_framework_plugin::AppOperationContext,semio_framework_plugin::ArtifactBoundedFirstStepProof,semio_framework_plugin::ArtifactEditor,semio_framework_plugin::ArtifactOwnedToolJobFactory,semio_framework_plugin::ArtifactOwnedToolJobRequest,semio_framework_plugin::ArtifactToolFactoryRegistry,semio_framework_plugin::ArtifactToolPublicationContract,semio_framework_plugin::ArtifactToolPublicationLane,semio_framework_artifact_reference::Dialect,semio_framework_plugin::EditorApp,semio_framework_plugin::Emit,semio_framework_plugin::InteractiveJobClassification,semio_framework_plugin::ToolExecutionContract,semio_framework_plugin::ToolFactoryKey,semio_framework_plugin::ToolJobFactory,semio_framework_plugin::ToolJobFactoryError,semio_framework_plugin::ToolOperationSpec};
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::{BTreeMap, BTreeSet};

#[path = "🧵️bytes/🦀️.rs"]
pub mod bytes;
#[path = "🪟️details/🦀️.rs"]
pub mod details;
#[path = "🩹️patch/🦀️.rs"]
pub mod patch;
#[path = "🖼️raster/🦀️.rs"]
pub mod raster;
#[path = "🧭️rules/🦀️.rs"]
pub mod rules;
pub use rules::{edited_subtree, Carried, EditPlan, EditRules, EntityRule, InsertRule, ItemField, RemoveRule, RowKey, Selector};
pub use details::{
    render_file_source_editor, render_snapshot_details, render_snapshot_details_provider_revisioned, snapshot_details_split_layout, snapshot_details_window_definition, DslSnapshotDetailsProvider,
    SnapshotDetailPathSegment, SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailsProvider, SNAPSHOT_DETAILS_BODY_KEY, SNAPSHOT_DETAILS_WINDOW_KIND_ID,
};
pub use patch::{
    apply_snapshot_patch, apply_snapshot_patch_checked, apply_snapshot_patch_for_dialect, inverse_snapshot_patch, inverse_snapshot_patches, inverse_snapshot_patches_within, prepare_snapshot_patch, snapshot_patch_from_bytes,
    snapshot_patch_from_hex, snapshot_patch_from_text, snapshot_patch_hex, snapshot_patch_input_schema, snapshot_patch_input_schema_text, snapshot_patch_label, snapshot_patch_text, snapshot_schema_location, SnapshotPatch,
    SnapshotSchemaLocation, SnapshotSchemaResolver, SNAPSHOT_PATCH_MAX_BYTES, SNAPSHOT_PATCH_MAX_INVERSE_PARTS, SNAPSHOT_PATCH_MAX_SEGMENTS,
};

pub const SET_SNAPSHOT_VALUE_ACTION_ID: &str = "setSnapshotValue";
pub const INSERT_SNAPSHOT_VALUE_ACTION_ID: &str = "insertSnapshotValue";
pub const REMOVE_SNAPSHOT_VALUE_ACTION_ID: &str = "removeSnapshotValue";
pub const MOVE_SNAPSHOT_VALUE_ACTION_ID: &str = "moveSnapshotValue";
pub const RENAME_SNAPSHOT_KEY_ACTION_ID: &str = "renameSnapshotKey";
pub const REPLACE_SNAPSHOT_SOURCE_ACTION_ID: &str = "replaceSnapshotSource";
pub const SNAPSHOT_EDIT_ACTION_IDS: &[&str] = &[SET_SNAPSHOT_VALUE_ACTION_ID, INSERT_SNAPSHOT_VALUE_ACTION_ID, REMOVE_SNAPSHOT_VALUE_ACTION_ID, MOVE_SNAPSHOT_VALUE_ACTION_ID, RENAME_SNAPSHOT_KEY_ACTION_ID, REPLACE_SNAPSHOT_SOURCE_ACTION_ID];

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

pub trait SnapshotEditingNativeCommand: OpBinary {
    const ALL_TOOL_JOB_IDS: &'static [&'static str];
}

impl<C: OpBinary> kernel::OpText for SnapshotEditingCommand<C> {
    fn print_op(&self) -> String {
        match self {
            Self::Native(command) => format!("native {}", hex_encode(&command.encode_op().unwrap_or_default())),
            Self::Edit(event) => format!("edit {}", hex_encode(&<SnapshotEditEvent as OpBinary>::encode_op(event).unwrap_or_default())),
        }
    }

    fn parse_op(line: &str) -> Result<Self, kernel::TextError> {
        let (channel, payload) = line.split_once(' ').ok_or_else(|| kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "snapshot editing command requires a channel", kernel::TextSpan::at(1, 1)))?;
        let bytes = hex_decode(payload).map_err(|error| kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, kernel::TextSpan::at(1, 1)))?;
        match channel {
            "native" => C::decode_op(&bytes).map(Self::Native).map_err(|error| kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), kernel::TextSpan::at(1, 1))),
            "edit" => <SnapshotEditEvent as OpBinary>::decode_op(&bytes).map(Self::Edit).map_err(|error| kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), kernel::TextSpan::at(1, 1))),
            _ => Err(kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown snapshot editing command channel '{channel}'"), kernel::TextSpan::at(1, 1))),
        }
    }
}

impl<C: SnapshotEditingNativeCommand> OpBinary for SnapshotEditingCommand<C> {
    const TOOL_JOB_IDS: &'static [&'static str] = C::ALL_TOOL_JOB_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> {
        let (channel, payload) = match self {
            Self::Native(command) => (0, command.encode_op()?),
            Self::Edit(event) => (1, <SnapshotEditEvent as OpBinary>::encode_op(event)?),
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
            1 => <SnapshotEditEvent as OpBinary>::decode_op(payload).map(Self::Edit),
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
        semio_framework_pack_json::to_json_string(self)
    }

    fn parse_op(line: &str) -> Result<Self, kernel::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| kernel::TextError::from_value_error(error, kernel::TextSpan::at(1, 1)))
    }
}

impl OpBinary for SnapshotEditEvent {
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
    pub span: Option<semio_framework_diagnostic::TextSpan>,
}

impl SnapshotEditError {
    /// 🧾️ A refusal with one of the module's `snapshot-edit.*` codes (`snapshot-edit.schema-invalid`, …) at the pointer `path`.
    pub fn new(code: &'static str, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code, path: path.into(), message: message.into(), span: None }
    }

    /// 📍️ Locates a source refusal at its exact one-based text span.
    pub fn with_span(mut self, span: semio_framework_diagnostic::TextSpan) -> Self {
        self.span = Some(span);
        self
    }

    /// ⚖️ The frozen mutation outcome code this refusal reports as (`📡️replication/🎮️mutation/🧫️fixtures/🧫️outcome-code`):
    /// an address the snapshot lacks is `target-missing`, a key it already holds `duplicate-id`, an edit the snapshot's
    /// current shape contradicts `target-mismatch`, and a patch malformed or out of bounds on its own `invariant`.
    pub fn outcome_code(&self) -> kernel::OutcomeCode {
        match self.code {
            "snapshot-edit.path-missing" | "snapshot-edit.index-out-of-bounds" => kernel::OutcomeCode::TargetMissing,
            "snapshot-edit.key-exists" => kernel::OutcomeCode::DuplicateId,
            "snapshot-edit.ambiguous-object"
            | "snapshot-edit.not-container"
            | "snapshot-edit.not-object"
            | "snapshot-edit.descendant-move"
            | "snapshot-edit.invalid-move"
            | "snapshot-edit.schema-invalid"
            | "snapshot-edit.lossy-conversion"
            | "snapshot-edit.constraint-invalid"
            | "snapshot-edit.splice-mismatch"
            | "snapshot-edit.char-boundary"
            | "snapshot-edit.inverse-limit" => kernel::OutcomeCode::TargetMismatch,
            _ => kernel::OutcomeCode::Invariant,
        }
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
    semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&snapshot.to_value()))
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

fn parse_snapshot_json(source: &str) -> Result<DslValue, SnapshotEditError> {
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

static SNAPSHOT_SCHEMA_VALIDATORS: std::sync::OnceLock<std::sync::RwLock<std::collections::HashMap<String, semio_framework_schema::OwnedJsonSchemaValidator>>> = std::sync::OnceLock::new();

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

fn snapshot_schema_descriptor_for_dialect(dialect: Dialect, document_schema: &str) -> Result<String, SnapshotEditError> {
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

fn validate_value(value: &DslValue, path: &str) -> Result<(), SnapshotEditError> {
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
        (DslValue::Number(semio_framework_value::Number::Float(float)), DslValue::Number(semio_framework_value::Number::UInt(integer))) | (DslValue::Number(semio_framework_value::Number::UInt(integer)), DslValue::Number(semio_framework_value::Number::Float(float))) => {
            *integer <= 9_007_199_254_740_991 && *float == *integer as f64
        }
        (DslValue::Number(semio_framework_value::Number::Float(float)), DslValue::Number(semio_framework_value::Number::Int(integer))) | (DslValue::Number(semio_framework_value::Number::Int(integer)), DslValue::Number(semio_framework_value::Number::Float(float))) => {
            integer.unsigned_abs() <= 9_007_199_254_740_991 && *float == *integer as f64
        }
        _ => left == right,
    }
}

fn edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

fn snapshot_edit_fault(error: SnapshotEditError) -> Fault {
    let mut fault = Fault::new(FaultOrigin::App, FaultCode::new(error.code), error.message);
    if !error.path.is_empty() {
        fault = fault.with_param("path", error.path);
    }
    fault.span = error.span;
    fault
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
                bytes =
                    bytes.checked_add(part.len()).filter(|bytes| *bytes <= SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES).ok_or_else(|| edit_fault("snapshot-edit.path-too-large", format!("snapshot edit argument '{chunks}' exceeds the bounded path extent")))?;
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
            return parse_snapshot_json(source).map_err(snapshot_edit_fault);
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
        mutation(
            MOVE_SNAPSHOT_VALUE_ACTION_ID,
            "Move Value",
            "Wert verschieben",
            vec![ActionArgDef::text("from", LocalizedLabel::native("From", "Von")).min_length(0), ActionArgDef::text_list("fromChunks", LocalizedLabel::native("From Chunks", "Von-Segmente")), path(), path_chunks()],
        ),
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
    /// 📦️ Declares exact borrowed operation-wire authority for retained prestage host receipts.
    fn snapshot_operation_wire_source(_mutation: &Self::Mutation) -> Option<semio_framework_plugin::plugin_app_close_prelude::store::ArtifactPreparedOperationSource<'_>> { None }

    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent>;
    fn snapshot_edit_is_admitted(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> bool {
        snapshot_edit_value_is_admitted(event, snapshot)
    }
    /// 🧭️ The pointer → kind table of this artifact's details pane: which JSON-pointer edit raises which ONE concrete kind.
    fn snapshot_edit_rules() -> &'static EditRules;

    /// 🎯️ Edits the table cannot express because the kind's payload is computed from the edit rather than carried by it (a byte
    /// splice, a text range): answered here with the concrete kind of the gesture; `None` falls through to [`Self::snapshot_edit_rules`].
    fn snapshot_edit_special(_event: &SnapshotEditEvent, _snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        Ok(None)
    }

    /// ⚖️ What one snapshot edit publishes: replacing the whole source loads the document (no history row, no differencing), every other
    /// edit resolves to the concrete kind its pointer names or is refused naming the unsupported path.
    fn snapshot_edit_emit(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        if let SnapshotEditEvent::ReplaceSource { source } = event {
            let next = snapshot_from_edit_source::<Self::Snapshot>(source).map_err(snapshot_edit_fault)?;
            return Ok(Emit { effects: vec![crate::load_example_effect(&next, Self::DOCUMENT_SCHEMA)], ..Default::default() });
        }
        let mutations = match Self::snapshot_edit_special(event, snapshot)? {
            Some(mutations) => mutations,
            None => Self::snapshot_edit_rules().resolve::<Self::Snapshot, Self::Mutation>(snapshot, event).map_err(snapshot_edit_fault)?,
        };
        check_publication_limits(snapshot, &mutations)?;
        Ok(Emit { artifact_mutations: mutations, ..Default::default() })
    }
}

/// 🧵️ Supplies native mutations to the same retained, cancelable execution lane as snapshot edits.
pub trait BoundedNativeEditingEditor: SnapshotEditingEditor {
    const NATIVE_TOOL_IDS: &'static [&'static str];
    const NATIVE_PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract];
    const NATIVE_PAYLOAD_SCHEMA: &'static str;
    const NATIVE_MAXIMUM_RAW_BYTES: usize = SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES;
    const NATIVE_MAXIMUM_WORK_ITEMS: usize = 2;
    const NATIVE_CHECKPOINT_RESUME: bool = false;

    fn native_edit_execution_contract() -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(Self::NATIVE_MAXIMUM_RAW_BYTES, 4_096, 1, Self::NATIVE_MAXIMUM_RAW_BYTES, 7_500)
    }

    fn native_edit_extent(command: &Self::Command, _snapshot: &Self::Snapshot, _interaction: &kernel::InteractionState) -> Option<usize> {
        Self::NATIVE_TOOL_IDS.contains(&Self::command_id(command)).then(|| command.encode_op().ok()).flatten().filter(|encoded| encoded.len() <= Self::NATIVE_MAXIMUM_RAW_BYTES).map(|_| Self::NATIVE_MAXIMUM_WORK_ITEMS)
    }

    fn native_edit_mutations(command: &Self::Command, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault>;

    /// 🧵️ Supplies the retained command cursor for one native edit. Editors whose reduction can
    /// traverse or copy artifact-sized values override this with an `ArtifactCommandWork` that
    /// advances one bounded semantic unit per step and owns its checkpoint/close lifecycle.
    fn native_edit_work(tool_id: &'static str) -> Box<dyn ArtifactCommandWork<EditorApp<Self>>>
    where
        Self: Sized,
    {
        Box::new(BoundedArtifactCommandWork::new(tool_id, bounded_native_edit_reduce::<Self>, Self::native_edit_extent))
    }

    /// 📬️ Supplies an exact mutation predicate and retained Store cursor for native edits whose
    /// inverse or post snapshot requires artifact-sized work. The surrounding router keeps shared
    /// snapshot mutations on their own preparation factory and never falls back after a recognized
    /// native mutation is refused.
    fn native_edit_preparation_route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<Self::Snapshot, Self::Mutation>>
    where
        Self: Sized,
    {
        None
    }
}

pub type ArtifactPreparationFactory<S, M> = std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemPreparationFactory<S, M>>;

fn retained_copy_close_demands(bytes:&Vec<u8>,reserved:bool,complete:bool)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{release_bytes:bytes.capacity(),depth:usize::from(bytes.capacity()!=0||reserved||complete),..Default::default()}}
fn retained_copy_close_step(bytes:&mut Vec<u8>,reserved:&mut bool,complete:&mut bool,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{
 use semio_framework_job::InteractiveJobCloseStep;use semio_framework_value::retained_clone::RetainedCloneProgress;let demand=retained_copy_close_demands(bytes,*reserved,*complete);
 if demand.depth==0{return InteractiveJobCloseStep::Complete{progress:Default::default()};}if grant.maximum_items==0||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return InteractiveJobCloseStep::Pending{progress:Default::default()};}
 if bytes.capacity()!=0{let original=std::mem::take(bytes);let released_bytes=original.capacity();drop(original);return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}};}
 *reserved=false;*complete=false;InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}}
}

/// 🧵️ Copies one immutable UTF-8 owner through fixed byte grants without cloning the whole value
/// in a scheduler turn.
#[derive(Default)]
pub struct RetainedTextCopy {
    bytes: Vec<u8>,
    reserved: bool,
    complete: bool,
}

impl RetainedTextCopy {
    pub fn advance(&mut self, source: &str, maximum_bytes: usize) -> Result<Option<usize>, String> {
        if maximum_bytes == 0 {
            return Ok(None);
        }
        if !self.reserved {
            self.bytes.try_reserve_exact(source.len()).map_err(|_| "retained text allocation admission failed")?;
            self.reserved = true;
            self.complete = source.is_empty();
            return Ok(Some(0));
        }
        if self.complete {
            return Ok(Some(0));
        }
        let start = self.bytes.len();
        let count = maximum_bytes.min(source.len().checked_sub(start).ok_or("retained text source changed during copy")?);
        self.bytes.extend_from_slice(&source.as_bytes()[start..start + count]);
        self.complete = self.bytes.len() == source.len();
        Ok(Some(count))
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn take(&mut self) -> Option<String> {
        if !self.complete {
            return None;
        }
        self.complete = false;
        self.reserved = false;
        String::from_utf8(std::mem::take(&mut self.bytes)).ok()
    }

    pub fn take_partial_bytes(&mut self) -> Vec<u8> {
        self.complete = false;
        self.reserved = false;
        std::mem::take(&mut self.bytes)
    }

    pub fn close_demands(&self)->semio_framework_value::RetirementDemand{retained_copy_close_demands(&self.bytes,self.reserved,self.complete)}
    pub fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{retained_copy_close_step(&mut self.bytes,&mut self.reserved,&mut self.complete,grant)}
    pub fn terminal_is_empty(&self) -> bool {
        self.bytes.is_empty() && self.bytes.capacity() == 0 && !self.reserved && !self.complete
    }
}

/// 🧵️ Copies one immutable byte owner through fixed grants and supports the same bounded
/// cancellation cleanup as [`RetainedTextCopy`].
#[derive(Default)]
pub struct RetainedBytesCopy {
    bytes: Vec<u8>,
    reserved: bool,
    complete: bool,
}

impl RetainedBytesCopy {
    pub fn advance(&mut self, source: &[u8], maximum_bytes: usize) -> Result<Option<usize>, String> {
        if maximum_bytes == 0 {
            return Ok(None);
        }
        if !self.reserved {
            self.bytes.try_reserve_exact(source.len()).map_err(|_| "retained byte allocation admission failed")?;
            self.reserved = true;
            self.complete = source.is_empty();
            return Ok(Some(0));
        }
        if self.complete {
            return Ok(Some(0));
        }
        let start = self.bytes.len();
        let count = maximum_bytes.min(source.len().checked_sub(start).ok_or("retained byte source changed during copy")?);
        self.bytes.extend_from_slice(&source[start..start + count]);
        self.complete = self.bytes.len() == source.len();
        Ok(Some(count))
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn take(&mut self) -> Option<Vec<u8>> {
        if !self.complete {
            return None;
        }
        self.complete = false;
        self.reserved = false;
        Some(std::mem::take(&mut self.bytes))
    }

    pub fn take_partial(&mut self) -> Vec<u8> {
        self.complete = false;
        self.reserved = false;
        std::mem::take(&mut self.bytes)
    }

    pub fn close_demands(&self)->semio_framework_value::RetirementDemand{retained_copy_close_demands(&self.bytes,self.reserved,self.complete)}
    pub fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{retained_copy_close_step(&mut self.bytes,&mut self.reserved,&mut self.complete,grant)}
    pub fn terminal_is_empty(&self) -> bool {
        self.bytes.is_empty() && self.bytes.capacity() == 0 && !self.reserved && !self.complete
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct NativeEditPreparationRoute<S, M> {
    recognizes: fn(&M) -> bool,
    #[factory_child]
    factory: std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemPreparationFactory<S, M>>,
}

impl<S, M> NativeEditPreparationRoute<S, M> {
    pub fn new(recognizes: fn(&M) -> bool, factory: ArtifactPreparationFactory<S, M>) -> Self {
        Self { recognizes, factory }
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RoutedNativeEditPreparationFactory<S, M> {
    #[factory_owned]
    route: NativeEditPreparationRoute<S, M>,
    #[factory_child]
    fallback: std::sync::Arc<dyn semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemPreparationFactory<S, M>>,
}

impl<S, M> semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemPreparationFactory<S, M> for RoutedNativeEditPreparationFactory<S, M>
where
    S: Send + Sync + 'static,
    M: Send + Sync + 'static,
{
    fn preflight(&self, mutation: &M, lane: semio_framework_plugin::plugin_app_close_prelude::store::HistoryLane) -> Result<semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemFootprint, String> {
        if (self.route.recognizes)(mutation) {
            self.route.factory.preflight(mutation, lane)
        } else {
            self.fallback.preflight(mutation, lane)
        }
    }

    fn begin_demand(&self,mutation:&M,lane:semio_framework_plugin::plugin_app_close_prelude::store::HistoryLane)->Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand,semio_framework_value::ValueError>{if(self.route.recognizes)(mutation){self.route.factory.begin_demand(mutation,lane)}else{self.fallback.begin_demand(mutation,lane)}}
    fn begin(
        &self,
        request: semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemPreparationRequest<S, M>,
        grant:semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemPreparation<S, M>>,semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError,semio_framework_plugin::plugin_app_close_prelude::store::ArtifactStoreOneItemPreparationRequest<S, M>)> {
        if (self.route.recognizes)(&request.mutation) {
            self.route.factory.begin(request,grant)
        } else {
            self.fallback.begin(request,grant)
        }
    }
}

/// 🌱️ Prices the concrete routed wrapper around its two actual original child factory trees.
pub fn routed_native_edit_preparation_factory_birth_bytes<S: 'static, M: 'static>(route_birth: Option<usize>, fallback_birth: usize) -> usize {
    route_birth.map_or(fallback_birth, |bytes| semio_framework_value::factory_constructor_birth_bytes::<RoutedNativeEditPreparationFactory<S, M>>(bytes.checked_add(fallback_birth).expect("routed child constructor layout")))
}

pub fn routed_native_edit_preparation_factory<S, M>(route: Option<NativeEditPreparationRoute<S, M>>, fallback: ArtifactPreparationFactory<S, M>) -> ArtifactPreparationFactory<S, M>
where
    S: Send + Sync + 'static,
    M: Send + Sync + 'static,
{
    match route {
        Some(route) => std::sync::Arc::new(RoutedNativeEditPreparationFactory { route, fallback }),
        None => fallback,
    }
}

#[macro_export]
macro_rules! bounded_native_editing_editor {
    (
        editor: $editor:ty,
        tools: [$($tool:literal),+ $(,)?],
        payload_schema: $payload_schema:literal,
        reduce: |$command:ident, $snapshot:ident| $reduce:block
        $(, work: $work:path)?
        $(, preparation_route: $preparation_route:path)?
        $(, checkpoint_resume: $checkpoint_resume:expr)?
        $(,)?) => {
        impl $crate::editing::BoundedNativeEditingEditor for $editor {
            const NATIVE_TOOL_IDS: &'static [&'static str] = &[$($tool),+];
            const NATIVE_PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
                $(semio_framework_plugin::ArtifactToolPublicationContract {
                    tool_id: $tool,
                    lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact],
                }),+
            ];
            const NATIVE_PAYLOAD_SCHEMA: &'static str = $payload_schema;
            $(const NATIVE_CHECKPOINT_RESUME: bool = $checkpoint_resume;)?

            fn native_edit_mutations(
                $command: &Self::Command,
                $snapshot: &Self::Snapshot,
            ) -> Result<
                semio_framework_plugin::Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>,
                semio_framework_plugin::Fault,
            > $reduce

            $(
                fn native_edit_work(
                    tool_id: &'static str,
                ) -> Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> {
                    $work(tool_id)
                }
            )?

            $(
                fn native_edit_preparation_route(
                    prefix: &'static str,
                ) -> Option<$crate::editing::NativeEditPreparationRoute<Self::Snapshot, Self::Mutation>> {
                    $preparation_route(prefix)
                }
            )?
        }
    };
}

pub struct BoundedNativeEditToolJobFactory<E: BoundedNativeEditingEditor> {
    keys: Vec<ToolFactoryKey>,
    marker: std::marker::PhantomData<fn() -> E>,
}

impl<E: BoundedNativeEditingEditor> BoundedNativeEditToolJobFactory<E> {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: E::NATIVE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(), marker: std::marker::PhantomData }
    }
}

impl<E: BoundedNativeEditingEditor> ToolJobFactory for BoundedNativeEditToolJobFactory<E> {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<E>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<E>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        E::NATIVE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        E::native_edit_execution_contract()
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
        if input.declared_bytes() > E::NATIVE_MAXIMUM_RAW_BYTES {
            return Err((ToolJobFactoryError::new("bounded native edit rejects oversized wire"), input, checkpoint));
        }
        match checkpoint {
            Some(checkpoint) if E::NATIVE_CHECKPOINT_RESUME => Ok(ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint)),
            Some(checkpoint) => Err((ToolJobFactoryError::new("bounded native edit does not declare checkpoint resumption"), input, Some(checkpoint))),
            None => Ok(ArtifactRetainedCommandJob::from_wire(payload, input)),
        }
    }
}

impl<E: BoundedNativeEditingEditor> ArtifactOwnedToolJobFactory for BoundedNativeEditToolJobFactory<E> {
    type Owner = EditorApp<E>;
    const TOOL_IDS: &'static [&'static str] = E::NATIVE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = E::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = E::NATIVE_PUBLICATION_CONTRACTS;
}

pub fn register_bounded_native_edit_tool_factory<E: BoundedNativeEditingEditor>(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<E>>) -> Result<(), Fault> {
    let controller = registry.controller_id().to_string();
    registry.register(BoundedNativeEditToolJobFactory::<E>::new(&controller))
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn bounded_native_edit_reduce<E: BoundedNativeEditingEditor>(
    command: &E::Command,
    snapshot: &E::Snapshot,
    _config: &E::Config,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &kernel::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<E::Mutation, E::ConfigMutation, E::DraftMutation>, Fault> {
    if !E::NATIVE_TOOL_IDS.contains(&E::command_id(command)) {
        return Err(edit_fault("bounded-native-edit.command-mismatch", "bounded native edit factory received a command for another tool"));
    }
    E::native_edit_mutations(command, snapshot)
}

pub fn admit_bounded_native_command<C: OpBinary>(command: &C, maximum_bytes: usize) -> Result<usize, Fault> {
    let encoded = command.encode_op().map_err(|error| edit_fault("bounded-native-edit.command-encoding", error.to_string()))?;
    if encoded.len() > maximum_bytes {
        return Err(edit_fault("bounded-native-edit.command-too-large", format!("bounded native edit command is {} bytes; maximum is {maximum_bytes}", encoded.len())));
    }
    Ok(encoded.len())
}

pub fn build_bounded_native_edit_tool_job<E: BoundedNativeEditingEditor>(request: ArtifactOwnedToolJobRequest<EditorApp<E>>) -> Result<Option<ToolOperationSpec>, Fault> {
    if !E::NATIVE_TOOL_IDS.contains(&request.tool_id.as_str()) {
        return Ok(None);
    }
    if E::command_id(&request.command) != request.tool_id {
        return Err(edit_fault("app.command.tool-mismatch", "bounded native edit command does not match its addressed tool"));
    }
    admit_bounded_native_command(request.command.as_ref(), E::NATIVE_MAXIMUM_RAW_BYTES)?;
    let tool_id = E::command_id(&request.command);
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id,
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        authoring_seed: request.authoring_seed.clone(),
    };
    let payload = ArtifactRetainedCommandPayload::new(
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
        E::command_id,
        E::NATIVE_MAXIMUM_RAW_BYTES,
        E::NATIVE_MAXIMUM_WORK_ITEMS,
        E::native_edit_work(tool_id),
    );
    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}

pub fn bounded_native_edit_bounded_first_step_proofs<E: BoundedNativeEditingEditor>(owner_file: &'static str, controller_id: &'static str, artifact_schema: &'static str) -> Vec<ArtifactBoundedFirstStepProof> {
    let factory_type = std::any::type_name::<BoundedNativeEditToolJobFactory<E>>();
    let factory = factory_type.rsplit("::").next().unwrap_or(factory_type);
    E::NATIVE_TOOL_IDS
        .iter()
        .map(|tool_id| ArtifactBoundedFirstStepProof::new::<EditorApp<E>>(owner_file, controller_id, factory, tool_id, artifact_schema, E::native_edit_execution_contract()).with_factory_type::<EditorApp<E>, BoundedNativeEditToolJobFactory<E>>())
        .collect()
}

pub fn snapshot_edit_value_is_admitted<S: ToValue>(event: &SnapshotEditEvent, snapshot: &S) -> bool {
    fn value_is_admitted(value: &DslValue, remaining: &mut usize, depth: usize) -> bool {
        if *remaining == 0 || depth > 128 {
            return false;
        }
        *remaining -= 1;
        match value {
            DslValue::Number(semio_framework_value::Number::Float(value)) => value.is_finite(),
            DslValue::Array(items) => items.iter().all(|item| value_is_admitted(item, remaining, depth + 1)),
            DslValue::Object(entries) => {
                let mut keys = BTreeSet::new();
                entries.iter().all(|(key, item)| keys.insert(key.as_str()) && value_is_admitted(item, remaining, depth + 1))
            }
            _ => true,
        }
    }
    fn path(path: &str) -> Option<Vec<String>> {
        if path.len() > SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES {
            return None;
        }
        let segments = decode_pointer(path).ok()?;
        (segments.len() <= 128).then_some(segments)
    }
    fn shape<S: ToValue>(snapshot: &S, path: &[String]) -> Option<semio_framework_value::ValueShape> {
        let segments = path.iter().map(String::as_str).collect::<Vec<_>>();
        snapshot.value_shape_at_path(&segments).ok()
    }
    fn insertion_parent_is_admitted<S: ToValue>(snapshot: &S, path: &[String], pointer: &str) -> bool {
        let Some((last, parent)) = path.split_last() else { return false };
        match shape(snapshot, parent) {
            Some(semio_framework_value::ValueShape::Array { len }) => array_index(last, len, pointer, true).is_ok(),
            Some(semio_framework_value::ValueShape::Object { .. }) => true,
            _ => false,
        }
    }
    if <SnapshotEditEvent as OpBinary>::encode_op(event).map_or(true, |bytes| bytes.len() > SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES) {
        return false;
    }
    let mut remaining = SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES;
    match event {
        SnapshotEditEvent::SetValue { path: pointer, value } => {
            let Some(path) = path(pointer) else { return false };
            value_is_admitted(value, &mut remaining, 0) && (path.is_empty() || shape(snapshot, &path).is_some())
        }
        SnapshotEditEvent::InsertValue { path: pointer, value } => {
            let Some(path) = path(pointer) else { return false };
            value_is_admitted(value, &mut remaining, 0) && insertion_parent_is_admitted(snapshot, &path, pointer)
        }
        SnapshotEditEvent::RemoveValue { path: pointer } => {
            let Some(path) = path(pointer) else { return false };
            !path.is_empty() && shape(snapshot, &path).is_some()
        }
        SnapshotEditEvent::MoveValue { from, path: pointer } => {
            let (Some(from_path), Some(path)) = (path(from), path(pointer)) else { return false };
            !from_path.is_empty() && !path.is_empty() && !(path.len() > from_path.len() && path.starts_with(&from_path)) && shape(snapshot, &from_path).is_some()
        }
        SnapshotEditEvent::RenameKey { path: pointer, key } => {
            if key.len() > SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES {
                return false;
            }
            let Some(path) = path(pointer) else { return false };
            let Some((_, parent)) = path.split_last() else { return false };
            matches!(shape(snapshot, parent), Some(semio_framework_value::ValueShape::Object { .. })) && shape(snapshot, &path).is_some()
        }
        SnapshotEditEvent::ReplaceSource { source } => source.len() <= SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES,
    }
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

/// 📏️ Refuses a resolved edit whose mutation or exact undo cannot be carried as one native publication item.
fn check_publication_limits<S, M: Mutation<S> + OpBinary>(snapshot: &S, mutations: &[M]) -> Result<(), Fault> {
    mutations.iter().try_for_each(|mutation| {
        let inverses = mutation.inverse(snapshot).map_err(|error| edit_fault("snapshot-edit.inverse-refused", error.into_message()))?;
        std::iter::once(mutation).chain(inverses.iter()).try_for_each(|operation| {
            let bytes = operation.encode_op().map_err(|error| edit_fault("snapshot-edit.publication-codec", error.to_string()))?;
            (bytes.len() <= kernel::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).then_some(()).ok_or_else(|| edit_fault("snapshot-edit.publication-limit", "the edit or its exact undo exceeds the native publication item limit"))
        })
    })
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
        return Err(edit_fault("app.command.tool-mismatch", "snapshot edit command does not match its addressed tool"));
    }
    let tool_id = event.action_id();
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id,
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        authoring_seed: request.authoring_seed.clone(),
    };
    let payload = ArtifactRetainedCommandPayload::new(
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
    );
    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}

pub fn snapshot_edit_bounded_first_step_proofs<E: SnapshotEditingEditor>(owner_file: &'static str, controller_id: &'static str, artifact_schema: &'static str) -> Vec<ArtifactBoundedFirstStepProof> {
    let factory_type = std::any::type_name::<SnapshotEditToolJobFactory<E>>();
    let factory = factory_type.rsplit("::").next().unwrap_or(factory_type);
    SNAPSHOT_EDIT_ACTION_IDS
        .iter()
        .map(|tool| ArtifactBoundedFirstStepProof::new::<EditorApp<E>>(owner_file, controller_id, factory, tool, artifact_schema, snapshot_edit_execution_contract()).with_factory_type::<EditorApp<E>, SnapshotEditToolJobFactory<E>>())
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
