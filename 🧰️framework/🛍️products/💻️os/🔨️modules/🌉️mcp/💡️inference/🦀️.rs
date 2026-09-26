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
//! `🌎️hub/🧫️fixtures/🗳️gis-map-proposal-approval-v1`.

use crate::catalog::{Catalog, CapabilityAudience, CapabilityDefinition, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, ToolExposure};
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::tool_from_capability;
use crate::protocol::{CallToolResult, ContentBlock, GatewayBackend, InMemoryToolRegistry, Resource, ResourceContent};
use crate::policy::{AgentPrincipal, PolicyEngine};
use crate::schema::{
    inference_approve_input_schema, inference_get_input_schema, inference_get_output_schema, inference_job_handle_input_schema, inference_job_output_schema, inference_list_input_schema, inference_list_output_schema,
    inference_submit_input_schema,
};
use crate::workspace::remote::percent_encode;
use crate::workspace::{HeadlessWorkspace, PROBE_SCHEMA};
use semio_framework_async::OperationContext;
use semio_framework_os_kernel::os_directory::DocumentScope;
use semio_framework_os_kernel::{FromValue, ToValue};
use std::sync::Arc;

//#region 🔖️DeclaredInference
/// 💡️ One declared inference service, wire-shaped 1:1 from `semio_framework::ContributedInferenceMetadata`
/// — the exact static fields a plugin's own `🔣️.json` carries, never a live guest call.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
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
    match workspace.origin() {
        crate::workspace::WorkspaceOrigin::Hub { .. } => Ok(Vec::new()),
        crate::workspace::WorkspaceOrigin::Folder { .. } => crate::registry::discover_descriptors(&crate::find_repo_root()?),
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
fn execution_not_wired_error(item: &DeclaredInference) -> GatewayError {
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
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
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

//#region 🔖️Capabilities
fn inference_capability(id: &str, tool_name: &str, title: &str, description: &str, input_schema: serde_json::Value, output_schema: serde_json::Value) -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Query,
        audience: CapabilityAudience::Agent,
        title: title.to_string(),
        description: description.to_string(),
        artifact_kind: None,
        use_when: vec!["what inferences exist for this artifact".to_string(), "read an inferred value".to_string()],
        input_schema,
        output_schema,
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: tool_name.to_string() },
        presentation: CapabilityPresentation { icon_id: Some("brain".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn inference_list_capability() -> CapabilityDefinition {
    inference_capability(
        "inference.list",
        "inference_list",
        "List Declared Inferences",
        "Lists inference metadata declared for one resolvable artifact or every catalogued workspace plugin. Discovery does not grant execution or document access.",
        inference_list_input_schema(),
        inference_list_output_schema(),
    )
}

fn inference_get_capability() -> CapabilityDefinition {
    inference_capability("inference.get", "inference_get", "Get Inference", "Reads one declared inference field for an artifact — a typed, retryable gap until a real artifact-infer route is wired.", inference_get_input_schema(), inference_get_output_schema())
}

/// 💡️ The plugin-declared inference capabilities — discovery (`inference_list`/`inference_get`) and
/// the general execution route (`inference_run`) — folded into `CatalogSource.gateway` by root
/// wiring, same pattern as `🦀️.rs`'s own `core_tool_capabilities()`.
pub fn inference_capabilities() -> Vec<CapabilityDefinition> {
    vec![inference_list_capability(), inference_get_capability(), inference_run_capability()]
}
//#endregion 🔖️Capabilities

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

//#region 💡️InferenceJobWire
/// 💡️ The exact closed wire vocabulary the hub's four authenticated GIS Map inference routes
/// publish (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs`, `🌎️hub/🏗️bootstrap/🦀️.rs`'s own `//#region 💡️Inference`).
/// Mirrored here as typed Rust rather than reached for as free-form JSON: nothing on this
/// boundary is a `serde_json::Value`, and a hub field this client does not know about is a loud
/// decode failure rather than a silently-dropped one.
pub const HUB_INFERENCE_REQUEST_SCHEMA: &str = "semio.hub.inference-request/v1";
pub const HUB_INFERENCE_APPROVAL_SCHEMA: &str = "semio.hub.inference-approval/v1";
pub const HUB_INFERENCE_RECEIPT_SCHEMA: &str = "semio.hub.inference-job-receipt/v1";
pub const HUB_INFERENCE_EVENTS_SCHEMA: &str = "semio.hub.inference-job-events/v1";
pub const HUB_INFERENCE_APPROVAL_RECEIPT_SCHEMA: &str = "semio.hub.inference-approval-receipt/v1";
pub const HUB_INFERENCE_APPROVAL_UNDO_RECEIPT_SCHEMA: &str = "semio.hub.gis-map-approval-undo-receipt/v1";
pub const HUB_INFERENCE_ERROR_SCHEMA: &str = "semio.hub.inference-error/v1";
pub const GIS_MAP_INFERENCE_PREVIEW_SCHEMA: &str = "semio.hub.gis-map-inference-preview/v1";
pub const GIS_MAP_INFERENCE_PREVIEW_RING_POINTS: usize = 5;
const _: () = assert!(GIS_MAP_INFERENCE_PREVIEW_RING_POINTS == 5, "the closed preview ring is exactly five points, first equal to last");
pub const INFERENCE_REQUEST_MAX_BYTES: usize = 1024;
pub const INFERENCE_RESPONSE_MAX_BYTES: usize = 8192;
pub const INFERENCE_JOB_MAX_LIFETIME_MS: u64 = 120_000;
pub const INFERENCE_PROGRESS_MAX_CURSOR: u64 = 16;
pub const INFERENCE_EVENT_PAGE_MAX_ITEMS: usize = 8;
pub const INFERENCE_POLICY_VERSION: u32 = 1;
pub const INFERENCE_REQUEST_ID_HEX_LENGTH: usize = 32;
pub const INFERENCE_PROPOSAL_HASH_HEX_LENGTH: usize = 64;
pub const INFERENCE_INFLIGHT_CAPACITY: usize = 32;

/// 🚦️ The one stable failure vocabulary those routes publish, code and HTTP status verbatim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceRouteErrorV1 {
    Unavailable,
    Denied,
    NotFound,
    Invalid,
    Bounds,
    Conflict,
    Capacity,
    Expired,
    Cancelled,
    CommitUnavailable,
    Storage,
}

pub const INFERENCE_ROUTE_ERRORS: [InferenceRouteErrorV1; 11] = [
    InferenceRouteErrorV1::Unavailable,
    InferenceRouteErrorV1::Denied,
    InferenceRouteErrorV1::NotFound,
    InferenceRouteErrorV1::Invalid,
    InferenceRouteErrorV1::Bounds,
    InferenceRouteErrorV1::Conflict,
    InferenceRouteErrorV1::Capacity,
    InferenceRouteErrorV1::Expired,
    InferenceRouteErrorV1::Cancelled,
    InferenceRouteErrorV1::CommitUnavailable,
    InferenceRouteErrorV1::Storage,
];

impl InferenceRouteErrorV1 {
    /// 🏷️ The exact wire code; a caller never learns which private hub object was missing.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "inference.unavailable",
            Self::Denied => "inference.denied",
            Self::NotFound => "inference.not-found",
            Self::Invalid => "inference.invalid",
            Self::Bounds => "inference.bounds",
            Self::Conflict => "inference.conflict",
            Self::Capacity => "inference.capacity",
            Self::Expired => "inference.expired",
            Self::Cancelled => "inference.cancelled",
            Self::CommitUnavailable => "approval.commit-unavailable",
            Self::Storage => "inference.storage",
        }
    }

    /// 🔢️ The exact HTTP status the hub publishes for this code.
    pub const fn status(self) -> u16 {
        match self {
            Self::Unavailable | Self::CommitUnavailable | Self::Storage => 503,
            Self::Denied => 403,
            Self::NotFound => 404,
            Self::Invalid => 400,
            Self::Bounds => 413,
            Self::Conflict | Self::Cancelled => 409,
            Self::Capacity => 429,
            Self::Expired => 410,
        }
    }

    /// 🔎️ Resolves a published code back to its variant; an unknown code is never guessed at.
    pub fn from_code(code: &str) -> Option<Self> {
        INFERENCE_ROUTE_ERRORS.into_iter().find(|candidate| candidate.code() == code)
    }

    /// 🔢️ The conservative fallback when a hub reply carries no decodable closed error body: the
    /// status alone is ambiguous for 503 and 409, so it resolves to the widest honest member.
    pub fn from_status(status: u16) -> Self {
        match status {
            403 => Self::Denied,
            404 => Self::NotFound,
            400 => Self::Invalid,
            413 => Self::Bounds,
            409 => Self::Conflict,
            429 => Self::Capacity,
            410 => Self::Expired,
            _ => Self::Unavailable,
        }
    }

    /// ♻️ Whether a caller may retry this exact call unchanged without changing anything else.
    pub const fn retryable(self) -> bool {
        matches!(self, Self::Unavailable | Self::CommitUnavailable | Self::Storage | Self::Capacity)
    }

    /// 🧭️ The gateway code this maps to, plus the one sentence that names the missing binding.
    pub fn gateway_code(self) -> GatewayErrorCode {
        match self {
            Self::Unavailable | Self::CommitUnavailable | Self::Storage | Self::Capacity => GatewayErrorCode::PluginUnavailable,
            Self::Denied => GatewayErrorCode::PermissionDenied,
            Self::NotFound => GatewayErrorCode::NotFound,
            Self::Invalid => GatewayErrorCode::InputInvalid,
            Self::Bounds => GatewayErrorCode::BudgetExceeded,
            Self::Conflict | Self::Expired => GatewayErrorCode::PreconditionFailed,
            Self::Cancelled => GatewayErrorCode::Cancelled,
        }
    }

    fn explanation(self) -> &'static str {
        match self {
            Self::Unavailable => "this hub executes no such inference service — bind a hub whose readiness lists it in `features.inferenceServices`",
            Self::CommitUnavailable => "the hub has no registered atomic parent+existing-child composition transaction, so an approval cannot be published; the prepared proposal is retained and nothing was applied",
            Self::Storage => "the hub's private inference ledger is temporarily unavailable",
            Self::Capacity => "the hub's fixed inference operation capacity is exhausted",
            Self::Denied => "the hub re-checked the live author, session, authorization generation and scope and refused; only the document's current `Author` may run, read, cancel or approve a job, and only its original owner",
            Self::NotFound => "the hub has no such job or document for this authenticated subject",
            Self::Invalid => "the hub rejected this closed client intent",
            Self::Bounds => "the request or the document it runs on exceeded one of the hub's declared inference bounds (`InferenceLimitsV1`: the intent, the document's map base, the result or the proposal)",
            Self::Conflict => "the frozen binding, document frontier, base pack or proposal hash drifted from the accepted job",
            Self::Expired => "this job outlived the hub's fixed job lifetime",
            Self::Cancelled => "this job carries a durable cancel request",
        }
    }

    /// ⚠️ The typed gateway error one tool call answers with; retryable members stay retryable.
    pub fn to_gateway_error(self, what: &str) -> GatewayError {
        let error = GatewayError::new(self.gateway_code(), format!("`{what}` was refused by the hub with `{}`: {}", self.code(), self.explanation()))
            .with_details(serde_json::json!({ "inferenceCode": self.code(), "httpStatus": self.status(), "retryable": self.retryable() }));
        if self.retryable() {
            error.retryable()
        } else {
            error
        }
    }
}

/// ⚠️ Why a protected inference request never reached a decodable hub reply at all.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InferenceHubTransportErrorV1 {
    Cancelled,
    DeadlineExceeded,
    Unauthorized,
    Unavailable,
    ResourceLimit,
    InvalidRequest(&'static str),
}

impl From<InferenceHubTransportErrorV1> for InferenceRouteErrorV1 {
    fn from(error: InferenceHubTransportErrorV1) -> Self {
        match error {
            InferenceHubTransportErrorV1::Cancelled => Self::Cancelled,
            InferenceHubTransportErrorV1::DeadlineExceeded | InferenceHubTransportErrorV1::Unavailable => Self::Unavailable,
            InferenceHubTransportErrorV1::Unauthorized => Self::Denied,
            InferenceHubTransportErrorV1::ResourceLimit => Self::Bounds,
            InferenceHubTransportErrorV1::InvalidRequest(_) => Self::Invalid,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceJobStateV1 {
    Accepted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceProposalStateV1 {
    None,
    Offered,
    Approved,
    Stale,
    Cancelled,
}

/// 📥️ The closed client intent `POST …/inference/gis-map/jobs` accepts, byte for byte.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceSubmitRequestV1 {
    pub schema: String,
    pub version: u32,
    pub request_id: String,
    pub service_id: String,
    pub policy_version: u32,
    pub lifetime_ms: u64,
}

impl HubInferenceSubmitRequestV1 {
    /// 🧭️ `service_id` comes from [`resolve_document_inference_service`] — the artifact's own
    /// descriptor kind and the hub's published roster decide which service this intent names.
    pub fn new(service_id: impl Into<String>, request_id: impl Into<String>, lifetime_ms: u64) -> Self {
        Self {
            schema: HUB_INFERENCE_REQUEST_SCHEMA.to_string(),
            version: 1,
            request_id: request_id.into(),
            service_id: service_id.into(),
            policy_version: INFERENCE_POLICY_VERSION,
            lifetime_ms,
        }
    }

    pub fn validate(&self) -> Result<(), InferenceRouteErrorV1> {
        if self.schema != HUB_INFERENCE_REQUEST_SCHEMA
            || self.version != 1
            || !is_lower_hex(&self.request_id, INFERENCE_REQUEST_ID_HEX_LENGTH)
            || !is_service_id(&self.service_id)
            || self.policy_version != INFERENCE_POLICY_VERSION
            || self.lifetime_ms == 0
            || self.lifetime_ms > INFERENCE_JOB_MAX_LIFETIME_MS
        {
            return Err(InferenceRouteErrorV1::Invalid);
        }
        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>, InferenceRouteErrorV1> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        if bytes.len() > INFERENCE_REQUEST_MAX_BYTES {
            return Err(InferenceRouteErrorV1::Bounds);
        }
        Ok(bytes)
    }
}

/// ✅️ The closed approval intent `POST …/jobs/{job_id}/approval` accepts, byte for byte.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceApprovalRequestV1 {
    pub schema: String,
    pub version: u32,
    pub job_id: String,
    pub proposal_hash: String,
}

impl HubInferenceApprovalRequestV1 {
    pub fn new(job_id: impl Into<String>, proposal_hash: impl Into<String>) -> Self {
        Self { schema: HUB_INFERENCE_APPROVAL_SCHEMA.to_string(), version: 1, job_id: job_id.into(), proposal_hash: proposal_hash.into() }
    }

    pub fn validate(&self) -> Result<(), InferenceRouteErrorV1> {
        if self.schema != HUB_INFERENCE_APPROVAL_SCHEMA || self.version != 1 || !is_lower_hex(&self.job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) || !is_lower_hex(&self.proposal_hash, INFERENCE_PROPOSAL_HASH_HEX_LENGTH) {
            return Err(InferenceRouteErrorV1::Invalid);
        }
        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>, InferenceRouteErrorV1> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        if bytes.len() > INFERENCE_REQUEST_MAX_BYTES {
            return Err(InferenceRouteErrorV1::Bounds);
        }
        Ok(bytes)
    }
}

/// 🧾️ The closed receipt a submitted job returns; it never carries private result or base bytes.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceJobReceiptV1 {
    pub schema: String,
    pub job_id: String,
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub proposal_hash: Option<String>,
    pub cursor: u64,
    pub expires_at_ms: u64,
}

/// 🗓️ One owner-private lifecycle event.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceEventV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
}

/// 📈️ One owner-private progress row.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceProgressV1 {
    pub cursor: u64,
    pub run_epoch: u64,
    pub completed: u64,
    pub total: u64,
    pub at_ms: u64,
}

/// 📃️ The owner-private bounded page one `events` read returns — MCP's only progress channel.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceEventPageV1 {
    pub schema: String,
    pub job_id: String,
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub cancel_requested: bool,
    pub stale: bool,
    pub proposal_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<GisMapInferencePreviewV1>,
    pub events: Vec<HubInferenceEventV1>,
    pub progress: Vec<HubInferenceProgressV1>,
    pub next_cursor: u64,
}

/// 🗺️ The Hub-validated bounds geometry an authenticated owner may inspect before approval.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferencePreviewV1 {
    pub schema: String,
    pub job_id: String,
    pub proposal_hash: String,
    pub region_id: String,
    pub ring: [[f64; 2]; 5],
}

impl GisMapInferencePreviewV1 {
    /// 🔒️ The exact shape law the hub enforces before publishing one, re-checked client side so a
    /// renderer never draws geometry this gateway did not verify: the declared schema, a region id
    /// derived from the job alone, a lower-hex proposal digest, and a CLOSED axis-aligned ring whose
    /// first and last points coincide. It is a preview, never a mutation — the hub rebuilds the
    /// typed effect from its own base at approval time regardless of what any client rendered.
    pub fn validate(&self, job_id: &str) -> Result<(), InferenceRouteErrorV1> {
        if self.schema != GIS_MAP_INFERENCE_PREVIEW_SCHEMA || self.job_id != job_id || self.region_id != format!("inference-{job_id}") || !is_lower_hex(&self.proposal_hash, INFERENCE_PROPOSAL_HASH_HEX_LENGTH) {
            return Err(InferenceRouteErrorV1::Invalid);
        }
        let [lon_min, lat_min] = self.ring[0];
        let [lon_max, lat_max] = self.ring[2];
        if lon_min > lon_max || lat_min > lat_max || self.ring != [[lon_min, lat_min], [lon_max, lat_min], [lon_max, lat_max], [lon_min, lat_max], [lon_min, lat_min]] {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        Ok(())
    }
}

/// ✅️ The closed approval outcome; `applied` is true only after a real committed-WAL witness.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceApprovalReceiptV1 {
    pub schema: String,
    pub job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub proposal_hash: String,
    pub applied: bool,
    pub undo: semio_framework_os_kernel::os_directory::GisMapApprovalUndoHandleV1,
}

/// 🧾️ The two-field closed body every failing inference route publishes.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceErrorBodyV1 {
    pub schema: String,
    pub code: String,
}

/// 🔖️ The one schema string each decodable hub reply must declare for its own shape.
pub trait InferenceHubBodyV1: serde::de::DeserializeOwned {
    const SCHEMA: &'static str;
    fn declared_schema(&self) -> &str;
}

impl InferenceHubBodyV1 for HubInferenceJobReceiptV1 {
    const SCHEMA: &'static str = HUB_INFERENCE_RECEIPT_SCHEMA;
    fn declared_schema(&self) -> &str {
        &self.schema
    }
}

impl InferenceHubBodyV1 for HubInferenceEventPageV1 {
    const SCHEMA: &'static str = HUB_INFERENCE_EVENTS_SCHEMA;
    fn declared_schema(&self) -> &str {
        &self.schema
    }
}

impl InferenceHubBodyV1 for HubInferenceApprovalReceiptV1 {
    const SCHEMA: &'static str = HUB_INFERENCE_APPROVAL_RECEIPT_SCHEMA;
    fn declared_schema(&self) -> &str {
        &self.schema
    }
}

impl InferenceHubBodyV1 for semio_framework_os_kernel::os_directory::GisMapApprovalUndoReceiptV1 {
    const SCHEMA: &'static str = HUB_INFERENCE_APPROVAL_UNDO_RECEIPT_SCHEMA;
    fn declared_schema(&self) -> &str {
        &self.schema
    }
}

/// 🪪️ A declared inference service id: the `InferenceIdV1` shape the discovery schema pins.
fn is_service_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256 && value.bytes().next().is_some_and(|byte| byte.is_ascii_alphanumeric()) && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'))
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
//#endregion 💡️InferenceJobWire

//#region 💡️InferenceHubClient
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceHubMethodV1 {
    Get,
    Post,
}

/// 📨️ One protected inference request: an origin, an exact path, and a bounded closed body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceHubRequestV1 {
    pub hub_origin: String,
    pub method: InferenceHubMethodV1,
    pub path: String,
    pub body: Vec<u8>,
    pub maximum_response_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceHubResponseV1 {
    pub status: u16,
    pub body: Vec<u8>,
}

/// 🔌️ The injection seam for the four inference routes — the JSON twin of `CanonicalPairTransport`.
/// No concrete HTTP type ever appears above it, and the bearer never crosses it.
pub trait InferenceHubTransport: Send + Sync {
    async fn request(&self, context: &OperationContext, request: &InferenceHubRequestV1) -> Result<InferenceHubResponseV1, InferenceHubTransportErrorV1>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeInferenceHubTransport<R: semio_framework_async::HostAsyncRuntime> {
    transport: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<R>,
    credential: Arc<semio_framework_os_kernel::os_directory::client::LocalHubCredential>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<R: semio_framework_async::HostAsyncRuntime> NativeInferenceHubTransport<R> {
    pub fn new(transport: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<R>, credential: Arc<semio_framework_os_kernel::os_directory::client::LocalHubCredential>) -> Self {
        Self { transport, credential }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<R: semio_framework_async::HostAsyncRuntime + 'static> InferenceHubTransport for NativeInferenceHubTransport<R> {
    async fn request(&self, context: &OperationContext, request: &InferenceHubRequestV1) -> Result<InferenceHubResponseV1, InferenceHubTransportErrorV1> {
        use semio_framework_os_kernel::os_directory::client::{HttpMethod, TransportError};
        if self.credential.hub_origin().trim_end_matches('/') != request.hub_origin.trim_end_matches('/') {
            return Err(InferenceHubTransportErrorV1::InvalidRequest("inference request authority mismatch"));
        }
        if !request.path.starts_with('/') || request.path.starts_with("//") || request.path.contains('#') || request.body.len() > INFERENCE_REQUEST_MAX_BYTES {
            return Err(InferenceHubTransportErrorV1::InvalidRequest("inference request path or body is out of bounds"));
        }
        let method = match request.method {
            InferenceHubMethodV1::Get => HttpMethod::Get,
            InferenceHubMethodV1::Post => HttpMethod::Post,
        };
        let url = format!("{}{}", request.hub_origin.trim_end_matches('/'), request.path);
        let body = match request.method {
            InferenceHubMethodV1::Get => None,
            InferenceHubMethodV1::Post => Some(request.body.clone()),
        };
        let response = self.transport.request_protected_json(context, self.credential.as_ref(), method, &url, body).await.map_err(|error| match error {
            TransportError::Cancelled => InferenceHubTransportErrorV1::Cancelled,
            TransportError::DeadlineExceeded => InferenceHubTransportErrorV1::DeadlineExceeded,
            TransportError::Io(_) => InferenceHubTransportErrorV1::Unavailable,
        })?;
        if response.body.len() > request.maximum_response_bytes {
            return Err(InferenceHubTransportErrorV1::ResourceLimit);
        }
        Ok(InferenceHubResponseV1 { status: response.status, body: response.body })
    }
}

/// 🛣️ The exact hub paths of one hub-executed service, built from the route family the hub
/// publishes for it and percent-encoded per segment — never string-concatenated by a caller.
pub fn hub_inference_jobs_path(scope: &DocumentScope, route: &str) -> String {
    format!("/spaces/{}/documents/{}/{route}/jobs", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

pub fn hub_inference_job_events_path(scope: &DocumentScope, route: &str, job_id: &str, after: u64) -> String {
    format!("{}/{}/events?after={after}", hub_inference_jobs_path(scope, route), percent_encode(job_id))
}

pub fn hub_inference_job_cancel_path(scope: &DocumentScope, route: &str, job_id: &str) -> String {
    format!("{}/{}/cancel", hub_inference_jobs_path(scope, route), percent_encode(job_id))
}

pub fn hub_inference_job_approval_path(scope: &DocumentScope, route: &str, job_id: &str) -> String {
    format!("{}/{}/approval", hub_inference_jobs_path(scope, route), percent_encode(job_id))
}

pub fn hub_inference_approval_undo_path(scope: &DocumentScope, route: &str) -> String {
    format!("/spaces/{}/documents/{}/{route}/approval-undos", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

/// 🛂️ A route family the hub published: relative, lower-case segments only, so a hostile readiness
/// body can never steer a protected request off its document scope.
pub fn is_hub_inference_route(route: &str) -> bool {
    !route.is_empty() && route.len() <= 64 && route.split('/').all(|segment| !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'))
}

/// 🔓️ Decodes one hub reply: a 2xx must be the exact declared schema, anything else resolves
/// through the closed `{schema, code}` body and only falls back to the status when that body is
/// itself undecodable.
pub fn decode_inference_reply<B: InferenceHubBodyV1>(response: &InferenceHubResponseV1) -> Result<B, InferenceRouteErrorV1> {
    if !(200..=299).contains(&response.status) {
        let error = serde_json::from_slice::<HubInferenceErrorBodyV1>(&response.body)
            .ok()
            .filter(|body| body.schema == HUB_INFERENCE_ERROR_SCHEMA)
            .and_then(|body| InferenceRouteErrorV1::from_code(&body.code));
        return Err(error.unwrap_or_else(|| InferenceRouteErrorV1::from_status(response.status)));
    }
    if response.body.len() > INFERENCE_RESPONSE_MAX_BYTES {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let body: B = serde_json::from_slice(&response.body).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    if body.declared_schema() != B::SCHEMA {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    Ok(body)
}

/// 🧾️ The part of a hub's readiness body that declares the inference services it executes.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HubReadinessInferenceV1 {
    features: HubReadinessInferenceFeaturesV1,
    #[serde(default)]
    startup: Option<serde::de::IgnoredAny>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HubReadinessInferenceFeaturesV1 {
    inference_services: Vec<HubInferenceServiceV1>,
}

/// 🌎️ `GET /readyz` → `features.inferenceServices`. A body that declares none is an empty roster;
/// one whose routes are not relative lower-case route families is refused whole. A hub still booting
/// (`startup` present) has not loaded the catalog its roster comes from, so it is `Unavailable` — a
/// retryable answer — never an empty roster the caller would act on.
pub async fn read_hub_inference_services<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str) -> Result<Vec<HubInferenceServiceV1>, InferenceRouteErrorV1> {
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Get, path: "/readyz".to_string(), body: Vec::new(), maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    if !(200..=299).contains(&response.status) && response.status != 503 {
        return Err(InferenceRouteErrorV1::from_status(response.status));
    }
    let readiness: HubReadinessInferenceV1 = serde_json::from_slice(&response.body).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    if readiness.startup.is_some() {
        return Err(InferenceRouteErrorV1::Unavailable);
    }
    let services = readiness.features.inference_services;
    if services.iter().any(|service| !is_service_id(&service.service_id) || !is_hub_inference_route(&service.route)) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    Ok(services)
}

/// 📥️ `POST /spaces/{space}/documents/{document}/{route}/jobs`.
pub async fn submit_hub_inference_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, route: &str, request: &HubInferenceSubmitRequestV1) -> Result<HubInferenceJobReceiptV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let body = request.encode()?;
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: hub_inference_jobs_path(scope, route), body, maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    decode_inference_reply(&response)
}

/// 📤️ `GET …/jobs/{job}/events?after=<cursor>` — the poll MCP uses in place of a progress push.
pub async fn read_hub_inference_job_events<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, route: &str, job_id: &str, after: u64) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) || !is_lower_hex(job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) || after > INFERENCE_PROGRESS_MAX_CURSOR {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Get, path: hub_inference_job_events_path(scope, route, job_id, after), body: Vec::new(), maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    let page: HubInferenceEventPageV1 = decode_inference_reply(&response)?;
    checked_page(page, job_id)
}

/// 🔒️ Refuses a page whose job id or offered preview does not match what was asked for, so a
/// renderer never receives geometry this gateway did not verify itself.
fn checked_page(page: HubInferenceEventPageV1, job_id: &str) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
    if page.job_id != job_id {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    if let Some(preview) = page.preview.as_ref() {
        preview.validate(job_id)?;
        if page.proposal_hash.as_deref() != Some(preview.proposal_hash.as_str()) {
            return Err(InferenceRouteErrorV1::Conflict);
        }
    }
    Ok(page)
}

/// 🛑️ `POST …/jobs/{job}/cancel` — the only durable cancellation; never the discarded
/// `notifications/cancelled` JSON-RPC no-op, which cancels a REQUEST and not a job.
pub async fn cancel_hub_inference_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, route: &str, job_id: &str) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) || !is_lower_hex(job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: hub_inference_job_cancel_path(scope, route, job_id), body: Vec::new(), maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    let page: HubInferenceEventPageV1 = decode_inference_reply(&response)?;
    checked_page(page, job_id)
}

/// ✅️ `POST …/jobs/{job}/approval` — explicit approval only; the hub rebuilds the typed effect.
pub async fn approve_hub_inference_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, route: &str, request: &HubInferenceApprovalRequestV1) -> Result<HubInferenceApprovalReceiptV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let body = request.encode()?;
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: hub_inference_job_approval_path(scope, route, &request.job_id), body, maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    let receipt: HubInferenceApprovalReceiptV1 = decode_inference_reply(&response)?;
    if receipt.job_id != request.job_id
        || receipt.proposal_hash != request.proposal_hash
        || !is_lower_hex(&receipt.mutation_id, 32)
        || !is_lower_hex(&receipt.command_hash, 64)
        || !is_lower_hex(&receipt.undo.target_id, 32)
        || !receipt.undo.expected_current.validate()
        || receipt.undo.expected_current.document_id != scope.document_id
    {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    Ok(receipt)
}

/// ↩️ `POST …/approval-undos` carries only the Hub-minted target, exact current frontier and retry identity.
pub async fn undo_hub_inference_approval<T: InferenceHubTransport>(
    transport: &T,
    context: &OperationContext,
    hub_origin: &str,
    scope: &DocumentScope,
    route: &str,
    request: &semio_framework_os_kernel::os_directory::GisMapApprovalUndoRequestV1,
) -> Result<semio_framework_os_kernel::os_directory::GisMapApprovalUndoReceiptV1, InferenceRouteErrorV1> {
    if !is_hub_inference_route(route) || !request.validate() || request.expected_current.document_id != scope.document_id {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let body = serde_json::to_vec(request).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    if body.len() > INFERENCE_REQUEST_MAX_BYTES {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let wire = InferenceHubRequestV1 {
        hub_origin: hub_origin.to_string(),
        method: InferenceHubMethodV1::Post,
        path: hub_inference_approval_undo_path(scope, route),
        body,
        maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES,
    };
    let response = transport.request(context, &wire).await?;
    let receipt: semio_framework_os_kernel::os_directory::GisMapApprovalUndoReceiptV1 = decode_inference_reply(&response)?;
    if receipt.target_id != request.target_id || !receipt.applied || !receipt.frontier.validate() || receipt.frontier.document_id != scope.document_id {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    Ok(receipt)
}
//#endregion 💡️InferenceHubClient

//#region 💡️InferenceJobBinding
/// 🧊️ The client-side frozen base one job was submitted against — the P4-C canonical pair mount's
/// own identity (`descriptor_digest_v1`, `active_checkpoint_id`, `etag`, `catalog_generation`) plus
/// its verified baseline `ArtifactFrontier`. It is a LOCAL staleness and display record: the hub
/// re-derives its own binding from server objects at admission and never trusts a client field.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceBaseBindingV1 {
    pub hub_origin: String,
    pub space_id: String,
    pub document_id: String,
    pub authority_generation: u64,
    pub descriptor_digest_v1: String,
    pub active_checkpoint_id: String,
    pub etag: String,
    pub catalog_generation: Option<u64>,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_hash: String,
}

/// 🎫️ The payload one `job_` handle carries, for either execution site. The handle is owned by the
/// connection's own `SessionHandle`, so a job minted by one MCP connection is unreadable by another;
/// a hub job additionally carries the subject that minted it, and the hub applies its own
/// owner-private check on top.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceJobHandlePayloadV1 {
    pub site: InferenceExecutionSiteV1,
    pub service_id: String,
    pub artifact_kind: String,
    pub plugin_id: String,
    pub document_id: String,
    pub job_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hub: Option<HubInferenceJobBindingV1>,
}

/// 🌎️ What a hub-executed job's handle binds beyond the job itself: the published route family, the
/// minting subject and authority generation, the client idempotency key, and the frozen base.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceJobBindingV1 {
    pub route: String,
    pub space_id: String,
    pub subject_user_id: String,
    pub authority_generation: u64,
    pub request_id: String,
    pub base: Option<HubInferenceBaseBindingV1>,
}

/// 👤️ The live authenticated subject one hub-bound workspace is speaking as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubInferenceSubjectV1 {
    pub hub_origin: String,
    pub space_id: String,
    pub user_id: String,
    pub authority_generation: u64,
}
//#endregion 💡️InferenceJobBinding

//#region 💡️InferenceJobTools
/// 🛑️ Every in-flight inference call this process started, keyed by its bounded operation label, so
/// `inference_cancel` interrupts the local wait as well as recording the hub's durable cancel.
/// MCP has no progress notification and no out-of-band request cancellation wired to jobs
/// (`notifications/cancelled` is a recognized no-op for JSON-RPC REQUESTS), so this registry is the
/// only local half of cancellation; the durable half is always the hub's `/cancel` route.
static INFERENCE_INFLIGHT: std::sync::OnceLock<std::sync::Mutex<Vec<(String, semio_framework_async::CancelToken)>>> = std::sync::OnceLock::new();

fn inflight_registry() -> &'static std::sync::Mutex<Vec<(String, semio_framework_async::CancelToken)>> {
    INFERENCE_INFLIGHT.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// 🎛️ Retains one bounded cancellation token for the lifetime of one local inference call.
pub fn retain_inference_operation(label: &str, cancel: semio_framework_async::CancelToken) {
    let mut registry = inflight_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    registry.retain(|(existing, token)| existing.as_str() != label && !token.is_cancelled_now());
    if registry.len() >= INFERENCE_INFLIGHT_CAPACITY {
        registry.remove(0);
    }
    registry.push((label.to_string(), cancel));
}

/// 🧹️ Releases one retained token once its bounded local wait has terminated.
pub fn release_inference_operation(label: &str) {
    inflight_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).retain(|(existing, _)| existing.as_str() != label);
}

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

/// 🏷️ The bounded operation label one local wait is retained under. `None` is the document-wide
/// label a submit or approve waits under before its job id is known — the hub serializes every
/// checked phase on one per-`DocumentScope` gate, so at most one such wait per document is live.
pub fn inference_operation_label(space_id: &str, document_id: &str, job_id: Option<&str>) -> String {
    match job_id {
        Some(job_id) => format!("gis-map:{space_id}/{document_id}/{job_id}"),
        None => format!("gis-map:{space_id}/{document_id}/*"),
    }
}

fn inference_wall_now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

/// 🎲️ A fresh 32-lower-hex client idempotency key. The hub mints the `jobId`; this is only the
/// client's own scoped `(user, authorization generation, space, document, request_id)` key.
pub fn mint_inference_request_id() -> String {
    let entropy = format!("semio.mcp.inference-request/v1\0{}\0{}", inference_wall_now_ms(), crate::handles::mint_id(crate::handles::HandleKind::Job, inference_wall_now_ms()));
    framework_hash::hash_bytes(entropy.as_bytes())[..INFERENCE_REQUEST_ID_HEX_LENGTH].to_string()
}

fn inference_scope_ids() -> Vec<semio_framework::manifest::kernel::CapabilityId> {
    vec![semio_framework::manifest::kernel::CapabilityId("artifacts.read".to_string()), semio_framework::manifest::kernel::CapabilityId("artifacts.write".to_string()), semio_framework::manifest::kernel::CapabilityId("jobs.spawn".to_string())]
}

/// 📝️ The resources one hub inference job capability really writes — a Mutation that names none is
/// a conformance error, and these three name exactly what they touch: the owner's durable hub job
/// row, plus the bound document for the one capability (`inference.approve`) that actually commits.
fn inference_job_writes(id: &str) -> Vec<semio_framework::manifest::ResourceSelector> {
    match id {
        "inference.submit" | "inference.cancel" => vec![semio_framework::manifest::ResourceSelector::new("job:{self}")],
        "inference.approve" => vec![semio_framework::manifest::ResourceSelector::new("job:{self}"), semio_framework::manifest::ResourceSelector::new("artifact:{self}")],
        _ => Vec::new(),
    }
}

fn inference_job_capability(id: &str, tool_name: &str, title: &str, description: &str, kind: CapabilityKind, scopes: Vec<semio_framework::manifest::kernel::CapabilityId>, input_schema: serde_json::Value, output_schema: serde_json::Value) -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind,
        audience: CapabilityAudience::Agent,
        title: title.to_string(),
        description: description.to_string(),
        artifact_kind: None,
        use_when: vec!["run a declared inference over an open document and commit its result".to_string(), "watch, cancel or approve an inference job".to_string()],
        input_schema,
        output_schema,
        effects: semio_framework::manifest::CapabilityEffects { external: true, writes: inference_job_writes(id), ..Default::default() },
        policy: semio_framework::manifest::CapabilityPolicy { scopes, approval: semio_framework::manifest::ApprovalMode::Never },
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: tool_name.to_string() },
        presentation: CapabilityPresentation { icon_id: Some("brain".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

pub fn inference_submit_capability() -> CapabilityDefinition {
    inference_job_capability(
        "inference.submit",
        "inference_submit",
        "Submit Inference Job",
        "Starts one job for an inference service the document's own kind declares (in the plugin's guest, or on the bound hub when the hub executes that service) and returns a session-owned job handle and its first page. Nothing is applied to the document until an approval commits the job's proposal. — Startet einen Auftrag für einen Inferenzdienst, den die Art des Dokuments deklariert (im Gast des Plugins oder auf dem gebundenen Hub, wenn dieser den Dienst ausführt), und liefert ein sitzungsgebundenes Auftrags-Handle mit seiner ersten Seite. Am Dokument wird nichts angewendet, bis eine Genehmigung den Vorschlag des Auftrags festschreibt.",
        CapabilityKind::Mutation,
        inference_scope_ids(),
        inference_submit_input_schema(),
        inference_job_output_schema("inference.submit"),
    )
}

pub fn inference_events_capability() -> CapabilityDefinition {
    inference_job_capability(
        "inference.events",
        "inference_events",
        "Read Inference Job Events",
        "Reads the job's lifecycle events and progress rows after a cursor, with its state, proposal and result. The same page shape for every service and execution site. — Liest die Lebenszyklus-Ereignisse und Fortschrittszeilen des Auftrags nach einem Cursor, mit Zustand, Vorschlag und Ergebnis. Dieselbe Seitenform für jeden Dienst und jeden Ausführungsort.",
        CapabilityKind::Query,
        vec![semio_framework::manifest::kernel::CapabilityId("artifacts.read".to_string())],
        inference_job_handle_input_schema("inference.events"),
        inference_job_output_schema("inference.events"),
    )
}

pub fn inference_cancel_capability() -> CapabilityDefinition {
    inference_job_capability(
        "inference.cancel",
        "inference_cancel",
        "Cancel Inference Job",
        "Cancels a running job, or withdraws the proposal of one awaiting approval. Cancellation never applies anything. — Bricht einen laufenden Auftrag ab oder zieht den Vorschlag eines auf Genehmigung wartenden Auftrags zurück. Ein Abbruch wendet niemals etwas an.",
        CapabilityKind::Mutation,
        vec![semio_framework::manifest::kernel::CapabilityId("jobs.spawn".to_string())],
        inference_job_handle_input_schema("inference.cancel"),
        inference_job_output_schema("inference.cancel"),
    )
}

pub fn inference_approve_capability() -> CapabilityDefinition {
    inference_job_capability(
        "inference.approve",
        "inference_approve",
        "Approve Inference Proposal",
        "Commits one offered proposal, named by its exact hash, as an edit of the document through the normal edit path, so every collaborator sees it and it can be undone. — Schreibt einen angebotenen Vorschlag, benannt durch seinen exakten Hash, als Bearbeitung des Dokuments über den normalen Bearbeitungsweg fest, sodass alle Mitwirkenden ihn sehen und er rückgängig gemacht werden kann.",
        CapabilityKind::Mutation,
        inference_scope_ids(),
        inference_approve_input_schema(),
        inference_job_output_schema("inference.approve"),
    )
}

/// 💡️ The general plugin-declared inference execution capability — the one that is NOT hub-bound.
/// Every inference service any installed plugin declares is reachable through it, routed to that
/// plugin's own guest by `ArtifactInferenceRouter` (`🏠️workspace`'s `PluginArtifactChannel`), so
/// this is the tool that retires the blanket `channel.not-wired` gap. Expensive by construction, so
/// it always mints a job handle: progress is readable with `job_get` and cancellation is requested
/// with `job_cancel`, the same plugin-agnostic registry every other long-running gateway job uses.
pub fn inference_run_capability() -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef("inference.run".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Job,
        audience: CapabilityAudience::Agent,
        title: "Run Declared Inference".to_string(),
        description: "Runs one plugin-declared inference service in its own plugin's guest and returns the guest's own result, plus a job handle whose progress is readable with `job_get` and cancellable with `job_cancel`. — Führt einen von einem Plugin deklarierten Inferenzdienst im Gast dieses Plugins aus und liefert dessen eigenes Ergebnis sowie ein Auftrags-Handle, dessen Fortschritt mit `job_get` gelesen und mit `job_cancel` abgebrochen werden kann.".to_string(),
        artifact_kind: None,
        use_when: vec!["run a plugin's own inference".to_string(), "compute a declared inference value".to_string()],
        input_schema: crate::schema::inference_run_input_schema(),
        output_schema: crate::schema::inference_run_output_schema(),
        effects: Default::default(),
        policy: semio_framework::manifest::CapabilityPolicy { scopes: vec![semio_framework::manifest::kernel::CapabilityId("artifacts.read".to_string()), semio_framework::manifest::kernel::CapabilityId("jobs.spawn".to_string())], approval: semio_framework::manifest::ApprovalMode::Never },
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "inference_run".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("brain".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

/// 💡️ The four inference job capabilities, folded into `CatalogSource.gateway`. `inference_run` is
/// their synchronous sibling in the plugin-declared family ([`inference_capabilities`]): the same
/// service resolution and engine, answered in one call with no proposal.
pub fn inference_job_capabilities() -> Vec<CapabilityDefinition> {
    vec![inference_submit_capability(), inference_events_capability(), inference_cancel_capability(), inference_approve_capability()]
}

//#region 🔖️InferenceServiceResolution
pub fn hub_inference_binding_required(what: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{what}` needs an authenticated hub binding — start this gateway with `--hub <url> --space <id>`"))
        .with_details(serde_json::json!({ "bindWith": ["--hub", "--space"] }))
        .retryable()
}

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
    workspace: Option<&'a Arc<HeadlessWorkspace>>,
    actions: &'a Arc<crate::actions::ActionAdapter>,
    catalog: &'a Arc<Catalog>,
    principal: &'a AgentPrincipal,
    session: crate::handles::SessionHandle,
}

impl InferenceToolContext<'_> {
    fn policy(&self) -> &PolicyEngine {
        self.actions.policy()
    }

    fn handles(&self) -> &crate::handles::HandleTable {
        self.actions.handles().as_ref()
    }

    fn workspace(&self, what: &str) -> Result<&Arc<HeadlessWorkspace>, GatewayError> {
        self.workspace.ok_or_else(|| workspace_binding_required(what))
    }

    /// 🎫️ Resolves one `job_` handle against this connection's own session; a hub job's live subject
    /// must still be the exact subject and authority generation that minted it.
    fn job(&self, workspace: &HeadlessWorkspace, handle: &str) -> Result<InferenceJobHandlePayloadV1, GatewayError> {
        let payload = resolve_inference_job_handle(self.handles(), &self.session, handle, inference_wall_now_ms())?;
        if payload.hub.is_some() {
            check_hub_job_subject(&payload, &workspace.hub_inference_subject()?)?;
        }
        Ok(payload)
    }
}

/// 🎫️ One `job_` handle of THIS session, decoded to the inference job it names.
pub fn resolve_inference_job_handle(handles: &crate::handles::HandleTable, session: &crate::handles::SessionHandle, handle: &str, now_ms: u64) -> Result<InferenceJobHandlePayloadV1, GatewayError> {
    let record = handles.resolve(handle, session, now_ms)?;
    if record.kind != crate::handles::HandleKind::Job {
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
    let registry = crate::ui::job_registry();
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
    validate_inference_request(&service.declared, arguments.get("payload").filter(|value| !value.is_null()), Some(document_id))?;
    let jobs = crate::ui::job_registry();
    let job_id = jobs.begin("inference.submit");
    let cancel = crate::actions::InferenceCancel::default();
    let hook = cancel.0.clone();
    jobs.bind_cancel(&job_id, move || hook.cancel_now());
    retain_guest_job(&job_id, GuestInferenceJob { service: service.clone(), document_id: document_id.to_string(), proposal: None, committed: None, commit_refused: false });
    let payload = InferenceJobHandlePayloadV1 { site: InferenceExecutionSiteV1::Guest, service_id: service.service_id().to_string(), artifact_kind: service.declared.artifact_kind.clone(), plugin_id: service.plugin_id().to_string(), document_id: document_id.to_string(), job_id: job_id.clone(), hub: None };
    let handle = context.handles().mint(crate::handles::HandleKind::Job, context.session.clone(), crate::handles::Attachment::Artifact { artifact_id: document_id.to_string() }, serde_json::to_value(&payload).unwrap_or(serde_json::Value::Null), inference_wall_now_ms());
    spawn_guest_job(Arc::clone(workspace), Arc::clone(context.actions), Arc::clone(context.catalog), service, document_id.to_string(), arguments.clone(), job_id.clone(), cancel)?;
    let page = guest_job_page(&job_id, 0)?;
    Ok((page_text(&page), serde_json::json!({ "jobHandle": handle, "sessionId": context.session.0, "job": page_value(&page) })))
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_guest_job(workspace: Arc<HeadlessWorkspace>, actions: Arc<crate::actions::ActionAdapter>, catalog: Arc<Catalog>, service: InferenceService, document_id: String, arguments: serde_json::Value, job_id: String, cancel: crate::actions::InferenceCancel) -> Result<(), GatewayError> {
    let thread_job = job_id.clone();
    std::thread::Builder::new()
        .name(format!("semio-inference-{job_id}"))
        .spawn(move || run_guest_job(&workspace, &actions, &catalog, &service, &document_id, &arguments, &thread_job, cancel))
        .map(|_| ())
        .map_err(|error| {
            let error = GatewayError::new(GatewayErrorCode::Internal, format!("could not start the inference job thread: {error}"));
            crate::ui::job_registry().fail(&job_id, error.clone());
            error
        })
}

#[cfg(target_arch = "wasm32")]
fn spawn_guest_job(_workspace: Arc<HeadlessWorkspace>, _actions: Arc<crate::actions::ActionAdapter>, _catalog: Arc<Catalog>, _service: InferenceService, _document_id: String, _arguments: serde_json::Value, job_id: String, _cancel: crate::actions::InferenceCancel) -> Result<(), GatewayError> {
    let error = GatewayError::new(GatewayErrorCode::PluginUnavailable, "a guest inference job needs a native gateway with job threads");
    crate::ui::job_registry().fail(&job_id, error.clone());
    Err(error)
}

/// 🏃️ The job thread: run the guest, then either finish with its result, or — for a service that
/// declares a commit action — offer the proposal that action would commit and park for approval.
fn run_guest_job(workspace: &Arc<HeadlessWorkspace>, actions: &crate::actions::ActionAdapter, catalog: &Catalog, service: &InferenceService, document_id: &str, arguments: &serde_json::Value, job_id: &str, cancel: crate::actions::InferenceCancel) {
    let jobs = crate::ui::job_registry();
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
    let result = inference_run_result_value(&run.payload).unwrap_or(serde_json::Value::Null);
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
    let handle = context.handles().mint(crate::handles::HandleKind::Job, context.session.clone(), crate::handles::Attachment::Artifact { artifact_id: document_id.to_string() }, serde_json::to_value(&payload).unwrap_or(serde_json::Value::Null), inference_wall_now_ms());
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
                crate::ui::job_registry().request_cancel(&payload.job_id)?;
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
    let jobs = crate::ui::job_registry();
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

fn commit_guest_proposal(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, document_id: &str, proposal: &InferenceProposalV1) -> Result<crate::schema::InvocationReport, GatewayError> {
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
    let request = crate::actions::InvokeRequest { prepared_handle: Some(prepared.prepared_handle), capability_id: None, input: None, expected_revision: prepared.expected_revision, idempotency_key: Some(format!("inference-{}", proposal.hash)), approval_handle: None };
    let report = context.actions.invoke(context.catalog, context.principal, &context.session, request, instance, inference_wall_now_ms())?;
    if report.status != crate::schema::InvocationStatus::Succeeded {
        return Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("the commit action `{capability_id}` answered {:?}", report.status)));
    }
    Ok(report)
}

#[cfg(not(target_arch = "wasm32"))]
fn commit_instance_slot(catalog: &Catalog, capability_id: &str) -> u32 {
    crate::workspace::capability_instance_slot(catalog, capability_id).unwrap_or(0)
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
fn validate_inference_request(item: &DeclaredInference, payload: Option<&serde_json::Value>, artifact_id: Option<&str>) -> Result<(), GatewayError> {
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
fn resolve_inference_artifact_document(workspace: &Arc<HeadlessWorkspace>, item: &DeclaredInference, artifact_id: Option<&str>, job_id: &str, cancel: &semio_framework_async::CancelToken) -> Result<Option<crate::actions::ArtifactDocumentBinding>, GatewayError> {
    let Some(artifact_id) = artifact_id else { return Ok(None) };
    if item.payload.as_ref().and_then(|contract| contract.artifact_binding.as_ref()).is_none() {
        return Err(inference_field_invalid("artifactId", format!("`{}` publishes no artifact binding, so naming `artifactId` cannot change what it computes — send `payload` instead", item.inference_schema)));
    }
    let job_id = job_id.to_string();
    let scope = crate::actions::ActivationScope::new(cancel.clone(), move |phase, fraction| {
        crate::ui::job_registry().report_progress(&job_id, INFERENCE_BINDING_PROGRESS_START + INFERENCE_BINDING_PROGRESS_SPAN * phase.at(fraction), Some(format!("binding: {}", phase.id())));
    });
    match workspace.bind_artifact_document_cancellably(artifact_id, scope)? {
        Some((pack, spr)) => Ok(Some(crate::actions::ArtifactDocumentBinding { pack, spr })),
        None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no readable document for artifact `{artifact_id}` — open or create it before running an inference on it"))),
    }
}


/// 🏁 One finished guest run: the guest's own result bytes and whether it reported completion.
struct GuestInferenceRun {
    inference_schema: String,
    complete: bool,
    payload: Vec<u8>,
}

/// 🏃️ The one guest engine both `inference_run` and a guest job drive: bind the artifact's document
/// into the request, route it to the service's plugin guest, run it under `cancel`, reporting each
/// step on `job_id`. `Ok(None)` is a run the caller's cancel stopped.
fn run_guest_inference(workspace: &Arc<HeadlessWorkspace>, actions: &crate::actions::ActionAdapter, item: &DeclaredInference, artifact_id: Option<&str>, arguments: &serde_json::Value, job_id: &str, cancel: crate::actions::InferenceCancel) -> Result<Option<GuestInferenceRun>, GatewayError> {
    let jobs = crate::ui::job_registry();
    jobs.report_progress(job_id, INFERENCE_BINDING_PROGRESS_START, Some(match artifact_id {
        Some(artifact_id) => format!("binding artifact `{artifact_id}` into the request body"),
        None => "no artifact binding declared — the caller's own body travels verbatim".to_string(),
    }));
    let artifact_document = match resolve_inference_artifact_document(workspace, item, artifact_id, job_id, &cancel.0) {
        Ok(document) => document,
        Err(_) if jobs.is_cancel_requested(job_id) => return Ok(None),
        Err(error) => return Err(error.with_details(serde_json::json!({ "field": "artifactId" }))),
    };
    let command = crate::actions::InferCommand {
        plugin_id: item.route_plugin_id().to_string(),
        artifact_kind: item.artifact_kind.clone(),
        inference_schema: item.inference_schema.clone(),
        revision: arguments.get("revision").and_then(serde_json::Value::as_u64).unwrap_or(0),
        generation: arguments.get("generation").and_then(serde_json::Value::as_u64).unwrap_or(0),
        cancellation_id: arguments.get("cancellationId").and_then(serde_json::Value::as_str).map_or_else(mint_inference_request_id, str::to_string),
        work_units: arguments.get("workUnits").and_then(serde_json::Value::as_u64).unwrap_or(INFERENCE_DEFAULT_WORK_UNITS),
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
    let cancellation_id = arguments.get("cancellationId").and_then(serde_json::Value::as_str).map(str::to_string).unwrap_or_else(mint_inference_request_id);
    let arguments = merge_inference_run_fields(arguments.clone(), serde_json::json!({ "cancellationId": cancellation_id }));
    let cancel = crate::actions::InferenceCancel::default();

    let jobs = crate::ui::job_registry();
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
        crate::ui::job_registry().mark_cancelled(job_id);
        CallToolResult::ok(vec![ContentBlock::Text { text: format!("inference job {job_id} was cancelled") }], Some(merge_inference_run_fields(base, serde_json::json!({ "status": "CANCELLED", "complete": false }))))
    };
    match run_guest_inference(workspace, context.actions, &item, requested_artifact, &arguments, &job_id, cancel) {
        Ok(Some(_)) | Err(_) if jobs.is_cancel_requested(&job_id) => cancelled(&job_id, base),
        Ok(None) => cancelled(&job_id, base),
        Ok(Some(run)) => {
            let structured = merge_inference_run_fields(base, serde_json::json!({ "status": "SUCCEEDED", "complete": run.complete, "payload": inference_run_result_value(&run.payload), "payloadBytes": run.payload.len() }));
            jobs.succeed(&job_id, structured.clone());
            CallToolResult::ok(vec![ContentBlock::Text { text: format!("`{}` produced {} byte(s) (complete: {})", run.inference_schema, run.payload.len(), run.complete) }], Some(structured))
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

/// ⏱️ The default work-unit budget one `inference_run` grants when the caller names none — the same
/// order of magnitude `job_infer`'s own user-visible lane clamps to.
const INFERENCE_DEFAULT_WORK_UNITS: u64 = 1_024;

/// 🔢️ `RoutingArtifactChannel` resolves an inference's plugin from the command's own `pluginId`, so
/// the `instance` slot the mutation protocol encodes carries no meaning here — a fixed, documented
/// zero rather than a fabricated per-capability slot.
const INFERENCE_ROUTED_INSTANCE: u32 = 0;


/// 💡️ Registers the four inference job tools plus `inference_run`. Like every other tool in this
/// crate they are ALWAYS present in `tools/list`; only a call's result varies by whether a
/// workspace (and, for a hub-executed service, a hub binding) exists, and every call is first gated
/// by the connection's own granted MCP scopes.
pub fn register_inference_job_tools(registry: &mut InMemoryToolRegistry, catalog: Arc<Catalog>, workspace: Option<Arc<HeadlessWorkspace>>, actions: Arc<crate::actions::ActionAdapter>, principal: AgentPrincipal, session: crate::handles::SessionHandle) {
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
#[path = "💼️jobs/🦀️.rs"]
pub mod jobs;
pub use jobs::*;
//#endregion 💼️Jobs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests

//#region 🧪️InferenceJobTests
/// 🧪️ The MCP-side laws for the hub inference bridge. They read the SAME neutral fixture the hub's
/// own Rust laws and Bun/AJV oracle read (`🌎️hub/🧫️fixtures/🗳️gis-map-proposal-approval-v1`), so a
/// hub-side change to the closed vocabulary, limits or lifecycle fails here loudly instead of
/// drifting. Nothing here starts a hub, a model, a renderer or a second process.
#[cfg(test)]
#[path = "🧪️tests/🔬️inference-jobs/🦀️.rs"]
mod inference_jobs;
//#endregion 🧪️InferenceJobTests
