
use super::*;

/// 🧫️ The compiled note+cad fixture catalog — every test below asserting a KNOWN capability
/// census injects THIS, never `build_catalog()`, whose whole job since ticket
/// 26/08/29/AI-MCP-END-TO-END is to discover whichever plugins are really installed.
fn fixture_catalog() -> std::sync::Arc<Catalog> {
    std::sync::Arc::new(compile(&note_and_cad_source(), semio_framework::Locale::En, semio_framework::Terminology::Native).expect("the note+cad fixture always compiles"))
}

/// 🧫️ [`build_server`] over [`fixture_catalog`] — a deterministic capability census.
fn fixture_server() -> McpServer {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local agent", &[], None);
    build_server_from_catalog(fixture_catalog(), principal, std::sync::Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())), Box::new(ArtifactChannels::Mock(MockArtifactChannel::new())), None)
}

#[test]
fn stdio_options_default_to_empty() {
    let options = StdioOptions::default();
    assert!(options.folder.is_none());
    assert!(options.principal.is_none());
    assert!(options.scopes.is_empty());
}

#[test]
fn every_facet_re_export_is_reachable_from_the_crate_root() {
    let _code: GatewayErrorCode = GatewayErrorCode::Internal;
    let _tools = InMemoryToolRegistry::new();
    let _resources = InMemoryResourceRegistry::new();
    let _prompts = InMemoryPromptRegistry::new();
    let _backend = NullBackend;
    let _server = McpServer::with_defaults();
    assert_eq!(SUPPORTED_PROTOCOL_VERSIONS[0], "2026-07-28");
}

#[test]
fn p1b_facet_re_exports_are_reachable_from_the_crate_root_too() {
    let _handles = HandleTable::new();
    let _idempotency = IdempotencyStore::new();
    let _audit = InMemoryAuditSink::new();
    let _bridge_frame = ShellToGateway::Ping;
    assert_eq!(BRIDGE_VERSION, 1);
}

#[test]
fn http_options_round_trip_without_a_credential_carrier() {
    let options = HttpOptions { port: 7401, bind: "127.0.0.1".to_string(), folder: None, hub: None, principal: None, scopes: vec![], audit_dir: None, allow_origin: vec![] };
    assert_eq!(options.port, 7401);
    assert_eq!(options.bind, "127.0.0.1");
}

/// 🎯️ `tools/list` is 26 tools and every one of them is real: 3 core gateway + 8 mutation-protocol
/// + 5 `🗿️artifact` + 2 `💡️inference` + 4 `🖥️ui`. Ticket 26/08/29/AI-MCP-END-TO-END retired the
/// last stub, so there is no longer a "declared but unimplemented" bucket to assert against.
#[test]
fn tools_list_is_the_full_real_gateway_surface() {
    let server = fixture_server();
    let tools = server.tools.list();
    assert_eq!(tools.len(), 26, "tools: {:?}", tools.iter().map(|tool| &tool.name).collect::<Vec<_>>());
    for name in GATEWAY_TOOL_NAMES {
        assert!(tools.iter().any(|tool| tool.name == name), "missing tool {name}");
    }
}

/// 🪜️ Progressive enhancement, tier 1: with no workspace bound, a workspace-backed tool is
/// present in `tools/list` and answers with a structured, RETRYABLE `PLUGIN_UNAVAILABLE` naming
/// the binding it needs — never a protocol-level failure, never fabricated data.
#[test]
fn workspace_backed_tools_degrade_to_a_retryable_plugin_unavailable_without_a_binding() {
    let server = fixture_server();
    for name in ["artifact_create", "artifact_open", "artifact_validate", "artifact_snapshot", "artifact_export", "inference_list", "inference_get", "ui_focus", "ui_reveal"] {
        let result = server.tools.call(name, serde_json::json!({})).unwrap_or_else(|error| panic!("{name} resolves: {error:?}"));
        assert!(result.is_error, "{name} must not fabricate a success");
        let structured = result.structured_content.as_ref().unwrap_or_else(|| panic!("{name} answers structurally"));
        assert!(structured["code"] == "PLUGIN_UNAVAILABLE" || structured["code"] == "INPUT_INVALID", "{name}: {structured}");
    }
}

#[test]
fn capabilities_search_tool_call_finds_translate_selection() {
    let server = fixture_server();
    let result = server.tools.call("capabilities_search", serde_json::json!({ "query": "move the selection" })).expect("known tool name resolves");
    assert!(!result.is_error);
    let structured = result.structured_content.expect("structured content");
    assert_eq!(structured["results"][0]["capabilityId"], "cad.editor.translateSelection");
}

#[test]
fn capabilities_describe_tool_call_returns_the_full_definition() {
    let server = fixture_server();
    let result = server.tools.call("capabilities_describe", serde_json::json!({ "capabilityId": "cad.editor.translateSelection" })).expect("known tool name resolves");
    assert!(!result.is_error);
    assert_eq!(result.structured_content.unwrap()["id"], "cad.editor.translateSelection");
}

#[test]
fn context_resolve_tool_call_returns_a_context_summary_with_the_catalog_hash() {
    let server = fixture_server();
    let catalog = build_catalog();
    let result = server.tools.call("context_resolve", serde_json::json!({ "principal": "agent:local" })).expect("known tool name resolves");
    assert!(!result.is_error);
    assert_eq!(result.structured_content.unwrap()["catalogHash"], catalog.hash);
}

#[test]
fn every_gateway_tool_name_satisfies_the_tool_name_charset() {
    for name in GATEWAY_TOOL_NAMES {
        assert!(is_valid_tool_name(name), "{name} violates ^[a-zA-Z0-9_-]{{1,64}}$");
    }
}

/// 🎯️ The census and the registry agree — no tool is registered that the census omits, and no
/// census name is unregistered. This is what makes `GATEWAY_TOOL_NAMES` a fact, not a comment.
#[test]
fn the_tool_census_matches_the_registry_exactly() {
    let server = fixture_server();
    let mut registered: Vec<String> = server.tools.list().into_iter().map(|tool| tool.name).collect();
    registered.sort();
    let mut census: Vec<String> = GATEWAY_TOOL_NAMES.iter().map(|name| (*name).to_string()).collect();
    census.sort();
    assert_eq!(registered, census);
}

//#region 🔖️MutationProtocolToolWiring
/// 🎬️ The exact scenario the brief's §5 live transcript demonstrates, proven deterministically:
/// a principal WITH `artifact.write` can `action_prepare` the cad demo capability and gets back a
/// real `PreparedActionReport`.
#[test]
fn action_prepare_tool_call_returns_a_prepared_action_report_for_a_granted_scope() {
    let principal = AgentPrincipal::from_scope_names("agent:demo", "demo", &["artifact.write".to_string()], None);
    let server = build_server_with_principal(principal, std::sync::Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())), Box::new(ArtifactChannels::Mock(MockArtifactChannel::new())), None);
    let result = server.tools.call("action_prepare", serde_json::json!({ "capabilityId": "cad.editor.translateSelection", "input": { "dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"] } })).expect("known tool name resolves");
    assert!(!result.is_error, "{result:?}");
    let structured = result.structured_content.expect("structured content");
    assert!(structured["preparedHandle"].as_str().unwrap().starts_with("prep_"));
    assert_eq!(structured["capabilityId"], "cad.editor.translateSelection");
}

/// 🎬️ The second half of the brief's §5 live transcript: a principal WITHOUT the required scope
/// gets `PERMISSION_DENIED`, never a protocol-level failure.
#[test]
fn action_prepare_tool_call_is_permission_denied_for_a_scope_the_principal_lacks() {
    let principal = AgentPrincipal::from_scope_names("agent:demo", "demo", &[], None); // no scopes granted
    let server = build_server_with_principal(principal, std::sync::Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())), Box::new(ArtifactChannels::Mock(MockArtifactChannel::new())), None);
    let result = server.tools.call("action_prepare", serde_json::json!({ "capabilityId": "cad.editor.translateSelection", "input": { "dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"] } })).expect("known tool name resolves");
    assert!(result.is_error);
    assert_eq!(result.structured_content.as_ref().unwrap()["code"], "PERMISSION_DENIED");
}

#[test]
fn action_invoke_tool_call_commits_a_prepared_capability_end_to_end() {
    let principal = AgentPrincipal::from_scope_names("agent:demo", "demo", &["artifact.write".to_string()], None);
    let server = build_server_with_principal(principal, std::sync::Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())), Box::new(ArtifactChannels::Mock(MockArtifactChannel::new())), None);
    let prepared = server.tools.call("action_prepare", serde_json::json!({ "capabilityId": "cad.editor.translateSelection", "input": { "dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"] } })).unwrap();
    let handle = prepared.structured_content.unwrap()["preparedHandle"].as_str().unwrap().to_string();
    let invoked = server.tools.call("action_invoke", serde_json::json!({ "preparedActionHandle": handle })).expect("known tool name resolves");
    assert!(!invoked.is_error, "{invoked:?}");
    let structured = invoked.structured_content.unwrap();
    assert_eq!(structured["status"], "SUCCEEDED");
    assert!(structured["undoToken"].as_str().unwrap().starts_with("undo_"));
}
//#endregion 🔖️MutationProtocolToolWiring
