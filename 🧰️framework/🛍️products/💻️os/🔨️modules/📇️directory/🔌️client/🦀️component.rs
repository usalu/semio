//! 🔌️ Directory hub client (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS,
//! contract §C2/§C6) — the Rust twin of lane 1-C's TypeScript `DirectoryClient`, talking the SAME
//! frozen hub HTTP/WS surface: `POST /directory/commands`, `GET /directory/spaces[/{id}]`,
//! `GET /directory/events`, `GET /directory/ws`, `GET /auth/sessions/me`, `POST /auth/sessions`.
//! No concrete HTTP/WS client type ever appears in a public signature here (CLAUDE.md: external
//! libraries sit behind our own interface) — `DirectoryTransport`/`DirectoryWsConnection` are the
//! seam, mirroring `🎒️pack/🌐️http`'s `RangeTransport` and this crate's own `🏪️store/🔄️sync`
//! (native `tokio-tungstenite`, browser `web_sys`) pattern.
//! `🪪️identity/🦀️component.rs` (sibling module) layers the mint-or-restore session helper on top.
//!
//! 🌀️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet terra-directory-and-run:
//! every request-issuing method now takes an `OperationContext` (`semio-framework-async` —
//! cancellation/deadline/trace/capability), checked up front so an already-cancelled caller never
//! reaches the transport. The native transport (`🔖️Native` below) is rebuilt on
//! `semio-framework-os-services`'s `HttpPool`/`ComputePool` — the per-call blocking-`ureq`-on-a-
//! dedicated-thread pattern and the ad hoc private tokio runtime this crate's own native `open_ws`
//! used to need are RETIRED, not wrapped: tokio stays confined to `semio-framework-os-services`
//! (this crate names it nowhere), and every native HTTP call is now admitted onto that crate's
//! bounded `ComputePool` semaphore instead of an unbounded `std::thread::spawn` per call.

use super::schema::{DirectoryCommand, DirectoryEvent, DirectoryStreamMessage, DocumentView, InviteView, MemberView, SpaceView};
use semio_framework_async::OperationContext;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, PoisonError, RwLock};

//#region 🔖️Transport
/// 📨️ One HTTP verb `DirectoryClient` issues against the hub REST surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Delete,
}

/// 📬️ A transport-agnostic HTTP response: status code plus raw body bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

/// ⚠️ A transport-layer failure (connection refused, DNS, closed socket, …) — distinct from an
/// HTTP error status (that decodes to `DirectoryClientError::Http`/`Unauthorized`) or a JSON
/// decode failure (`DirectoryClientError::Decode`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransportError {
    Io(String),
    /// 🛑️ The `OperationContext` handed to this call was already cancelled, or was cancelled
    /// before the call finished — a transport checks this itself (rather than relying solely on
    /// the caller's own pre-check) so an in-flight call started just before cancellation still
    /// surfaces it instead of returning a stale success.
    Cancelled,
    /// ⏰️ `ctx.deadline_ms` elapsed before this call could complete.
    DeadlineExceeded,
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(detail) => write!(formatter, "transport io: {detail}"),
            Self::Cancelled => formatter.write_str("cancelled"),
            Self::DeadlineExceeded => formatter.write_str("deadline exceeded"),
        }
    }
}

impl std::error::Error for TransportError {}

/// 🧵️ Native transports cross worker turns; browser transports remain local to the wasm event loop.
#[cfg(not(target_arch = "wasm32"))]
pub trait DirectoryTransportPlatform: Send + Sync {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Send + Sync> DirectoryTransportPlatform for T {}

#[cfg(target_arch = "wasm32")]
pub trait DirectoryTransportPlatform {}

#[cfg(target_arch = "wasm32")]
impl<T> DirectoryTransportPlatform for T {}

/// 🧵️ Native connections move between finite worker turns; browser sockets remain event-loop local.
#[cfg(not(target_arch = "wasm32"))]
pub trait DirectoryConnectionPlatform: Send {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> DirectoryConnectionPlatform for T {}

#[cfg(target_arch = "wasm32")]
pub trait DirectoryConnectionPlatform {}

#[cfg(target_arch = "wasm32")]
impl<T> DirectoryConnectionPlatform for T {}

/// 🔌️ The injection seam: no concrete HTTP client type may appear in any public signature
/// outside an implementor of this trait. Native `ureq`, browser `web_sys::fetch`, or a test
/// double all implement this identically. Native transports are `Send + Sync` and native HTTP
/// futures are `Send`; wasm uses a platform-local marker because `JsValue` handles cannot move.
/// `ctx` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME) carries cancellation/deadline/
/// trace/capability through to whichever concrete transport is wired — a native call routes it
/// into `semio-framework-os-services`'s `HttpPool`/`ComputePool` (see `🔖️Native` below), so the
/// SAME `OperationContext` that gates `SpaceRunner::compute_node`'s `exchange` calls
/// (`🏃️run/🦀️component.rs`) also gates a directory request, rather than each path inventing its
/// own bolted-on cancellation.
// 🔀️ dedyn-fw-os-misc, O1/R11(b): `DirectoryTransport` is already generic (never `dyn`, see
// `DirectoryClient<T: DirectoryTransport>` below) — `open_ws` is a trait method that RETURNS a
// runtime-chosen `DirectoryWsConnection` implementation, so the associated type `Ws` pushes that
// choice to the implementor exactly per the `ResourceResolver` precedent, replacing
// `Box<dyn DirectoryWsConnection>`.
pub trait DirectoryTransport: DirectoryTransportPlatform {
    type Ws: DirectoryWsConnection;
    async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError>;
    fn open_ws(&self, ctx: &OperationContext, url: &str, timeout_ms: u64) -> Result<Self::Ws, TransportError>;
}

/// 🔌️ One open `/directory/ws` connection. Sequential by construction (`DirectoryStream` never
/// calls `send_text`/`try_recv_text` concurrently), so a single stream/sink object suffices — no
/// split halves needed the way `🏪️store/🔄️sync`'s bidirectional relay requires.
pub trait DirectoryWsConnection: DirectoryConnectionPlatform {
    fn send_text(&mut self, text: String) -> Result<(), TransportError>;
    /// 📭️ Nonblocking receive: `Pending` yields the worker immediately; `Closed` reconnects.
    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError>;
    fn close(&mut self);
}

/// 📬️ One nonblocking WebSocket receive observation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectoryWsPoll {
    Text(String),
    Pending,
    Closed,
}
//#endregion 🔖️Transport

//#region 🔖️Errors
#[derive(Debug)]
pub enum DirectoryClientError {
    Transport(TransportError),
    Decode(serde_json::Error),
    Unauthorized,
    Http {
        status: u16,
        body: String,
    },
    /// 🛑️ `ctx.cancel` was already cancelled BEFORE this call ever reached the transport — checked
    /// in `DirectoryClient::request_json`/`DirectoryStream::recv` up front, so a cancelled caller
    /// never even builds a request. `TransportError::Cancelled` (via the `Transport` variant above)
    /// is the other half: cancelled WHILE the transport call was in flight.
    Cancelled,
}

impl std::fmt::Display for DirectoryClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(error) => std::fmt::Display::fmt(error, formatter),
            Self::Decode(error) => write!(formatter, "decode: {error}"),
            Self::Unauthorized => formatter.write_str("unauthorized"),
            Self::Http { status, body } => write!(formatter, "http {status}: {body}"),
            Self::Cancelled => formatter.write_str("cancelled"),
        }
    }
}

impl std::error::Error for DirectoryClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            Self::Decode(error) => Some(error),
            _ => None,
        }
    }
}

impl From<TransportError> for DirectoryClientError {
    fn from(error: TransportError) -> Self {
        Self::Transport(error)
    }
}

impl From<serde_json::Error> for DirectoryClientError {
    fn from(error: serde_json::Error) -> Self {
        Self::Decode(error)
    }
}
//#endregion 🔖️Errors

//#region 🔖️Wire
/// 📮️ `POST /directory/commands`'s `202` body (contract §C2).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandOutcome {
    pub events: Vec<DirectoryEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

/// 🏠️ `GET /directory/spaces/{id}`'s body: the space itself plus its members/documents/invites
/// (contract §C2), flattened onto one JSON object rather than nested under a `space` key.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceDetail {
    #[serde(flatten)]
    pub space: SpaceView,
    #[serde(default)]
    pub members: Vec<MemberView>,
    #[serde(default)]
    pub documents: Vec<DocumentView>,
    #[serde(default)]
    pub invites: Vec<InviteView>,
}

/// 🪪️ `GET /auth/sessions/me`'s body (contract §C2, camelCase — this route is NEW this wave).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionView {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    #[serde(rename = "expiresAt")]
    pub expires_at_ms: i64,
}

/// 🎫️ `POST /auth/sessions`'s body. Wire is snake_case (`token`, `user_id`), NOT this contract's
/// general camelCase convention: this route predates the wave (`🌎️hub/📦️bin.rs`'s
/// `CreateAuthSessionResponse` has no `rename_all`) and §C2 marks it "unchanged" — the client
/// matches the ACTUAL wire, not the convention.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionMintResponse {
    pub token: String,
    pub user_id: String,
}
//#endregion 🔖️Wire

//#region 🔖️Client
/// 📇️ Talks the hub's directory REST/WS surface over an injected `DirectoryTransport`. Holds the
/// current bearer token in an `RwLock` so a long-lived client (one per shell session) can be
/// re-authenticated in place after a mint/restore without callers re-constructing it.
pub struct DirectoryClient<T: DirectoryTransport> {
    transport: T,
    base_url: String,
    token: RwLock<Option<String>>,
}

impl<T: DirectoryTransport> DirectoryClient<T> {
    pub fn new(transport: T, base_url: impl Into<String>) -> Self {
        Self { transport, base_url: base_url.into(), token: RwLock::new(None) }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn token(&self) -> Option<String> {
        self.token.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub fn set_token(&self, token: Option<String>) {
        *self.token.write().unwrap_or_else(PoisonError::into_inner) = token;
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }

    /// 🛑️ Checked BEFORE every call reaches `self.transport` — an already-cancelled `ctx` never
    /// builds a request at all (`TransportError::Cancelled`, via the SAME transport call, is the
    /// other half: cancelled WHILE the call was in flight — see that variant's own doc).
    async fn request_json<R: DeserializeOwned>(&self, ctx: &OperationContext, method: HttpMethod, path: &str, body: Option<Vec<u8>>) -> Result<R, DirectoryClientError> {
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.token();
        let response = self.transport.http(ctx, method, &self.url(path), bearer.as_deref(), body).await?;
        match response.status {
            200..=299 => Ok(serde_json::from_slice(&response.body)?),
            401 => Err(DirectoryClientError::Unauthorized),
            status => Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
    }

    pub async fn spaces(&self, ctx: &OperationContext) -> Result<Vec<SpaceView>, DirectoryClientError> {
        self.request_json(ctx, HttpMethod::Get, "/directory/spaces", None).await
    }

    pub async fn space(&self, ctx: &OperationContext, id: &str) -> Result<SpaceDetail, DirectoryClientError> {
        self.request_json(ctx, HttpMethod::Get, &format!("/directory/spaces/{id}"), None).await
    }

    pub async fn events(&self, ctx: &OperationContext, since: u64) -> Result<Vec<DirectoryEvent>, DirectoryClientError> {
        self.request_json(ctx, HttpMethod::Get, &format!("/directory/events?since={since}"), None).await
    }

    pub async fn me(&self, ctx: &OperationContext) -> Result<SessionView, DirectoryClientError> {
        self.request_json(ctx, HttpMethod::Get, "/auth/sessions/me", None).await
    }

    /// 🎫️ Dev-mode session mint (§C2 "unchanged"): does NOT touch `self.token` — the caller
    /// (typically `🪪️identity::mint_or_restore`) decides when a freshly minted token replaces
    /// the current one.
    pub async fn mint_session(&self, ctx: &OperationContext, email: &str) -> Result<SessionMintResponse, DirectoryClientError> {
        let body = serde_json::to_vec(&serde_json::json!({ "email": email }))?;
        self.request_json(ctx, HttpMethod::Post, "/auth/sessions", Some(body)).await
    }

    pub async fn command(&self, ctx: &OperationContext, command: &DirectoryCommand) -> Result<CommandOutcome, DirectoryClientError> {
        let body = serde_json::to_vec(command)?;
        self.request_json(ctx, HttpMethod::Post, "/directory/commands", Some(body)).await
    }

    pub fn stream(self: &Arc<Self>, since: u64) -> DirectoryStream<T>
    where
        T: Clone,
    {
        DirectoryStream::new(self.clone(), since)
    }
}
//#endregion 🔖️Client

//#region 🔖️Stream
/// ⏱️ Reconnect backoff floor/ceiling — same constants `🟦️backbone-worker.ts`'s
/// `HUB_RECONNECT_MIN_MS`/`HUB_RECONNECT_MAX_MS` already use for the document WS.
pub const HUB_RECONNECT_MIN_MS: u64 = 500;
pub const HUB_RECONNECT_MAX_MS: u64 = 30_000;

/// 🔗️ `remote://host:port` / `http(s)://…` → `ws(s)://host:port/directory/ws?token=&since=`
/// (contract §C2). Pure and independently testable, mirroring `🏪️store/🔄️sync`'s `hub_ws_url`.
pub fn directory_ws_url(base_url: &str, token: &str, since: u64) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map_or(base_url, |(_, rest)| rest).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    let encoded_token = urlencoding_component(token);
    format!("{scheme}://{authority}/directory/ws?token={encoded_token}&since={since}")
}

/// 🔤️ Minimal percent-encoding for the one query-string slot (`token`) that can carry
/// characters `&`/`=`/`#`/space — avoids a new dependency for a three-character rule.
fn urlencoding_component(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => encoded.push(byte as char),
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

/// ⏱️ Doubling backoff capped at `HUB_RECONNECT_MAX_MS`, floored at `HUB_RECONNECT_MIN_MS`.
pub fn next_backoff_ms(current_ms: u64) -> u64 {
    current_ms.saturating_mul(2).clamp(HUB_RECONNECT_MIN_MS, HUB_RECONNECT_MAX_MS)
}

/// 📡️ One finite stream turn. `Dial` is executed separately on `Lane::Io`; `ReconnectAt` is armed
/// on the shared `TimerWheel`; `Idle` yields immediately instead of parking a worker on a socket.
pub enum DirectoryStreamTurn<T: DirectoryTransport> {
    Dial { transport: T, url: String },
    Message(DirectoryStreamMessage),
    ReconnectAt(u64),
    Idle,
    Closed,
}

/// 🔁️ `GET /directory/ws?since=` with auto-reconnect that resumes from the last `seq`/`headSeq`
/// this stream has observed — never re-subscribes from the caller's original `since` after a
/// drop, so the hub replays only the gap.
pub struct DirectoryStream<T: DirectoryTransport> {
    client: Arc<DirectoryClient<T>>,
    since: u64,
    connection: Option<T::Ws>,
    backoff_ms: u64,
    reconnect_at_ms: Option<u64>,
    dialing: bool,
    closed: bool,
}

impl<T: DirectoryTransport + Clone> DirectoryStream<T> {
    fn new(client: Arc<DirectoryClient<T>>, since: u64) -> Self {
        Self { client, since, connection: None, backoff_ms: HUB_RECONNECT_MIN_MS, reconnect_at_ms: None, dialing: false, closed: false }
    }

    pub fn since(&self) -> u64 {
        self.since
    }

    pub fn close(&mut self) {
        self.closed = true;
        if let Some(mut connection) = self.connection.take() {
            connection.close();
        }
    }

    /// 🔄️ Applies a completed I/O-lane dial without reading from the socket in the same turn.
    pub fn complete_dial(&mut self, now_ms: u64, result: Result<T::Ws, TransportError>) -> DirectoryStreamTurn<T> {
        self.dialing = false;
        if self.closed {
            return DirectoryStreamTurn::Closed;
        }
        match result {
            Ok(connection) => {
                self.connection = Some(connection);
                self.backoff_ms = HUB_RECONNECT_MIN_MS;
                self.reconnect_at_ms = None;
                DirectoryStreamTurn::Idle
            }
            Err(_) => self.reconnecting(now_ms),
        }
    }

    /// 🔁️ Performs bounded, nonblocking protocol work only; at most eight invalid/control frames
    /// are skipped before yielding so malformed traffic cannot monopolize a worker turn.
    pub fn turn(&mut self, ctx: &OperationContext, now_ms: u64) -> DirectoryStreamTurn<T> {
        if self.closed || ctx.cancel.is_cancelled_now() {
            self.close();
            return DirectoryStreamTurn::Closed;
        }
        if self.dialing {
            return DirectoryStreamTurn::Idle;
        }
        if let Some(reconnect_at_ms) = self.reconnect_at_ms {
            if now_ms < reconnect_at_ms {
                return DirectoryStreamTurn::ReconnectAt(reconnect_at_ms);
            }
            self.reconnect_at_ms = None;
        }
        if self.connection.is_none() {
            let token = self.client.token.read().unwrap_or_else(PoisonError::into_inner).clone().unwrap_or_default();
            let url = directory_ws_url(&self.client.base_url, &token, self.since);
            self.dialing = true;
            return DirectoryStreamTurn::Dial { transport: self.client.transport.clone(), url };
        }
        for _ in 0..8 {
            let Some(connection) = self.connection.as_mut() else { return self.reconnecting(now_ms) };
            match connection.try_recv_text() {
                Ok(DirectoryWsPoll::Text(text)) => {
                    if let Ok(message) = serde_json::from_str::<DirectoryStreamMessage>(&text) {
                        self.track(&message);
                        return DirectoryStreamTurn::Message(message);
                    }
                }
                Ok(DirectoryWsPoll::Pending) => return DirectoryStreamTurn::Idle,
                Ok(DirectoryWsPoll::Closed) | Err(_) => {
                    self.connection = None;
                    return self.reconnecting(now_ms);
                }
            }
        }
        DirectoryStreamTurn::Idle
    }

    fn reconnecting(&mut self, now_ms: u64) -> DirectoryStreamTurn<T> {
        let reconnect_at_ms = now_ms.saturating_add(self.backoff_ms);
        self.reconnect_at_ms = Some(reconnect_at_ms);
        self.backoff_ms = next_backoff_ms(self.backoff_ms);
        DirectoryStreamTurn::ReconnectAt(reconnect_at_ms)
    }

    fn track(&mut self, message: &DirectoryStreamMessage) {
        match message {
            DirectoryStreamMessage::Event { event } => self.since = self.since.max(event.seq),
            DirectoryStreamMessage::Heartbeat { head_seq } => self.since = self.since.max(*head_seq),
            DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } => {}
        }
    }
}

//#endregion 🔖️Stream

//#region 🔖️Native
/// 🐎️ Native transport: HTTP routes through `semio-framework-os-services`'s `HttpPool`/
/// `ComputePool` (this crate's EXISTING optional `ureq` dep supplies the ONE blocking call
/// `HttpPool` admits onto its bounded pool — no per-call `std::thread::spawn` and no private
/// tokio runtime built by this crate), `tokio-tungstenite` for WS (this crate's EXISTING `sync`
/// feature, which already wires it up for `🏪️store/🔄️sync`'s native actor). Both features are
/// required together because a real client needs both halves; either alone would leave the other
/// method type unusable.
#[cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))]
pub mod native {
    use super::{DirectoryTransport, DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
    use semio_framework_actor::{ActorId, PackageId};
    use semio_framework_async::{HostAsyncRuntime, HostFuture, OperationContext, ScopeHandle};
    use semio_framework_os_services::{AsyncHttpTransport, ComputeError, ComputePool, HttpBody, HttpPool, HttpPoolError, HttpRequest as PoolHttpRequest, HttpResponseHead, TokioHostRuntime};
    use std::io::Read;
    use std::net::{TcpStream, ToSocketAddrs};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio_tungstenite::tungstenite::{self, client::IntoClientRequest, stream::MaybeTlsStream, Message};

    const UREQ_HTTP_URL_BYTES: usize = 2_048;
    const UREQ_HTTP_HEADER_ITEMS: usize = 64;
    const UREQ_HTTP_HEADER_BYTES: usize = 16 * 1024;
    const UREQ_HTTP_REQUEST_BODY_BYTES: usize = 16 * 1024 * 1024;
    const UREQ_HTTP_BODY_PAGE_BYTES: usize = 16 * 1024;

    type UreqBodyReader = Box<dyn Read + Send + Sync + 'static>;

    #[cfg(test)]
    mod runtime_identity_tests {
        include!("🪪️runtime/🧪️tests/🦀️.rs");
    }

    pub struct UreqStreamingHttpTransport {
        agent: ureq::Agent,
        compute: Arc<ComputePool>,
        runtime: Arc<TokioHostRuntime>,
        scope: ScopeHandle,
    }

    impl UreqStreamingHttpTransport {
        pub fn new(compute: Arc<ComputePool>, runtime: Arc<TokioHostRuntime>, scope: ScopeHandle) -> Self {
            Self { agent: ureq::Agent::new(), compute, runtime, scope }
        }
    }

    struct UreqStreamingHttpBody {
        reader: Arc<Mutex<Option<UreqBodyReader>>>,
        compute: Arc<ComputePool>,
        runtime: Arc<TokioHostRuntime>,
        scope: ScopeHandle,
        ctx: OperationContext,
    }

    impl HttpBody for UreqStreamingHttpBody {
        fn next_chunk(&mut self) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>> {
            if self.ctx.cancel.is_cancelled_now() {
                return Box::pin(async { Err(HttpPoolError::Transport("ureq HTTP body cancelled".into())) });
            }
            let reader = self.reader.clone();
            let compute = self.compute.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            let ctx = self.ctx.clone();
            Box::pin(async move { compute.run_io(runtime.as_ref(), &scope, ctx, move || ureq_stream_read_page(&reader)).await.map_err(HttpPoolError::Compute)? })
        }
    }

    impl AsyncHttpTransport for UreqStreamingHttpTransport {
        fn start(&self, ctx: &OperationContext, request: PoolHttpRequest) -> HostFuture<Result<(HttpResponseHead, Box<dyn HttpBody>), HttpPoolError>> {
            if ctx.cancel.is_cancelled_now() {
                return Box::pin(async { Err(HttpPoolError::Transport("ureq HTTP request cancelled".into())) });
            }
            let agent = self.agent.clone();
            let compute = self.compute.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            let connect_compute = compute.clone();
            let connect_runtime = runtime.clone();
            let connect_scope = scope.clone();
            let connect_ctx = ctx.clone();
            let body_ctx = ctx.clone();
            Box::pin(async move {
                let (head, reader) = connect_compute.run_io(connect_runtime.as_ref(), &connect_scope, connect_ctx, move || ureq_stream_start(&agent, request)).await.map_err(HttpPoolError::Compute)??;
                let body: Box<dyn HttpBody> = Box::new(UreqStreamingHttpBody { reader: Arc::new(Mutex::new(Some(reader))), compute, runtime, scope, ctx: body_ctx });
                Ok((head, body))
            })
        }
    }

    fn ureq_stream_start(agent: &ureq::Agent, request: PoolHttpRequest) -> Result<(HttpResponseHead, UreqBodyReader), HttpPoolError> {
        if request.url.len() > UREQ_HTTP_URL_BYTES || request.headers.len() > UREQ_HTTP_HEADER_ITEMS || request.body.len() > UREQ_HTTP_REQUEST_BODY_BYTES {
            return Err(HttpPoolError::Transport("ureq HTTP request exceeded fixed credits".into()));
        }
        let mut header_bytes = 0usize;
        for (name, value) in &request.headers {
            if name.bytes().any(|byte| byte <= b' ' || byte == b':') || value.bytes().any(|byte| byte == b'\r' || byte == b'\n') {
                return Err(HttpPoolError::Transport("ureq HTTP request contained invalid header bytes".into()));
            }
            header_bytes = header_bytes
                .checked_add(name.len())
                .and_then(|total| total.checked_add(value.len()))
                .and_then(|total| total.checked_add(4))
                .filter(|total| *total <= UREQ_HTTP_HEADER_BYTES)
                .ok_or_else(|| HttpPoolError::Transport("ureq HTTP request headers exceeded fixed credits".into()))?;
        }
        let _ = header_bytes;
        let response = {
            let mut builder = match request.method.as_str() {
                "GET" => agent.get(&request.url),
                "POST" => agent.post(&request.url),
                "DELETE" => agent.delete(&request.url),
                other => return Err(HttpPoolError::Transport(format!("ureq HTTP transport does not admit method {other}"))),
            };
            for (name, value) in &request.headers {
                builder = builder.set(name, value);
            }
            let outcome = if request.body.is_empty() { builder.call() } else { builder.set("Content-Type", "application/json").send_bytes(&request.body) };
            match outcome {
                Ok(response) => response,
                Err(ureq::Error::Status(_, response)) => response,
                Err(error) => return Err(HttpPoolError::Transport(error.to_string())),
            }
        };
        let status = response.status();
        let mut headers = Vec::with_capacity(4);
        let mut response_header_bytes = 0usize;
        for name in ["Content-Length", "Content-Type", "ETag", "Last-Modified"] {
            if let Some(value) = response.header(name) {
                response_header_bytes = response_header_bytes
                    .checked_add(name.len())
                    .and_then(|total| total.checked_add(value.len()))
                    .filter(|total| *total <= UREQ_HTTP_HEADER_BYTES)
                    .ok_or_else(|| HttpPoolError::Transport("ureq HTTP response headers exceeded fixed credits".into()))?;
                headers.push((name.to_string(), value.to_string()));
            }
        }
        Ok((HttpResponseHead { status, headers }, response.into_reader()))
    }

    fn ureq_stream_read_page(reader: &Arc<Mutex<Option<UreqBodyReader>>>) -> Result<Option<Vec<u8>>, HttpPoolError> {
        let mut slot = reader.lock().map_err(|_| HttpPoolError::Transport("ureq HTTP body lock poisoned".into()))?;
        let body = slot.as_mut().ok_or_else(|| HttpPoolError::Transport("ureq HTTP body reached terminal ownership".into()))?;
        let mut page = vec![0u8; UREQ_HTTP_BODY_PAGE_BYTES];
        let count = body.read(&mut page).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
        if count == 0 {
            slot.take();
            return Ok(None);
        }
        page.truncate(count);
        Ok(Some(page))
    }

    async fn http_method_str(method: HttpMethod) -> &'static str {
        match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Delete => "DELETE",
        }
    }

    type TungsteniteStream = tungstenite::WebSocket<MaybeTlsStream<TcpStream>>;

    pub struct TungsteniteConnection(TungsteniteStream);

    impl DirectoryWsConnection for TungsteniteConnection {
        fn send_text(&mut self, text: String) -> Result<(), TransportError> {
            self.0.send(Message::Text(text.into())).map_err(|error| TransportError::Io(error.to_string()))
        }

        fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
            for _ in 0..8 {
                match self.0.read() {
                    Ok(Message::Text(text)) => return Ok(DirectoryWsPoll::Text(text.to_string())),
                    Ok(Message::Close(_)) => return Ok(DirectoryWsPoll::Closed),
                    Ok(Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_)) => {}
                    Err(tungstenite::Error::Io(error)) if error.kind() == std::io::ErrorKind::WouldBlock => return Ok(DirectoryWsPoll::Pending),
                    Err(tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) => return Ok(DirectoryWsPoll::Closed),
                    Err(error) => return Err(TransportError::Io(error.to_string())),
                }
            }
            Ok(DirectoryWsPoll::Pending)
        }

        fn close(&mut self) {
            let _ = self.0.close(None);
        }
    }

    /// 🚀️ A `DirectoryTransport` for the wgpu native host and native tests. `runtime`/`scope`/
    /// `http_pool` are INJECTED, never constructed by this crate — `TokioHostRuntime` stays
    /// confined to `semio-framework-os-services` (this crate names no tokio type anywhere; see the
    /// packet report's `## tokio-containment evidence`), and one `HttpPool` belongs to the WHOLE
    /// host process (its byte budget/outstanding cap are per-package/per-actor accounting,
    /// meaningless re-created fresh per client). The ideal caller is whoever already owns the host
    /// process's single `TokioHostRuntime`/`HttpPool` — seeded via `with_new_http_pool` below if a
    /// caller has a runtime/scope/`ComputePool` but wants its own pool. `Clone` (every field is
    /// itself cheap to clone — `Arc`/`ScopeHandle`/`PackageId`/`Copy` `ActorId`) so a host that
    /// wants several `DirectoryClient`s sharing the SAME pool/quota bucket can build one transport
    /// once and `.clone()` it per client, rather than re-deriving the pool per client.
    pub struct NativeDirectoryTransport<R: HostAsyncRuntime> {
        runtime: Arc<R>,
        scope: ScopeHandle,
        http_pool: Arc<HttpPool>,
        package: PackageId,
        actor: ActorId,
    }

    impl<R: HostAsyncRuntime> Clone for NativeDirectoryTransport<R> {
        fn clone(&self) -> Self {
            Self { runtime: Arc::clone(&self.runtime), scope: self.scope.clone(), http_pool: Arc::clone(&self.http_pool), package: self.package.clone(), actor: self.actor }
        }
    }

    impl<R: HostAsyncRuntime> NativeDirectoryTransport<R> {
        pub async fn new(runtime: Arc<R>, scope: ScopeHandle, http_pool: Arc<HttpPool>, package: PackageId, actor: ActorId) -> Self {
            Self::new_now(runtime, scope, http_pool, package, actor)
        }

        pub fn new_now(runtime: Arc<R>, scope: ScopeHandle, http_pool: Arc<HttpPool>, package: PackageId, actor: ActorId) -> Self {
            Self { runtime, scope, http_pool, package, actor }
        }
    }

    impl NativeDirectoryTransport<TokioHostRuntime> {
        pub async fn with_new_http_pool(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, package: PackageId, actor: ActorId) -> Self {
            Self::with_new_http_pool_now(runtime, scope, compute, bytes_per_minute_cap, outstanding_cap, package, actor)
        }

        pub fn with_new_http_pool_now(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, package: PackageId, actor: ActorId) -> Self {
            let transport: Arc<dyn AsyncHttpTransport> = Arc::new(UreqStreamingHttpTransport::new(compute, runtime.clone(), scope.clone()));
            Self::new_now(runtime, scope, Arc::new(HttpPool::new_with_async_transport_now(transport, bytes_per_minute_cap, outstanding_cap)), package, actor)
        }
    }

    impl<R: HostAsyncRuntime + 'static> DirectoryTransport for NativeDirectoryTransport<R> {
        type Ws = TungsteniteConnection;
        async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            let mut headers = Vec::new();
            if let Some(token) = bearer {
                headers.push(("Authorization".to_string(), format!("Bearer {token}")));
            }
            let request = PoolHttpRequest { method: http_method_str(method).await.to_string(), url: url.to_string(), headers, body: body.unwrap_or_default() };
            match self.http_pool.request(self.runtime.as_ref(), &self.scope, ctx.clone(), self.package.clone(), self.actor, request).await {
                Ok(response) => Ok(HttpResponse { status: response.status, body: response.body }),
                Err(error) => {
                    let cancelled = matches!(error, HttpPoolError::Compute(ComputeError::WorkerLost)) && ctx.cancel.is_cancelled().await;
                    Err(match error {
                        HttpPoolError::Compute(ComputeError::DeadlineExceeded) => TransportError::DeadlineExceeded,
                        _ if cancelled => TransportError::Cancelled,
                        other => TransportError::Io(other.to_string()),
                    })
                }
            }
        }

        fn open_ws(&self, ctx: &OperationContext, url: &str, timeout_ms: u64) -> Result<Self::Ws, TransportError> {
            if ctx.cancel.is_cancelled_now() {
                return Err(TransportError::Cancelled);
            }
            let request = url.into_client_request().map_err(|error| TransportError::Io(error.to_string()))?;
            let host = request.uri().host().ok_or_else(|| TransportError::Io("directory websocket URL has no host".to_string()))?.to_string();
            let port = request.uri().port_u16().unwrap_or_else(|| if request.uri().scheme_str() == Some("wss") { 443 } else { 80 });
            let timeout = Duration::from_millis(timeout_ms.max(1));
            let addresses = (host.as_str(), port).to_socket_addrs().map_err(|error| TransportError::Io(error.to_string()))?;
            let mut tcp = None;
            for address in addresses {
                if ctx.cancel.is_cancelled_now() {
                    return Err(TransportError::Cancelled);
                }
                if let Ok(stream) = TcpStream::connect_timeout(&address, timeout) {
                    tcp = Some(stream);
                    break;
                }
            }
            let tcp = tcp.ok_or_else(|| TransportError::Io(format!("unable to connect to {host}:{port}")))?;
            tcp.set_read_timeout(Some(timeout)).map_err(|error| TransportError::Io(error.to_string()))?;
            tcp.set_write_timeout(Some(timeout)).map_err(|error| TransportError::Io(error.to_string()))?;
            let (mut socket, _) = tungstenite::client_tls(request, tcp).map_err(|error| TransportError::Io(error.to_string()))?;
            match socket.get_mut() {
                MaybeTlsStream::Plain(stream) => stream.set_nonblocking(true).map_err(|error| TransportError::Io(error.to_string()))?,
                MaybeTlsStream::Rustls(stream) => stream.sock.set_nonblocking(true).map_err(|error| TransportError::Io(error.to_string()))?,
                _ => return Err(TransportError::Io("nonblocking TLS directory sockets are not enabled".to_string())),
            }
            Ok(TungsteniteConnection(socket))
        }
    }

    #[cfg(test)]
    mod streaming_tests {
        use super::*;
        use semio_framework_async::{CancelToken, ScopeOwner, TraceId};
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct MockReader {
            bytes: Vec<u8>,
            cursor: usize,
            page: usize,
            reads: Arc<AtomicUsize>,
            fail_at: Option<usize>,
        }

        impl Read for MockReader {
            fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
                let turn = self.reads.fetch_add(1, Ordering::SeqCst);
                if self.fail_at == Some(turn) {
                    return Err(std::io::Error::other("scripted reader failure"));
                }
                if self.cursor == self.bytes.len() {
                    return Ok(0);
                }
                let count = self.page.min(output.len()).min(self.bytes.len() - self.cursor);
                output[..count].copy_from_slice(&self.bytes[self.cursor..self.cursor + count]);
                self.cursor += count;
                Ok(count)
            }
        }

        fn mock_reader(bytes: Vec<u8>, page: usize, reads: Arc<AtomicUsize>, fail_at: Option<usize>) -> Arc<Mutex<Option<UreqBodyReader>>> {
            Arc::new(Mutex::new(Some(Box::new(MockReader { bytes, cursor: 0, page, reads, fail_at }))))
        }

        #[test]
        fn https_owned_reader_yields_one_bounded_page_per_pull_and_reaches_terminal() {
            let reads = Arc::new(AtomicUsize::new(0));
            let reader = mock_reader(vec![1, 2, 3, 4, 5, 6, 7], 3, reads.clone(), None);
            assert_eq!(reads.load(Ordering::SeqCst), 0);
            assert_eq!(ureq_stream_read_page(&reader).expect("first page"), Some(vec![1, 2, 3]));
            assert_eq!(reads.load(Ordering::SeqCst), 1);
            assert_eq!(ureq_stream_read_page(&reader).expect("second page"), Some(vec![4, 5, 6]));
            assert_eq!(ureq_stream_read_page(&reader).expect("third page"), Some(vec![7]));
            assert_eq!(ureq_stream_read_page(&reader).expect("eof"), None);
            assert!(reader.lock().expect("reader lock").is_none());
        }

        #[test]
        fn https_owned_reader_reports_partial_failure_without_an_implicit_retry() {
            let reads = Arc::new(AtomicUsize::new(0));
            let reader = mock_reader(vec![1, 2, 3, 4], 2, reads.clone(), Some(1));
            assert_eq!(ureq_stream_read_page(&reader).expect("first page"), Some(vec![1, 2]));
            assert!(matches!(ureq_stream_read_page(&reader), Err(HttpPoolError::Transport(message)) if message == "scripted reader failure"));
            assert_eq!(reads.load(Ordering::SeqCst), 2);
        }

        #[test]
        fn https_owned_reader_honours_cancel_before_pulling_the_next_page() {
            let runtime = Arc::new(TokioHostRuntime::new());
            let scope = runtime.open_scope_now(ScopeOwner::Service("ureq_stream_test"), None);
            let compute = Arc::new(runtime.block_on(ComputePool::new(1)));
            let cancel = CancelToken::root_now();
            cancel.cancel_now();
            let reads = Arc::new(AtomicUsize::new(0));
            let reader = mock_reader(vec![1, 2, 3], 1, reads.clone(), None);
            let mut body = UreqStreamingHttpBody { reader, compute, runtime: runtime.clone(), scope, ctx: OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel, capability: None } };
            let result = runtime.block_on(body.next_chunk());
            assert!(matches!(result, Err(HttpPoolError::Transport(message)) if message == "ureq HTTP body cancelled"));
            assert_eq!(reads.load(Ordering::SeqCst), 0);
        }
    }
}

/// 🌉️ Browser wgpu build seam: `web_sys::WebSocket`/`fetch` transport, kept coherent with
/// `🏪️store/🔄️sync`'s own `wasm_actor` precedent (event-callback bridged into an `async`-shaped
/// interface via an unbounded channel) so a future in-wasm host can link this client. NOT the
/// production browser path today — the React shell uses lane 1-C's TypeScript `DirectoryClient`
/// (`🏪️store/🔄️sync`'s own module doc: "The production browser shell instead uses a TS twin").
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, and this bridge is browser-only,
// so it is narrowed to exclude the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub mod browser {
    use super::{DirectoryTransport, DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
    use semio_framework_async::browser::JsFuture;
    use semio_framework_async::OperationContext;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{BinaryType, MessageEvent, RequestInit, Response, WebSocket};

    #[derive(Clone)]
    pub struct BrowserDirectoryTransport;

    impl Default for BrowserDirectoryTransport {
        fn default() -> Self {
            Self
        }
    }

    impl DirectoryTransport for BrowserDirectoryTransport {
        type Ws = BrowserWsConnection;
        async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            let window = web_sys::window().ok_or_else(|| TransportError::Io("no window".to_string()))?;
            let init = RequestInit::new();
            init.set_method(match method {
                HttpMethod::Get => "GET",
                HttpMethod::Post => "POST",
                HttpMethod::Delete => "DELETE",
            });
            if let Some(bytes) = &body {
                let array = js_sys::Uint8Array::from(bytes.as_slice());
                init.set_body(&array);
            }
            let request = web_sys::Request::new_with_str_and_init(url, &init).map_err(|error| TransportError::Io(format!("{error:?}")))?;
            if let Some(token) = bearer {
                request.headers().set("Authorization", &format!("Bearer {token}")).map_err(|error| TransportError::Io(format!("{error:?}")))?;
            }
            let response_value = JsFuture::from(window.fetch_with_request(&request)).await.map_err(|error| TransportError::Io(format!("{error:?}")))?;
            let response: Response = response_value.dyn_into().map_err(|error| TransportError::Io(format!("{error:?}")))?;
            let status = response.status();
            let array_buffer = JsFuture::from(response.array_buffer().map_err(|error| TransportError::Io(format!("{error:?}")))?).await.map_err(|error| TransportError::Io(format!("{error:?}")))?;
            let bytes = js_sys::Uint8Array::new(&array_buffer).to_vec();
            Ok(HttpResponse { status, body: bytes })
        }

        fn open_ws(&self, ctx: &OperationContext, url: &str, _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
            if ctx.cancel.is_cancelled_now() {
                return Err(TransportError::Cancelled);
            }
            let socket = WebSocket::new(url).map_err(|error| TransportError::Io(format!("{error:?}")))?;
            socket.set_binary_type(BinaryType::Blob);
            let (incoming_tx, incoming_rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
                if let Some(text) = event.data().as_string() {
                    let _ = incoming_tx.send(text);
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            Ok(BrowserWsConnection { socket, _onmessage: onmessage, incoming_rx })
        }
    }

    pub struct BrowserWsConnection {
        socket: WebSocket,
        _onmessage: Closure<dyn FnMut(MessageEvent)>,
        incoming_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
    }

    impl DirectoryWsConnection for BrowserWsConnection {
        fn send_text(&mut self, text: String) -> Result<(), TransportError> {
            self.socket.send_with_str(&text).map_err(|error| TransportError::Io(format!("{error:?}")))
        }

        fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
            match self.incoming_rx.try_recv() {
                Ok(text) => Ok(DirectoryWsPoll::Text(text)),
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => Ok(DirectoryWsPoll::Pending),
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => Ok(DirectoryWsPoll::Closed),
            }
        }

        fn close(&mut self) {
            let _ = self.socket.close();
        }
    }
}
//#endregion 🔖️Native

//#region 🧪️Tests
/// 🧪️ Reused by `🪪️identity`'s own tests (`super::client::test_support::FakeTransport`) so the
/// mint-vs-restore decision is exercised against the SAME double this module's stream/HTTP tests
/// use, rather than a second hand-rolled copy.
#[cfg(test)]
pub mod test_support {
    use super::{DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
    use semio_framework_async::OperationContext;
    use std::collections::VecDeque;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Debug, PartialEq)]
    pub struct RecordedRequest {
        pub method: HttpMethod,
        pub url: String,
        pub bearer: Option<String>,
    }

    #[derive(Clone, Default)]
    pub struct FakeTransport {
        pub responses: Arc<Mutex<VecDeque<Result<HttpResponse, TransportError>>>>,
        pub requests: Arc<Mutex<Vec<RecordedRequest>>>,
        pub ws_outcomes: Arc<Mutex<VecDeque<Result<VecDeque<Result<Option<String>, TransportError>>, TransportError>>>>,
        pub ws_urls: Arc<Mutex<Vec<String>>>,
        /// 🧪️ Cooperative yield points `http()` passes through (checking `ctx.cancel` at each one)
        /// BEFORE touching `responses`/`requests` — 0 (the default) keeps every existing test's
        /// synchronous-looking behavior unchanged; a cancellation test sets this > 0 so an
        /// interleaved caller has a real window to flip the token between yields (see
        /// `an_in_flight_request_is_cancelled_when_its_context_is_cancelled` below).
        pub yields_before_response: Arc<AtomicU32>,
    }

    impl FakeTransport {
        pub async fn push_response(&self, response: Result<HttpResponse, TransportError>) {
            self.responses.lock().unwrap().push_back(response);
        }

        pub async fn push_ws(&self, outcome: Result<VecDeque<Result<Option<String>, TransportError>>, TransportError>) {
            self.ws_outcomes.lock().unwrap().push_back(outcome);
        }

        pub async fn json_response(status: u16, body: &serde_json::Value) -> Result<HttpResponse, TransportError> {
            Ok(HttpResponse { status, body: serde_json::to_vec(body).unwrap() })
        }
    }

    // 🔀️ `pub` (was private): now named in `impl DirectoryTransport for FakeTransport`'s public
    // `type Ws = FakeWs;` associated type.
    pub struct FakeWs(VecDeque<Result<Option<String>, TransportError>>);

    impl DirectoryWsConnection for FakeWs {
        fn send_text(&mut self, _text: String) -> Result<(), TransportError> {
            Ok(())
        }

        fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
            match self.0.pop_front() {
                Some(Ok(Some(text))) => Ok(DirectoryWsPoll::Text(text)),
                Some(Ok(None)) => Ok(DirectoryWsPoll::Closed),
                Some(Err(error)) => Err(error),
                None => Ok(DirectoryWsPoll::Pending),
            }
        }

        fn close(&mut self) {}
    }

    impl super::DirectoryTransport for FakeTransport {
        type Ws = FakeWs;
        async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, _body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
            for _ in 0..self.yields_before_response.load(Ordering::SeqCst) {
                if ctx.cancel.is_cancelled().await {
                    return Err(TransportError::Cancelled);
                }
                semio_framework_async::yield_once().await;
            }
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            self.requests.lock().unwrap().push(RecordedRequest { method, url: url.to_string(), bearer: bearer.map(str::to_string) });
            self.responses.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted response".to_string())))
        }

        fn open_ws(&self, ctx: &OperationContext, url: &str, _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
            if ctx.cancel.is_cancelled_now() {
                return Err(TransportError::Cancelled);
            }
            self.ws_urls.lock().unwrap().push(url.to_string());
            let frames = self.ws_outcomes.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted ws".to_string())))?;
            Ok(FakeWs(frames))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::FakeTransport;
    use super::*;
    use semio_framework_async::{CancelToken, TraceId};

    fn root_ctx() -> OperationContext {
        OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: CancelToken::root_now(), capability: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn ws_url_switches_scheme_and_encodes_query() {
        assert_eq!(directory_ws_url("http://127.0.0.1:8787", "tok en", 0), "ws://127.0.0.1:8787/directory/ws?token=tok%20en&since=0");
        assert_eq!(directory_ws_url("https://hub.example", "abc", 42), "wss://hub.example/directory/ws?token=abc&since=42");
    }

    #[semio_framework_async_macros::async_test]
    async fn backoff_doubles_and_caps() {
        assert_eq!(next_backoff_ms(500), 1000);
        assert_eq!(next_backoff_ms(20_000), 30_000);
        assert_eq!(next_backoff_ms(30_000), 30_000);
        assert_eq!(next_backoff_ms(0), HUB_RECONNECT_MIN_MS);
    }

    #[semio_framework_async_macros::async_test]
    async fn spaces_decodes_and_sends_bearer() {
        let transport = FakeTransport::default();
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");
        client.set_token(Some("tok".to_string()));

        let spaces = client.spaces(&root_ctx()).await.expect("decodes");
        assert!(spaces.is_empty());
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests[0].url, "http://hub.local/directory/spaces");
        assert_eq!(requests[0].bearer.as_deref(), Some("tok"));
    }

    #[semio_framework_async_macros::async_test]
    async fn unauthorized_status_maps_to_unauthorized_error() {
        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status: 401, body: Vec::new() })).await;
        let client = DirectoryClient::new(transport, "http://hub.local");

        let error = client.me(&root_ctx()).await.expect_err("401 is unauthorized");
        assert!(matches!(error, DirectoryClientError::Unauthorized));
    }

    #[semio_framework_async_macros::async_test]
    async fn stream_reconnects_and_resumes_from_last_seq() {
        let transport = FakeTransport::default();
        transport.push_ws(Err(TransportError::Io("refused".to_string()))).await;
        let event =
            serde_json::json!({ "kind": "event", "event": { "seq": 7, "id": "e1", "hlc": { "physicalMs": 1, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": "sp-1" }, "recordedAtMs": 1 } })
                .to_string();
        transport.push_ws(Ok(std::collections::VecDeque::from([Ok(Some(event)), Ok(None)]))).await;
        transport.push_ws(Err(TransportError::Io("refused again".to_string()))).await;

        let client = Arc::new(DirectoryClient::new(transport.clone(), "http://hub.local"));
        client.set_token(Some("tok".to_string()));
        let mut stream = client.stream(0);
        let ctx = root_ctx();

        let DirectoryStreamTurn::Dial { transport: dial, url } = stream.turn(&ctx, 0) else { panic!("first turn must dial") };
        assert!(matches!(stream.complete_dial(0, dial.open_ws(&ctx, &url, 100)), DirectoryStreamTurn::ReconnectAt(HUB_RECONNECT_MIN_MS)));
        assert!(matches!(stream.turn(&ctx, HUB_RECONNECT_MIN_MS), DirectoryStreamTurn::Dial { .. }));
        let result = transport.open_ws(&ctx, &directory_ws_url("http://hub.local", "tok", 0), 100);
        assert!(matches!(stream.complete_dial(HUB_RECONNECT_MIN_MS, result), DirectoryStreamTurn::Idle));
        match stream.turn(&ctx, HUB_RECONNECT_MIN_MS) {
            DirectoryStreamTurn::Message(DirectoryStreamMessage::Event { event }) => assert_eq!(event.seq, 7),
            _ => panic!("second connection must deliver the event"),
        }
        assert_eq!(stream.since(), 7);
        assert!(matches!(stream.turn(&ctx, HUB_RECONNECT_MIN_MS), DirectoryStreamTurn::ReconnectAt(1_000)));
        let DirectoryStreamTurn::Dial { transport: dial, url } = stream.turn(&ctx, 1_000) else { panic!("reconnect deadline must dial") };
        assert!(matches!(stream.complete_dial(1_000, dial.open_ws(&ctx, &url, 100)), DirectoryStreamTurn::ReconnectAt(2_000)));

        let ws_urls = transport.ws_urls.lock().unwrap();
        assert_eq!(ws_urls.len(), 3);
        assert!(ws_urls[0].ends_with("since=0"));
        assert!(ws_urls[2].ends_with("since=7"), "the resumed dial carries the last-seen seq, not the original since");
    }

    #[semio_framework_async_macros::async_test]
    async fn stream_turn_is_bounded_and_preserves_event_order() {
        let transport = FakeTransport::default();
        let frames = (1..=2)
            .map(|seq| {
                Ok(Some(
                    serde_json::json!({ "kind": "event", "event": { "seq": seq, "id": format!("e{seq}"), "hlc": { "physicalMs": seq, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": format!("sp-{seq}") }, "recordedAtMs": seq } })
                        .to_string(),
                ))
            })
            .collect();
        transport.push_ws(Ok(frames)).await;
        let client = Arc::new(DirectoryClient::new(transport, "http://hub.local"));
        client.set_token(Some("tok".to_string()));
        let ctx = root_ctx();
        let mut stream = client.stream(0);
        let DirectoryStreamTurn::Dial { transport, url } = stream.turn(&ctx, 0) else { panic!("first turn must dial") };
        assert!(matches!(stream.complete_dial(0, transport.open_ws(&ctx, &url, 100)), DirectoryStreamTurn::Idle));

        let started = std::time::Instant::now();
        let mut seqs = Vec::new();
        for _ in 0..2 {
            match stream.turn(&ctx, 0) {
                DirectoryStreamTurn::Message(DirectoryStreamMessage::Event { event }) => seqs.push(event.seq),
                _ => panic!("scripted event must be delivered"),
            }
        }
        assert_eq!(seqs, vec![1, 2]);
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }

    //#region 🔖️CancellationTests
    /// 🧪️ A `ctx` that is ALREADY cancelled before the call starts must never reach the transport
    /// at all — `request_json`'s own up-front check (see `DirectoryClientError::Cancelled`'s doc).
    #[semio_framework_async_macros::async_test]
    async fn a_request_with_an_already_cancelled_context_never_reaches_the_transport() {
        let transport = FakeTransport::default();
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");
        let ctx = root_ctx();
        ctx.cancel.cancel_now();

        let result = client.spaces(&ctx).await;
        assert!(matches!(result, Err(DirectoryClientError::Cancelled)), "got {result:?}");
        assert!(transport.requests.lock().unwrap().is_empty(), "an already-cancelled context must never even build a request");
    }

    /// 🧪️ The property this ticket asks for: an IN-FLIGHT request — already past
    /// `request_json`'s up-front check, genuinely inside the transport call — is cancelled once its
    /// `OperationContext` is cancelled. `FakeTransport::yields_before_response` gives `http()` two
    /// cooperative yield points (checking `ctx.cancel` at each); `semio_framework_async::join2` drives
    /// the request future and a "canceller" future that cancels after ONE yield in lockstep on the
    /// SAME thread — no real time, no real thread, fully deterministic.
    #[semio_framework_async_macros::async_test]
    async fn an_in_flight_request_is_cancelled_when_its_context_is_cancelled() {
        let transport = FakeTransport::default();
        transport.yields_before_response.store(2, std::sync::atomic::Ordering::SeqCst);
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");
        let ctx = root_ctx();

        let request_fut = client.spaces(&ctx);
        let canceller_fut = async {
            semio_framework_async::yield_once().await;
            ctx.cancel.cancel().await;
        };
        let (result, ()) = semio_framework_async::join2(request_fut, canceller_fut).await;
        assert!(matches!(result, Err(DirectoryClientError::Transport(TransportError::Cancelled))), "an in-flight request must observe cancellation, got {result:?}");
        assert!(transport.requests.lock().unwrap().is_empty(), "the cancelled call must never reach the scripted response — the response stays queued, unconsumed");
    }

    /// 🧪️ A cancelled context closes the finite stream state machine rather than reconnecting.
    #[semio_framework_async_macros::async_test]
    async fn cancelling_the_context_closes_an_open_stream() {
        let transport = FakeTransport::default();
        let client = Arc::new(DirectoryClient::new(transport, "http://hub.local"));
        let mut stream = client.stream(0);
        let ctx = root_ctx();
        ctx.cancel.cancel_now();

        assert!(matches!(stream.turn(&ctx, 0), DirectoryStreamTurn::Closed));
        assert!(matches!(stream.turn(&root_ctx(), 0), DirectoryStreamTurn::Closed));
    }
    //#endregion 🔖️CancellationTests
}
//#endregion 🧪️Tests
