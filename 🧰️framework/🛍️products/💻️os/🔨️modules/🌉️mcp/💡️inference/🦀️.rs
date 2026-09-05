//! 💡️ Inference access — packet W5 of ticket 26/08/29/AI-MCP-END-TO-END. Before this facet there was
//! not one `infer` symbol anywhere under `🌉️mcp/**`: `job_infer`
//! (`🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs`) runs an inference INSIDE a plugin's own wasm
//! guest via its reactor, and `ArtifactInferenceRouter` (`semio_framework_plugin_host`, owned by
//! `🏃️run/🦀️.rs`'s separate `run` process) routes to it there — but this crate's own
//! `HeadlessWorkspace` (`🏠️workspace/🦀️.rs`) is a DIFFERENT process with its own wasmtime
//! activation (`open_artifact_channel`) and its own narrow wire port (`crate::actions::ArtifactChannel`
//! — `AppCommand` has exactly `ReadHistory`/`PureCommand`/`Transaction*`, no infer variant at all,
//! confirmed by reading `🔀️dispatch/🦀️.rs` in full). So an inference cannot be EXECUTED
//! through this crate today — the same `channel.not-wired` class of gap `🏠️workspace`'s own
//! `PureCommand`/`TransactionPrepare` doc already names for mutations pre-W3.
//!
//! What IS real and reachable without touching any of that: every plugin's own committed
//! `🔣️.json` already carries its declared inference roster verbatim —
//! `PackageDescriptor.contributions.inference_services` (`Vec<semio_framework::ContributedInferenceMetadata>`,
//! owner-authored) plus every `artifact_contributions[].inferences` entry (contributed onto a
//! dependency's kind) — the EXACT roster `🏃️run/🦀️.rs`'s own `register_plugin` builds
//! (`descriptor.contributions.inference_services.iter().chain(...artifact_contributions...)`) before
//! handing it to `ArtifactInferenceRouter::register_plugin`. Reading that static roster needs no wasm
//! compile, no activation, no live plugin process — `HeadlessWorkspace::catalog_plugin_ids` +
//! `load_plugin_registry`/`find_plugin_entry`/`load_package_descriptor` (all already `pub fn` on
//! `🏠️workspace`) are enough. This facet is therefore REAL, honest discovery over declared metadata,
//! plus a typed, retryable gap (never a fabricated value) for the one part — execution — this crate
//! genuinely cannot do yet.

use crate::catalog::{CapabilityDefinition, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, ToolExposure};
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::tool_from_capability;
use crate::protocol::{CallToolResult, ContentBlock, GatewayBackend, InMemoryToolRegistry, Resource, ResourceContent, Tool};
use crate::policy::{AgentPrincipal, PolicyEngine};
use crate::workspace::remote::percent_encode;
use crate::workspace::{find_plugin_entry, find_repo_root, load_package_descriptor, load_plugin_registry, HeadlessWorkspace, PROBE_SCHEMA};
use semio_framework_async::OperationContext;
use semio_framework_os_kernel::os_directory::DocumentScope;
use semio_framework_os_kernel::{FromValue, ToValue};
use std::sync::Arc;

//#region 🔖️DeclaredInference
/// 💡️ One declared inference service, wire-shaped 1:1 from `semio_framework::ContributedInferenceMetadata`
/// — the exact static fields a plugin's own `🔣️.json` carries, never a live guest call.
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DeclaredInference {
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub document_schema: String,
    pub document_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub contributor: String,
    pub depends_on: Vec<String>,
}

impl From<&semio_framework::ContributedInferenceMetadata> for DeclaredInference {
    fn from(metadata: &semio_framework::ContributedInferenceMetadata) -> Self {
        Self {
            owner: metadata.owner.clone(),
            artifact_kind: metadata.artifact_kind.clone(),
            artifact_schema: metadata.artifact_schema.clone(),
            artifact_schema_version: metadata.artifact_schema_version,
            document_schema: metadata.document_schema.clone(),
            document_schema_version: metadata.document_schema_version,
            inference_schema: metadata.inference_schema.clone(),
            inference_schema_version: metadata.inference_schema_version,
            algorithm_version: metadata.algorithm_version,
            policy_version: metadata.policy_version,
            contributor: metadata.contributor.clone(),
            depends_on: metadata.depends_on.clone(),
        }
    }
}

/// 💡️ `descriptor.contributions.inference_services` (owner-authored) plus every
/// `artifact_contributions[].inferences` entry (contributed onto a dependency's kind) — the same
/// chain `🏃️run/🦀️.rs`'s `register_plugin` builds for `ArtifactInferenceRouter`.
fn declared_inferences_from_descriptor(descriptor: &semio_framework::PackageDescriptor) -> Vec<DeclaredInference> {
    descriptor.contributions.inference_services.iter().chain(descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter())).map(DeclaredInference::from).collect()
}

/// 💡️ Real, static, plugin-agnostic discovery: the UNION of every registered plugin's own declared
/// inference roster (`workspace.catalog_plugin_ids()` — no plugin id is hardcoded here, no
/// single-plugin assumption; ticket 26/08/29/AI-MCP-END-TO-END packet W8, `📓️w8-capability-routing.md`
/// replaced the OLD `HeadlessWorkspace::resolve_default_plugin_id`-based version, which could only
/// ever read ONE plugin's descriptor and outright FAILED — the same `PLUGIN_UNAVAILABLE` "2+ plugins
/// is ambiguous" defect this whole ticket fixes — the moment a catalog named more than one). Zero
/// registered plugins is still the same typed, retryable `PLUGIN_UNAVAILABLE`; any OTHER plugin's
/// registry/descriptor lookup failing aborts the whole roster rather than silently dropping it.
pub fn declared_inferences_for_workspace(workspace: &HeadlessWorkspace) -> Result<Vec<DeclaredInference>, GatewayError> {
    let plugin_ids = workspace.catalog_plugin_ids();
    if plugin_ids.is_empty() {
        return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "no plugin-owned capability is registered in this workspace's catalog — nothing to read a declared inference roster from").retryable());
    }
    let repo_root = find_repo_root()?;
    let registry = load_plugin_registry(&repo_root)?;
    let mut roster = Vec::new();
    for plugin_id in plugin_ids {
        let entry = find_plugin_entry(&registry, &plugin_id)?;
        let descriptor = load_package_descriptor(&entry.owner_root)?;
        roster.extend(declared_inferences_from_descriptor(&descriptor));
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
/// resolved schema is matched against the workspace-wide roster by `documentSchema`/`artifactSchema`
/// (never by plugin id) — real once a non-probe artifact kind becomes resolvable through this
/// workspace (a `🏠️workspace` gap this facet does not own, see `resolve_artifact_schema`'s own doc).
pub fn declared_inferences_for_artifact(workspace: &HeadlessWorkspace, artifact_id: &str) -> Result<(String, Vec<DeclaredInference>), GatewayError> {
    let schema = resolve_artifact_schema(workspace, artifact_id)?;
    if schema == PROBE_SCHEMA {
        return Ok((schema, Vec::new()));
    }
    let roster = declared_inferences_for_workspace(workspace)?;
    let matches = roster.into_iter().filter(|item| item.document_schema == schema || item.artifact_schema == schema).collect();
    Ok((schema, matches))
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

/// 💡️ The one real gap standing between a declared inference and a live value: retryable, since a
/// later packet wiring a real `artifact-infer` channel command (the inference analogue of W3's
/// mutation wiring) turns this into a real read with zero change to the discovery path above.
fn execution_not_wired_error(item: &DeclaredInference) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{}/{}` is declared but no artifact-infer route is wired through this workspace's plugin channel yet (channel.not-wired)", item.artifact_kind, item.inference_schema))
        .with_details(serde_json::json!({ "artifactKind": item.artifact_kind, "inferenceSchema": item.inference_schema, "owner": item.owner }))
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
fn inference_list_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/inference.list/input",
        "type": "object",
        "properties": { "artifactId": { "type": "string" } },
        "additionalProperties": false,
    })
}

fn inference_list_output_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/inference.list/output",
        "type": "object",
        "properties": { "artifactId": {}, "artifactKind": {}, "declared": { "type": "array" } },
    })
}

fn inference_get_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/inference.get/input",
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "inferenceSchema": { "type": "string" } },
        "required": ["artifactId", "inferenceSchema"],
        "additionalProperties": false,
    })
}

fn inference_get_output_schema() -> serde_json::Value {
    serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "$id": "semio://capability/inference.get/output", "type": "object" })
}

fn inference_capability(id: &str, tool_name: &str, title: &str, description: &str, input_schema: serde_json::Value, output_schema: serde_json::Value) -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Query,
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

/// 💡️ The inference capabilities, folded into `CatalogSource.gateway` by root wiring — same pattern
/// as `🦀️.rs`'s own `core_tool_capabilities()`.
pub fn inference_capabilities() -> Vec<CapabilityDefinition> {
    vec![inference_list_capability(), inference_get_capability()]
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
    if let Some(outcome) = gis_map_hub_inference_read(workspace, artifact_id, inference_schema) {
        return match outcome {
            Ok(body) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("hub GIS Map inference job for {artifact_id}") }], Some(body)),
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
    if let Some(outcome) = field.and_then(|field| gis_map_hub_inference_read(workspace, artifact_id, field)) {
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
/// publish (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs`, `🚀️bin.rs`'s own `//#region 💡️Inference`).
/// Mirrored here as typed Rust rather than reached for as free-form JSON: nothing on this
/// boundary is a `serde_json::Value`, and a hub field this client does not know about is a loud
/// decode failure rather than a silently-dropped one.
pub const GIS_MAP_INFERENCE_SERVICE_ID: &str = "s.gis.gismap.inference";
pub const GIS_MAP_INFERENCE_DOCUMENT_SCHEMA: &str = "gis.map";
pub const GIS_MAP_INFERENCE_ARTIFACT_KIND: &str = "s.gis.gismap";
pub const GIS_MAP_INFERENCE_REQUEST_SCHEMA: &str = "semio.hub.inference-request/v1";
pub const GIS_MAP_INFERENCE_APPROVAL_SCHEMA: &str = "semio.hub.inference-approval/v1";
pub const GIS_MAP_INFERENCE_RECEIPT_SCHEMA: &str = "semio.hub.inference-job-receipt/v1";
pub const GIS_MAP_INFERENCE_EVENTS_SCHEMA: &str = "semio.hub.inference-job-events/v1";
pub const GIS_MAP_INFERENCE_APPROVAL_RECEIPT_SCHEMA: &str = "semio.hub.inference-approval-receipt/v1";
pub const GIS_MAP_INFERENCE_ERROR_SCHEMA: &str = "semio.hub.inference-error/v1";
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
            Self::Unavailable => "this hub publishes no trusted GIS Map inference binding, so all four inference routes fail closed — bind a hub whose readiness reports `features.inference: true`",
            Self::CommitUnavailable => "the hub has no registered atomic parent+existing-child composition transaction, so an approval cannot be published; the prepared proposal is retained and nothing was applied",
            Self::Storage => "the hub's private inference ledger is temporarily unavailable",
            Self::Capacity => "the hub's fixed inference operation capacity is exhausted",
            Self::Denied => "the hub re-checked the live author, session, authorization generation and scope and refused; only the document's current `Author` may run, read, cancel or approve a job, and only its original owner",
            Self::NotFound => "the hub has no such job or document for this authenticated subject",
            Self::Invalid => "the hub rejected this closed client intent",
            Self::Bounds => "the request exceeded the hub's fixed 1024-byte intent bound",
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
pub enum GisMapInferenceJobStateV1 {
    Accepted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GisMapInferenceProposalStateV1 {
    None,
    Offered,
    Approved,
    Stale,
    Cancelled,
}

/// 📥️ The closed client intent `POST …/inference/gis-map/jobs` accepts, byte for byte.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceSubmitRequestV1 {
    pub schema: String,
    pub version: u32,
    pub request_id: String,
    pub service_id: String,
    pub policy_version: u32,
    pub lifetime_ms: u64,
}

impl GisMapInferenceSubmitRequestV1 {
    pub fn new(request_id: impl Into<String>, lifetime_ms: u64) -> Self {
        Self {
            schema: GIS_MAP_INFERENCE_REQUEST_SCHEMA.to_string(),
            version: 1,
            request_id: request_id.into(),
            service_id: GIS_MAP_INFERENCE_SERVICE_ID.to_string(),
            policy_version: INFERENCE_POLICY_VERSION,
            lifetime_ms,
        }
    }

    pub fn validate(&self) -> Result<(), InferenceRouteErrorV1> {
        if self.schema != GIS_MAP_INFERENCE_REQUEST_SCHEMA
            || self.version != 1
            || !is_lower_hex(&self.request_id, INFERENCE_REQUEST_ID_HEX_LENGTH)
            || self.service_id != GIS_MAP_INFERENCE_SERVICE_ID
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
pub struct GisMapInferenceApprovalRequestV1 {
    pub schema: String,
    pub version: u32,
    pub job_id: String,
    pub proposal_hash: String,
}

impl GisMapInferenceApprovalRequestV1 {
    pub fn new(job_id: impl Into<String>, proposal_hash: impl Into<String>) -> Self {
        Self { schema: GIS_MAP_INFERENCE_APPROVAL_SCHEMA.to_string(), version: 1, job_id: job_id.into(), proposal_hash: proposal_hash.into() }
    }

    pub fn validate(&self) -> Result<(), InferenceRouteErrorV1> {
        if self.schema != GIS_MAP_INFERENCE_APPROVAL_SCHEMA || self.version != 1 || !is_lower_hex(&self.job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) || !is_lower_hex(&self.proposal_hash, INFERENCE_PROPOSAL_HASH_HEX_LENGTH) {
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
pub struct GisMapInferenceJobReceiptV1 {
    pub schema: String,
    pub job_id: String,
    pub state: GisMapInferenceJobStateV1,
    pub proposal_state: GisMapInferenceProposalStateV1,
    pub proposal_hash: Option<String>,
    pub cursor: u64,
    pub expires_at_ms: u64,
}

/// 🗓️ One owner-private lifecycle event.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceEventV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
}

/// 📈️ One owner-private progress row.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceProgressV1 {
    pub cursor: u64,
    pub run_epoch: u64,
    pub completed: u64,
    pub total: u64,
    pub at_ms: u64,
}

/// 📃️ The owner-private bounded page one `events` read returns — MCP's only progress channel.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceEventPageV1 {
    pub schema: String,
    pub job_id: String,
    pub state: GisMapInferenceJobStateV1,
    pub proposal_state: GisMapInferenceProposalStateV1,
    pub cancel_requested: bool,
    pub stale: bool,
    pub proposal_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<GisMapInferencePreviewV1>,
    pub events: Vec<GisMapInferenceEventV1>,
    pub progress: Vec<GisMapInferenceProgressV1>,
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
pub struct GisMapInferenceApprovalReceiptV1 {
    pub schema: String,
    pub job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub proposal_hash: String,
    pub applied: bool,
}

/// 🧾️ The two-field closed body every failing inference route publishes.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceErrorBodyV1 {
    pub schema: String,
    pub code: String,
}

/// 🔖️ The one schema string each decodable hub reply must declare for its own shape.
pub trait InferenceHubBodyV1: serde::de::DeserializeOwned {
    const SCHEMA: &'static str;
    fn declared_schema(&self) -> &str;
}

impl InferenceHubBodyV1 for GisMapInferenceJobReceiptV1 {
    const SCHEMA: &'static str = GIS_MAP_INFERENCE_RECEIPT_SCHEMA;
    fn declared_schema(&self) -> &str {
        &self.schema
    }
}

impl InferenceHubBodyV1 for GisMapInferenceEventPageV1 {
    const SCHEMA: &'static str = GIS_MAP_INFERENCE_EVENTS_SCHEMA;
    fn declared_schema(&self) -> &str {
        &self.schema
    }
}

impl InferenceHubBodyV1 for GisMapInferenceApprovalReceiptV1 {
    const SCHEMA: &'static str = GIS_MAP_INFERENCE_APPROVAL_RECEIPT_SCHEMA;
    fn declared_schema(&self) -> &str {
        &self.schema
    }
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

/// 🛣️ The four exact hub paths, percent-encoded per segment, never string-concatenated by a caller.
pub fn gis_map_jobs_path(scope: &DocumentScope) -> String {
    format!("/spaces/{}/documents/{}/inference/gis-map/jobs", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

pub fn gis_map_job_events_path(scope: &DocumentScope, job_id: &str, after: u64) -> String {
    format!("{}/{}/events?after={after}", gis_map_jobs_path(scope), percent_encode(job_id))
}

pub fn gis_map_job_cancel_path(scope: &DocumentScope, job_id: &str) -> String {
    format!("{}/{}/cancel", gis_map_jobs_path(scope), percent_encode(job_id))
}

pub fn gis_map_job_approval_path(scope: &DocumentScope, job_id: &str) -> String {
    format!("{}/{}/approval", gis_map_jobs_path(scope), percent_encode(job_id))
}

/// 🔓️ Decodes one hub reply: a 2xx must be the exact declared schema, anything else resolves
/// through the closed `{schema, code}` body and only falls back to the status when that body is
/// itself undecodable.
pub fn decode_inference_reply<B: InferenceHubBodyV1>(response: &InferenceHubResponseV1) -> Result<B, InferenceRouteErrorV1> {
    if !(200..=299).contains(&response.status) {
        let error = serde_json::from_slice::<GisMapInferenceErrorBodyV1>(&response.body)
            .ok()
            .filter(|body| body.schema == GIS_MAP_INFERENCE_ERROR_SCHEMA)
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

/// 📥️ `POST /spaces/{space}/documents/{document}/inference/gis-map/jobs`.
pub async fn submit_gis_map_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, request: &GisMapInferenceSubmitRequestV1) -> Result<GisMapInferenceJobReceiptV1, InferenceRouteErrorV1> {
    let body = request.encode()?;
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: gis_map_jobs_path(scope), body, maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    decode_inference_reply(&response)
}

/// 📤️ `GET …/jobs/{job}/events?after=<cursor>` — the poll MCP uses in place of a progress push.
pub async fn read_gis_map_job_events<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, job_id: &str, after: u64) -> Result<GisMapInferenceEventPageV1, InferenceRouteErrorV1> {
    if !is_lower_hex(job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) || after > INFERENCE_PROGRESS_MAX_CURSOR {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Get, path: gis_map_job_events_path(scope, job_id, after), body: Vec::new(), maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    let page: GisMapInferenceEventPageV1 = decode_inference_reply(&response)?;
    checked_page(page, job_id)
}

/// 🔒️ Refuses a page whose job id or offered preview does not match what was asked for, so a
/// renderer never receives geometry this gateway did not verify itself.
fn checked_page(page: GisMapInferenceEventPageV1, job_id: &str) -> Result<GisMapInferenceEventPageV1, InferenceRouteErrorV1> {
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
pub async fn cancel_gis_map_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, job_id: &str) -> Result<GisMapInferenceEventPageV1, InferenceRouteErrorV1> {
    if !is_lower_hex(job_id, INFERENCE_REQUEST_ID_HEX_LENGTH) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: gis_map_job_cancel_path(scope, job_id), body: Vec::new(), maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    let page: GisMapInferenceEventPageV1 = decode_inference_reply(&response)?;
    checked_page(page, job_id)
}

/// ✅️ `POST …/jobs/{job}/approval` — explicit approval only; the hub rebuilds the typed effect.
pub async fn approve_gis_map_job<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str, scope: &DocumentScope, request: &GisMapInferenceApprovalRequestV1) -> Result<GisMapInferenceApprovalReceiptV1, InferenceRouteErrorV1> {
    let body = request.encode()?;
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Post, path: gis_map_job_approval_path(scope, &request.job_id), body, maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    decode_inference_reply(&response)
}
//#endregion 💡️InferenceHubClient

//#region 💡️InferenceJobBinding
/// 🧊️ The client-side frozen base one job was submitted against — the P4-C canonical pair mount's
/// own identity (`descriptor_digest_v1`, `active_checkpoint_id`, `etag`, `catalog_generation`) plus
/// its verified baseline `ArtifactFrontier`. It is a LOCAL staleness and display record: the hub
/// re-derives its own binding from server objects at admission and never trusts a client field.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceBaseBindingV1 {
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

/// 🎫️ The payload one `job_` handle carries. The handle is owned by the connection's own
/// `SessionHandle`, so a job id minted by one MCP connection is unreadable by another; the hub then
/// applies the authoritative owner-private check on top of it.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GisMapInferenceJobHandlePayloadV1 {
    pub space_id: String,
    pub document_id: String,
    pub job_id: String,
    pub subject_user_id: String,
    pub authority_generation: u64,
    pub request_id: String,
    pub base: Option<GisMapInferenceBaseBindingV1>,
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
    vec![semio_framework::manifest::kernel::CapabilityId("documents.read".to_string()), semio_framework::manifest::kernel::CapabilityId("documents.write".to_string()), semio_framework::manifest::kernel::CapabilityId("jobs.spawn".to_string())]
}

fn inference_job_capability(id: &str, tool_name: &str, title: &str, description: &str, kind: CapabilityKind, scopes: Vec<semio_framework::manifest::kernel::CapabilityId>, input_schema: serde_json::Value, output_schema: serde_json::Value) -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind,
        title: title.to_string(),
        description: description.to_string(),
        artifact_kind: None,
        use_when: vec!["run the hub's GIS Map inference over a bound document".to_string(), "watch, cancel or approve a hub inference job".to_string()],
        input_schema,
        output_schema,
        effects: semio_framework::manifest::CapabilityEffects { external: true, ..Default::default() },
        policy: semio_framework::manifest::CapabilityPolicy { scopes, approval: semio_framework::manifest::ApprovalMode::Never },
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: tool_name.to_string() },
        presentation: CapabilityPresentation { icon_id: Some("brain".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn inference_submit_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/inference.submit/input",
        "type": "object",
        "properties": { "documentId": { "type": "string" }, "lifetimeMs": { "type": "integer", "minimum": 1, "maximum": INFERENCE_JOB_MAX_LIFETIME_MS }, "requestId": { "type": "string", "pattern": "^[0-9a-f]{32}$" } },
        "required": ["documentId"],
        "additionalProperties": false,
    })
}

fn inference_job_handle_input_schema(id: &str) -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": format!("semio://capability/{id}/input"),
        "type": "object",
        "properties": { "jobHandle": { "type": "string" }, "after": { "type": "integer", "minimum": 0, "maximum": INFERENCE_PROGRESS_MAX_CURSOR } },
        "required": ["jobHandle"],
        "additionalProperties": false,
    })
}

fn inference_approve_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/inference.approve/input",
        "type": "object",
        "properties": { "jobHandle": { "type": "string" }, "proposalHash": { "type": "string", "pattern": "^[0-9a-f]{64}$" } },
        "required": ["jobHandle", "proposalHash"],
        "additionalProperties": false,
    })
}

fn inference_job_output_schema(id: &str) -> serde_json::Value {
    serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "$id": format!("semio://capability/{id}/output"), "type": "object" })
}

pub fn inference_submit_capability() -> CapabilityDefinition {
    inference_job_capability(
        "inference.submit",
        "inference_submit",
        "Submit GIS Map Inference Job",
        "Submits one bounded, deterministic GIS Map inference job to the bound hub and returns its owner-private receipt and a session-owned job handle. Nothing is applied to the document. — Reicht einen begrenzten, deterministischen GIS-Karten-Inferenzauftrag beim gebundenen Hub ein und liefert dessen nur dem Eigentümer sichtbare Quittung sowie ein sitzungsgebundenes Auftrags-Handle. Es wird nichts am Dokument angewendet.",
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
        "Poll GIS Map Inference Job Events",
        "Reads the next owner-private bounded page of lifecycle events and progress rows for one job handle. MCP has no progress push, so poll this cursor. — Liest die nächste, nur dem Eigentümer sichtbare begrenzte Seite mit Lebenszyklus-Ereignissen und Fortschrittszeilen zu einem Auftrags-Handle. MCP kennt keine Fortschrittsmeldung, frage diesen Cursor also ab.",
        CapabilityKind::Query,
        vec![semio_framework::manifest::kernel::CapabilityId("documents.read".to_string())],
        inference_job_handle_input_schema("inference.events"),
        inference_job_output_schema("inference.events"),
    )
}

pub fn inference_cancel_capability() -> CapabilityDefinition {
    inference_job_capability(
        "inference.cancel",
        "inference_cancel",
        "Cancel GIS Map Inference Job",
        "Records the owner's durable cancel request on the hub and interrupts this process's local wait. Cancellation is idempotent and never applies anything. — Vermerkt die dauerhafte Abbruchanforderung des Eigentümers beim Hub und unterbricht das lokale Warten dieses Prozesses. Der Abbruch ist idempotent und wendet niemals etwas an.",
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
        "Approve GIS Map Inference Proposal",
        "Explicitly approves one offered proposal by its exact hash. The hub rebuilds the typed effect and its inverse server-side; `applied` is true only after a real committed-WAL witness. — Genehmigt ausdrücklich einen angebotenen Vorschlag anhand seines exakten Hashes. Der Hub baut die typisierte Wirkung und ihre Umkehrung serverseitig neu auf; `applied` ist nur nach einem echten festgeschriebenen WAL-Zeugen wahr.",
        CapabilityKind::Mutation,
        inference_scope_ids(),
        inference_approve_input_schema(),
        inference_job_output_schema("inference.approve"),
    )
}

/// 💡️ The four hub-backed inference job capabilities, folded into `CatalogSource.gateway`.
pub fn inference_job_capabilities() -> Vec<CapabilityDefinition> {
    vec![inference_submit_capability(), inference_events_capability(), inference_cancel_capability(), inference_approve_capability()]
}

pub fn hub_inference_binding_required(what: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{what}` needs an authenticated hub binding — start this gateway with `--hub <url> --space <id>`; a `--folder` workspace has no inference authority at all"))
        .with_details(serde_json::json!({ "bindWith": ["--hub", "--space"] }))
        .retryable()
}

fn inference_input_invalid(detail: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::InputInvalid, detail.to_string())
}

/// 🔐️ The one local admission gate: the connection's granted MCP scopes must cover the
/// capability's declared scopes. It is never a substitute for hub authority — the hub re-runs
/// `check_live_inference_author` on every phase and only `Author` is ever admitted.
fn authorize_inference(policy: &PolicyEngine, principal: &AgentPrincipal, capability: &CapabilityDefinition) -> Result<(), GatewayError> {
    policy.authorize_scopes(principal, capability)
}

/// 🎫️ Resolves one `job_` handle against the connection's own session and re-checks that the live
/// hub subject is still the exact subject that minted it, so a job is owner-private locally as well
/// as on the hub.
fn resolve_inference_job_handle(handles: &crate::handles::HandleTable, session: &crate::handles::SessionHandle, subject: &HubInferenceSubjectV1, handle: &str, now_ms: u64) -> Result<GisMapInferenceJobHandlePayloadV1, GatewayError> {
    let record = handles.resolve(handle, session, now_ms)?;
    if record.kind != crate::handles::HandleKind::Job {
        return Err(inference_input_invalid("handle is not an inference job handle"));
    }
    let payload: GisMapInferenceJobHandlePayloadV1 = serde_json::from_value(record.payload).map_err(|_| GatewayError::new(GatewayErrorCode::Internal, "inference job handle payload is not a GIS Map job"))?;
    if payload.subject_user_id != subject.user_id || payload.authority_generation != subject.authority_generation || payload.space_id != subject.space_id {
        return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "this inference job belongs to a different authenticated subject or authority generation"));
    }
    Ok(payload)
}

fn inference_receipt_value(receipt: &GisMapInferenceJobReceiptV1) -> serde_json::Value {
    serde_json::to_value(receipt).unwrap_or(serde_json::Value::Null)
}

fn inference_page_value(page: &GisMapInferenceEventPageV1) -> serde_json::Value {
    serde_json::to_value(page).unwrap_or(serde_json::Value::Null)
}

struct InferenceToolContext<'a> {
    workspace: Option<&'a Arc<HeadlessWorkspace>>,
    policy: &'a PolicyEngine,
    handles: &'a crate::handles::HandleTable,
    principal: &'a AgentPrincipal,
    session: crate::handles::SessionHandle,
}

fn inference_submit_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let capability = inference_submit_capability();
    if let Err(error) = authorize_inference(context.policy, context.principal, &capability) {
        return CallToolResult::tool_error(&error);
    }
    let Some(document_id) = arguments.get("documentId").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&inference_input_invalid("documentId is required"));
    };
    let Some(workspace) = context.workspace else {
        return CallToolResult::tool_error(&hub_inference_binding_required("inference_submit"));
    };
    let lifetime_ms = arguments.get("lifetimeMs").and_then(serde_json::Value::as_u64).unwrap_or(INFERENCE_JOB_MAX_LIFETIME_MS);
    let request_id = match arguments.get("requestId").and_then(serde_json::Value::as_str) {
        Some(value) => value.to_string(),
        None => mint_inference_request_id(),
    };
    let subject = match workspace.hub_inference_subject() {
        Ok(subject) => subject,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let request = GisMapInferenceSubmitRequestV1::new(request_id.clone(), lifetime_ms);
    if let Err(error) = request.validate() {
        return CallToolResult::tool_error(&error.to_gateway_error("inference_submit"));
    }
    let (base, base_diagnostic) = match workspace.gis_map_inference_base(document_id) {
        Ok(base) => (Some(base), None),
        Err(error) => (None, Some(error.message.clone())),
    };
    let receipt = match workspace.submit_gis_map_inference_job(document_id, &request) {
        Ok(receipt) => receipt,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let payload = GisMapInferenceJobHandlePayloadV1 {
        space_id: subject.space_id.clone(),
        document_id: document_id.to_string(),
        job_id: receipt.job_id.clone(),
        subject_user_id: subject.user_id.clone(),
        authority_generation: subject.authority_generation,
        request_id,
        base: base.clone(),
    };
    let handle = context.handles.mint(
        crate::handles::HandleKind::Job,
        context.session.clone(),
        crate::handles::Attachment::Artifact { artifact_id: document_id.to_string() },
        serde_json::to_value(&payload).unwrap_or(serde_json::Value::Null),
        inference_wall_now_ms(),
    );
    let structured = serde_json::json!({
        "jobHandle": handle,
        "receipt": inference_receipt_value(&receipt),
        "baseBinding": base,
        "baseBindingUnavailable": base_diagnostic,
        "sessionId": context.session.0,
    });
    CallToolResult::ok(vec![ContentBlock::Text { text: format!("inference job {} is {:?} / {:?}", receipt.job_id, receipt.state, receipt.proposal_state) }], Some(structured))
}

fn inference_events_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let capability = inference_events_capability();
    if let Err(error) = authorize_inference(context.policy, context.principal, &capability) {
        return CallToolResult::tool_error(&error);
    }
    let Some(handle) = arguments.get("jobHandle").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&inference_input_invalid("jobHandle is required"));
    };
    let Some(workspace) = context.workspace else {
        return CallToolResult::tool_error(&hub_inference_binding_required("inference_events"));
    };
    let after = arguments.get("after").and_then(serde_json::Value::as_u64).unwrap_or(0);
    let subject = match workspace.hub_inference_subject() {
        Ok(subject) => subject,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let payload = match resolve_inference_job_handle(context.handles, &context.session, &subject, handle, inference_wall_now_ms()) {
        Ok(payload) => payload,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    match workspace.read_gis_map_inference_job_events(&payload.document_id, &payload.job_id, after) {
        Ok(page) => {
            let text = format!("job {} is {:?} / {:?}, {} event(s), {} progress row(s), next cursor {}", page.job_id, page.state, page.proposal_state, page.events.len(), page.progress.len(), page.next_cursor);
            CallToolResult::ok(vec![ContentBlock::Text { text }], Some(serde_json::json!({ "jobHandle": handle, "page": inference_page_value(&page), "baseBinding": payload.base })))
        }
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn inference_cancel_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let capability = inference_cancel_capability();
    if let Err(error) = authorize_inference(context.policy, context.principal, &capability) {
        return CallToolResult::tool_error(&error);
    }
    let Some(handle) = arguments.get("jobHandle").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&inference_input_invalid("jobHandle is required"));
    };
    let Some(workspace) = context.workspace else {
        return CallToolResult::tool_error(&hub_inference_binding_required("inference_cancel"));
    };
    let subject = match workspace.hub_inference_subject() {
        Ok(subject) => subject,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let payload = match resolve_inference_job_handle(context.handles, &context.session, &subject, handle, inference_wall_now_ms()) {
        Ok(payload) => payload,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let interrupted = interrupt_inference_operation(&inference_operation_label(&payload.space_id, &payload.document_id, Some(&payload.job_id))) | interrupt_inference_operation(&inference_operation_label(&payload.space_id, &payload.document_id, None));
    match workspace.cancel_gis_map_inference_job(&payload.document_id, &payload.job_id) {
        Ok(page) => {
            let text = format!("job {} cancel requested: {} (local wait interrupted: {interrupted})", page.job_id, page.cancel_requested);
            CallToolResult::ok(vec![ContentBlock::Text { text }], Some(serde_json::json!({ "jobHandle": handle, "page": inference_page_value(&page), "localWaitInterrupted": interrupted })))
        }
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn inference_approve_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let capability = inference_approve_capability();
    if let Err(error) = authorize_inference(context.policy, context.principal, &capability) {
        return CallToolResult::tool_error(&error);
    }
    let Some(handle) = arguments.get("jobHandle").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&inference_input_invalid("jobHandle is required"));
    };
    let Some(proposal_hash) = arguments.get("proposalHash").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&inference_input_invalid("proposalHash is required"));
    };
    let Some(workspace) = context.workspace else {
        return CallToolResult::tool_error(&hub_inference_binding_required("inference_approve"));
    };
    let subject = match workspace.hub_inference_subject() {
        Ok(subject) => subject,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let payload = match resolve_inference_job_handle(context.handles, &context.session, &subject, handle, inference_wall_now_ms()) {
        Ok(payload) => payload,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let request = GisMapInferenceApprovalRequestV1::new(payload.job_id.clone(), proposal_hash.to_string());
    if let Err(error) = request.validate() {
        return CallToolResult::tool_error(&error.to_gateway_error("inference_approve"));
    }
    match workspace.approve_gis_map_inference_job(&payload.document_id, &request) {
        Ok(receipt) => {
            let text = format!("approval of job {} produced mutation {} (applied: {})", receipt.job_id, receipt.mutation_id, receipt.applied);
            CallToolResult::ok(vec![ContentBlock::Text { text }], Some(serde_json::json!({ "jobHandle": handle, "receipt": serde_json::to_value(&receipt).unwrap_or(serde_json::Value::Null), "baseBinding": payload.base })))
        }
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// 💡️ Registers the four hub-backed inference job tools. Like every other tool in this crate they
/// are ALWAYS present in `tools/list`; only a call's result varies by whether an authenticated hub
/// binding exists, and every call is first gated by the connection's own granted MCP scopes.
pub fn register_inference_job_tools(registry: &mut InMemoryToolRegistry, workspace: Option<Arc<HeadlessWorkspace>>, actions: Arc<crate::actions::ActionAdapter>, principal: AgentPrincipal, session: crate::handles::SessionHandle) {
    let definitions: [(CapabilityDefinition, &str, fn(&InferenceToolContext<'_>, serde_json::Value) -> CallToolResult); 4] = [
        (inference_submit_capability(), "inference_submit", inference_submit_handler),
        (inference_events_capability(), "inference_events", inference_events_handler),
        (inference_cancel_capability(), "inference_cancel", inference_cancel_handler),
        (inference_approve_capability(), "inference_approve", inference_approve_handler),
    ];
    for (capability, tool_name, handler) in definitions {
        let tool = tool_from_capability(&capability, tool_name);
        let (tool_workspace, tool_actions, tool_principal, tool_session) = (workspace.clone(), actions.clone(), principal.clone(), session.clone());
        registry
            .register(tool, move |arguments| {
                let context = InferenceToolContext { workspace: tool_workspace.as_ref(), policy: tool_actions.policy(), handles: tool_actions.handles().as_ref(), principal: &tool_principal, session: tool_session.clone() };
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

/// 💡️ The hub-backed replacement for `channel.not-wired` on the ONE inference service this gateway
/// can really execute today. It never touches this crate's local `ArtifactChannel` — which still
/// has no infer variant, so every OTHER declared inference keeps answering the same honest,
/// retryable gap — it calls the hub's own authenticated `POST …/inference/gis-map/jobs` route and
/// returns only the owner-private receipt the hub hands back to this exact subject. `None` means
/// "not ours", so the caller falls through to the unchanged discovery path.
pub fn gis_map_hub_inference_read(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str, inference_schema: &str) -> Option<Result<serde_json::Value, GatewayError>> {
    if inference_schema != GIS_MAP_INFERENCE_SERVICE_ID {
        return None;
    }
    let subject = match workspace.hub_inference_subject() {
        Ok(subject) => subject,
        Err(error) => return Some(Err(error)),
    };
    let request = GisMapInferenceSubmitRequestV1::new(deterministic_inference_request_id(&subject, artifact_id), INFERENCE_JOB_MAX_LIFETIME_MS);
    Some(workspace.submit_gis_map_inference_job(artifact_id, &request).map(|receipt| {
        serde_json::json!({
            "artifactId": artifact_id,
            "inferenceSchema": inference_schema,
            "subjectUserId": subject.user_id,
            "authorityGeneration": subject.authority_generation,
            "receipt": inference_receipt_value(&receipt),
        })
    }))
}
//#endregion 💡️InferenceHubRead

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::protocol::ToolRegistry;
    use crate::catalog::{compile, Catalog, CatalogSource};
    use crate::protocol::is_valid_tool_name;
    use semio_framework::{Locale, Terminology};

    fn empty_catalog() -> Arc<Catalog> {
        Arc::new(compile(&CatalogSource::default(), Locale::En, Terminology::Native).expect("empty catalog compiles"))
    }

    fn procedural_only_catalog() -> Arc<Catalog> {
        plugin_only_catalog("procedural")
    }

    fn plugin_only_catalog(plugin_id: &str) -> Arc<Catalog> {
        let capability = CapabilityDefinition {
            id: CapabilityRef(format!("{plugin_id}.probe")),
            version: 1,
            owner: CapabilityOwner::Plugin { plugin_id: plugin_id.to_string(), app_id: None, window_kind_id: None, mode_id: None },
            kind: CapabilityKind::Query,
            title: "Probe".to_string(),
            description: "test fixture".to_string(),
            artifact_kind: None,
            use_when: Vec::new(),
            input_schema: serde_json::json!({ "type": "object" }),
            output_schema: serde_json::json!({ "type": "object" }),
            effects: Default::default(),
            policy: Default::default(),
            execution: Default::default(),
            exposure: ToolExposure::CatalogOnly,
            presentation: CapabilityPresentation { icon_id: None, category: None, keys: None, in_palette: false, args: Vec::new() },
            examples: Vec::new(),
            source: CapabilitySource::Gateway,
        };
        Arc::new(compile(&CatalogSource { gateway: vec![capability], ..Default::default() }, Locale::En, Terminology::Native).expect("single gateway capability compiles"))
    }

    fn open_workspace(catalog: Arc<Catalog>) -> Arc<HeadlessWorkspace> {
        let dir = store::test_support::tempdir().expect("tempdir");
        Arc::new(HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), catalog).expect("opens"))
    }

    //#region 🧪️Capabilities
    #[test]
    fn capability_schemas_are_2020_12_object_typed_at_the_top_level() {
        for capability in inference_capabilities() {
            assert_eq!(capability.input_schema["type"], "object", "{}", capability.id);
            assert_eq!(capability.input_schema["$schema"], "https://json-schema.org/draft/2020-12/schema");
            assert_eq!(capability.output_schema["type"], "object", "{}", capability.id);
            assert_eq!(capability.output_schema["$schema"], "https://json-schema.org/draft/2020-12/schema");
        }
    }

    #[test]
    fn tool_names_are_valid_mcp_names() {
        for capability in inference_capabilities() {
            let ToolExposure::Direct { tool_name } = &capability.exposure else { panic!("every inference capability is Direct") };
            assert!(is_valid_tool_name(tool_name), "{tool_name}");
        }
    }
    //#endregion 🧪️Capabilities

    //#region 🧪️ToolsBareTier
    #[test]
    fn bare_registry_still_registers_both_inference_tools() {
        let mut registry = InMemoryToolRegistry::new();
        register_inference_tools(&mut registry, None);
        let names: Vec<String> = registry.list().into_iter().map(|tool| tool.name).collect();
        assert!(names.contains(&"inference_list".to_string()));
        assert!(names.contains(&"inference_get".to_string()));
    }

    #[test]
    fn bare_tier_every_inference_tool_is_a_retryable_plugin_unavailable() {
        let mut registry = InMemoryToolRegistry::new();
        register_inference_tools(&mut registry, None);
        for (name, arguments) in [("inference_list", serde_json::json!({})), ("inference_get", serde_json::json!({ "artifactId": "a", "inferenceSchema": "s" }))] {
            let result = registry.call(name, arguments).expect("registered tool");
            assert!(result.is_error, "{name} must fail with no workspace bound");
            let payload = result.structured_content.expect("structured error payload");
            assert_eq!(payload["code"], "PLUGIN_UNAVAILABLE", "{name}");
            assert_eq!(payload["retryable"], true, "{name}");
        }
    }
    //#endregion 🧪️ToolsBareTier

    //#region 🧪️Discovery
    #[test]
    fn gis_inference_discovery_reads_committed_descriptor_through_registered_mcp_tool_without_execution_authority() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🗺️gis-discovery/🔣️.json")).expect("neutral GIS discovery fixture");
        let workspace = open_workspace(plugin_only_catalog(fixture["pluginId"].as_str().unwrap()));
        let mut registry = InMemoryToolRegistry::new();
        register_inference_tools(&mut registry, Some(workspace));
        let result = registry.call(fixture["tool"].as_str().unwrap(), fixture["arguments"].clone()).expect("registered MCP discovery tool");
        assert!(!result.is_error, "committed GIS descriptor must load");
        assert_eq!(result.structured_content.unwrap(), fixture["expected"], "the exact committed descriptor, not source scraping, supplies the GIS roster");
        let denied = registry.call("inference_get", serde_json::json!({ "artifactId": fixture["unboundArtifact"], "inferenceSchema": fixture["expected"]["declared"][0]["inferenceSchema"] })).expect("registered MCP inference tool");
        assert!(denied.is_error, "metadata discovery must not create execution authority");
        let error = denied.structured_content.unwrap();
        assert_eq!(error["code"], fixture["executionError"]);
        assert_eq!(error["retryable"], true);
    }

    #[test]
    fn declared_inferences_for_workspace_finds_the_real_procedural_roster() {
        let workspace = open_workspace(procedural_only_catalog());
        let declared = declared_inferences_for_workspace(&workspace).expect("procedural is the sole plugin owner");
        assert_eq!(declared.len(), 1, "{declared:?}");
        assert_eq!(declared[0].owner, "procedural");
        assert_eq!(declared[0].artifact_kind, "s.assembly");
        assert_eq!(declared[0].inference_schema, "s.assembly.solve");
    }

    #[test]
    fn declared_inferences_for_workspace_is_plugin_unavailable_for_an_empty_catalog() {
        let workspace = open_workspace(empty_catalog());
        let error = declared_inferences_for_workspace(&workspace).expect_err("no plugin owner");
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable);
    }

    #[tokio::test]
    async fn declared_inferences_for_artifact_is_empty_for_an_open_probe() {
        let workspace = open_workspace(procedural_only_catalog());
        workspace.ensure_probe_artifact("probe-inf", serde_json::json!({ "n": 1 })).await.expect("seed");
        let (schema, declared) = declared_inferences_for_artifact(&workspace, "probe-inf").expect("probe schema resolves");
        assert_eq!(schema, PROBE_SCHEMA);
        assert!(declared.is_empty(), "no plugin declares an inference against this crate's own probe schema");
    }

    #[test]
    fn declared_inferences_for_artifact_is_retryable_plugin_unavailable_for_an_unknown_id() {
        let workspace = open_workspace(procedural_only_catalog());
        let error = declared_inferences_for_artifact(&workspace, "does-not-exist").expect_err("never seen — same gap as 🏠️workspace's own /schema arm");
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable);
    }
    #[tokio::test]
    async fn inference_get_on_an_open_probe_names_the_missing_service_not_found() {
        let workspace = open_workspace(procedural_only_catalog());
        workspace.ensure_probe_artifact("probe-get", serde_json::json!({ "n": 1 })).await.expect("seed");
        let mut registry = InMemoryToolRegistry::new();
        register_inference_tools(&mut registry, Some(workspace));
        let result = registry.call("inference_get", serde_json::json!({ "artifactId": "probe-get", "inferenceSchema": "s.assembly.solve" })).expect("registered tool");
        assert!(result.is_error);
        let payload = result.structured_content.expect("structured error payload");
        assert_eq!(payload["code"], "NOT_FOUND");
    }
    //#endregion 🧪️Discovery

    //#region 🧪️ExecutionSeam
    #[test]
    fn lookup_inference_distinguishes_no_such_service_from_execute() {
        let declared = vec![DeclaredInference {
            owner: "procedural".to_string(),
            artifact_kind: "s.assembly".to_string(),
            artifact_schema: "s.assembly".to_string(),
            artifact_schema_version: 1,
            document_schema: "s.assembly".to_string(),
            document_schema_version: 1,
            inference_schema: "s.assembly.solve".to_string(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            contributor: "procedural".to_string(),
            depends_on: Vec::new(),
        }];
        assert!(matches!(lookup_inference(&declared, "s.assembly.solve"), InferenceLookup::Execute(_)));
        assert!(matches!(lookup_inference(&declared, "no.such.schema"), InferenceLookup::NoSuchService));
    }

    #[test]
    fn execute_lookup_reports_a_retryable_channel_not_wired_gap() {
        let item = DeclaredInference {
            owner: "procedural".to_string(),
            artifact_kind: "s.assembly".to_string(),
            artifact_schema: "s.assembly".to_string(),
            artifact_schema_version: 1,
            document_schema: "s.assembly".to_string(),
            document_schema_version: 1,
            inference_schema: "s.assembly.solve".to_string(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            contributor: "procedural".to_string(),
            depends_on: Vec::new(),
        };
        let error = execution_not_wired_error(&item);
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable);
        let payload = inference_job_payload("art-1", &item, "cancel-1");
        assert_eq!(payload.artifact_kind, "s.assembly");
        assert_eq!(payload.inference_schema, "s.assembly.solve");
    }
    //#endregion 🧪️ExecutionSeam

    //#region 🧪️Resources
    #[test]
    fn a_non_inference_uri_falls_through_as_none() {
        for uri in ["semio://workspace", "semio://artifact/a", "semio://artifact/a/schema", "semio://artifact/a/history"] {
            assert!(read_inference_resource(uri, None).is_none(), "{uri}");
        }
    }

    #[test]
    fn a_malformed_inference_uri_is_a_well_formed_not_found() {
        for uri in ["semio://artifact//inference", "semio://artifact/a/inference/", "semio://artifact/a/inference/field/extra"] {
            let result = read_inference_resource(uri, None).unwrap_or_else(|| panic!("{uri} must be recognized as ours"));
            let error = result.expect_err("malformed");
            assert_eq!(error.code, GatewayErrorCode::NotFound, "{uri}");
        }
    }

    #[test]
    fn bare_tier_inference_index_read_is_retryable_plugin_unavailable() {
        let result = read_inference_resource("semio://artifact/a/inference", None).expect("ours");
        let error = result.expect_err("no workspace bound");
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable);
    }

    #[tokio::test]
    async fn bound_tier_inference_index_read_lists_the_real_declared_roster() {
        let workspace = open_workspace(procedural_only_catalog());
        workspace.ensure_probe_artifact("probe-idx", serde_json::json!({ "n": 1 })).await.expect("seed");
        let result = read_inference_resource("semio://artifact/probe-idx/inference", Some(&workspace)).expect("ours");
        let contents = result.expect("bound workspace resolves");
        let body: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().unwrap()).unwrap();
        assert_eq!(body["artifactKind"], PROBE_SCHEMA);
        assert_eq!(body["declared"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn bare_tier_inference_resources_list_is_empty() {
        assert!(inference_resources(None).is_empty());
    }

    #[tokio::test]
    async fn bound_tier_inference_resources_list_names_every_known_artifact() {
        let workspace = open_workspace(procedural_only_catalog());
        workspace.ensure_probe_artifact("probe-list", serde_json::json!({ "n": 1 })).await.expect("seed");
        let resources = inference_resources(Some(&workspace));
        assert!(resources.iter().any(|resource| resource.uri == "semio://artifact/probe-list/inference"));
    }
    //#endregion 🧪️Resources
}
//#endregion 🧪️Tests

//#region 🧪️InferenceJobTests
/// 🧪️ The MCP-side laws for the hub inference bridge. They read the SAME neutral fixture the hub's
/// own Rust laws and Bun/AJV oracle read (`🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1`), so a
/// hub-side change to the closed vocabulary, limits or lifecycle fails here loudly instead of
/// drifting. Nothing here starts a hub, a model, a renderer or a second process.
#[cfg(test)]
mod inference_jobs {
    use super::*;
    use semio_framework_async::{CancelToken, TraceId};

    const FIXTURE: &str = include_str!("../../../../../../🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json");

    fn fixture() -> serde_json::Value {
        serde_json::from_str(FIXTURE).expect("the neutral gis-map-proposal-approval fixture parses")
    }

    fn sample_job_id() -> String {
        fixture()["sampleJobId"].as_str().expect("sampleJobId").to_string()
    }

    fn sample_proposal_hash() -> String {
        fixture()["proposalHash"].as_str().expect("proposalHash").to_string()
    }

    fn context(cancel: &CancelToken) -> OperationContext {
        OperationContext { actor: 1, generation: 0, trace: TraceId(1), lane: 1, deadline_ms: Some(u64::MAX), cancel: cancel.child_now(), capability: None }
    }

    fn scope() -> DocumentScope {
        DocumentScope::new("space:alpha".to_string(), "doc:tokyo".to_string())
    }

    /// 🎭️ A scripted transport: it records every request it is handed and replays one canned reply,
    /// so the client's own decoding, bounds and error mapping are exercised with no hub at all.
    struct ScriptedTransport {
        replies: std::sync::Mutex<Vec<Result<InferenceHubResponseV1, InferenceHubTransportErrorV1>>>,
        seen: std::sync::Mutex<Vec<InferenceHubRequestV1>>,
    }

    impl ScriptedTransport {
        fn new(replies: Vec<Result<InferenceHubResponseV1, InferenceHubTransportErrorV1>>) -> Self {
            Self { replies: std::sync::Mutex::new(replies), seen: std::sync::Mutex::new(Vec::new()) }
        }

        fn ok(status: u16, body: serde_json::Value) -> Self {
            Self::new(vec![Ok(InferenceHubResponseV1 { status, body: serde_json::to_vec(&body).expect("scripted body") })])
        }

        fn requests(&self) -> Vec<InferenceHubRequestV1> {
            self.seen.lock().expect("scripted transport lock").clone()
        }
    }

    impl InferenceHubTransport for ScriptedTransport {
        async fn request(&self, context: &OperationContext, request: &InferenceHubRequestV1) -> Result<InferenceHubResponseV1, InferenceHubTransportErrorV1> {
            if context.cancel.is_cancelled_now() {
                return Err(InferenceHubTransportErrorV1::Cancelled);
            }
            self.seen.lock().expect("scripted transport lock").push(request.clone());
            let mut replies = self.replies.lock().expect("scripted transport lock");
            if replies.is_empty() {
                return Err(InferenceHubTransportErrorV1::Unavailable);
            }
            replies.remove(0)
        }
    }

    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread().enable_all().build().expect("current-thread runtime").block_on(future)
    }

    //#region 🧪️Vocabulary
    #[test]
    fn the_published_error_vocabulary_is_exactly_the_neutral_fixtures_and_status_alone_is_ambiguous() {
        let fixture = fixture();
        let rows = fixture["errors"].as_array().expect("error vocabulary");
        assert_eq!(rows.len(), INFERENCE_ROUTE_ERRORS.len(), "the closed vocabulary drifted from the neutral corpus");
        for row in rows {
            let code = row["code"].as_str().expect("code");
            let status = row["status"].as_u64().expect("status");
            let published = InferenceRouteErrorV1::from_code(code).unwrap_or_else(|| panic!("{code} is not a published inference route code"));
            assert_eq!(u64::from(published.status()), status, "{code}");
        }
        for published in INFERENCE_ROUTE_ERRORS {
            assert!(rows.iter().any(|row| row["code"] == published.code()), "{} is published but not pinned by the corpus", published.code());
        }
        assert_eq!(InferenceRouteErrorV1::from_status(503), InferenceRouteErrorV1::Unavailable, "503 is shared by three codes, so the widest honest member is the only safe fallback");
        assert_eq!(InferenceRouteErrorV1::from_status(409), InferenceRouteErrorV1::Conflict);
        assert_eq!(InferenceRouteErrorV1::from_status(418), InferenceRouteErrorV1::Unavailable);
    }

    #[test]
    fn a_503_inference_unavailable_becomes_a_retryable_plugin_unavailable_that_names_the_missing_binding() {
        let error = InferenceRouteErrorV1::Unavailable.to_gateway_error("inference_submit");
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable, "a hub without a trusted binding is a tier, not a permanent failure");
        assert!(error.message.contains("inference.unavailable"), "{}", error.message);
        assert!(error.message.contains("features.inference"), "the caller must learn which binding is missing: {}", error.message);
        assert_eq!(error.details["httpStatus"], 503);
        assert_eq!(error.details["inferenceCode"], "inference.unavailable");

        let commit = InferenceRouteErrorV1::CommitUnavailable.to_gateway_error("inference_approve");
        assert_eq!(commit.code, GatewayErrorCode::PluginUnavailable);
        assert!(commit.retryable);
        assert!(commit.message.contains("approval.commit-unavailable") && commit.message.contains("nothing was applied"), "{}", commit.message);

        assert_eq!(InferenceRouteErrorV1::Denied.to_gateway_error("x").code, GatewayErrorCode::PermissionDenied);
        assert!(!InferenceRouteErrorV1::Denied.to_gateway_error("x").retryable);
        assert_eq!(InferenceRouteErrorV1::NotFound.to_gateway_error("x").code, GatewayErrorCode::NotFound);
        assert_eq!(InferenceRouteErrorV1::Invalid.to_gateway_error("x").code, GatewayErrorCode::InputInvalid);
        assert_eq!(InferenceRouteErrorV1::Bounds.to_gateway_error("x").code, GatewayErrorCode::BudgetExceeded);
        assert_eq!(InferenceRouteErrorV1::Conflict.to_gateway_error("x").code, GatewayErrorCode::PreconditionFailed);
        assert_eq!(InferenceRouteErrorV1::Expired.to_gateway_error("x").code, GatewayErrorCode::PreconditionFailed);
        assert_eq!(InferenceRouteErrorV1::Cancelled.to_gateway_error("x").code, GatewayErrorCode::Cancelled);
        assert_eq!(InferenceRouteErrorV1::Storage.to_gateway_error("x").code, GatewayErrorCode::PluginUnavailable);
        assert_eq!(InferenceRouteErrorV1::Capacity.to_gateway_error("x").code, GatewayErrorCode::PluginUnavailable);
    }

    #[test]
    fn the_client_mirrors_the_neutral_fixtures_exact_fixed_limits() {
        let fixture = fixture();
        let limits = &fixture["limits"];
        assert_eq!(limits["requestMaxBytes"], INFERENCE_REQUEST_MAX_BYTES as u64);
        assert_eq!(limits["jobMaxLifetimeMs"], INFERENCE_JOB_MAX_LIFETIME_MS);
        assert_eq!(limits["progressMaxCursor"], INFERENCE_PROGRESS_MAX_CURSOR);
        assert_eq!(limits["eventPageMaxItems"], INFERENCE_EVENT_PAGE_MAX_ITEMS as u64);
        assert_eq!(fixture["binding"]["serviceId"], GIS_MAP_INFERENCE_SERVICE_ID);
        assert_eq!(fixture["binding"]["documentSchema"], GIS_MAP_INFERENCE_DOCUMENT_SCHEMA);
        assert_eq!(fixture["binding"]["artifactKind"], GIS_MAP_INFERENCE_ARTIFACT_KIND);
    }
    //#endregion 🧪️Vocabulary

    //#region 🧪️WireShapes
    #[test]
    fn a_submit_intent_encodes_within_the_fixed_bound_and_every_hostile_field_is_refused() {
        let request = GisMapInferenceSubmitRequestV1::new(sample_job_id(), INFERENCE_JOB_MAX_LIFETIME_MS);
        let encoded = request.encode().expect("a well-formed intent encodes");
        assert!(encoded.len() <= INFERENCE_REQUEST_MAX_BYTES);
        let decoded: GisMapInferenceSubmitRequestV1 = serde_json::from_slice(&encoded).expect("closed round trip");
        assert_eq!(decoded, request);

        let hostile: Vec<(&str, Box<dyn Fn(&mut GisMapInferenceSubmitRequestV1)>)> = vec![
            ("wrong-schema", Box::new(|value: &mut GisMapInferenceSubmitRequestV1| value.schema = "semio.hub.inference-request/v2".into())),
            ("wrong-version", Box::new(|value: &mut GisMapInferenceSubmitRequestV1| value.version = 2)),
            ("non-hex-request-id", Box::new(|value: &mut GisMapInferenceSubmitRequestV1| value.request_id = "Z".repeat(32))),
            ("short-request-id", Box::new(|value: &mut GisMapInferenceSubmitRequestV1| value.request_id.truncate(31))),
            ("foreign-service", Box::new(|value: &mut GisMapInferenceSubmitRequestV1| value.service_id = "s.gis.gismap.other".into())),
            ("wrong-policy", Box::new(|value: &mut GisMapInferenceSubmitRequestV1| value.policy_version = 2)),
            ("zero-lifetime", Box::new(|value: &mut GisMapInferenceSubmitRequestV1| value.lifetime_ms = 0)),
            ("over-lifetime", Box::new(|value: &mut GisMapInferenceSubmitRequestV1| value.lifetime_ms = INFERENCE_JOB_MAX_LIFETIME_MS + 1)),
        ];
        for (name, mutate) in &hostile {
            let mut candidate = request.clone();
            mutate(&mut candidate);
            assert_eq!(candidate.validate(), Err(InferenceRouteErrorV1::Invalid), "{name} was admitted");
        }
        assert!(serde_json::from_str::<GisMapInferenceSubmitRequestV1>(&format!("{{\"schema\":\"{GIS_MAP_INFERENCE_REQUEST_SCHEMA}\",\"version\":1,\"requestId\":\"{}\",\"serviceId\":\"{GIS_MAP_INFERENCE_SERVICE_ID}\",\"policyVersion\":1,\"lifetimeMs\":1,\"mapPack\":\"smuggled\"}}", sample_job_id())).is_err(), "a client may never smuggle an extra field past the closed intent");
    }

    #[test]
    fn an_approval_intent_carries_only_the_job_and_its_exact_proposal_digest() {
        let request = GisMapInferenceApprovalRequestV1::new(sample_job_id(), sample_proposal_hash());
        assert!(request.encode().is_ok());
        for (name, mutate) in [("short-hash", 63_usize), ("long-hash", 65)] {
            let mut candidate = request.clone();
            candidate.proposal_hash = "a".repeat(mutate);
            assert_eq!(candidate.validate(), Err(InferenceRouteErrorV1::Invalid), "{name} was admitted");
        }
        let mut foreign_job = request.clone();
        foreign_job.job_id = "not-hex-at-all".into();
        assert_eq!(foreign_job.validate(), Err(InferenceRouteErrorV1::Invalid));
        assert!(serde_json::from_str::<GisMapInferenceApprovalRequestV1>(&format!("{{\"schema\":\"{GIS_MAP_INFERENCE_APPROVAL_SCHEMA}\",\"version\":1,\"jobId\":\"{}\",\"proposalHash\":\"{}\",\"actor\":\"user:forged\"}}", sample_job_id(), sample_proposal_hash())).is_err());
    }

    #[test]
    fn the_four_client_paths_are_exact_percent_encoded_hub_paths() {
        let scope = scope();
        let job = sample_job_id();
        assert_eq!(gis_map_jobs_path(&scope), "/spaces/space%3Aalpha/documents/doc%3Atokyo/inference/gis-map/jobs");
        assert_eq!(gis_map_job_events_path(&scope, &job, 4), format!("/spaces/space%3Aalpha/documents/doc%3Atokyo/inference/gis-map/jobs/{job}/events?after=4"));
        assert_eq!(gis_map_job_cancel_path(&scope, &job), format!("/spaces/space%3Aalpha/documents/doc%3Atokyo/inference/gis-map/jobs/{job}/cancel"));
        assert_eq!(gis_map_job_approval_path(&scope, &job), format!("/spaces/space%3Aalpha/documents/doc%3Atokyo/inference/gis-map/jobs/{job}/approval"));
    }

    #[test]
    fn a_reply_decodes_by_its_closed_code_and_never_by_its_ambiguous_status() {
        let commit = InferenceHubResponseV1 { status: 503, body: br#"{"schema":"semio.hub.inference-error/v1","code":"approval.commit-unavailable"}"#.to_vec() };
        assert_eq!(decode_inference_reply::<GisMapInferenceApprovalReceiptV1>(&commit).unwrap_err(), InferenceRouteErrorV1::CommitUnavailable);
        let storage = InferenceHubResponseV1 { status: 503, body: br#"{"schema":"semio.hub.inference-error/v1","code":"inference.storage"}"#.to_vec() };
        assert_eq!(decode_inference_reply::<GisMapInferenceJobReceiptV1>(&storage).unwrap_err(), InferenceRouteErrorV1::Storage);
        let cancelled = InferenceHubResponseV1 { status: 409, body: br#"{"schema":"semio.hub.inference-error/v1","code":"inference.cancelled"}"#.to_vec() };
        assert_eq!(decode_inference_reply::<GisMapInferenceEventPageV1>(&cancelled).unwrap_err(), InferenceRouteErrorV1::Cancelled);
        let opaque = InferenceHubResponseV1 { status: 403, body: b"<html>proxy</html>".to_vec() };
        assert_eq!(decode_inference_reply::<GisMapInferenceJobReceiptV1>(&opaque).unwrap_err(), InferenceRouteErrorV1::Denied);
        let foreign = InferenceHubResponseV1 { status: 500, body: br#"{"schema":"some.other/v1","code":"inference.denied"}"#.to_vec() };
        assert_eq!(decode_inference_reply::<GisMapInferenceJobReceiptV1>(&foreign).unwrap_err(), InferenceRouteErrorV1::Unavailable, "an error body that is not the hub's own closed shape is never trusted for its code");
    }

    #[test]
    fn a_two_hundred_reply_must_declare_its_own_exact_schema_and_carry_no_unknown_field() {
        let job = sample_job_id();
        let good = serde_json::json!({ "schema": GIS_MAP_INFERENCE_RECEIPT_SCHEMA, "jobId": job, "state": "succeeded", "proposalState": "offered", "proposalHash": sample_proposal_hash(), "cursor": 4, "expiresAtMs": 1_000 });
        let receipt: GisMapInferenceJobReceiptV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&good).expect("body") }).expect("a well-formed receipt decodes");
        assert_eq!(receipt.state, GisMapInferenceJobStateV1::Succeeded);
        assert_eq!(receipt.proposal_state, GisMapInferenceProposalStateV1::Offered);

        let mut wrong_schema = good.clone();
        wrong_schema["schema"] = serde_json::json!(GIS_MAP_INFERENCE_EVENTS_SCHEMA);
        assert_eq!(decode_inference_reply::<GisMapInferenceJobReceiptV1>(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&wrong_schema).expect("body") }).unwrap_err(), InferenceRouteErrorV1::Invalid);

        let mut leaked = good.clone();
        leaked["proposal"] = serde_json::json!("private bytes");
        assert_eq!(decode_inference_reply::<GisMapInferenceJobReceiptV1>(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&leaked).expect("body") }).unwrap_err(), InferenceRouteErrorV1::Invalid, "a private field appearing on the wire must fail loudly, never be dropped");

        assert_eq!(decode_inference_reply::<GisMapInferenceJobReceiptV1>(&InferenceHubResponseV1 { status: 200, body: vec![b'{'; INFERENCE_RESPONSE_MAX_BYTES + 1] }).unwrap_err(), InferenceRouteErrorV1::Bounds);
    }

    #[test]
    fn an_offered_page_carries_the_corpus_preview_and_a_forged_or_open_ring_is_refused() {
        let fixture = fixture();
        let job = sample_job_id();
        let preview: GisMapInferencePreviewV1 = serde_json::from_value(fixture["preview"].clone()).expect("the corpus preview decodes closed");
        assert_eq!(preview.validate(&job), Ok(()));
        assert_eq!(preview.job_id, job);
        assert_eq!(preview.region_id, format!("inference-{job}"));
        assert_eq!(preview.proposal_hash, sample_proposal_hash());
        assert_eq!(preview.ring[0], preview.ring[GIS_MAP_INFERENCE_PREVIEW_RING_POINTS - 1], "the published ring is closed");
        let bounds = &fixture["base"]["expectedInference"]["bounds"];
        let (lon_min, lat_min) = (bounds["lonMin"].as_f64().expect("lonMin"), bounds["latMin"].as_f64().expect("latMin"));
        let (lon_max, lat_max) = (bounds["lonMax"].as_f64().expect("lonMax"), bounds["latMax"].as_f64().expect("latMax"));
        assert_eq!(preview.ring, [[lon_min, lat_min], [lon_max, lat_min], [lon_max, lat_max], [lon_min, lat_max], [lon_min, lat_min]], "the preview does not fold to the corpus's own bounds");

        let mut foreign_job = preview.clone();
        foreign_job.job_id = "2".repeat(32);
        assert_eq!(foreign_job.validate(&job), Err(InferenceRouteErrorV1::Invalid));
        let mut forged_region = preview.clone();
        forged_region.region_id = "inference-forged".into();
        assert_eq!(forged_region.validate(&job), Err(InferenceRouteErrorV1::Invalid));
        let mut wrong_schema = preview.clone();
        wrong_schema.schema = GIS_MAP_INFERENCE_EVENTS_SCHEMA.into();
        assert_eq!(wrong_schema.validate(&job), Err(InferenceRouteErrorV1::Invalid));
        let mut open_ring = preview.clone();
        open_ring.ring[4] = [lon_max, lat_max];
        assert_eq!(open_ring.validate(&job), Err(InferenceRouteErrorV1::Conflict), "an unclosed ring is never rendered");
        let mut inverted = preview.clone();
        inverted.ring = [[lon_max, lat_max], [lon_min, lat_max], [lon_min, lat_min], [lon_max, lat_min], [lon_max, lat_max]];
        assert_eq!(inverted.validate(&job), Err(InferenceRouteErrorV1::Conflict));

        let page = serde_json::json!({
            "schema": GIS_MAP_INFERENCE_EVENTS_SCHEMA,
            "jobId": job,
            "state": "succeeded",
            "proposalState": "offered",
            "cancelRequested": false,
            "stale": false,
            "proposalHash": sample_proposal_hash(),
            "preview": fixture["preview"],
            "events": [],
            "progress": [],
            "nextCursor": 0,
        });
        let decoded: GisMapInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&page).expect("body") }).expect("an offered page decodes");
        assert_eq!(checked_page(decoded, &job).expect("the checked page keeps its verified preview").preview, Some(preview));

        let mut mismatched = page.clone();
        mismatched["proposalHash"] = serde_json::json!("0".repeat(64));
        let decoded: GisMapInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&mismatched).expect("body") }).expect("it still decodes");
        assert_eq!(checked_page(decoded, &job).unwrap_err(), InferenceRouteErrorV1::Conflict, "a preview whose digest disagrees with the page is never handed on");

        let mut foreign_page = page.clone();
        foreign_page["jobId"] = serde_json::json!("3".repeat(32));
        let decoded: GisMapInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&foreign_page).expect("body") }).expect("it still decodes");
        assert_eq!(checked_page(decoded, &job).unwrap_err(), InferenceRouteErrorV1::Conflict, "a page for another job is never accepted");

        let cancelled = serde_json::json!({
            "schema": GIS_MAP_INFERENCE_EVENTS_SCHEMA,
            "jobId": job,
            "state": "cancelled",
            "proposalState": "cancelled",
            "cancelRequested": true,
            "stale": false,
            "proposalHash": serde_json::Value::Null,
            "events": [],
            "progress": [],
            "nextCursor": 0,
        });
        let decoded: GisMapInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&cancelled).expect("body") }).expect("an omitted preview is absent, not an error");
        assert_eq!(decoded.preview, None);
    }

    #[test]
    fn the_neutral_lifecycles_decode_into_the_closed_event_page_in_order() {
        let fixture = fixture();
        for (name, trace) in [("lifecycle", &fixture["lifecycle"]), ("cancelLifecycle", &fixture["cancelLifecycle"])] {
            let rows = trace.as_array().expect("trace");
            assert!(rows.len() <= INFERENCE_EVENT_PAGE_MAX_ITEMS, "{name} exceeds one bounded page");
            let events: Vec<serde_json::Value> = rows.iter().map(|row| serde_json::json!({ "ordinal": row["ordinal"], "kind": row["kind"], "atMs": 1_000 })).collect();
            let body = serde_json::json!({
                "schema": GIS_MAP_INFERENCE_EVENTS_SCHEMA,
                "jobId": sample_job_id(),
                "state": if name == "lifecycle" { "succeeded" } else { "cancelled" },
                "proposalState": if name == "lifecycle" { "approved" } else { "cancelled" },
                "cancelRequested": name != "lifecycle",
                "stale": false,
                "proposalHash": sample_proposal_hash(),
                "events": events,
                "progress": [],
                "nextCursor": 0,
            });
            let page: GisMapInferenceEventPageV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&body).expect("body") }).unwrap_or_else(|error| panic!("{name} did not decode: {error:?}"));
            assert_eq!(page.events.iter().map(|event| event.ordinal).collect::<Vec<_>>(), (1..=rows.len() as u64).collect::<Vec<_>>());
            assert_eq!(page.events.iter().map(|event| event.kind.clone()).collect::<Vec<_>>(), rows.iter().map(|row| row["kind"].as_str().expect("kind").to_string()).collect::<Vec<_>>());
        }
    }
    //#endregion 🧪️WireShapes

    //#region 🧪️Calls
    #[test]
    fn a_submit_call_posts_the_bounded_closed_intent_to_the_exact_job_route() {
        let transport = ScriptedTransport::ok(200, serde_json::json!({ "schema": GIS_MAP_INFERENCE_RECEIPT_SCHEMA, "jobId": sample_job_id(), "state": "accepted", "proposalState": "none", "proposalHash": serde_json::Value::Null, "cursor": 0, "expiresAtMs": 9 }));
        let cancel = CancelToken::root_now();
        let request = GisMapInferenceSubmitRequestV1::new(sample_job_id(), 1_000);
        let receipt = block_on(submit_gis_map_job(&transport, &context(&cancel), "https://hub.invalid", &scope(), &request)).expect("scripted receipt");
        assert_eq!(receipt.job_id, sample_job_id());
        assert_eq!(receipt.proposal_hash, None);
        let seen = transport.requests();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].method, InferenceHubMethodV1::Post);
        assert_eq!(seen[0].path, gis_map_jobs_path(&scope()));
        assert!(seen[0].body.len() <= INFERENCE_REQUEST_MAX_BYTES);
        assert_eq!(seen[0].maximum_response_bytes, INFERENCE_RESPONSE_MAX_BYTES);
    }

    #[test]
    fn an_events_call_refuses_a_foreign_job_id_or_an_out_of_range_cursor_before_any_request() {
        let transport = ScriptedTransport::new(Vec::new());
        let cancel = CancelToken::root_now();
        assert_eq!(block_on(read_gis_map_job_events(&transport, &context(&cancel), "https://hub.invalid", &scope(), "not-a-job", 0)).unwrap_err(), InferenceRouteErrorV1::Invalid);
        assert_eq!(block_on(read_gis_map_job_events(&transport, &context(&cancel), "https://hub.invalid", &scope(), &sample_job_id(), INFERENCE_PROGRESS_MAX_CURSOR + 1)).unwrap_err(), InferenceRouteErrorV1::Invalid);
        assert!(transport.requests().is_empty(), "a malformed read never reaches the network");
    }

    #[test]
    fn an_already_cancelled_operation_context_never_reaches_the_hub_and_maps_to_cancelled() {
        let transport = ScriptedTransport::ok(200, serde_json::json!({}));
        let cancel = CancelToken::root_now();
        cancel.cancel_now();
        let error = block_on(cancel_gis_map_job(&transport, &context(&cancel), "https://hub.invalid", &scope(), &sample_job_id())).unwrap_err();
        assert_eq!(error, InferenceRouteErrorV1::Cancelled);
        assert_eq!(error.to_gateway_error("inference_cancel").code, GatewayErrorCode::Cancelled);
        assert!(transport.requests().is_empty(), "a cancelled call is never sent");
    }

    #[test]
    fn a_transport_failure_maps_onto_the_closed_route_vocabulary_and_never_a_fabricated_success() {
        for (transport_error, expected) in [
            (InferenceHubTransportErrorV1::Unauthorized, InferenceRouteErrorV1::Denied),
            (InferenceHubTransportErrorV1::DeadlineExceeded, InferenceRouteErrorV1::Unavailable),
            (InferenceHubTransportErrorV1::Unavailable, InferenceRouteErrorV1::Unavailable),
            (InferenceHubTransportErrorV1::ResourceLimit, InferenceRouteErrorV1::Bounds),
            (InferenceHubTransportErrorV1::InvalidRequest("authority mismatch"), InferenceRouteErrorV1::Invalid),
            (InferenceHubTransportErrorV1::Cancelled, InferenceRouteErrorV1::Cancelled),
        ] {
            let transport = ScriptedTransport::new(vec![Err(transport_error.clone())]);
            let cancel = CancelToken::root_now();
            let approval = GisMapInferenceApprovalRequestV1::new(sample_job_id(), sample_proposal_hash());
            assert_eq!(block_on(approve_gis_map_job(&transport, &context(&cancel), "https://hub.invalid", &scope(), &approval)).unwrap_err(), expected, "{transport_error:?}");
        }
    }

    #[test]
    fn a_retained_local_wait_is_interrupted_by_its_own_operation_label_and_by_nothing_else() {
        let label = inference_operation_label("space:alpha", "doc:tokyo", Some(&sample_job_id()));
        let other = inference_operation_label("space:alpha", "doc:tokyo", None);
        let cancel = CancelToken::root_now();
        retain_inference_operation(&label, cancel.clone());
        assert!(!interrupt_inference_operation("gis-map:space:alpha/doc:other/*"), "an unrelated label interrupts nothing");
        assert!(!cancel.is_cancelled_now());
        assert!(!interrupt_inference_operation(&other), "the document-wide label is a different retained wait");
        assert!(interrupt_inference_operation(&label));
        assert!(cancel.is_cancelled_now(), "the retained token is really cancelled, not merely reported");
        release_inference_operation(&label);
        assert!(!interrupt_inference_operation(&label), "a released wait is gone");
    }
    //#endregion 🧪️Calls

    //#region 🧪️Policy
    fn principal(scopes: &[&str]) -> AgentPrincipal {
        AgentPrincipal::from_scope_names("agent:test", "test agent", &scopes.iter().map(|scope| (*scope).to_string()).collect::<Vec<_>>(), None)
    }

    #[test]
    fn every_inference_job_tool_is_denied_without_its_scope_and_admitted_by_inference_execute() {
        let engine = PolicyEngine::new(Arc::new(crate::handles::HandleTable::new()), crate::policy::AutoApprovePolicy::Never);
        let unscoped = principal(&[]);
        for capability in inference_job_capabilities() {
            let denied = engine.authorize_scopes(&unscoped, &capability).expect_err("an unscoped principal is denied");
            assert_eq!(denied.code, GatewayErrorCode::PermissionDenied, "{}", capability.id);
        }
        let granted = principal(&["inference.execute"]);
        for capability in inference_job_capabilities() {
            engine.authorize_scopes(&granted, &capability).unwrap_or_else(|error| panic!("{} was refused for a granted principal: {error:?}", capability.id));
        }
        let read_only = principal(&["artifact.read"]);
        engine.authorize_scopes(&read_only, &inference_events_capability()).expect("documents.read alone reads the owner-private page");
        assert_eq!(engine.authorize_scopes(&read_only, &inference_submit_capability()).expect_err("a reader cannot submit").code, GatewayErrorCode::PermissionDenied);
        assert_eq!(engine.authorize_scopes(&read_only, &inference_approve_capability()).expect_err("a reader cannot approve").code, GatewayErrorCode::PermissionDenied);
        assert_eq!(engine.authorize_scopes(&read_only, &inference_cancel_capability()).expect_err("a reader cannot cancel").code, GatewayErrorCode::PermissionDenied);
    }

    #[test]
    fn a_job_handle_is_readable_only_by_its_own_session_and_its_own_authenticated_subject() {
        let handles = crate::handles::HandleTable::new();
        let mine = crate::handles::SessionHandle::new("sess_mine");
        let theirs = crate::handles::SessionHandle::new("sess_theirs");
        let subject = HubInferenceSubjectV1 { hub_origin: "https://hub.invalid".into(), space_id: "space:alpha".into(), user_id: "user-a".into(), authority_generation: 7 };
        let payload = GisMapInferenceJobHandlePayloadV1 {
            space_id: subject.space_id.clone(),
            document_id: "doc:tokyo".into(),
            job_id: sample_job_id(),
            subject_user_id: subject.user_id.clone(),
            authority_generation: subject.authority_generation,
            request_id: sample_job_id(),
            base: None,
        };
        let handle = handles.mint(crate::handles::HandleKind::Job, mine.clone(), crate::handles::Attachment::Artifact { artifact_id: "doc:tokyo".into() }, serde_json::to_value(&payload).expect("payload"), 1_000);
        assert_eq!(resolve_inference_job_handle(&handles, &mine, &subject, &handle, 1_001).expect("the minting session reads its own job"), payload);
        assert_eq!(resolve_inference_job_handle(&handles, &theirs, &subject, &handle, 1_001).expect_err("a second connection cannot read it").code, GatewayErrorCode::PermissionDenied);
        for (name, mutate) in [
            ("other-user", HubInferenceSubjectV1 { user_id: "user-b".into(), ..subject.clone() }),
            ("stale-authorization-generation", HubInferenceSubjectV1 { authority_generation: 8, ..subject.clone() }),
            ("cross-space", HubInferenceSubjectV1 { space_id: "space:beta".into(), ..subject.clone() }),
        ] {
            assert_eq!(resolve_inference_job_handle(&handles, &mine, &mutate, &handle, 1_001).expect_err(name).code, GatewayErrorCode::PermissionDenied, "{name} read another subject's job");
        }
        assert_eq!(resolve_inference_job_handle(&handles, &mine, &subject, "job_never_minted", 1_001).expect_err("unknown handle").code, GatewayErrorCode::NotFound);
    }

    #[test]
    fn the_four_capabilities_are_direct_object_typed_gateway_tools_with_bilingual_descriptions() {
        let expected = ["inference_submit", "inference_events", "inference_cancel", "inference_approve"];
        let capabilities = inference_job_capabilities();
        assert_eq!(capabilities.len(), expected.len());
        for (capability, name) in capabilities.iter().zip(expected) {
            let ToolExposure::Direct { tool_name } = &capability.exposure else { panic!("{} is not directly exposed", capability.id) };
            assert_eq!(tool_name, name);
            assert!(crate::protocol::is_valid_tool_name(tool_name), "{tool_name} is not a valid MCP tool name");
            assert_eq!(capability.input_schema["type"], "object", "{}", capability.id);
            assert_eq!(capability.input_schema["$schema"], "https://json-schema.org/draft/2020-12/schema");
            assert_eq!(capability.output_schema["type"], "object", "{}", capability.id);
            assert!(capability.effects.external, "{} crosses the network to the hub", capability.id);
            let (english, german) = capability.description.split_once(" — ").unwrap_or_else(|| panic!("{} is not bilingual", capability.id));
            assert!(english.len() > 40 && german.len() > 40, "{} has an empty half", capability.id);
            assert_ne!(english, german);
        }
        assert!(crate::GATEWAY_TOOL_NAMES.iter().filter(|name| expected.contains(name)).count() == expected.len(), "the census must list every inference job tool");
    }
    //#endregion 🧪️Policy
}
//#endregion 🧪️InferenceJobTests
