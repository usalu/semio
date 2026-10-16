//! 📬️ Owned HTTP response building and the POST, GET and bridge handshake dispatch.

use semio_framework_os_mcp_wire::protocol::{
    JsonRpcId,
    JsonRpcRequest,
    JsonRpcResponse,
    PARSE_ERROR,
    SUPPORTED_PROTOCOL_VERSIONS,
    UNSUPPORTED_PROTOCOL_VERSION,
    extract_meta_protocol_version,
};
use std::io::Write;
use super::http_surface::{HEADER_MISMATCH, HttpAdmission, HttpState, HttpTerminalReason};
use super::http_transport_state::{HTTP_RESPONSE_BYTES, ParsedHttpHead};
use super::origin_and_handlers::origin_allowed;
use super::websocket_codec::{websocket_accept, websocket_key_nonce};

//#region 🔖️HttpTransport
pub(crate) struct OwnedHttpResponse {
    pub bytes: Vec<u8>,
    pub(super) upgrade: bool,
}

struct CappedBytes {
    pub bytes: Vec<u8>,
    pub(super) cap: usize,
    exceeded: bool,
}

impl CappedBytes {
    pub(super) fn new(cap: usize) -> Self {
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(cap).expect("http fixed byte reservation failed");
        Self { bytes, cap, exceeded: false }
    }
}

impl Write for CappedBytes {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.cap.saturating_sub(self.bytes.len()) {
            self.exceeded = true;
            return Err(std::io::Error::other("fixed HTTP byte credit exhausted"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(crate) fn find_crlf_bounded(bytes: &[u8], start: usize, end: usize) -> Option<usize> {
    if start >= end || end > bytes.len() {
        return None;
    }
    bytes.get(start..end)?.windows(2).position(|pair| pair == b"\r\n").map(|offset| start + offset)
}

pub(super) fn is_http_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte)
}

fn owned_header_matches(head: &ParsedHttpHead, name: &str, expected: &str) -> bool {
    head.header(name).is_some_and(|value| value.split(',').any(|part| part.trim().eq_ignore_ascii_case(expected)))
}

fn owned_bearer_matches(head: &ParsedHttpHead, admission: &HttpAdmission) -> bool {
    head.header("authorization").is_some_and(|value| admission.authorizes_bearer_header(value))
}

fn serialize_json_capped<T: serde::Serialize>(value: &T, cap: usize) -> Result<Vec<u8>, HttpTerminalReason> {
    let mut writer = CappedBytes::new(cap);
    match serde_json::to_writer(&mut writer, value) {
        Ok(()) => Ok(writer.bytes),
        Err(_) if writer.exceeded => Err(HttpTerminalReason::Capacity),
        Err(_) => Err(HttpTerminalReason::Malformed),
    }
}

fn build_http_response(status: u16, reason: &str, content_type: Option<&str>, body: Vec<u8>, extra_headers: &[(&str, &str)], upgrade: bool) -> Result<OwnedHttpResponse, HttpTerminalReason> {
    let mut head = CappedBytes::new(HTTP_RESPONSE_BYTES);
    write!(head, "HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\n", body.len()).map_err(|_| HttpTerminalReason::Capacity)?;
    if let Some(content_type) = content_type {
        write!(head, "Content-Type: {content_type}\r\n").map_err(|_| HttpTerminalReason::Capacity)?;
    }
    for (name, value) in extra_headers {
        write!(head, "{name}: {value}\r\n").map_err(|_| HttpTerminalReason::Capacity)?;
    }
    write!(head, "Connection: {}\r\n\r\n", if upgrade { "Upgrade" } else { "close" }).map_err(|_| HttpTerminalReason::Capacity)?;
    if body.len() > HTTP_RESPONSE_BYTES.saturating_sub(head.bytes.len()) {
        return Err(HttpTerminalReason::Capacity);
    }
    head.bytes.extend_from_slice(&body);
    Ok(OwnedHttpResponse { bytes: head.bytes, upgrade })
}

fn owned_json_rpc_error(status: u16, reason: &str, id: JsonRpcId, code: i64, message: String, data: Option<serde_json::Value>) -> Result<OwnedHttpResponse, HttpTerminalReason> {
    let response = JsonRpcResponse::error(id, code, message, data);
    let body = serialize_json_capped(&response, HTTP_RESPONSE_BYTES)?;
    build_http_response(status, reason, Some("application/json"), body, &[], false)
}

pub(crate) fn dispatch_owned_http(state: &HttpState, head: &ParsedHttpHead, request: &[u8]) -> Result<OwnedHttpResponse, HttpTerminalReason> {
    let method = head.method.as_deref().ok_or(HttpTerminalReason::Malformed)?;
    let path = head.path.as_deref().ok_or(HttpTerminalReason::Malformed)?;
    let origin = head.header("origin");
    if !origin_allowed(origin, &state.allowed_origins) {
        return build_http_response(403, "Forbidden", Some("text/plain"), b"origin not allowed".to_vec(), &[], false);
    }
    if path.starts_with("/bridge") {
        return dispatch_owned_bridge_handshake(&state.admission, head);
    }
    if path != "/mcp" || state.server.is_none() {
        return build_http_response(404, "Not Found", Some("text/plain"), b"not found".to_vec(), &[], false);
    }
    if !owned_bearer_matches(head, &state.admission) {
        return build_http_response(401, "Unauthorized", Some("text/plain"), b"missing or invalid bearer token".to_vec(), &[], false);
    }
    match method {
        "POST" => dispatch_owned_post(state, head, request),
        "GET" => dispatch_owned_get(state, head),
        _ => build_http_response(405, "Method Not Allowed", Some("text/plain"), b"method not allowed".to_vec(), &[], false),
    }
}

fn dispatch_owned_post(state: &HttpState, head: &ParsedHttpHead, request: &[u8]) -> Result<OwnedHttpResponse, HttpTerminalReason> {
    let header_end = head.header_end.ok_or(HttpTerminalReason::Malformed)?;
    let request_end = head.request_end().ok_or(HttpTerminalReason::Malformed)?;
    let body = request.get(header_end..request_end).ok_or(HttpTerminalReason::Malformed)?;
    let request: JsonRpcRequest = match serde_json::from_slice(body) {
        Ok(request) => request,
        Err(error) => return owned_json_rpc_error(400, "Bad Request", JsonRpcId::Null, PARSE_ERROR, format!("parse error: {error}"), None),
    };
    if let Some(requested) = extract_meta_protocol_version(request.params.as_ref()) {
        let fallback_id = || request.id.clone().unwrap_or(JsonRpcId::Null);
        match head.header("mcp-protocol-version") {
            None => return owned_json_rpc_error(400, "Bad Request", fallback_id(), HEADER_MISMATCH, "missing required MCP-Protocol-Version header".to_string(), None),
            Some(header_value) if header_value != requested => {
                return owned_json_rpc_error(400, "Bad Request", fallback_id(), HEADER_MISMATCH, format!("MCP-Protocol-Version header `{header_value}` does not match body protocol version `{requested}`"), None);
            }
            Some(_) => {}
        }
        if !SUPPORTED_PROTOCOL_VERSIONS.contains(&requested.as_str()) {
            return owned_json_rpc_error(
                400,
                "Bad Request",
                fallback_id(),
                UNSUPPORTED_PROTOCOL_VERSION,
                format!("unsupported protocol version: {requested}"),
                Some(serde_json::json!({ "supported": SUPPORTED_PROTOCOL_VERSIONS, "requested": requested })),
            );
        }
    }
    let Some(server) = state.server.as_ref() else { return build_http_response(404, "Not Found", Some("text/plain"), b"not found".to_vec(), &[], false) };
    let response = server.lock().expect("mcp server lock poisoned").dispatch(&request);
    match response {
        None => build_http_response(202, "Accepted", None, Vec::new(), &[], false),
        Some(response) => build_http_response(200, "OK", Some("application/json"), serialize_json_capped(&response, HTTP_RESPONSE_BYTES)?, &[], false),
    }
}

fn dispatch_owned_get(state: &HttpState, head: &ParsedHttpHead) -> Result<OwnedHttpResponse, HttpTerminalReason> {
    let floor = head.header("last-event-id").and_then(|value| value.parse::<u64>().ok()).unwrap_or(0);
    let events = state.events.lock().expect("event log lock poisoned");
    let mut body = CappedBytes::new(HTTP_RESPONSE_BYTES);
    for (id, notification) in events.entries.iter().filter(|(id, _)| *id > floor) {
        write!(body, "id: {id}\nevent: message\ndata: ").map_err(|_| HttpTerminalReason::Capacity)?;
        serde_json::to_writer(&mut body, notification).map_err(|_| if body.exceeded { HttpTerminalReason::Capacity } else { HttpTerminalReason::Malformed })?;
        write!(body, "\n\n").map_err(|_| HttpTerminalReason::Capacity)?;
    }
    build_http_response(200, "OK", Some("text/event-stream"), body.bytes, &[("Cache-Control", "no-cache")], false)
}

pub(crate) fn dispatch_owned_bridge_handshake(admission: &HttpAdmission, head: &ParsedHttpHead) -> Result<OwnedHttpResponse, HttpTerminalReason> {
    if head.method.as_deref() != Some("GET") {
        return build_http_response(405, "Method Not Allowed", Some("text/plain"), b"method not allowed".to_vec(), &[], false);
    }
    if head.path.as_deref() != Some("/bridge") || head.header_occurrences("sec-websocket-protocol") != 1 {
        return build_http_response(401, "Unauthorized", Some("text/plain"), b"invalid bridge admission".to_vec(), &[], false);
    }
    let protocols = head.header("sec-websocket-protocol").map(|value| value.split(',').map(str::trim).collect::<Vec<_>>()).unwrap_or_default();
    if protocols.len() != 2 || protocols[0] != "semio.mcp.bridge.v1" || !admission.authorizes_capability(protocols[1]) {
        return build_http_response(401, "Unauthorized", Some("text/plain"), b"invalid bridge admission".to_vec(), &[], false);
    }
    if !owned_header_matches(head, "upgrade", "websocket") || !owned_header_matches(head, "connection", "upgrade") || head.header("sec-websocket-version") != Some("13") {
        return build_http_response(426, "Upgrade Required", Some("text/plain"), b"websocket version 13 required".to_vec(), &[("Sec-WebSocket-Version", "13")], false);
    }
    if head.header_occurrences("sec-websocket-key") != 1 {
        return Err(HttpTerminalReason::Malformed);
    }
    let key = head.header("sec-websocket-key").ok_or(HttpTerminalReason::Malformed)?;
    websocket_key_nonce(key).ok_or(HttpTerminalReason::Malformed)?;
    let accept = websocket_accept(key);
    build_http_response(101, "Switching Protocols", None, Vec::new(), &[("Upgrade", "websocket"), ("Sec-WebSocket-Accept", &accept), ("Sec-WebSocket-Protocol", "semio.mcp.bridge.v1")], true)
}
//#endregion 🔖️HttpTransport
