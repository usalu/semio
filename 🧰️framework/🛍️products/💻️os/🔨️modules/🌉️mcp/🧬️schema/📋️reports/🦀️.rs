//! 📋️ Reports of the foundation crate: Gateway vocabulary: errors, handles, audit lane, reports, local session files and delegated credentials.

use schemars::JsonSchema;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use serde::Deserialize;
use serde::Serialize;
use crate::errors::GatewayError;

/// 🌉️ `serde_json::Value` ↔ `DslValue` bridge, built on `🌱️value/🦀️.rs`'s own infallible
/// `From<&DslValue>`/`From<&serde_json::Value>` impls — shared by every field in this file typed
/// `serde_json::Value`.
fn json_value_to_dsl(value: &serde_json::Value) -> DslValue {
    DslValue::from(value)
}

/// 🌉️ See [`json_value_to_dsl`] — the `FromValue` direction, infallible.
fn dsl_to_json_value(value: DslValue) -> Result<serde_json::Value, ValueError> {
    Ok(serde_json::Value::from(value))
}

/// 🌉️ `Option<serde_json::Value>` ↔ `DslValue` bridge — `None` becomes `DslValue::Null`, `Some`
/// routes through [`json_value_to_dsl`].
fn optional_json_value_to_dsl(value: &Option<serde_json::Value>) -> DslValue {
    match value {
        Some(inner) => json_value_to_dsl(inner),
        None => DslValue::Null,
    }
}

/// 🌉️ See [`optional_json_value_to_dsl`] — the `FromValue` direction, infallible.
fn dsl_to_optional_json_value(value: DslValue) -> Result<Option<serde_json::Value>, ValueError> {
    match value {
        DslValue::Null => Ok(None),
        other => dsl_to_json_value(other).map(Some),
    }
}

//#region 🔖️RevisionStamp

/// 🧾️ `RevisionStamp{artifact_id, head_edit_id, cursor}` — read from `AppCommand::ReadHistory` →
/// `HistorySnapshot` (`📋️master.md` §"Observe"), the optimistic-concurrency token every
/// `action.invoke{expectedRevision}` and `InvocationReport.revisionBefore/After` carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct RevisionStamp {
    pub artifact_id: String,
    pub head_edit_id: String,
    pub cursor: String,
}

//#endregion 🔖️RevisionStamp

//#region 🔖️InvocationReport

/// 🏁️ Terminal status of one `action.invoke` — `Succeeded`/`Failed` are self-explanatory,
/// `Cancelled` covers a client-issued `job.cancel`/`notifications/cancelled` landing before
/// completion.
// 🌱️ `rename_all = "SCREAMING_SNAKE_CASE"` has no `#[value(rename_all = …)]` equivalent — spelled
// out per-variant instead, same wire names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvocationStatus {
    #[value(rename = "SUCCEEDED")]
    Succeeded,
    #[value(rename = "FAILED")]
    Failed,
    #[value(rename = "CANCELLED")]
    Cancelled,
}

/// 🧾️ `InvocationReport{invocationId, capabilityId, status, affectedResources, revisionBefore/After,
/// diffUri, warnings, undoToken, postconditions, replayed}` (`📋️master.md` §"Verify") — the result
/// re-read after `DispatchReport`/`MergeReport`, and the value an `IdempotencyStore` replay returns
/// verbatim on a repeated `idempotencyKey`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InvocationReport {
    pub invocation_id: String,
    pub capability_id: String,
    pub status: InvocationStatus,
    pub affected_resources: Vec<String>,
    pub revision_before: Option<RevisionStamp>,
    pub revision_after: Option<RevisionStamp>,
    pub diff_uri: Option<String>,
    pub warnings: Vec<String>,
    pub undo_token: Option<String>,
    pub postconditions: Vec<String>,
    pub replayed: bool,
}

//#endregion 🔖️InvocationReport

//#region 🔖️PreparedActionReport

/// 🧾️ Result of `action.prepare` — a `prep_`-prefixed handle (`📋️master.md` §"AgentSession", the
/// handle-kind prefix table) bound to `capability_id` and (when the caller supplied one)
/// `expected_revision`; `preview` is the arbitrary structured preview payload a capability's
/// `command_from_action` bridge produced, TTL-bounded by `expires_at_ms` (10 min per the frozen
/// `HandleRecord` TTL table).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PreparedActionReport {
    pub prepared_handle: String,
    pub capability_id: String,
    pub expected_revision: Option<RevisionStamp>,
    #[value(serialize_with = "json_value_to_dsl", deserialize_with = "dsl_to_json_value")]
    pub preview: serde_json::Value,
    pub expires_at_ms: u64,
}

//#endregion 🔖️PreparedActionReport

//#region 🔖️SearchHit

/// 🔎️ One `capabilities.search` result — `capability_id` is the full `<plugin_id>.<app_id>.<action_id>`
/// grammar (`📋️master.md` D3/§"Id grammar"), never a bare action id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SearchHit {
    pub capability_id: String,
    pub title: String,
    pub description: String,
    pub score: f64,
    pub plugin_id: String,
    pub app_id: String,
    /// 🎯️ `agent` on every published hit — carried on the wire so a client can SEE that the catalog
    /// is an agent projection rather than having to infer it from what is missing.
    pub audience: String,
    /// 🗿️ The artifact kind this verb acts on (`s.draw.drawing`), empty for gateway/framework verbs —
    /// the cheapest way for an agent to tell two same-titled verbs of different plugins apart.
    pub artifact_kind: String,
}

//#endregion 🔖️SearchHit

//#region 🔖️JobStatus

/// 🏃️ Lifecycle state of a `job_`-prefixed handle (long-running `action.invoke`/background work).
/// `AwaitingApproval` is a job whose work is done and whose effect waits on an explicit approval.
// 🌱️ `rename_all = "SCREAMING_SNAKE_CASE"` has no `#[value(rename_all = …)]` equivalent — spelled
// out per-variant instead, same wire names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobState {
    #[value(rename = "PENDING")]
    Pending,
    #[value(rename = "RUNNING")]
    Running,
    #[value(rename = "AWAITING_APPROVAL")]
    AwaitingApproval,
    #[value(rename = "SUCCEEDED")]
    Succeeded,
    #[value(rename = "FAILED")]
    Failed,
    #[value(rename = "CANCELLED")]
    Cancelled,
}

/// 🧾️ `job.get` response — `progress` is `0.0..=1.0` when known, `result`/`error` are populated only
/// once `state` is terminal (`Succeeded`/`Failed`/`Cancelled`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct JobStatus {
    pub job_id: String,
    pub state: JobState,
    pub progress: Option<f64>,
    #[value(serialize_with = "optional_json_value_to_dsl", deserialize_with = "dsl_to_optional_json_value")]
    pub result: Option<serde_json::Value>,
    pub error: Option<GatewayError>,
}

//#endregion 🔖️JobStatus

//#region 🔖️ContextSummary

/// 🧾️ `context.resolve` response — mints/refreshes the implicit `AgentSession` for stdio clients
/// (`📋️master.md` §"AgentSession"). `scopes` are plain capability-id strings rather than
/// `kernel::CapabilityId` — this crate has zero dependency on the kernel crate (§2.6 of this
/// packet's brief), so the newtype cannot be named here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContextSummary {
    pub session_id: String,
    pub principal: String,
    pub scopes: Vec<String>,
    pub active_artifact_id: Option<String>,
    pub catalog_hash: String,
    pub locale: String,
    /// 🔀️ Which document owner this session's artifact verbs execute against — `"shell"` when a
    /// live shell is attached over `/bridge` and declared `relayAppCommands` (the agent's edits
    /// happen in the window the human is looking at, land in that human's undo history and
    /// replicate to their collaborators), `"headless"` when this process's own `--folder`/`--hub`
    /// workspace owns them. The decision is taken HERE, once per session, and is sticky: a client
    /// that reads `"shell"` is never silently re-routed onto a different document.
    pub channel: String,
}

//#endregion 🔖️ContextSummary
