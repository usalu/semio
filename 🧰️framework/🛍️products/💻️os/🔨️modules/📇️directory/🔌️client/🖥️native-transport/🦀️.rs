//! 🖥️ Native transport of the directory client: the ureq backed HTTP transport routed through the services HTTP pool, the compute pool admission, the WS connection and the credential frame reads. See [`super`].

use super::{DirectoryTransport, DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, LocalHubCredential, TransportError};
use semio_framework_actor::{ActorId, PackageId};
use semio_framework_async::{HostAsyncRuntime, HostFuture, OperationContext, ScopeHandle};
use semio_framework_os_services::{
    AsyncHttpTransport, ComputeError, ComputePool, HttpBody, HttpBodyCancellationHandle, HttpPool, HttpPoolError, HttpRequest as PoolHttpRequest, HttpResponseHead, HttpTransportStart, HttpTransportTerminalGuard, TokioHostRuntime,
    HTTP_BUCKET_REFILL_INTERVAL_MS,
};
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
#[doc(hidden)] pub const UREQ_HTTP_READ_TIMEOUT_MS: u64 = 15_000;
const UREQ_HTTP_CONNECT_TIMEOUT_MS: u64 = 15_000;
/// ⏱️ The overall bound of a request whose caller named no deadline: long enough for a loaded hub's slowest
/// answer and a 64 MiB execution-target body, finite so the blocking IO thread it occupies is freed.
pub const UREQ_HTTP_UNBOUNDED_REQUEST_MS: u64 = 120_000;

/// ⏱️ One request's ureq overall timeout: what is left of the caller's deadline (at least 1 ms), else
/// [`UREQ_HTTP_UNBOUNDED_REQUEST_MS`]. `now_ms` is the runtime clock `ComputePool::run_io` compares the
/// deadline against, so the transport never cuts a request the caller still waits for.
pub fn ureq_request_budget_ms(deadline_ms: Option<u64>, now_ms: u64) -> u64 {
    deadline_ms.map_or(UREQ_HTTP_UNBOUNDED_REQUEST_MS, |deadline| deadline.saturating_sub(now_ms).max(1))
}

#[doc(hidden)] pub type UreqBodyReader = Box<dyn Read + Send + Sync + 'static>;

pub struct UreqStreamingHttpTransport {
    agent: ureq::Agent,
    #[doc(hidden)] pub compute: Arc<ComputePool>,
    #[doc(hidden)] pub runtime: Arc<TokioHostRuntime>,
    #[doc(hidden)] pub scope: ScopeHandle,
}

impl UreqStreamingHttpTransport {
    pub fn new(compute: Arc<ComputePool>, runtime: Arc<TokioHostRuntime>, scope: ScopeHandle) -> Self {
        Self { agent: ureq::AgentBuilder::new().timeout_connect(Duration::from_millis(UREQ_HTTP_CONNECT_TIMEOUT_MS)).build(), compute, runtime, scope }
    }
}

#[doc(hidden)] pub struct UreqStreamingHttpBody {
    reader: Arc<Mutex<Option<UreqBodyReader>>>,
    cancellation: HttpBodyCancellationHandle,
    compute: Arc<ComputePool>,
    runtime: Arc<TokioHostRuntime>,
    scope: ScopeHandle,
    ctx: OperationContext,
}

impl UreqStreamingHttpBody {
    /// 🌊️ Wraps one owned response reader with the cancellation, compute lane and operation context of its stream.
    #[doc(hidden)] pub fn new(reader: Arc<Mutex<Option<UreqBodyReader>>>, cancellation: HttpBodyCancellationHandle, compute: Arc<ComputePool>, runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, ctx: OperationContext) -> Self {
        Self { reader, cancellation, compute, runtime, scope, ctx }
    }
}

impl HttpBody for UreqStreamingHttpBody {
    fn cancellation_handle(&self) -> HttpBodyCancellationHandle {
        self.cancellation.clone()
    }

    fn next_chunk(&mut self, terminal: HttpTransportTerminalGuard) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>> {
        if self.ctx.cancel.is_cancelled_now() {
            return Box::pin(async move {
                let _terminal = terminal;
                Err(HttpPoolError::Transport("ureq HTTP body cancelled".into()))
            });
        }
        let reader = self.reader.clone();
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let ctx = self.ctx.clone();
        Box::pin(async move {
            compute
                .run_io(runtime.as_ref(), &scope, ctx, move || {
                    let _terminal = terminal;
                    ureq_stream_read_page(&reader)
                })
                .await
                .map_err(HttpPoolError::Compute)?
        })
    }
}

impl AsyncHttpTransport for UreqStreamingHttpTransport {
    fn begin(&self, ctx: &OperationContext, request: PoolHttpRequest) -> HttpTransportStart {
        let cancellation = HttpBodyCancellationHandle::read_deadline(UREQ_HTTP_READ_TIMEOUT_MS);
        let body_cancellation = cancellation.clone();
        let cancelled = ctx.cancel.is_cancelled_now();
        let agent = self.agent.clone();
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let connect_compute = compute.clone();
        let connect_runtime = runtime.clone();
        let connect_scope = scope.clone();
        let connect_ctx = ctx.clone();
        let body_ctx = ctx.clone();
        let (terminal, terminal_guard) = semio_framework_os_services::HttpTransportTerminalHandle::pair();
        let response = Box::pin(async move {
            if cancelled {
                drop(terminal_guard);
                return Err(HttpPoolError::Transport("ureq HTTP request cancelled".into()));
            }
            let budget_ms = ureq_request_budget_ms(connect_ctx.deadline_ms, connect_runtime.now_ms().await);
            let (head, reader) = connect_compute
                .run_io(connect_runtime.as_ref(), &connect_scope, connect_ctx, move || {
                    let _terminal = terminal_guard;
                    ureq_stream_start(&agent, request, Duration::from_millis(budget_ms))
                })
                .await
                .map_err(HttpPoolError::Compute)??;
            let body: Box<dyn HttpBody> = Box::new(UreqStreamingHttpBody::new(Arc::new(Mutex::new(Some(reader))), cancellation, compute, runtime, scope, body_ctx));
            Ok((head, body))
        });
        HttpTransportStart::new(body_cancellation, terminal, response)
    }
}

fn ureq_stream_start(agent: &ureq::Agent, request: PoolHttpRequest, budget: Duration) -> Result<(HttpResponseHead, UreqBodyReader), HttpPoolError> {
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
            other => return Err(HttpPoolError::Transport(format!("ureq HTTP transport does not admit method {other}"))),
        };
        builder = builder.timeout(budget);
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

#[doc(hidden)] pub fn ureq_stream_read_page(reader: &Arc<Mutex<Option<UreqBodyReader>>>) -> Result<Option<Vec<u8>>, HttpPoolError> {
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
    }
}

type TungsteniteStream = tungstenite::WebSocket<MaybeTlsStream<TcpStream>>;

pub struct TungsteniteConnection(TungsteniteStream);

/// 📬️ One nonblocking receive that KEEPS binary frames, unlike
/// [`DirectoryWsConnection::try_recv_text`], which skips them because the directory endpoint's
/// own wire is text JSON. Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W15e: the MCP agent
/// bridge's whole wire is binary (`🌉️mcp/🧵️bridge`'s length-prefixed frames), so a consumer that
/// dropped `Message::Binary` would see an eternally silent socket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeWsPoll {
    Text(String),
    Binary(Vec<u8>),
    Pending,
    Closed(Option<u16>),
}

impl TungsteniteConnection {
    /// 📬️ Bounded nonblocking receive: at most eight control frames are skipped before yielding,
    /// the same ceiling `try_recv_text` uses, so malformed traffic cannot monopolize a turn.
    pub fn try_recv_any(&mut self) -> Result<NativeWsPoll, TransportError> {
        for _ in 0..8 {
            match self.0.read() {
                Ok(Message::Text(text)) => return Ok(NativeWsPoll::Text(text.to_string())),
                Ok(Message::Binary(bytes)) => return Ok(NativeWsPoll::Binary(bytes.to_vec())),
                Ok(Message::Close(frame)) => return Ok(NativeWsPoll::Closed(frame.map(|frame| u16::from(frame.code)))),
                Ok(Message::Ping(_) | Message::Pong(_) | Message::Frame(_)) => {}
                Err(tungstenite::Error::Io(error)) if error.kind() == std::io::ErrorKind::WouldBlock => return Ok(NativeWsPoll::Pending),
                Err(tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) => return Ok(NativeWsPoll::Closed(None)),
                Err(error) => return Err(TransportError::Io(error.to_string())),
            }
        }
        Ok(NativeWsPoll::Pending)
    }
}

impl DirectoryWsConnection for TungsteniteConnection {
    fn send_text(&mut self, text: String) -> Result<(), TransportError> {
        self.0.send(Message::Text(text.into())).map_err(|error| TransportError::Io(error.to_string()))
    }

    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), TransportError> {
        self.0.send(Message::Binary(bytes.into())).map_err(|error| TransportError::Io(error.to_string()))
    }

    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
        for _ in 0..8 {
            match self.0.read() {
                Ok(Message::Text(text)) => return Ok(DirectoryWsPoll::Text(text.to_string())),
                Ok(Message::Close(frame)) => return Ok(DirectoryWsPoll::Closed(frame.map(|frame| u16::from(frame.code)))),
                Ok(Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_)) => {}
                Err(tungstenite::Error::Io(error)) if error.kind() == std::io::ErrorKind::WouldBlock => return Ok(DirectoryWsPoll::Pending),
                Err(tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) => return Ok(DirectoryWsPoll::Closed(None)),
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

    /// 💰️ This transport's package byte budget right now (see [`HttpPool::package_budget_now`]).
    pub fn network_budget(&self) -> semio_framework_os_services::HttpPackageBudget {
        self.http_pool.package_budget_now(&self.package)
    }

    /// 🛡️ Starts one protected bounded binary GET without exposing bearer text to the caller.
    pub async fn fetch_protected_stream(&self, ctx: &OperationContext, credential: &LocalHubCredential, url: &str, accept: &str) -> Result<(HttpResponseHead, semio_framework_os_services::HttpPoolBody), TransportError>
    where
        R: 'static,
    {
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        let origin = credential.hub_origin().trim_end_matches('/');
        if !url.strip_prefix(origin).is_some_and(|path| path.starts_with('/') && !path.starts_with("//")) || !matches!(accept, "application/vnd.semio.canonical-checkpoint-pair.v1") {
            return Err(TransportError::Io("protected binary request authority mismatch".into()));
        }
        let bearer = credential.capability().map_err(|_| TransportError::Io("protected binary credential was invalid".into()))?;
        let request = PoolHttpRequest { method: "GET".to_string(), url: url.to_string(), headers: vec![("Authorization".to_string(), format!("Bearer {bearer}")), ("Accept".to_string(), accept.to_string())], body: Vec::new() };
        self.http_pool.fetch(self.runtime.as_ref(), &self.scope, ctx.clone(), self.package.clone(), self.actor, request).await.map_err(|error| pool_stream_error(ctx, error))
    }

    /// 🧩️ Starts one protected execution-target COMPONENT stream for `intent`: the plugin wasm the hub selected for that
    /// document, handed over chunk by chunk as it arrives, so a caller reports its progress, verifies it against the lease
    /// and keeps it. A component is an immutable, catalog-verified, content-addressed asset, so it belongs on a transport of
    /// its own whose bytes the per-minute budget of the directory's API requests never meters (the semio MCP gateway's
    /// asset transport); the route is built from the intent and the credential's own origin, never from a caller's URL.
    pub async fn execution_target_component_stream(&self, ctx: &OperationContext, credential: &LocalHubCredential, intent: &super::DocumentOpenIntentV1) -> Result<(HttpResponseHead, semio_framework_os_services::HttpPoolBody), TransportError>
    where
        R: 'static,
    {
        intent.validate().map_err(|_| TransportError::Io("document execution-target intent is invalid".into()))?;
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        let bearer = credential.capability().map_err(|_| TransportError::Io("protected asset credential was invalid".into()))?;
        let url = format!(
            "{}/spaces/{}/documents/{}/execution-target/component",
            credential.hub_origin().trim_end_matches('/'),
            super::encode_url_component(&intent.scope.space_id),
            super::encode_url_component(&intent.scope.document_id)
        );
        let request = PoolHttpRequest { method: "POST".to_string(), url, headers: vec![("Authorization".to_string(), format!("Bearer {bearer}"))], body: semio_framework_pack_json::to_json_string(intent).into_bytes() };
        self.http_pool.fetch(self.runtime.as_ref(), &self.scope, ctx.clone(), self.package.clone(), self.actor, request).await.map_err(|error| pool_stream_error(ctx, error))
    }
}

/// 🚦️ A pooled stream's start failure on the transport's closed errors.
fn pool_stream_error(ctx: &OperationContext, error: HttpPoolError) -> TransportError {
    match error {
        HttpPoolError::Compute(ComputeError::DeadlineExceeded) => TransportError::DeadlineExceeded,
        HttpPoolError::ByteBudgetExhausted { .. } => TransportError::BudgetExhausted,
        _ if ctx.cancel.is_cancelled_now() => TransportError::Cancelled,
        other => TransportError::Io(other.to_string()),
    }
}

//#region 💡️Inference
impl<R: HostAsyncRuntime> NativeDirectoryTransport<R> {
    /// 🛡️ Issues one protected bounded JSON request without ever exposing bearer text to the
    /// caller — the small-body twin of [`NativeDirectoryTransport::fetch_protected_stream`], for
    /// hub routes whose request and response are closed JSON documents rather than a stream.
    /// The URL must be an exact path under the credential's own origin, so a caller can never
    /// redirect a protected credential at a foreign host.
    pub async fn request_protected_json(&self, ctx: &OperationContext, credential: &LocalHubCredential, method: HttpMethod, url: &str, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError>
    where
        R: 'static,
    {
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        let origin = credential.hub_origin().trim_end_matches('/');
        if !url.strip_prefix(origin).is_some_and(|path| path.starts_with('/') && !path.starts_with("//")) {
            return Err(TransportError::Io("protected json request authority mismatch".into()));
        }
        let bearer = credential.capability().map_err(|_| TransportError::Io("protected json credential was invalid".into()))?;
        self.http(ctx, method, url, Some(bearer), body).await
    }
}
//#endregion 💡️Inference

impl NativeDirectoryTransport<TokioHostRuntime> {
    pub async fn with_new_http_pool(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, package: PackageId, actor: ActorId) -> Self {
        Self::with_new_http_pool_now(runtime, scope, compute, bytes_per_minute_cap, outstanding_cap, package, actor)
    }

    /// 🔁️ A transport over its own fresh [`HttpPool`] whose per-package byte budget refills every
    /// [`HTTP_BUCKET_REFILL_INTERVAL_MS`] on the runtime's worker pool. Without that driver the "per minute"
    /// budget was a lifetime one: a long-lived client (the semio MCP gateway, a wgpu shell's directory
    /// client) that had moved `bytes_per_minute_cap` bytes answered every later request with an exhausted
    /// budget, forever (ticket 26/09/23, session 14b).
    pub fn with_new_http_pool_now(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, package: PackageId, actor: ActorId) -> Self {
        let http_pool = refilled_http_pool(&runtime, &scope, compute, bytes_per_minute_cap, outstanding_cap, HTTP_BUCKET_REFILL_INTERVAL_MS);
        Self::new_now(runtime, scope, http_pool, package, actor)
    }
}

/// 🚰️ One fresh [`HttpPool`] over the ureq streaming transport whose refill driver already runs on
/// `runtime`'s worker pool every `refill_interval_ms`.
#[doc(hidden)] pub fn refilled_http_pool(runtime: &Arc<TokioHostRuntime>, scope: &ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, refill_interval_ms: u64) -> Arc<HttpPool> {
    let transport: Arc<dyn AsyncHttpTransport> = Arc::new(UreqStreamingHttpTransport::new(compute, runtime.clone(), scope.clone()));
    let http_pool = Arc::new(HttpPool::new_with_async_transport_now(transport, bytes_per_minute_cap, outstanding_cap));
    http_pool.spawn_refill_driver(runtime.worker_pool(), refill_interval_ms);
    http_pool
}

impl<R: HostAsyncRuntime + 'static> NativeDirectoryTransport<R> {
    /// 🌊️ One request through the pooled HTTP lane, its pool failures mapped onto the transport's closed errors.
    async fn pool_request(&self, ctx: &OperationContext, request: PoolHttpRequest) -> Result<HttpResponse, TransportError> {
        match self.http_pool.request(self.runtime.as_ref(), &self.scope, ctx.clone(), self.package.clone(), self.actor, request).await {
            Ok(response) => Ok(HttpResponse { status: response.status, body: response.body }),
            Err(error) => {
                let cancelled = matches!(error, HttpPoolError::Compute(ComputeError::WorkerLost)) && ctx.cancel.is_cancelled().await;
                Err(match error {
                    HttpPoolError::Compute(ComputeError::DeadlineExceeded) => TransportError::DeadlineExceeded,
                    HttpPoolError::ByteBudgetExhausted { .. } => TransportError::BudgetExhausted,
                    _ if cancelled => TransportError::Cancelled,
                    other => TransportError::Io(other.to_string()),
                })
            }
        }
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
        self.pool_request(ctx, request).await
    }

    async fn get_accepting(&self, ctx: &OperationContext, url: &str, bearer: Option<&str>, accept: &str) -> Result<HttpResponse, TransportError> {
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        let mut headers = vec![("Accept".to_string(), accept.to_string())];
        if let Some(token) = bearer {
            headers.push(("Authorization".to_string(), format!("Bearer {token}")));
        }
        self.pool_request(ctx, PoolHttpRequest { method: "GET".to_string(), url: url.to_string(), headers, body: Vec::new() }).await
    }

    fn issue_socket_grant(&self, ctx: &OperationContext, url: &str, bearer: &str, body: &[u8], timeout_ms: u64) -> Result<HttpResponse, TransportError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(TransportError::Cancelled);
        }
        let timeout = Duration::from_millis(timeout_ms.max(1));
        let agent = ureq::AgentBuilder::new().timeout_connect(timeout).timeout_read(timeout).timeout_write(timeout).build();
        let authorization = super::WipeSecretText(format!("Bearer {bearer}"));
        let response = match agent.post(url).set("Authorization", &authorization.0).set("Content-Type", "application/json").send_bytes(body) {
            Ok(response) => response,
            Err(ureq::Error::Status(_, response)) => response,
            Err(error) => return Err(TransportError::Io(error.to_string())),
        };
        let status = response.status();
        let mut body = Vec::new();
        response.into_reader().take((super::DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES + 1) as u64).read_to_end(&mut body).map_err(|error| TransportError::Io(error.to_string()))?;
        Ok(HttpResponse { status, body })
    }

    fn open_ws(&self, ctx: &OperationContext, url: &str, protocols: &[String], timeout_ms: u64) -> Result<Self::Ws, TransportError> {
        if protocols.len() != 2 || protocols[0] != "semio.socket.v1" || !super::valid_socket_grant(&protocols[1]) {
            return Err(TransportError::Io("directory websocket protocol offer invalid".into()));
        }
        open_client_ws(ctx, url, protocols, timeout_ms, Some("semio.socket.v1"))
    }
}

/// 🔌️ ONE native websocket dial, with the directory's own grant policy lifted OUT of it (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W15e). The directory transport keeps its
/// `semio.socket.v1` + grant assertion above and calls this; the wgpu shell's MCP agent bridge
/// calls it with `semio.mcp.bridge.v1` + its supervisor's admission proof.
///
/// ⚖️ Extracted rather than duplicated: the non-blocking-socket setup, the per-address connect
/// timeout and the TLS-stream arms are the parts a second caller would get subtly wrong, and a
/// second copy of them is exactly the kind of drift the directory lane already paid for once.
/// `expect_protocol` is the subprotocol the server MUST have negotiated — `None` accepts whatever
/// it answered, for an endpoint that negotiates none.
pub fn open_client_ws(ctx: &OperationContext, url: &str, protocols: &[String], timeout_ms: u64, expect_protocol: Option<&str>) -> Result<TungsteniteConnection, TransportError> {
    if ctx.cancel.is_cancelled_now() {
        return Err(TransportError::Cancelled);
    }
    let mut request = url.into_client_request().map_err(|error| TransportError::Io(error.to_string()))?;
    if !protocols.is_empty() {
        request.headers_mut().insert("Sec-WebSocket-Protocol", protocols.join(", ").parse().map_err(|_| TransportError::Io("websocket protocol header invalid".into()))?);
    }
    let host = request.uri().host().ok_or_else(|| TransportError::Io("websocket URL has no host".to_string()))?.to_string();
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
    let (mut socket, response) = tungstenite::client_tls(request, tcp).map_err(|error| TransportError::Io(error.to_string()))?;
    if let Some(expected) = expect_protocol {
        if response.headers().get("Sec-WebSocket-Protocol").and_then(|value| value.to_str().ok()) != Some(expected) {
            return Err(TransportError::Io("websocket protocol negotiation invalid".into()));
        }
    }
    match socket.get_mut() {
        MaybeTlsStream::Plain(stream) => stream.set_nonblocking(true).map_err(|error| TransportError::Io(error.to_string()))?,
        MaybeTlsStream::Rustls(stream) => stream.sock.set_nonblocking(true).map_err(|error| TransportError::Io(error.to_string()))?,
        _ => return Err(TransportError::Io("nonblocking TLS sockets are not enabled".to_string())),
    }
    Ok(TungsteniteConnection(socket))
}
