
use super::*;
use crate::protocol::{InMemoryPromptRegistry, InMemoryResourceRegistry, InMemoryToolRegistry, META_PROTOCOL_VERSION_KEY, McpServer, NullBackend, Tool};
use crate::workspace::GatewayBackends;
use axum::body::Body;
use axum::http::Request;

#[derive(Clone)]
struct HttpTestDriver {
    state: HttpState,
}

impl HttpTestDriver {
    async fn request(&self, request: Request<Body>) -> Response {
        let (parts, body) = request.into_parts();
        match (parts.method, parts.uri.path()) {
            (axum::http::Method::POST, "/mcp") => match axum::body::to_bytes(body, usize::MAX).await {
                Ok(body) => handle_post(State(self.state.clone()), parts.headers, body).await,
                Err(error) => (StatusCode::BAD_REQUEST, error.to_string()).into_response(),
            },
            (axum::http::Method::GET, "/mcp") => handle_get(State(self.state.clone()), parts.headers).await,
            (_, "/mcp") => StatusCode::METHOD_NOT_ALLOWED.into_response(),
            _ => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

fn fresh_server() -> McpServer {
    McpServer::new(Box::new(InMemoryToolRegistry::new()), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)))
}

fn transport() -> HttpTransport {
    HttpTransport::new(HttpTransportOptions::fixture("test-token"))
}

fn test_driver(server: McpServer) -> (HttpTestDriver, HttpEventPublisher) {
    let transport = transport();
    let events = Arc::new(Mutex::new(EventLog::default()));
    let state = HttpState { server: Arc::new(Mutex::new(server)), admission: transport.options.admission.clone(), allowed_origins: Arc::new(transport.options.allowed_origins.clone()), events: events.clone() };
    (HttpTestDriver { state }, HttpEventPublisher { events })
}

fn post_request(body: serde_json::Value, headers: &[(&str, &str)]) -> Request<Body> {
    let mut builder = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer test-token");
    for (name, value) in headers {
        builder = builder.header(*name, *value);
    }
    builder.body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap()
}

async fn one_shot(driver: HttpTestDriver, request: Request<Body>) -> Response {
    driver.request(request).await
}

fn owned_head(method: &str, path: &str, headers: &[(&str, &str)], body_len: usize) -> ParsedHttpHead {
    let mut head = ParsedHttpHead::new();
    head.method = Some(method.to_string());
    head.path = Some(path.to_string());
    head.version = Some("HTTP/1.1".to_string());
    head.header_end = Some(0);
    head.content_length = body_len;
    for (name, value) in headers {
        head.headers[head.header_count] = Some(OwnedHeader { name: name.to_ascii_lowercase(), value: value.to_string() });
        head.header_count += 1;
    }
    head
}

fn owned_response_body(response: &OwnedHttpResponse) -> &[u8] {
    let split = response.bytes.windows(4).position(|window| window == b"\r\n\r\n").expect("owned response header terminator");
    &response.bytes[split + 4..]
}

#[tokio::test]
async fn owned_health_dispatch_matches_the_current_axum_adapter() {
    let body = serde_json::to_vec(&serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" })).unwrap();
    let (owned_state, _) = test_driver(fresh_server());
    let head = owned_head("POST", "/mcp", &[("authorization", "Bearer test-token")], body.len());
    let owned = dispatch_owned_http(&owned_state.state, &head, &body).unwrap();
    assert!(owned.bytes.starts_with(b"HTTP/1.1 200 OK\r\n"));

    let (axum_driver, _) = test_driver(fresh_server());
    let axum = one_shot(axum_driver, post_request(serde_json::from_slice(&body).unwrap(), &[])).await;
    assert_eq!(axum.status(), StatusCode::OK);
    let axum_body = axum::body::to_bytes(axum.into_body(), HTTP_RESPONSE_BYTES).await.unwrap();
    let owned_json: serde_json::Value = serde_json::from_slice(owned_response_body(&owned)).unwrap();
    let axum_json: serde_json::Value = serde_json::from_slice(&axum_body).unwrap();
    assert_eq!(owned_json, axum_json);
}

#[test]
fn owned_bridge_handshake_matches_current_rfc_message_close_and_error_contracts() {
    let head = owned_head("GET", "/bridge", &[("connection", "Upgrade"), ("upgrade", "websocket"), ("sec-websocket-version", "13"), ("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ=="), ("sec-websocket-protocol", "semio.mcp.bridge.v1, test-token")], 0);
    let admission = HttpAdmission::Fixture(Arc::from(b"test-token".as_slice()));
    let response = dispatch_owned_bridge_handshake(&admission, &head).unwrap();
    let text = String::from_utf8(response.bytes).unwrap();
    assert!(text.starts_with("HTTP/1.1 101 Switching Protocols\r\n"));
    assert!(text.contains("Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo="));
    assert!(text.contains("Sec-WebSocket-Protocol: semio.mcp.bridge.v1"));

    let hello = quick::masked_client_frame(
        0x2,
        &crate::bridge::ShellToGateway::Hello {
            bridge_version: crate::bridge::BRIDGE_VERSION,
            shell_kind: crate::bridge::ShellKind::React,
            shell_session_id: "owned".into(),
            principal_actor: "agent:local".into(),
            flags: crate::bridge::BridgeFlags::NONE,
        }
        .encode(),
        true,
    );
    assert!(matches!(decode_client_websocket_frame(&hello).unwrap().unwrap().opcode, 0x2));
    let close = quick::masked_client_frame(0x8, &[], true);
    assert_eq!(decode_client_websocket_frame(&close).unwrap().unwrap().opcode, 0x8);
    let text_frame = quick::masked_client_frame(0x1, b"unsupported", true);
    assert!(matches!(decode_client_websocket_frame(&text_frame), Err(HttpTerminalReason::Unsupported)));
}

//#region 🔖️PostModern
#[tokio::test]
async fn modern_tools_list_over_http_returns_200_with_the_json_rpc_result() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
    let request = post_request(body, &[("MCP-Protocol-Version", "2026-07-28")]);
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["result"]["resultType"], "complete");
}

#[tokio::test]
async fn legacy_initialize_over_http_returns_200_and_negotiates_legacy() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": { "name": "t", "version": "0" } } });
    let request = post_request(body, &[]);
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["result"]["protocolVersion"], "2025-11-25");
}

#[tokio::test]
async fn a_notification_over_http_is_202_accepted_with_no_body() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "method": "notifications/cancelled" });
    let request = post_request(body, &[]);
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert!(bytes.is_empty());
}
//#endregion 🔖️PostModern

//#region 🔖️ProtocolVersionHeader
#[tokio::test]
async fn missing_protocol_version_header_on_a_modern_request_is_400_header_mismatch() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
    let request = post_request(body, &[]);
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["error"]["code"], HEADER_MISMATCH);
}

#[tokio::test]
async fn mismatched_protocol_version_header_and_body_is_400_header_mismatch() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
    let request = post_request(body, &[("MCP-Protocol-Version", "2025-11-25")]);
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["error"]["code"], HEADER_MISMATCH);
}

#[tokio::test]
async fn unsupported_protocol_version_is_400_with_the_supported_list() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "1999-01-01" } } });
    let request = post_request(body, &[("MCP-Protocol-Version", "1999-01-01")]);
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["error"]["code"], UNSUPPORTED_PROTOCOL_VERSION);
    assert_eq!(value["error"]["data"]["supported"], serde_json::json!(SUPPORTED_PROTOCOL_VERSIONS));
}
//#endregion 🔖️ProtocolVersionHeader

//#region 🔖️Security
#[tokio::test]
async fn an_evil_origin_is_rejected_with_403() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
    let request =
        Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer test-token").header("origin", "https://evil.example").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn a_loopback_origin_is_accepted() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
    let request =
        Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer test-token").header("origin", "http://127.0.0.1:6300").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn missing_bearer_token_is_401() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
    let request = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn incorrect_bearer_token_is_401() {
    let (router, _events) = test_driver(fresh_server());
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
    let request = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer wrong-token").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
//#endregion 🔖️Security

//#region 🔖️BothErasOverHttp
#[tokio::test]
async fn both_eras_are_served_over_the_same_http_endpoint_by_the_same_server() {
    let mut tools = InMemoryToolRegistry::new();
    tools.register(Tool::new("ping_tool", serde_json::json!({"type":"object"})), |_arguments| crate::protocol::CallToolResult::ok(vec![], None)).unwrap();
    let server = McpServer::new(Box::new(tools), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)));
    let (router, _events) = test_driver(server);

    let modern_body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "ping_tool", "arguments": {}, "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
    let modern_request = post_request(modern_body, &[("MCP-Protocol-Version", "2026-07-28")]);
    let modern_response = one_shot(router.clone(), modern_request).await;
    assert_eq!(modern_response.status(), StatusCode::OK);

    let legacy_body = serde_json::json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": { "name": "ping_tool", "arguments": {} } });
    let legacy_request = post_request(legacy_body, &[]);
    let legacy_response = one_shot(router, legacy_request).await;
    assert_eq!(legacy_response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(legacy_response.into_body(), usize::MAX).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["result"]["isError"], false);
}
//#endregion 🔖️BothErasOverHttp

//#region 🔖️GetSseResumption
#[tokio::test]
async fn get_with_no_last_event_id_replays_every_buffered_notification() {
    let (router, events) = test_driver(fresh_server());
    events.push(JsonRpcNotification::new("notifications/tools/list_changed", None));
    events.push(JsonRpcNotification::new("notifications/resources/list_changed", None));

    let request = Request::builder().method("GET").uri("/mcp").header("authorization", "Bearer test-token").body(Body::empty()).unwrap();
    let response = one_shot(router, request).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get(axum::http::header::CONTENT_TYPE).unwrap(), "text/event-stream");
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(text.contains("id: 1"));
    assert!(text.contains("id: 2"));
    assert!(text.contains("notifications/tools/list_changed"));
}

#[tokio::test]
async fn get_with_last_event_id_resumes_after_that_id_only() {
    let (router, events) = test_driver(fresh_server());
    events.push(JsonRpcNotification::new("notifications/tools/list_changed", None));
    let second_id = events.push(JsonRpcNotification::new("notifications/resources/list_changed", None));
    let _ = second_id;

    let request = Request::builder().method("GET").uri("/mcp").header("authorization", "Bearer test-token").header("Last-Event-ID", "1").body(Body::empty()).unwrap();
    let response = one_shot(router, request).await;
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(!text.contains("tools/list_changed"), "must not replay an event at-or-before Last-Event-ID");
    assert!(text.contains("resources/list_changed"), "must replay events after Last-Event-ID");
}
//#endregion 🔖️GetSseResumption

//#region 🔖️BridgeOnTheMergedApp
/// 🌉️ P1c acceptance: `/bridge` is mounted on the SAME app `/mcp` lives on — this test binds a
/// real ephemeral socket to the ACTUAL `HttpTransport::router()` output (not a bridge-only
/// router), connects a real `tokio-tungstenite` client, and drives the full scenario
/// `📓️sol-P1c-packet.md`'s acceptance list names: `Hello`→`Welcome`, a `ShellState` publish, a
/// pushed `ShellCommand` answered by a `ShellCommandResult`, a wrong-token rejection, and a bad-
/// `Origin` rejection — all in one foreground `#[tokio::test]`, no background process left running.
#[tokio::test]
async fn bridge_is_live_on_the_same_merged_app_run_http_builds() {
    use crate::bridge::{BRIDGE_VERSION, BridgeFlags, ShellKind, ShellToGateway};
    use futures::{SinkExt, StreamExt};
    use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
    use tokio_tungstenite::tungstenite::client::IntoClientRequest;

    let transport = HttpTransport::new(HttpTransportOptions::fixture("mcp-bearer"));
    let (router, _events, bridge_handle) = transport.router(fresh_server());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server_task = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    // /mcp still answers on the SAME bound socket the bridge is about to connect to — proves
    // both endpoints really are one merged app, not two separate servers on the same port by
    // coincidence.
    let mcp_client = reqwest_free_post(addr, "/mcp", "Bearer mcp-bearer", &serde_json::json!({"jsonrpc":"2.0","id":1,"method":"ping"})).await;
    assert_eq!(mcp_client, 200);

    // Hello -> Welcome.
    let mut bridge_request = format!("ws://{addr}/bridge").into_client_request().unwrap();
    bridge_request.headers_mut().insert("sec-websocket-protocol", "semio.mcp.bridge.v1, mcp-bearer".parse().unwrap());
    let (mut socket, response) = tokio_tungstenite::connect_async(bridge_request).await.expect("client connects with protected bridge admission");
    assert_eq!(response.headers().get("sec-websocket-protocol").and_then(|value| value.to_str().ok()), Some("semio.mcp.bridge.v1"));
    let hello = ShellToGateway::Hello { bridge_version: BRIDGE_VERSION, shell_kind: ShellKind::React, shell_session_id: "shell-live".into(), principal_actor: "agent:local".into(), flags: BridgeFlags::NONE };
    socket.send(TungsteniteMessage::Binary(hello.encode().into())).await.unwrap();
    let welcome_bytes = match socket.next().await.unwrap().unwrap() {
        TungsteniteMessage::Binary(bytes) => bytes,
        other => panic!("expected a binary Welcome frame, got {other:?}"),
    };
    assert!(matches!(crate::bridge::GatewayToShell::decode(&welcome_bytes).unwrap(), crate::bridge::GatewayToShell::Welcome { .. }));

    // ShellState publish becomes visible through BridgeHandle.
    let id = bridge_handle.connections().first().copied().expect("one live connection");
    let state_frame = ShellToGateway::ShellState { revision: 1, state: vec![7] };
    socket.send(TungsteniteMessage::Binary(state_frame.clone().encode().into())).await.unwrap();
    socket.send(TungsteniteMessage::Binary(ShellToGateway::Ping.encode().into())).await.unwrap();
    let _pong = socket.next().await.unwrap().unwrap();
    assert_eq!(bridge_handle.last_shell_state(id), Some(state_frame));

    // A pushed ShellCommand reaches the client, whose ShellCommandResult becomes visible.
    let pushed = crate::bridge::GatewayToShell::ShellCommand { seq: 1, command: vec![9] };
    assert!(bridge_handle.send_to(id, pushed.clone()));
    let received_bytes = match socket.next().await.unwrap().unwrap() {
        TungsteniteMessage::Binary(bytes) => bytes,
        other => panic!("expected a binary ShellCommand frame, got {other:?}"),
    };
    assert_eq!(crate::bridge::GatewayToShell::decode(&received_bytes).unwrap(), pushed);
    let result_frame = ShellToGateway::ShellCommandResult { in_reply_to: 1, ok: true, fault: None };
    socket.send(TungsteniteMessage::Binary(result_frame.encode().into())).await.unwrap();
    socket.send(TungsteniteMessage::Binary(ShellToGateway::Ping.encode().into())).await.unwrap();
    let _pong2 = socket.next().await.unwrap().unwrap();
    assert_eq!(bridge_handle.last_command_result(id), Some((1, true, None)));
    drop(socket);

    // Wrong token is rejected before the upgrade completes.
    let mut wrong_request = format!("ws://{addr}/bridge").into_client_request().unwrap();
    wrong_request.headers_mut().insert("sec-websocket-protocol", "semio.mcp.bridge.v1, nope".parse().unwrap());
    let wrong_token = tokio_tungstenite::connect_async(wrong_request).await;
    assert!(wrong_token.is_err(), "a mismatched bridge token must never complete the websocket handshake");

    // Bad Origin is rejected before the upgrade completes.
    let mut bad_origin_request = format!("ws://{addr}/bridge").into_client_request().unwrap();
    bad_origin_request.headers_mut().insert("sec-websocket-protocol", "semio.mcp.bridge.v1, mcp-bearer".parse().unwrap());
    bad_origin_request.headers_mut().insert("origin", "https://evil.example".parse().unwrap());
    let bad_origin = tokio_tungstenite::connect_async(bad_origin_request).await;
    assert!(bad_origin.is_err(), "a non-loopback Origin must never complete the websocket handshake");

    server_task.abort();
}

/// 🧰️ The smallest possible real HTTP/1.1 POST over a bound TCP socket — used ONLY by the
/// merged-app test above, which already needs a real listener for the websocket half; avoids
/// adding a `reqwest` dependency for one confirming assertion by speaking just enough HTTP/1.1 by
/// hand (`Connection: close`, read to EOF, parse the status line) for a single fire-and-forget
/// request/response.
async fn reqwest_free_post(addr: SocketAddr, path: &str, bearer: &str, body: &serde_json::Value) -> u16 {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let body_bytes = serde_json::to_vec(body).unwrap();
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    let request = format!("POST {path} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: {bearer}\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n", body_bytes.len());
    stream.write_all(request.as_bytes()).await.unwrap();
    stream.write_all(&body_bytes).await.unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    let response_text = String::from_utf8_lossy(&response);
    let status_line = response_text.lines().next().unwrap_or("");
    status_line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0)
}
//#endregion 🔖️BridgeOnTheMergedApp
