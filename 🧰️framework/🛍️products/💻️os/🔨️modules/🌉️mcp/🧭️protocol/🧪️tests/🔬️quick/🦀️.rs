//! 🧱️ Protocol_quick of the server crate: The MCP server dispatcher and the stdio transport.

use crate::mcp_server::McpServer;
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_wire::tool_paging::page_window;
use semio_os_mcp_wire::protocol::{ContentBlock, CallToolResult, META_PROTOCOL_VERSION_KEY, ToolRegistry, InMemoryPromptRegistry, InMemoryResourceRegistry, InMemoryToolRegistry, JsonRpcId, JsonRpcIncoming, JsonRpcOutcome, JsonRpcRequest, METHOD_INITIALIZE, METHOD_NOTIFICATIONS_INITIALIZED, METHOD_NOT_FOUND, METHOD_SERVER_DISCOVER, METHOD_TOOLS_CALL, METHOD_TOOLS_LIST, NullBackend, ProtocolEra, SUPPORTED_PROTOCOL_VERSIONS, Tool, UNSUPPORTED_PROTOCOL_VERSION, compute_catalog_hash, is_valid_tool_name};
use semio_os_mcp_workspace::gateway_backend::GatewayBackends;

fn request(id: Option<i64>, method: &str, params: Option<serde_json::Value>) -> JsonRpcRequest {
    JsonRpcRequest { jsonrpc: "2.0".to_string(), id: id.map(JsonRpcId::Number), method: method.to_string(), params }
}

fn modern_meta(version: &str) -> serde_json::Value {
    serde_json::json!({ "_meta": { META_PROTOCOL_VERSION_KEY: version } })
}

//#region 🔖️FramingRoundTrips

#[test]
fn single_request_round_trips_through_json() {
    let original = request(Some(1), "ping", None);
    let json = serde_json::to_string(&original).unwrap();
    let parsed: JsonRpcIncoming = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, JsonRpcIncoming::Single(original));
}

#[test]
fn batch_round_trips_through_json() {
    let batch = vec![request(Some(1), "ping", None), request(Some(2), "server/discover", Some(modern_meta("2026-07-28")))];
    let json = serde_json::to_string(&batch).unwrap();
    let parsed: JsonRpcIncoming = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, JsonRpcIncoming::Batch(batch));
}

#[test]
fn absent_id_field_parses_as_a_notification() {
    let json = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
    let parsed: JsonRpcRequest = serde_json::from_str(json).unwrap();
    assert!(parsed.is_notification());
    assert_eq!(parsed.id, None);
}

#[test]
fn explicit_null_id_is_not_a_notification() {
    let json = r#"{"jsonrpc":"2.0","id":null,"method":"ping"}"#;
    let parsed: JsonRpcRequest = serde_json::from_str(json).unwrap();
    assert!(!parsed.is_notification());
    assert_eq!(parsed.id, Some(JsonRpcId::Null));
}

//#endregion 🔖️FramingRoundTrips

//#region 🔖️EraDetection

#[test]
fn modern_request_is_routed_via_meta_version_and_recorded_as_modern_era() {
    let mut server = McpServer::with_defaults();
    let response = server.dispatch(&request(Some(1), METHOD_TOOLS_LIST, Some(modern_meta("2026-07-28")))).unwrap();
    assert!(!response.is_error());
    assert_eq!(server.era(), Some(ProtocolEra::Modern));
    assert_eq!(server.negotiated_version(), Some("2026-07-28"));
}

#[test]
fn legacy_initialize_handshake_echoes_a_supported_client_version() {
    let mut server = McpServer::with_defaults();
    let response = server.dispatch(&request(Some(1), METHOD_INITIALIZE, Some(serde_json::json!({ "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": { "name": "t", "version": "0" } })))).unwrap();
    assert!(!response.is_error());
    let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
    assert_eq!(result["protocolVersion"], "2025-06-18");
    assert_eq!(server.era(), Some(ProtocolEra::Legacy));
    assert_eq!(server.negotiated_version(), Some("2025-06-18"));

    let notified = server.dispatch(&JsonRpcRequest { jsonrpc: "2.0".to_string(), id: None, method: METHOD_NOTIFICATIONS_INITIALIZED.to_string(), params: None });
    assert!(notified.is_none(), "a notification never gets a response");
    assert!(server.is_initialized());
}

#[test]
fn legacy_initialize_with_unknown_version_falls_back_to_latest_rather_than_erroring() {
    let mut server = McpServer::with_defaults();
    let response = server.dispatch(&request(Some(1), METHOD_INITIALIZE, Some(serde_json::json!({ "protocolVersion": "1999-01-01", "capabilities": {}, "clientInfo": { "name": "t", "version": "0" } })))).unwrap();
    let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
    assert_eq!(result["protocolVersion"], SUPPORTED_PROTOCOL_VERSIONS[0]);
}

#[test]
fn era_is_decided_by_the_opening_request_of_the_connection() {
    let mut modern_first = McpServer::with_defaults();
    assert!(modern_first.era().is_none());
    modern_first.dispatch(&request(Some(1), METHOD_SERVER_DISCOVER, None));
    assert_eq!(modern_first.era(), Some(ProtocolEra::Modern));

    let mut legacy_first = McpServer::with_defaults();
    legacy_first.dispatch(&request(Some(1), METHOD_INITIALIZE, Some(serde_json::json!({ "protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": { "name": "t", "version": "0" } }))));
    assert_eq!(legacy_first.era(), Some(ProtocolEra::Legacy));
}

//#endregion 🔖️EraDetection

//#region 🔖️VersionRejection

#[test]
fn unsupported_meta_version_returns_dash_32022_with_supported_list() {
    let mut server = McpServer::with_defaults();
    let response = server.dispatch(&request(Some(1), METHOD_TOOLS_LIST, Some(modern_meta("2020-01-01")))).unwrap();
    let JsonRpcOutcome::Error { error } = response.outcome else { panic!("expected an error") };
    assert_eq!(error.code, UNSUPPORTED_PROTOCOL_VERSION);
    let data = error.data.expect("data must carry supported/requested");
    assert_eq!(data["requested"], "2020-01-01");
    assert_eq!(data["supported"], serde_json::json!(SUPPORTED_PROTOCOL_VERSIONS));
}

//#endregion 🔖️VersionRejection

//#region 🔖️ServerDiscover

#[test]
fn server_discover_shape_carries_protocol_version_capabilities_and_server_info() {
    let mut server = McpServer::with_defaults();
    let response = server.dispatch(&request(Some(1), METHOD_SERVER_DISCOVER, Some(modern_meta("2025-11-25")))).unwrap();
    let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
    assert_eq!(result["resultType"], "complete");
    assert_eq!(result["protocolVersion"], "2025-11-25");
    assert_eq!(result["serverInfo"]["name"], "semio");
    assert!(result["capabilities"]["tools"]["listChanged"].as_bool().unwrap());
}

#[test]
fn server_discover_without_meta_defaults_to_the_newest_supported_version() {
    let mut server = McpServer::with_defaults();
    let response = server.dispatch(&request(Some(1), METHOD_SERVER_DISCOVER, None)).unwrap();
    let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
    assert_eq!(result["protocolVersion"], SUPPORTED_PROTOCOL_VERSIONS[0]);
}

//#endregion 🔖️ServerDiscover

//#region 🔖️ToolNameCharset

#[test]
fn tool_name_charset_accepts_and_rejects_correctly() {
    assert!(is_valid_tool_name("context_resolve"));
    assert!(is_valid_tool_name("cad__translateSelection"));
    assert!(is_valid_tool_name("a"));
    assert!(!is_valid_tool_name(""));
    assert!(!is_valid_tool_name("has spaces"));
    assert!(!is_valid_tool_name("has.dots"));
    assert!(!is_valid_tool_name(&"x".repeat(65)));
}

#[test]
fn registry_rejects_registration_of_an_invalid_tool_name() {
    let mut registry = InMemoryToolRegistry::new();
    let result = registry.register(Tool::new("bad name!", serde_json::json!({"type": "object"})), |_arguments| CallToolResult::ok(vec![], None));
    let error = result.unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::InputInvalid);
}

//#endregion 🔖️ToolNameCharset

//#region 🔖️ToolVsProtocolError

#[test]
fn calling_an_unregistered_tool_is_a_protocol_error() {
    let mut server = McpServer::with_defaults();
    let response = server.dispatch(&request(Some(1), METHOD_TOOLS_CALL, Some(serde_json::json!({ "name": "does_not_exist", "arguments": {} })))).unwrap();
    assert!(response.is_error(), "unknown tool must be a JSON-RPC protocol error, not a successful isError result");
}

#[test]
fn a_registered_tool_reporting_failure_is_a_successful_response_with_is_error_true() {
    let mut tools = InMemoryToolRegistry::new();
    tools.register(Tool::new("flaky_tool", serde_json::json!({"type": "object"})), |_arguments| CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::PreconditionFailed, "not ready"))).unwrap();
    let mut server = McpServer::new(Box::new(tools), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)));
    let response = server.dispatch(&request(Some(1), METHOD_TOOLS_CALL, Some(serde_json::json!({ "name": "flaky_tool", "arguments": {} })))).unwrap();
    assert!(!response.is_error(), "a tool's own failure must stay a JSON-RPC success envelope");
    let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
    assert_eq!(result["isError"], true);
    assert_eq!(result["structuredContent"]["code"], "PRECONDITION_FAILED");
}

//#endregion 🔖️ToolVsProtocolError

#[test]
fn unknown_method_is_method_not_found() {
    let mut server = McpServer::with_defaults();
    let response = server.dispatch(&request(Some(1), "nonexistent/method", None)).unwrap();
    let JsonRpcOutcome::Error { error } = response.outcome else { panic!("expected an error") };
    assert_eq!(error.code, METHOD_NOT_FOUND);
}

#[test]
fn catalog_hash_is_stable_under_reordering_and_changes_when_the_name_set_changes() {
    let a = vec![Tool::new("b_tool", serde_json::json!({})), Tool::new("a_tool", serde_json::json!({}))];
    let b = vec![Tool::new("a_tool", serde_json::json!({})), Tool::new("b_tool", serde_json::json!({}))];
    assert_eq!(compute_catalog_hash(&a), compute_catalog_hash(&b));
    let c = vec![Tool::new("a_tool", serde_json::json!({}))];
    assert_ne!(compute_catalog_hash(&a), compute_catalog_hash(&c));
}

//#region 🔖️InputGateAndPaging

#[test]
fn the_registry_gate_follows_the_language_neutral_input_law() {
    let document: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/🔣️.json")).unwrap();
    let cases: Vec<serde_json::Value> = serde_json::from_str(include_str!("../🧫️fixtures/🚪️input-gate-law/🔣️.json")).unwrap();
    for case in cases {
        let name = case["name"].as_str().unwrap();
        let mut registry = InMemoryToolRegistry::new();
        registry.register(Tool::new("probe", document["$defs"][case["shape"].as_str().unwrap()].clone()), |_| CallToolResult::ok(Vec::new(), None)).unwrap();
        let result = registry.call("probe", case["input"].clone()).unwrap();
        assert_eq!(!result.is_error, case["valid"].as_bool().unwrap(), "{name}");
        if let Some(mention) = case.get("mentions").and_then(serde_json::Value::as_str) {
            let structured = result.structured_content.unwrap();
            assert_eq!(structured["code"], "INPUT_INVALID", "{name}");
            assert_eq!(structured["retryable"], false, "{name}");
            assert!(structured["message"].as_str().unwrap().contains(mention), "{name}: {structured}");
        }
    }
}

#[test]
fn the_gate_reads_absent_arguments_as_an_empty_object_and_rejects_other_types() {
    let mut registry = InMemoryToolRegistry::new();
    registry.register(Tool::new("probe", serde_json::json!({ "type": "object", "properties": { "a": { "type": "string" } }, "additionalProperties": false })), |_| CallToolResult::ok(Vec::new(), None)).unwrap();
    assert!(!registry.call("probe", serde_json::Value::Null).unwrap().is_error);
    assert!(registry.call("probe", serde_json::json!([1])).unwrap().is_error);
}

#[test]
fn a_tool_error_text_leads_with_its_code() {
    let result = CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::NotFound, "no such job: j"));
    let ContentBlock::Text { text } = &result.content[0] else { panic!("text block") };
    assert_eq!(text, "NOT_FOUND: no such job: j");
    assert_eq!(result.structured_content.unwrap()["code"], "NOT_FOUND");
}

#[test]
fn list_paging_follows_the_language_neutral_paging_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📄️paging-law/🔣️.json")).unwrap();
    let (default, maximum) = (law["default"].as_u64().unwrap() as usize, law["maximum"].as_u64().unwrap() as usize);
    for case in law["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let outcome = page_window(&case["arguments"], default, maximum);
        match case.get("error").and_then(serde_json::Value::as_str) {
            Some(code) => assert_eq!(outcome.unwrap_err().code.wire_name(), code, "{name}"),
            None => {
                let window = outcome.unwrap();
                assert_eq!((window.offset as u64, window.size as u64), (case["offset"].as_u64().unwrap(), case["size"].as_u64().unwrap()), "{name}");
            }
        }
    }
}

#[test]
fn a_window_offers_a_next_cursor_only_while_rows_remain_and_the_cursor_round_trips() {
    let first = page_window(&serde_json::json!({ "limit": 3 }), 20, 100).unwrap();
    let cursor = first.next_cursor(7).unwrap();
    let second = page_window(&serde_json::json!({ "limit": 3, "cursor": cursor }), 20, 100).unwrap();
    assert_eq!(second.offset, 3);
    assert_eq!(second.next_cursor(7).map(|cursor| page_window(&serde_json::json!({ "cursor": cursor }), 20, 100).unwrap().offset), Some(6));
    assert_eq!(page_window(&serde_json::json!({ "limit": 3, "cursor": second.next_cursor(7).unwrap() }), 20, 100).unwrap().next_cursor(7), None);
}

//#endregion 🔖️InputGateAndPaging
