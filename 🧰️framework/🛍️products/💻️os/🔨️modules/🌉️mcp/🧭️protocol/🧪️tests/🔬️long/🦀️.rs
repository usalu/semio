
use super::*;

#[test]
fn a_full_modern_session_lists_reads_and_subscribes_resources_end_to_end() {
    let mut resources = InMemoryResourceRegistry::new();
    resources.register(
        Resource { uri: "semio://audit/log".to_string(), name: "audit-log".to_string(), title: None, description: None, mime_type: Some("text/plain".to_string()), size: None },
        vec![ResourceContent { uri: "semio://audit/log".to_string(), mime_type: Some("text/plain".to_string()), text: Some("hello".to_string()), blob: None }],
    );
    let mut server = McpServer::new(Box::new(InMemoryToolRegistry::new()), Box::new(resources), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)));

    let meta = serde_json::json!({ "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } });
    let list = server.dispatch(&tests_support::request_with(1, METHOD_RESOURCES_LIST, meta.clone())).unwrap();
    let JsonRpcOutcome::Result { result } = list.outcome else { panic!("expected a result") };
    assert_eq!(result["resources"].as_array().unwrap().len(), 1);

    let read = server.dispatch(&tests_support::request_with(2, METHOD_RESOURCES_READ, serde_json::json!({ "uri": "semio://audit/log", "_meta": meta["_meta"] }))).unwrap();
    let JsonRpcOutcome::Result { result } = read.outcome else { panic!("expected a result") };
    assert_eq!(result["contents"][0]["text"], "hello");

    let subscribed = server.dispatch(&tests_support::request_with(3, METHOD_RESOURCES_SUBSCRIBE, serde_json::json!({ "uri": "semio://audit/log", "_meta": meta["_meta"] }))).unwrap();
    assert!(!subscribed.is_error());
    assert_eq!(server.era(), Some(ProtocolEra::Modern));
}

#[test]
fn batch_of_all_notifications_yields_no_responses() {
    let mut server = McpServer::with_defaults();
    let notifications = vec![
        JsonRpcRequest { jsonrpc: "2.0".to_string(), id: None, method: METHOD_NOTIFICATIONS_INITIALIZED.to_string(), params: None },
        JsonRpcRequest { jsonrpc: "2.0".to_string(), id: None, method: METHOD_NOTIFICATIONS_CANCELLED.to_string(), params: None },
    ];
    assert!(server.dispatch_batch(&notifications).is_empty());
}
