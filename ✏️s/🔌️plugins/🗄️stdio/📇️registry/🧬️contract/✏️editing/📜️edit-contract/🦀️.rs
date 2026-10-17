//! 📜️ The snapshot edit contract: action ids, events, commands, their text and binary wire and the edit error.

use super::*;

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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
pub enum SnapshotEditEvent {
    SetValue { path: String, value: DslValue },
    InsertValue { path: String, value: DslValue },
    RemoveValue { path: String },
    MoveValue { from: String, path: String },
    RenameKey { path: String, key: String },
    ReplaceSource { source: String },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
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
