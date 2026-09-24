//! 💼️ One inference service model for every declared service. A plugin declares an inference in its
//! descriptor; this module resolves WHERE it executes and runs it as one job whose lifecycle, events,
//! cancellation, proposal and approval read the same whichever site runs it:
//!
//! - **guest** — the service's own plugin guest, in this gateway (the engine `inference_run` uses).
//!   Its result becomes a PROPOSAL when the service declares a commit action
//!   (`InferencePayloadContract.commit`); an approval commits that proposal by invoking the action on
//!   the document through the ordinary `ActionAdapter` path, so the edit reaches the document ledger
//!   (and, hub-bound, every collaborator) exactly like any other agent edit.
//! - **hub** — a service the bound hub executes server-side and publishes in its readiness
//!   (`features.inferenceServices[]`), relayed through the route family the hub publishes with it.
//!
//! Schema: `🧬️schema/🔣️.json` (`InferenceJobPageV1`, `HubInferenceServiceV1`, `InferenceServiceLawV1`).
//! Law: `🧫️fixtures/💼️inference-service-law.json`, replayed by `🧪️tests/💼️inference-service-law`.

use super::{DeclaredInference, InferenceJobStateV1, InferenceProposalStateV1, HubInferenceEventPageV1, HubInferenceJobReceiptV1};
use crate::catalog::{Catalog, CapabilitySource};
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::ui::{JobEvent, JobSnapshot, JobStatus};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

//#region 🔖️Wire
pub const INFERENCE_JOB_PAGE_SCHEMA: &str = "semio.mcp.inference-job/v1";
pub const INFERENCE_PROPOSAL_DIGEST_DOMAIN: &str = "semio.mcp.inference-proposal/v1";
pub const INFERENCE_JOB_EVENT_PAGE_MAX_ITEMS: usize = 64;

/// 🧭️ Where one declared inference service executes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceExecutionSiteV1 {
    Guest,
    Hub,
}

/// 🌎️ One service a hub executes itself, exactly as its readiness publishes it.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceServiceV1 {
    pub service_id: String,
    pub route: String,
}

/// 🗓️ One lifecycle fact of one job.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceJobEventV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// 📈️ One progress row, as the fraction of the job's own work done.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceJobProgressV1 {
    pub cursor: u64,
    pub fraction: f64,
    pub at_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// 🧾️ What an approval would commit: the declared commit action, its exact input, and the digest an
/// approval must name. `preview` is the service's own result for a reader to inspect first.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceProposalV1 {
    pub hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<serde_json::Value>,
}

/// 📃️ The one page every quartet tool answers with, for either execution site.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceJobPageV1 {
    pub schema: String,
    pub job_id: String,
    pub service_id: String,
    pub artifact_kind: String,
    pub plugin_id: String,
    pub document_id: String,
    pub site: InferenceExecutionSiteV1,
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub cancel_requested: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposal: Option<InferenceProposalV1>,
    pub events: Vec<InferenceJobEventV1>,
    pub progress: Vec<InferenceJobProgressV1>,
    pub next_cursor: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}
//#endregion 🔖️Wire

//#region 🔖️Service
/// 💡️ One declared inference, resolved to the site that executes it and the action that commits it.
#[derive(Clone, Debug, PartialEq)]
pub struct InferenceService {
    pub declared: DeclaredInference,
    pub site: InferenceExecutionSiteV1,
    pub route: Option<String>,
    pub commit_action: Option<String>,
}

impl InferenceService {
    pub fn service_id(&self) -> &str {
        &self.declared.inference_schema
    }

    pub fn plugin_id(&self) -> &str {
        self.declared.route_plugin_id()
    }
}

/// 📜️ The commit action a declared service names in its published contract, if any.
pub fn declared_commit_action(declared: &DeclaredInference) -> Option<String> {
    declared.payload.as_ref().and_then(|contract| contract.commit.as_ref()).map(|commit| commit.action.clone())
}

/// 🧭️ Picks the ONE declared service a request names on `artifact_kind`, and its execution site. The
/// hub site is chosen only for a service the bound hub itself publishes; every other service runs in
/// its plugin's guest here. Pure: the law replays it row for row.
pub fn select_inference_service(declared: &[DeclaredInference], hub_services: &[HubInferenceServiceV1], artifact_kind: &str, inference_schema: Option<&str>, plugin_id: Option<&str>) -> Result<InferenceService, GatewayError> {
    let on_kind: Vec<&DeclaredInference> = declared.iter().filter(|item| item.artifact_kind == artifact_kind).collect();
    let matches: Vec<&DeclaredInference> = on_kind.iter().copied().filter(|item| inference_schema.is_none_or(|schema| item.inference_schema == schema) && plugin_id.is_none_or(|plugin| item.route_plugin_id() == plugin)).collect();
    let declared_here: Vec<&str> = on_kind.iter().map(|item| item.inference_schema.as_str()).collect();
    let item = match matches.as_slice() {
        [] => {
            return Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no inference service {} is declared for artifact kind `{artifact_kind}`", inference_schema.map_or_else(|| "at all".to_string(), |schema| format!("`{schema}`"))))
                .with_details(serde_json::json!({ "artifactKind": artifact_kind, "inferenceSchema": inference_schema, "declaredForThisKind": declared_here })))
        }
        [single] => (*single).clone(),
        several => {
            let schemas: std::collections::BTreeSet<&str> = several.iter().map(|item| item.inference_schema.as_str()).collect();
            let (field, message) = if schemas.len() > 1 { ("inferenceSchema", format!("`{artifact_kind}` declares {} inference services — name one with `inferenceSchema`", schemas.len())) } else { ("pluginId", format!("`{artifact_kind}/{}` is declared by {} plugins — name one with `pluginId`", several[0].inference_schema, several.len())) };
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, message).with_details(serde_json::json!({ "field": field, "inferenceSchemas": schemas, "pluginIds": several.iter().map(|item| item.route_plugin_id()).collect::<Vec<_>>() })));
        }
    };
    let hub = hub_services.iter().find(|service| service.service_id == item.inference_schema);
    Ok(InferenceService {
        site: if hub.is_some() { InferenceExecutionSiteV1::Hub } else { InferenceExecutionSiteV1::Guest },
        route: hub.map(|service| service.route.clone()),
        commit_action: declared_commit_action(&item),
        declared: item,
    })
}
//#endregion 🔖️Service

//#region 🔖️Proposal
/// 🧾️ Canonical JSON: object keys sorted at every depth, no whitespace — the one byte form a proposal
/// digest is taken over, independent of any map's insertion order.
pub fn canonical_json(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let fields: Vec<String> = keys.into_iter().map(|key| format!("{}:{}", serde_json::Value::String(key.clone()), canonical_json(&map[key]))).collect();
            format!("{{{}}}", fields.join(","))
        }
        serde_json::Value::Array(items) => format!("[{}]", items.iter().map(canonical_json).collect::<Vec<_>>().join(",")),
        other => other.to_string(),
    }
}

/// 🎯️ The capability that invokes `action` on `artifact_kind` for `plugin_id` — the first in catalog
/// order, since an app-level action is compiled once per window kind with one shared effect.
pub fn commit_capability<'a>(catalog: &'a Catalog, artifact_kind: &str, plugin_id: &str, action: &str) -> Option<&'a crate::catalog::CapabilityDefinition> {
    catalog.entries.iter().find(|entry| entry.artifact_kind.as_deref() == Some(artifact_kind) && matches!(&entry.source, CapabilitySource::Action { plugin_id: owner, action_id, .. } if owner == plugin_id && action_id == action))
}

/// 🧾️ Builds the proposal a guest result offers: the commit action's input is the result's fields
/// the action declares as arguments (`declared_args`), nothing else, and the digest binds service,
/// document, capability and input together. Pure: the law replays it.
pub fn guest_proposal(service_id: &str, document_id: &str, action: &str, capability_id: &str, declared_args: &[String], result: &serde_json::Value) -> Result<InferenceProposalV1, GatewayError> {
    let Some(object) = result.as_object() else {
        return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("`{service_id}` answered a non-object result, so its commit action `{action}` has no arguments to take")));
    };
    let input: serde_json::Map<String, serde_json::Value> = object.iter().filter(|(key, _)| declared_args.iter().any(|arg| arg == *key)).map(|(key, value)| (key.clone(), value.clone())).collect();
    let input = serde_json::Value::Object(input);
    let bound = serde_json::json!({ "domain": INFERENCE_PROPOSAL_DIGEST_DOMAIN, "serviceId": service_id, "documentId": document_id, "capabilityId": capability_id, "input": input });
    Ok(InferenceProposalV1 { hash: framework_hash::sha256_hex(canonical_json(&bound).as_bytes()), action: Some(action.to_string()), capability_id: Some(capability_id.to_string()), input: Some(input), preview: Some(result.clone()) })
}

/// 🧾️ The argument ids one capability's input schema declares.
pub fn capability_argument_ids(capability: &crate::catalog::CapabilityDefinition) -> Vec<String> {
    capability.input_schema.get("properties").and_then(serde_json::Value::as_object).map(|properties| properties.keys().cloned().collect()).unwrap_or_default()
}
//#endregion 🔖️Proposal

//#region 🔖️GuestJobs
/// 📒️ What a guest job holds beyond the generic job registry's fold: the service it runs, the
/// document it runs on, its proposal and what committing that proposal produced.
#[derive(Clone, Debug, PartialEq)]
pub struct GuestInferenceJob {
    pub service: InferenceService,
    pub document_id: String,
    pub proposal: Option<InferenceProposalV1>,
    pub committed: Option<serde_json::Value>,
    pub commit_refused: bool,
}

fn guest_jobs() -> &'static Mutex<HashMap<String, GuestInferenceJob>> {
    static JOBS: OnceLock<Mutex<HashMap<String, GuestInferenceJob>>> = OnceLock::new();
    JOBS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn retain_guest_job(job_id: &str, job: GuestInferenceJob) {
    guest_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(job_id.to_string(), job);
}

pub fn guest_job(job_id: &str) -> Option<GuestInferenceJob> {
    guest_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(job_id).cloned()
}

pub fn update_guest_job(job_id: &str, update: impl FnOnce(&mut GuestInferenceJob)) {
    if let Some(job) = guest_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get_mut(job_id) {
        update(job);
    }
}

/// 📃️ Folds one guest job — the generic registry's snapshot and journal plus its own record — into
/// the page every quartet tool answers with. Pure: the law replays it from journals alone.
pub fn guest_page(job_id: &str, snapshot: &JobSnapshot, journal: &[JobEvent], next_cursor: u64, job: &GuestInferenceJob) -> InferenceJobPageV1 {
    let offered = job.proposal.is_some();
    let (state, proposal_state) = match snapshot.status {
        JobStatus::Pending => (InferenceJobStateV1::Accepted, InferenceProposalStateV1::None),
        JobStatus::Running if journal.iter().any(|event| event.kind == "approved") => (InferenceJobStateV1::Running, InferenceProposalStateV1::Approved),
        JobStatus::Running => (InferenceJobStateV1::Running, InferenceProposalStateV1::None),
        JobStatus::AwaitingApproval => (InferenceJobStateV1::Succeeded, InferenceProposalStateV1::Offered),
        JobStatus::Succeeded if job.committed.is_some() => (InferenceJobStateV1::Succeeded, InferenceProposalStateV1::Approved),
        JobStatus::Succeeded => (InferenceJobStateV1::Succeeded, InferenceProposalStateV1::None),
        JobStatus::Failed if job.commit_refused => (InferenceJobStateV1::Failed, InferenceProposalStateV1::Stale),
        JobStatus::Failed => (InferenceJobStateV1::Failed, InferenceProposalStateV1::None),
        JobStatus::Cancelled if offered => (InferenceJobStateV1::Cancelled, InferenceProposalStateV1::Cancelled),
        JobStatus::Cancelled => (InferenceJobStateV1::Cancelled, InferenceProposalStateV1::None),
    };
    let result = match (&snapshot.status, &job.proposal) {
        (JobStatus::Succeeded, None) => snapshot.result.clone(),
        _ => None,
    };
    InferenceJobPageV1 {
        schema: INFERENCE_JOB_PAGE_SCHEMA.to_string(),
        job_id: job_id.to_string(),
        service_id: job.service.service_id().to_string(),
        artifact_kind: job.service.declared.artifact_kind.clone(),
        plugin_id: job.service.plugin_id().to_string(),
        document_id: job.document_id.clone(),
        site: InferenceExecutionSiteV1::Guest,
        state,
        proposal_state,
        cancel_requested: snapshot.cancel_requested,
        proposal: job.proposal.clone(),
        events: journal.iter().filter(|event| event.kind != "progress").map(|event| InferenceJobEventV1 { ordinal: event.ordinal, kind: event.kind.clone(), at_ms: event.at_ms, message: event.message.clone() }).collect(),
        progress: journal.iter().filter(|event| event.kind == "progress").map(|event| InferenceJobProgressV1 { cursor: event.ordinal, fraction: event.progress.unwrap_or(0.0), at_ms: event.at_ms, message: event.message.clone() }).collect(),
        next_cursor,
        result,
        commit: job.committed.clone(),
        error: snapshot.error.as_ref().map(GatewayError::to_tool_error_payload),
    }
}
//#endregion 🔖️GuestJobs

//#region 🔖️HubJobs
/// 🌎️ Projects one hub receipt onto the generic page — the answer `inference_submit` gives for a
/// hub-executed service before its first event read.
pub fn hub_receipt_page(service: &InferenceService, document_id: &str, receipt: &HubInferenceJobReceiptV1) -> InferenceJobPageV1 {
    InferenceJobPageV1 {
        schema: INFERENCE_JOB_PAGE_SCHEMA.to_string(),
        job_id: receipt.job_id.clone(),
        service_id: service.service_id().to_string(),
        artifact_kind: service.declared.artifact_kind.clone(),
        plugin_id: service.plugin_id().to_string(),
        document_id: document_id.to_string(),
        site: InferenceExecutionSiteV1::Hub,
        state: receipt.state,
        proposal_state: receipt.proposal_state,
        cancel_requested: false,
        proposal: receipt.proposal_hash.clone().map(|hash| InferenceProposalV1 { hash, action: None, capability_id: None, input: None, preview: None }),
        events: Vec::new(),
        progress: Vec::new(),
        next_cursor: receipt.cursor,
        result: None,
        commit: None,
        error: None,
    }
}

/// 🌎️ Projects one hub event page onto the generic page. Pure: the law replays it.
pub fn hub_events_page(service: &InferenceService, document_id: &str, page: &HubInferenceEventPageV1) -> InferenceJobPageV1 {
    InferenceJobPageV1 {
        schema: INFERENCE_JOB_PAGE_SCHEMA.to_string(),
        job_id: page.job_id.clone(),
        service_id: service.service_id().to_string(),
        artifact_kind: service.declared.artifact_kind.clone(),
        plugin_id: service.plugin_id().to_string(),
        document_id: document_id.to_string(),
        site: InferenceExecutionSiteV1::Hub,
        state: page.state,
        proposal_state: page.proposal_state,
        cancel_requested: page.cancel_requested,
        proposal: page.proposal_hash.clone().map(|hash| InferenceProposalV1 { hash, action: None, capability_id: None, input: None, preview: page.preview.as_ref().and_then(|preview| serde_json::to_value(preview).ok()) }),
        events: page.events.iter().map(|event| InferenceJobEventV1 { ordinal: event.ordinal, kind: event.kind.clone(), at_ms: event.at_ms, message: None }).collect(),
        progress: page.progress.iter().map(|row| InferenceJobProgressV1 { cursor: row.cursor, fraction: if row.total == 0 { 0.0 } else { row.completed as f64 / row.total as f64 }, at_ms: row.at_ms, message: None }).collect(),
        next_cursor: page.next_cursor,
        result: None,
        commit: None,
        error: None,
    }
}
//#endregion 🔖️HubJobs

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/💼️inference-service-law/🦀️.rs"]
mod inference_service_law;
//#endregion 🧪️Tests
