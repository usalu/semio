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
use std::sync::{PoisonError, RwLock};

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
#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum TransportError {
    #[error("transport io: {0}")]
    Io(String),
    /// 🛑️ The `OperationContext` handed to this call was already cancelled, or was cancelled
    /// before the call finished — a transport checks this itself (rather than relying solely on
    /// the caller's own pre-check) so an in-flight call started just before cancellation still
    /// surfaces it instead of returning a stale success.
    #[error("cancelled")]
    Cancelled,
    /// ⏰️ `ctx.deadline_ms` elapsed before this call could complete.
    #[error("deadline exceeded")]
    DeadlineExceeded,
}

/// 🔌️ The injection seam: no concrete HTTP client type may appear in any public signature
/// outside an implementor of this trait. Native `ureq`, browser `web_sys::fetch`, or a test
/// double all implement this identically. `?Send` (no `Send`/`Sync` supertrait): the browser
/// impl closes over `wasm_bindgen::JsValue`-backed handles, which are never `Send` — same reason
/// `🏪️store/🔄️sync`'s native actor deliberately runs on a CURRENT-THREAD tokio runtime rather
/// than a multi-threaded one, so no future crossing an `.await` here ever needs to be `Send`.
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
// `Box<dyn DirectoryWsConnection>`. NOT touching the `#[async_trait(?Send)]` wrapper itself here —
// its removal for this `🌎️hub/📇️directory` half is `os-ripple`'s assigned packet (R8), a different
// concern from de-dyn'ing this one return type.
#[async_trait::async_trait(?Send)]
pub trait DirectoryTransport {
    type Ws: DirectoryWsConnection;
    async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError>;
    async fn open_ws(&self, ctx: &OperationContext, url: &str) -> Result<Self::Ws, TransportError>;
}

/// 🔌️ One open `/directory/ws` connection. Sequential by construction (`DirectoryStream` never
/// calls `send_text`/`recv_text` concurrently), so a single stream/sink object suffices — no
/// split halves needed the way `🏪️store/🔄️sync`'s bidirectional relay requires.
#[async_trait::async_trait(?Send)]
pub trait DirectoryWsConnection {
    async fn send_text(&mut self, text: String) -> Result<(), TransportError>;
    /// 📭️ `Ok(None)` means the peer closed the connection cleanly — `DirectoryStream` treats
    /// this the same as a transport error: reconnect with backoff.
    async fn recv_text(&mut self) -> Result<Option<String>, TransportError>;
    async fn close(&mut self);
}
//#endregion 🔖️Transport

//#region 🔖️Errors
#[derive(Debug, thiserror::Error)]
pub enum DirectoryClientError {
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error("decode: {0}")]
    Decode(#[from] serde_json::Error),
    #[error("unauthorized")]
    Unauthorized,
    #[error("http {status}: {body}")]
    Http { status: u16, body: String },
    /// 🛑️ `ctx.cancel` was already cancelled BEFORE this call ever reached the transport — checked
    /// in `DirectoryClient::request_json`/`DirectoryStream::recv` up front, so a cancelled caller
    /// never even builds a request. `TransportError::Cancelled` (via the `Transport` variant above)
    /// is the other half: cancelled WHILE the transport call was in flight.
    #[error("cancelled")]
    Cancelled,
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
    pub async fn new(transport: T, base_url: impl Into<String>) -> Self {
        Self { transport, base_url: base_url.into(), token: RwLock::new(None) }
    }

    pub async fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn token(&self) -> Option<String> {
        self.token.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub async fn set_token(&self, token: Option<String>) {
        *self.token.write().unwrap_or_else(PoisonError::into_inner) = token;
    }

    async fn url(&self, path: &str) -> String {
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
        let response = self.transport.http(ctx, method, &self.url(path).await, bearer.await.as_deref(), body).await?;
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

    pub async fn stream(&self, since: u64) -> DirectoryStream<'_, T> {
        DirectoryStream::new(self, since).await
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
pub async fn directory_ws_url(base_url: &str, token: &str, since: u64) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map(|(_, rest)| rest).unwrap_or(base_url).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    let encoded_token = urlencoding_component(token).await;
    format!("{scheme}://{authority}/directory/ws?token={encoded_token}&since={since}")
}

/// 🔤️ Minimal percent-encoding for the one query-string slot (`token`) that can carry
/// characters `&`/`=`/`#`/space — avoids a new dependency for a three-character rule.
async fn urlencoding_component(value: &str) -> String {
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
pub async fn next_backoff_ms(current_ms: u64) -> u64 {
    current_ms.saturating_mul(2).clamp(HUB_RECONNECT_MIN_MS, HUB_RECONNECT_MAX_MS)
}

/// 📡️ One `DirectoryStream::recv` outcome: either a decoded frame, or a signal that the
/// connection dropped and the caller should sleep `after_ms` before calling `recv` again (the
/// stream owns no timer itself — same division of labor as `🏪️store/🔄️sync`'s actor, which owns
/// its own `tokio::time::interval`).
#[derive(Clone, Debug, PartialEq)]
pub enum DirectoryStreamEvent {
    Message(DirectoryStreamMessage),
    Reconnecting { after_ms: u64 },
}

/// 🔁️ `GET /directory/ws?since=` with auto-reconnect that resumes from the last `seq`/`headSeq`
/// this stream has observed — never re-subscribes from the caller's original `since` after a
/// drop, so the hub replays only the gap.
pub struct DirectoryStream<'a, T: DirectoryTransport> {
    client: &'a DirectoryClient<T>,
    since: u64,
    connection: Option<T::Ws>,
    backoff_ms: u64,
    closed: bool,
}

impl<'a, T: DirectoryTransport> DirectoryStream<'a, T> {
    async fn new(client: &'a DirectoryClient<T>, since: u64) -> Self {
        Self { client, since, connection: None, backoff_ms: HUB_RECONNECT_MIN_MS, closed: false }
    }

    pub async fn since(&self) -> u64 {
        self.since
    }

    pub async fn close(&mut self) {
        self.closed = true;
        self.connection = None;
    }

    /// 🛑️ `ctx` is checked once per loop iteration (top of the loop, same as `request_json`) —
    /// cancelling it closes the stream (same as `close()`) rather than leaving it silently stuck
    /// reconnecting forever against a caller nobody is listening for anymore.
    pub async fn recv(&mut self, ctx: &OperationContext) -> Option<DirectoryStreamEvent> {
        if self.closed {
            return None;
        }
        loop {
            if ctx.cancel.is_cancelled().await {
                self.close().await;
                return None;
            }
            if self.connection.is_none() {
                let token = self.client.token().await.unwrap_or_default();
                let url = directory_ws_url(self.client.base_url().await, &token, self.since).await;
                match self.client.transport.open_ws(ctx, &url).await {
                    Ok(connection) => {
                        self.connection = Some(connection);
                        self.backoff_ms = HUB_RECONNECT_MIN_MS;
                    }
                    Err(_) => return Some(self.reconnecting().await),
                }
            }
            let Some(connection) = self.connection.as_mut() else { return Some(self.reconnecting().await) };
            match connection.recv_text().await {
                Ok(Some(text)) => match serde_json::from_str::<DirectoryStreamMessage>(&text) {
                    Ok(message) => {
                        self.track(&message).await;
                        return Some(DirectoryStreamEvent::Message(message));
                    }
                    Err(_) => continue,
                },
                Ok(None) | Err(_) => {
                    self.connection = None;
                    return Some(self.reconnecting().await);
                }
            }
        }
    }

    async fn reconnecting(&mut self) -> DirectoryStreamEvent {
        let after_ms = self.backoff_ms;
        self.backoff_ms = next_backoff_ms(self.backoff_ms).await;
        DirectoryStreamEvent::Reconnecting { after_ms }
    }

    async fn track(&mut self, message: &DirectoryStreamMessage) {
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
    use super::{DirectoryTransport, DirectoryWsConnection, HttpMethod, HttpResponse, TransportError};
    use futures_util::{SinkExt, StreamExt};
    use semio_framework_actor::{ActorId, PackageId};
    use semio_framework_async::{HostAsyncRuntime, OperationContext, ScopeHandle};
    use semio_framework_os_services::{ComputeError, ComputePool, HttpPool, HttpPoolError, HttpRequest as PoolHttpRequest, HttpResponse as PoolHttpResponse, HttpTransport};
    use std::io::Read;
    use std::sync::Arc;
    use tokio_tungstenite::tungstenite::Message;

    /// 🐎️ The ONE place this crate still names `ureq` directly — a plain, SYNCHRONOUS
    /// `HttpTransport` for `HttpPool`. No thread spawn here anymore: `HttpPool::request` runs this
    /// through `ComputePool::run_blocking`, which admits it onto a BOUNDED semaphore (replacing the
    /// old unbounded `std::thread::spawn` per call) and races it against `ctx.deadline_ms` via the
    /// shared `HostAsyncRuntime`.
    struct UreqHttpTransport {
        agent: ureq::Agent,
    }

    impl HttpTransport for UreqHttpTransport {
        async fn call(&self, request: PoolHttpRequest) -> Result<PoolHttpResponse, std::io::Error> {
            let mut builder = match request.method.as_str() {
                "GET" => self.agent.get(&request.url),
                "POST" => self.agent.post(&request.url),
                "DELETE" => self.agent.delete(&request.url),
                other => return Err(std::io::Error::other(format!("NativeDirectoryTransport: unsupported HTTP method {other}"))),
            };
            for (name, value) in &request.headers {
                builder = builder.set(name, value);
            }
            let outcome = if request.body.is_empty() { builder.call() } else { builder.set("Content-Type", "application/json").send_bytes(&request.body) };
            let response = match outcome {
                Ok(response) => response,
                Err(ureq::Error::Status(status, response)) => {
                    let mut bytes = Vec::new();
                    let _ = response.into_reader().read_to_end(&mut bytes);
                    return Ok(PoolHttpResponse { status, headers: Vec::new(), body: bytes });
                }
                Err(error) => return Err(std::io::Error::other(error.to_string())),
            };
            let status = response.status();
            let mut bytes = Vec::new();
            response.into_reader().read_to_end(&mut bytes).map_err(std::io::Error::other)?;
            Ok(PoolHttpResponse { status, headers: Vec::new(), body: bytes })
        }
    }

    async fn http_method_str(method: HttpMethod) -> &'static str {
        match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Delete => "DELETE",
        }
    }

    type TungsteniteStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

    pub struct TungsteniteConnection(TungsteniteStream);

    #[async_trait::async_trait(?Send)]
    impl DirectoryWsConnection for TungsteniteConnection {
        async fn send_text(&mut self, text: String) -> Result<(), TransportError> {
            self.0.send(Message::Text(text.into())).await.map_err(|error| TransportError::Io(error.to_string()))
        }

        async fn recv_text(&mut self) -> Result<Option<String>, TransportError> {
            loop {
                return match self.0.next().await {
                    Some(Ok(Message::Text(text))) => Ok(Some(text.to_string())),
                    Some(Ok(Message::Ping(_) | Message::Pong(_))) => continue,
                    Some(Ok(_)) => continue,
                    Some(Err(error)) => Err(TransportError::Io(error.to_string())),
                    None => Ok(None),
                };
            }
        }

        async fn close(&mut self) {
            let _ = self.0.close(None).await;
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
    #[derive(Clone)]
    pub struct NativeDirectoryTransport<R: HostAsyncRuntime> {
        runtime: Arc<R>,
        scope: ScopeHandle,
        http_pool: Arc<HttpPool>,
        package: PackageId,
        actor: ActorId,
    }

    impl<R: HostAsyncRuntime> NativeDirectoryTransport<R> {
        pub async fn new(runtime: Arc<R>, scope: ScopeHandle, http_pool: Arc<HttpPool>, package: PackageId, actor: ActorId) -> Self {
            Self { runtime, scope, http_pool, package, actor }
        }

        /// 🐎️ Convenience constructor: builds its own `HttpPool` (and the one `ureq::Agent` behind
        /// it) over a caller-supplied `runtime`/`scope`/`ComputePool` — still never constructs the
        /// runtime itself.
        pub async fn with_new_http_pool(runtime: Arc<R>, scope: ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, package: PackageId, actor: ActorId) -> Self {
            let transport: Arc<dyn HttpTransport> = Arc::new(UreqHttpTransport { agent: ureq::Agent::new() });
            Self::new(runtime, scope, Arc::new(HttpPool::new(transport, compute, bytes_per_minute_cap, outstanding_cap)), package, actor)
        }
    }

    #[async_trait::async_trait(?Send)]
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
            let request = PoolHttpRequest { method: http_method_str(method).to_string(), url: url.to_string(), headers, body: body.unwrap_or_default() };
            match self
                .http_pool
                .request(self.runtime.as_ref(), &self.scope, ctx.clone(), self.package.clone(), self.actor, request)
                .await
            {
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

        async fn open_ws(&self, ctx: &OperationContext, url: &str) -> Result<Self::Ws, TransportError> {
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            let (stream, _response) = tokio_tungstenite::connect_async(url).await.map_err(|error| TransportError::Io(error.to_string()))?;
            Ok(TungsteniteConnection(stream))
        }
    }
}

/// 🌉️ Browser wgpu build seam: `web_sys::WebSocket`/`fetch` transport, kept coherent with
/// `🏪️store/🔄️sync`'s own `wasm_actor` precedent (event-callback bridged into an `async`-shaped
/// interface via an unbounded channel) so a future in-wasm host can link this client. NOT the
/// production browser path today — the React shell uses lane 1-C's TypeScript `DirectoryClient`
/// (`🏪️store/🔄️sync`'s own module doc: "The production browser shell instead uses a TS twin").
#[cfg(target_arch = "wasm32")]
pub mod browser {
    use super::{DirectoryTransport, DirectoryWsConnection, HttpMethod, HttpResponse, TransportError};
    use semio_framework_async::OperationContext;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{BinaryType, MessageEvent, RequestInit, Response, WebSocket};

    pub struct BrowserDirectoryTransport;

    impl Default for BrowserDirectoryTransport {
        fn default() -> Self {
            Self
        }
    }

    #[async_trait::async_trait(?Send)]
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

        async fn open_ws(&self, ctx: &OperationContext, url: &str) -> Result<Self::Ws, TransportError> {
            if ctx.cancel.is_cancelled().await {
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

    #[async_trait::async_trait(?Send)]
    impl DirectoryWsConnection for BrowserWsConnection {
        async fn send_text(&mut self, text: String) -> Result<(), TransportError> {
            self.socket.send_with_str(&text).map_err(|error| TransportError::Io(format!("{error:?}")))
        }

        async fn recv_text(&mut self) -> Result<Option<String>, TransportError> {
            Ok(self.incoming_rx.recv().await)
        }

        async fn close(&mut self) {
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
    use super::{DirectoryWsConnection, HttpMethod, HttpResponse, TransportError};
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

    #[async_trait::async_trait(?Send)]
    impl DirectoryWsConnection for FakeWs {
        async fn send_text(&mut self, _text: String) -> Result<(), TransportError> {
            Ok(())
        }

        async fn recv_text(&mut self) -> Result<Option<String>, TransportError> {
            self.0.pop_front().unwrap_or(Ok(None))
        }

        async fn close(&mut self) {}
    }

    #[async_trait::async_trait(?Send)]
    impl super::DirectoryTransport for FakeTransport {
        type Ws = FakeWs;
        async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, _body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
            for _ in 0..self.yields_before_response.load(Ordering::SeqCst) {
                if ctx.cancel.is_cancelled().await {
                    return Err(TransportError::Cancelled);
                }
                futures_lite::future::yield_now().await;
            }
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            self.requests.lock().unwrap().push(RecordedRequest { method, url: url.to_string(), bearer: bearer.map(str::to_string) });
            self.responses.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted response".to_string())))
        }

        async fn open_ws(&self, ctx: &OperationContext, url: &str) -> Result<Self::Ws, TransportError> {
            if ctx.cancel.is_cancelled().await {
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

    async fn root_ctx() -> OperationContext {
        OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: CancelToken::root().await, capability: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn ws_url_switches_scheme_and_encodes_query() {
        assert_eq!(directory_ws_url("http://127.0.0.1:8787", "tok en", 0).await, "ws://127.0.0.1:8787/directory/ws?token=tok%20en&since=0");
        assert_eq!(directory_ws_url("https://hub.example", "abc", 42).await, "wss://hub.example/directory/ws?token=abc&since=42");
    }

    #[semio_framework_async_macros::async_test]
    async fn backoff_doubles_and_caps() {
        assert_eq!(next_backoff_ms(500).await, 1000);
        assert_eq!(next_backoff_ms(20_000).await, 30_000);
        assert_eq!(next_backoff_ms(30_000).await, 30_000);
        assert_eq!(next_backoff_ms(0).await, HUB_RECONNECT_MIN_MS);
    }

    #[semio_framework_async_macros::async_test]
    async fn spaces_decodes_and_sends_bearer() {
        let transport = FakeTransport::default();
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
        let client = DirectoryClient::new(transport.clone(), "http://hub.local").await;
        client.set_token(Some("tok".to_string())).await;

        let spaces = futures_lite::future::block_on(client.spaces(&root_ctx().await)).expect("decodes");
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

        let error = futures_lite::future::block_on(client.await.me(&root_ctx().await)).expect_err("401 is unauthorized");
        assert!(matches!(error, DirectoryClientError::Unauthorized));
    }

    #[semio_framework_async_macros::async_test]
    async fn stream_reconnects_and_resumes_from_last_seq() {
        let transport = FakeTransport::default();
        transport.push_ws(Err(TransportError::Io("refused".to_string()))).await;
        let event = serde_json::json!({ "kind": "event", "event": { "seq": 7, "id": "e1", "hlc": { "physicalMs": 1, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": "sp-1" }, "recordedAtMs": 1 } }).to_string();
        transport.push_ws(Ok(std::collections::VecDeque::from([Ok(Some(event)), Ok(None)]))).await;
        transport.push_ws(Err(TransportError::Io("refused again".to_string()))).await;

        let client = DirectoryClient::new(transport.clone(), "http://hub.local").await;
        client.set_token(Some("tok".to_string())).await;
        let mut stream = client.stream(0).await;

        assert_eq!(futures_lite::future::block_on(stream.recv(&root_ctx().await)), Some(DirectoryStreamEvent::Reconnecting { after_ms: HUB_RECONNECT_MIN_MS }), "call 1: the first dial fails outright");
        match futures_lite::future::block_on(stream.recv(&root_ctx().await)) {
            Some(DirectoryStreamEvent::Message(DirectoryStreamMessage::Event { event })) => assert_eq!(event.seq, 7),
            other => panic!("call 2: expected the decoded event from the second (successful) dial, got {other:?}"),
        }
        assert_eq!(stream.since().await, 7);
        assert_eq!(
            futures_lite::future::block_on(stream.recv(&root_ctx().await)),
            Some(DirectoryStreamEvent::Reconnecting { after_ms: HUB_RECONNECT_MIN_MS }),
            "call 3: the live connection's next frame is Ok(None) (peer closed) — reported WITHOUT a new dial yet, backoff still at the floor because the prior dial succeeded"
        );
        assert_eq!(
            futures_lite::future::block_on(stream.recv(&root_ctx().await)),
            Some(DirectoryStreamEvent::Reconnecting { after_ms: next_backoff_ms(HUB_RECONNECT_MIN_MS).await }),
            "call 4: NOW the third dial actually happens (lazily, on this call) and fails, so the reported backoff has already advanced past the floor"
        );

        let ws_urls = transport.ws_urls.lock().unwrap();
        assert_eq!(ws_urls.len(), 3, "one failed dial (call 1), one live connection (call 2), one resumed dial (call 4)");
        assert!(ws_urls[0].ends_with("since=0"));
        assert!(ws_urls[2].ends_with("since=7"), "the resumed dial carries the last-seen seq, not the original since");
    }

    //#region 🔖️CancellationTests
    /// 🧪️ A `ctx` that is ALREADY cancelled before the call starts must never reach the transport
    /// at all — `request_json`'s own up-front check (see `DirectoryClientError::Cancelled`'s doc).
    #[semio_framework_async_macros::async_test]
    async fn a_request_with_an_already_cancelled_context_never_reaches_the_transport() {
        let transport = FakeTransport::default();
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");
        let ctx = root_ctx().await;
        ctx.cancel.cancel().await;

        let result = futures_lite::future::block_on(client.await.spaces(&ctx));
        assert!(matches!(result, Err(DirectoryClientError::Cancelled)), "got {result:?}");
        assert!(transport.requests.lock().unwrap().is_empty(), "an already-cancelled context must never even build a request");
    }

    /// 🧪️ The property this ticket asks for: an IN-FLIGHT request — already past
    /// `request_json`'s up-front check, genuinely inside the transport call — is cancelled once its
    /// `OperationContext` is cancelled. `FakeTransport::yields_before_response` gives `http()` two
    /// cooperative yield points (checking `ctx.cancel` at each); `futures_lite::future::zip` drives
    /// the request future and a "canceller" future that cancels after ONE yield in lockstep on the
    /// SAME thread — no real time, no real thread, fully deterministic.
    #[semio_framework_async_macros::async_test]
    async fn an_in_flight_request_is_cancelled_when_its_context_is_cancelled() {
        let transport = FakeTransport::default();
        transport.yields_before_response.store(2, std::sync::atomic::Ordering::SeqCst);
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
        let client = DirectoryClient::new(transport.clone(), "http://hub.local").await;
        let ctx = root_ctx().await;

        let request_fut = client.spaces(&ctx);
        let canceller_fut = async {
            futures_lite::future::yield_now().await;
            ctx.cancel.cancel().await;
        };
        let (result, ()) = futures_lite::future::block_on(futures_lite::future::zip(request_fut, canceller_fut));
        assert!(matches!(result, Err(DirectoryClientError::Transport(TransportError::Cancelled))), "an in-flight request must observe cancellation, got {result:?}");
        assert!(transport.requests.lock().unwrap().is_empty(), "the cancelled call must never reach the scripted response — the response stays queued, unconsumed");
    }

    /// 🧪️ `DirectoryStream::recv` checks `ctx` at the top of its own loop too — cancelling mid-
    /// stream must close it (same as `DirectoryStream::close`) rather than reconnecting forever.
    #[semio_framework_async_macros::async_test]
    async fn cancelling_the_context_closes_an_open_stream() {
        let transport = FakeTransport::default();
        let client = DirectoryClient::new(transport, "http://hub.local").await;
        let mut stream = client.stream(0).await;
        let ctx = root_ctx().await;
        ctx.cancel.cancel().await;

        assert_eq!(futures_lite::future::block_on(stream.recv(&ctx)), None, "a cancelled context must end the stream, never dial");
        assert_eq!(futures_lite::future::block_on(stream.recv(&root_ctx().await)), None, "close() is sticky — a fresh, live context does not resurrect the stream");
    }
    //#endregion 🔖️CancellationTests
}
//#endregion 🧪️Tests
