//! 🛡️ Origin and bearer admission plus the Axum differential oracle handlers.

use semio_framework_os_mcp_wire::protocol::{
    JsonRpcId,
    JsonRpcRequest,
    JsonRpcResponse,
    PARSE_ERROR,
    SUPPORTED_PROTOCOL_VERSIONS,
    UNSUPPORTED_PROTOCOL_VERSION,
    extract_meta_protocol_version,
};
#[cfg(test)]
use axum::body::Body;
#[cfg(test)]
use axum::extract::State;
#[cfg(test)]
use axum::http::{HeaderMap, StatusCode};
#[cfg(test)]
use axum::response::Response;
#[cfg(test)]
use super::http_surface::{HEADER_MISMATCH, HttpAdmission, HttpState};

//#region 🔖️HttpTransport
//#region 🔖️OriginAndBearer

pub(crate) fn is_loopback_or_null(origin: &str) -> bool {
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

/// 🔓️ Also used by `🧵️bridge`'s protected subprotocol comparison (`pub(crate)`) — one
/// constant-time comparison helper, not duplicated.
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

#[cfg(test)]
fn bearer_matches(headers: &HeaderMap, admission: &HttpAdmission) -> bool {
    let Some(value) = headers.get(axum::http::header::AUTHORIZATION).and_then(|value| value.to_str().ok()) else { return false };
    admission.authorizes_bearer_header(value)
}

#[cfg(test)]
fn reject_by_origin(headers: &HeaderMap, state: &HttpState) -> Option<Response> {
    let origin = headers.get(axum::http::header::ORIGIN).and_then(|value| value.to_str().ok());
    if origin_allowed(origin, &state.allowed_origins) {
        None
    } else {
        Some((StatusCode::FORBIDDEN, "origin not allowed").into_response())
    }
}

#[cfg(test)]
fn reject_by_bearer(headers: &HeaderMap, state: &HttpState) -> Option<Response> {
    if bearer_matches(headers, &state.admission) {
        None
    } else {
        Some((StatusCode::UNAUTHORIZED, "missing or invalid bearer token").into_response())
    }
}

//#endregion 🔖️OriginAndBearer

#[cfg(test)]
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
#[cfg(test)]
pub(crate) async fn handle_post(State(state): State<HttpState>, headers: HeaderMap, body: axum::body::Bytes) -> Response {
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

    let Some(owner) = state.server.as_ref() else { return StatusCode::NOT_FOUND.into_response() };
    let mut server = owner.lock().expect("mcp server lock poisoned");
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
#[cfg(test)]
pub(crate) async fn handle_get(State(state): State<HttpState>, headers: HeaderMap) -> Response {
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
    Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, "text/event-stream")
        .header(axum::http::header::CACHE_CONTROL, "no-cache")
        .body(Body::from(body))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}
//#endregion 🔖️HttpTransport
