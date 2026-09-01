//! 💡️ Inference access — packet W5 of ticket 26/08/29/AI-MCP-END-TO-END. Before this facet there was
//! not one `infer` symbol anywhere under `🌉️mcp/**`: `job_infer`
//! (`🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs`) runs an inference INSIDE a plugin's own wasm
//! guest via its reactor, and `ArtifactInferenceRouter` (`semio_framework_plugin_host`, owned by
//! `🏃️run/🦀️component.rs`'s separate `run` process) routes to it there — but this crate's own
//! `HeadlessWorkspace` (`🏠️workspace/🦀️component.rs`) is a DIFFERENT process with its own wasmtime
//! activation (`open_artifact_channel`) and its own narrow wire port (`crate::actions::ArtifactChannel`
//! — `AppCommand` has exactly `ReadHistory`/`PureCommand`/`Transaction*`, no infer variant at all,
//! confirmed by reading `🔀️dispatch/🦀️component.rs` in full). So an inference cannot be EXECUTED
//! through this crate today — the same `channel.not-wired` class of gap `🏠️workspace`'s own
//! `PureCommand`/`TransactionPrepare` doc already names for mutations pre-W3.
//!
//! What IS real and reachable without touching any of that: every plugin's own committed
//! `🔣️descriptor.json` already carries its declared inference roster verbatim —
//! `PackageDescriptor.contributions.inference_services` (`Vec<semio_framework::ContributedInferenceMetadata>`,
//! owner-authored) plus every `artifact_contributions[].inferences` entry (contributed onto a
//! dependency's kind) — the EXACT roster `🏃️run/🦀️component.rs`'s own `register_plugin` builds
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
use crate::workspace::{find_plugin_entry, find_repo_root, load_package_descriptor, load_plugin_registry, HeadlessWorkspace, PROBE_SCHEMA};
use std::sync::Arc;

//#region 🔖️DeclaredInference
/// 💡️ One declared inference service, wire-shaped 1:1 from `semio_framework::ContributedInferenceMetadata`
/// — the exact static fields a plugin's own `🔣️descriptor.json` carries, never a live guest call.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
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
/// chain `🏃️run/🦀️component.rs`'s `register_plugin` builds for `ArtifactInferenceRouter`.
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
/// exists — `job_get`/`job_cancel` (`🦀️component.rs`'s own `DECLARED_STUB_TOOL_NAMES`, still
/// unimplemented) resolve/`mark_terminal` that SAME shared `HandleTable` by the id `mint` returns.
/// This facet never mints one itself: no execution route exists to mint FOR yet, and an inert job
/// handle nobody could ever progress would be its own kind of fabrication. Pinning this shape now
/// means wiring the two sides together later is additive, not a redesign.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
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
        "Lists inference services declared for one artifact (by id) or, with no artifactId, every inference declared by this workspace's default plugin.",
        inference_list_input_schema(),
        inference_list_output_schema(),
    )
}

fn inference_get_capability() -> CapabilityDefinition {
    inference_capability("inference.get", "inference_get", "Get Inference", "Reads one declared inference field for an artifact — a typed, retryable gap until a real artifact-infer route is wired.", inference_get_input_schema(), inference_get_output_schema())
}

/// 💡️ The inference capabilities, folded into `CatalogSource.gateway` by root wiring — same pattern
/// as `🦀️component.rs`'s own `core_tool_capabilities()`.
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
    match declared_inferences_for_artifact(workspace, artifact_id) {
        Err(error) => CallToolResult::tool_error(&error),
        Ok((schema, declared)) => match lookup_inference(&declared, inference_schema) {
            InferenceLookup::NoSuchService => CallToolResult::tool_error(&no_such_service_error(&schema, inference_schema)),
            InferenceLookup::Execute(item) => CallToolResult::tool_error(&execution_not_wired_error(&item)),
        },
    }
}

/// 💡️ Registers the inference tool(s) — always present in `tools/list` regardless of tier; only
/// their RESULT varies by whether `workspace` is bound (`🦀️component.rs`'s own
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
        let capability = CapabilityDefinition {
            id: CapabilityRef("procedural.probe".to_string()),
            version: 1,
            owner: CapabilityOwner::Plugin { plugin_id: "procedural".to_string(), app_id: None, window_kind_id: None, mode_id: None },
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
