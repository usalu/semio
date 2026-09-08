
use super::*;
use crate::catalog::{Catalog, CatalogSource, compile};
use crate::protocol::ToolRegistry;
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
