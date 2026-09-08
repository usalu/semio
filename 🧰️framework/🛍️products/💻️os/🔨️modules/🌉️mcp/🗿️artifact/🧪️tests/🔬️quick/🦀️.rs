
use super::*;
use crate::catalog::{Catalog, CatalogSource, compile};
use crate::protocol::ToolRegistry;

const ARTIFACT_TOOL_NAMES: [&str; 5] = ["artifact_open", "artifact_create", "artifact_validate", "artifact_snapshot", "artifact_export"];

fn empty_catalog() -> Arc<Catalog> {
    Arc::new(compile(&CatalogSource::default(), semio_framework::Locale::En, semio_framework::Terminology::Native).expect("empty catalog source compiles"))
}

/// 🧪️ A minimal, self-built (never `🧫️fixtures`, reserved for `🗂️catalog`/`🔎️search`/`🧠️context`/
/// `🧪️conformance`'s own tests) single-`CapabilityOwner::Plugin` catalog — just enough for
/// `require_workspace_has_a_plugin`/`require_resolvable_export_plugin` to resolve exactly one id,
/// so tier 3 is reachable without a real installed wasm plugin.
fn single_plugin_catalog(plugin_id: &str) -> Arc<Catalog> {
    let probe_capability = CapabilityDefinition {
        id: CapabilityRef("test.probe".to_string()),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: plugin_id.to_string(), app_id: None, window_kind_id: None, mode_id: None },
        kind: CapabilityKind::Query,
        title: "Test Probe".to_string(),
        description: "test-only capability so this workspace resolves exactly one plugin".to_string(),
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
    let source = CatalogSource { gateway: vec![probe_capability], ..Default::default() };
    Arc::new(compile(&source, semio_framework::Locale::En, semio_framework::Terminology::Native).expect("single-plugin source compiles"))
}

fn assert_object_typed_2020_12(schema: &serde_json::Value) {
    assert_eq!(schema.get("$schema").and_then(serde_json::Value::as_str), Some("https://json-schema.org/draft/2020-12/schema"));
    assert_eq!(schema.get("type").and_then(serde_json::Value::as_str), Some("object"), "schema: {schema}");
}

#[test]
fn every_artifact_tool_registers_under_its_declared_name() {
    let mut registry = InMemoryToolRegistry::new();
    register_artifact_tools(&mut registry, None);
    let tools = registry.list();
    assert_eq!(tools.len(), 5, "tools: {:?}", tools.iter().map(|tool| &tool.name).collect::<Vec<_>>());
    for name in ARTIFACT_TOOL_NAMES {
        assert!(tools.iter().any(|tool| tool.name == name), "missing tool {name}");
    }
}

#[test]
fn every_top_level_schema_is_object_typed_2020_12() {
    for schema in [
        artifact_open_input_schema(),
        artifact_open_output_schema(),
        artifact_create_input_schema(),
        artifact_create_output_schema(),
        artifact_validate_input_schema(),
        artifact_validate_output_schema(),
        artifact_snapshot_input_schema(),
        artifact_snapshot_output_schema(),
        artifact_export_input_schema(),
        artifact_export_output_schema(),
    ] {
        assert_object_typed_2020_12(&schema);
    }
}

#[test]
fn no_workspace_bound_is_a_retryable_plugin_unavailable_for_every_artifact_tool() {
    let mut registry = InMemoryToolRegistry::new();
    register_artifact_tools(&mut registry, None);
    let arguments_by_tool = [
        ("artifact_open", serde_json::json!({ "artifactId": "a" })),
        ("artifact_create", serde_json::json!({ "artifactId": "a", "kind": "k" })),
        ("artifact_validate", serde_json::json!({ "artifactId": "a" })),
        ("artifact_snapshot", serde_json::json!({ "artifactId": "a" })),
        ("artifact_export", serde_json::json!({ "artifactId": "a" })),
    ];
    for (name, arguments) in arguments_by_tool {
        let result = registry.call(name, arguments).unwrap_or_else(|error| panic!("{name} must be a known tool name: {error:?}"));
        assert!(result.is_error, "{name} must fail with no workspace bound");
        let structured = result.structured_content.expect("structured content");
        assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE", "{name}: {structured}");
        assert_eq!(structured["retryable"], true, "{name}: {structured}");
    }
}

#[test]
fn missing_required_field_is_input_invalid_before_any_workspace_check() {
    let mut registry = InMemoryToolRegistry::new();
    register_artifact_tools(&mut registry, None);
    let empty_arguments_by_tool = ["artifact_open", "artifact_create", "artifact_validate", "artifact_snapshot", "artifact_export"];
    for name in empty_arguments_by_tool {
        let result = registry.call(name, serde_json::json!({})).unwrap();
        assert!(result.is_error, "{name} must reject a missing required field");
        assert_eq!(result.structured_content.unwrap()["code"], "INPUT_INVALID", "{name}");
    }
}

#[test]
fn workspace_bound_with_zero_resolvable_plugins_is_still_plugin_unavailable() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = Arc::new(HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens"));
    let mut registry = InMemoryToolRegistry::new();
    register_artifact_tools(&mut registry, Some(workspace));
    let result = registry.call("artifact_open", serde_json::json!({ "artifactId": "whatever" })).unwrap();
    assert!(result.is_error);
    assert_eq!(result.structured_content.unwrap()["code"], "PLUGIN_UNAVAILABLE");
}

#[test]
fn artifact_create_then_open_round_trips_for_real_with_exactly_one_resolvable_plugin() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = Arc::new(HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), single_plugin_catalog("test-plugin")).expect("opens"));
    let mut registry = InMemoryToolRegistry::new();
    register_artifact_tools(&mut registry, Some(workspace));

    let created = registry.call("artifact_create", serde_json::json!({ "artifactId": "doc-1", "kind": "test.kind", "initial": { "n": 1 } })).unwrap();
    assert!(!created.is_error, "{created:?}");
    let created_structured = created.structured_content.expect("structured content");
    assert_eq!(created_structured["artifactId"], "doc-1");
    assert!(!created_structured["revision"]["headEditId"].as_str().unwrap_or_default().is_empty(), "a real applied edit has a non-empty head edit id: {created_structured}");

    let duplicate = registry.call("artifact_create", serde_json::json!({ "artifactId": "doc-1", "kind": "test.kind" })).unwrap();
    assert!(duplicate.is_error, "creating the same id twice must not silently no-op");
    assert_eq!(duplicate.structured_content.unwrap()["code"], "PRECONDITION_FAILED");

    let opened = registry.call("artifact_open", serde_json::json!({ "artifactId": "doc-1" })).unwrap();
    assert!(!opened.is_error, "{opened:?}");
    let opened_structured = opened.structured_content.expect("structured content");
    assert_eq!(opened_structured["artifactId"], "doc-1");
    assert!(opened_structured["sizeBytes"].as_u64().unwrap_or(0) > 0, "a real committed edit persists non-empty bytes: {opened_structured}");

    let missing = registry.call("artifact_open", serde_json::json!({ "artifactId": "does-not-exist" })).unwrap();
    assert!(missing.is_error);
    assert_eq!(missing.structured_content.unwrap()["code"], "NOT_FOUND");
}

#[test]
fn artifact_validate_is_a_real_typed_gap_never_a_fabricated_pass() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), single_plugin_catalog("test-plugin")).expect("opens");
    semio_framework::io::resolve_ready(workspace.ensure_probe_artifact("doc-2", serde_json::json!({}))).expect("seed");
    let mut registry = InMemoryToolRegistry::new();
    register_artifact_tools(&mut registry, Some(Arc::new(workspace)));
    let result = registry.call("artifact_validate", serde_json::json!({ "artifactId": "doc-2" })).unwrap();
    assert!(result.is_error, "no live validate command is wired yet — this must never silently pass");
    let structured = result.structured_content.unwrap();
    assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE");
    assert_eq!(structured["retryable"], true);
}

#[test]
fn artifact_snapshot_returns_real_bytes_for_the_current_revision_and_rejects_a_stale_one() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), single_plugin_catalog("test-plugin")).expect("opens");
    semio_framework::io::resolve_ready(workspace.ensure_probe_artifact("doc-3", serde_json::json!({ "n": 7 }))).expect("seed");
    let mut registry = InMemoryToolRegistry::new();
    register_artifact_tools(&mut registry, Some(Arc::new(workspace)));

    let current = registry.call("artifact_snapshot", serde_json::json!({ "artifactId": "doc-3" })).unwrap();
    assert!(!current.is_error, "{current:?}");
    let structured = current.structured_content.expect("structured content");
    assert!(structured["packBytes"].as_u64().unwrap_or(0) > 0, "a real committed edit snapshots non-empty bytes: {structured}");

    let stale = registry.call("artifact_snapshot", serde_json::json!({ "artifactId": "doc-3", "revision": { "artifactId": "doc-3", "headEditId": "not-a-real-edit-id", "cursor": "999" } })).unwrap();
    assert!(stale.is_error, "a mismatched revision must not silently answer with the current snapshot");
    assert_eq!(stale.structured_content.unwrap()["code"], "PRECONDITION_FAILED");
}

#[test]
fn artifact_export_never_fabricates_a_successful_export() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), single_plugin_catalog("test-plugin")).expect("opens");
    semio_framework::io::resolve_ready(workspace.ensure_probe_artifact("doc-4", serde_json::json!({}))).expect("seed");
    let mut registry = InMemoryToolRegistry::new();
    register_artifact_tools(&mut registry, Some(Arc::new(workspace)));
    let result = registry.call("artifact_export", serde_json::json!({ "artifactId": "doc-4", "format": "pdf" })).unwrap();
    assert!(result.is_error, "no live export command is wired yet — this must never silently succeed");
}
