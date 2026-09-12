
use super::*;
use crate::catalog::compile;
use crate::source_builders;
use semio_framework::{Locale, Terminology};

fn test_catalog() -> Catalog {
    compile(&source_builders::note_and_cad_source(), Locale::En, Terminology::Native).expect("compiles")
}

#[test]
fn estimate_tokens_rounds_up() {
    assert_eq!(estimate_tokens(0), 0);
    assert_eq!(estimate_tokens(1), 1);
    assert_eq!(estimate_tokens(4), 1);
    assert_eq!(estimate_tokens(5), 2);
}

#[test]
fn small_value_is_never_truncated() {
    let value = serde_json::json!({ "entries": [1, 2, 3] });
    let truncated = truncate_to_budget(value.clone(), DEFAULT_MAX_TOKENS);
    assert_eq!(truncated.value, value);
    assert!(truncated.omitted.is_empty());
}

#[test]
fn oversized_entries_array_is_truncated_with_recorded_pointers() {
    let entries: Vec<serde_json::Value> = (0..5000).map(|index| serde_json::json!({ "id": format!("capability-{index}"), "title": "x".repeat(50) })).collect();
    let value = serde_json::json!({ "entries": entries });
    let truncated = truncate_to_budget(value, 64);
    assert!(truncated.token_estimate <= 64 * 2, "truncation should bring the payload near budget, got {}", truncated.token_estimate);
    assert!(!truncated.omitted.is_empty());
    assert!(truncated.omitted.iter().all(|pointer| pointer.starts_with("/entries/")));
}

#[test]
fn mint_session_id_is_unique_per_counter() {
    let a = mint_session_id("agent:local", 0);
    let b = mint_session_id("agent:local", 1);
    assert_ne!(a, b);
    assert!(a.starts_with("sess_"));
}

#[test]
fn resolve_context_carries_the_catalog_hash() {
    let catalog = test_catalog();
    let summary = resolve_context(&catalog, "sess_1".to_string(), "agent:local", vec!["documents.read".to_string()], None, "en");
    assert_eq!(summary.catalog_hash, catalog.hash);
    assert_eq!(summary.session_id, "sess_1");
}

#[test]
fn capability_resource_with_id_returns_the_full_definition() {
    let catalog = test_catalog();
    let contents = capability_resource_contents(&catalog, Some("cad.editor.translateSelection")).expect("known capability resolves");
    assert_eq!(contents.len(), 1);
    let text = contents[0].text.as_ref().expect("json text");
    assert!(text.contains("translateSelection"));
}

#[test]
fn capability_resource_with_unknown_id_is_not_found() {
    let catalog = test_catalog();
    let result = capability_resource_contents(&catalog, Some("no.such.capability"));
    assert!(matches!(result, Err(error) if error.code == GatewayErrorCode::NotFound));
}

#[test]
fn capability_resource_without_id_lists_every_entry() {
    let catalog = test_catalog();
    let contents = capability_resource_contents(&catalog, None).expect("list resolves");
    let value: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().unwrap()).unwrap();
    assert_eq!(value["entries"].as_array().unwrap().len(), catalog.entries.len());
}

#[test]
fn bare_registry_still_lists_catalog_and_workspace_resources_and_templates() {
    let registry = WorkspaceResourceRegistry::new(Arc::new(test_catalog()));
    let listed = registry.list();
    assert!(listed.iter().any(|resource| resource.uri == "semio://capability"));
    assert!(listed.iter().any(|resource| resource.uri == "semio://workspace"));
    assert!(listed.iter().any(|resource| resource.uri == "semio://workspace/artifacts"));
    let templates = registry.templates();
    assert!(templates.iter().any(|template| template.uri_template == "semio://capability/{id}"));
    assert!(templates.iter().any(|template| template.uri_template == "semio://artifact/{artifactId}"));
}

#[test]
fn bare_registry_still_serves_real_catalog_reads() {
    let registry = WorkspaceResourceRegistry::new(Arc::new(test_catalog()));
    assert!(registry.read("semio://capability").is_ok());
    assert!(registry.read("semio://capability/cad.editor.translateSelection").is_ok());
}

#[test]
fn bare_registry_read_of_a_workspace_uri_is_plugin_unavailable_not_not_found() {
    let registry = WorkspaceResourceRegistry::new(Arc::new(test_catalog()));
    for uri in ["semio://workspace", "semio://workspace/artifacts", "semio://workspace/scopes/space-a/doc-a/checkpoint", "semio://artifact/probe-a"] {
        let error = registry.read(uri).expect_err("no workspace bound yet");
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable, "uri {uri} should report PLUGIN_UNAVAILABLE, not fabricate or panic");
        assert!(error.retryable);
    }
}

#[test]
fn registry_read_of_an_unknown_uri_is_a_well_formed_not_found() {
    let registry = WorkspaceResourceRegistry::new(Arc::new(test_catalog()));
    let error = registry.read("semio://not-a-resource").expect_err("unknown uri");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}

#[test]
fn subscribe_and_unsubscribe_stay_accepted_no_ops() {
    let registry = WorkspaceResourceRegistry::new(Arc::new(test_catalog()));
    assert!(registry.subscribe("semio://workspace").is_ok());
    assert!(registry.unsubscribe("semio://workspace").is_ok());
}

#[tokio::test]
async fn bound_registry_reads_a_workspace_uri_through_to_the_live_backend() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = Arc::new(HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), Arc::new(test_catalog())).expect("opens"));
    workspace.ensure_probe_artifact("probe-a", serde_json::json!({ "n": 1 })).await.expect("seed");
    let registry = WorkspaceResourceRegistry::with_workspace(Arc::new(test_catalog()), workspace.clone());

    let workspace_contents = registry.read("semio://workspace").expect("bound workspace resolves");
    let value: serde_json::Value = serde_json::from_str(workspace_contents[0].text.as_ref().unwrap()).unwrap();
    assert_eq!(value["artifacts"].as_array().unwrap(), &vec![serde_json::json!("probe-a")]);

    let artifact_contents = registry.read("semio://artifact/probe-a").expect("real open artifact resolves");
    let artifact_value: serde_json::Value = serde_json::from_str(artifact_contents[0].text.as_ref().unwrap()).unwrap();
    assert_eq!(artifact_value["artifactId"], "probe-a");

    assert!(registry.list().iter().any(|resource| resource.uri == "semio://artifact/probe-a"), "a real open artifact appears in list() once a workspace is bound");
}

#[test]
fn bound_registry_keeps_serving_real_catalog_reads_unchanged() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = Arc::new(HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), Arc::new(test_catalog())).expect("opens"));
    let registry = WorkspaceResourceRegistry::with_workspace(Arc::new(test_catalog()), workspace);
    let contents = registry.read("semio://capability").expect("catalog read still works once bound");
    let value: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().unwrap()).unwrap();
    assert_eq!(value["entries"].as_array().unwrap().len(), test_catalog().entries.len());
}
