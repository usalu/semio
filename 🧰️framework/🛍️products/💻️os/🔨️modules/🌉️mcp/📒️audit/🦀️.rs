//! 📒️ Agent audit lane (`📌️sol-P1b-packet.md` §2.3, `📋️master.md` §3.4 verbatim field list) —
//! append-only [`AgentAuditEvent`] rows behind [`AuditSink`], the seam a later packet's real
//! event-sourced `os.agent.audit` lane (via `ArtifactHost`) implements instead of [`FileAuditSink`].
//! **Secrets and full sensitive args never reach a sink**: [`redact_input`] produces `input_redacted`
//! from the raw call arguments BEFORE anything is constructed, and `mod quick`'s
//! `sensitive_field_never_reaches_the_sink` test proves it end-to-end through a real [`AuditSink`].

use crate::errors::{GatewayError, GatewayErrorCode};
use crate::schema::RevisionStamp;
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

//#region 🔖️ClientInfo
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}
//#endregion 🔖️ClientInfo

//#region 🔖️AuditDecision
/// ⚖️ `Allowed|Denied{code}|Approved{by,mode}` — `📋️master.md` §3.4's frozen decision shape.
// 🌱️ `rename_all = "SCREAMING_SNAKE_CASE"` has no `#[value(rename_all = …)]` equivalent — spelled
// out per-variant instead, same wire names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
#[value(tag = "kind")]
pub enum AuditDecision {
    #[value(rename = "ALLOWED")]
    Allowed,
    #[value(rename = "DENIED")]
    Denied { code: GatewayErrorCode },
    #[value(rename = "APPROVED")]
    Approved { by: String, mode: String },
}
//#endregion 🔖️AuditDecision

//#region 🔖️AgentAuditEvent
/// 🌉️ `serde_json::Value` ↔ `DslValue` bridge for `AgentAuditEvent::input_redacted`, built on
/// `🌱️value/🦀️.rs`'s own infallible `From<&DslValue>`/`From<&serde_json::Value>` impls.
fn json_value_to_dsl(value: &serde_json::Value) -> DslValue {
    DslValue::from(value)
}

/// 🌉️ See [`json_value_to_dsl`] — the `FromValue` direction, infallible.
fn dsl_to_json_value(value: DslValue) -> Result<serde_json::Value, ValueError> {
    Ok(serde_json::Value::from(value))
}

/// 🧾️ The exact field list from `📋️master.md` §3.4 — `input_hash` is blake3 over the RAW (unredacted)
/// call arguments (so two identical calls hash identically for correlation) while `input_redacted` is
/// the only projection of the arguments that is ever written to a sink.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AgentAuditEvent {
    pub invocation_id: String,
    pub ts_ms: u64,
    pub principal: String,
    pub session: String,
    pub capability: String,
    pub input_hash: String,
    #[value(serialize_with = "json_value_to_dsl", deserialize_with = "dsl_to_json_value")]
    pub input_redacted: serde_json::Value,
    pub decision: AuditDecision,
    pub preview_hash: Option<String>,
    pub txn_id: Option<String>,
    pub edit_ids: Vec<String>,
    pub revision_before: Option<RevisionStamp>,
    pub revision_after: Option<RevisionStamp>,
    pub outcome: String,
    pub error: Option<GatewayError>,
    pub duration_ms: u64,
    pub undo_token: Option<String>,
    pub client: ClientInfo,
}
//#endregion 🔖️AgentAuditEvent

//#region 🔖️Redaction
/// 🙈️ Case-insensitive key names a real capability's raw arguments must never leak past — a later
/// packet's per-capability policy can widen this per call site; this is the crate-wide floor.
pub const SENSITIVE_KEYS: &[&str] = &["password", "token", "secret", "apikey", "api_key", "authorization", "bearer", "credential", "credentials"];

/// #️⃣️ blake3 of the raw (unredacted) JSON args — stable across argument key reordering because
/// `serde_json::Value`'s own `Display`/`to_string` serializes object keys in insertion order, so
/// callers that always build args the same way get a stable hash; this is a correlation id, not a
/// content-addressed guarantee.
pub fn hash_input(input: &serde_json::Value) -> String {
    framework_hash::hash_bytes(input.to_string().as_bytes())
}

/// 🧼️ Recursively replaces the VALUE of any object key matching `sensitive_keys`
/// (case-insensitively) with a fixed placeholder — arrays/objects are walked, everything else is
/// cloned through unchanged.
pub fn redact_input(input: &serde_json::Value, sensitive_keys: &[&str]) -> serde_json::Value {
    match input {
        serde_json::Value::Object(map) => {
            let mut redacted = serde_json::Map::new();
            for (key, value) in map {
                let is_sensitive = sensitive_keys.iter().any(|sensitive| sensitive.eq_ignore_ascii_case(key));
                redacted.insert(key.clone(), if is_sensitive { serde_json::Value::String("«redacted»".to_string()) } else { redact_input(value, sensitive_keys) });
            }
            serde_json::Value::Object(redacted)
        }
        serde_json::Value::Array(items) => serde_json::Value::Array(items.iter().map(|item| redact_input(item, sensitive_keys)).collect()),
        other => other.clone(),
    }
}
//#endregion 🔖️Redaction

//#region 🔖️AuditSink
/// 🖇️ Append-only writer seam — the event-sourced OS lane (`os.agent.audit` via `ArtifactHost`) a
/// later packet builds implements this trait instead of [`FileAuditSink`]; nothing upstream of the
/// trait changes when that lands.
// 🔀️ dedyn-fw-os-misc, O1/R11: closed 2-implementor set (`InMemoryAuditSink`, `FileAuditSink`,
// both below) — `#[dyn_enum]` + `dyn_enum_close!` (`AuditSinks`, right after both impls) close it
// into an enum instead of `Arc<dyn AuditSink>`.
#[dyn_enum]
pub trait AuditSink: Send + Sync {
    fn append(&self, event: &AgentAuditEvent) -> Result<(), GatewayError>;
}

/// 🧪️ Test-only sink — records every appended event in order, nothing more.
#[derive(Default)]
pub struct InMemoryAuditSink {
    events: Mutex<Vec<AgentAuditEvent>>,
}

impl InMemoryAuditSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn events(&self) -> Vec<AgentAuditEvent> {
        self.events.lock().expect("audit sink lock poisoned").clone()
    }
}

impl AuditSink for InMemoryAuditSink {
    fn append(&self, event: &AgentAuditEvent) -> Result<(), GatewayError> {
        self.events.lock().expect("audit sink lock poisoned").push(event.clone());
        Ok(())
    }
}

/// 📁️ JSON-lines file sink under [`default_audit_dir`] (overridable — `semio-os-mcp`'s CLI flags
/// pass an explicit path) — one file, append-only, one `AgentAuditEvent` per line.
pub struct FileAuditSink {
    path: PathBuf,
    write_lock: Mutex<()>,
}

impl FileAuditSink {
    /// 🏗️ Creates the parent directory (and its parents) if missing, so a misconfigured `--audit-dir`
    /// fails fast at startup rather than on the first audit write.
    pub fn new(directory: impl Into<PathBuf>) -> Result<Self, GatewayError> {
        let directory = directory.into();
        fs::create_dir_all(&directory).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot create audit directory `{}`: {error}", directory.display())))?;
        Ok(Self { path: directory.join(AUDIT_FILE_NAME), write_lock: Mutex::new(()) })
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl AuditSink for FileAuditSink {
    fn append(&self, event: &AgentAuditEvent) -> Result<(), GatewayError> {
        let _guard = self.write_lock.lock().expect("audit file lock poisoned");
        let line = serde_json::to_string(event).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        let mut file = OpenOptions::new().create(true).append(true).open(&self.path).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot open audit file `{}`: {error}", self.path.display())))?;
        writeln!(file, "{line}").map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))
    }
}

// 🔀️ dedyn-fw-os-misc, O1/R11: closes `AuditSink`'s implementor set. Replaces `Arc<dyn AuditSink>`.
dyn_enum_close! {
    pub enum AuditSinks: AuditSink {
        InMemory(InMemoryAuditSink),
        File(FileAuditSink),
    }
}

pub const AUDIT_FILE_NAME: &str = "agent-audit.jsonl";

/// 🏠️ `~/.semio/agent/audit` (`📋️master.md` §3.4) — cross-platform `HOME`/`USERPROFILE` lookup, no
/// new dependency (`dirs`/etc) for a single-purpose path join.
pub fn default_audit_dir() -> PathBuf {
    home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".semio").join("agent").join("audit")
}

fn home_dir() -> Option<PathBuf> {
    for variable in ["HOME", "USERPROFILE"] {
        if let Ok(value) = std::env::var(variable) {
            if !value.is_empty() {
                return Some(PathBuf::from(value));
            }
        }
    }
    None
}
//#endregion 🔖️AuditSink

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
