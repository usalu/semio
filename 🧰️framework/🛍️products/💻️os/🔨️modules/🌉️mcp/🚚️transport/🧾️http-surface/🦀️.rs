//! 🧾️ The HTTP transport surface: admission, options, event log, state handle, terminal records and the run handle.

use semio_framework_async::{Job, ProcessKind, WorkerPoolConfig};
use semio_framework_os_kernel::os_directory::client::LocalHubCredential;
use semio_framework_os_mcp_bridge::session::BridgeHandle;
use semio_framework_os_mcp_bridge::shell_command_dispatch::BridgeSlot;
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_server::mcp_server::McpServer;
use semio_framework_os_mcp_wire::notify::{NotificationSink, NotificationSlot};
use semio_framework_os_mcp_wire::protocol::JsonRpcNotification;
use std::collections::VecDeque;
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64};
#[cfg(test)]
use axum::Router;
#[cfg(test)]
use axum::routing::post;
#[cfg(test)]
use crate::bridge_server::bridge_router;
use super::http_authority::HttpTransportAuthority;
use super::http_transport_state::{BridgeWriteCursor, HttpTerminalPolicy, HttpTransportMode, HttpTransportState};
use super::origin_and_handlers::constant_time_eq;
#[cfg(test)]
use super::origin_and_handlers::{handle_get, handle_post};

//#region 🔖️HttpTransport

/// 🔢️ `HeaderMismatch` (Streamable HTTP, `MCP-Protocol-Version` header vs body `_meta` mismatch) —
/// transport-local because it is an HTTP-framing concern the stdio-only `🦀️🟪️protocol.rs` never needs.
pub const HEADER_MISMATCH: i64 = -32020;

#[derive(Clone)]
pub(crate) enum HttpAdmission {
    Credential(Arc<LocalHubCredential>),
    /// 🔑️ A per-process proof minted by `🛰️rendezvous` and published ONLY through the owner-only
    /// offer file — how a stdio gateway, which inherits no hub fd-3 credential, still admits exactly
    /// the shell that read its offer and nothing else on loopback.
    LocalProof(Arc<[u8]>),
    #[cfg(test)]
    Fixture(Arc<[u8]>),
}

impl std::fmt::Debug for HttpAdmission {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("<protected-http-admission>")
    }
}

impl HttpAdmission {
    pub(crate) fn authorizes_capability(&self, candidate: &str) -> bool {
        match self {
            Self::Credential(credential) => credential.authorizes_capability(candidate),
            Self::LocalProof(expected) => constant_time_eq(candidate.as_bytes(), expected),
            #[cfg(test)]
            Self::Fixture(expected) => constant_time_eq(candidate.as_bytes(), expected),
        }
    }

    pub(super) fn authorizes_bearer_header(&self, header: &str) -> bool {
        header.strip_prefix("Bearer ").is_some_and(|candidate| self.authorizes_capability(candidate))
    }
}

/// ⚙️ HTTP and bridge admission share the protected process-entry credential without copying it
/// into an argv-, URL-, file-, or transport-owned string.
#[derive(Debug, Clone)]
pub struct HttpTransportOptions {
    pub bind_addr: SocketAddr,
    pub allowed_origins: Vec<String>,
    pub(crate) admission: HttpAdmission,
}

/// 🌉️ Options for a listener that serves ONLY `/bridge` — the shape `run_stdio` binds so a live os
/// session can attach to a gateway whose MCP surface is stdin/stdout. It never carries an
/// `McpServer`, so `/mcp` is genuinely absent (404) rather than present-but-empty.
impl HttpTransportOptions {
    /// 🔑️ Admission by a per-process [`crate::rendezvous::mint_admission_proof`] value instead of an
    /// inherited hub credential.
    pub fn with_local_proof(proof: &str) -> Self {
        Self { bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0), allowed_origins: Vec::new(), admission: HttpAdmission::LocalProof(Arc::from(proof.as_bytes())) }
    }
}

impl HttpTransportOptions {
    pub fn new(credential: Arc<LocalHubCredential>) -> Self {
        Self { bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6300), allowed_origins: Vec::new(), admission: HttpAdmission::Credential(credential) }
    }

    #[cfg(test)]
    pub fn fixture(capability: &str) -> Self {
        Self { bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6300), allowed_origins: Vec::new(), admission: HttpAdmission::Fixture(Arc::from(capability.as_bytes())) }
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
pub(crate) struct EventLog {
    next_id: u64,
    pub entries: VecDeque<(u64, JsonRpcNotification)>,
}

impl EventLog {
    const CAPACITY: usize = 256;

    pub fn push(&mut self, notification: JsonRpcNotification) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.entries.push_back((id, notification));
        if self.entries.len() > Self::CAPACITY {
            self.entries.pop_front();
        }
        id
    }

    pub fn since(&self, last_id: Option<u64>) -> Vec<(u64, JsonRpcNotification)> {
        let floor = last_id.unwrap_or(0);
        self.entries.iter().filter(|(id, _)| *id > floor).cloned().collect()
    }
}

/// 📤️ A cloneable handle a test (or a future notification-emitting call site) uses to push a
/// server-initiated notification into the same [`EventLog`] the legacy GET handler reads from.
#[derive(Clone)]
pub struct HttpEventPublisher {
    pub(crate) events: Arc<Mutex<EventLog>>,
}

impl HttpEventPublisher {
    pub fn push(&self, notification: JsonRpcNotification) -> u64 {
        self.events.lock().expect("event log lock poisoned").push(notification)
    }
}

/// 📤️ The HTTP connection's server→client notification lane: the very [`EventLog`] the resumable
/// `GET` stream replays from, so `resources/updated`/`progress` reach an HTTP client through the one
/// mechanism this transport already had for server-initiated frames.
impl NotificationSink for HttpEventPublisher {
    fn publish(&self, notification: JsonRpcNotification) {
        self.push(notification);
    }
}

#[derive(Clone)]
pub(crate) struct HttpState {
    /// 🕳️ `None` on a bridge-only listener: that socket serves `/bridge` and answers 404 for `/mcp`.
    pub server: Option<Arc<Mutex<McpServer>>>,
    pub(crate) admission: HttpAdmission,
    pub allowed_origins: Arc<Vec<String>>,
    pub(crate) events: Arc<Mutex<EventLog>>,
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
    pub options: HttpTransportOptions,
    pub bridge_slot: Option<BridgeSlot>,
    pub notification_slot: Option<NotificationSlot>,
}

impl HttpTransport {
    pub fn new(options: HttpTransportOptions) -> Self {
        Self { options, bridge_slot: None, notification_slot: None }
    }

    /// 📤️ Publishes this transport's event log as the server→client notification sink, so an
    /// [`McpServer`] built with `publishing_notifications_into(slot)` reaches HTTP clients through
    /// the same resumable stream the `GET` handler replays. Filled on `start`, because the log the
    /// running transport uses does not exist until then.
    #[must_use]
    pub fn publishing_notifications_into(mut self, slot: NotificationSlot) -> Self {
        self.notification_slot = Some(slot);
        self
    }

    /// 🔌️ Publishes the `/bridge` handle this transport mints on `start` into `slot`, so the tool
    /// registry the caller already built (before this transport existed) resolves the live bridge
    /// from that same slot. The handle is minted from THIS transport's worker pool, which is why the
    /// caller cannot construct it up front and hand it in: doing so would spin a second pool.
    #[must_use]
    pub fn publishing_bridge_into(mut self, slot: BridgeSlot) -> Self {
        self.bridge_slot = Some(slot);
        self
    }

    /// 🧪️ Builds the real axum [`Router`] WITHOUT binding a socket — the foreground, deterministic
    /// entry point every `/mcp` HTTP test in this crate drives through an owned one-shot adapter (no
    /// port allocation, no background process, no timing races). `/bridge` is a real websocket
    /// upgrade, which `oneshot` cannot drive (it never performs a genuine hyper connection upgrade) —
    /// tests exercising `/bridge` bind a real ephemeral (`:0`) socket instead (`mod long` in this file
    /// and in `🧵️bridge/🦀️.rs`), same as P1a/P1b's own websocket tests already did.
    #[cfg(test)]
    pub(crate) fn router(&self, server: McpServer) -> (Router, HttpEventPublisher, BridgeHandle) {
        let events = Arc::new(Mutex::new(EventLog::default()));
        let state = HttpState { server: Some(Arc::new(Mutex::new(server))), admission: self.options.admission.clone(), allowed_origins: Arc::new(self.options.allowed_origins.clone()), events: events.clone() };
        let mcp_router = Router::new().route("/mcp", post(handle_post).get(handle_get)).with_state(state);
        let (bridge_router, bridge_handle) = bridge_router(self.options.admission.clone(), self.options.allowed_origins.clone());
        let router = mcp_router.merge(bridge_router);
        (router, HttpEventPublisher { events }, bridge_handle)
    }

    /// ▶️ Binds a nonblocking listener and transfers it into one retained I/O-lane authority. The
    /// returned run is the only live owner; a process entry may wait on it, cancel it, or retrieve a
    /// terminal connection without exposing a socket/runtime-specific type.
    pub fn start(&mut self, server: McpServer) -> Result<HttpTransportRun, GatewayError> {
        self.start_with(Some(server))
    }

    /// 🌉️ Binds the SAME loopback listener, serving `/bridge` only — no `McpServer`, so `/mcp`
    /// answers 404. `run_stdio` uses this so a live os session can attach to a gateway whose MCP
    /// surface is stdin/stdout. The bound port is read back off the run, because the caller binds
    /// `:0` and only then knows what to publish into its rendezvous offer.
    ///
    /// 🧹️ Nobody drains a bridge-only run's terminal connections (`run_stdio` never calls
    /// [`HttpTransportRun::wait`]), so it closes them itself from the start. Retaining them leaked one
    /// slot per shell connection that ever ended until [`HTTP_CONNECTION_CAPACITY`] was reached, after
    /// which the listener parked for good: every later shell's TCP connect was accepted by the kernel
    /// and never answered (measured 2026-09-26: an 8.6 h old gateway holding exactly 64 CLOSED sockets).
    pub fn start_bridge_only(&mut self) -> Result<HttpTransportRun, GatewayError> {
        let run = self.start_with(None)?;
        run.inner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).terminal_policy = HttpTerminalPolicy::Close;
        run.inner.request_schedule();
        Ok(run)
    }

    fn start_with(&mut self, server: Option<McpServer>) -> Result<HttpTransportRun, GatewayError> {
        let listener = TcpListener::bind(self.options.bind_addr).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot bind {}: {error}", self.options.bind_addr)))?;
        let local_addr = listener.local_addr().map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot read the bound address of {}: {error}", self.options.bind_addr)))?;
        listener.set_nonblocking(true).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot make {} nonblocking: {error}", self.options.bind_addr)))?;
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let pool = semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores));
        let events = Arc::new(Mutex::new(EventLog::default()));
        if let Some(slot) = self.notification_slot.as_ref() {
            let _ = slot.set(Arc::new(HttpEventPublisher { events: events.clone() }));
        }
        let bridge = BridgeHandle::with_pool(pool.clone());
        if let Some(slot) = self.bridge_slot.as_ref() {
            let _ = slot.set(Arc::new(bridge.clone()));
        }
        let state = HttpTransportState::new(listener, server, &self.options, events, bridge.clone());
        let inner = Arc::new(HttpTransportAuthority {
            pool,
            state: Mutex::new(state),
            scheduled: AtomicBool::new(false),
            wake_requested: AtomicBool::new(false),
            retry_armed: AtomicBool::new(false),
            readiness_armed: AtomicBool::new(false),
            retry_generation: AtomicU64::new(0),
            readiness_generation: AtomicU64::new(0),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            completion: (Mutex::new(None), Condvar::new()),
        });
        inner.request_schedule();
        Ok(HttpTransportRun { inner, bridge, local_addr })
    }
}

/// 🧭️ Stable generation key for one accepted connection slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpConnectionKey {
    pub slot: u16,
    pub generation: u64,
}

/// 🧯️ Why an exact connection owner left the live transport authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpTerminalReason {
    Cancelled,
    ClosedByPeer,
    Interrupted,
    Malformed,
    Capacity,
    Unsupported,
    Io,
}

/// 📦️ Exact terminal socket plus every admitted ingress/egress byte still owned by it. Socket
/// internals remain private; a caller may inspect identity/bytes and close the owner explicitly.
pub struct HttpTerminalConnection {
    pub key: HttpConnectionKey,
    pub(crate) peer: SocketAddr,
    pub(crate) stream: Option<TcpStream>,
    pub(crate) ingress: Vec<u8>,
    pub(crate) egress: Vec<u8>,
    pub(crate) bridge_egress: Option<BridgeWriteCursor>,
    pub(crate) reason: HttpTerminalReason,
}

impl HttpTerminalConnection {
    pub fn key(&self) -> HttpConnectionKey {
        self.key
    }

    pub fn peer(&self) -> SocketAddr {
        self.peer
    }

    pub fn ingress(&self) -> &[u8] {
        &self.ingress
    }

    pub fn egress(&self) -> &[u8] {
        &self.egress
    }

    pub fn retained_egress_bytes(&self) -> usize {
        self.egress.len() + self.bridge_egress.as_ref().map_or(0, BridgeWriteCursor::remaining)
    }

    pub fn reason(&self) -> HttpTerminalReason {
        self.reason
    }

    pub fn close(mut self) {
        if let Some(stream) = self.stream.take() {
            let _ = stream.shutdown(Shutdown::Both);
        }
    }
}

/// 🏁️ A waitable view of one run's completion, see [`HttpTransportRun::completion`].
pub struct HttpTransportCompletion {
    inner: Arc<HttpTransportAuthority>,
}

impl HttpTransportCompletion {
    pub fn wait(&self) {
        let (lock, wake) = &self.inner.completion;
        let mut completion = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while completion.is_none() {
            completion = wake.wait(completion).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }
}

/// 🕹️ Opaque owner returned by [`HttpTransport::start`]. Waiting is permitted only at the process
/// entry boundary; every socket operation itself remains a finite `Lane::Io` pool turn.
pub struct HttpTransportRun {
    inner: Arc<HttpTransportAuthority>,
    pub bridge: BridgeHandle,
    pub local_addr: SocketAddr,
}

impl HttpTransportRun {
    pub fn bridge(&self) -> BridgeHandle {
        self.bridge.clone()
    }

    /// 📍️ The address this run actually bound — the caller that asked for an ephemeral port (`:0`,
    /// how `run_stdio` binds its bridge-only listener) learns the real port only here, and only then
    /// can publish a dialable `ws://` url into its rendezvous offer.
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    pub fn cancel(&self) {
        self.bridge.cancel_broadcasts();
        let mut state = self.inner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.mode = HttpTransportMode::Closing;
        state.run_generation = state.run_generation.wrapping_add(1).max(1);
        drop(state);
        self.inner.request_schedule();
    }

    /// 🏁️ Blocks until this run has completed (cancelled, closed or failed) without consuming it or
    /// changing how it treats terminal connections — for a caller that must act when the listener
    /// can no longer serve, e.g. withdraw the rendezvous offer that points at it.
    pub fn completion(&self) -> HttpTransportCompletion {
        HttpTransportCompletion { inner: Arc::clone(&self.inner) }
    }

    pub fn take_terminal_connection(&self) -> Option<HttpTerminalConnection> {
        let owner = self.inner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).terminal.pop_front();
        if owner.is_some() {
            self.inner.clear_readiness();
            self.inner.request_schedule();
        }
        owner
    }

    pub fn take_terminal_job(&self) -> Option<Job> {
        self.inner.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }

    pub fn wait(self) -> Result<(), GatewayError> {
        {
            let mut state = self.inner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            state.terminal_policy = HttpTerminalPolicy::Close;
        }
        self.inner.clear_readiness();
        self.inner.request_schedule();
        let (lock, wake) = &self.inner.completion;
        let mut completion = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            if let Some(result) = completion.take() {
                return result;
            }
            completion = wake.wait(completion).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }
}
//#endregion 🔖️HttpTransport
