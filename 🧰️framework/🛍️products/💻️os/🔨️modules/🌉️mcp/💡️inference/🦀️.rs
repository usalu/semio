//! 💡️ Inference access: discovery, execution and the job quartet over ONE service model. Every
//! plugin's committed `🔣️.json` declares its inference roster — `contributions.inference_services`
//! plus every `artifact_contributions[].inferences` row, each with its published payload contract —
//! and that roster is the only source this module reads services from; no plugin id is hardcoded.
//!
//! - `inference_list`/`inference_get` answer discovery from the declared roster.
//! - `inference_submit`/`inference_events`/`inference_cancel`/`inference_approve` run one declared
//!   service as a job (`💼️jobs`): in its plugin's guest here, or — for a service the bound hub
//!   publishes in `features.inferenceServices` — on the hub, through the route family it publishes.
//!   A guest result whose contract declares a commit action is offered as a proposal; approving it
//!   invokes that action on the document through `ActionAdapter`, the normal edit path.
//! - `inference_run` is the synchronous sibling: same resolution, same guest engine, no proposal.
//!
//! Schema: `🧬️schema/🔣️.json`. Laws: `🧫️fixtures/💼️inference-service-law.json`,
//! `🧫️fixtures/⏱️binding-cancellation-law.json`, and the hub wire corpus
//! Installed owner declarations bind request and reply codecs.

use semio_framework_os_kernel::os_directory::DocumentScope;
use std::sync::Arc;
use crate::jobs::{GuestInferenceJob, InferenceService, capability_argument_ids, commit_capability, guest_job, guest_page, guest_proposal, hub_events_page, hub_receipt_page, retain_guest_job, select_inference_service, update_guest_job};
use semio_framework_os_mcp_capabilities::capability_declarations::{inference_approve_capability, inference_cancel_capability, inference_events_capability, inference_get_capability, inference_list_capability, inference_run_capability, inference_submit_capability};
use semio_framework_os_mcp_catalog::capability_tools::tool_from_capability;
use semio_framework_os_mcp_catalog::catalog::{CapabilityDefinition, Catalog};
use semio_framework_os_mcp_catalog::plugin_paths_and_descriptor::find_repo_root;
use semio_framework_os_mcp_catalog::registry::discover_descriptors;
use semio_framework_os_mcp_dispatch::actions::ActionAdapter;
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_foundation::handles::{Attachment, HandleKind, HandleTable, SessionHandle, mint_id};
use semio_framework_os_mcp_foundation::reports::{InvocationReport, InvocationStatus};
use semio_framework_os_mcp_inference_hub::hub_wire::{HubInferenceApprovalRequestV1, HubInferenceJobBindingV1, HubInferenceSubjectV1, HubInferenceSubmitRequestV1, INFERENCE_JOB_EVENT_PAGE_MAX_ITEMS, INFERENCE_JOB_MAX_LIFETIME_MS, INFERENCE_REQUEST_ID_HEX_LENGTH, InferenceExecutionSiteV1, InferenceJobHandlePayloadV1, InferenceJobPageV1, InferenceJobStateV1, InferenceProposalV1, inference_operation_label, inflight_registry};
use semio_framework_os_mcp_plugin_runtime::plugin_binding_and_activation::WorkspaceOrigin;
use semio_framework_os_mcp_port::channel_port::{ActivationScope, ArtifactDocumentBinding, InferCommand, InferenceCancel, InvokeRequest};
use semio_framework_os_mcp_port::policy::{AgentPrincipal, PolicyEngine};
use semio_framework_os_mcp_probe_document::probe_document::PROBE_SCHEMA;
use semio_framework_os_mcp_wire::job_registry::job_registry;
use semio_framework_os_mcp_wire::protocol::{CallToolResult, ContentBlock, GatewayBackend, InMemoryToolRegistry, Resource, ResourceContent};
use semio_framework_os_mcp_workspace::headless_workspace_state::HeadlessWorkspace;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_routing::routing_and_shell_routed::capability_instance_slot;

//#region 🔖️DeclaredInference

/// 💡️ One declared inference service, wire-shaped 1:1 from `semio_framework::ContributedInferenceMetadata`
/// — the exact static fields a plugin's own `🔣️.json` carries, never a live guest call.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DeclaredInference {
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub contributor: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    /// 📜️ The inference's own published payload contract, straight off the committed descriptor —
    /// what a client must SEND and what it gets back. `inference_list` and `capabilities_describe`
    /// carry it, so "what do I put in `payload`?" is answerable without reading plugin source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<semio_framework::InferencePayloadContract>,
}

impl DeclaredInference {
    /// 🧭️ The plugin whose guest actually answers this row — the `contributor` (which equals
    /// `owner` for an owner-authored inference, and names the contributing plugin for a row
    /// contributed onto a dependency's artifact kind, exactly the id
    /// `ArtifactInferenceRouter::register_plugin` is called with). Falls back to `owner` only for a
    /// descriptor old enough to have shipped without the field at all.
    pub fn route_plugin_id(&self) -> &str {
        if self.contributor.is_empty() {
            &self.owner
        } else {
            &self.contributor
        }
    }
}

impl From<&semio_framework::ContributedInferenceMetadata> for DeclaredInference {
    fn from(metadata: &semio_framework::ContributedInferenceMetadata) -> Self {
        Self {
            owner: metadata.owner.clone(),
            artifact_kind: metadata.artifact_kind.clone(),
            artifact_schema: metadata.artifact_schema.clone(),
            artifact_schema_version: metadata.artifact_schema_version,
            inference_schema: metadata.inference_schema.clone(),
            inference_schema_version: metadata.inference_schema_version,
            algorithm_version: metadata.algorithm_version,
            policy_version: metadata.policy_version,
            contributor: metadata.contributor.clone(),
            depends_on: metadata.depends_on.clone(),
            payload: metadata.payload.clone(),
        }
    }
}

/// 💡️ `descriptor.contributions.inference_services` (owner-authored) plus every
/// `artifact_contributions[].inferences` entry (contributed onto a dependency's kind) — the same
/// chain `🏃️run/🦀️.rs`'s `register_plugin` builds for `ArtifactInferenceRouter`.
fn declared_inferences_from_descriptor(descriptor: &semio_framework::PackageDescriptor) -> Vec<DeclaredInference> {
    descriptor.contributions.inference_services.iter().chain(descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter())).map(DeclaredInference::from).collect()
}

/// 🗂️ Every artifact kind the packages in `descriptors` OWN — each app's own dialect kind, plus
/// every plugin-level `artifact_kinds` row (a library package with zero apps declares its kinds
/// there and nowhere else). This is the set an extension's contribution has to land on to be
/// reachable from this workspace at all.
fn owned_artifact_kinds(descriptors: &[semio_framework::PackageDescriptor]) -> std::collections::BTreeSet<String> {
    descriptors.iter().flat_map(|descriptor| descriptor.manifest.apps.iter().map(|app| app.dialect.artifact_kind.clone()).chain(descriptor.manifest.artifact_kinds.iter().map(|kind| kind.id.clone()))).collect()
}

/// 🧩️ The installed packages that CONTRIBUTE an inference onto an artifact kind `primary` already
/// owns, and that `primary` does not itself carry.
///
/// 🐛️ A contribution-only package declares no app, no command and no owner-authored inference
/// service, so `catalog::compile` — which walks apps, commands, modes and
/// `contributions.{inference,mutation,io,composer}_services`, never
/// `contributions.artifact_contributions` — emits no catalog entry for it at all. Folder-mode
/// `HeadlessWorkspace::discovery_descriptors` derives its descriptor set from exactly those catalog
/// entries (`catalog_plugin_ids`), so an extension's inference could never appear in `inference_list`
/// — `cad-extension-aec-building`'s `s.cad-extension-aec-building.building-structure-summary` on
/// `s.cad.cad` is real, committed, and was invisible (`📓️g19-ai-user-experience-audit.md` gap 3,
/// measured by CE1 §3b: the roster was `gis ×1 + wfc ×5`).
///
/// 🎯️ The membership rule is the reachability one, not "every installed package": a contributed
/// inference is listed exactly when the artifact kind it contributes onto is owned by a package this
/// workspace already reaches. An extension whose host plugin is absent contributes to nothing here
/// and stays out, so the roster never advertises a service no artifact in this workspace can carry.
fn contributed_inference_descriptors(primary: &[semio_framework::PackageDescriptor], installed: Vec<semio_framework::PackageDescriptor>) -> Vec<semio_framework::PackageDescriptor> {
    let owned = owned_artifact_kinds(primary);
    let present: std::collections::BTreeSet<&str> = primary.iter().map(|descriptor| descriptor.manifest.plugin_id.as_str()).collect();
    let mut extra: Vec<semio_framework::PackageDescriptor> = installed
        .into_iter()
        .filter(|descriptor| {
            !present.contains(descriptor.manifest.plugin_id.as_str()) && descriptor.contributions.artifact_contributions.iter().any(|contribution| !contribution.inferences.is_empty() && owned.contains(&contribution.artifact_kind))
        })
        .collect();
    extra.sort_by(|left, right| left.manifest.plugin_id.cmp(&right.manifest.plugin_id));
    extra
}

/// 🧩️ Folder mode's installed package set, for [`contributed_inference_descriptors`] — the same
/// `registry::discover_descriptors` scan `build_catalog` already runs, over the SAME generated
/// registry, which lists extension packages alongside plugin ones. A hub workspace never reaches
/// this: its `discovery_descriptors` is the authenticated catalog's whole selection set, extension
/// packages included, so the hub arm is already complete and must never consult a local registry.
fn installed_descriptors_for_contributions(workspace: &HeadlessWorkspace) -> Result<Vec<semio_framework::PackageDescriptor>, GatewayError> {
    #[cfg(test)]
    if let Some(descriptors)=&workspace.test_discovery {return Ok(descriptors.clone());}
    match workspace.origin() {
        WorkspaceOrigin::Hub { .. } => Ok(Vec::new()),
        WorkspaceOrigin::Folder { .. } => discover_descriptors(&find_repo_root()?),
    }
}

/// 💡️ Real, static, plugin-agnostic discovery: the UNION of every registered plugin's own declared
/// inference roster (`workspace.catalog_plugin_ids()` — no plugin id is hardcoded here, no
/// single-plugin assumption; ticket 26/08/29/AI-MCP-END-TO-END packet W8, `📓️w8-capability-routing.md`
/// replaced the OLD `HeadlessWorkspace::resolve_default_plugin_id`-based version, which could only
/// ever read ONE plugin's descriptor and outright FAILED — the same `PLUGIN_UNAVAILABLE` "2+ plugins
/// is ambiguous" defect this whole ticket fixes — the moment a catalog named more than one). Zero
/// registered plugins is still the same typed, retryable `PLUGIN_UNAVAILABLE`; any OTHER plugin's
/// registry/descriptor lookup failing aborts the whole roster rather than silently dropping it.
///
/// 🧩️ …plus every extension-contributed service reachable from that set — see
/// [`contributed_inference_descriptors`] for why a contribution-only package is structurally absent
/// from the catalog the primary set is derived from.
pub fn declared_inferences_for_workspace(workspace: &HeadlessWorkspace) -> Result<Vec<DeclaredInference>, GatewayError> {
    let descriptors = workspace.discovery_descriptors()?;
    if descriptors.is_empty() {
        return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "no plugin-owned capability is registered in this workspace's catalog — nothing to read a declared inference roster from").retryable());
    }
    let contributed = contributed_inference_descriptors(&descriptors, installed_descriptors_for_contributions(workspace)?);
    let mut roster = Vec::new();
    for descriptor in descriptors.iter().chain(contributed.iter()) {
        roster.extend(declared_inferences_from_descriptor(descriptor));
    }
    Ok(roster)
}

/// 💡️ Resolves `artifact_id`'s real schema by delegating to the SAME `semio://artifact/{id}/schema`
/// answer `HeadlessWorkspace::read_artifact_resource` already implements, rather than re-deriving
/// "is this an open probe" a second time: a real schema for an open probe artifact (`PROBE_SCHEMA`),
/// the same typed, retryable `PLUGIN_UNAVAILABLE` for any other id — including one this workspace has
/// never seen at all — since the real wire protocol has no schema/describe query command yet to tell
/// the two apart (`🏠️workspace`'s own `/schema` arm never answers `NOT_FOUND` either, only `Ok` or
/// this one retryable gap).
fn resolve_artifact_schema(workspace: &HeadlessWorkspace, artifact_id: &str) -> Result<String, GatewayError> {
    let contents = workspace.read_resource(&format!("semio://artifact/{artifact_id}/schema"))?;
    let body: serde_json::Value = contents.first().and_then(|content| content.text.as_deref()).and_then(|text| serde_json::from_str(text).ok()).ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, format!("`{artifact_id}` schema resource returned no decodable body")))?;
    body.get("schema").and_then(serde_json::Value::as_str).map(str::to_string).ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, format!("`{artifact_id}` schema resource body carries no `schema` field")))
}

/// 💡️ Discovery for one artifact id: `PROBE_SCHEMA` (this crate's own synthetic probe schema, real
/// for any workspace-opened probe artifact) genuinely has zero declared inferences — no plugin
/// depends on it — so that case answers `[]` honestly rather than propagating a gap. Any other
/// document is matched against the workspace-wide roster by its artifact kind (the owning app's
/// dialect coordinate, [`document_artifact_kind`]) — the one key [`select_inference_service`] runs a
/// request on, so what a list names is exactly what a submit can start.
pub fn declared_inferences_for_artifact(workspace: &HeadlessWorkspace, artifact_id: &str) -> Result<(String, Vec<DeclaredInference>), GatewayError> {
    if resolve_artifact_schema(workspace, artifact_id)? == PROBE_SCHEMA {
        return Ok((PROBE_SCHEMA.to_string(), Vec::new()));
    }
    let artifact_kind = document_artifact_kind(workspace, artifact_id)?;
    let matches = declared_inferences_for_workspace(workspace)?.into_iter().filter(|item| item.artifact_kind == artifact_kind).collect();
    Ok((artifact_kind, matches))
}

//#endregion 🔖️DeclaredInference

//#region 🔖️ExecutionSeam

/// 💡️ Outcome of matching one `inferenceSchema` against a `declared_inferences_for_artifact` roster
/// — the fully-bound tier's seam. `Execute`'s only caller-visible outcome today is
/// `execution_not_wired_error` (no `artifact-infer` route exists on `crate::actions::ArtifactChannel`
/// anywhere in this crate — see this file's own module doc); wiring a real channel command later
/// only needs to replace THAT one call site, not this lookup.
pub enum InferenceLookup {
    NoSuchService,
    Execute(DeclaredInference),
}

/// 💡️ Pure lookup — exercised directly by this file's own tests independently of whether any live
/// tool call can reach the `Execute` arm today.
pub fn lookup_inference(declared: &[DeclaredInference], inference_schema: &str) -> InferenceLookup {
    match declared.iter().find(|item| item.inference_schema == inference_schema) {
        Some(item) => InferenceLookup::Execute(item.clone()),
        None => InferenceLookup::NoSuchService,
    }
}

/// 💡️ Why a bare READ of a declared inference still cannot answer a value: `inference_get` and the
/// `semio://artifact/{id}/inference/{field}` resource carry no canonical request body, and an
/// inference is a computation over one — the guest has nothing to run against. Execution itself is
/// no longer missing: `inference_run` (`AppCommand::Infer` →
/// `semio_framework_plugin_host::ArtifactInferenceRouter`) runs any declared service for real. This
/// stays retryable because a later packet that caches a document-derived canonical payload per
/// artifact turns this exact read into a hit with no change to the discovery path above.
///
/// 🔗️ An artifact-bound inference HAS a request body for this artifact — the gateway can
/// build it — but building it means running the guest, which a READ must not do on its
/// own budget. So the refusal is now an instruction with everything in it.
///
/// 📜️ The published contract travels WITH the refusal, so a client that asked for a value
/// and got a gap learns in the same round trip what to call and what to send — instead of
/// having to find `inference_list` and correlate the row itself.
pub(crate) fn execution_not_wired_error(item: &DeclaredInference) -> GatewayError {
    GatewayError::new(
        GatewayErrorCode::PluginUnavailable,
        match item.payload.as_ref().and_then(|contract| contract.artifact_binding.as_ref()) {
            Some(binding) => format!(
                "`{}/{}` is declared, executable and artifact-bound: run it with `inference_run {{ artifactKind, inferenceSchema, artifactId }}` — this gateway binds the artifact's document into `payload.{}` for you. A READ does not spend guest time on its own",
                item.artifact_kind, item.inference_schema, binding.field
            ),
            None => format!("`{}/{}` is declared and executable, but a bare read carries no canonical request payload to run it against — call `inference_run` with `payload` instead", item.artifact_kind, item.inference_schema),
        },
    )
    .with_details(serde_json::json!({
        "artifactKind": item.artifact_kind,
        "inferenceSchema": item.inference_schema,
        "owner": item.owner,
        "runWith": "inference_run",
        "pluginId": item.route_plugin_id(),
        "payload": item.payload,
    }))
    .retryable()
}

fn no_such_service_error(schema: &str, inference_schema: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::NotFound, format!("no inference service `{inference_schema}` is declared for artifact kind `{schema}`")).with_details(serde_json::json!({ "artifactKind": schema, "inferenceSchema": inference_schema }))
}

//#endregion 🔖️ExecutionSeam

//#region 🔖️JobSeam

/// 💡️ The `job_` handle payload an expensive inference execution would mint via
/// `crate::handles::HandleTable::mint(HandleKind::Job, ..)` once a real `artifact-infer` route
/// exists — `job_get`/`job_cancel` (`🦀️.rs`'s own `DECLARED_STUB_TOOL_NAMES`, still
/// unimplemented) resolve/`mark_terminal` that SAME shared `HandleTable` by the id `mint` returns.
/// This facet never mints one itself: no execution route exists to mint FOR yet, and an inert job
/// handle nobody could ever progress would be its own kind of fabrication. Pinning this shape now
/// means wiring the two sides together later is additive, not a redesign.
#[derive(Clone, Debug, PartialEq, serde::Serialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InferenceJobPayload {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub inference_schema: String,
    pub cancellation_id: String,
}

pub fn inference_job_payload(artifact_id: &str, item: &DeclaredInference, cancellation_id: &str) -> InferenceJobPayload {
    InferenceJobPayload { artifact_id: artifact_id.to_string(), artifact_kind: item.artifact_kind.clone(), inference_schema: item.inference_schema.clone(), cancellation_id: cancellation_id.to_string() }
}

//#endregion 🔖️JobSeam

//#region 🔖️Tools

fn workspace_binding_required(what: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{what}` needs a live workspace — bind one with `--folder <path>` or `--hub <url> --space <id>`")).with_details(serde_json::json!({ "bindWith": ["--folder", "--hub"] })).retryable()
}

fn inference_list_handler(workspace: Option<&Arc<HeadlessWorkspace>>, arguments: serde_json::Value) -> CallToolResult {
    let Some(workspace) = workspace else {
        return CallToolResult::tool_error(&workspace_binding_required("inference_list"));
    };
    match arguments.get("artifactId").and_then(serde_json::Value::as_str) {
        Some(artifact_id) => match declared_inferences_for_artifact(workspace, artifact_id) {
            Ok((schema, declared)) => {
                let count = declared.len();
                let structured = serde_json::json!({ "artifactId": artifact_id, "artifactKind": schema, "declared": declared });
                CallToolResult::ok(vec![ContentBlock::Text { text: format!("{count} declared inference(s) for {artifact_id}") }], Some(structured))
            }
            Err(error) => CallToolResult::tool_error(&error),
        },
        None => match declared_inferences_for_workspace(workspace) {
            Ok(declared) => {
                let count = declared.len();
                let structured = serde_json::json!({ "declared": declared });
                CallToolResult::ok(vec![ContentBlock::Text { text: format!("{count} declared inference(s) in this workspace") }], Some(structured))
            }
            Err(error) => CallToolResult::tool_error(&error),
        },
    }
}

fn inference_get_handler(workspace: Option<&Arc<HeadlessWorkspace>>, arguments: serde_json::Value) -> CallToolResult {
    let Some(workspace) = workspace else {
        return CallToolResult::tool_error(&workspace_binding_required("inference_get"));
    };
    let Some(artifact_id) = arguments.get("artifactId").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "artifactId is required"));
    };
    let Some(inference_schema) = arguments.get("inferenceSchema").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "inferenceSchema is required"));
    };
    if let Some(outcome) = hub_inference_read(workspace, artifact_id, inference_schema) {
        return match outcome {
            Ok(body) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("hub inference job for `{inference_schema}` on {artifact_id}") }], Some(body)),
            Err(error) => CallToolResult::tool_error(&error),
        };
    }
    match declared_inferences_for_artifact(workspace, artifact_id) {
        Err(error) => CallToolResult::tool_error(&error),
        Ok((schema, declared)) => match lookup_inference(&declared, inference_schema) {
            InferenceLookup::NoSuchService => CallToolResult::tool_error(&no_such_service_error(&schema, inference_schema)),
            InferenceLookup::Execute(item) => CallToolResult::tool_error(&execution_not_wired_error(&item)),
        },
    }
}

/// 💡️ Registers the inference tool(s) — always present in `tools/list` regardless of tier; only
/// their RESULT varies by whether `workspace` is bound (`🦀️.rs`'s own
/// `DECLARED_STUB_TOOL_NAMES` convention: presence never depends on tier).
pub fn register_inference_tools(registry: &mut InMemoryToolRegistry, workspace: Option<Arc<HeadlessWorkspace>>) {
    let list_capability = inference_list_capability();
    let list_tool = tool_from_capability(&list_capability, "inference_list");
    let list_workspace = workspace.clone();
    registry.register(list_tool, move |arguments| inference_list_handler(list_workspace.as_ref(), arguments)).expect("inference_list is a valid tool name");

    let get_capability = inference_get_capability();
    let get_tool = tool_from_capability(&get_capability, "inference_get");
    let get_workspace = workspace.clone();
    registry.register(get_tool, move |arguments| inference_get_handler(get_workspace.as_ref(), arguments)).expect("inference_get is a valid tool name");
}

//#endregion 🔖️Tools

//#region 🔖️Resources

/// 💡️ The `semio://artifact/{id}/inference…` resources this facet answers; `None` when `uri` is not
/// one of ours (so the composing registry falls through to the next facet), `Some(Err(NOT_FOUND))`
/// for a URI that IS clearly ours but malformed (empty artifact id, empty/nested field segment).
pub fn read_inference_resource(uri: &str, workspace: Option<&Arc<HeadlessWorkspace>>) -> Option<Result<Vec<ResourceContent>, GatewayError>> {
    let rest = uri.strip_prefix("semio://artifact/")?;
    let (artifact_id, sub) = rest.split_once('/')?;
    if sub != "inference" && !sub.starts_with("inference/") {
        return None;
    }
    if artifact_id.is_empty() {
        return Some(Err(GatewayError::new(GatewayErrorCode::NotFound, format!("malformed inference resource uri: {uri}"))));
    }
    let field = match sub.strip_prefix("inference/") {
        None => None,
        Some(field) if !field.is_empty() && !field.contains('/') => Some(field),
        Some(_) => return Some(Err(GatewayError::new(GatewayErrorCode::NotFound, format!("malformed inference resource uri: {uri}")))),
    };
    let Some(workspace) = workspace else {
        return Some(Err(workspace_binding_required(uri)));
    };
    if let Some(outcome) = field.and_then(|field| hub_inference_read(workspace, artifact_id, field)) {
        return Some(outcome.map(|body| vec![ResourceContent { uri: uri.to_string(), mime_type: Some("application/json".to_string()), text: Some(body.to_string()), blob: None }]));
    }
    Some(match declared_inferences_for_artifact(workspace, artifact_id) {
        Err(error) => Err(error),
        Ok((schema, declared)) => match field {
            None => {
                let body = serde_json::json!({ "artifactId": artifact_id, "artifactKind": schema, "declared": declared });
                Ok(vec![ResourceContent { uri: uri.to_string(), mime_type: Some("application/json".to_string()), text: Some(body.to_string()), blob: None }])
            }
            Some(field) => match lookup_inference(&declared, field) {
                InferenceLookup::NoSuchService => Err(no_such_service_error(&schema, field)),
                InferenceLookup::Execute(item) => Err(execution_not_wired_error(&item)),
            },
        },
    })
}

/// 💡️ The inference resource entries to advertise in `resources/list` — one `.../inference` index
/// per artifact id this workspace currently knows about (bare tier: empty, matching
/// `WorkspaceResourceRegistry`'s own convention of only listing real, known ids).
pub fn inference_resources(workspace: Option<&Arc<HeadlessWorkspace>>) -> Vec<Resource> {
    let Some(workspace) = workspace else {
        return Vec::new();
    };
    let Ok(ids) = workspace.workspace_artifact_ids() else {
        return Vec::new();
    };
    ids.into_iter()
        .map(|artifact_id| Resource {
            uri: format!("semio://artifact/{artifact_id}/inference"),
            name: format!("{artifact_id}-inference"),
            title: Some(format!("Declared inferences for {artifact_id}")),
            description: Some("Every inference service declared for this artifact's kind, and whether it can be read yet".to_string()),
            mime_type: Some("application/json".to_string()),
            size: None,
        })
        .collect()
}

//#endregion 🔖️Resources

//#region 💡️InferenceJobTools

/// 🛑️ Interrupts the retained local wait for one operation label; `false` when nothing was waiting.
pub fn interrupt_inference_operation(label: &str) -> bool {
    let registry = inflight_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut interrupted = false;
    for (existing, token) in registry.iter() {
        if existing.as_str() == label {
            token.cancel_now();
            interrupted = true;
        }
    }
    interrupted
}

fn inference_wall_now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

/// 🎲️ A fresh 32-lower-hex client idempotency key. The hub mints the `jobId`; this is only the
/// client's own scoped `(user, authorization generation, space, document, request_id)` key.
pub fn mint_inference_request_id() -> String {
    let entropy = format!("semio.mcp.inference-request/v1\0{}\0{}", inference_wall_now_ms(), mint_id(HandleKind::Job, inference_wall_now_ms()));
    framework_hash::hash_bytes(entropy.as_bytes())[..INFERENCE_REQUEST_ID_HEX_LENGTH].to_string()
}

//#region 🔖️InferenceServiceResolution

/// 🗂️ The artifact kind one readable document IS — the owning app's dialect coordinate when the
/// workspace knows it, else the schema a folder document shares with its kind.
fn document_artifact_kind(workspace: &HeadlessWorkspace, document_id: &str) -> Result<String, GatewayError> {
    let contents = workspace.read_resource(&format!("semio://artifact/{document_id}/schema"))?;
    let body: serde_json::Value = contents.first().and_then(|content| content.text.as_deref()).and_then(|text| serde_json::from_str(text).ok()).ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, format!("`{document_id}` schema resource returned no decodable body")))?;
    body.get("artifactKind").or_else(|| body.get("schema")).and_then(serde_json::Value::as_str).map(str::to_string).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("document `{document_id}` names no artifact kind")))
}

/// 🧭️ The one service a request names on an open document, resolved against the workspace's
/// declared roster and the services its hub (if any) executes itself.
pub fn resolve_document_inference_service(workspace: &HeadlessWorkspace, document_id: &str, inference_schema: Option<&str>, plugin_id: Option<&str>) -> Result<InferenceService, GatewayError> {
    let artifact_kind = document_artifact_kind(workspace, document_id)?;
    select_inference_service(&declared_inferences_for_workspace(workspace)?, &workspace.hub_inference_services()?, &artifact_kind, inference_schema, plugin_id)
}

//#endregion 🔖️InferenceServiceResolution

fn inference_input_invalid(detail: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::InputInvalid, detail.to_string())
}

/// 🔐️ The one local admission gate: the connection's granted MCP scopes must cover the
/// capability's declared scopes. A hub-executed job is re-authorized by the hub on every route; a
/// guest job's commit is authorized again by the commit action's own policy.
fn authorize_inference(policy: &PolicyEngine, principal: &AgentPrincipal, capability: &CapabilityDefinition) -> Result<(), GatewayError> {
    policy.authorize_scopes(principal, capability)
}

struct InferenceToolContext<'a> {
    pub workspace: Option<&'a Arc<HeadlessWorkspace>>,
    pub actions: &'a Arc<ActionAdapter>,
    pub catalog: &'a Arc<Catalog>,
    pub principal: &'a AgentPrincipal,
    session: SessionHandle,
}

impl InferenceToolContext<'_> {
    pub fn policy(&self) -> &PolicyEngine {
        self.actions.policy()
    }

    pub fn handles(&self) -> &HandleTable {
        self.actions.handles().as_ref()
    }

    pub fn workspace(&self, what: &str) -> Result<&Arc<HeadlessWorkspace>, GatewayError> {
        self.workspace.ok_or_else(|| workspace_binding_required(what))
    }

    /// 🎫️ Resolves one `job_` handle against this connection's own session; a hub job's live subject
    /// must still be the exact subject and authority generation that minted it.
    pub(crate) fn job(&self, workspace: &HeadlessWorkspace, handle: &str) -> Result<InferenceJobHandlePayloadV1, GatewayError> {
        let payload = resolve_inference_job_handle(self.handles(), &self.session, handle, inference_wall_now_ms())?;
        if payload.hub.is_some() {
            check_hub_job_subject(&payload, &workspace.hub_inference_subject()?)?;
        }
        Ok(payload)
    }
}

/// 🎫️ One `job_` handle of THIS session, decoded to the inference job it names.
pub fn resolve_inference_job_handle(handles: &HandleTable, session: &SessionHandle, handle: &str, now_ms: u64) -> Result<InferenceJobHandlePayloadV1, GatewayError> {
    let record = handles.resolve(handle, session, now_ms)?;
    if record.kind != HandleKind::Job {
        return Err(inference_input_invalid("handle is not an inference job handle"));
    }
    serde_json::from_value(record.payload).map_err(|_| GatewayError::new(GatewayErrorCode::InputInvalid, "this job handle does not name an inference job"))
}

/// 👤️ A hub job is owner-private locally as well as on the hub: the live subject must be the one
/// that minted it, under the same authority generation, in the same space.
pub fn check_hub_job_subject(payload: &InferenceJobHandlePayloadV1, subject: &HubInferenceSubjectV1) -> Result<(), GatewayError> {
    let Some(hub) = payload.hub.as_ref() else { return Ok(()) };
    if hub.subject_user_id != subject.user_id || hub.authority_generation != subject.authority_generation || hub.space_id != subject.space_id {
        return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "this inference job belongs to a different authenticated subject or authority generation"));
    }
    Ok(())
}

fn job_handle_argument(arguments: &serde_json::Value) -> Result<&str, GatewayError> {
    arguments.get("jobHandle").and_then(serde_json::Value::as_str).ok_or_else(|| inference_input_invalid("jobHandle is required"))
}

fn job_reply(text: String, structured: serde_json::Value) -> CallToolResult {
    CallToolResult::ok(vec![ContentBlock::Text { text }], Some(structured))
}

fn page_value(page: &InferenceJobPageV1) -> serde_json::Value {
    serde_json::to_value(page).unwrap_or(serde_json::Value::Null)
}

fn page_text(page: &InferenceJobPageV1) -> String {
    format!("{} job {} ({}) is {:?} / {:?}, {} event(s), {} progress row(s), next cursor {}", page.service_id, page.job_id, if page.site == InferenceExecutionSiteV1::Guest { "guest" } else { "hub" }, page.state, page.proposal_state, page.events.len(), page.progress.len(), page.next_cursor)
}

/// 📃️ The current page of one guest job, after `after`.
fn guest_job_page(job_id: &str, after: u64) -> Result<InferenceJobPageV1, GatewayError> {
    let registry = job_registry();
    let (Some(snapshot), Some(journal), Some(job)) = (registry.snapshot(job_id), registry.events(job_id, after, INFERENCE_JOB_EVENT_PAGE_MAX_ITEMS), guest_job(job_id)) else {
        return Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no inference job {job_id} in this process")));
    };
    Ok(guest_page(job_id, &snapshot, &journal.events, journal.next_cursor, &job))
}

fn hub_scope_route(payload: &InferenceJobHandlePayloadV1) -> Result<&HubInferenceJobBindingV1, GatewayError> {
    payload.hub.as_ref().ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, "a hub-executed job handle carries no hub binding"))
}

fn inference_submit_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let submitted = (|| {
        authorize_inference(context.policy(), context.principal, &inference_submit_capability())?;
        let document_id = arguments.get("documentId").and_then(serde_json::Value::as_str).ok_or_else(|| inference_input_invalid("documentId is required"))?;
        let workspace = context.workspace("inference_submit")?;
        let service = resolve_document_inference_service(workspace, document_id, arguments.get("inferenceSchema").and_then(serde_json::Value::as_str), arguments.get("pluginId").and_then(serde_json::Value::as_str))?;
        match service.site {
            InferenceExecutionSiteV1::Guest => submit_guest_job(context, workspace, service, document_id, &arguments),
            InferenceExecutionSiteV1::Hub => submit_hub_job(context, workspace, service, document_id, &arguments),
        }
    })();
    match submitted {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// 🧵️ Starts one guest job: the contract is checked on the caller, the job is minted and handed a
/// session-owned handle, and the plugin's guest runs on a job thread of its own so the call answers
/// at once. Every later fact — progress, result, proposal, cancel — lands in the job's journal.
fn submit_guest_job(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, service: InferenceService, document_id: &str, arguments: &serde_json::Value) -> Result<(String, serde_json::Value), GatewayError> {
    inference_original_turn(arguments)?;
    validate_inference_request(&service.declared, arguments.get("payload").filter(|value| !value.is_null()), Some(document_id))?;
    let jobs = job_registry();
    let job_id = jobs.begin("inference.submit");
    let cancel = InferenceCancel::default();
    let hook = cancel.0.clone();
    jobs.bind_cancel(&job_id, move || hook.cancel_now());
    retain_guest_job(&job_id, GuestInferenceJob { service: service.clone(), document_id: document_id.to_string(), proposal: None, committed: None, commit_refused: false });
    let payload = InferenceJobHandlePayloadV1 { site: InferenceExecutionSiteV1::Guest, service_id: service.service_id().to_string(), artifact_kind: service.declared.artifact_kind.clone(), plugin_id: service.plugin_id().to_string(), document_id: document_id.to_string(), job_id: job_id.clone(), hub: None };
    let handle = context.handles().mint(HandleKind::Job, context.session.clone(), Attachment::Artifact { artifact_id: document_id.to_string() }, serde_json::to_value(&payload).unwrap_or(serde_json::Value::Null), inference_wall_now_ms());
    spawn_guest_job(Arc::clone(workspace), Arc::clone(context.actions), Arc::clone(context.catalog), service, document_id.to_string(), arguments.clone(), job_id.clone(), cancel)?;
    let page = guest_job_page(&job_id, 0)?;
    Ok((page_text(&page), serde_json::json!({ "jobHandle": handle, "sessionId": context.session.0, "job": page_value(&page) })))
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_guest_job(workspace: Arc<HeadlessWorkspace>, actions: Arc<ActionAdapter>, catalog: Arc<Catalog>, service: InferenceService, document_id: String, arguments: serde_json::Value, job_id: String, cancel: InferenceCancel) -> Result<(), GatewayError> {
    let thread_job = job_id.clone();
    std::thread::Builder::new()
        .name(format!("semio-inference-{job_id}"))
        .spawn(move || run_guest_job(&workspace, &actions, &catalog, &service, &document_id, &arguments, &thread_job, cancel))
        .map(|_| ())
        .map_err(|error| {
            let error = GatewayError::new(GatewayErrorCode::Internal, format!("could not start the inference job thread: {error}"));
            job_registry().fail(&job_id, error.clone());
            error
        })
}

#[cfg(target_arch = "wasm32")]
fn spawn_guest_job(_workspace: Arc<HeadlessWorkspace>, _actions: Arc<ActionAdapter>, _catalog: Arc<Catalog>, _service: InferenceService, _document_id: String, _arguments: serde_json::Value, job_id: String, _cancel: InferenceCancel) -> Result<(), GatewayError> {
    let error = GatewayError::new(GatewayErrorCode::PluginUnavailable, "a guest inference job needs a native gateway with job threads");
    job_registry().fail(&job_id, error.clone());
    Err(error)
}

/// 🏃️ The job thread: run the guest, then either finish with its result, or — for a service that
/// declares a commit action — offer the proposal that action would commit and park for approval.
fn run_guest_job(workspace: &Arc<HeadlessWorkspace>, actions: &ActionAdapter, catalog: &Catalog, service: &InferenceService, document_id: &str, arguments: &serde_json::Value, job_id: &str, cancel: InferenceCancel) {
    let jobs = job_registry();
    let run = match run_guest_inference(workspace, actions, &service.declared, Some(document_id), arguments, job_id, cancel) {
        Ok(Some(run)) if !jobs.is_cancel_requested(job_id) => run,
        Ok(_) => {
            jobs.mark_cancelled(job_id);
            return;
        }
        Err(_) if jobs.is_cancel_requested(job_id) => {
            jobs.mark_cancelled(job_id);
            return;
        }
        Err(error) => {
            jobs.fail(job_id, error);
            return;
        }
    };
    let result = run.payload.as_deref().and_then(inference_run_result_value).unwrap_or(serde_json::Value::Null);
    let Some(action) = service.commit_action.as_deref() else {
        jobs.succeed(job_id, result);
        return;
    };
    let Some(capability) = commit_capability(catalog, &service.declared.artifact_kind, service.plugin_id(), action) else {
        jobs.fail(job_id, GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{}` declares the commit action `{action}`, but `{}` publishes no such action on `{}` in this catalog", service.service_id(), service.plugin_id(), service.declared.artifact_kind)));
        return;
    };
    match guest_proposal(service.service_id(), document_id, action, capability.id.as_str(), &capability_argument_ids(capability), &result) {
        Ok(proposal) => {
            update_guest_job(job_id, |job| job.proposal = Some(proposal.clone()));
            jobs.await_approval(job_id, serde_json::to_value(&proposal).unwrap_or(serde_json::Value::Null));
        }
        Err(error) => {
            jobs.fail(job_id, error);
        }
    }
}

/// 🌎️ Starts one hub-executed job through the route family the hub published for the service.
fn submit_hub_job(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, service: InferenceService, document_id: &str, arguments: &serde_json::Value) -> Result<(String, serde_json::Value), GatewayError> {
    let route = service.route.clone().ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, "a hub-executed service resolved without its route"))?;
    let lifetime_ms = arguments.get("lifetimeMs").and_then(serde_json::Value::as_u64).unwrap_or(INFERENCE_JOB_MAX_LIFETIME_MS);
    let request_id = arguments.get("requestId").and_then(serde_json::Value::as_str).map_or_else(mint_inference_request_id, str::to_string);
    let subject = workspace.hub_inference_subject()?;
    let request = HubInferenceSubmitRequestV1::new(service.service_id(), request_id.clone(), lifetime_ms);
    request.validate().map_err(|error| error.to_gateway_error("inference_submit"))?;
    let (base, base_diagnostic) = match workspace.hub_inference_base(document_id) {
        Ok(base) => (Some(base), None),
        Err(error) => (None, Some(error.message.clone())),
    };
    let receipt = workspace.submit_hub_inference_job(document_id, &route, &request)?;
    let payload = InferenceJobHandlePayloadV1 {
        site: InferenceExecutionSiteV1::Hub,
        service_id: service.service_id().to_string(),
        artifact_kind: service.declared.artifact_kind.clone(),
        plugin_id: service.plugin_id().to_string(),
        document_id: document_id.to_string(),
        job_id: receipt.job_id.clone(),
        hub: Some(HubInferenceJobBindingV1 { route, space_id: subject.space_id.clone(), subject_user_id: subject.user_id.clone(), authority_generation: subject.authority_generation, request_id, base: base.clone() }),
    };
    let handle = context.handles().mint(HandleKind::Job, context.session.clone(), Attachment::Artifact { artifact_id: document_id.to_string() }, serde_json::to_value(&payload).unwrap_or(serde_json::Value::Null), inference_wall_now_ms());
    let page = hub_receipt_page(&service, document_id, &receipt);
    Ok((page_text(&page), serde_json::json!({ "jobHandle": handle, "sessionId": context.session.0, "job": page_value(&page), "baseBinding": base, "baseBindingUnavailable": base_diagnostic })))
}

/// 🌎️ The minimal service identity a hub job's handle carries, enough to project its pages.
fn handle_service(payload: &InferenceJobHandlePayloadV1) -> InferenceService {
    InferenceService {
        declared: DeclaredInference {
            owner: payload.plugin_id.clone(),
            artifact_kind: payload.artifact_kind.clone(),
            artifact_schema: payload.artifact_kind.clone(),
            artifact_schema_version: 1,
            inference_schema: payload.service_id.clone(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            contributor: payload.plugin_id.clone(),
            depends_on: Vec::new(),
            payload: None,
        },
        site: payload.site,
        route: payload.hub.as_ref().map(|hub| hub.route.clone()),
        commit_action: None,
    }
}

fn inference_events_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let read = (|| {
        authorize_inference(context.policy(), context.principal, &inference_events_capability())?;
        let handle = job_handle_argument(&arguments)?;
        let workspace = context.workspace("inference_events")?;
        let payload = context.job(workspace, handle)?;
        let after = arguments.get("after").and_then(serde_json::Value::as_u64).unwrap_or(0);
        let page = match payload.site {
            InferenceExecutionSiteV1::Guest => guest_job_page(&payload.job_id, after)?,
            InferenceExecutionSiteV1::Hub => {
                let hub = hub_scope_route(&payload)?;
                let page = workspace.read_hub_inference_job_events(&payload.document_id, &hub.route, &payload.job_id, after)?;
                hub_events_page(&handle_service(&payload), &payload.document_id, &page)
            }
        };
        Ok((page_text(&page), serde_json::json!({ "jobHandle": handle, "job": page_value(&page), "baseBinding": payload.hub.as_ref().and_then(|hub| hub.base.clone()) })))
    })();
    match read {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn inference_cancel_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let cancelled = (|| {
        authorize_inference(context.policy(), context.principal, &inference_cancel_capability())?;
        let handle = job_handle_argument(&arguments)?;
        let workspace = context.workspace("inference_cancel")?;
        let payload = context.job(workspace, handle)?;
        let (page, interrupted) = match payload.site {
            InferenceExecutionSiteV1::Guest => {
                job_registry().request_cancel(&payload.job_id)?;
                (guest_job_page(&payload.job_id, 0)?, true)
            }
            InferenceExecutionSiteV1::Hub => {
                let hub = hub_scope_route(&payload)?;
                let interrupted = interrupt_inference_operation(&inference_operation_label(&hub.space_id, &payload.document_id, Some(&payload.job_id))) | interrupt_inference_operation(&inference_operation_label(&hub.space_id, &payload.document_id, None));
                let page = workspace.cancel_hub_inference_job(&payload.document_id, &hub.route, &payload.job_id)?;
                (hub_events_page(&handle_service(&payload), &payload.document_id, &page), interrupted)
            }
        };
        Ok((format!("cancel requested — {}", page_text(&page)), serde_json::json!({ "jobHandle": handle, "job": page_value(&page), "localWaitInterrupted": interrupted })))
    })();
    match cancelled {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn inference_approve_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let approved = (|| {
        authorize_inference(context.policy(), context.principal, &inference_approve_capability())?;
        let handle = job_handle_argument(&arguments)?;
        let proposal_hash = arguments.get("proposalHash").and_then(serde_json::Value::as_str).ok_or_else(|| inference_input_invalid("proposalHash is required"))?;
        let workspace = context.workspace("inference_approve")?;
        let payload = context.job(workspace, handle)?;
        match payload.site {
            InferenceExecutionSiteV1::Guest => approve_guest_job(context, workspace, handle, &payload, proposal_hash),
            InferenceExecutionSiteV1::Hub => approve_hub_job(context, workspace, handle, &payload, proposal_hash),
        }
    })();
    match approved {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// ✅️ Commits one guest proposal: the approval is consumed exactly once, the document is bound as
/// this session's document for the plugin, and the declared commit action is prepared and invoked
/// on it through `ActionAdapter` — the normal edit path, whose relay carries the edit to the hub's
/// document ledger for a hub-bound document.
fn approve_guest_job(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, handle: &str, payload: &InferenceJobHandlePayloadV1, proposal_hash: &str) -> Result<(String, serde_json::Value), GatewayError> {
    let job = guest_job(&payload.job_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no inference job {} in this process", payload.job_id)))?;
    let proposal = job.proposal.clone().ok_or_else(|| GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("job {} has offered no proposal", payload.job_id)))?;
    if proposal.hash != proposal_hash {
        return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, "proposalHash does not name this job's offered proposal").with_details(serde_json::json!({ "field": "proposalHash" })));
    }
    let jobs = job_registry();
    if !jobs.resume_approved(&payload.job_id) {
        return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("job {} has no proposal awaiting approval — it was already approved, withdrawn or cancelled", payload.job_id)));
    }
    match commit_guest_proposal(context, workspace, &payload.document_id, &proposal) {
        Ok(report) => {
            let committed = serde_json::to_value(&report).unwrap_or(serde_json::Value::Null);
            update_guest_job(&payload.job_id, |job| job.committed = Some(committed.clone()));
            jobs.record_event(&payload.job_id, "committed", Some(report.invocation_id.clone()));
            jobs.succeed(&payload.job_id, serde_json::json!({ "proposal": proposal, "commit": committed }));
        }
        Err(error) => {
            update_guest_job(&payload.job_id, |job| job.commit_refused = true);
            jobs.fail(&payload.job_id, error.clone());
            return Err(error);
        }
    }
    let page = guest_job_page(&payload.job_id, 0)?;
    let undo_token = page.commit.as_ref().and_then(|commit| commit.get("undoToken")).cloned();
    Ok((format!("approved — {}", page_text(&page)), serde_json::json!({ "jobHandle": handle, "job": page_value(&page), "undoToken": undo_token })))
}

fn commit_guest_proposal(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, document_id: &str, proposal: &InferenceProposalV1) -> Result<InvocationReport, GatewayError> {
    let (Some(capability_id), Some(input)) = (proposal.capability_id.as_deref(), proposal.input.clone()) else {
        return Err(GatewayError::new(GatewayErrorCode::Internal, "an offered guest proposal carries no commit capability"));
    };
    if let Some((pack, spr)) = workspace.read_artifact_bytes(document_id)? {
        workspace.bind_hub_session_document(document_id, &pack, &spr)?;
    }
    let instance = commit_instance_slot(context.catalog, capability_id);
    let prepared = context.actions.prepare(context.catalog, context.principal, &context.session, capability_id, input, instance, inference_wall_now_ms())?;
    if prepared.expected_revision.as_ref().is_some_and(|revision| revision.artifact_id != document_id) {
        let _ = context.actions.cancel(&context.session, &prepared.prepared_handle, inference_wall_now_ms());
        return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("this session addresses another document of `{capability_id}`'s plugin, so the commit cannot land on `{document_id}` unambiguously")));
    }
    let request = InvokeRequest { prepared_handle: Some(prepared.prepared_handle), capability_id: None, input: None, expected_revision: prepared.expected_revision, idempotency_key: Some(format!("inference-{}", proposal.hash)), approval_handle: None };
    let report = context.actions.invoke(context.catalog, context.principal, &context.session, request, instance, inference_wall_now_ms())?;
    if report.status != InvocationStatus::Succeeded {
        return Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("the commit action `{capability_id}` answered {:?}", report.status)));
    }
    Ok(report)
}

#[cfg(not(target_arch = "wasm32"))]
fn commit_instance_slot(catalog: &Catalog, capability_id: &str) -> u32 {
    capability_instance_slot(catalog, capability_id).unwrap_or(0)
}

#[cfg(target_arch = "wasm32")]
fn commit_instance_slot(_catalog: &Catalog, _capability_id: &str) -> u32 {
    0
}

/// ✅️ Relays one approval to the hub that executed the job; the hub rebuilds and commits the typed
/// effect server-side and hands back a durable undo target, retained as this session's undo token.
/// Every precondition is checked BEFORE the relay; once the hub's receipt exists the edit is a fact,
/// so a follow-up that cannot complete (undo retention, the events page) is a named `warnings` row
/// on a successful answer, never an error that would tell the agent to retry an approval that landed.
fn approve_hub_job(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, handle: &str, payload: &InferenceJobHandlePayloadV1, proposal_hash: &str) -> Result<(String, serde_json::Value), GatewayError> {
    let hub = hub_scope_route(payload)?;
    let request = HubInferenceApprovalRequestV1::new(payload.job_id.clone(), proposal_hash.to_string());
    request.validate().map_err(|error| error.to_gateway_error("inference_approve"))?;
    let base = hub.base.as_ref().ok_or_else(|| GatewayError::new(GatewayErrorCode::PreconditionFailed, "approved inference job has no retained canonical base binding"))?;
    let receipt = workspace.approve_hub_inference_job(&payload.document_id, &hub.route, &request)?;
    let scope = DocumentScope::new(hub.space_id.clone(), payload.document_id.clone());
    let mut warnings = Vec::new();
    let undo_token = context
        .actions
        .retain_hub_inference_approval_undo(&context.session, &base.hub_origin, &scope, &hub.route, &receipt.undo, inference_wall_now_ms())
        .map_err(|error| warnings.push(format!("undo-unavailable: {}", error.message)))
        .ok();
    let job = workspace
        .read_hub_inference_job_events(&payload.document_id, &hub.route, &payload.job_id, 0)
        .map(|events| page_value(&hub_events_page(&handle_service(payload), &payload.document_id, &events)))
        .map_err(|error| warnings.push(format!("events-unavailable: {}", error.message)))
        .ok();
    Ok((
        format!("approval of job {} produced mutation {} (applied: {})", receipt.job_id, receipt.mutation_id, receipt.applied),
        serde_json::json!({ "jobHandle": handle, "job": job, "undoToken": undo_token, "commit": serde_json::to_value(&receipt).unwrap_or(serde_json::Value::Null), "baseBinding": hub.base, "warnings": warnings }),
    ))
}

/// 📦️ The canonical request body one `inference_run` call carries into the guest. The gateway never
/// interprets it: whatever JSON the caller supplies is encoded as UTF-8 JSON and handed to the
/// plugin's own inference schema verbatim, because only that plugin's guest understands its own
/// canonical payload (the same host-opaque rule `🏠️workspace`'s module doc states for documents).
fn inference_run_payload_bytes(arguments: &serde_json::Value) -> Vec<u8> {
    match arguments.get("payload") {
        None | Some(serde_json::Value::Null) => b"{}".to_vec(),
        Some(value) => serde_json::to_vec(value).unwrap_or_else(|_| b"{}".to_vec()),
    }
}

/// 🔎️ The guest's result bytes, projected for the tool reply: a UTF-8 JSON body becomes structured
/// `payload`, anything else is reported by length alone rather than guessed at.
fn inference_run_result_value(payload: &[u8]) -> Option<serde_json::Value> {
    std::str::from_utf8(payload).ok().and_then(|text| serde_json::from_str(text).ok())
}

/// 📜️ The published contract check, BEFORE a single guest is touched. Three named refusals, each
/// carrying the exact field a client has to fix:
///
/// 1. an artifact-bound inference with neither `artifactId` nor a caller-authored payload field —
///    the case that used to spend a whole component compile and 240 s of guest time before the
///    guest answered `missing field 'snapshot'` (`📓️pz2-…md` §5.2, `📓️ce3-…md` §3.3);
/// 2. a caller payload missing a field the contract's own `input_schema` declares `required`;
/// 3. an inference that publishes NO contract at all AND was called with no payload — nothing in
///    the tree can tell the caller what to send, so saying so beats dispatching `{}`.
///
/// 🧾️ `input_schema` is the plugin's own JSON Schema TEXT. Only its top-level `required` list is
/// enforced here: that is what makes the difference between "answers in 200 ms naming the field"
/// and "hangs" — a full JSON Schema evaluator belongs to the guest that owns the schema.
pub(crate) fn validate_inference_request(item: &DeclaredInference, payload: Option<&serde_json::Value>, artifact_id: Option<&str>) -> Result<(), GatewayError> {
    let Some(contract) = item.payload.as_ref() else {
        if payload.is_none() {
            return Err(inference_field_invalid(
                "payload",
                format!("`{}/{}` publishes no payload contract, so this gateway cannot build its request body — send `payload` yourself, or have `{}` declare one on its inference metadata and be re-described", item.artifact_kind, item.inference_schema, item.route_plugin_id()),
            ));
        }
        return Ok(());
    };
    let binding = contract.artifact_binding.as_ref();
    if let Some(body) = payload.filter(|value| value.as_object().is_none_or(|object| !object.is_empty())) {
        let Some(object) = body.as_object() else {
            return Err(inference_field_invalid("payload", format!("`{}` takes a JSON object body, per `{}`", item.inference_schema, contract.payload_schema_id)));
        };
        for field in contract_required_fields(&contract.input_schema) {
            if object.contains_key(&field) {
                continue;
            }
            if binding.is_some_and(|binding| binding.field == field) && artifact_id.is_some() {
                continue;
            }
            return Err(inference_field_invalid(&field, format!("`{}` requires `payload.{}` (contract `{}`)", item.inference_schema, field, contract.payload_schema_id)));
        }
        return Ok(());
    }
    if let Some(binding) = binding {
        if binding.required && artifact_id.is_none() {
            return Err(inference_field_invalid(
                "artifactId",
                format!("`{}` runs ON an artifact: name it with `artifactId` and this gateway binds its document into `payload.{}` for you (contract `{}`)", item.inference_schema, binding.field, contract.payload_schema_id),
            ));
        }
        return Ok(());
    }
    for field in contract_required_fields(&contract.input_schema) {
        return Err(inference_field_invalid(&field, format!("`{}` requires `payload.{}` (contract `{}`)", item.inference_schema, field, contract.payload_schema_id)));
    }
    Ok(())
}

/// 🧾️ The top-level `required` names of a JSON Schema document carried as TEXT. A schema this
/// gateway cannot parse declares nothing here rather than refusing every call — the plugin's own
/// guest is still the authority on its body.
fn contract_required_fields(input_schema: &str) -> Vec<String> {
    serde_json::from_str::<serde_json::Value>(input_schema)
        .ok()
        .and_then(|schema| schema.get("required").and_then(serde_json::Value::as_array).cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|value| value.as_str().map(str::to_string))
        .collect()
}

/// 🏷️ The field a refusal already named, so folding the job identity onto its details never loses
/// it. `details` is a plain value here, not a map the two writers share, so the name is read back
/// rather than threaded through every construction site.
fn inference_error_field(error: &GatewayError) -> String {
    error.details.get("field").and_then(serde_json::Value::as_str).unwrap_or("artifactId").to_string()
}

/// 🚧️ `INPUT_INVALID` that NAMES the field — the difference between a client that can retry and one
/// that can only guess. `details.field` is machine-readable on purpose.
fn inference_field_invalid(field: &str, message: impl Into<String>) -> GatewayError {
    GatewayError::new(GatewayErrorCode::InputInvalid, message).with_details(serde_json::json!({ "field": field }))
}

/// 🔗️ The named artifact's canonical pair, for an inference whose contract binds one. `None` when
/// no artifact was named or the contract declares no binding; a NAMED artifact this workspace
/// cannot read is `NOT_FOUND`, never a silent empty document.
///
/// ⏳️ Binding is the job's own first expensive step — on the first call of a session it activates
/// the plugin guest the artifact lives in — so it runs as a job: each phase it actually runs is
/// pushed to the client as progress (`crate::actions::ActivationPhase`), and a cancel is answered as
/// soon as it lands. Laws: `🧫️fixtures/⏱️binding-cancellation-law.json` and
/// `🏠️workspace/🧫️fixtures/⏱️compile-cancellation-law.json`.
fn resolve_inference_artifact_document(workspace: &Arc<HeadlessWorkspace>, item: &DeclaredInference, artifact_id: Option<&str>, job_id: &str, cancel: &semio_framework_async::CancelToken) -> Result<Option<ArtifactDocumentBinding>, GatewayError> {
    let Some(artifact_id) = artifact_id else { return Ok(None) };
    if item.payload.as_ref().and_then(|contract| contract.artifact_binding.as_ref()).is_none() {
        return Err(inference_field_invalid("artifactId", format!("`{}` publishes no artifact binding, so naming `artifactId` cannot change what it computes — send `payload` instead", item.inference_schema)));
    }
    let job_id = job_id.to_string();
    let scope = ActivationScope::new(cancel.clone(), move |phase, fraction| {
        job_registry().report_progress(&job_id, INFERENCE_BINDING_PROGRESS_START + INFERENCE_BINDING_PROGRESS_SPAN * phase.at(fraction), Some(format!("binding: {}", phase.id())));
    });
    match workspace.bind_artifact_document_cancellably(artifact_id, scope)? {
        Some((pack, spr)) => Ok(Some(ArtifactDocumentBinding { pack, spr })),
        None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no readable document for artifact `{artifact_id}` — open or create it before running an inference on it"))),
    }
}

/// 🏁 One finished guest run: the guest's own result bytes and whether it reported completion.
struct GuestInferenceRun {
    pub inference_schema: String,
    pub complete: bool,
    pub payload: Option<Vec<u8>>,
}

/// 🎟️ All retained axes come from the original turn's explicit grant.
pub(crate) fn inference_original_retained(grant:&serde_json::Value)->Result<semio_framework_value::RetainedCloneGrant,GatewayError>{
    let invalid=||inference_field_invalid("turn.grant","all five explicit retained axes are required within the cross-platform address range");
    let fields=grant.as_object().ok_or_else(invalid)?;
    if fields.len()!=5{return Err(invalid())}
    let axis=|name|fields.get(name).and_then(serde_json::Value::as_u64).and_then(|number|u32::try_from(number).ok()).map(|number|number as usize).ok_or_else(invalid);
    Ok(semio_framework_value::RetainedCloneGrant{maximum_items:axis("maximumItems")?,maximum_copy_bytes:axis("maximumCopyBytes")?,maximum_capacity_bytes:axis("maximumCapacityBytes")?,maximum_release_bytes:axis("maximumReleaseBytes")?,maximum_depth:axis("maximumDepth")?})
}

/// 🎟️ The command borrows original cancellation and carries its one caller turn and work policy.
struct InferenceCallerAuthority<'a>{pub turn:semio_framework_actor::RetainedTurnInput,pub retained_policy:semio_framework_value::RetainedCloneGrant,pub revision:u64,pub cancellation_id:&'a str,pub work_units:u64,pub maximum_elapsed_milliseconds:u64}

/// 🎟️ Exact JSON identities are mandatory before any guest command or job is born.
pub(crate) fn inference_original_turn(arguments: &serde_json::Value) -> Result<InferenceCallerAuthority<'_>, GatewayError> {
    let integer = |value: &serde_json::Value, name: &str, minimum: u64| value.get(name)
        .and_then(serde_json::Value::as_u64)
        .filter(|number| *number >= minimum && *number <= 9007199254740991)
        .ok_or_else(|| inference_field_invalid(name, "an explicit exact unsigned JSON identity or work bound is required"));
    let source = arguments.get("turn").and_then(serde_json::Value::as_object)
        .filter(|source| source.len() == 5)
        .ok_or_else(|| inference_field_invalid("turn", "the original operation, generation, epoch, work and total grant are required"))?;
    let turn_value = &arguments["turn"];
    let work = source.get("work").filter(|value| value.as_object().is_some_and(|fields| fields.len() == 3))
        .ok_or_else(|| inference_field_invalid("turn.work", "the original clock binding, absolute deadline and total fuel are required"))?;
    let turn = semio_framework_actor::RetainedTurnInput {
        operation: integer(turn_value, "operation", 1)?,
        generation: integer(turn_value, "generation", 1)?,
        epoch: integer(turn_value, "epoch", 1)?,
        work: semio_framework_actor::OriginalActorTurnWork {
            clock_binding: integer(work, "clockBinding", 1)?,
            absolute_deadline_us: integer(work, "absoluteDeadlineUs", 1)?,
            fuel: integer(work, "fuel", 0)?,
        },
        grant: inference_original_retained(source.get("grant")
            .ok_or_else(|| inference_field_invalid("turn.grant", "the original total grant is required"))?)?,
    };
    let policy = arguments.get("retainedPolicy").ok_or_else(|| inference_field_invalid("retainedPolicy", "an explicit original call policy is required"))?;
    let retained_policy = inference_original_retained(policy)
        .map_err(|_| inference_field_invalid("retainedPolicy", "all five explicit call limits are required"))?;
    let total = turn.grant;
    if retained_policy.maximum_items > total.maximum_items
        || retained_policy.maximum_copy_bytes > total.maximum_copy_bytes
        || retained_policy.maximum_capacity_bytes > total.maximum_capacity_bytes
        || retained_policy.maximum_release_bytes > total.maximum_release_bytes
        || retained_policy.maximum_depth > total.maximum_depth {
        return Err(inference_field_invalid("retainedPolicy", "the call policy exceeds its original total grant"));
    }
    let work_units = integer(arguments, "workUnits", 1)?;
    if work_units > turn.work.fuel {
        return Err(inference_field_invalid("workUnits", "the call work limit exceeds its original remaining fuel"));
    }
    let cancellation_id = arguments.get("cancellationId").and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| inference_field_invalid("cancellationId", "the original nonempty cancellation identity is required"))?;
    Ok(InferenceCallerAuthority {
        turn, retained_policy, revision: integer(arguments, "revision", 0)?, cancellation_id,
        work_units, maximum_elapsed_milliseconds: integer(arguments, "maximumElapsedMilliseconds", 1)?,
    })
}

/// 🏃️ The one guest engine both `inference_run` and a guest job drive: bind the artifact's document
/// into the request, route it to the service's plugin guest, run it under `cancel`, reporting each
/// step on `job_id`. `Ok(None)` is a run the caller's cancel stopped.
fn run_guest_inference(workspace: &Arc<HeadlessWorkspace>, actions: &ActionAdapter, item: &DeclaredInference, artifact_id: Option<&str>, arguments: &serde_json::Value, job_id: &str, cancel: InferenceCancel) -> Result<Option<GuestInferenceRun>, GatewayError> {
    let authority=inference_original_turn(arguments)?;
    let jobs = job_registry();
    jobs.report_progress(job_id, INFERENCE_BINDING_PROGRESS_START, Some(match artifact_id {
        Some(artifact_id) => format!("binding artifact `{artifact_id}` into the request body"),
        None => "no artifact binding declared — the caller's own body travels verbatim".to_string(),
    }));
    let artifact_document = match resolve_inference_artifact_document(workspace, item, artifact_id, job_id, &cancel.0) {
        Ok(document) => document,
        Err(_) if jobs.is_cancel_requested(job_id) => return Ok(None),
        Err(error) => return Err(error.with_details(serde_json::json!({ "field": "artifactId" }))),
    };
    let command = InferCommand {
        plugin_id: item.route_plugin_id().to_string(),
        artifact_kind: item.artifact_kind.clone(),
        inference_schema: item.inference_schema.clone(),
        revision: authority.revision,
        turn: authority.turn,
        retained_policy: authority.retained_policy,
        cancellation_id: authority.cancellation_id.to_string(),
        work_units: authority.work_units,
        maximum_elapsed_milliseconds: authority.maximum_elapsed_milliseconds,
        canonical_payload: inference_run_payload_bytes(arguments),
        artifact_id: artifact_id.unwrap_or_default().to_string(),
        artifact_document,
        cancel,
    };
    jobs.report_progress(job_id, 0.25, Some(format!("routing `{}/{}` to `{}`", command.artifact_kind, command.inference_schema, command.plugin_id)));
    if jobs.is_cancel_requested(job_id) {
        return Ok(None);
    }
    jobs.report_progress(job_id, 0.35, Some("running in the plugin's guest".to_string()));
    match actions.run_inference(INFERENCE_ROUTED_INSTANCE, command) {
        Err(_) if jobs.is_cancel_requested(job_id) => Ok(None),
        Err(error) => Err(error),
        Ok(outcome) => Ok(Some(GuestInferenceRun { inference_schema: outcome.inference_schema, complete: outcome.complete, payload: outcome.payload })),
    }
}

/// 💡️ Runs one declared inference to completion in a single call — the synchronous sibling of
/// `inference_submit`, resolved through the same service model. A guest service runs here, in its
/// plugin's guest; a hub-executed service is submitted to the hub and its events are followed to a
/// terminal page. Nothing is committed: an offered proposal is returned, never applied.
///
/// 🎫️ The job is minted BEFORE the contract is checked, and that ordering is load-bearing:
/// `JobRegistry::begin` binds the id to this call's `_meta.progressToken`, so every step from
/// here on — including a REFUSAL — is pushed to the client as `notifications/progress` and is
/// readable afterwards with `job_get`/`job_cancel`.
fn inference_run_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    if let Err(error) = authorize_inference(context.policy(), context.principal, &inference_run_capability()) {
        return CallToolResult::tool_error(&error);
    }
    let Some(artifact_kind) = arguments.get("artifactKind").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&inference_input_invalid("artifactKind is required"));
    };
    let Some(inference_schema) = arguments.get("inferenceSchema").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&inference_input_invalid("inferenceSchema is required"));
    };
    let workspace = match context.workspace("inference_run") {
        Ok(workspace) => workspace,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let service = match declared_inferences_for_workspace(workspace).and_then(|declared| Ok((declared, workspace.hub_inference_services()?))).and_then(|(declared, hub)| select_inference_service(&declared, &hub, artifact_kind, Some(inference_schema), arguments.get("pluginId").and_then(serde_json::Value::as_str))) {
        Ok(service) => service,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let requested_artifact = arguments.get("artifactId").and_then(serde_json::Value::as_str);
    if service.site == InferenceExecutionSiteV1::Hub {
        return match requested_artifact {
            Some(document_id) => run_hub_inference(context, workspace, service, document_id, &arguments),
            None => CallToolResult::tool_error(&inference_field_invalid("artifactId", format!("`{}` is executed by the bound hub on a document: name it with `artifactId`", service.service_id()))),
        };
    }
    let item = service.declared.clone();
    let caller_payload = arguments.get("payload").filter(|value| !value.is_null());
    let authority=match inference_original_turn(&arguments){Ok(authority)=>authority,Err(error)=>return CallToolResult::tool_error(&error)};
    let cancellation_id=authority.cancellation_id;
    let cancel = InferenceCancel::default();

    let jobs = job_registry();
    let job_id = jobs.begin("inference.run");
    let base = serde_json::json!({
        "jobId": job_id,
        "artifactKind": item.artifact_kind,
        "inferenceSchema": item.inference_schema,
        "pluginId": item.route_plugin_id(),
        "cancellationId": cancellation_id,
        "artifactId": requested_artifact.unwrap_or_default(),
        "site": InferenceExecutionSiteV1::Guest,
    });
    let hook = cancel.0.clone();
    jobs.bind_cancel(&job_id, move || hook.cancel_now());
    jobs.report_progress(&job_id, 0.05, Some(format!("checking `{}` against its published payload contract", item.inference_schema)));
    if let Err(error) = validate_inference_request(&item, caller_payload, requested_artifact) {
        let field = inference_error_field(&error);
        let error = error.with_details(merge_inference_run_fields(base.clone(), serde_json::json!({ "field": field })));
        jobs.fail(&job_id, error.clone());
        return CallToolResult::tool_error(&error);
    }
    let cancelled = |job_id: &str, base: serde_json::Value| {
        job_registry().mark_cancelled(job_id);
        CallToolResult::ok(vec![ContentBlock::Text { text: format!("inference job {job_id} was cancelled") }], Some(merge_inference_run_fields(base, serde_json::json!({ "status": "CANCELLED", "complete": false }))))
    };
    match run_guest_inference(workspace, context.actions, &item, requested_artifact, &arguments, &job_id, cancel) {
        Ok(Some(_)) | Err(_) if jobs.is_cancel_requested(&job_id) => cancelled(&job_id, base),
        Ok(None) => cancelled(&job_id, base),
        Ok(Some(run)) => {
            let structured = merge_inference_run_fields(base, serde_json::json!({ "status": if run.complete{"SUCCEEDED"}else{"RUNNING"}, "complete": run.complete, "payload": run.payload.as_deref().and_then(inference_run_result_value), "payloadBytes": run.payload.as_ref().map_or(0,Vec::len) }));
            jobs.succeed(&job_id, structured.clone());
            CallToolResult::ok(vec![ContentBlock::Text { text: format!("`{}` produced {} byte(s) (complete: {})", run.inference_schema, run.payload.as_ref().map_or(0,Vec::len), run.complete) }], Some(structured))
        }
        Err(error) => {
            let field = inference_error_field(&error);
            let error = error.with_details(merge_inference_run_fields(base, serde_json::json!({ "field": field })));
            jobs.fail(&job_id, error.clone());
            CallToolResult::tool_error(&error)
        }
    }
}

/// ⏳️ How often `inference_run` re-reads a hub-executed job's events while it follows it.
const HUB_INFERENCE_RUN_POLL_MS: u64 = 200;

/// 🌎️ `inference_run` on a hub-executed service: submit, then follow the job's events until the hub
/// reports it terminal or offering a proposal, within the job's own lifetime.
fn run_hub_inference(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, service: InferenceService, document_id: &str, arguments: &serde_json::Value) -> CallToolResult {
    let followed = (|| {
        let (_, submitted) = submit_hub_job(context, workspace, service.clone(), document_id, arguments)?;
        let handle = submitted.get("jobHandle").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        let payload = context.job(workspace, &handle)?;
        let hub = hub_scope_route(&payload)?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(arguments.get("lifetimeMs").and_then(serde_json::Value::as_u64).unwrap_or(INFERENCE_JOB_MAX_LIFETIME_MS));
        loop {
            let page = hub_events_page(&service, document_id, &workspace.read_hub_inference_job_events(document_id, &hub.route, &payload.job_id, 0)?);
            let settled = !matches!(page.state, InferenceJobStateV1::Accepted | InferenceJobStateV1::Running);
            if settled || std::time::Instant::now() >= deadline {
                let status = match page.state {
                    InferenceJobStateV1::Succeeded => "SUCCEEDED",
                    InferenceJobStateV1::Cancelled => "CANCELLED",
                    InferenceJobStateV1::Failed => "FAILED",
                    InferenceJobStateV1::Accepted | InferenceJobStateV1::Running => "RUNNING",
                };
                return Ok((page_text(&page), serde_json::json!({ "jobId": page.job_id, "status": status, "artifactKind": service.declared.artifact_kind, "inferenceSchema": service.service_id(), "pluginId": service.plugin_id(), "artifactId": document_id, "site": InferenceExecutionSiteV1::Hub, "jobHandle": handle, "job": page_value(&page) })));
            }
            std::thread::sleep(std::time::Duration::from_millis(HUB_INFERENCE_RUN_POLL_MS));
        }
    })();
    match followed {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// 🧩️ Folds the terminal fields onto the invariant ones — one object, built once, so the tool reply
/// and the job registry's own retained result are literally the same value.
fn merge_inference_run_fields(mut base: serde_json::Value, extra: serde_json::Value) -> serde_json::Value {
    if let (Some(base_map), Some(extra_map)) = (base.as_object_mut(), extra.as_object()) {
        for (key, value) in extra_map {
            base_map.insert(key.clone(), value.clone());
        }
    }
    base
}

/// 📈️ The span of `inference_run`'s job progress its artifact binding reports into, between the
/// binding row and routing.
const INFERENCE_BINDING_PROGRESS_START: f64 = 0.15;

const INFERENCE_BINDING_PROGRESS_SPAN: f64 = 0.1;

/// 🔢️ `RoutingArtifactChannel` resolves an inference's plugin from the command's own `pluginId`, so
/// the `instance` slot the mutation protocol encodes carries no meaning here — a fixed, documented
/// zero rather than a fabricated per-capability slot.
const INFERENCE_ROUTED_INSTANCE: u32 = 0;

/// 💡️ Registers the four inference job tools plus `inference_run`. Like every other tool in this
/// crate they are ALWAYS present in `tools/list`; only a call's result varies by whether a
/// workspace (and, for a hub-executed service, a hub binding) exists, and every call is first gated
/// by the connection's own granted MCP scopes.
pub fn register_inference_job_tools(registry: &mut InMemoryToolRegistry, catalog: Arc<Catalog>, workspace: Option<Arc<HeadlessWorkspace>>, actions: Arc<ActionAdapter>, principal: AgentPrincipal, session: SessionHandle) {
    let definitions: [(CapabilityDefinition, &str, fn(&InferenceToolContext<'_>, serde_json::Value) -> CallToolResult); 5] = [
        (inference_submit_capability(), "inference_submit", inference_submit_handler),
        (inference_events_capability(), "inference_events", inference_events_handler),
        (inference_cancel_capability(), "inference_cancel", inference_cancel_handler),
        (inference_approve_capability(), "inference_approve", inference_approve_handler),
        (inference_run_capability(), "inference_run", inference_run_handler),
    ];
    for (capability, tool_name, handler) in definitions {
        let tool = tool_from_capability(&capability, tool_name);
        let (tool_workspace, tool_actions, tool_catalog, tool_principal, tool_session) = (workspace.clone(), actions.clone(), catalog.clone(), principal.clone(), session.clone());
        registry
            .register(tool, move |arguments| {
                let context = InferenceToolContext { workspace: tool_workspace.as_ref(), actions: &tool_actions, catalog: &tool_catalog, principal: &tool_principal, session: tool_session.clone() };
                handler(&context, arguments)
            })
            .expect("inference job tool names are valid");
    }
}

//#endregion 💡️InferenceJobTools

//#region 💡️InferenceHubRead

/// 🆔️ The deterministic client idempotency key one hub-backed inference READ uses, derived from
/// nothing but the authenticated subject and the document. The hub's ledger scopes idempotency on
/// `(user, authorization generation, space, document, request_id)`, so re-reading the same field
/// reconciles to exactly ONE hub job instead of starting a fresh one each poll.
pub fn deterministic_inference_request_id(subject: &HubInferenceSubjectV1, document_id: &str) -> String {
    let seed = format!("semio.mcp.inference-read/v1\0{}\0{}\0{}\0{}\0{}", subject.hub_origin, subject.space_id, document_id, subject.user_id, subject.authority_generation);
    framework_hash::hash_bytes(seed.as_bytes())[..INFERENCE_REQUEST_ID_HEX_LENGTH].to_string()
}

/// 💡️ A READ of a hub-executed service's field: submits (or reconciles to) the one job this subject
/// owns for the document under a deterministic request id and returns its owner-private receipt. A
/// guest service answers `None` — a read never spends guest time — so the caller falls through to
/// the discovery path, which tells the reader to run it with `inference_run` or `inference_submit`.
pub fn hub_inference_read(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str, inference_schema: &str) -> Option<Result<serde_json::Value, GatewayError>> {
    let service = match resolve_document_inference_service(workspace, artifact_id, Some(inference_schema), None) {
        Ok(service) if service.site == InferenceExecutionSiteV1::Hub => service,
        Ok(_) | Err(_) => return None,
    };
    let route = service.route.clone()?;
    let subject = match workspace.hub_inference_subject() {
        Ok(subject) => subject,
        Err(error) => return Some(Err(error)),
    };
    let request = HubInferenceSubmitRequestV1::new(service.service_id(), deterministic_inference_request_id(&subject, artifact_id), INFERENCE_JOB_MAX_LIFETIME_MS);
    Some(workspace.submit_hub_inference_job(artifact_id, &route, &request).map(|receipt| {
        serde_json::json!({
            "artifactId": artifact_id,
            "inferenceSchema": inference_schema,
            "subjectUserId": subject.user_id,
            "authorityGeneration": subject.authority_generation,
            "job": page_value(&hub_receipt_page(&service, artifact_id, &receipt)),
        })
    }))
}

//#endregion 💡️InferenceHubRead

//#region 💼️Jobs
//#endregion 💼️Jobs

//#region 🧪️Tests
//#endregion 🧪️Tests

//#region 🧪️InferenceJobTests
//#endregion 🧪️InferenceJobTests
