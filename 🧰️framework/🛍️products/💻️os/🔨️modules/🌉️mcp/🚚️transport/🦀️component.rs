//! 🚚️ Transport-level framing over an arbitrary duplex channel — `StdioTransport` (P1a, newline-
//! delimited JSON-RPC on stdin/stdout) and `HttpTransport` (P1b, Streamable HTTP over axum/tokio,
//! `📓️luna-mcpspec-audit.md` §A.7 + the spec page fetched live for this packet — resumable legacy GET
//! is this gateway's own dual-era accommodation, NOT part of the 2026-07-28 revision, which removed
//! the GET stream endpoint and protocol-level sessions entirely; see this file's `🔖️HttpTransport`
//! region doc for the exact split). **All diagnostic output goes to a SEPARATE log writer for stdio,
//! never to the response stream** — a stray byte on stdout corrupts every later line the client tries
//! to parse as JSON-RPC (`luna-mcpspec-audit.md`'s stdio guidance).
//!
//! `McpTransport::serve` takes `server: McpServer` BY VALUE (a deviation from P1a's `&mut McpServer`
//! — see this packet's report §7): `HttpTransport` must own the server for the `'static` lifetime
//! axum's connection-per-task model requires, and by-value ownership transfer is the only signature
//! that lets ONE trait serve both a single-shot stdio process and a long-lived multi-connection HTTP
//! server without `McpServer`/`protocol.rs` itself changing at all (which this packet must not touch).

use crate::errors::{GatewayError, GatewayErrorCode};
use crate::protocol::{extract_meta_protocol_version, JsonRpcId, JsonRpcIncoming, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, McpServer, PARSE_ERROR, SUPPORTED_PROTOCOL_VERSIONS, UNSUPPORTED_PROTOCOL_VERSION};
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::Router;
use std::collections::VecDeque;
use std::io::{BufRead, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

//#region 🔖️McpTransport
/// 🔌️ Drives one [`McpServer`] to completion over whatever duplex channel a concrete implementor
/// speaks — stdio (one process = one connection) or HTTP (one process = every connection, sharing the
/// one `McpServer` exactly as the stdio case shares its one pipe; see this file's module doc).
pub trait McpTransport {
    fn serve(&mut self, server: McpServer) -> Result<(), GatewayError>;
}
//#endregion 🔖️McpTransport

//#region 🔖️StdioTransport
fn io_error(error: std::io::Error) -> GatewayError {
    GatewayError::new(GatewayErrorCode::Internal, format!("stdio transport io error: {error}"))
}

/// 📻️ Newline-delimited JSON-RPC over `input`/`output`, with a THIRD writer (`log`) for every
/// diagnostic line — generic over the three streams so tests exercise the exact same code path a real
/// `stdin`/`stdout`/`stderr` wiring uses, in-memory, without touching the process's real file
/// descriptors.
pub struct StdioTransport<R: BufRead, W: Write, L: Write> {
    input: R,
    output: W,
    log: L,
}

impl<R: BufRead, W: Write, L: Write> StdioTransport<R, W, L> {
    pub fn new(input: R, output: W, log: L) -> Self {
        Self { input, output, log }
    }

    fn write_line(&mut self, line: &str) -> Result<(), GatewayError> {
        writeln!(self.output, "{line}").map_err(io_error)?;
        self.output.flush().map_err(io_error)
    }

    fn write_response(&mut self, response: &JsonRpcResponse) -> Result<(), GatewayError> {
        let line = serde_json::to_string(response).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        self.write_line(&line)
    }

    fn log_line(&mut self, line: &str) {
        let _ = writeln!(self.log, "{line}");
    }
}

impl<R: BufRead, W: Write, L: Write> McpTransport for StdioTransport<R, W, L> {
    /// 🔁️ One line in → zero-or-one line out, until EOF (client closed stdin) or a hard io error. A
    /// blank line is skipped silently (not an error — some clients send a trailing newline). A batch
    /// that dispatches to zero responses (all-notification batch) writes nothing, per JSON-RPC.
    fn serve(&mut self, mut server: McpServer) -> Result<(), GatewayError> {
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = self.input.read_line(&mut line).map_err(io_error)?;
            if bytes_read == 0 {
                return Ok(());
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            match serde_json::from_str::<JsonRpcIncoming>(trimmed) {
                Ok(JsonRpcIncoming::Single(request)) => {
                    if let Some(response) = server.dispatch(&request) {
                        self.write_response(&response)?;
                    }
                }
                Ok(JsonRpcIncoming::Batch(requests)) => {
                    let responses = server.dispatch_batch(&requests);
                    if !responses.is_empty() {
                        let line = serde_json::to_string(&responses).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
                        self.write_line(&line)?;
                    }
                }
                Err(error) => {
                    self.log_line(&format!("stdio transport: malformed JSON-RPC line rejected: {error}"));
                    let response = JsonRpcResponse::error(JsonRpcId::Null, PARSE_ERROR, format!("parse error: {error}"), None);
                    self.write_response(&response)?;
                }
            }
        }
    }
}
//#endregion 🔖️StdioTransport

//#region 🔖️HttpTransport
/// 🔢️ `HeaderMismatch` (Streamable HTTP, `MCP-Protocol-Version` header vs body `_meta` mismatch) —
/// transport-local because it is an HTTP-framing concern the stdio-only `protocol.rs` never needs.
pub const HEADER_MISMATCH: i64 = -32020;

/// ⚙️ `HttpTransport` construction options — bind address (default `127.0.0.1:6300`, never
/// `0.0.0.0`), the required `/mcp` bearer token, the SEPARATE `/bridge` query-string token (P1c —
/// freshly minted per process start, distinct secret from the MCP bearer, see
/// `📓️sol-P1c-packet.md`'s design), and an explicit `Origin` allowlist beyond the always-allowed
/// loopback/`null` set (shared by both endpoints).
#[derive(Debug, Clone)]
pub struct HttpTransportOptions {
    pub bind_addr: SocketAddr,
    pub bearer_token: String,
    pub bridge_token: String,
    pub allowed_origins: Vec<String>,
}

impl HttpTransportOptions {
    pub fn new(bearer_token: impl Into<String>, bridge_token: impl Into<String>) -> Self {
        Self { bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6300), bearer_token: bearer_token.into(), bridge_token: bridge_token.into(), allowed_origins: Vec::new() }
    }

    pub fn bind_addr(mut self, addr: SocketAddr) -> Self {
        self.bind_addr = addr;
        self
    }

    pub fn allowed_origins(mut self, origins: Vec<String>) -> Self {
        self.allowed_origins = origins;
        self
    }
}

/// 🌊️ A bounded ring buffer of server-initiated notifications, each stamped with a monotonic id — the
/// resumption source for the legacy `GET` stream's `Last-Event-ID` support. Capacity-bounded so a
/// long-running gateway with no GET client attached never grows this without limit.
#[derive(Default)]
struct EventLog {
    next_id: u64,
    entries: VecDeque<(u64, JsonRpcNotification)>,
}

impl EventLog {
    const CAPACITY: usize = 256;

    fn push(&mut self, notification: JsonRpcNotification) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.entries.push_back((id, notification));
        if self.entries.len() > Self::CAPACITY {
            self.entries.pop_front();
        }
        id
    }

    fn since(&self, last_id: Option<u64>) -> Vec<(u64, JsonRpcNotification)> {
        let floor = last_id.unwrap_or(0);
        self.entries.iter().filter(|(id, _)| *id > floor).cloned().collect()
    }
}

/// 📤️ A cloneable handle a test (or a future notification-emitting call site) uses to push a
/// server-initiated notification into the same [`EventLog`] the legacy GET handler reads from.
#[derive(Clone)]
pub struct HttpEventPublisher {
    events: Arc<Mutex<EventLog>>,
}

impl HttpEventPublisher {
    pub fn push(&self, notification: JsonRpcNotification) -> u64 {
        self.events.lock().expect("event log lock poisoned").push(notification)
    }
}

#[derive(Clone)]
struct HttpState {
    server: Arc<Mutex<McpServer>>,
    bearer_token: Arc<str>,
    allowed_origins: Arc<Vec<String>>,
    events: Arc<Mutex<EventLog>>,
}

/// 🌐️ Streamable HTTP transport (`📌️sol-P1b-packet.md` §2.1) — a single `/mcp` endpoint: `POST` for
/// every JSON-RPC request/notification of EITHER era (dispatched through the exact same [`McpServer`]
/// stdio uses, no era logic duplicated here), `GET` for this gateway's legacy resumable server→client
/// stream (`Last-Event-ID` replay against [`EventLog`]) — the 2026-07-28 revision itself removed GET
/// entirely, but this dual-era gateway also serves `2025-11-25`/`2025-06-18` clients, whose Streamable
/// HTTP shape still has one (`📓️design-decisions.md` D1). Every request is checked against `Origin`
/// (reject non-loopback/non-`null`/non-allowlisted with `403`) and the bearer token (`401`) before
/// anything else runs.
pub struct HttpTransport {
    options: HttpTransportOptions,
}

impl HttpTransport {
    pub fn new(options: HttpTransportOptions) -> Self {
        Self { options }
    }

    /// 🧪️ Builds the real axum [`Router`] WITHOUT binding a socket — the foreground, deterministic
    /// entry point every `/mcp` HTTP test in this crate drives via `tower::ServiceExt::oneshot` (no
    /// port allocation, no background process, no timing races). `/bridge` is a real websocket
    /// upgrade, which `oneshot` cannot drive (it never performs a genuine hyper connection upgrade) —
    /// tests exercising `/bridge` bind a real ephemeral (`:0`) socket instead (`mod long` in this file
    /// and in `🧵️bridge/🦀️component.rs`), same as P1a/P1b's own websocket tests already did.
    pub fn router(&self, server: McpServer) -> (Router, HttpEventPublisher, crate::bridge::BridgeHandle) {
        let events = Arc::new(Mutex::new(EventLog::default()));
        let state = HttpState { server: Arc::new(Mutex::new(server)), bearer_token: Arc::from(self.options.bearer_token.as_str()), allowed_origins: Arc::new(self.options.allowed_origins.clone()), events: events.clone() };
        let mcp_router = Router::new().route("/mcp", post(handle_post).get(handle_get)).with_state(state);
        let (bridge_router, bridge_handle) = crate::bridge::server::bridge_router(self.options.bridge_token.clone(), self.options.allowed_origins.clone());
        let router = mcp_router.merge(bridge_router);
        (router, HttpEventPublisher { events }, bridge_handle)
    }

    /// ▶️ Binds `self.options.bind_addr` and serves forever (`/mcp` + `/bridge` on the SAME socket) —
    /// the real binary's entry point.
    pub async fn run(&self, server: McpServer) -> Result<(), GatewayError> {
        let (router, _events, _bridge) = self.router(server);
        let listener = tokio::net::TcpListener::bind(self.options.bind_addr).await.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot bind {}: {error}", self.options.bind_addr)))?;
        axum::serve(listener, router).await.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("http transport io error: {error}")))
    }
}

impl McpTransport for HttpTransport {
    fn serve(&mut self, server: McpServer) -> Result<(), GatewayError> {
        let runtime = tokio::runtime::Runtime::new().map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot start tokio runtime: {error}")))?;
        runtime.block_on(self.run(server))
    }
}

//#region 🔖️OriginAndBearer
fn is_loopback_or_null(origin: &str) -> bool {
    if origin.eq_ignore_ascii_case("null") {
        return true;
    }
    let Some(after_scheme) = origin.split("://").nth(1) else { return false };
    let host_and_port = after_scheme.split('/').next().unwrap_or("");
    let host = if let Some(bracket_end) = host_and_port.strip_prefix('[') { bracket_end.split(']').next().unwrap_or("") } else { host_and_port.rsplit_once(':').map(|(host, _)| host).unwrap_or(host_and_port) };
    matches!(host, "localhost" | "127.0.0.1" | "::1")
}

/// 🔓️ Also used by `🧵️bridge`'s websocket upgrade handler (`pub(crate)`, same crate, one Origin
/// policy shared by both `/mcp` and `/bridge`) — never made fully `pub`, this is an internal seam.
pub(crate) fn origin_allowed(origin: Option<&str>, allowed_origins: &[String]) -> bool {
    match origin {
        None => true,
        Some(value) => is_loopback_or_null(value) || allowed_origins.iter().any(|allowed| allowed == value),
    }
}

/// 🔓️ Also used by `🧵️bridge`'s `?token=` comparison (`pub(crate)`) — one constant-time comparison
/// helper, not duplicated.
pub(crate) fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn bearer_matches(headers: &HeaderMap, expected: &str) -> bool {
    let Some(value) = headers.get(axum::http::header::AUTHORIZATION).and_then(|value| value.to_str().ok()) else { return false };
    let Some(token) = value.strip_prefix("Bearer ") else { return false };
    constant_time_eq(token.as_bytes(), expected.as_bytes())
}

fn reject_by_origin(headers: &HeaderMap, state: &HttpState) -> Option<Response> {
    let origin = headers.get(axum::http::header::ORIGIN).and_then(|value| value.to_str().ok());
    if origin_allowed(origin, &state.allowed_origins) {
        None
    } else {
        Some((StatusCode::FORBIDDEN, "origin not allowed").into_response())
    }
}

fn reject_by_bearer(headers: &HeaderMap, state: &HttpState) -> Option<Response> {
    if bearer_matches(headers, &state.bearer_token) {
        None
    } else {
        Some((StatusCode::UNAUTHORIZED, "missing or invalid bearer token").into_response())
    }
}
//#endregion 🔖️OriginAndBearer

fn json_rpc_error_response(status: StatusCode, id: JsonRpcId, code: i64, message: String, data: Option<serde_json::Value>) -> Response {
    let response = JsonRpcResponse::error(id, code, message, data);
    (status, axum::Json(response)).into_response()
}

/// 📮️ `POST /mcp` — the single endpoint for every JSON-RPC request/notification, either era. Modern
/// requests (a body carrying `_meta.protocolVersion`) additionally require the `MCP-Protocol-Version`
/// header to be PRESENT and to MATCH the body value (`400 HeaderMismatch` otherwise, per the spec page
/// fetched for this packet — see this file's module doc); legacy requests (`initialize`, or any
/// already-negotiated legacy call) pass straight through to [`McpServer::dispatch`], exactly as stdio
/// does. A JSON-RPC notification (absent `id`) is `202 Accepted` with no body; a request gets `200 OK`
/// `application/json`.
async fn handle_post(State(state): State<HttpState>, headers: HeaderMap, body: axum::body::Bytes) -> Response {
    if let Some(response) = reject_by_origin(&headers, &state) {
        return response;
    }
    if let Some(response) = reject_by_bearer(&headers, &state) {
        return response;
    }

    let request: JsonRpcRequest = match serde_json::from_slice(&body) {
        Ok(request) => request,
        Err(error) => return json_rpc_error_response(StatusCode::BAD_REQUEST, JsonRpcId::Null, PARSE_ERROR, format!("parse error: {error}"), None),
    };

    if let Some(requested) = extract_meta_protocol_version(request.params.as_ref()) {
        let header_value = headers.get("MCP-Protocol-Version").and_then(|value| value.to_str().ok());
        let fallback_id = || request.id.clone().unwrap_or(JsonRpcId::Null);
        match header_value {
            None => return json_rpc_error_response(StatusCode::BAD_REQUEST, fallback_id(), HEADER_MISMATCH, "missing required MCP-Protocol-Version header".to_string(), None),
            Some(header_value) if header_value != requested => {
                return json_rpc_error_response(StatusCode::BAD_REQUEST, fallback_id(), HEADER_MISMATCH, format!("MCP-Protocol-Version header `{header_value}` does not match body protocol version `{requested}`"), None);
            }
            Some(_) => {}
        }
        if !SUPPORTED_PROTOCOL_VERSIONS.contains(&requested.as_str()) {
            return json_rpc_error_response(
                StatusCode::BAD_REQUEST,
                fallback_id(),
                UNSUPPORTED_PROTOCOL_VERSION,
                format!("unsupported protocol version: {requested}"),
                Some(serde_json::json!({ "supported": SUPPORTED_PROTOCOL_VERSIONS, "requested": requested })),
            );
        }
    }

    let mut server = state.server.lock().expect("mcp server lock poisoned");
    match server.dispatch(&request) {
        None => StatusCode::ACCEPTED.into_response(),
        Some(response) => (StatusCode::OK, axum::Json(response)).into_response(),
    }
}

/// 📡️ `GET /mcp` — this gateway's legacy resumable server→client stream: replays every buffered
/// notification with an id greater than the `Last-Event-ID` header (absent ⇒ replay everything
/// buffered), each formatted as one SSE `id:`/`event:`/`data:` block, then closes. A real `EventSource`
/// client reconnects (carrying the last id it saw) to keep watching — this endpoint's job is correct
/// resumption on each such (re)connect, not holding one socket open forever.
async fn handle_get(State(state): State<HttpState>, headers: HeaderMap) -> Response {
    if let Some(response) = reject_by_origin(&headers, &state) {
        return response;
    }
    if let Some(response) = reject_by_bearer(&headers, &state) {
        return response;
    }
    let last_event_id = headers.get("Last-Event-ID").and_then(|value| value.to_str().ok()).and_then(|value| value.parse::<u64>().ok());
    let events = state.events.lock().expect("event log lock poisoned").since(last_event_id);
    let mut body = String::new();
    for (id, notification) in events {
        let data = serde_json::to_string(&notification).unwrap_or_default();
        body.push_str(&format!("id: {id}\nevent: message\ndata: {data}\n\n"));
    }
    Response::builder().status(StatusCode::OK).header(axum::http::header::CONTENT_TYPE, "text/event-stream").header(axum::http::header::CACHE_CONTROL, "no-cache").body(Body::from(body)).unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}
//#endregion 🔖️HttpTransport

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::protocol::{InMemoryPromptRegistry, InMemoryResourceRegistry, InMemoryToolRegistry, NullBackend};
    use std::io::Cursor;

    fn fresh_server() -> McpServer {
        McpServer::new(Box::new(InMemoryToolRegistry::new()), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(NullBackend))
    }

    //#region 🔖️Stdio
    #[test]
    fn one_request_line_produces_exactly_one_response_line_on_stdout() {
        let input = Cursor::new(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\"}\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(fresh_server()).unwrap();

        let output_text = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_text.lines().collect();
        assert_eq!(lines.len(), 1);
        let response: JsonRpcResponse = serde_json::from_str(lines[0]).unwrap();
        assert!(!response.is_error());
    }

    #[test]
    fn malformed_json_logs_to_the_log_writer_and_never_pollutes_stdout_with_non_json_text() {
        let input = Cursor::new(b"not json at all\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(fresh_server()).unwrap();

        let log_text = String::from_utf8(log).unwrap();
        assert!(log_text.contains("malformed JSON-RPC line rejected"), "diagnostic text must land in the log writer");

        let output_text = String::from_utf8(output).unwrap();
        for line in output_text.lines() {
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(line);
            assert!(parsed.is_ok(), "every stdout line must be valid JSON, got: {line}");
        }
        assert!(!output_text.contains("malformed JSON-RPC line rejected"), "stdout must never carry a diagnostic line");
    }

    #[test]
    fn blank_lines_are_skipped_without_producing_output() {
        let input = Cursor::new(b"\n\n{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\"}\n\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(fresh_server()).unwrap();
        let output_text = String::from_utf8(output).unwrap();
        assert_eq!(output_text.lines().count(), 1);
    }

    #[test]
    fn a_notification_line_produces_no_output_line_at_all() {
        let input = Cursor::new(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\"}\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(fresh_server()).unwrap();
        assert!(output.is_empty());
    }

    #[test]
    fn eof_ends_the_loop_cleanly() {
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        assert!(transport.serve(fresh_server()).is_ok());
        assert!(output.is_empty());
    }
    //#endregion 🔖️Stdio

    //#region 🔖️OriginAndBearerUnit
    #[test]
    fn loopback_and_null_origins_are_always_allowed() {
        assert!(is_loopback_or_null("null"));
        assert!(is_loopback_or_null("http://127.0.0.1:6300"));
        assert!(is_loopback_or_null("http://localhost:6300"));
        assert!(is_loopback_or_null("http://[::1]:6300"));
        assert!(!is_loopback_or_null("https://evil.example"));
    }

    #[test]
    fn origin_allowed_falls_back_to_the_explicit_allowlist() {
        assert!(origin_allowed(None, &[]));
        assert!(!origin_allowed(Some("https://evil.example"), &[]));
        assert!(origin_allowed(Some("https://trusted.example"), &["https://trusted.example".to_string()]));
    }

    #[test]
    fn constant_time_eq_matches_regular_equality() {
        assert!(constant_time_eq(b"secret", b"secret"));
        assert!(!constant_time_eq(b"secret", b"secre1"));
        assert!(!constant_time_eq(b"secret", b"longer-secret"));
    }
    //#endregion 🔖️OriginAndBearerUnit
}

/// 🌐️ HTTP integration tests — every one drives the REAL `axum::Router` via `oneshot`, in-process, no
/// bound socket, no background process (`📌️important.md` rule 5: foreground only).
#[cfg(test)]
mod long {
    use super::*;
    use crate::protocol::{InMemoryPromptRegistry, InMemoryResourceRegistry, InMemoryToolRegistry, McpServer, NullBackend, Tool, META_PROTOCOL_VERSION_KEY};
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    fn fresh_server() -> McpServer {
        McpServer::new(Box::new(InMemoryToolRegistry::new()), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(NullBackend))
    }

    fn transport() -> HttpTransport {
        HttpTransport::new(HttpTransportOptions::new("test-token", "bridge-token"))
    }

    fn post_request(body: serde_json::Value, headers: &[(&str, &str)]) -> Request<Body> {
        let mut builder = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer test-token");
        for (name, value) in headers {
            builder = builder.header(*name, *value);
        }
        builder.body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap()
    }

    //#region 🔖️PostModern
    #[tokio::test]
    async fn modern_tools_list_over_http_returns_200_with_the_json_rpc_result() {
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
        let request = post_request(body, &[("MCP-Protocol-Version", "2026-07-28")]);
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["result"]["resultType"], "complete");
    }

    #[tokio::test]
    async fn legacy_initialize_over_http_returns_200_and_negotiates_legacy() {
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": { "name": "t", "version": "0" } } });
        let request = post_request(body, &[]);
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["result"]["protocolVersion"], "2025-11-25");
    }

    #[tokio::test]
    async fn a_notification_over_http_is_202_accepted_with_no_body() {
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "method": "notifications/cancelled" });
        let request = post_request(body, &[]);
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(bytes.is_empty());
    }
    //#endregion 🔖️PostModern

    //#region 🔖️ProtocolVersionHeader
    #[tokio::test]
    async fn missing_protocol_version_header_on_a_modern_request_is_400_header_mismatch() {
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
        let request = post_request(body, &[]);
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["error"]["code"], HEADER_MISMATCH);
    }

    #[tokio::test]
    async fn mismatched_protocol_version_header_and_body_is_400_header_mismatch() {
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
        let request = post_request(body, &[("MCP-Protocol-Version", "2025-11-25")]);
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["error"]["code"], HEADER_MISMATCH);
    }

    #[tokio::test]
    async fn unsupported_protocol_version_is_400_with_the_supported_list() {
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "1999-01-01" } } });
        let request = post_request(body, &[("MCP-Protocol-Version", "1999-01-01")]);
        let response = router.oneshot(request).await.unwrap();
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
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
        let request = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer test-token").header("origin", "https://evil.example").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn a_loopback_origin_is_accepted() {
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
        let request = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer test-token").header("origin", "http://127.0.0.1:6300").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn missing_bearer_token_is_401() {
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
        let request = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn incorrect_bearer_token_is_401() {
        let (router, _events, _bridge) = transport().router(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
        let request = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer wrong-token").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    //#endregion 🔖️Security

    //#region 🔖️BothErasOverHttp
    #[tokio::test]
    async fn both_eras_are_served_over_the_same_http_endpoint_by_the_same_server() {
        let mut tools = InMemoryToolRegistry::new();
        tools.register(Tool::new("ping_tool", serde_json::json!({"type":"object"})), |_arguments| crate::protocol::CallToolResult::ok(vec![], None)).unwrap();
        let server = McpServer::new(Box::new(tools), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(NullBackend));
        let (router, _events, _bridge) = transport().router(server);

        let modern_body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "ping_tool", "arguments": {}, "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
        let modern_request = post_request(modern_body, &[("MCP-Protocol-Version", "2026-07-28")]);
        let modern_response = router.clone().oneshot(modern_request).await.unwrap();
        assert_eq!(modern_response.status(), StatusCode::OK);

        let legacy_body = serde_json::json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": { "name": "ping_tool", "arguments": {} } });
        let legacy_request = post_request(legacy_body, &[]);
        let legacy_response = router.oneshot(legacy_request).await.unwrap();
        assert_eq!(legacy_response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(legacy_response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["result"]["isError"], false);
    }
    //#endregion 🔖️BothErasOverHttp

    //#region 🔖️GetSseResumption
    #[tokio::test]
    async fn get_with_no_last_event_id_replays_every_buffered_notification() {
        let (router, events, _bridge) = transport().router(fresh_server());
        events.push(JsonRpcNotification::new("notifications/tools/list_changed", None));
        events.push(JsonRpcNotification::new("notifications/resources/list_changed", None));

        let request = Request::builder().method("GET").uri("/mcp").header("authorization", "Bearer test-token").body(Body::empty()).unwrap();
        let response = router.oneshot(request).await.unwrap();
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
        let (router, events, _bridge) = transport().router(fresh_server());
        events.push(JsonRpcNotification::new("notifications/tools/list_changed", None));
        let second_id = events.push(JsonRpcNotification::new("notifications/resources/list_changed", None));
        let _ = second_id;

        let request = Request::builder().method("GET").uri("/mcp").header("authorization", "Bearer test-token").header("Last-Event-ID", "1").body(Body::empty()).unwrap();
        let response = router.oneshot(request).await.unwrap();
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
        use crate::bridge::{BridgeFlags, ShellKind, ShellToGateway, BRIDGE_VERSION};
        use futures::{SinkExt, StreamExt};
        use tokio_tungstenite::tungstenite::client::IntoClientRequest;
        use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

        let transport = HttpTransport::new(HttpTransportOptions::new("mcp-bearer", "bridge-secret"));
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
        let (mut socket, _response) = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge?token=bridge-secret")).await.expect("client connects with the correct bridge token");
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
        let wrong_token = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge?token=nope")).await;
        assert!(wrong_token.is_err(), "a mismatched bridge token must never complete the websocket handshake");

        // Bad Origin is rejected before the upgrade completes.
        let mut bad_origin_request = format!("ws://{addr}/bridge?token=bridge-secret").into_client_request().unwrap();
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
}
//#endregion 🧪️Tests
