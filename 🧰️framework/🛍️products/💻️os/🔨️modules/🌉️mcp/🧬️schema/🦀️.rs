//! 🧬️ Gateway wire types (`📋️master.md` §"MCP tool names"/§"Observe"/§"Verify"/§"Idempotency") —
//! `schemars`-derived so every type publishes a normative JSON Schema (2020-12) through
//! [`schemas`], validated at kernel/backend boundaries the same way `🧰️framework/🔨️modules/🧬️schema`
//! validates artifact snapshots. `GatewayError` is re-exported from `crate::errors` rather than
//! redefined here — one type, one owning facet.

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

//#region 🔖️SchemaCatalog
/// 📋️ `(name, JSON Schema 2020-12 document)` for every gateway wire type this facet owns — the
/// normative source `🧬️schema/🔣️.json`/`🟦️.ts` mirrors are generated from, and
/// what a future `SchemaCatalog::register` (framework `🧬️schema` module) loads at startup.
pub fn schemas() -> Vec<(&'static str, serde_json::Value)> {
    let mut entries = vec![
        ("RevisionStamp", serde_json::to_value(schema_for!(RevisionStamp)).expect("RevisionStamp schema")),
        ("InvocationReport", serde_json::to_value(schema_for!(InvocationReport)).expect("InvocationReport schema")),
        ("PreparedActionReport", serde_json::to_value(schema_for!(PreparedActionReport)).expect("PreparedActionReport schema")),
        ("SearchHit", serde_json::to_value(schema_for!(SearchHit)).expect("SearchHit schema")),
        ("JobStatus", serde_json::to_value(schema_for!(JobStatus)).expect("JobStatus schema")),
        ("ContextSummary", serde_json::to_value(schema_for!(ContextSummary)).expect("ContextSummary schema")),
        ("GatewayError", serde_json::to_value(schema_for!(GatewayError)).expect("GatewayError schema")),
    ];
    for (_, schema) in entries.iter_mut() {
        normalize_boolean_subschemas(schema);
    }
    entries
}
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
        assert_eq!(catalog.len(), examples.len(), "every schema must have exactly one example in this test");
        for (name, schema) in &catalog {
            let (_, example) = examples.iter().find(|(example_name, _)| example_name == name).unwrap_or_else(|| panic!("missing example for {name}"));
            let owned = compile_validator(schema).unwrap_or_else(|error| panic!("{name}: owned schema did not compile: {error}"));
            validate(&owned, example).unwrap_or_else(|error| panic!("{name}: own example failed validation: {error}"));
            assert!(validate(&owned, &serde_json::Value::Null).is_err(), "{name}: null must fail the object-root schema");
        }
    }

    #[test]
    fn schemas_cover_exactly_the_seven_gateway_wire_types() {
        let names: Vec<&str> = schemas().into_iter().map(|(name, _)| name).collect();
        assert_eq!(names, vec!["RevisionStamp", "InvocationReport", "PreparedActionReport", "SearchHit", "JobStatus", "ContextSummary", "GatewayError"]);
    }
}
//#endregion 🧪️Tests
