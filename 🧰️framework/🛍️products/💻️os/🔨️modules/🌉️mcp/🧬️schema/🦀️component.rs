//! 🧬️ Gateway wire types (`📋️master.md` §"MCP tool names"/§"Observe"/§"Verify"/§"Idempotency") —
//! `schemars`-derived so every type publishes a normative JSON Schema (2020-12) through
//! [`schemas`], validated at kernel/backend boundaries the same way `🧰️framework/🔨️modules/🧬️schema`
//! validates artifact snapshots. `GatewayError` is re-exported from `crate::errors` rather than
//! redefined here — one type, one owning facet.

use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

pub use crate::errors::{GatewayError, GatewayErrorCode};

//#region 🔖️RevisionStamp
/// 🧾️ `RevisionStamp{artifact_id, head_edit_id, cursor}` — read from `AppCommand::ReadHistory` →
/// `HistorySnapshot` (`📋️master.md` §"Observe"), the optimistic-concurrency token every
/// `action.invoke{expectedRevision}` and `InvocationReport.revisionBefore/After` carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvocationStatus {
    Succeeded,
    Failed,
    Cancelled,
}

/// 🧾️ `InvocationReport{invocationId, capabilityId, status, affectedResources, revisionBefore/After,
/// diffUri, warnings, undoToken, postconditions, replayed}` (`📋️master.md` §"Verify") — the result
/// re-read after `DispatchReport`/`MergeReport`, and the value an `IdempotencyStore` replay returns
/// verbatim on a repeated `idempotencyKey`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PreparedActionReport {
    pub prepared_handle: String,
    pub capability_id: String,
    pub expected_revision: Option<RevisionStamp>,
    pub preview: serde_json::Value,
    pub expires_at_ms: u64,
}
//#endregion 🔖️PreparedActionReport

//#region 🔖️SearchHit
/// 🔎️ One `capabilities.search` result — `capability_id` is the full `<plugin_id>.<app_id>.<action_id>`
/// grammar (`📋️master.md` D3/§"Id grammar"), never a bare action id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobState {
    Pending,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

/// 🧾️ `job.get` response — `progress` is `0.0..=1.0` when known, `result`/`error` are populated only
/// once `state` is terminal (`Succeeded`/`Failed`/`Cancelled`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    pub job_id: String,
    pub state: JobState,
    pub progress: Option<f64>,
    pub result: Option<serde_json::Value>,
    pub error: Option<GatewayError>,
}
//#endregion 🔖️JobStatus

//#region 🔖️ContextSummary
/// 🧾️ `context.resolve` response — mints/refreshes the implicit `AgentSession` for stdio clients
/// (`📋️master.md` §"AgentSession"). `scopes` are plain capability-id strings rather than
/// `kernel::CapabilityId` — this crate has zero dependency on the kernel crate (§2.6 of this
/// packet's brief), so the newtype cannot be named here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContextSummary {
    pub session_id: String,
    pub principal: String,
    pub scopes: Vec<String>,
    pub active_artifact_id: Option<String>,
    pub catalog_hash: String,
    pub locale: String,
}
//#endregion 🔖️ContextSummary

//#region 🔖️SchemaCatalog
/// 📋️ `(name, JSON Schema 2020-12 document)` for every gateway wire type this facet owns — the
/// normative source `🧬️schema/🔣️component.json`/`🟦️component.ts` mirrors are generated from, and
/// what a future `SchemaCatalog::register` (framework `🧬️schema` module) loads at startup.
pub fn schemas() -> Vec<(&'static str, serde_json::Value)> {
    vec![
        ("RevisionStamp", serde_json::to_value(schema_for!(RevisionStamp)).expect("RevisionStamp schema")),
        ("InvocationReport", serde_json::to_value(schema_for!(InvocationReport)).expect("InvocationReport schema")),
        ("PreparedActionReport", serde_json::to_value(schema_for!(PreparedActionReport)).expect("PreparedActionReport schema")),
        ("SearchHit", serde_json::to_value(schema_for!(SearchHit)).expect("SearchHit schema")),
        ("JobStatus", serde_json::to_value(schema_for!(JobStatus)).expect("JobStatus schema")),
        ("ContextSummary", serde_json::to_value(schema_for!(ContextSummary)).expect("ContextSummary schema")),
        ("GatewayError", serde_json::to_value(schema_for!(GatewayError)).expect("GatewayError schema")),
    ]
}
//#endregion 🔖️SchemaCatalog

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use jsonschema::Validator;

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
            let validator = Validator::new(schema).unwrap_or_else(|error| panic!("{name}: schema did not compile: {error}"));
            validator.validate(example).unwrap_or_else(|error| panic!("{name}: example failed its own schema: {error}"));
        }
    }

    #[test]
    fn schemas_cover_exactly_the_seven_gateway_wire_types() {
        let names: Vec<&str> = schemas().into_iter().map(|(name, _)| name).collect();
        assert_eq!(names, vec!["RevisionStamp", "InvocationReport", "PreparedActionReport", "SearchHit", "JobStatus", "ContextSummary", "GatewayError"]);
    }
}
//#endregion 🧪️Tests
