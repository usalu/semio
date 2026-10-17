//! 🌐️ Servers and sockets: spawned servers, raw HTTP, server frames, close codes, socket hellos and requests.

use super::*;

pub async fn spawn_server(state: HubState) -> SocketAddr {
    spawn_server_with_posture(state, CrossOriginPolicyV1::LoopbackDevelopment, ForwardedTlsTrustV1::Untrusted).await
}

pub async fn spawn_server_with_posture(state: HubState, cross_origin: CrossOriginPolicyV1, forwarded_tls: ForwardedTlsTrustV1) -> SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = router(state, cross_origin, forwarded_tls).into_make_service_with_connect_info::<SocketAddr>();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    addr
}

pub async fn spawn_restartable_server(state: HubState) -> (SocketAddr, tokio::sync::oneshot::Sender<()>, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = router(state, CrossOriginPolicyV1::LoopbackDevelopment, ForwardedTlsTrustV1::Untrusted).into_make_service_with_connect_info::<SocketAddr>();
    let (shutdown, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    (addr, shutdown, task)
}

#[derive(Debug)]
pub struct RawHttpResponse {
    pub status: u16,
    pub headers: String,
    pub body: Vec<u8>,
}

pub async fn raw_http_request_transport(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> Option<RawHttpResponse> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = tokio::net::TcpStream::connect(addr).await.expect("HTTP connect");
    let mut request = format!("{method} {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\nContent-Length: {}\r\n", body.len());
    for (name, value) in headers {
        request.push_str(name);
        request.push_str(": ");
        request.push_str(value);
        request.push_str("\r\n");
    }
    request.push_str("\r\n");
    stream.write_all(request.as_bytes()).await.expect("HTTP write");
    stream.write_all(body).await.expect("HTTP body write");
    stream.flush().await.expect("HTTP request flush");
    let mut response = Vec::new();
    let read = tokio::time::timeout(RAW_HTTP_READ_HANG_GUARD, async {
        let mut chunk = [0_u8; 4096];
        loop {
            let read = stream.read(&mut chunk).await?;
            if read == 0 {
                return std::io::Result::Ok(());
            }
            response.extend_from_slice(&chunk[..read]);
            if let Some(boundary) = response.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                let head = std::str::from_utf8(&response[..boundary]).unwrap_or_default();
                let content_length = head.lines().find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length").then(|| value.trim().parse::<usize>().ok()).flatten()
                });
                if content_length.is_some_and(|length| response.len() >= boundary + 4 + length) {
                    return std::io::Result::Ok(());
                }
            }
        }
    })
    .await
    .expect("HTTP deadline");
    if let Err(error) = read {
        assert!(matches!(error.kind(), std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::BrokenPipe), "HTTP read: {error}");
    }
    if response.is_empty() {
        return None;
    }
    let boundary = response.windows(4).position(|bytes| bytes == b"\r\n\r\n").expect("HTTP header boundary");
    let head = std::str::from_utf8(&response[..boundary]).expect("HTTP headers").to_string();
    let status = head.split_whitespace().nth(1).expect("HTTP status").parse().expect("numeric HTTP status");
    Some(RawHttpResponse { status, headers: head, body: response[boundary + 4..].to_vec() })
}

pub async fn raw_http_request(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> RawHttpResponse {
    raw_http_request_transport(addr, method, path, headers, body).await.expect("HTTP response")
}

pub async fn raw_http_get(addr: SocketAddr, path: &str, headers: &[(&str, &str)]) -> RawHttpResponse {
    raw_http_request(addr, "GET", path, headers, &[]).await
}

/// 🚧️ One hostile request on its own connection. Unlike [`raw_http_request`] it tolerates the hub
/// closing a request it refused early (an oversized body), and answers `Err` only when the hub never
/// answered — a dropped connection is a finding, not a transport detail.
pub async fn hostile_http_request(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> Result<RawHttpResponse, String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = tokio::net::TcpStream::connect(addr).await.map_err(|error| format!("connect: {error}"))?;
    let mut request = format!("{method} {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\nContent-Length: {}\r\n", body.len());
    for (name, value) in headers {
        request.push_str(&format!("{name}: {value}\r\n"));
    }
    request.push_str("\r\n");
    let written = async {
        stream.write_all(request.as_bytes()).await?;
        stream.write_all(body).await?;
        stream.flush().await
    }
    .await;
    let mut response = Vec::new();
    let read = tokio::time::timeout(RAW_HTTP_READ_HANG_GUARD, stream.read_to_end(&mut response)).await.map_err(|_| format!("{method} {path} never answered"))?;
    if response.is_empty() {
        return Err(format!("{method} {path} dropped the connection (write {written:?}, read {read:?})"));
    }
    let boundary = response.windows(4).position(|bytes| bytes == b"\r\n\r\n").ok_or_else(|| format!("{method} {path} answered no HTTP head"))?;
    let head = String::from_utf8_lossy(&response[..boundary]).to_string();
    let status = head.split_whitespace().nth(1).and_then(|status| status.parse().ok()).ok_or_else(|| format!("{method} {path} answered no status"))?;
    let raw = response[boundary + 4..].to_vec();
    let body = if head.to_ascii_lowercase().contains("transfer-encoding: chunked") { dechunk_http_body(&raw) } else { raw };
    Ok(RawHttpResponse { status, headers: head, body })
}

fn dechunk_http_body(raw: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    let mut rest = raw;
    while let Some(line_end) = rest.windows(2).position(|bytes| bytes == b"\r\n") {
        let size = usize::from_str_radix(std::str::from_utf8(&rest[..line_end]).unwrap_or("0").trim(), 16).unwrap_or(0);
        if size == 0 || rest.len() < line_end + 2 + size {
            break;
        }
        body.extend_from_slice(&rest[line_end + 2..line_end + 2 + size]);
        rest = &rest[(line_end + 4 + size).min(rest.len())..];
    }
    body
}

/// 🧪️ A loopback `ConnectInfo` for handlers called directly in tests. Network
/// proximity confers no authorization; every protected test also supplies a verified session.
pub fn loopback_peer() -> axum::extract::ConnectInfo<SocketAddr> {
    axum::extract::ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 0)))
}

pub async fn next_server_frame<S>(ws: &mut S) -> ServerFrame
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    next_server_frame_at(ws, "unnamed").await
}

/// 🏷️ The same wait, with the caller's own phase in every refusal. One helper serves 49 call sites
/// across this suite, so an unnamed `no server frame before 5s deadline` names neither the law's
/// line nor which of its frames is missing — HT13's whole first hour went into recovering that by
/// arithmetic. A law that waits on more than one frame passes a phase here so its capture says which.
pub async fn next_server_frame_at<S>(ws: &mut S, phase: &str) -> ServerFrame
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    next_server_frame_within(ws, phase, std::time::Duration::from_secs(5)).await
}

/// ⏳️ [`next_server_frame_at`] with the law's own bound, for a frame a loaded host may take longer to answer.
pub async fn next_server_frame_within<S>(ws: &mut S, phase: &str, bound: std::time::Duration) -> ServerFrame
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + bound;
    loop {
        match tokio::time::timeout_at(deadline, ws.next()).await {
            Ok(Some(Ok(WsMessage::Binary(bytes)))) => return protocol::decode_server_frame(&bytes).await.unwrap_or_else(|error| panic!("server frame at {phase}: {error:?}")).1,
            Ok(Some(Ok(WsMessage::Close(frame)))) => panic!("the server closed before its next frame at {phase}: {frame:?} plan refusal {:?}", last_document_plan_refusal()),
            Ok(Some(Ok(_))) => continue,
            Ok(Some(other)) => panic!("expected binary frame at {phase}, got {other:?}"),
            Ok(None) => panic!("stream ended before server frame at {phase}"),
            Err(_) => panic!("no server frame within {bound:?} at {phase}"),
        }
    }
}

pub async fn next_directory_message<S>(ws: &mut S) -> DirectoryStreamMessage
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match tokio::time::timeout_at(deadline, ws.next()).await {
            Ok(Some(Ok(WsMessage::Text(text)))) => return semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("directory message"),
            Ok(Some(Ok(WsMessage::Close(frame)))) => panic!("the server closed before its next directory message: {frame:?}"),
            Ok(Some(Ok(_))) => continue,
            Ok(Some(other)) => panic!("expected directory message, got {other:?}"),
            Ok(None) => panic!("stream ended before directory message"),
            Err(_) => panic!("no directory message before 5s deadline"),
        }
    }
}

pub async fn client_binary(frame: &ClientFrame, lane: Lane) -> WsMessage {
    WsMessage::Binary(protocol::encode_client_frame(frame, lane).await.into())
}

pub async fn next_close_code<S>(ws: &mut S, allow_text: bool) -> u16
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match tokio::time::timeout_at(deadline, ws.next()).await {
            Ok(Some(Ok(WsMessage::Close(Some(frame))))) => return frame.code.into(),
            Ok(Some(Ok(WsMessage::Text(text)))) if !allow_text => panic!("unauthorized rebootstrap control leaked: {text}"),
            Ok(Some(Ok(_))) => continue,
            Ok(Some(other)) => panic!("expected close frame, got {other:?}"),
            Ok(None) => panic!("stream ended before close frame"),
            Err(_) => panic!("no close frame before 5s deadline"),
        }
    }
}

pub async fn next_close_without_authority<S>(ws: &mut S) -> u16
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    match tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, ws.next()).await {
        Ok(Some(Ok(WsMessage::Close(Some(frame))))) => frame.code.into(),
        Ok(Some(Ok(WsMessage::Binary(bytes)))) => panic!("authority-bearing binary frame crossed revocation: {:?}", protocol::decode_server_frame(&bytes).await.map(|decoded| decoded.1)),
        Ok(Some(Ok(WsMessage::Text(_)))) => panic!("authority-bearing directory frame crossed revocation"),
        Ok(Some(other)) => panic!("expected close after revocation, got {other:?}"),
        Ok(None) => panic!("stream ended before revocation close"),
        Err(_) => panic!("the revocation close never arrived: the server closed no socket for this revoke"),
    }
}

pub fn socket_hello() -> ClientFrame {
    ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: "test.v1".to_string(), pack_schema_hash: [0x11; 32], resume_token: None, frontier: None }
}

pub fn bearer_headers(capability: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {capability}").parse().expect("bearer header"));
    headers
}

pub fn socket_request(url: &str, grant: &str) -> tokio_tungstenite::tungstenite::http::Request<()> {
    let mut request = url.into_client_request().expect("socket request");
    request.headers_mut().insert(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {grant}").parse().expect("socket protocols"));
    request
}

pub fn document_socket_request(url: &str, session: &str) -> tokio_tungstenite::tungstenite::http::Request<()> {
    let mut request = url.into_client_request().expect("document socket request");
    request.headers_mut().insert(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SESSION_PROTOCOL_V1}, {session}").parse().expect("session protocol"));
    request
}
