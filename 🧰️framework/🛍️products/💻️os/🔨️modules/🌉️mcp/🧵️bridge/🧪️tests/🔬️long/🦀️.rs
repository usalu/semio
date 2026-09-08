
use super::server::bridge_router;
use super::*;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

async fn boot(token: &str, allowed_origins: Vec<String>) -> (std::net::SocketAddr, BridgeHandle, tokio::task::JoinHandle<()>) {
    let admission = crate::transport::HttpAdmission::Fixture(Arc::from(token.as_bytes()));
    let (router, handle) = bridge_router(admission, allowed_origins);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server_task = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    (addr, handle, server_task)
}

fn hello(session_id: &str) -> ShellToGateway {
    ShellToGateway::Hello { bridge_version: BRIDGE_VERSION, shell_kind: ShellKind::WgpuNative, shell_session_id: session_id.into(), principal_actor: "agent:local".into(), flags: BridgeFlags::NONE }
}

async fn recv_frame(socket: &mut (impl StreamExt<Item = Result<TungsteniteMessage, tokio_tungstenite::tungstenite::Error>> + Unpin)) -> GatewayToShell {
    let message = socket.next().await.expect("a response frame").expect("a valid ws message");
    let TungsteniteMessage::Binary(bytes) = message else { panic!("expected a binary frame, got {message:?}") };
    GatewayToShell::decode(&bytes).unwrap()
}

#[tokio::test]
async fn bridge_websocket_replies_welcome_to_hello() {
    let (addr, _handle, server_task) = boot("secret", vec![]).await;
    let mut request = format!("ws://{addr}/bridge").into_client_request().unwrap();
    request.headers_mut().insert("sec-websocket-protocol", "semio.mcp.bridge.v1, secret".parse().unwrap());
    let (mut socket, response) = tokio_tungstenite::connect_async(request).await.expect("client connects");
    assert_eq!(response.headers().get("sec-websocket-protocol").and_then(|value| value.to_str().ok()), Some("semio.mcp.bridge.v1"));
    socket.send(TungsteniteMessage::Binary(hello("shell-42").encode().into())).await.unwrap();
    let welcome = recv_frame(&mut socket).await;
    assert!(matches!(welcome, GatewayToShell::Welcome { bridge_version, ref principal, .. } if bridge_version == BRIDGE_VERSION && principal == "agent:local"));
    server_task.abort();
}

#[tokio::test]
async fn wrong_token_is_rejected_before_the_websocket_upgrade() {
    let (addr, _handle, server_task) = boot("correct-token", vec![]).await;
    let mut request = format!("ws://{addr}/bridge").into_client_request().unwrap();
    request.headers_mut().insert("sec-websocket-protocol", "semio.mcp.bridge.v1, wrong-token".parse().unwrap());
    let result = tokio_tungstenite::connect_async(request).await;
    assert!(result.is_err(), "a mismatched token must never complete the websocket handshake");
    server_task.abort();
}

#[tokio::test]
async fn missing_token_is_rejected() {
    let (addr, _handle, server_task) = boot("correct-token", vec![]).await;
    let result = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge")).await;
    assert!(result.is_err());
    server_task.abort();
}

#[tokio::test]
async fn query_credential_is_rejected_even_with_the_valid_protocol_proof() {
    let (addr, _handle, server_task) = boot("secret", vec![]).await;
    let mut request = format!("ws://{addr}/bridge?token=secret").into_client_request().unwrap();
    request.headers_mut().insert("sec-websocket-protocol", "semio.mcp.bridge.v1, secret".parse().unwrap());
    assert!(tokio_tungstenite::connect_async(request).await.is_err());
    server_task.abort();
}

#[tokio::test]
async fn an_evil_origin_is_rejected_before_the_websocket_upgrade() {
    let (addr, _handle, server_task) = boot("secret", vec![]).await;
    let mut request = format!("ws://{addr}/bridge").into_client_request().unwrap();
    request.headers_mut().insert("sec-websocket-protocol", "semio.mcp.bridge.v1, secret".parse().unwrap());
    request.headers_mut().insert("origin", "https://evil.example".parse().unwrap());
    let result = tokio_tungstenite::connect_async(request).await;
    assert!(result.is_err(), "a non-loopback Origin must never complete the websocket handshake");
    server_task.abort();
}

/// 🔁️ The full scenario `📓️sol-P1c-packet.md`'s acceptance list names in one place: `Hello`→
/// `Welcome`, a `ShellState` publish becomes readable via [`BridgeHandle::last_shell_state`], a
/// server-pushed `ShellCommand` reaches the client, and the client's `ShellCommandResult` becomes
/// readable via [`BridgeHandle::last_command_result`].
#[tokio::test]
async fn full_bridge_lifecycle_hello_state_push_and_command_result() {
    let (addr, handle, server_task) = boot("secret", vec![]).await;
    let mut request = format!("ws://{addr}/bridge").into_client_request().unwrap();
    request.headers_mut().insert("sec-websocket-protocol", "semio.mcp.bridge.v1, secret".parse().unwrap());
    let (mut socket, _response) = tokio_tungstenite::connect_async(request).await.expect("client connects");
    socket.send(TungsteniteMessage::Binary(hello("shell-1").encode().into())).await.unwrap();
    let _welcome = recv_frame(&mut socket).await;

    let id = handle.connections().first().copied().expect("exactly one connection registered");
    assert!(handle.last_shell_state(id).is_none());

    let state_frame = ShellToGateway::ShellState { revision: 5, state: vec![9, 9, 9] };
    socket.send(TungsteniteMessage::Binary(state_frame.clone().encode().into())).await.unwrap();
    socket.send(TungsteniteMessage::Binary(ShellToGateway::Ping.encode().into())).await.unwrap();
    assert_eq!(recv_frame(&mut socket).await, GatewayToShell::Pong);
    assert_eq!(handle.last_shell_state(id), Some(state_frame));

    let pushed = GatewayToShell::ShellCommand { seq: 7, command: vec![1, 2, 3] };
    assert!(handle.send_to(id, pushed.clone()), "send_to must reach the live connection");
    assert_eq!(recv_frame(&mut socket).await, pushed);

    let result_frame = ShellToGateway::ShellCommandResult { in_reply_to: 7, ok: true, fault: None };
    socket.send(TungsteniteMessage::Binary(result_frame.encode().into())).await.unwrap();
    socket.send(TungsteniteMessage::Binary(ShellToGateway::Ping.encode().into())).await.unwrap();
    assert_eq!(recv_frame(&mut socket).await, GatewayToShell::Pong);
    assert_eq!(handle.last_command_result(id), Some((7, true, None)));

    assert_eq!(handle.broadcast(GatewayToShell::Pong), Ok(1), "broadcast must reach exactly the one live connection");
    assert_eq!(recv_frame(&mut socket).await, GatewayToShell::Pong);

    drop(socket);
    server_task.abort();
}

#[test]
fn send_to_an_unknown_connection_returns_false() {
    let handle = BridgeHandle::new();
    assert!(!handle.send_to(ShellConnectionId(999), GatewayToShell::Pong));
}
