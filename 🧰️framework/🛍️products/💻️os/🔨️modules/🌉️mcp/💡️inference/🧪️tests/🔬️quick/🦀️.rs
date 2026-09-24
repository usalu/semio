
use super::*;
use crate::catalog::{CapabilityAudience, Catalog, CatalogSource, compile};
use crate::protocol::ToolRegistry;
use crate::protocol::is_valid_tool_name;
use semio_framework::{Locale, Terminology};

fn empty_catalog() -> Arc<Catalog> {
    Arc::new(compile(&CatalogSource::default(), Locale::En, Terminology::Native).expect("empty catalog compiles"))
}

fn wfc_only_catalog() -> Arc<Catalog> {
    plugin_only_catalog("wfc")
}

fn plugin_only_catalog(plugin_id: &str) -> Arc<Catalog> {
    let capability = CapabilityDefinition {
        id: CapabilityRef(format!("{plugin_id}.probe")),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: plugin_id.to_string(), label: None, app_id: None, window_kind_id: None, mode_id: None },
        kind: CapabilityKind::Query,
        audience: CapabilityAudience::Agent,
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🗺️gis-discovery/🔣️.json")).expect("neutral GIS discovery fixture");
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
fn declared_inferences_for_workspace_finds_the_real_wfc_roster() {
    let workspace = open_workspace(wfc_only_catalog());
    let declared = declared_inferences_for_workspace(&workspace).expect("wfc is the sole plugin owner");
    assert!(declared.iter().all(|row| row.owner == "wfc" && row.contributor == "wfc"), "{declared:?}");
    let solved: Vec<(&str, &str)> = declared.iter().map(|row| (row.artifact_kind.as_str(), row.inference_schema.as_str())).collect();
    assert_eq!(
        solved,
        vec![
            ("s.wfc.bitmap", "s.wfc.bitmap.solve"),
            ("s.wfc.grid2d", "s.wfc.grid2d.solve"),
            ("s.wfc.grid3d", "s.wfc.grid3d.solve"),
            ("s.wfc.wfc2d", "s.wfc.wfc2d.solve"),
            ("s.wfc.wfc3d", "s.wfc.wfc3d.solve"),
        ],
        "the committed 🀄️wfc descriptor's own five artifact kinds, in its own declaration order"
    );
}

/// 🧩️ ticket 26/09/18 slice M10 (`📓️g19-ai-user-experience-audit.md` gap 3): an extension package
/// declares no app, no command and no owner-authored inference service, so it emits no catalog entry
/// and folder-mode `discovery_descriptors` — which is derived from catalog entries — never loads its
/// descriptor. Its contributed inference was therefore unreachable from `inference_list` for any
/// workspace, however complete the install. The law pins both halves of the reachability rule: the
/// contribution IS listed when the kind it contributes onto is owned by a package in the roster's
/// source set, and is NOT listed when it is not.
#[test]
fn declared_inferences_for_workspace_lists_an_extension_contributed_service_on_a_kind_it_reaches() {
    let workspace = open_workspace(plugin_only_catalog("cad"));
    let declared = declared_inferences_for_workspace(&workspace).expect("cad is the sole plugin owner");
    let contributed: Vec<(&str, &str, &str)> = declared.iter().filter(|row| row.contributor != row.owner || row.owner != "cad").map(|row| (row.owner.as_str(), row.artifact_kind.as_str(), row.inference_schema.as_str())).collect();
    assert_eq!(
        contributed,
        vec![("cad-extension-aec-building", "s.cad.cad", "s.cad-extension-aec-building.building-structure-summary")],
        "the committed 📐️cad extension's own contributed inference, routed by its own contributor id"
    );
    assert_eq!(declared.iter().filter(|row| row.inference_schema == "s.cad-extension-aec-building.building-structure-summary").count(), 1, "a contributed service is listed once, never once per source set it appears in");
}

#[test]
fn declared_inferences_for_workspace_omits_an_extension_contribution_onto_an_unreachable_kind() {
    let workspace = open_workspace(wfc_only_catalog());
    let declared = declared_inferences_for_workspace(&workspace).expect("wfc is the sole plugin owner");
    assert!(declared.iter().all(|row| row.artifact_kind.starts_with("s.wfc.")), "no contribution onto a kind this workspace does not own may reach the roster: {declared:?}");
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
    let workspace = open_workspace(wfc_only_catalog());
    workspace.ensure_probe_artifact("probe-inf", serde_json::json!({ "n": 1 })).await.expect("seed");
    let (schema, declared) = declared_inferences_for_artifact(&workspace, "probe-inf").expect("probe schema resolves");
    assert_eq!(schema, PROBE_SCHEMA);
    assert!(declared.is_empty(), "no plugin declares an inference against this crate's own probe schema");
}

#[test]
fn declared_inferences_for_artifact_is_retryable_plugin_unavailable_for_an_unknown_id() {
    let workspace = open_workspace(wfc_only_catalog());
    let error = declared_inferences_for_artifact(&workspace, "does-not-exist").expect_err("never seen — same gap as 🏠️workspace's own /schema arm");
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}
#[tokio::test]
async fn inference_get_on_an_open_probe_names_the_missing_service_not_found() {
    let workspace = open_workspace(wfc_only_catalog());
    workspace.ensure_probe_artifact("probe-get", serde_json::json!({ "n": 1 })).await.expect("seed");
    let mut registry = InMemoryToolRegistry::new();
    register_inference_tools(&mut registry, Some(workspace));
    let result = registry.call("inference_get", serde_json::json!({ "artifactId": "probe-get", "inferenceSchema": "s.wfc.wfc3d.solve" })).expect("registered tool");
    assert!(result.is_error);
    let payload = result.structured_content.expect("structured error payload");
    assert_eq!(payload["code"], "NOT_FOUND");
}
//#endregion 🧪️Discovery

//#region 🧪️ExecutionSeam
#[test]
fn lookup_inference_distinguishes_no_such_service_from_execute() {
    let declared = vec![DeclaredInference {
        owner: "wfc".to_string(),
        artifact_kind: "s.wfc.wfc3d".to_string(),
        artifact_schema: "s.wfc.wfc3d".to_string(),
        artifact_schema_version: 1,
        inference_schema: "s.wfc.wfc3d.solve".to_string(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        contributor: "wfc".to_string(),
        depends_on: Vec::new(),
        payload: None,
    }];
    assert!(matches!(lookup_inference(&declared, "s.wfc.wfc3d.solve"), InferenceLookup::Execute(_)));
    assert!(matches!(lookup_inference(&declared, "no.such.schema"), InferenceLookup::NoSuchService));
}

#[test]
fn execute_lookup_reports_a_retryable_channel_not_wired_gap() {
    let item = DeclaredInference {
        owner: "wfc".to_string(),
        artifact_kind: "s.wfc.wfc3d".to_string(),
        artifact_schema: "s.wfc.wfc3d".to_string(),
        artifact_schema_version: 1,
        inference_schema: "s.wfc.wfc3d.solve".to_string(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        contributor: "wfc".to_string(),
        depends_on: Vec::new(),
        payload: None,
    };
    let error = execution_not_wired_error(&item);
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
    let payload = inference_job_payload("art-1", &item, "cancel-1");
    assert_eq!(payload.artifact_kind, "s.wfc.wfc3d");
    assert_eq!(payload.inference_schema, "s.wfc.wfc3d.solve");
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
    let workspace = open_workspace(wfc_only_catalog());
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
    let workspace = open_workspace(wfc_only_catalog());
    workspace.ensure_probe_artifact("probe-list", serde_json::json!({ "n": 1 })).await.expect("seed");
    let resources = inference_resources(Some(&workspace));
    assert!(resources.iter().any(|resource| resource.uri == "semio://artifact/probe-list/inference"));
}
//#endregion 🧪️Resources

//#region 🧪️Execution
/// 🏗️ A tool registry with `inference_run` wired exactly the way the live binary wires it, plus the
/// shared `MockArtifactChannel` so the test can assert on the REAL `AppCommand::Infer` that left the
/// dispatch path. The workspace is a real `--folder` one over `catalog`, so the declared roster is
/// read from the plugins' own COMMITTED `🔣️.json` descriptors — no fixture roster is invented here.
fn inference_run_harness(catalog: Arc<Catalog>) -> (InMemoryToolRegistry, crate::actions::MockArtifactChannel, Arc<HeadlessWorkspace>) {
    let workspace = open_workspace(catalog.clone());
    let channel = crate::actions::MockArtifactChannel::new();
    let actions = Arc::new(crate::actions::ActionAdapter::new(
        Box::new(crate::workspace::ArtifactChannels::Mock(channel.clone())),
        Arc::new(crate::handles::HandleTable::new()),
        Arc::new(crate::handles::IdempotencyStore::new()),
        Arc::new(crate::audit::AuditSinks::InMemory(crate::audit::InMemoryAuditSink::new())),
        crate::policy::AutoApprovePolicy::Never,
        crate::audit::ClientInfo { name: "test".into(), version: "0".into() },
    ));
    let principal = AgentPrincipal::from_scope_names("agent:test", "claude-code", &["artifacts.read".to_string(), "jobs.spawn".to_string()], None);
    let mut registry = InMemoryToolRegistry::new();
    register_inference_job_tools(&mut registry, catalog, Some(workspace.clone()), actions, principal, crate::handles::SessionHandle::new("sess_inference"));
    (registry, channel, workspace)
}

fn call_inference_run(registry: &InMemoryToolRegistry, arguments: serde_json::Value) -> CallToolResult {
    registry.call("inference_run", arguments).expect("inference_run is registered")
}

/// 💡️ The GIS Map service is the oracle: it is the ONE inference this gateway could already reach
/// (hub-backed), so proving the general route resolves the SAME declared row from the SAME committed
/// descriptor and dispatches it through `AppCommand::Infer` is what makes the new path trustworthy.
#[tokio::test]
async fn inference_run_dispatches_the_gis_map_oracle_through_the_infer_command() {
    let (registry, channel, _workspace) = inference_run_harness(plugin_only_catalog("gis"));
    let result = call_inference_run(
        &registry,
        serde_json::json!({ "artifactKind": "s.gis.gismap", "inferenceSchema": "s.gis.gismap.inference", "payload": { "probe": 1 }, "cancellationId": "cancel-gis" }),
    );
    assert!(!result.is_error, "gis map inference must not be a tool error: {result:?}");
    let structured = result.structured_content.expect("inference_run answers structured content");
    assert_eq!(structured["status"], "SUCCEEDED");
    assert_eq!(structured["pluginId"], "gis");
    assert_eq!(structured["inferenceSchema"], "s.gis.gismap.inference");
    assert_eq!(structured["payload"], serde_json::json!({ "probe": 1 }), "the guest's own result payload, not a host-synthesised one");
    assert!(structured["jobId"].as_str().expect("a job handle").starts_with("job_"), "an expensive call always mints a job for job_get/job_cancel");

    let log = channel.frame_log();
    assert_eq!(log.len(), 1, "exactly one command left the dispatch path: {log:?}");
    let crate::actions::AppCommand::Infer(command) = &log[0].1 else { panic!("expected an Infer command, got {:?}", log[0].1) };
    assert_eq!(command.plugin_id, "gis");
    assert_eq!(command.artifact_kind, "s.gis.gismap");
    assert_eq!(command.inference_schema, "s.gis.gismap.inference");
    assert_eq!(command.cancellation_id, "cancel-gis");
}

/// 💡️ …and a service that was previously `channel.not-wired` for ALL of its life — `wfc`'s own
/// `s.wfc.wfc3d.solve` — travels the identical route with no per-plugin special case anywhere.
#[tokio::test]
async fn inference_run_dispatches_a_previously_not_wired_service_identically() {
    let (registry, channel, _workspace) = inference_run_harness(wfc_only_catalog());
    let result = call_inference_run(&registry, serde_json::json!({ "artifactKind": "s.wfc.wfc3d", "inferenceSchema": "s.wfc.wfc3d.solve", "payload": { "seed": 7 } }));
    assert!(!result.is_error, "wfc inference must not be a tool error: {result:?}");
    let structured = result.structured_content.expect("structured content");
    assert_eq!(structured["status"], "SUCCEEDED");
    assert_eq!(structured["pluginId"], "wfc");
    assert_eq!(structured["payload"], serde_json::json!({ "seed": 7 }));

    let log = channel.frame_log();
    let crate::actions::AppCommand::Infer(command) = &log[0].1 else { panic!("expected an Infer command, got {:?}", log[0].1) };
    assert_eq!(command.plugin_id, "wfc");
    assert_eq!(command.inference_schema, "s.wfc.wfc3d.solve");
    assert!(!command.cancellation_id.is_empty(), "a cancellation identity is always minted, so job_cancel has something to address");
}

/// 🚫️ An inference nobody declares is a typed `NOT_FOUND` naming both halves of the pair — never a
/// silent empty result and never a guessed plugin.
#[tokio::test]
async fn inference_run_refuses_an_undeclared_service() {
    let (registry, channel, _workspace) = inference_run_harness(wfc_only_catalog());
    let result = call_inference_run(&registry, serde_json::json!({ "artifactKind": "s.wfc.wfc3d", "inferenceSchema": "s.wfc.wfc3d.nonexistent" }));
    assert!(result.is_error);
    assert!(channel.frame_log().is_empty(), "an undeclared service must never reach a plugin channel");
}

/// 🔐️ A principal without `jobs.spawn` is refused by the gateway's OWN policy engine before any
/// plugin is touched — the same scope gate every other inference tool applies.
#[tokio::test]
async fn inference_run_is_scope_gated() {
    let catalog = wfc_only_catalog();
    let workspace = open_workspace(catalog.clone());
    let channel = crate::actions::MockArtifactChannel::new();
    let actions = Arc::new(crate::actions::ActionAdapter::new(
        Box::new(crate::workspace::ArtifactChannels::Mock(channel.clone())),
        Arc::new(crate::handles::HandleTable::new()),
        Arc::new(crate::handles::IdempotencyStore::new()),
        Arc::new(crate::audit::AuditSinks::InMemory(crate::audit::InMemoryAuditSink::new())),
        crate::policy::AutoApprovePolicy::Never,
        crate::audit::ClientInfo { name: "test".into(), version: "0".into() },
    ));
    let mut registry = InMemoryToolRegistry::new();
    register_inference_job_tools(&mut registry, catalog, Some(workspace), actions, AgentPrincipal::from_scope_names("agent:test", "claude-code", &[], None), crate::handles::SessionHandle::new("sess_unscoped"));
    let result = call_inference_run(&registry, serde_json::json!({ "artifactKind": "s.wfc.wfc3d", "inferenceSchema": "s.wfc.wfc3d.solve" }));
    assert!(result.is_error);
    assert!(channel.frame_log().is_empty());
}
//#endregion 🧪️Execution

//#region 🧪️PayloadContract
/// 📜️ One declared row carrying the contract a re-described `🀄️wfc` publishes.
fn contract_row(required: bool, input_schema: &str) -> DeclaredInference {
    DeclaredInference {
        owner: "wfc".to_string(),
        artifact_kind: "s.wfc.bitmap".to_string(),
        artifact_schema: "s.wfc.bitmap".to_string(),
        artifact_schema_version: 1,
        inference_schema: "s.wfc.bitmap.solve".to_string(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        contributor: "wfc".to_string(),
        depends_on: Vec::new(),
        payload: Some(semio_framework::InferencePayloadContract {
            payload_schema_id: "s.wfc.bitmap.inference.request.v1".to_string(),
            input_schema: input_schema.to_string(),
            output_schema: "{}".to_string(),
            progress_unit: "cells".to_string(),
            artifact_binding: Some(semio_framework::InferenceArtifactBinding { field: "document".to_string(), encoding: semio_framework::INFERENCE_ARTIFACT_PACK_BASE64.to_string(), required }),
            commit: None,
        }),
    }
}

/// 🚧️ The refusal `📓️pz2-puzzle-describe-under-budget.md` §5.2 asked for: an artifact-bound
/// inference called with neither `artifactId` nor `payload` answers `INPUT_INVALID` naming
/// `artifactId` — machine-readable in `details.field` — instead of dispatching `{}` into a guest
/// that answers `missing field 'snapshot'` after a whole component compile.
#[test]
fn an_artifact_bound_inference_without_an_artifact_is_input_invalid_naming_artifact_id() {
    let error = validate_inference_request(&contract_row(true, "{\"type\":\"object\"}"), None, None).expect_err("an artifact-bound inference needs an artifact");
    assert_eq!(error.code, GatewayErrorCode::InputInvalid);
    assert_eq!(error.details.get("field").and_then(serde_json::Value::as_str), Some("artifactId"));
    assert!(error.message.contains("payload.document"), "the refusal names the field the binding fills: {}", error.message);
}

/// ✅️ …and the same call WITH an artifact passes the gate, so the refusal is a gate and not a wall.
#[test]
fn an_artifact_bound_inference_with_an_artifact_is_admitted() {
    validate_inference_request(&contract_row(true, "{\"type\":\"object\"}"), None, Some("art-1")).expect("naming the artifact is the whole requirement");
}

/// 🧾️ A caller-authored payload is checked against the contract's OWN `required` list, and the
/// refusal names the missing field rather than the tool.
#[test]
fn a_payload_missing_a_contract_required_field_names_that_field() {
    let row = contract_row(false, "{\"type\":\"object\",\"required\":[\"snapshot\"]}");
    let payload = serde_json::json!({ "checkpoint": [] });
    let error = validate_inference_request(&row, Some(&payload), None).expect_err("a required field is missing");
    assert_eq!(error.details.get("field").and_then(serde_json::Value::as_str), Some("snapshot"));
}

/// 🫙 An inference that publishes no contract at all and is called with no payload says so — the
/// honest answer when nothing in the tree can tell a client what to send.
#[test]
fn an_unpublished_contract_refuses_by_naming_payload_and_its_owner() {
    let mut row = contract_row(true, "{}");
    row.payload = None;
    let error = validate_inference_request(&row, None, None).expect_err("nothing can build this body");
    assert_eq!(error.details.get("field").and_then(serde_json::Value::as_str), Some("payload"));
    assert!(error.message.contains("wfc"), "the refusal names the plugin that must declare one: {}", error.message);
}

/// 📜️ The contract a plugin commits in its descriptor is the contract `inference_list` publishes —
/// one declaration, carried, never re-derived.
#[test]
fn a_committed_contract_reaches_the_published_roster_row() {
    let metadata = semio_framework::ContributedInferenceMetadata {
        owner: "wfc".into(),
        artifact_kind: "s.wfc.bitmap".into(),
        artifact_schema: "s.wfc.bitmap".into(),
        artifact_schema_version: 1,
        inference_schema: "s.wfc.bitmap.solve".into(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        contributor: "wfc".into(),
        depends_on: Vec::new(),
        payload: contract_row(true, "{\"type\":\"object\"}").payload,
    };
    let row = DeclaredInference::from(&metadata);
    let published = serde_json::to_value(&row).expect("a roster row serialises");
    assert_eq!(published["payload"]["payloadSchemaId"], "s.wfc.bitmap.inference.request.v1");
    assert_eq!(published["payload"]["artifactBinding"]["field"], "document");
    assert_eq!(published["payload"]["progressUnit"], "cells");
}
//#endregion 🧪️PayloadContract

/// 📜️ A READ of an artifact-bound inference is still a gap, but it is now an INSTRUCTION: the
/// refusal names `inference_run`, the field the binding fills, and carries the whole published
/// contract in its details — so a client learns what to send in the same round trip it asked in.
#[test]
fn a_read_of_an_artifact_bound_inference_answers_with_its_contract() {
    let error = execution_not_wired_error(&contract_row(true, "{\"type\":\"object\"}"));
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
    assert!(error.message.contains("inference_run") && error.message.contains("payload.document"), "{}", error.message);
    assert_eq!(error.details["payload"]["payloadSchemaId"], "s.wfc.bitmap.inference.request.v1");
    assert_eq!(error.details["payload"]["artifactBinding"]["field"], "document");
}

/// 🫙 …and a row with no published contract keeps the older wording, so nothing is claimed for a
/// plugin that has declared nothing.
#[test]
fn a_read_of_an_unbound_inference_keeps_its_plain_refusal() {
    let mut row = contract_row(true, "{}");
    row.payload = None;
    let error = execution_not_wired_error(&row);
    assert!(error.message.contains("carries no canonical request payload"), "{}", error.message);
    assert!(error.details["payload"].is_null());
}
