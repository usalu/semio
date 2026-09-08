
use super::{BRIDGE_VERSION, BridgeHandle, GatewayToShell, ShellToGateway};
use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use futures::{SinkExt, StreamExt};
use semio_framework_os_kernel::{FromValue, ToValue};
use std::sync::Arc;

#[derive(Clone)]
struct BridgeServerState {
    admission: crate::transport::HttpAdmission,
    allowed_origins: Arc<Vec<String>>,
    handle: BridgeHandle,
}

/// 🏗️ Builds the `/bridge` route + its [`BridgeHandle`] — never bound to a socket by itself; a
/// caller `.merge()`s the returned [`Router`] into the app it actually serves.
pub(crate) fn bridge_router(admission: crate::transport::HttpAdmission, allowed_origins: Vec<String>) -> (Router, BridgeHandle) {
    let handle = BridgeHandle::new();
    let state = BridgeServerState { admission, allowed_origins: Arc::new(allowed_origins), handle: handle.clone() };
    let router = Router::new().route("/bridge", get(upgrade)).with_state(state);
    (router, handle)
}

async fn upgrade(ws: WebSocketUpgrade, uri: Uri, headers: HeaderMap, State(state): State<BridgeServerState>) -> Response {
    if uri.path() != "/bridge" || uri.query().is_some() {
        return (StatusCode::UNAUTHORIZED, "invalid bridge admission").into_response();
    }
    let origin = headers.get(axum::http::header::ORIGIN).and_then(|value| value.to_str().ok());
    if !crate::transport::origin_allowed(origin, &state.allowed_origins) {
        return (StatusCode::FORBIDDEN, "origin not allowed").into_response();
    }
    let protocols = headers.get(axum::http::header::SEC_WEBSOCKET_PROTOCOL).and_then(|value| value.to_str().ok()).map(|value| value.split(',').map(str::trim).collect::<Vec<_>>()).unwrap_or_default();
    if protocols.len() != 2 || protocols[0] != "semio.mcp.bridge.v1" || !state.admission.authorizes_capability(protocols[1]) {
        return (StatusCode::UNAUTHORIZED, "invalid bridge admission").into_response();
    }
    let handle = state.handle.clone();
    ws.protocols(["semio.mcp.bridge.v1"]).on_upgrade(move |socket| handle_socket(socket, handle))
}

/// 🔁️ One connection's full lifecycle: the OPENING frame must decode as `Hello` (anything else,
/// or a closed/errored socket, ends the connection immediately without ever registering it) →
/// reply `Welcome` → loop reading client frames (`Ping`→`Pong` inline, `Bye`/close ends the loop,
/// `ShellState`/`ShellStatePatch`/`Instances`/`ShellCommandResult`/`Approval` recorded via
/// [`BridgeHandle::record`]; a frame that fails to decode is skipped rather than killing the
/// connection) while concurrently draining this connection's OWN outbox
/// ([`BridgeHandle::send_to`]/[`BridgeHandle::broadcast`] push onto it) to the socket.
async fn handle_socket(socket: WebSocket, handle: BridgeHandle) {
    let (mut sender, mut receiver) = socket.split();
    let Some(Ok(Message::Binary(bytes))) = receiver.next().await else { return };
    let Ok(ShellToGateway::Hello { .. }) = ShellToGateway::decode(&bytes) else { return };

    let (id, mut outbox) = handle.register();
    let welcome = GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: id.to_string(), principal: "agent:local".to_string() };
    if sender.send(Message::Binary(welcome.encode().into())).await.is_err() {
        handle.unregister(id);
        return;
    }

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Binary(bytes))) => match ShellToGateway::decode(&bytes) {
                        Ok(ShellToGateway::Ping) => {
                            if sender.send(Message::Binary(GatewayToShell::Pong.encode().into())).await.is_err() {
                                break;
                            }
                        }
                        Ok(ShellToGateway::Bye) => break,
                        Ok(frame) => handle.record(id, frame),
                        Err(_malformed) => {}
                    },
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            pushed = outbox.recv() => {
                match pushed {
                    Some(frame) => {
                        if sender.send(Message::Binary(frame.encode().into())).await.is_err() {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }
    }
    handle.unregister(id);
}
