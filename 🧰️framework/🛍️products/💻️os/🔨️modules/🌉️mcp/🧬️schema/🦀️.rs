//! 🧬️ The `os.mcp` schema registry — the ONE place this crate declares a schema.
//!
//! It owns three families, all published through [`schemas`]: the gateway wire types
//! (`📋️master.md` §"MCP tool names"/§"Observe"/§"Verify"/§"Idempotency", `schemars`-derived from the
//! structs below and validated at kernel/backend boundaries the same way
//! `🧰️framework/🔨️modules/🧬️schema` validates artifact snapshots); the MCP protocol types, mirrored
//! from `🧭️protocol/🦀️.rs`; and every tool `inputSchema`/`outputSchema` shape the facets stamp a
//! `semio://capability/{id}/{input|output}` `$id` onto ([`🔖️ToolSchemas`]). No facet under `🌉️mcp/`
//! writes a schema literal of its own.
//!
//! [`schema_mirror_document`] projects the whole registry into the committed draft-07 document
//! `🧬️schema/🔣️.json`, from which `🧬️schema/🟦️.ts` is generated
//! (`bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`, whose emitter is `semio-os-mcp
//! schemas`). `GatewayError` is re-exported from `crate::errors` rather than redefined here — one
//! type, one owning facet.

use schemars::{schema_for, JsonSchema};
use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};
use serde::{Deserialize, Serialize};

pub use crate::errors::{GatewayError, GatewayErrorCode};

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
}
//#endregion 🔖️SearchHit

//#region 🔖️JobStatus
/// 🏃️ Lifecycle state of a `job_`-prefixed handle (long-running `action.invoke`/background work).
// 🌱️ `rename_all = "SCREAMING_SNAKE_CASE"` has no `#[value(rename_all = …)]` equivalent — spelled
// out per-variant instead, same wire names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobState {
    #[value(rename = "PENDING")]
    Pending,
    #[value(rename = "RUNNING")]
    Running,
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
}
//#endregion 🔖️ContextSummary

//#region 🔖️McpSchemaShape
/// 🧷️ Rewrites every boolean sub-schema into its object equivalent, in place and recursively.
///
/// JSON Schema 2020-12 allows `true`/`false` as complete schemas, and `schemars` emits `true` for a
/// free-form `serde_json::Value` field. The MCP SDKs, however, validate tool `inputSchema`/
/// `outputSchema` with a Zod model that requires every sub-schema to be an **object** — a bare
/// `true` makes the official client reject the whole `tools/list` response with a `$ZodError`
/// (observed against `@modelcontextprotocol/sdk` 1.30.0 on `action_prepare.outputSchema.properties
/// .preview`). `{}` and `{"not": {}}` are the semantically identical object forms, so this narrows
/// the encoding without changing what any schema accepts.
pub fn normalize_boolean_subschemas(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Bool(true) => *value = serde_json::json!({}),
        serde_json::Value::Bool(false) => *value = serde_json::json!({ "not": {} }),
        serde_json::Value::Object(map) => {
            for (key, entry) in map.iter_mut() {
                if SCHEMA_BOOLEAN_KEYWORDS.contains(&key.as_str()) {
                    continue;
                }
                normalize_boolean_subschemas(entry);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                normalize_boolean_subschemas(item);
            }
        }
        _ => {}
    }
}

/// 🔑️ Keywords whose value is a genuine boolean **flag**, not a sub-schema — rewriting these would
/// corrupt the schema rather than normalize it.
const SCHEMA_BOOLEAN_KEYWORDS: &[&str] = &["additionalProperties", "unevaluatedProperties", "additionalItems", "unevaluatedItems", "readOnly", "writeOnly", "deprecated", "uniqueItems", "exclusiveMinimum", "exclusiveMaximum"];

/// 🔁️ Converts a `schemars` 0.8 draft-07 document into JSON Schema 2020-12 in place.
///
/// MCP requires tool `inputSchema`/`outputSchema` to be 2020-12, but `schemars` 0.8 (the version
/// this workspace pins) emits draft-07. The two differ, for the struct shapes this crate mirrors,
/// only in where subschema definitions live and how they are referenced — so this is a real
/// conversion of those two things, not a relabelling of the `$schema` URI:
/// `definitions` → `$defs`, and every `#/definitions/X` reference → `#/$defs/X`.
/// Anything already declaring 2020-12 is left untouched.
pub fn convert_draft07_to_2020_12(value: &mut serde_json::Value) {
    const DRAFT_07: &str = "http://json-schema.org/draft-07/schema#";
    const DIALECT_2020_12: &str = "https://json-schema.org/draft/2020-12/schema";
    let is_draft_07 = value.get("$schema").and_then(serde_json::Value::as_str) == Some(DRAFT_07);
    if !is_draft_07 {
        return;
    }
    if let Some(map) = value.as_object_mut() {
        map.insert("$schema".to_string(), serde_json::Value::String(DIALECT_2020_12.to_string()));
        if let Some(definitions) = map.remove("definitions") {
            map.insert("$defs".to_string(), definitions);
        }
    }
    rewrite_definition_refs(value);
}

/// 🔗️ Repoints every `$ref` from draft-07's `#/definitions/` to 2020-12's `#/$defs/`.
fn rewrite_definition_refs(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(reference)) = map.get_mut("$ref") {
                if let Some(name) = reference.strip_prefix("#/definitions/") {
                    *reference = format!("#/$defs/{name}");
                }
            }
            for entry in map.values_mut() {
                rewrite_definition_refs(entry);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                rewrite_definition_refs(item);
            }
        }
        _ => {}
    }
}
//#endregion 🔖️McpSchemaShape

//#region 🔖️ToolSchemas
/// 🌐️ The dialect every MCP `inputSchema`/`outputSchema` must declare — both SDKs validate
/// `tools/list` against 2020-12, so this is the ONE place in the crate that writes that URI.
pub const MCP_SCHEMA_DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

/// 🏷️ Stamps the MCP dialect plus the `semio://capability/{id}/{input|output}` `$id` onto one
/// catalogued shape. The shapes below stay `$id`-free so [`schemas`] can publish them verbatim as
/// `$defs` entries of `🧬️schema/🔣️.json`; only the wire copy a `Tool`/`CapabilityDefinition`
/// carries is stamped.
fn wire(capability_id: &str, direction: &str, mut shape: serde_json::Value) -> serde_json::Value {
    if let Some(map) = shape.as_object_mut() {
        map.insert("$schema".to_string(), serde_json::Value::String(MCP_SCHEMA_DIALECT.to_string()));
        map.insert("$id".to_string(), serde_json::Value::String(format!("semio://capability/{capability_id}/{direction}")));
    }
    shape
}

/// 📐️ `RevisionStamp`-shaped but nullable — a sub-schema of `🗿️artifact`'s open/create/snapshot
/// envelopes, never a top-level tool schema, so it carries neither `$schema` nor `$id`.
pub fn nullable_revision_stamp_shape() -> serde_json::Value {
    serde_json::json!({
        "type": ["object", "null"],
        "properties": { "artifactId": { "type": "string" }, "headEditId": { "type": "string" }, "cursor": { "type": "string" } },
    })
}

//#region 🔖️CoreToolSchemas
pub fn capabilities_search_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "query": { "type": "string" },
            "kind": { "type": "array", "items": { "type": "string" } },
            "owner": { "type": "string" },
            "artifactKind": { "type": "string" },
            "requiresScope": { "type": "string" },
        },
        "required": ["query"],
        "additionalProperties": false,
    })
}

pub fn capabilities_search_input_schema() -> serde_json::Value {
    wire("capabilities.search", "input", capabilities_search_input_shape())
}

/// 📐️ MCP's `tools/list` requires an `outputSchema` (when present) to describe a JSON OBJECT at the
/// top level — a bare `type:"array"` fails the SDK client's own Zod validation of the `Tool` shape
/// (caught live running `bun nx run @semio-tech/framework-os-mcp:test-quick`, not read off the spec).
/// The hits themselves are therefore wrapped under a `results` property.
pub fn capabilities_search_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "results": { "type": "array" } } })
}

pub fn capabilities_search_output_schema() -> serde_json::Value {
    wire("capabilities.search", "output", capabilities_search_output_shape())
}

pub fn capabilities_describe_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "capabilityId": { "type": "string" } }, "required": ["capabilityId"], "additionalProperties": false })
}

pub fn capabilities_describe_input_schema() -> serde_json::Value {
    wire("capabilities.describe", "input", capabilities_describe_input_shape())
}

pub fn capabilities_describe_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn capabilities_describe_output_schema() -> serde_json::Value {
    wire("capabilities.describe", "output", capabilities_describe_output_shape())
}

pub fn context_resolve_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "principal": { "type": "string" }, "locale": { "type": "string" } }, "additionalProperties": false })
}

pub fn context_resolve_input_schema() -> serde_json::Value {
    wire("context.resolve", "input", context_resolve_input_shape())
}

pub fn context_resolve_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn context_resolve_output_schema() -> serde_json::Value {
    wire("context.resolve", "output", context_resolve_output_shape())
}
//#endregion 🔖️CoreToolSchemas

//#region 🔖️MutationToolSchemas
pub fn action_prepare_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "capabilityId": { "type": "string" }, "input": { "type": "object" } }, "required": ["capabilityId"], "additionalProperties": false })
}

pub fn action_prepare_input_schema() -> serde_json::Value {
    wire("action.prepare", "input", action_prepare_input_shape())
}

pub fn action_invoke_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "preparedActionHandle": { "type": "string" },
            "capabilityId": { "type": "string" },
            "input": { "type": "object" },
            "expectedRevision": { "type": "object" },
            "idempotencyKey": { "type": "string" },
            "approvalHandle": { "type": "string" },
        },
        "additionalProperties": false,
    })
}

pub fn action_invoke_input_schema() -> serde_json::Value {
    wire("action.invoke", "input", action_invoke_input_shape())
}

/// 🎫️ Every handle field the five one-handle tools draw from — `action.cancel`'s
/// `preparedActionHandle`, `transaction.commit`/`transaction.rollback`'s `transactionHandle`, and
/// `history.undo`/`history.redo`'s `undoToken`.
pub const HANDLE_INPUT_FIELDS: &[&str] = &["preparedActionHandle", "transactionHandle", "undoToken"];

/// 🎫️ The whole one-handle tool family as ONE contract: a call carries exactly one of
/// [`HANDLE_INPUT_FIELDS`] and nothing else. Each per-tool document [`handle_input_schema`] stamps is
/// literally one branch of this `oneOf`, so the family and its members cannot drift apart.
pub fn handle_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "oneOf": HANDLE_INPUT_FIELDS.iter().map(|field| handle_input_branch(field)).collect::<Vec<_>>() })
}

/// 🎫️ One closed single-field member of the [`handle_input_shape`] family.
fn handle_input_branch(field: &str) -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { field: { "type": "string" } }, "required": [field], "additionalProperties": false })
}

/// 🎫️ The wire document for the one tool that accepts `field`.
pub fn handle_input_schema(field: &str, capability_id: &str) -> serde_json::Value {
    assert!(HANDLE_INPUT_FIELDS.contains(&field), "`{field}` is not a declared handle field");
    wire(capability_id, "input", handle_input_branch(field))
}

pub fn transaction_begin_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "preparedHandles": { "type": "array", "items": { "type": "string" } } }, "required": ["preparedHandles"], "additionalProperties": false })
}

pub fn transaction_begin_input_schema() -> serde_json::Value {
    wire("transaction.begin", "input", transaction_begin_input_shape())
}
//#endregion 🔖️MutationToolSchemas

//#region 🔖️ArtifactToolSchemas
pub fn artifact_open_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" } }, "required": ["artifactId"], "additionalProperties": false })
}

pub fn artifact_open_input_schema() -> serde_json::Value {
    wire("artifact.open", "input", artifact_open_input_shape())
}

/// 📐️ Deliberately NOT the resource's full-body shape (`semio://artifact/{id}` — packBytes/sprBytes/
/// packBase64): `artifact_open` answers identity/kind/revision/size, never the whole body.
pub fn artifact_open_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "kind": { "type": ["string", "null"] }, "revision": nullable_revision_stamp_shape(), "sizeBytes": { "type": ["integer", "null"] } },
    })
}

pub fn artifact_open_output_schema() -> serde_json::Value {
    wire("artifact.open", "output", artifact_open_output_shape())
}

/// 📐️ `artifact_create`'s DIRECT-tool input (`🗿️artifact`), distinct from
/// [`artifact_create_template_input_shape`] — the catalog-compiled `artifact.create` capability that
/// enumerates every plugin's declared example templates.
pub fn artifact_create_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" }, "kind": { "type": "string" }, "initial": {} }, "required": ["artifactId", "kind"], "additionalProperties": false })
}

pub fn artifact_create_input_schema() -> serde_json::Value {
    wire("artifact.create", "input", artifact_create_input_shape())
}

pub fn artifact_create_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" }, "kind": { "type": "string" }, "revision": nullable_revision_stamp_shape() } })
}

pub fn artifact_create_output_schema() -> serde_json::Value {
    wire("artifact.create", "output", artifact_create_output_shape())
}

pub fn artifact_validate_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" } }, "required": ["artifactId"], "additionalProperties": false })
}

pub fn artifact_validate_input_schema() -> serde_json::Value {
    wire("artifact.validate", "input", artifact_validate_input_shape())
}

/// 📐️ Deliberately permissive: the real wire protocol has no validate query command yet
/// (`🏠️workspace/🦀️.rs` `read_artifact_resource`'s `validation` arm), so no shape can be pinned
/// down before that lands.
pub fn artifact_validate_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn artifact_validate_output_schema() -> serde_json::Value {
    wire("artifact.validate", "output", artifact_validate_output_shape())
}

pub fn artifact_snapshot_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" }, "revision": nullable_revision_stamp_shape() }, "required": ["artifactId"], "additionalProperties": false })
}

pub fn artifact_snapshot_input_schema() -> serde_json::Value {
    wire("artifact.snapshot", "input", artifact_snapshot_input_shape())
}

pub fn artifact_snapshot_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "packBytes": { "type": ["integer", "null"] }, "sprBytes": { "type": ["integer", "null"] }, "packBase64": { "type": ["string", "null"] } },
    })
}

pub fn artifact_snapshot_output_schema() -> serde_json::Value {
    wire("artifact.snapshot", "output", artifact_snapshot_output_shape())
}

pub fn artifact_export_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" }, "format": { "type": "string" } }, "required": ["artifactId"], "additionalProperties": false })
}

pub fn artifact_export_input_schema() -> serde_json::Value {
    wire("artifact.export", "input", artifact_export_input_shape())
}

/// 📐️ The shape a real export would answer with once the wire protocol grows an export command —
/// today every call ends in a tool-error carrying `availableFormats` in its `details` instead.
pub fn artifact_export_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "format": { "type": "string" }, "contentBase64": { "type": ["string", "null"] }, "mimeType": { "type": ["string", "null"] } },
    })
}

pub fn artifact_export_output_schema() -> serde_json::Value {
    wire("artifact.export", "output", artifact_export_output_shape())
}
//#endregion 🔖️ArtifactToolSchemas

//#region 🔖️UiToolSchemas
pub fn ui_focus_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "windowId": { "type": "string" } }, "additionalProperties": false })
}

pub fn ui_focus_input_schema() -> serde_json::Value {
    wire("ui.focus", "input", ui_focus_input_shape())
}

pub fn ui_focus_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "ok": { "type": "boolean" }, "windowId": {} } })
}

pub fn ui_focus_output_schema() -> serde_json::Value {
    wire("ui.focus", "output", ui_focus_output_shape())
}

pub fn ui_reveal_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "anchor": { "type": "string", "enum": ["left", "right", "top", "bottom"] }, "path": { "type": "array", "items": { "type": "string" } } },
        "required": ["anchor", "path"],
        "additionalProperties": false,
    })
}

pub fn ui_reveal_input_schema() -> serde_json::Value {
    wire("ui.reveal", "input", ui_reveal_input_shape())
}

pub fn ui_reveal_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "ok": { "type": "boolean" }, "anchor": { "type": "string" }, "path": { "type": "array", "items": { "type": "string" } } } })
}

pub fn ui_reveal_output_schema() -> serde_json::Value {
    wire("ui.reveal", "output", ui_reveal_output_shape())
}

pub fn job_get_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "jobId": { "type": "string" } }, "required": ["jobId"], "additionalProperties": false })
}

pub fn job_get_input_schema() -> serde_json::Value {
    wire("job.get", "input", job_get_input_shape())
}

pub fn job_cancel_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "jobId": { "type": "string" } }, "required": ["jobId"], "additionalProperties": false })
}

pub fn job_cancel_input_schema() -> serde_json::Value {
    wire("job.cancel", "input", job_cancel_input_shape())
}

/// 📐️ `job.get` and `job.cancel` answer the SAME snapshot — one shape, stamped with whichever
/// capability id asked for it.
pub fn job_snapshot_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "jobId": { "type": "string" },
            "kind": { "type": "string" },
            "status": { "type": "string" },
            "progress": {},
            "message": {},
            "result": {},
            "error": {},
            "cancelRequested": { "type": "boolean" },
        },
    })
}

pub fn job_snapshot_output_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "output", job_snapshot_output_shape())
}
//#endregion 🔖️UiToolSchemas

//#region 🔖️CatalogToolSchemas
/// 📐️ `📋️master.md` §3.2 step 1's input envelope — `{type:"object", properties, required,
/// additionalProperties:false}`. The catalogued shape is the zero-argument member of the family
/// (`additionalProperties:false` with nothing declared accepts exactly `{}`, which IS the contract of
/// an argument-less action); [`capability_action_input_schema`] folds one capability's declared
/// `ActionArgDef` leaves in.
pub fn capability_action_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": {}, "additionalProperties": false })
}

pub fn capability_action_input_schema(capability_id: &str, properties: serde_json::Map<String, serde_json::Value>, required: Vec<String>) -> serde_json::Value {
    let mut schema = capability_action_input_shape();
    let map = schema.as_object_mut().expect("object schema");
    map.insert("properties".to_string(), serde_json::Value::Object(properties));
    if !required.is_empty() {
        map.insert("required".to_string(), serde_json::Value::Array(required.into_iter().map(serde_json::Value::String).collect()));
    }
    wire(capability_id, "input", schema)
}

/// 📐️ The permissive input envelope a descriptor-derived catalog row publishes before its arguments
/// are typed.
pub fn capability_generic_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn capability_generic_input_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "input", capability_generic_input_shape())
}

/// 📐️ No `manifest::ActionDefinition` carries a typed output shape yet (the bridge's
/// `AppFrame::Emit`/`DispatchReport` payload is dynamic), so every capability's `output_schema` is
/// this envelope tagged with its own `$id` until a later packet types individual results.
pub fn capability_generic_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn capability_generic_output_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "output", capability_generic_output_shape())
}

/// 📐️ `📋️master.md` §3.2: "`dialogs` → `ui.dialog.open`". The catalogued shape is the real value
/// space (any declared dialog id, plus an opaque argument bag);
/// [`ui_dialog_open_input_schema`] narrows `dialogId` to the ids one compiled workspace actually
/// declares.
pub fn ui_dialog_open_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "dialogId": { "type": "string", "minLength": 1 }, "args": { "type": "object" } },
        "required": ["dialogId"],
        "additionalProperties": false,
    })
}

pub fn ui_dialog_open_input_schema(dialog_ids: &[String]) -> serde_json::Value {
    let mut schema = ui_dialog_open_input_shape();
    schema["properties"]["dialogId"] = serde_json::json!({ "type": "string", "enum": dialog_ids.iter().map(|id| serde_json::Value::String(id.clone())).collect::<Vec<_>>() });
    wire("ui.dialog.open", "input", schema)
}

/// 📐️ `📋️master.md` §3.2: "`examples` → the `artifact.create` template enum". The catalogued shape
/// is the real value space (`kind` plus any `<plugin_id>:<example_id>` template);
/// [`artifact_create_template_input_schema`] narrows `template` to the templates one compiled
/// workspace actually declares.
pub fn artifact_create_template_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "kind": { "type": "string" }, "template": { "type": "string", "minLength": 1 } },
        "required": ["kind"],
        "additionalProperties": false,
    })
}

pub fn artifact_create_template_input_schema(template_ids: &[String]) -> serde_json::Value {
    let mut schema = artifact_create_template_input_shape();
    schema["properties"]["template"] = serde_json::json!({ "type": "string", "enum": template_ids.iter().map(|id| serde_json::Value::String(id.clone())).collect::<Vec<_>>() });
    wire("artifact.create", "input", schema)
}
//#endregion 🔖️CatalogToolSchemas

//#region 🔖️InferenceToolSchemas
pub fn inference_list_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" } }, "additionalProperties": false })
}

pub fn inference_list_input_schema() -> serde_json::Value {
    wire("inference.list", "input", inference_list_input_shape())
}

pub fn inference_list_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": {}, "artifactKind": {}, "declared": { "type": "array" } } })
}

pub fn inference_list_output_schema() -> serde_json::Value {
    wire("inference.list", "output", inference_list_output_shape())
}

pub fn inference_get_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "inferenceSchema": { "type": "string" } },
        "required": ["artifactId", "inferenceSchema"],
        "additionalProperties": false,
    })
}

pub fn inference_get_input_schema() -> serde_json::Value {
    wire("inference.get", "input", inference_get_input_shape())
}

pub fn inference_get_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn inference_get_output_schema() -> serde_json::Value {
    wire("inference.get", "output", inference_get_output_shape())
}

pub fn inference_submit_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "documentId": { "type": "string" },
            "lifetimeMs": { "type": "integer", "minimum": 1, "maximum": crate::inference::INFERENCE_JOB_MAX_LIFETIME_MS },
            "requestId": { "type": "string", "pattern": hex_pattern(crate::inference::INFERENCE_REQUEST_ID_HEX_LENGTH) },
        },
        "required": ["documentId"],
        "additionalProperties": false,
    })
}

pub fn inference_submit_input_schema() -> serde_json::Value {
    wire("inference.submit", "input", inference_submit_input_shape())
}

/// 📐️ `inference.events`/`inference.cancel` — one session-owned job handle plus a bounded progress
/// cursor.
pub fn inference_job_handle_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "jobHandle": { "type": "string" }, "after": { "type": "integer", "minimum": 0, "maximum": crate::inference::INFERENCE_PROGRESS_MAX_CURSOR } },
        "required": ["jobHandle"],
        "additionalProperties": false,
    })
}

pub fn inference_job_handle_input_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "input", inference_job_handle_input_shape())
}

pub fn inference_approve_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "jobHandle": { "type": "string" }, "proposalHash": { "type": "string", "pattern": hex_pattern(crate::inference::INFERENCE_PROPOSAL_HASH_HEX_LENGTH) } },
        "required": ["jobHandle", "proposalHash"],
        "additionalProperties": false,
    })
}

pub fn inference_approve_input_schema() -> serde_json::Value {
    wire("inference.approve", "input", inference_approve_input_shape())
}

pub fn inference_job_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn inference_job_output_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "output", inference_job_output_shape())
}
//#endregion 🔖️InferenceToolSchemas

//#region 🔖️HubMirror
/// 🔢️ `^[0-9a-f]{n}$` — the lower-hex pattern `🌎️hub`'s own schemas spell out literally and
/// `crate::inference::is_lower_hex` enforces in Rust. Built from the length constant so the two can
/// never drift.
fn hex_pattern(length: usize) -> String {
    format!("^[0-9a-f]{{{length}}}$")
}

/// ✅️ The closed approval intent `POST …/spaces/{space}/documents/{doc}/inference/gis-map/jobs/
/// {job}/approval` accepts, byte for byte — os is a CLIENT of hub here, so this export is an
/// explicit MIRROR of hub's own authority (decoded in Rust by
/// `🌎️hub/💡️inference/🧬️schema/✅️approval/🦀️.rs`'s `InferenceApprovalRequestV1::decode`), never a
/// second authority. `🧪️Tests::os_mirror_of_the_hub_approval_request_is_structurally_identical`
/// fails the moment hub changes it.
///
/// The authority is `🌎️hub/💡️inference/🧬️schema/🔣️.json#/$defs/InferenceApprovalRequestV1`; hub names
/// its `jobId`/`proposalHash` patterns through `$ref`s, so the test inlines them before comparing.
pub fn gis_map_inference_approval_request_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["schema", "version", "jobId", "proposalHash"],
        "properties": {
            "schema": { "const": crate::inference::GIS_MAP_INFERENCE_APPROVAL_SCHEMA },
            "version": { "const": 1 },
            "jobId": { "type": "string", "pattern": hex_pattern(crate::inference::INFERENCE_REQUEST_ID_HEX_LENGTH) },
            "proposalHash": { "type": "string", "pattern": hex_pattern(crate::inference::INFERENCE_PROPOSAL_HASH_HEX_LENGTH) },
        },
    })
}
//#endregion 🔖️HubMirror
//#endregion 🔖️ToolSchemas

//#region 🔖️SchemaCatalog
/// 📋️ `(ExportId, JSON Schema document)` for every schema the `os.mcp` scope owns — the ONE
/// registry every facet reads from and the normative source `🧬️schema/🔣️.json` (draft-07) and
/// `🧬️schema/🟦️.ts` are generated from (`bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`).
/// Three families, one catalogue:
///
/// 1. **gateway wire types** — `schemars`-derived from the structs above (`RevisionStamp` …
///    `GatewayError`);
/// 2. **MCP protocol types** — `schemars`-derived from `🧭️protocol/🦀️.rs`. These MIRROR the external
///    Model Context Protocol specification rather than defining it; they are catalogued so the
///    surface this crate speaks is visible, never invisible;
/// 3. **tool schemas** — the hand-authored `inputSchema`/`outputSchema` shapes every facet stamps a
///    `$id` onto (`🔖️ToolSchemas` above), plus the [`gis_map_inference_approval_request_schema`]
///    mirror of hub's own approval contract.
pub fn schemas() -> Vec<(&'static str, serde_json::Value)> {
    let mut entries = vec![
        ("RevisionStamp", serde_json::to_value(schema_for!(RevisionStamp)).expect("RevisionStamp schema")),
        ("InvocationStatus", serde_json::to_value(schema_for!(InvocationStatus)).expect("InvocationStatus schema")),
        ("InvocationReport", serde_json::to_value(schema_for!(InvocationReport)).expect("InvocationReport schema")),
        ("JobState", serde_json::to_value(schema_for!(JobState)).expect("JobState schema")),
        ("GatewayErrorCode", serde_json::to_value(schema_for!(GatewayErrorCode)).expect("GatewayErrorCode schema")),
        ("PreparedActionReport", serde_json::to_value(schema_for!(PreparedActionReport)).expect("PreparedActionReport schema")),
        ("SearchHit", serde_json::to_value(schema_for!(SearchHit)).expect("SearchHit schema")),
        ("JobStatus", serde_json::to_value(schema_for!(JobStatus)).expect("JobStatus schema")),
        ("ContextSummary", serde_json::to_value(schema_for!(ContextSummary)).expect("ContextSummary schema")),
        ("GatewayError", serde_json::to_value(schema_for!(GatewayError)).expect("GatewayError schema")),
        ("ContentBlock", serde_json::to_value(schema_for!(crate::protocol::ContentBlock)).expect("ContentBlock schema")),
        ("Tool", serde_json::to_value(schema_for!(crate::protocol::Tool)).expect("Tool schema")),
        ("CallToolResult", serde_json::to_value(schema_for!(crate::protocol::CallToolResult)).expect("CallToolResult schema")),
        ("Resource", serde_json::to_value(schema_for!(crate::protocol::Resource)).expect("Resource schema")),
        ("ResourceTemplate", serde_json::to_value(schema_for!(crate::protocol::ResourceTemplate)).expect("ResourceTemplate schema")),
        ("ResourceContent", serde_json::to_value(schema_for!(crate::protocol::ResourceContent)).expect("ResourceContent schema")),
        ("PromptArgument", serde_json::to_value(schema_for!(crate::protocol::PromptArgument)).expect("PromptArgument schema")),
        ("Prompt", serde_json::to_value(schema_for!(crate::protocol::Prompt)).expect("Prompt schema")),
        ("PromptMessage", serde_json::to_value(schema_for!(crate::protocol::PromptMessage)).expect("PromptMessage schema")),
        ("PromptGetResult", serde_json::to_value(schema_for!(crate::protocol::PromptGetResult)).expect("PromptGetResult schema")),
        ("NullableRevisionStamp", nullable_revision_stamp_shape()),
        ("CapabilitiesSearchInput", capabilities_search_input_shape()),
        ("CapabilitiesSearchOutput", capabilities_search_output_shape()),
        ("CapabilitiesDescribeInput", capabilities_describe_input_shape()),
        ("CapabilitiesDescribeOutput", capabilities_describe_output_shape()),
        ("ContextResolveInput", context_resolve_input_shape()),
        ("ContextResolveOutput", context_resolve_output_shape()),
        ("ActionPrepareInput", action_prepare_input_shape()),
        ("ActionInvokeInput", action_invoke_input_shape()),
        ("HandleInput", handle_input_shape()),
        ("TransactionBeginInput", transaction_begin_input_shape()),
        ("ArtifactOpenInput", artifact_open_input_shape()),
        ("ArtifactOpenOutput", artifact_open_output_shape()),
        ("ArtifactCreateInput", artifact_create_input_shape()),
        ("ArtifactCreateOutput", artifact_create_output_shape()),
        ("ArtifactValidateInput", artifact_validate_input_shape()),
        ("ArtifactValidateOutput", artifact_validate_output_shape()),
        ("ArtifactSnapshotInput", artifact_snapshot_input_shape()),
        ("ArtifactSnapshotOutput", artifact_snapshot_output_shape()),
        ("ArtifactExportInput", artifact_export_input_shape()),
        ("ArtifactExportOutput", artifact_export_output_shape()),
        ("UiFocusInput", ui_focus_input_shape()),
        ("UiFocusOutput", ui_focus_output_shape()),
        ("UiRevealInput", ui_reveal_input_shape()),
        ("UiRevealOutput", ui_reveal_output_shape()),
        ("JobGetInput", job_get_input_shape()),
        ("JobCancelInput", job_cancel_input_shape()),
        ("JobSnapshotOutput", job_snapshot_output_shape()),
        ("CapabilityActionInput", capability_action_input_shape()),
        ("CapabilityGenericInput", capability_generic_input_shape()),
        ("CapabilityGenericOutput", capability_generic_output_shape()),
        ("UiDialogOpenInput", ui_dialog_open_input_shape()),
        ("ArtifactCreateTemplateInput", artifact_create_template_input_shape()),
        ("InferenceListInput", inference_list_input_shape()),
        ("InferenceListOutput", inference_list_output_shape()),
        ("InferenceGetInput", inference_get_input_shape()),
        ("InferenceGetOutput", inference_get_output_shape()),
        ("InferenceSubmitInput", inference_submit_input_shape()),
        ("InferenceJobHandleInput", inference_job_handle_input_shape()),
        ("InferenceApproveInput", inference_approve_input_shape()),
        ("InferenceJobOutput", inference_job_output_shape()),
        ("GisMapInferenceApprovalRequestV1", gis_map_inference_approval_request_schema()),
    ];
    for (_, schema) in entries.iter_mut() {
        normalize_boolean_subschemas(schema);
    }
    entries
}

/// 🪞️ The whole `os.mcp` scope as ONE draft-07 document — the exact bytes
/// `🧬️schema/🔣️.json` carries, emitted by `semio-os-mcp schemas` and written by
/// `bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`.
///
/// Three real conversions turn [`schemas`]'s wire documents into `$defs` entries, none of them a
/// relabelling: the per-entry dialect/`$id` header is dropped (a `$defs` member may declare
/// neither), `schemars`' own `definitions` bucket is hoisted to the document's single `$defs` (every
/// hoisted name must already BE a registry export, or this panics rather than publishing a schema
/// nothing catalogues), and every `#/definitions/X` reference is repointed at `#/$defs/X`.
pub fn schema_mirror_document() -> serde_json::Value {
    let catalog = schemas();
    let export_ids: std::collections::BTreeSet<&str> = catalog.iter().map(|(name, _)| *name).collect();
    let mut defs = serde_json::Map::new();
    for (name, schema) in catalog.iter() {
        let mut entry = schema.clone();
        let hoisted = strip_wire_header(&mut entry);
        for (definition_name, definition) in hoisted {
            assert!(export_ids.contains(definition_name.as_str()), "`{definition_name}` is reachable from `{name}` but is not an os.mcp export — register it in `schemas()`");
            let mut definition = definition;
            strip_wire_header(&mut definition);
            defs.entry(definition_name).or_insert(definition);
        }
        defs.insert((*name).to_string(), entry);
    }
    for entry in defs.values_mut() {
        resolve_schemars_number_formats(entry);
    }
    serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": SCHEMA_MIRROR_ID,
        "title": "Semio OS MCP Gateway",
        "$defs": defs,
    })
}

/// 🖨️ [`schema_mirror_document`] rendered exactly as `🧬️schema/🔣️.json` stores it: two-space
/// indentation, one trailing newline — so `semio-os-mcp schemas > 🔣️.json` is byte-idempotent.
pub fn schema_mirror_json() -> String {
    let mut buffer = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(&mut buffer, serde_json::ser::PrettyFormatter::with_indent(b"  "));
    Serialize::serialize(&schema_mirror_document(), &mut serializer).expect("the mirror serializes");
    let mut rendered = String::from_utf8(buffer).expect("the mirror is utf-8");
    rendered.push('\n');
    rendered
}

/// ✂️ Removes the per-entry dialect/`$id` header and the `schemars` `definitions` bucket, repointing
/// every `#/definitions/X` reference at `#/$defs/X`; returns whatever was hoisted out.
fn strip_wire_header(value: &mut serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    let mut hoisted = serde_json::Map::new();
    if let Some(map) = value.as_object_mut() {
        map.remove("$schema");
        map.remove("$id");
        map.remove("title");
        if let Some(serde_json::Value::Object(definitions)) = map.remove("definitions") {
            hoisted = definitions;
        }
    }
    repoint_definition_refs(value);
    hoisted
}

/// 🔢️ Replaces `schemars`' Rust-typed number `format` annotations with the draft-07 semantics they
/// stand for, because they are the ONE thing in these documents a strict validator cannot read:
/// `new Ajv({ strict: true })` throws `unknown format "double"` and refuses to compile the whole
/// mirror (observed live running `bun ./📜️script.ts schema-mirror`). An unsigned format becomes a
/// real `minimum: 0`; a signed/floating one carries no constraint JSON can express and is dropped.
/// Only the generated mirror is normalized — `schemas()` keeps the annotation for the MCP wire,
/// whose SDK validators accept it.
fn resolve_schemars_number_formats(value: &mut serde_json::Value) {
    const UNSIGNED: &[&str] = &["uint", "uint8", "uint16", "uint32", "uint64", "uint128", "usize"];
    const UNCONSTRAINED: &[&str] = &["int", "int8", "int16", "int32", "int64", "int128", "isize", "float", "double"];
    match value {
        serde_json::Value::Object(map) => {
            if let Some(format) = map.get("format").and_then(serde_json::Value::as_str).map(str::to_string) {
                if UNSIGNED.contains(&format.as_str()) {
                    map.remove("format");
                    map.entry("minimum").or_insert(serde_json::json!(0));
                } else if UNCONSTRAINED.contains(&format.as_str()) {
                    map.remove("format");
                }
            }
            for entry in map.values_mut() {
                resolve_schemars_number_formats(entry);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                resolve_schemars_number_formats(item);
            }
        }
        _ => {}
    }
}

/// 🔗️ `#/definitions/X` → `#/$defs/X`, everywhere in one document.
fn repoint_definition_refs(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(reference)) = map.get_mut("$ref") {
                if let Some(name) = reference.strip_prefix("#/definitions/") {
                    *reference = format!("#/$defs/{name}");
                }
            }
            for entry in map.values_mut() {
                repoint_definition_refs(entry);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                repoint_definition_refs(item);
            }
        }
        _ => {}
    }
}

/// 🔧️ The catalogued document one `Tool.outputSchema` publishes, looked up by its `ExportId` —
/// the ONE `schemars` derivation per type, normalized once by [`normalize_boolean_subschemas`]
/// (`PreparedActionReport.preview`'s free-form `serde_json::Value` field otherwise emits a bare
/// boolean sub-schema, which the MCP SDK's Zod model rejects outright: `$ZodError` at
/// `tools[2].outputSchema.properties.preview`, caught live by
/// `bun nx run @semio-tech/framework-os-mcp:test-quick`).
pub fn tool_output_schema(export_id: &str) -> serde_json::Value {
    schemas().into_iter().find(|(name, _)| *name == export_id).map(|(_, schema)| schema).unwrap_or_else(|| panic!("{export_id} is not an os.mcp schema export"))
}

/// 🪞️ The generated draft-07 mirror `🧬️schema/🟦️.ts` and every TypeScript consumer read — its
/// `$defs` key set is asserted equal to [`schemas`] by
/// `🧪️Tests::the_json_mirror_publishes_exactly_the_registry_exports`.
pub const SCHEMA_MIRROR_JSON: &str = include_str!("🔣️.json");

/// 🆔️ The `$id` of [`SCHEMA_MIRROR_JSON`] — the `os.mcp` scope's one component document.
pub const SCHEMA_MIRROR_ID: &str = "https://semio.tech/schema/os/mcp/component.json";
//#endregion 🔖️SchemaCatalog

//#region ✅️Validation

/// 🧬️ Compiles MCP's serde boundary through the repo-owned string contract without leaking either
/// JSON implementation across the framework schema API.
pub(crate) fn compile_validator(schema: &serde_json::Value) -> Result<semio_framework_schema::OwnedJsonSchemaValidator, String> {
    let schema = serde_json::to_string(schema).map_err(|error| error.to_string())?;
    semio_framework_schema::OwnedJsonSchemaValidator::compile(&schema).map_err(|error| error.to_string())
}

/// ✅️ Validates one MCP value and preserves the owned validator's deterministic first diagnostic.
pub(crate) fn validate(validator: &semio_framework_schema::OwnedJsonSchemaValidator, value: &serde_json::Value) -> Result<semio_framework_schema::ValidationProgress, String> {
    let value = serde_json::to_string(value).map_err(|error| error.to_string())?;
    validator.validate_json(&value).map_err(|error| error.to_string())
}

//#endregion ✅️Validation

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    fn has_bare_boolean(value: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::Bool(_) => true,
            serde_json::Value::Object(map) => map.iter().any(|(key, entry)| !SCHEMA_BOOLEAN_KEYWORDS.contains(&key.as_str()) && has_bare_boolean(entry)),
            serde_json::Value::Array(items) => items.iter().any(has_bare_boolean),
            _ => false,
        }
    }

    #[test]
    fn no_published_schema_contains_a_bare_boolean_subschema() {
        for (name, schema) in schemas() {
            assert!(!has_bare_boolean(&schema), "{name} still publishes a boolean sub-schema — the MCP SDK's Zod model rejects the whole tools/list response when it sees one");
        }
    }

    #[test]
    fn normalize_rewrites_booleans_but_leaves_boolean_keywords_alone() {
        let mut value = serde_json::json!({ "properties": { "free": true, "never": false }, "additionalProperties": false, "uniqueItems": true });
        normalize_boolean_subschemas(&mut value);
        assert_eq!(value["properties"]["free"], serde_json::json!({}));
        assert_eq!(value["properties"]["never"], serde_json::json!({ "not": {} }));
        assert_eq!(value["additionalProperties"], serde_json::json!(false));
        assert_eq!(value["uniqueItems"], serde_json::json!(true));
    }

    fn revision_stamp_example() -> serde_json::Value {
        serde_json::to_value(RevisionStamp { artifact_id: "cad-1".into(), head_edit_id: "edit-7".into(), cursor: "c0".into() }).unwrap()
    }

    fn invocation_report_example() -> serde_json::Value {
        serde_json::to_value(InvocationReport {
            invocation_id: "inv-1".into(),
            capability_id: "cad.viewport.translateSelection".into(),
            status: InvocationStatus::Succeeded,
            affected_resources: vec!["semio://artifact/cad-1".into()],
            revision_before: Some(RevisionStamp { artifact_id: "cad-1".into(), head_edit_id: "edit-6".into(), cursor: "c0".into() }),
            revision_after: Some(RevisionStamp { artifact_id: "cad-1".into(), head_edit_id: "edit-7".into(), cursor: "c1".into() }),
            diff_uri: None,
            warnings: Vec::new(),
            undo_token: Some("undo_abc".into()),
            postconditions: vec!["selection.moved".into()],
            replayed: false,
        })
        .unwrap()
    }

    fn prepared_action_report_example() -> serde_json::Value {
        serde_json::to_value(PreparedActionReport {
            prepared_handle: "prep_abc".into(),
            capability_id: "cad.viewport.translateSelection".into(),
            expected_revision: Some(RevisionStamp { artifact_id: "cad-1".into(), head_edit_id: "edit-6".into(), cursor: "c0".into() }),
            preview: serde_json::json!({ "dx": 1.0, "dy": 0.0, "dz": 0.0 }),
            expires_at_ms: 1_000,
        })
        .unwrap()
    }

    fn search_hit_example() -> serde_json::Value {
        serde_json::to_value(SearchHit {
            capability_id: "cad.viewport.translateSelection".into(),
            title: "Translate selection".into(),
            description: "Moves the current selection by (dx, dy, dz)".into(),
            score: 0.92,
            plugin_id: "cad".into(),
            app_id: "viewport".into(),
        })
        .unwrap()
    }

    fn job_status_example() -> serde_json::Value {
        serde_json::to_value(JobStatus { job_id: "job_1".into(), state: JobState::Running, progress: Some(0.5), result: None, error: None }).unwrap()
    }

    fn context_summary_example() -> serde_json::Value {
        serde_json::to_value(ContextSummary {
            session_id: "sess_1".into(),
            principal: "agent:local".into(),
            scopes: vec!["cad.viewport.translateSelection".into()],
            active_artifact_id: Some("cad-1".into()),
            catalog_hash: "blake3:abc".into(),
            locale: "en".into(),
        })
        .unwrap()
    }

    fn gateway_error_example() -> serde_json::Value {
        serde_json::to_value(GatewayError::new(GatewayErrorCode::NotFound, "no such capability")).unwrap()
    }

    #[test]
    fn every_schema_compiles_and_validates_its_own_example() {
        let examples: Vec<(&str, serde_json::Value)> = vec![
            ("RevisionStamp", revision_stamp_example()),
            ("InvocationReport", invocation_report_example()),
            ("PreparedActionReport", prepared_action_report_example()),
            ("SearchHit", search_hit_example()),
            ("JobStatus", job_status_example()),
            ("ContextSummary", context_summary_example()),
            ("GatewayError", gateway_error_example()),
        ];
        let catalog = schemas();
        for (name, schema) in &catalog {
            let owned = compile_validator(schema).unwrap_or_else(|error| panic!("{name}: owned schema did not compile: {error}"));
            let Some((_, example)) = examples.iter().find(|(example_name, _)| example_name == name) else { continue };
            validate(&owned, example).unwrap_or_else(|error| panic!("{name}: own example failed validation: {error}"));
            assert!(validate(&owned, &serde_json::Value::Null).is_err(), "{name}: null must fail the object-root schema");
        }
        for (name, _) in &examples {
            assert!(catalog.iter().any(|(export_id, _)| export_id == name), "{name} lost its registry export");
        }
    }

    #[test]
    fn every_export_id_is_unique_and_pascal_case() {
        let names: Vec<&str> = schemas().into_iter().map(|(name, _)| name).collect();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), names.len(), "an ExportId is registered twice");
        for name in &names {
            let first = name.chars().next().expect("non-empty ExportId");
            assert!(first.is_ascii_uppercase(), "{name} is not PascalCase");
            assert!(name.chars().all(|character| character.is_ascii_alphanumeric()), "{name} is not PascalCase");
        }
    }

    #[test]
    fn the_json_mirror_publishes_exactly_the_registry_exports() {
        let mirror: serde_json::Value = serde_json::from_str(SCHEMA_MIRROR_JSON).expect("🔣️.json parses");
        assert_eq!(mirror["$schema"], "http://json-schema.org/draft-07/schema#", "the module mirror must be draft-07 at its root");
        assert_eq!(mirror["$id"], SCHEMA_MIRROR_ID);
        let mut published: Vec<&str> = mirror["$defs"].as_object().expect("$defs object").keys().map(String::as_str).collect();
        published.sort_unstable();
        let mut registered: Vec<&str> = schemas().into_iter().map(|(name, _)| name).collect();
        registered.sort_unstable();
        assert_eq!(published, registered, "🔣️.json is stale — regenerate it with `bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`");
        assert_eq!(mirror, schema_mirror_document(), "🔣️.json's CONTENT drifted from the registry — regenerate it with `bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`");
        for value in mirror["$defs"].as_object().expect("$defs object").values() {
            assert!(value.get("$schema").is_none(), "a $defs entry must not redeclare the dialect");
            assert!(value.get("$id").is_none(), "a $defs entry must not carry a wire $id");
        }
    }

    /// 🪞️ os is a CLIENT of hub for the GIS Map approval intent — this proves the `os.mcp` mirror is
    /// byte-identical in value space to hub's own authority, so a hub-side change breaks here loudly
    /// instead of drifting. It only READS hub's file.
    #[test]
    fn os_mirror_of_the_hub_approval_request_is_structurally_identical() {
        let hub: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🌎️hub/💡️inference/🧬️schema/🔣️.json")).expect("hub module schema parses");
        let authority = hub["$defs"].get("InferenceApprovalRequestV1").expect("hub publishes InferenceApprovalRequestV1");
        let authority = inline_local_refs(authority, &hub);
        let mirror = gis_map_inference_approval_request_schema();
        for key in ["type", "additionalProperties", "required", "properties"] {
            assert_eq!(&mirror[key], &authority[key], "the os.mcp approval mirror drifted from hub on `{key}`");
        }
        let approval = crate::inference::GisMapInferenceApprovalRequestV1::new("00112233445566778899aabbccddeeff", &"ab".repeat(32));
        let owned = compile_validator(&mirror).expect("the mirror compiles");
        validate(&owned, &serde_json::to_value(&approval).expect("approval serializes")).expect("the Rust type's own encoding satisfies hub's contract");
    }

    /// 🔗️ Replaces every `{"$ref": "#/$defs/X"}` with the document's own `X`, so a mirror that inlines
    /// a pattern can be compared with an authority that names it.
    fn inline_local_refs(value: &serde_json::Value, document: &serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(map) => match map.get("$ref").and_then(serde_json::Value::as_str).and_then(|reference| reference.strip_prefix("#/$defs/")) {
                Some(name) => inline_local_refs(&document["$defs"][name], document),
                None => serde_json::Value::Object(map.iter().map(|(key, entry)| (key.clone(), inline_local_refs(entry, document))).collect()),
            },
            serde_json::Value::Array(items) => serde_json::Value::Array(items.iter().map(|item| inline_local_refs(item, document)).collect()),
            other => other.clone(),
        }
    }
}
//#endregion 🧪️Tests
