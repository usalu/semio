//! 🛂️ CORS, client addressing, throttling and the credential readers at the edge.

use super::*;

//#region 🔖️Cors
/// 🌐️ Reflect the caller's own `Origin` back rather than answering a bare `*`, and short-circuit
/// every `OPTIONS` preflight with 204 before it reaches route dispatch.
///
/// Hand-rolled instead of pulled from a middleware library on purpose: the whole behaviour is five
/// headers — a dependency to produce them would be a dependency to audit them. The grant never
/// allows credentials: a caller identifies itself with a bearer it sets explicitly, never with
/// anything a browser attaches by itself, so no page gains another page's ambient authority here.
pub async fn cors_middleware(request: Request, next: axum::middleware::Next) -> Response {
    let origin = request.headers().get(header::ORIGIN).cloned();
    if request.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        apply_cors_headers(response.headers_mut(), origin.as_ref());
        return response;
    }
    let mut response = next.run(request).await;
    apply_cors_headers(response.headers_mut(), origin.as_ref());
    response
}

/// 🪞️ Write the CORS grant onto a response: the caller's own origin (never `*`, never with
/// credentials), and the verbs and headers this control plane actually uses.
pub fn apply_cors_headers(headers: &mut HeaderMap, origin: Option<&HeaderValue>) {
    if let Some(origin) = origin {
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
        headers.insert(header::VARY, HeaderValue::from_static("origin"));
    }
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST, PUT, HEAD, DELETE, OPTIONS"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("authorization, content-type, x-semio-capability"));
}
//#endregion 🔖️Cors

//#region 🔖️Edge
/// 🧭️ Where the address a client is throttled as comes from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ClientAddressing {
    /// 🔌️ The peer of the connection: right wherever clients connect to the instance themselves.
    #[default]
    Peer,
    /// 🛡️ The address a TLS-terminating proxy — the only peer this instance then has — wrote last
    /// into [`FORWARDED_FOR`]; the peer's own address when the header is absent or names none. Right
    /// only where nothing but that proxy can reach the instance: any other peer could name an
    /// address of its choice.
    Forwarded,
}

/// 🏷️ The header a terminating proxy names the client's address in.
pub const FORWARDED_FOR: &str = "x-forwarded-for";

/// 🛣️ The path of the one `POST` that reads.
pub const QUERIES_PATH: &str = "/queries";

/// 🏷️ The address `headers` and `peer` identify a client as, under `addressing`.
pub fn client_key(addressing: ClientAddressing, headers: &HeaderMap, peer: Option<SocketAddr>) -> ClientKey {
    let forwarded = match addressing {
        ClientAddressing::Peer => None,
        ClientAddressing::Forwarded => headers.get_all(FORWARDED_FOR).iter().next_back().and_then(|value| value.to_str().ok()).and_then(forwarded_address),
    };
    ClientKey::of(forwarded.or(peer.map(|peer| peer.ip())).unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)))
}

/// 🗂️ The bucket a request spends from: a websocket upgrade, a read (`GET`, `HEAD`, `OPTIONS` and
/// the query route) or a write (everything else).
pub fn request_class(method: &Method, path: &str, headers: &HeaderMap) -> RequestClass {
    if headers.get(header::UPGRADE).and_then(|value| value.to_str().ok()).is_some_and(|value| value.eq_ignore_ascii_case("websocket")) {
        return RequestClass::Upgrade;
    }
    if method == Method::GET || method == Method::HEAD || method == Method::OPTIONS || path == QUERIES_PATH {
        return RequestClass::Read;
    }
    RequestClass::Write
}

/// 🚧️ The edge of one instance as its throttle middleware needs it: the bookkeeping and where
/// client addresses come from. A bundle of handles, cheap to clone.
#[derive(Clone)]
pub struct Edge {
    pub throttle: Arc<Throttle>,
    pub addressing: ClientAddressing,
}

/// ⏱️ How long a refused request is given to deliver the rest of its body, so that the connection
/// it came on can carry the next request.
pub const REFUSED_BODY_PATIENCE: Duration = Duration::from_millis(500);

/// 🚦️ Spend the caller's token for this request, take its body, and count it in flight while it
/// is served; a refused request is answered `429` (the address's allowance) or `503` (an instance
/// cap) with `Retry-After` before anything behind this layer runs.
///
/// **The body is taken here**, whole and at most [`Limits::body_bytes`] of it, on an instance
/// that states that limit. Before the request counts in flight: a body that trickles in holds one
/// connection for at most [`Limits::body_patience`] (`408`) and never one of the places the
/// instance serves requests in. And of a refused request too, for at most [`REFUSED_BODY_PATIENCE`]: a
/// response sent while the body is still on its way makes the transport close the connection, so a
/// flood of refused requests would open connections as fast as it is refused — and behind a proxy
/// on the same host those are the proxy's connections to this instance, whose ports then run out
/// for every client. A body past the limit is not taken (`413`), and that connection closes.
///
/// It is a layer whoever composes the final router applies ([`Server::throttled`]), outside every
/// layer that does work per request, so a refused request costs a hash lookup and reading what it
/// sent.
pub async fn throttle_middleware(State(edge): State<Edge>, request: Request, next: axum::middleware::Next) -> Response {
    let peer = request.extensions().get::<ConnectInfo<SocketAddr>>().map(|info| info.0);
    let client = client_key(edge.addressing, request.headers(), peer);
    let class = request_class(request.method(), request.uri().path(), request.headers());
    let limit = edge.throttle.limits().body_bytes.filter(|_| class != RequestClass::Upgrade);
    if let Err(refusal) = edge.throttle.admit(client, class, std::time::Instant::now()) {
        if let Some(limit) = limit {
            let _ = body_taken(request, limit, REFUSED_BODY_PATIENCE).await;
        }
        return ServerError::from(refusal).into_response();
    }
    let request = match limit {
        Some(limit) => match body_taken(request, limit, edge.throttle.limits().body_patience).await {
            Ok(request) => request,
            Err(error) => return error.into_response(),
        },
        None => request,
    };
    match edge.throttle.enter() {
        Ok(_serving) => next.run(request).await,
        Err(refusal) => ServerError::from(refusal).into_response(),
    }
}

/// 📥️ `request` with its whole body in hand: at most `limit` bytes, delivered within `patience`.
/// An oversized body discards transport chunks until crossing twice the limit, for at most 500 ms,
/// allowing a modest transport tail to finish without storing it. A stalled accepted body is [`ServerError::Stalled`].
pub async fn body_taken(request: Request, limit: usize, patience: Duration) -> Result<Request, ServerError> {
    let declared = request.headers().get(header::CONTENT_LENGTH).and_then(|value| value.to_str().ok()).and_then(|value| value.parse::<usize>().ok());
    let discard_limit = limit.saturating_mul(2);
    if declared.is_some_and(|declared| declared > discard_limit) {
        return Err(ServerError::PayloadTooLarge);
    }
    let (parts, body) = request.into_parts();
    let mut oversized = declared.is_some_and(|declared| declared > limit);
    let taken = async {
        let mut chunks = body.into_data_stream();
        let mut bytes = Vec::with_capacity(if oversized { 0 } else { declared.unwrap_or_default() });
        let mut received = 0usize;
        let mut discard_deadline = oversized.then(|| tokio::time::Instant::now() + REFUSED_BODY_PATIENCE);
        loop {
            let next = match discard_deadline {
                Some(deadline) => tokio::time::timeout_at(deadline, chunks.next()).await.map_err(|_| ServerError::PayloadTooLarge)?,
                None => chunks.next().await,
            };
            let Some(chunk) = next else { break };
            let chunk = chunk.map_err(|error| ServerError::BadRequest(error.to_string()))?;
            received = received.saturating_add(chunk.len());
            if received > limit {
                oversized = true;
                discard_deadline.get_or_insert_with(|| tokio::time::Instant::now() + REFUSED_BODY_PATIENCE);
            }
            if received > discard_limit {
                return Err(ServerError::PayloadTooLarge);
            }
            if !oversized { bytes.extend_from_slice(&chunk); }
        }
        if oversized { Err(ServerError::PayloadTooLarge) } else { Ok(bytes) }
    };
    match tokio::time::timeout(patience, taken).await {
        Ok(bytes) => Ok(Request::from_parts(parts, axum::body::Body::from(bytes?))),
        Err(_) => Err(if oversized { ServerError::PayloadTooLarge } else { ServerError::Stalled }),
    }
}

/// 📏️ Bound what the protocol layer of a socket takes of one inbound message: a frame or message
/// past `inbound` bytes fails the socket while it is being read, so nothing larger is ever
/// buffered, and the read buffer itself is no larger.
pub fn bounded_socket(upgrade: WebSocketUpgrade, inbound: usize) -> WebSocketUpgrade {
    upgrade.max_message_size(inbound).max_frame_size(inbound).read_buffer_size(inbound)
}

/// 📏️ The most the durable-lane socket takes of one inbound message: its client only ever closes.
pub const EVENT_STREAM_INBOUND_BYTES: usize = 1024;

/// 🚀️ Make one accepted connection send each write at once instead of holding a small one back
/// for the peer's acknowledgement (Nagle's algorithm): the answers and presence frames of this
/// gateway are small and complete, and a held one is latency its reader sees. Meant for
/// `listener.tap_io(undelayed)`.
pub fn undelayed(connection: &mut tokio::net::TcpStream) {
    if let Err(error) = connection.set_nodelay(true) {
        report("a connection keeps delaying small writes", &error);
    }
}
//#endregion 🔖️Edge

//#region 🔖️Credential
/// 🛂️ The header a caller presents a [`CapabilityProof`](crate::contract::CapabilityProof) in.
pub const CAPABILITY_HEADER: &str = "x-semio-capability";

/// 🛂️ WebSocket subprotocol that carries a session bearer after this marker.
pub const SESSION_PROTOCOL_V1: &str = "semio.session.v1";

/// 🎟️ The bearer token of an `Authorization: Bearer …` header, if there is one.
pub fn bearer(headers: &HeaderMap) -> Option<String> {
    headers.get(header::AUTHORIZATION).and_then(|value| value.to_str().ok()).and_then(|value| value.strip_prefix("Bearer ")).map(|value| value.to_string())
}

/// 🪪️ Normalize what a caller presented into a transport-free [`Credential`]. `loopback` comes from
/// the peer address and never from a header — it is a fact only the transport can establish, and a
/// 🎟️ Session bearer offered as `Sec-WebSocket-Protocol: semio.session.v1, <token>` when the
/// browser cannot set an `Authorization` header on the upgrade.
pub fn session_protocol_bearer(headers: &HeaderMap) -> Option<String> {
    if headers.contains_key(header::AUTHORIZATION) {
        return None;
    }
    let values = headers.get_all(header::SEC_WEBSOCKET_PROTOCOL);
    if values.iter().count() != 1 {
        return None;
    }
    let offered = values.iter().next().and_then(|value| value.to_str().ok())?;
    let (protocol, token) = offered.split_once(", ")?;
    if protocol != SESSION_PROTOCOL_V1 || token.is_empty() || token.contains(',') {
        return None;
    }
    Some(token.to_string())
}

/// 🔑️ header claiming it would be a header granting itself the administration plane.
pub fn credential(headers: &HeaderMap, peer: Option<SocketAddr>) -> Credential {
    Credential { bearer: bearer(headers).or_else(|| session_protocol_bearer(headers)), capability: headers.get(CAPABILITY_HEADER).and_then(|value| value.to_str().ok()).map(|value| crate::contract::CapabilityProof(value.to_string())), loopback: peer.is_some_and(|peer| peer.ip().is_loopback()) }
}
//#endregion 🔖️Credential
