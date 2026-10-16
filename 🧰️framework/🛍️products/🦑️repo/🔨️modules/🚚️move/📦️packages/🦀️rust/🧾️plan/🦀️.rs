//! 🧾️ Plan: the change set an operation returns.

use std::collections::BTreeMap;

//#region 🧾️Plan

/// ✏️ One edit of a plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    /// 📝️ Replaces or creates the file at `path`.
    Write { path: String, content: String },
    /// 🚚️ Moves `from` to `to`; a directory moves with everything under it.
    Move { from: String, to: String },
    /// 🗑️ Removes `path` and, for a directory, everything under it.
    Delete { path: String },
    /// 📁️ Creates the directory at `path` and every directory leading to it.
    CreateDirectory { path: String },
}

/// 📣️ One line the Go twin writes to its command output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// ℹ️ Progress detail.
    Info(String),
    /// ✅️ The closing line of a successful operation.
    Success(String),
}

impl Message {
    /// 📝️ The line text without its level.
    pub fn text(&self) -> &str {
        match self {
            Message::Info(text) | Message::Success(text) => text.as_str(),
        }
    }
}

/// 📡️ One event the Go twin emits around an operation.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanEvent {
    /// 🏷️ The event kind, such as `file.move.starting`.
    pub kind: String,
    /// 🎁️ The payload, already encoded.
    pub payload: serde_json::Value,
}

impl PlanEvent {
    pub(super) fn new<T: serde::Serialize>(kind: &str, payload: &T) -> Self {
        Self { kind: kind.to_string(), payload: serde_json::to_value(payload).unwrap_or(serde_json::Value::Null) }
    }
}

/// 🧾️ Everything an operation decided: what to change, what to say, what to emit, what to count.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Plan {
    /// ✏️ The edits, in application order.
    pub changes: Vec<Change>,
    /// 📣️ The command output lines.
    pub messages: Vec<Message>,
    /// 📡️ The events, starting event first.
    pub events: Vec<PlanEvent>,
    /// 🔢️ The numeric result the Go twin returns as tool data.
    pub stats: BTreeMap<String, i64>,
}

impl Plan {
    pub(super) fn empty() -> Self {
        Self::default()
    }

    /// 📝️ Every message text in order.
    pub fn lines(&self) -> Vec<String> {
        self.messages.iter().map(|m| m.text().to_string()).collect()
    }
}

/// 🚫️ Why an operation refused to produce a plan. The message is the Go twin's, verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveError(pub String);

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MoveError {}

pub(super) fn fail<T>(message: impl Into<String>) -> Result<T, MoveError> {
    Err(MoveError(message.into()))
}

//#endregion 🧾️Plan
