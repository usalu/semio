//! 🚚️ Transport-level framing over an arbitrary duplex channel — `StdioTransport` (P1a, newline-
//! delimited JSON-RPC on stdin/stdout) and `HttpTransport` (P1b, retained nonblocking Streamable HTTP
//! and WebSocket turns on the process-wide WorkerPool I/O lane; Axum/Tokio is test-oracle only,
//! `📓️luna-mcpspec-audit.md` §A.7 + the spec page fetched live for this packet — resumable legacy GET
//! is this gateway's own dual-era accommodation, NOT part of the 2026-07-28 revision, which removed
//! the GET stream endpoint and protocol-level sessions entirely; see this file's `🔖️HttpTransport`
//! region doc for the exact split). **All diagnostic output goes to a SEPARATE log writer for stdio,
//! never to the response stream** — a stray byte on stdout corrupts every later line the client tries
//! to parse as JSON-RPC (`luna-mcpspec-audit.md`'s stdio guidance).
//!
//! `McpTransport::serve` takes `server: McpServer` BY VALUE for stdio. HTTP transfers the same exact
//! owner through `HttpTransport::start` into `HttpTransportRun`; the process entry may synchronously
//! wait on that opaque completion, but no transport or pool turn constructs/drives a second runtime.

use crate::errors::{GatewayError, GatewayErrorCode};
use crate::protocol::{extract_meta_protocol_version, JsonRpcId, JsonRpcIncoming, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, McpServer, PARSE_ERROR, SUPPORTED_PROTOCOL_VERSIONS, UNSUPPORTED_PROTOCOL_VERSION};
#[cfg(test)]
use axum::body::Body;
#[cfg(test)]
use axum::extract::State;
#[cfg(test)]
use axum::http::{HeaderMap, StatusCode};
#[cfg(test)]
use axum::response::{IntoResponse, Response};
#[cfg(test)]
use axum::routing::post;
#[cfg(test)]
use axum::Router;
use std::collections::VecDeque;
use std::io::{BufRead, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, Weak};

use semio_framework_async::{Job, Lane, ProcessKind, WorkerPool, WorkerPoolConfig, WorkerSubmitErrorKind};

//#region 🔖️McpTransport
/// 🔌️ Drives one [`McpServer`] to completion over the synchronous stdio channel. The retained HTTP
/// owner has an explicit start/wait/control lifecycle because its pool turns must never block.
pub trait McpTransport {
    fn serve(&mut self, server: McpServer) -> Result<(), GatewayError>;
}
//#endregion 🔖️McpTransport

//#region 🔖️StdioTransport
fn io_error(error: std::io::Error) -> GatewayError {
    GatewayError::new(GatewayErrorCode::Internal, format!("stdio transport io error: {error}"))
}

/// 📻️ Newline-delimited JSON-RPC over `input`/`output`, with a THIRD writer (`log`) for every
/// diagnostic line — generic over the three streams so tests exercise the exact same code path a real
/// `stdin`/`stdout`/`stderr` wiring uses, in-memory, without touching the process's real file
/// descriptors.
pub struct StdioTransport<R: BufRead, W: Write, L: Write> {
    input: R,
    output: W,
    log: L,
}

impl<R: BufRead, W: Write, L: Write> StdioTransport<R, W, L> {
    pub fn new(input: R, output: W, log: L) -> Self {
        Self { input, output, log }
    }

    fn write_line(&mut self, line: &str) -> Result<(), GatewayError> {
        writeln!(self.output, "{line}").map_err(io_error)?;
        self.output.flush().map_err(io_error)
    }

    fn write_response(&mut self, response: &JsonRpcResponse) -> Result<(), GatewayError> {
        let line = serde_json::to_string(response).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        self.write_line(&line)
    }

    fn log_line(&mut self, line: &str) {
        let _ = writeln!(self.log, "{line}");
    }
}

impl<R: BufRead, W: Write, L: Write> McpTransport for StdioTransport<R, W, L> {
    /// 🔁️ One line in → zero-or-one line out, until EOF (client closed stdin) or a hard io error. A
    /// blank line is skipped silently (not an error — some clients send a trailing newline). A batch
    /// that dispatches to zero responses (all-notification batch) writes nothing, per JSON-RPC.
    fn serve(&mut self, mut server: McpServer) -> Result<(), GatewayError> {
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = self.input.read_line(&mut line).map_err(io_error)?;
            if bytes_read == 0 {
                return Ok(());
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            match serde_json::from_str::<JsonRpcIncoming>(trimmed) {
                Ok(JsonRpcIncoming::Single(request)) => {
                    if let Some(response) = server.dispatch(&request) {
                        self.write_response(&response)?;
                    }
                }
                Ok(JsonRpcIncoming::Batch(requests)) => {
                    let responses = server.dispatch_batch(&requests);
                    if !responses.is_empty() {
                        let line = serde_json::to_string(&responses).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
                        self.write_line(&line)?;
                    }
                }
                Err(error) => {
                    self.log_line(&format!("stdio transport: malformed JSON-RPC line rejected: {error}"));
                    let response = JsonRpcResponse::error(JsonRpcId::Null, PARSE_ERROR, format!("parse error: {error}"), None);
                    self.write_response(&response)?;
                }
            }
        }
    }
}
//#endregion 🔖️StdioTransport

//#region 🔖️HttpTransport
/// 🔢️ `HeaderMismatch` (Streamable HTTP, `MCP-Protocol-Version` header vs body `_meta` mismatch) —
/// transport-local because it is an HTTP-framing concern the stdio-only `protocol.rs` never needs.
pub const HEADER_MISMATCH: i64 = -32020;

/// ⚙️ `HttpTransport` construction options — bind address (default `127.0.0.1:6300`, never
/// `0.0.0.0`), the required `/mcp` bearer token, the SEPARATE `/bridge` query-string token (P1c —
/// freshly minted per process start, distinct secret from the MCP bearer, see
/// `📓️sol-P1c-packet.md`'s design), and an explicit `Origin` allowlist beyond the always-allowed
/// loopback/`null` set (shared by both endpoints).
#[derive(Debug, Clone)]
pub struct HttpTransportOptions {
    pub bind_addr: SocketAddr,
    pub bearer_token: String,
    pub bridge_token: String,
    pub allowed_origins: Vec<String>,
}

impl HttpTransportOptions {
    pub fn new(bearer_token: impl Into<String>, bridge_token: impl Into<String>) -> Self {
        Self { bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6300), bearer_token: bearer_token.into(), bridge_token: bridge_token.into(), allowed_origins: Vec::new() }
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
struct EventLog {
    next_id: u64,
    entries: VecDeque<(u64, JsonRpcNotification)>,
}

impl EventLog {
    const CAPACITY: usize = 256;

    fn push(&mut self, notification: JsonRpcNotification) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.entries.push_back((id, notification));
        if self.entries.len() > Self::CAPACITY {
            self.entries.pop_front();
        }
        id
    }

    fn since(&self, last_id: Option<u64>) -> Vec<(u64, JsonRpcNotification)> {
        let floor = last_id.unwrap_or(0);
        self.entries.iter().filter(|(id, _)| *id > floor).cloned().collect()
    }
}

/// 📤️ A cloneable handle a test (or a future notification-emitting call site) uses to push a
/// server-initiated notification into the same [`EventLog`] the legacy GET handler reads from.
#[derive(Clone)]
pub struct HttpEventPublisher {
    events: Arc<Mutex<EventLog>>,
}

impl HttpEventPublisher {
    pub fn push(&self, notification: JsonRpcNotification) -> u64 {
        self.events.lock().expect("event log lock poisoned").push(notification)
    }
}

#[derive(Clone)]
struct HttpState {
    server: Arc<Mutex<McpServer>>,
    bearer_token: Arc<str>,
    allowed_origins: Arc<Vec<String>>,
    events: Arc<Mutex<EventLog>>,
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
    options: HttpTransportOptions,
    bridge_slot: Option<crate::ui::BridgeSlot>,
}

impl HttpTransport {
    pub fn new(options: HttpTransportOptions) -> Self {
        Self { options, bridge_slot: None }
    }

    /// 🔌️ Publishes the `/bridge` handle this transport mints on `start` into `slot`, so the tool
    /// registry the caller already built (before this transport existed) resolves the live bridge
    /// from that same slot. The handle is minted from THIS transport's worker pool, which is why the
    /// caller cannot construct it up front and hand it in: doing so would spin a second pool.
    #[must_use]
    pub fn publishing_bridge_into(mut self, slot: crate::ui::BridgeSlot) -> Self {
        self.bridge_slot = Some(slot);
        self
    }

    /// 🧪️ Builds the real axum [`Router`] WITHOUT binding a socket — the foreground, deterministic
    /// entry point every `/mcp` HTTP test in this crate drives through an owned one-shot adapter (no
    /// port allocation, no background process, no timing races). `/bridge` is a real websocket
    /// upgrade, which `oneshot` cannot drive (it never performs a genuine hyper connection upgrade) —
    /// tests exercising `/bridge` bind a real ephemeral (`:0`) socket instead (`mod long` in this file
    /// and in `🧵️bridge/🦀️component.rs`), same as P1a/P1b's own websocket tests already did.
    #[cfg(test)]
    pub(crate) fn router(&self, server: McpServer) -> (Router, HttpEventPublisher, crate::bridge::BridgeHandle) {
        let events = Arc::new(Mutex::new(EventLog::default()));
        let state = HttpState { server: Arc::new(Mutex::new(server)), bearer_token: Arc::from(self.options.bearer_token.as_str()), allowed_origins: Arc::new(self.options.allowed_origins.clone()), events: events.clone() };
        let mcp_router = Router::new().route("/mcp", post(handle_post).get(handle_get)).with_state(state);
        let (bridge_router, bridge_handle) = crate::bridge::server::bridge_router(self.options.bridge_token.clone(), self.options.allowed_origins.clone());
        let router = mcp_router.merge(bridge_router);
        (router, HttpEventPublisher { events }, bridge_handle)
    }

    /// ▶️ Binds a nonblocking listener and transfers it into one retained I/O-lane authority. The
    /// returned run is the only live owner; a process entry may wait on it, cancel it, or retrieve a
    /// terminal connection without exposing a socket/runtime-specific type.
    pub fn start(&mut self, server: McpServer) -> Result<HttpTransportRun, GatewayError> {
        let listener = TcpListener::bind(self.options.bind_addr).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot bind {}: {error}", self.options.bind_addr)))?;
        listener.set_nonblocking(true).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("cannot make {} nonblocking: {error}", self.options.bind_addr)))?;
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let pool = semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores));
        let events = Arc::new(Mutex::new(EventLog::default()));
        let bridge = crate::bridge::BridgeHandle::with_pool(pool.clone());
        if let Some(slot) = self.bridge_slot.as_ref() {
            let _ = slot.set(std::sync::Arc::new(bridge.clone()));
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
        Ok(HttpTransportRun { inner, bridge })
    }
}

/// 🧭️ Stable generation key for one accepted connection slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpConnectionKey {
    slot: u16,
    generation: u64,
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
    key: HttpConnectionKey,
    peer: SocketAddr,
    stream: Option<TcpStream>,
    ingress: Vec<u8>,
    egress: Vec<u8>,
    bridge_egress: Option<BridgeWriteCursor>,
    reason: HttpTerminalReason,
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

/// 🕹️ Opaque owner returned by [`HttpTransport::start`]. Waiting is permitted only at the process
/// entry boundary; every socket operation itself remains a finite `Lane::Io` pool turn.
pub struct HttpTransportRun {
    inner: Arc<HttpTransportAuthority>,
    bridge: crate::bridge::BridgeHandle,
}

impl HttpTransportRun {
    pub fn bridge(&self) -> crate::bridge::BridgeHandle {
        self.bridge.clone()
    }

    pub fn cancel(&self) {
        self.bridge.cancel_broadcasts();
        let mut state = self.inner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.mode = HttpTransportMode::Closing;
        state.run_generation = state.run_generation.wrapping_add(1).max(1);
        drop(state);
        self.inner.request_schedule();
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

const HTTP_CONNECTION_CAPACITY: usize = 64;
const HTTP_HEADER_CAPACITY: usize = 64;
const HTTP_HEADER_BYTES: usize = 65_536;
const HTTP_REQUEST_LINE_BYTES: usize = 4_096;
const HTTP_HEADER_NAME_BYTES: usize = 64;
const HTTP_HEADER_VALUE_BYTES: usize = 8_192;
const HTTP_PATH_BYTES: usize = 2_048;
const HTTP_REQUEST_BYTES: usize = 1_048_576;
const HTTP_RESPONSE_BYTES: usize = 1_048_576;
const HTTP_IO_PAGE_BYTES: usize = 16_384;
const HTTP_SLOWLORIS_MS: u64 = 15_000;
const HTTP_READINESS_POLL_MS: u64 = 4;
const HTTP_RETRY_MS: u64 = 1;
const WEBSOCKET_FRAME_BYTES: usize = 1_048_576;

struct FixedOwnerRing<T, const N: usize> {
    slots: [Option<T>; N],
    head: usize,
    len: usize,
}

struct FixedByteCredits {
    used: usize,
    cap: usize,
}

impl FixedByteCredits {
    fn new(cap: usize) -> Self {
        Self { used: 0, cap }
    }

    fn remaining(&self) -> usize {
        self.cap.saturating_sub(self.used)
    }

    fn try_acquire<T>(&mut self, bytes: usize, owner: T) -> Result<T, T> {
        if bytes > self.remaining() {
            return Err(owner);
        }
        self.used += bytes;
        Ok(owner)
    }

    fn release(&mut self, bytes: usize) {
        self.used = self.used.saturating_sub(bytes);
    }
}

impl<T, const N: usize> FixedOwnerRing<T, N> {
    fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), head: 0, len: 0 }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn try_push(&mut self, owner: T) -> Result<(), T> {
        if self.len == N {
            return Err(owner);
        }
        let index = (self.head + self.len) % N;
        self.slots[index] = Some(owner);
        self.len += 1;
        Ok(())
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let owner = self.slots[self.head].take();
        self.head = (self.head + 1) % N;
        self.len -= 1;
        owner
    }
}

#[derive(Clone)]
struct OwnedHeader {
    name: String,
    value: String,
}

struct ParsedHttpHead {
    method: Option<String>,
    path: Option<String>,
    version: Option<String>,
    headers: [Option<OwnedHeader>; HTTP_HEADER_CAPACITY],
    header_count: usize,
    cursor: usize,
    header_end: Option<usize>,
    content_length: usize,
}

impl ParsedHttpHead {
    fn new() -> Self {
        Self { method: None, path: None, version: None, headers: std::array::from_fn(|_| None), header_count: 0, cursor: 0, header_end: None, content_length: 0 }
    }

    fn header(&self, name: &str) -> Option<&str> {
        self.headers[..self.header_count].iter().flatten().find(|header| header.name.eq_ignore_ascii_case(name)).map(|header| header.value.as_str())
    }

    fn header_occurrences(&self, name: &str) -> usize {
        self.headers[..self.header_count].iter().flatten().filter(|header| header.name.eq_ignore_ascii_case(name)).count()
    }

    fn request_end(&self) -> Option<usize> {
        self.header_end.and_then(|end| end.checked_add(self.content_length))
    }
}

#[derive(Clone, Copy)]
enum HttpAfterWrite {
    Close,
    Upgrade,
    WebSocket,
    WebSocketClose,
}

#[derive(Clone, Copy)]
enum HttpConnectionPhase {
    ReadHttp,
    ParseHttp,
    DispatchHttp,
    Write(HttpAfterWrite),
    ReadWebSocket,
    ParseWebSocket,
    DrainBridgeOutbox,
}

struct BridgeSession {
    id: Option<crate::bridge::ShellConnectionId>,
    outbox: Option<crate::bridge::BridgeOutboxReceiver>,
    opening: bool,
    inbound: Option<BridgeInboundCursor>,
}

enum BridgeInboundPhase {
    Decode(crate::bridge::ShellToGatewayDecodeCursor),
    Materialize(crate::bridge::ShellToGatewayMaterializeCursor),
    Ready(Option<crate::bridge::ShellToGateway>),
}

struct BridgeInboundCursor {
    generation: u64,
    frame: ClientWebSocketFrame,
    phase: BridgeInboundPhase,
}

struct BridgeWriteCursor {
    header: [u8; 10],
    header_len: usize,
    frame: crate::bridge::BridgeEncodedLease,
    written: usize,
    credited: bool,
}

impl BridgeWriteCursor {
    fn total_len(&self) -> usize {
        self.header_len + self.frame.len()
    }

    fn remaining(&self) -> usize {
        self.total_len().saturating_sub(self.written)
    }

    fn credited_bytes(&self) -> usize {
        if self.credited {
            self.total_len()
        } else {
            0
        }
    }

    fn copy_page(&self, output: &mut [u8]) -> usize {
        let mut copied = 0;
        if self.written < self.header_len {
            let bytes = output.len().min(self.header_len - self.written);
            output[..bytes].copy_from_slice(&self.header[self.written..self.written + bytes]);
            copied += bytes;
        }
        let payload_offset = self.written.saturating_sub(self.header_len);
        if copied < output.len() {
            copied += self.frame.copy_into(payload_offset, &mut output[copied..]);
        }
        copied
    }
}

struct HttpConnection {
    key: HttpConnectionKey,
    peer: SocketAddr,
    stream: TcpStream,
    ingress: Vec<u8>,
    egress: Vec<u8>,
    bridge_egress: Option<BridgeWriteCursor>,
    written: usize,
    parser: ParsedHttpHead,
    phase: HttpConnectionPhase,
    bridge: BridgeSession,
    last_progress_ms: u64,
    header_deadline_ms: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HttpTransportMode {
    Running,
    Closing,
    Closed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HttpTerminalPolicy {
    Handback,
    Close,
}

enum HttpTurn {
    MoreWork,
    PollReadiness,
    Parked,
    Complete(Result<(), GatewayError>),
}

struct HttpTransportState {
    listener: Option<TcpListener>,
    connections: [Option<HttpConnection>; HTTP_CONNECTION_CAPACITY],
    terminal: FixedOwnerRing<HttpTerminalConnection, HTTP_CONNECTION_CAPACITY>,
    terminal_policy: HttpTerminalPolicy,
    mode: HttpTransportMode,
    terminal_result: Option<GatewayError>,
    run_generation: u64,
    next_connection_generation: u64,
    io_cursor: usize,
    request_credits: FixedByteCredits,
    response_credits: FixedByteCredits,
    http: HttpState,
    bridge_token: Arc<str>,
    bridge: crate::bridge::BridgeHandle,
}

impl HttpTransportState {
    fn new(listener: TcpListener, server: McpServer, options: &HttpTransportOptions, events: Arc<Mutex<EventLog>>, bridge: crate::bridge::BridgeHandle) -> Self {
        Self {
            listener: Some(listener),
            connections: std::array::from_fn(|_| None),
            terminal: FixedOwnerRing::new(),
            terminal_policy: HttpTerminalPolicy::Handback,
            mode: HttpTransportMode::Running,
            terminal_result: None,
            run_generation: 1,
            next_connection_generation: 1,
            io_cursor: 0,
            request_credits: FixedByteCredits::new(HTTP_CONNECTION_CAPACITY * HTTP_REQUEST_BYTES),
            response_credits: FixedByteCredits::new(HTTP_CONNECTION_CAPACITY * HTTP_RESPONSE_BYTES),
            http: HttpState { server: Arc::new(Mutex::new(server)), bearer_token: Arc::from(options.bearer_token.as_str()), allowed_origins: Arc::new(options.allowed_origins.clone()), events },
            bridge_token: Arc::from(options.bridge_token.as_str()),
            bridge,
        }
    }

    fn active_connections(&self) -> usize {
        self.connections.iter().filter(|slot| slot.is_some()).count()
    }

    fn drive_one(&mut self, now_ms: u64) -> HttpTurn {
        if self.terminal_policy == HttpTerminalPolicy::Close {
            if let Some(owner) = self.terminal.pop_front() {
                owner.close();
                return HttpTurn::MoreWork;
            }
        }
        if self.mode == HttpTransportMode::Closing {
            if let Some(index) = self.connections.iter().position(Option::is_some) {
                let connection = self.connections[index].take().expect("terminal connection slot disappeared");
                self.terminalize(connection, HttpTerminalReason::Cancelled);
                return HttpTurn::MoreWork;
            }
            self.listener.take();
            if self.terminal.len() != 0 {
                return HttpTurn::Parked;
            }
            self.mode = HttpTransportMode::Closed;
            return HttpTurn::Complete(self.terminal_result.take().map_or(Ok(()), Err));
        }
        if self.mode == HttpTransportMode::Closed {
            return HttpTurn::Complete(Ok(()));
        }

        if let Some(index) = self.next_non_io_connection() {
            return self.drive_connection(index, now_ms);
        }

        let active = self.active_connections();
        let opportunity = self.io_cursor % (active + 1);
        self.io_cursor = self.io_cursor.wrapping_add(1);
        if opportunity == 0 {
            self.accept_one(now_ms)
        } else {
            let index = self.connections.iter().enumerate().filter_map(|(index, slot)| slot.as_ref().map(|_| index)).nth(opportunity - 1).expect("active HTTP connection census changed inside one retained turn");
            self.drive_connection(index, now_ms)
        }
    }

    fn next_non_io_connection(&self) -> Option<usize> {
        self.connections.iter().position(|slot| {
            slot.as_ref()
                .is_some_and(|connection| matches!(connection.phase, HttpConnectionPhase::ParseHttp | HttpConnectionPhase::DispatchHttp | HttpConnectionPhase::ParseWebSocket | HttpConnectionPhase::DrainBridgeOutbox | HttpConnectionPhase::Write(_)))
        })
    }

    fn accept_one(&mut self, now_ms: u64) -> HttpTurn {
        if self.active_connections() + self.terminal.len() >= HTTP_CONNECTION_CAPACITY {
            return HttpTurn::Parked;
        }
        let Some(index) = self.connections.iter().position(Option::is_none) else { return HttpTurn::PollReadiness };
        let Some(listener) = self.listener.as_ref() else {
            return self.begin_failure(GatewayError::new(GatewayErrorCode::Internal, "http listener owner missing"));
        };
        let accepted = listener.accept();
        match accepted {
            Ok((stream, peer)) => {
                if let Err(_error) = stream.set_nonblocking(true) {
                    let key = HttpConnectionKey { slot: index as u16, generation: self.next_connection_generation };
                    self.next_connection_generation = self.next_connection_generation.wrapping_add(1).max(1);
                    let owner = HttpTerminalConnection { key, peer, stream: Some(stream), ingress: Vec::new(), egress: Vec::new(), bridge_egress: None, reason: HttpTerminalReason::Io };
                    self.retain_terminal(owner);
                    return HttpTurn::MoreWork;
                }
                let key = HttpConnectionKey { slot: index as u16, generation: self.next_connection_generation };
                self.next_connection_generation = self.next_connection_generation.wrapping_add(1).max(1);
                let mut ingress = Vec::new();
                let mut egress = Vec::new();
                ingress.try_reserve_exact(HTTP_REQUEST_BYTES).expect("http ingress fixed credit reservation failed");
                egress.try_reserve_exact(HTTP_RESPONSE_BYTES).expect("http egress fixed credit reservation failed");
                self.connections[index] = Some(HttpConnection {
                    key,
                    peer,
                    stream,
                    ingress,
                    egress,
                    bridge_egress: None,
                    written: 0,
                    parser: ParsedHttpHead::new(),
                    phase: HttpConnectionPhase::ReadHttp,
                    bridge: BridgeSession { id: None, outbox: None, opening: false, inbound: None },
                    last_progress_ms: now_ms,
                    header_deadline_ms: now_ms.saturating_add(HTTP_SLOWLORIS_MS),
                });
                HttpTurn::MoreWork
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => HttpTurn::PollReadiness,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => HttpTurn::MoreWork,
            Err(error) => self.begin_failure(GatewayError::new(GatewayErrorCode::Internal, format!("http accept error: {error}"))),
        }
    }

    fn begin_failure(&mut self, error: GatewayError) -> HttpTurn {
        self.terminal_result = Some(error);
        self.mode = HttpTransportMode::Closing;
        self.run_generation = self.run_generation.wrapping_add(1).max(1);
        HttpTurn::MoreWork
    }

    fn drive_connection(&mut self, index: usize, now_ms: u64) -> HttpTurn {
        let mut connection = self.connections[index].take().expect("http connection slot disappeared");
        if now_ms >= connection.header_deadline_ms && matches!(connection.phase, HttpConnectionPhase::ReadHttp | HttpConnectionPhase::ParseHttp) {
            self.terminalize(connection, HttpTerminalReason::Interrupted);
            return HttpTurn::MoreWork;
        }
        let turn = match connection.phase {
            HttpConnectionPhase::ReadHttp | HttpConnectionPhase::ReadWebSocket => self.read_one_page(&mut connection, now_ms),
            HttpConnectionPhase::ParseHttp => self.parse_one_http_token(&mut connection),
            HttpConnectionPhase::DispatchHttp => self.dispatch_one_request(&mut connection),
            HttpConnectionPhase::Write(_) => self.write_one_page(&mut connection, now_ms),
            HttpConnectionPhase::ParseWebSocket => self.parse_one_websocket_frame(&mut connection),
            HttpConnectionPhase::DrainBridgeOutbox => self.drain_one_bridge_frame(&mut connection),
        };
        match turn {
            ConnectionTurn::Keep(next) => {
                connection.phase = next;
                self.connections[index] = Some(connection);
                if matches!(self.connections[index].as_ref().expect("connection just retained").phase, HttpConnectionPhase::ReadHttp | HttpConnectionPhase::ReadWebSocket | HttpConnectionPhase::DrainBridgeOutbox) {
                    HttpTurn::PollReadiness
                } else {
                    HttpTurn::MoreWork
                }
            }
            ConnectionTurn::Close => {
                self.release_connection(connection);
                HttpTurn::MoreWork
            }
            ConnectionTurn::Terminal(reason) => {
                self.terminalize(connection, reason);
                HttpTurn::MoreWork
            }
        }
    }

    fn release_connection(&mut self, mut connection: HttpConnection) {
        self.request_credits.release(connection.ingress.len());
        self.response_credits.release(connection.egress.len() + connection.bridge_egress.as_ref().map_or(0, BridgeWriteCursor::credited_bytes));
        if let Some(id) = connection.bridge.id.take() {
            self.bridge.unregister(id);
        }
        let _ = connection.stream.shutdown(Shutdown::Both);
    }

    fn terminalize(&mut self, mut connection: HttpConnection, reason: HttpTerminalReason) {
        self.request_credits.release(connection.ingress.len());
        self.response_credits.release(connection.egress.len() + connection.bridge_egress.as_ref().map_or(0, BridgeWriteCursor::credited_bytes));
        if let Some(id) = connection.bridge.id.take() {
            self.bridge.unregister(id);
        }
        let owner = HttpTerminalConnection { key: connection.key, peer: connection.peer, stream: Some(connection.stream), ingress: connection.ingress, egress: connection.egress, bridge_egress: connection.bridge_egress, reason };
        self.retain_terminal(owner);
    }

    fn retain_terminal(&mut self, owner: HttpTerminalConnection) {
        if let Err(owner) = self.terminal.try_push(owner) {
            owner.close();
            self.terminal_result = Some(GatewayError::new(GatewayErrorCode::Internal, "http terminal owner capacity invariant violated"));
            self.mode = HttpTransportMode::Closing;
        }
    }
}

impl HttpTransportState {
    fn read_one_page(&mut self, connection: &mut HttpConnection, now_ms: u64) -> ConnectionTurn {
        let per_connection_remaining = HTTP_REQUEST_BYTES.saturating_sub(connection.ingress.len());
        let total_remaining = self.request_credits.remaining();
        let page_bytes = HTTP_IO_PAGE_BYTES.min(per_connection_remaining).min(total_remaining);
        if page_bytes == 0 {
            return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
        }
        let mut page = [0u8; HTTP_IO_PAGE_BYTES];
        match connection.stream.read(&mut page[..page_bytes]) {
            Ok(0) => ConnectionTurn::Terminal(HttpTerminalReason::ClosedByPeer),
            Ok(bytes) => {
                if self.request_credits.try_acquire(bytes, ()).is_err() {
                    return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
                }
                connection.ingress.extend_from_slice(&page[..bytes]);
                connection.last_progress_ms = now_ms;
                match connection.phase {
                    HttpConnectionPhase::ReadHttp => ConnectionTurn::Keep(HttpConnectionPhase::ParseHttp),
                    HttpConnectionPhase::ReadWebSocket => ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket),
                    _ => ConnectionTurn::Terminal(HttpTerminalReason::Malformed),
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => match connection.phase {
                HttpConnectionPhase::ReadWebSocket => ConnectionTurn::Keep(HttpConnectionPhase::DrainBridgeOutbox),
                _ => ConnectionTurn::Keep(connection.phase),
            },
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => ConnectionTurn::Keep(connection.phase),
            Err(_) => ConnectionTurn::Terminal(HttpTerminalReason::Io),
        }
    }

    fn parse_one_http_token(&mut self, connection: &mut HttpConnection) -> ConnectionTurn {
        if let Some(request_end) = connection.parser.request_end() {
            return if connection.ingress.len() >= request_end {
                ConnectionTurn::Keep(HttpConnectionPhase::DispatchHttp)
            } else if connection.ingress.len() == HTTP_REQUEST_BYTES {
                ConnectionTurn::Terminal(HttpTerminalReason::Capacity)
            } else {
                ConnectionTurn::Keep(HttpConnectionPhase::ReadHttp)
            };
        }
        if connection.parser.cursor >= HTTP_HEADER_BYTES {
            return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
        }
        let line_bound = if connection.parser.method.is_none() { HTTP_REQUEST_LINE_BYTES } else { HTTP_HEADER_BYTES };
        let search_end = connection.ingress.len().min(line_bound);
        let Some(line_end) = find_crlf_bounded(&connection.ingress, connection.parser.cursor, search_end) else {
            return if connection.ingress.len() >= line_bound { ConnectionTurn::Terminal(HttpTerminalReason::Capacity) } else { ConnectionTurn::Keep(HttpConnectionPhase::ReadHttp) };
        };
        let line_start = connection.parser.cursor;
        let line = &connection.ingress[line_start..line_end];
        connection.parser.cursor = line_end + 2;
        if connection.parser.method.is_none() {
            let Ok(line) = std::str::from_utf8(line) else { return ConnectionTurn::Terminal(HttpTerminalReason::Malformed) };
            let mut parts = line.split_whitespace();
            let (Some(method), Some(path), Some(version), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else { return ConnectionTurn::Terminal(HttpTerminalReason::Malformed) };
            if path.len() > HTTP_PATH_BYTES || !version.starts_with("HTTP/1.") {
                return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
            }
            connection.parser.method = Some(method.to_string());
            connection.parser.path = Some(path.to_string());
            connection.parser.version = Some(version.to_string());
            return ConnectionTurn::Keep(HttpConnectionPhase::ParseHttp);
        }
        if line.is_empty() {
            connection.parser.header_end = Some(connection.parser.cursor);
            if connection.parser.header("transfer-encoding").is_some() {
                return ConnectionTurn::Terminal(HttpTerminalReason::Unsupported);
            }
            connection.parser.content_length = match connection.parser.header("content-length") {
                Some(value) => match value.parse::<usize>() {
                    Ok(value) => value,
                    Err(_) => return ConnectionTurn::Terminal(HttpTerminalReason::Malformed),
                },
                None => 0,
            };
            let Some(request_end) = connection.parser.request_end() else { return ConnectionTurn::Terminal(HttpTerminalReason::Capacity) };
            if request_end > HTTP_REQUEST_BYTES {
                return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
            }
            return if connection.ingress.len() >= request_end { ConnectionTurn::Keep(HttpConnectionPhase::DispatchHttp) } else { ConnectionTurn::Keep(HttpConnectionPhase::ReadHttp) };
        }
        if connection.parser.header_count == HTTP_HEADER_CAPACITY {
            return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
        }
        let Ok(line) = std::str::from_utf8(line) else { return ConnectionTurn::Terminal(HttpTerminalReason::Malformed) };
        let Some((name, value)) = line.split_once(':') else { return ConnectionTurn::Terminal(HttpTerminalReason::Malformed) };
        let name = name.trim();
        let value = value.trim();
        if name.is_empty() || name.len() > HTTP_HEADER_NAME_BYTES || value.len() > HTTP_HEADER_VALUE_BYTES || !name.bytes().all(is_http_token_byte) {
            return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
        }
        connection.parser.headers[connection.parser.header_count] = Some(OwnedHeader { name: name.to_ascii_lowercase(), value: value.to_string() });
        connection.parser.header_count += 1;
        ConnectionTurn::Keep(HttpConnectionPhase::ParseHttp)
    }

    fn dispatch_one_request(&mut self, connection: &mut HttpConnection) -> ConnectionTurn {
        let Some(request_end) = connection.parser.request_end() else { return ConnectionTurn::Terminal(HttpTerminalReason::Malformed) };
        if request_end > connection.ingress.len() {
            return ConnectionTurn::Keep(HttpConnectionPhase::ReadHttp);
        }
        let response = match dispatch_owned_http(&self.http, &self.bridge_token, &connection.parser, &connection.ingress[..request_end]) {
            Ok(response) => response,
            Err(reason) => return ConnectionTurn::Terminal(reason),
        };
        if response.bytes.len() > HTTP_RESPONSE_BYTES || self.response_credits.try_acquire(response.bytes.len(), ()).is_err() {
            return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
        }
        let trailing = connection.ingress.split_off(request_end);
        self.request_credits.release(request_end);
        connection.ingress = trailing;
        connection.parser = ParsedHttpHead::new();
        connection.egress = response.bytes;
        connection.written = 0;
        connection.bridge.opening = response.upgrade;
        ConnectionTurn::Keep(HttpConnectionPhase::Write(if response.upgrade { HttpAfterWrite::Upgrade } else { HttpAfterWrite::Close }))
    }

    fn write_one_page(&mut self, connection: &mut HttpConnection, now_ms: u64) -> ConnectionTurn {
        let HttpConnectionPhase::Write(after) = connection.phase else { return ConnectionTurn::Terminal(HttpTerminalReason::Malformed) };
        if connection.bridge_egress.is_some() {
            return self.write_one_bridge_page(connection, now_ms, after);
        }
        if connection.written >= connection.egress.len() {
            return self.finish_write(connection, after);
        }
        let end = (connection.written + HTTP_IO_PAGE_BYTES).min(connection.egress.len());
        match connection.stream.write(&connection.egress[connection.written..end]) {
            Ok(0) => ConnectionTurn::Terminal(HttpTerminalReason::Interrupted),
            Ok(bytes) => {
                connection.written += bytes;
                connection.last_progress_ms = now_ms;
                if connection.written == connection.egress.len() {
                    self.finish_write(connection, after)
                } else {
                    ConnectionTurn::Keep(HttpConnectionPhase::Write(after))
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock || error.kind() == std::io::ErrorKind::Interrupted => ConnectionTurn::Keep(HttpConnectionPhase::Write(after)),
            Err(_) => ConnectionTurn::Terminal(HttpTerminalReason::Io),
        }
    }

    fn write_one_bridge_page(&mut self, connection: &mut HttpConnection, now_ms: u64, after: HttpAfterWrite) -> ConnectionTurn {
        let cursor = connection.bridge_egress.as_mut().expect("bridge write cursor disappeared");
        if cursor.remaining() == 0 {
            return self.finish_bridge_write(connection, after);
        }
        let mut page = [0u8; HTTP_IO_PAGE_BYTES];
        let page_len = cursor.copy_page(&mut page);
        match connection.stream.write(&page[..page_len]) {
            Ok(0) => ConnectionTurn::Terminal(HttpTerminalReason::Interrupted),
            Ok(bytes) => {
                cursor.written += bytes;
                connection.last_progress_ms = now_ms;
                if cursor.remaining() == 0 {
                    self.finish_bridge_write(connection, after)
                } else {
                    ConnectionTurn::Keep(HttpConnectionPhase::Write(after))
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock || error.kind() == std::io::ErrorKind::Interrupted => ConnectionTurn::Keep(HttpConnectionPhase::Write(after)),
            Err(_) => ConnectionTurn::Terminal(HttpTerminalReason::Io),
        }
    }

    fn finish_bridge_write(&mut self, connection: &mut HttpConnection, after: HttpAfterWrite) -> ConnectionTurn {
        let cursor = connection.bridge_egress.take().expect("bridge write cursor disappeared");
        self.response_credits.release(cursor.credited_bytes());
        match after {
            HttpAfterWrite::WebSocket => ConnectionTurn::Keep(if connection.ingress.is_empty() { HttpConnectionPhase::ReadWebSocket } else { HttpConnectionPhase::ParseWebSocket }),
            HttpAfterWrite::WebSocketClose => ConnectionTurn::Close,
            HttpAfterWrite::Close | HttpAfterWrite::Upgrade => ConnectionTurn::Terminal(HttpTerminalReason::Malformed),
        }
    }

    fn finish_write(&mut self, connection: &mut HttpConnection, after: HttpAfterWrite) -> ConnectionTurn {
        self.response_credits.release(connection.egress.len());
        connection.egress.clear();
        connection.written = 0;
        match after {
            HttpAfterWrite::Close | HttpAfterWrite::WebSocketClose => ConnectionTurn::Close,
            HttpAfterWrite::Upgrade | HttpAfterWrite::WebSocket => {
                if connection.ingress.is_empty() {
                    ConnectionTurn::Keep(HttpConnectionPhase::ReadWebSocket)
                } else {
                    ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket)
                }
            }
        }
    }

    fn parse_one_websocket_frame(&mut self, connection: &mut HttpConnection) -> ConnectionTurn {
        if connection.bridge.inbound.is_none() {
            let frame = match decode_client_websocket_frame(&connection.ingress) {
                Ok(Some(frame)) => frame,
                Ok(None) => {
                    return if connection.ingress.len() >= WEBSOCKET_FRAME_BYTES + 14 { ConnectionTurn::Terminal(HttpTerminalReason::Capacity) } else { ConnectionTurn::Keep(HttpConnectionPhase::ReadWebSocket) };
                }
                Err(reason) => return ConnectionTurn::Terminal(reason),
            };
            if frame.opcode & 0x8 != 0 {
                let mut payload = [0; 125];
                let Some(payload_len) = frame.copy_control_payload(&connection.ingress, &mut payload) else { return ConnectionTurn::Terminal(HttpTerminalReason::Malformed) };
                self.consume_websocket_ingress(connection, frame.consumed);
                return match frame.opcode {
                    0x8 => self.queue_websocket_payload(connection, 0x8, &payload[..payload_len], HttpAfterWrite::WebSocketClose),
                    0x9 => self.queue_websocket_payload(connection, 0xA, &payload[..payload_len], HttpAfterWrite::WebSocket),
                    0xA => ConnectionTurn::Keep(HttpConnectionPhase::DrainBridgeOutbox),
                    _ => ConnectionTurn::Terminal(HttpTerminalReason::Unsupported),
                };
            }
            if frame.opcode != 0x2 {
                return ConnectionTurn::Terminal(HttpTerminalReason::Unsupported);
            }
            connection.bridge.inbound = Some(BridgeInboundCursor { generation: connection.key.generation, phase: BridgeInboundPhase::Decode(crate::bridge::ShellToGatewayDecodeCursor::new(frame.payload_len)), frame });
            return ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket);
        }
        let opening = connection.bridge.opening;
        let Some(inbound) = connection.bridge.inbound.as_mut() else { unreachable!() };
        if inbound.generation != connection.key.generation {
            return ConnectionTurn::Terminal(HttpTerminalReason::Interrupted);
        }
        let message = match &mut inbound.phase {
            BridgeInboundPhase::Decode(decoder) => {
                let frame = inbound.frame;
                match decoder.step(|index| frame.payload_byte(&connection.ingress, index)) {
                    crate::bridge::ShellDecodeStep::Pending => return ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket),
                    crate::bridge::ShellDecodeStep::Fault(crate::bridge::ShellDecodeFault::Capacity) => return ConnectionTurn::Terminal(HttpTerminalReason::Capacity),
                    crate::bridge::ShellDecodeStep::Fault(crate::bridge::ShellDecodeFault::Malformed) => return ConnectionTurn::Terminal(HttpTerminalReason::Malformed),
                    crate::bridge::ShellDecodeStep::Complete(frame) => {
                        let kind = frame.kind();
                        if opening && kind != crate::bridge::ShellFrameKind::Hello {
                            return ConnectionTurn::Terminal(HttpTerminalReason::Malformed);
                        }
                        if !opening && matches!(kind, crate::bridge::ShellFrameKind::Hello | crate::bridge::ShellFrameKind::AppFrames) {
                            return ConnectionTurn::Terminal(HttpTerminalReason::Unsupported);
                        }
                        inbound.phase = BridgeInboundPhase::Materialize(crate::bridge::ShellToGatewayMaterializeCursor::new(frame));
                        return ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket);
                    }
                }
            }
            BridgeInboundPhase::Materialize(materializer) => match materializer.step() {
                crate::bridge::ShellMaterializeStep::Pending => return ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket),
                crate::bridge::ShellMaterializeStep::Fault(crate::bridge::ShellDecodeFault::Capacity) => return ConnectionTurn::Terminal(HttpTerminalReason::Capacity),
                crate::bridge::ShellMaterializeStep::Fault(crate::bridge::ShellDecodeFault::Malformed) => return ConnectionTurn::Terminal(HttpTerminalReason::Malformed),
                crate::bridge::ShellMaterializeStep::Complete(message) => {
                    inbound.phase = BridgeInboundPhase::Ready(Some(message));
                    return ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket);
                }
            },
            BridgeInboundPhase::Ready(message) => message.take().expect("ready bridge message disappeared"),
        };
        let consumed = connection.bridge.inbound.as_ref().expect("bridge inbound cursor disappeared").frame.consumed;
        match message {
            crate::bridge::ShellToGateway::Hello { .. } if connection.bridge.opening => {
                self.consume_websocket_ingress(connection, consumed);
                let (id, outbox) = self.bridge.register();
                connection.bridge.id = Some(id);
                connection.bridge.outbox = Some(outbox);
                connection.bridge.opening = false;
                let welcome = crate::bridge::GatewayToShell::Welcome { bridge_version: crate::bridge::BRIDGE_VERSION, connection: id.to_string(), principal: "agent:local".to_string() };
                self.queue_websocket_payload(connection, 0x2, &welcome.encode(), HttpAfterWrite::WebSocket)
            }
            crate::bridge::ShellToGateway::Ping => {
                self.consume_websocket_ingress(connection, consumed);
                self.queue_websocket_payload(connection, 0x2, &crate::bridge::GatewayToShell::Pong.encode(), HttpAfterWrite::WebSocket)
            }
            crate::bridge::ShellToGateway::Bye => {
                self.consume_websocket_ingress(connection, consumed);
                self.queue_websocket_payload(connection, 0x8, &[], HttpAfterWrite::WebSocketClose)
            }
            message @ (crate::bridge::ShellToGateway::ShellState { .. }
            | crate::bridge::ShellToGateway::ShellStatePatch { .. }
            | crate::bridge::ShellToGateway::Instances { .. }
            | crate::bridge::ShellToGateway::ShellCommandResult { .. }
            | crate::bridge::ShellToGateway::Approval { .. }) => {
                let Some(id) = connection.bridge.id else { return ConnectionTurn::Terminal(HttpTerminalReason::Malformed) };
                self.consume_websocket_ingress(connection, consumed);
                self.bridge.record(id, message);
                ConnectionTurn::Keep(HttpConnectionPhase::DrainBridgeOutbox)
            }
            crate::bridge::ShellToGateway::Hello { .. } | crate::bridge::ShellToGateway::AppFrames { .. } => ConnectionTurn::Terminal(HttpTerminalReason::Unsupported),
        }
    }

    fn consume_websocket_ingress(&mut self, connection: &mut HttpConnection, bytes: usize) {
        connection.bridge.inbound = None;
        connection.ingress.drain(..bytes);
        self.request_credits.release(bytes);
    }

    fn drain_one_bridge_frame(&mut self, connection: &mut HttpConnection) -> ConnectionTurn {
        let Some(outbox) = connection.bridge.outbox.as_mut() else { return ConnectionTurn::Keep(HttpConnectionPhase::ReadWebSocket) };
        match outbox.try_recv_encoded() {
            Some(frame) => self.queue_encoded_websocket_payload(connection, 0x2, frame, HttpAfterWrite::WebSocket),
            None => ConnectionTurn::Keep(HttpConnectionPhase::ReadWebSocket),
        }
    }

    fn queue_encoded_websocket_payload(&mut self, connection: &mut HttpConnection, opcode: u8, frame: crate::bridge::BridgeEncodedLease, after: HttpAfterWrite) -> ConnectionTurn {
        let Some((header, header_len)) = websocket_server_header(opcode, frame.len()) else {
            connection.bridge_egress = Some(BridgeWriteCursor { header: [0; 10], header_len: 0, frame, written: 0, credited: false });
            return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
        };
        let total = header_len + frame.len();
        match self.response_credits.try_acquire(total, frame) {
            Ok(frame) => {
                connection.bridge_egress = Some(BridgeWriteCursor { header, header_len, frame, written: 0, credited: true });
                ConnectionTurn::Keep(HttpConnectionPhase::Write(after))
            }
            Err(frame) => {
                connection.bridge_egress = Some(BridgeWriteCursor { header, header_len, frame, written: 0, credited: false });
                ConnectionTurn::Terminal(HttpTerminalReason::Capacity)
            }
        }
    }

    fn queue_websocket_payload(&mut self, connection: &mut HttpConnection, opcode: u8, payload: &[u8], after: HttpAfterWrite) -> ConnectionTurn {
        let Some(bytes) = encode_server_websocket_frame(opcode, payload) else { return ConnectionTurn::Terminal(HttpTerminalReason::Capacity) };
        if self.response_credits.try_acquire(bytes.len(), ()).is_err() {
            return ConnectionTurn::Terminal(HttpTerminalReason::Capacity);
        }
        connection.egress = bytes;
        connection.written = 0;
        ConnectionTurn::Keep(HttpConnectionPhase::Write(after))
    }
}

enum ConnectionTurn {
    Keep(HttpConnectionPhase),
    Close,
    Terminal(HttpTerminalReason),
}

struct HttpTransportAuthority {
    pool: WorkerPool,
    state: Mutex<HttpTransportState>,
    scheduled: AtomicBool,
    wake_requested: AtomicBool,
    retry_armed: AtomicBool,
    readiness_armed: AtomicBool,
    retry_generation: AtomicU64,
    readiness_generation: AtomicU64,
    retry_job: Mutex<Option<Job>>,
    terminal_job: Mutex<Option<Job>>,
    completion: (Mutex<Option<Result<(), GatewayError>>>, Condvar),
}

impl HttpTransportAuthority {
    fn request_schedule(self: &Arc<Self>) {
        self.wake_requested.store(true, Ordering::Release);
        if self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.wake_requested.store(false, Ordering::Release);
            let weak = Arc::downgrade(self);
            self.submit_exact(Box::new(move || {
                if let Some(authority) = weak.upgrade() {
                    authority.drive_pool_turn();
                }
            }));
        }
    }

    fn submit_exact(self: &Arc<Self>, job: Job) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => self.retain_rejected_job(error.kind(), error.into_job()),
        }
    }

    fn retain_rejected_job(self: &Arc<Self>, kind: WorkerSubmitErrorKind, job: Job) {
        match kind {
            WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated => {
                let mut retained = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if retained.is_none() {
                    *retained = Some(job);
                } else {
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(job);
                    self.finish(Err(GatewayError::new(GatewayErrorCode::Internal, "http transport retry owner already occupied")));
                    return;
                }
                drop(retained);
                self.arm_retry();
            }
            WorkerSubmitErrorKind::Shutdown | WorkerSubmitErrorKind::Poisoned => {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(job);
                self.finish(Err(GatewayError::new(GatewayErrorCode::Internal, format!("http transport worker pool unavailable: {kind:?}"))));
            }
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = self.retry_generation.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        let weak = Arc::downgrade(self);
        self.pool.callback_at(self.pool.now_ms().saturating_add(HTTP_RETRY_MS), move || retry_retained_job(weak, generation));
    }

    fn arm_readiness_poll(self: &Arc<Self>, run_generation: u64) {
        if self.readiness_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let readiness_generation = self.readiness_generation.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        let weak = Arc::downgrade(self);
        self.pool.callback_at(self.pool.now_ms().saturating_add(HTTP_READINESS_POLL_MS), move || {
            let Some(authority) = weak.upgrade() else { return };
            if !generation_is_current(authority.readiness_generation.load(Ordering::Acquire), readiness_generation) {
                return;
            }
            authority.readiness_armed.store(false, Ordering::Release);
            let current = authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).run_generation;
            if generation_is_current(current, run_generation) {
                authority.request_schedule();
            }
        });
    }

    fn clear_readiness(&self) {
        self.readiness_generation.fetch_add(1, Ordering::AcqRel);
        self.readiness_armed.store(false, Ordering::Release);
    }

    fn drive_pool_turn(self: Arc<Self>) {
        let (turn, generation) = {
            let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let generation = state.run_generation;
            (state.drive_one(self.pool.now_ms()), generation)
        };
        self.scheduled.store(false, Ordering::Release);
        if self.wake_requested.swap(false, Ordering::AcqRel) {
            self.request_schedule();
            return;
        }
        match turn {
            HttpTurn::MoreWork => self.request_schedule(),
            HttpTurn::PollReadiness => self.arm_readiness_poll(generation),
            HttpTurn::Parked => {}
            HttpTurn::Complete(result) => self.finish(result),
        }
    }

    fn finish(&self, result: Result<(), GatewayError>) {
        let (lock, wake) = &self.completion;
        let mut completion = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if completion.is_none() {
            *completion = Some(result);
            wake.notify_all();
        }
    }
}

fn retry_retained_job(weak: Weak<HttpTransportAuthority>, generation: u64) {
    let Some(authority) = weak.upgrade() else { return };
    if !generation_is_current(authority.retry_generation.load(Ordering::Acquire), generation) {
        return;
    }
    authority.retry_armed.store(false, Ordering::Release);
    let job = authority.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    if let Some(job) = job {
        authority.submit_exact(job);
    }
}

fn generation_is_current(current: u64, retained: u64) -> bool {
    current == retained
}

struct OwnedHttpResponse {
    bytes: Vec<u8>,
    upgrade: bool,
}

struct CappedBytes {
    bytes: Vec<u8>,
    cap: usize,
    exceeded: bool,
}

impl CappedBytes {
    fn new(cap: usize) -> Self {
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

fn find_crlf_bounded(bytes: &[u8], start: usize, end: usize) -> Option<usize> {
    if start >= end || end > bytes.len() {
        return None;
    }
    bytes.get(start..end)?.windows(2).position(|pair| pair == b"\r\n").map(|offset| start + offset)
}

fn is_http_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte)
}

fn owned_header_matches(head: &ParsedHttpHead, name: &str, expected: &str) -> bool {
    head.header(name).is_some_and(|value| value.split(',').any(|part| part.trim().eq_ignore_ascii_case(expected)))
}

fn owned_bearer_matches(head: &ParsedHttpHead, expected: &str) -> bool {
    let Some(value) = head.header("authorization") else { return false };
    let Some(token) = value.strip_prefix("Bearer ") else { return false };
    constant_time_eq(token.as_bytes(), expected.as_bytes())
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

fn dispatch_owned_http(state: &HttpState, bridge_token: &str, head: &ParsedHttpHead, request: &[u8]) -> Result<OwnedHttpResponse, HttpTerminalReason> {
    let method = head.method.as_deref().ok_or(HttpTerminalReason::Malformed)?;
    let path = head.path.as_deref().ok_or(HttpTerminalReason::Malformed)?;
    let origin = head.header("origin");
    if !origin_allowed(origin, &state.allowed_origins) {
        return build_http_response(403, "Forbidden", Some("text/plain"), b"origin not allowed".to_vec(), &[], false);
    }
    if path.starts_with("/bridge") {
        return dispatch_owned_bridge_handshake(bridge_token, head);
    }
    if path != "/mcp" {
        return build_http_response(404, "Not Found", Some("text/plain"), b"not found".to_vec(), &[], false);
    }
    if !owned_bearer_matches(head, &state.bearer_token) {
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
    let response = state.server.lock().expect("mcp server lock poisoned").dispatch(&request);
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

fn dispatch_owned_bridge_handshake(bridge_token: &str, head: &ParsedHttpHead) -> Result<OwnedHttpResponse, HttpTerminalReason> {
    if head.method.as_deref() != Some("GET") {
        return build_http_response(405, "Method Not Allowed", Some("text/plain"), b"method not allowed".to_vec(), &[], false);
    }
    let path = head.path.as_deref().ok_or(HttpTerminalReason::Malformed)?;
    let provided = path.split_once('?').and_then(|(_, query)| query.split('&').find_map(|part| part.strip_prefix("token="))).unwrap_or_default();
    if !constant_time_eq(provided.as_bytes(), bridge_token.as_bytes()) {
        return build_http_response(401, "Unauthorized", Some("text/plain"), b"invalid bridge token".to_vec(), &[], false);
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
    build_http_response(101, "Switching Protocols", None, Vec::new(), &[("Upgrade", "websocket"), ("Sec-WebSocket-Accept", &accept)], true)
}

fn websocket_accept(key: &str) -> String {
    let mut input = Vec::with_capacity(key.len() + 36);
    input.extend_from_slice(key.as_bytes());
    input.extend_from_slice(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    base64(&sha1(&input))
}

fn websocket_key_nonce(key: &str) -> Option<[u8; 16]> {
    let bytes = key.as_bytes();
    if bytes.len() != 24 || bytes[22..] != *b"==" {
        return None;
    }
    let mut nonce = [0u8; 16];
    for group in 0..5 {
        let index = group * 4;
        let a = base64_value(bytes[index])?;
        let b = base64_value(bytes[index + 1])?;
        let c = base64_value(bytes[index + 2])?;
        let d = base64_value(bytes[index + 3])?;
        nonce[group * 3] = (a << 2) | (b >> 4);
        nonce[group * 3 + 1] = (b << 4) | (c >> 2);
        nonce[group * 3 + 2] = (c << 6) | d;
    }
    let a = base64_value(bytes[20])?;
    let b = base64_value(bytes[21])?;
    if b & 0x0f != 0 {
        return None;
    }
    nonce[15] = (a << 2) | (b >> 4);
    Some(nonce)
}

fn base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn sha1(input: &[u8]) -> [u8; 20] {
    let bit_len = (input.len() as u64) * 8;
    let mut padded = input.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    let mut h = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    for chunk in padded.chunks_exact(64) {
        let mut words = [0u32; 80];
        for (index, word) in words[..16].iter_mut().enumerate() {
            *word = u32::from_be_bytes(chunk[index * 4..index * 4 + 4].try_into().expect("sha1 word width"));
        }
        for index in 16..80 {
            words[index] = (words[index - 3] ^ words[index - 8] ^ words[index - 14] ^ words[index - 16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for (index, word) in words.iter().enumerate() {
            let (f, k) = match index {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let next = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(*word);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = next;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut digest = [0u8; 20];
    for (index, word) in h.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let value = ((chunk[0] as u32) << 16) | ((chunk.get(1).copied().unwrap_or(0) as u32) << 8) | chunk.get(2).copied().unwrap_or(0) as u32;
        output.push(TABLE[((value >> 18) & 0x3f) as usize] as char);
        output.push(TABLE[((value >> 12) & 0x3f) as usize] as char);
        output.push(if chunk.len() > 1 { TABLE[((value >> 6) & 0x3f) as usize] as char } else { '=' });
        output.push(if chunk.len() > 2 { TABLE[(value & 0x3f) as usize] as char } else { '=' });
    }
    output
}

#[derive(Clone, Copy)]
struct ClientWebSocketFrame {
    opcode: u8,
    payload_start: usize,
    payload_len: usize,
    mask: [u8; 4],
    consumed: usize,
}

impl ClientWebSocketFrame {
    fn payload_byte(self, bytes: &[u8], index: usize) -> Option<u8> {
        if index >= self.payload_len {
            return None;
        }
        bytes.get(self.payload_start + index).map(|byte| byte ^ self.mask[index % 4])
    }

    fn copy_control_payload(self, bytes: &[u8], output: &mut [u8; 125]) -> Option<usize> {
        if self.payload_len > output.len() {
            return None;
        }
        for (index, target) in output[..self.payload_len].iter_mut().enumerate() {
            *target = self.payload_byte(bytes, index)?;
        }
        Some(self.payload_len)
    }
}

fn decode_client_websocket_frame(bytes: &[u8]) -> Result<Option<ClientWebSocketFrame>, HttpTerminalReason> {
    if bytes.len() < 2 {
        return Ok(None);
    }
    let fin = bytes[0] & 0x80 != 0;
    let rsv = bytes[0] & 0x70;
    let opcode = bytes[0] & 0x0f;
    let masked = bytes[1] & 0x80 != 0;
    if !fin || rsv != 0 || !masked || !matches!(opcode, 0x2 | 0x8 | 0x9 | 0xA) {
        return Err(HttpTerminalReason::Unsupported);
    }
    let mut cursor = 2;
    let marker = bytes[1] & 0x7f;
    let payload_len = match marker {
        0..=125 => marker as usize,
        126 => {
            if bytes.len() < cursor + 2 {
                return Ok(None);
            }
            let value = u16::from_be_bytes(bytes[cursor..cursor + 2].try_into().expect("websocket u16 width")) as usize;
            cursor += 2;
            value
        }
        _ => {
            if bytes.len() < cursor + 8 {
                return Ok(None);
            }
            let value = u64::from_be_bytes(bytes[cursor..cursor + 8].try_into().expect("websocket u64 width"));
            cursor += 8;
            usize::try_from(value).map_err(|_| HttpTerminalReason::Capacity)?
        }
    };
    if payload_len > WEBSOCKET_FRAME_BYTES || (opcode & 0x8 != 0 && payload_len > 125) {
        return Err(HttpTerminalReason::Capacity);
    }
    if bytes.len() < cursor + 4 {
        return Ok(None);
    }
    let mask: [u8; 4] = bytes[cursor..cursor + 4].try_into().expect("websocket mask width");
    cursor += 4;
    let Some(consumed) = cursor.checked_add(payload_len) else { return Err(HttpTerminalReason::Capacity) };
    if bytes.len() < consumed {
        return Ok(None);
    }
    Ok(Some(ClientWebSocketFrame { opcode, payload_start: cursor, payload_len, mask, consumed }))
}

fn encode_server_websocket_frame(opcode: u8, payload: &[u8]) -> Option<Vec<u8>> {
    let (header, header_bytes) = websocket_server_header(opcode, payload.len())?;
    let total = header_bytes.checked_add(payload.len())?;
    if total > HTTP_RESPONSE_BYTES {
        return None;
    }
    let mut frame = Vec::new();
    frame.try_reserve_exact(total).ok()?;
    frame.extend_from_slice(&header[..header_bytes]);
    frame.extend_from_slice(payload);
    Some(frame)
}

fn websocket_server_header(opcode: u8, payload_len: usize) -> Option<([u8; 10], usize)> {
    if payload_len > WEBSOCKET_FRAME_BYTES || (opcode & 0x8 != 0 && payload_len > 125) {
        return None;
    }
    let mut header = [0u8; 10];
    header[0] = 0x80 | opcode;
    let header_len = if payload_len <= 125 {
        header[1] = payload_len as u8;
        2
    } else if payload_len <= u16::MAX as usize {
        header[1] = 126;
        header[2..4].copy_from_slice(&(payload_len as u16).to_be_bytes());
        4
    } else {
        header[1] = 127;
        header[2..10].copy_from_slice(&(payload_len as u64).to_be_bytes());
        10
    };
    Some((header, header_len))
}

//#region 🔖️OriginAndBearer
fn is_loopback_or_null(origin: &str) -> bool {
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

/// 🔓️ Also used by `🧵️bridge`'s `?token=` comparison (`pub(crate)`) — one constant-time comparison
/// helper, not duplicated.
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
fn bearer_matches(headers: &HeaderMap, expected: &str) -> bool {
    let Some(value) = headers.get(axum::http::header::AUTHORIZATION).and_then(|value| value.to_str().ok()) else { return false };
    let Some(token) = value.strip_prefix("Bearer ") else { return false };
    constant_time_eq(token.as_bytes(), expected.as_bytes())
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
    if bearer_matches(headers, &state.bearer_token) {
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
async fn handle_post(State(state): State<HttpState>, headers: HeaderMap, body: axum::body::Bytes) -> Response {
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

    let mut server = state.server.lock().expect("mcp server lock poisoned");
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
async fn handle_get(State(state): State<HttpState>, headers: HeaderMap) -> Response {
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

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::protocol::{InMemoryPromptRegistry, InMemoryResourceRegistry, InMemoryToolRegistry, NullBackend};
    use crate::workspace::GatewayBackends;
    use std::io::Cursor;

    fn fresh_server() -> McpServer {
        McpServer::new(Box::new(InMemoryToolRegistry::new()), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)))
    }

    //#region 🔖️Stdio
    #[test]
    fn one_request_line_produces_exactly_one_response_line_on_stdout() {
        let input = Cursor::new(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\"}\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(fresh_server()).unwrap();

        let output_text = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_text.lines().collect();
        assert_eq!(lines.len(), 1);
        let response: JsonRpcResponse = serde_json::from_str(lines[0]).unwrap();
        assert!(!response.is_error());
    }

    #[test]
    fn malformed_json_logs_to_the_log_writer_and_never_pollutes_stdout_with_non_json_text() {
        let input = Cursor::new(b"not json at all\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(fresh_server()).unwrap();

        let log_text = String::from_utf8(log).unwrap();
        assert!(log_text.contains("malformed JSON-RPC line rejected"), "diagnostic text must land in the log writer");

        let output_text = String::from_utf8(output).unwrap();
        for line in output_text.lines() {
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(line);
            assert!(parsed.is_ok(), "every stdout line must be valid JSON, got: {line}");
        }
        assert!(!output_text.contains("malformed JSON-RPC line rejected"), "stdout must never carry a diagnostic line");
    }

    #[test]
    fn blank_lines_are_skipped_without_producing_output() {
        let input = Cursor::new(b"\n\n{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\"}\n\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(fresh_server()).unwrap();
        let output_text = String::from_utf8(output).unwrap();
        assert_eq!(output_text.lines().count(), 1);
    }

    #[test]
    fn a_notification_line_produces_no_output_line_at_all() {
        let input = Cursor::new(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\"}\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(fresh_server()).unwrap();
        assert!(output.is_empty());
    }

    #[test]
    fn eof_ends_the_loop_cleanly() {
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        assert!(transport.serve(fresh_server()).is_ok());
        assert!(output.is_empty());
    }
    //#endregion 🔖️Stdio

    //#region 🔖️OriginAndBearerUnit
    #[test]
    fn loopback_and_null_origins_are_always_allowed() {
        assert!(is_loopback_or_null("null"));
        assert!(is_loopback_or_null("http://127.0.0.1:6300"));
        assert!(is_loopback_or_null("http://localhost:6300"));
        assert!(is_loopback_or_null("http://[::1]:6300"));
        assert!(!is_loopback_or_null("https://evil.example"));
    }

    #[test]
    fn origin_allowed_falls_back_to_the_explicit_allowlist() {
        assert!(origin_allowed(None, &[]));
        assert!(!origin_allowed(Some("https://evil.example"), &[]));
        assert!(origin_allowed(Some("https://trusted.example"), &["https://trusted.example".to_string()]));
    }

    #[test]
    fn constant_time_eq_matches_regular_equality() {
        assert!(constant_time_eq(b"secret", b"secret"));
        assert!(!constant_time_eq(b"secret", b"secre1"));
        assert!(!constant_time_eq(b"secret", b"longer-secret"));
    }
    //#endregion 🔖️OriginAndBearerUnit

    //#region 🧵️OwnedHttpAuthority
    #[test]
    fn connection_cap_plus_one_returns_the_exact_owner_without_mutating_fifo() {
        let mut ring = FixedOwnerRing::<u64, HTTP_CONNECTION_CAPACITY>::new();
        for owner in 0..HTTP_CONNECTION_CAPACITY as u64 {
            assert_eq!(ring.try_push(owner), Ok(()));
        }
        assert_eq!(ring.try_push(HTTP_CONNECTION_CAPACITY as u64), Err(HTTP_CONNECTION_CAPACITY as u64));
        for owner in 0..HTTP_CONNECTION_CAPACITY as u64 {
            assert_eq!(ring.pop_front(), Some(owner));
        }
    }

    #[test]
    fn request_and_response_byte_cap_plus_one_return_the_exact_owner() {
        let mut request = FixedByteCredits::new(HTTP_REQUEST_BYTES);
        assert_eq!(request.try_acquire(HTTP_REQUEST_BYTES, "request"), Ok("request"));
        assert_eq!(request.try_acquire(1, "request-plus-one"), Err("request-plus-one"));
        request.release(HTTP_REQUEST_BYTES);
        assert_eq!(request.try_acquire(1, "request-rearmed"), Ok("request-rearmed"));

        let mut response = FixedByteCredits::new(HTTP_RESPONSE_BYTES);
        assert_eq!(response.try_acquire(HTTP_RESPONSE_BYTES, "response"), Ok("response"));
        assert_eq!(response.try_acquire(1, "response-plus-one"), Err("response-plus-one"));
    }

    #[test]
    fn stale_readiness_and_retry_generations_cannot_rearm_an_aba_run() {
        assert!(generation_is_current(7, 7));
        assert!(!generation_is_current(8, 7));
        assert!(!generation_is_current(7, 8));
    }

    #[test]
    fn websocket_handshake_matches_the_rfc6455_accept_vector() {
        assert_eq!(websocket_key_nonce("dGhlIHNhbXBsZSBub25jZQ==").unwrap(), *b"the sample nonce");
        assert_eq!(websocket_accept("dGhlIHNhbXBsZSBub25jZQ=="), "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    }

    #[test]
    fn websocket_key_rejects_duplicate_invalid_alphabet_padding_whitespace_width_and_noncanonical_bits() {
        for invalid in ["", "dGhlIHNhbXBsZSBub25jZQ=", "dGhlIHNhbXBsZSBub25jZQ=A", "dGhlIHNhbXBsZSBub25jZ!==", "dGhlIHNhbXBsZ SBub25jZQ==", "MTIzNDU2Nzg5MDEyMzQ1", "MTIzNDU2Nzg5MDEyMzQ1Njc=", "dGhlIHNhbXBsZSBub25jZR=="] {
            assert!(websocket_key_nonce(invalid).is_none(), "invalid key accepted: {invalid}");
        }
        let head = owned_bridge_head(&["dGhlIHNhbXBsZSBub25jZQ==", "dGhlIHNhbXBsZSBub25jZQ=="]);
        assert!(matches!(dispatch_owned_bridge_handshake("bridge-token", &head), Err(HttpTerminalReason::Malformed)));
    }

    #[test]
    fn masked_websocket_frame_waits_for_partial_payload_then_yields_one_exact_message() {
        let frame = masked_client_frame(0x2, b"hello", true);
        assert!(decode_client_websocket_frame(&frame[..frame.len() - 1]).unwrap().is_none());
        let decoded = decode_client_websocket_frame(&frame).unwrap().unwrap();
        assert_eq!(decoded.opcode, 0x2);
        let payload: Vec<_> = (0..decoded.payload_len).map(|index| decoded.payload_byte(&frame, index).unwrap()).collect();
        assert_eq!(payload, b"hello");
        assert_eq!(decoded.consumed, frame.len());
    }

    #[test]
    fn websocket_rejects_unmasked_fragmented_and_oversize_frames_before_consumption() {
        let unmasked = encode_server_websocket_frame(0x2, b"hello").unwrap();
        assert!(matches!(decode_client_websocket_frame(&unmasked), Err(HttpTerminalReason::Unsupported)));
        let fragmented = masked_client_frame(0x2, b"hello", false);
        assert!(matches!(decode_client_websocket_frame(&fragmented), Err(HttpTerminalReason::Unsupported)));
        let mut oversize = vec![0x82, 0xff];
        oversize.extend_from_slice(&((WEBSOCKET_FRAME_BYTES as u64) + 1).to_be_bytes());
        assert!(matches!(decode_client_websocket_frame(&oversize), Err(HttpTerminalReason::Capacity)));
    }

    #[test]
    fn websocket_control_and_close_frames_stay_one_bounded_message() {
        let ping = masked_client_frame(0x9, b"p", true);
        assert_eq!(decode_client_websocket_frame(&ping).unwrap().unwrap().opcode, 0x9);
        let close = masked_client_frame(0x8, &[], true);
        assert_eq!(decode_client_websocket_frame(&close).unwrap().unwrap().opcode, 0x8);
        assert!(encode_server_websocket_frame(0x9, &[0; 126]).is_none());
    }

    #[test]
    fn one_terminal_close_grant_drains_exactly_one_fifo_owner() {
        let mut terminal = FixedOwnerRing::<u64, 3>::new();
        terminal.try_push(11).unwrap();
        terminal.try_push(12).unwrap();
        terminal.try_push(13).unwrap();
        assert_eq!(terminal.pop_front(), Some(11));
        assert_eq!(terminal.len(), 2);
        assert_eq!(terminal.pop_front(), Some(12));
    }

    #[test]
    fn partial_http_read_and_parser_turn_advance_one_page_or_token_per_grant() {
        let (mut state, mut peer) = state_with_connection();
        peer.write_all(b"POST /mcp HTTP/1.1\r\nAuthorization: Bearer test-token\r\n").unwrap();
        assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
        let connection = state.connections[0].as_ref().unwrap();
        assert!(connection.ingress.len() <= HTTP_IO_PAGE_BYTES);
        assert_eq!(connection.parser.cursor, 0);
        assert!(matches!(state.drive_one(2), HttpTurn::MoreWork));
        let connection = state.connections[0].as_ref().unwrap();
        assert_eq!(connection.parser.method.as_deref(), Some("POST"));
        assert_eq!(connection.parser.header_count, 0, "one parse grant consumes only the request-line token");
    }

    #[test]
    fn request_line_and_header_delimiter_search_faults_at_cap_without_scanning_late_crlf() {
        let mut request_line = vec![b'a'; HTTP_REQUEST_LINE_BYTES + 2];
        request_line[HTTP_REQUEST_LINE_BYTES] = b'\r';
        request_line[HTTP_REQUEST_LINE_BYTES + 1] = b'\n';
        assert_eq!(find_crlf_bounded(&request_line, 0, HTTP_REQUEST_LINE_BYTES), None);
        let (mut state, _peer) = state_with_connection();
        state.connections[0].as_mut().unwrap().ingress = request_line;
        assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
        assert_eq!(state.terminal.len(), 1);

        let mut header = vec![b'a'; HTTP_HEADER_BYTES + 2];
        header[HTTP_HEADER_BYTES] = b'\r';
        header[HTTP_HEADER_BYTES + 1] = b'\n';
        assert_eq!(find_crlf_bounded(&header, 20, HTTP_HEADER_BYTES), None);
    }

    #[test]
    fn malformed_and_unsupported_open_bridge_binary_frames_retain_exact_raw_ingress() {
        for (payload, reason) in [
            (vec![99], HttpTerminalReason::Malformed),
            (
                crate::bridge::ShellToGateway::Hello { bridge_version: crate::bridge::BRIDGE_VERSION, shell_kind: crate::bridge::ShellKind::React, shell_session_id: "s".into(), principal_actor: "p".into(), flags: crate::bridge::BridgeFlags::NONE }
                    .encode(),
                HttpTerminalReason::Unsupported,
            ),
        ] {
            let (mut state, _peer) = state_with_connection();
            let (id, outbox) = state.bridge.register();
            let raw = masked_client_frame(0x2, &payload, true);
            state.request_credits.try_acquire(raw.len(), ()).unwrap();
            let connection = state.connections[0].as_mut().unwrap();
            connection.bridge = BridgeSession { id: Some(id), outbox: Some(outbox), opening: false, inbound: None };
            connection.phase = HttpConnectionPhase::ParseWebSocket;
            connection.ingress = raw.clone();
            for grant in 1..128 {
                assert!(matches!(state.drive_one(grant), HttpTurn::MoreWork));
                if state.terminal.len() != 0 {
                    break;
                }
                assert_eq!(state.connections[0].as_ref().unwrap().ingress, raw, "incremental validation must retain the exact raw masked owner");
            }
            let owner = state.terminal.pop_front().expect("protocol fault must retain terminal owner");
            assert_eq!(owner.reason(), reason);
            assert_eq!(owner.ingress(), raw);
            owner.close();
        }
    }

    #[test]
    fn incremental_bridge_decode_cancellation_and_stale_generation_retain_exact_raw_owner() {
        let (mut state, _peer) = state_with_connection();
        let (id, outbox) = state.bridge.register();
        let payload = crate::bridge::ShellToGateway::ShellState { revision: 1, state: vec![4; HTTP_IO_PAGE_BYTES + 1] }.encode();
        let raw = masked_client_frame(0x2, &payload, true);
        state.request_credits.try_acquire(raw.len(), ()).unwrap();
        let connection = state.connections[0].as_mut().unwrap();
        connection.bridge = BridgeSession { id: Some(id), outbox: Some(outbox), opening: false, inbound: None };
        connection.phase = HttpConnectionPhase::ParseWebSocket;
        connection.ingress = raw.clone();
        assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
        assert!(matches!(state.drive_one(2), HttpTurn::MoreWork));
        let connection = state.connections[0].as_mut().unwrap();
        assert!(connection.bridge.inbound.is_some());
        connection.key.generation = connection.key.generation.wrapping_add(1);
        assert!(matches!(state.drive_one(3), HttpTurn::MoreWork));
        let owner = state.terminal.pop_front().unwrap();
        assert_eq!(owner.reason(), HttpTerminalReason::Interrupted);
        assert_eq!(owner.ingress(), raw);
        owner.close();

        let (mut cancelled, _peer) = state_with_connection();
        let (id, outbox) = cancelled.bridge.register();
        cancelled.request_credits.try_acquire(raw.len(), ()).unwrap();
        let connection = cancelled.connections[0].as_mut().unwrap();
        connection.bridge = BridgeSession { id: Some(id), outbox: Some(outbox), opening: false, inbound: None };
        connection.phase = HttpConnectionPhase::ParseWebSocket;
        connection.ingress = raw.clone();
        assert!(matches!(cancelled.drive_one(1), HttpTurn::MoreWork));
        assert!(matches!(cancelled.drive_one(2), HttpTurn::MoreWork));
        cancelled.mode = HttpTransportMode::Closing;
        assert!(matches!(cancelled.drive_one(3), HttpTurn::MoreWork));
        let owner = cancelled.terminal.pop_front().unwrap();
        assert_eq!(owner.reason(), HttpTerminalReason::Cancelled);
        assert_eq!(owner.ingress(), raw);
        owner.close();
    }

    #[test]
    fn slowloris_deadline_terminalizes_once_and_parks_for_public_retrieval() {
        let (mut state, _peer) = state_with_connection();
        state.connections[0].as_mut().unwrap().header_deadline_ms = 5;
        assert!(matches!(state.drive_one(5), HttpTurn::MoreWork));
        assert_eq!(state.terminal.len(), 1);
        assert!(state.connections[0].is_none());
        let _ = state.drive_one(6);
        assert_eq!(state.terminal.len(), 1);
        state.terminal.pop_front().unwrap().close();
    }

    #[test]
    fn cancellation_and_shutdown_drain_one_connection_authority_per_grant() {
        let (mut state, _peer) = state_with_connection();
        state.mode = HttpTransportMode::Closing;
        state.terminal_policy = HttpTerminalPolicy::Close;
        state.run_generation = 2;
        assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
        assert_eq!(state.terminal.len(), 1);
        assert!(matches!(state.drive_one(2), HttpTurn::MoreWork));
        assert_eq!(state.terminal.len(), 0);
        assert!(matches!(state.drive_one(3), HttpTurn::Complete(Ok(()))));
    }

    #[test]
    fn terminal_public_fifo_preserves_generation_aba_and_process_close_is_one_owner_per_grant() {
        let (mut state, _peer) = state_with_connection();
        let first = state.connections[0].take().unwrap();
        let mut second = replacement_connection(&state, 0, 2);
        second.ingress.extend_from_slice(b"second");
        state.terminalize(first, HttpTerminalReason::Malformed);
        state.terminalize(second, HttpTerminalReason::Unsupported);
        assert_eq!(state.terminal.len(), 2);
        let first = state.terminal.pop_front().unwrap();
        let second = state.terminal.pop_front().unwrap();
        assert_eq!(first.key(), HttpConnectionKey { slot: 0, generation: 1 });
        assert_eq!(second.key(), HttpConnectionKey { slot: 0, generation: 2 });
        assert_eq!(second.ingress(), b"second");
        first.close();
        second.close();

        let first = replacement_connection(&state, 0, 3);
        let second = replacement_connection(&state, 1, 4);
        state.terminalize(first, HttpTerminalReason::Malformed);
        state.terminalize(second, HttpTerminalReason::Malformed);
        state.terminal_policy = HttpTerminalPolicy::Close;
        assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
        assert_eq!(state.terminal.len(), 1);
        assert!(matches!(state.drive_one(2), HttpTurn::MoreWork));
        assert_eq!(state.terminal.len(), 0);
    }

    #[test]
    fn partial_response_write_never_exceeds_one_io_page_per_grant() {
        let (mut state, mut peer) = state_with_connection();
        peer.set_nonblocking(true).unwrap();
        let connection = state.connections[0].as_mut().unwrap();
        connection.egress = vec![7; HTTP_IO_PAGE_BYTES * 2];
        connection.phase = HttpConnectionPhase::Write(HttpAfterWrite::Close);
        state.response_credits.try_acquire(connection.egress.len(), ()).unwrap();
        assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
        let written = state.connections[0].as_ref().map_or(HTTP_IO_PAGE_BYTES, |connection| connection.written);
        assert!(written <= HTTP_IO_PAGE_BYTES);
        let mut page = vec![0; HTTP_IO_PAGE_BYTES];
        let _ = peer.read(&mut page);
    }

    fn state_with_connection() -> (HttpTransportState, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let peer = TcpStream::connect(address).unwrap();
        let (stream, remote) = listener.accept().unwrap();
        stream.set_nonblocking(true).unwrap();
        listener.set_nonblocking(true).unwrap();
        let options = HttpTransportOptions::new("test-token", "bridge-token").bind_addr(address);
        let events = Arc::new(Mutex::new(EventLog::default()));
        let bridge = crate::bridge::BridgeHandle::new();
        let mut state = HttpTransportState::new(listener, fresh_server(), &options, events, bridge);
        state.io_cursor = 1;
        state.connections[0] = Some(HttpConnection {
            key: HttpConnectionKey { slot: 0, generation: 1 },
            peer: remote,
            stream,
            ingress: Vec::with_capacity(HTTP_REQUEST_BYTES),
            egress: Vec::with_capacity(HTTP_RESPONSE_BYTES),
            bridge_egress: None,
            written: 0,
            parser: ParsedHttpHead::new(),
            phase: HttpConnectionPhase::ReadHttp,
            bridge: BridgeSession { id: None, outbox: None, opening: false, inbound: None },
            last_progress_ms: 0,
            header_deadline_ms: HTTP_SLOWLORIS_MS,
        });
        (state, peer)
    }

    fn replacement_connection(state: &HttpTransportState, slot: u16, generation: u64) -> HttpConnection {
        let address = state.listener.as_ref().unwrap().local_addr().unwrap();
        let _peer = TcpStream::connect(address).unwrap();
        let (stream, remote) = state.listener.as_ref().unwrap().accept().unwrap();
        stream.set_nonblocking(true).unwrap();
        HttpConnection {
            key: HttpConnectionKey { slot, generation },
            peer: remote,
            stream,
            ingress: Vec::with_capacity(HTTP_REQUEST_BYTES),
            egress: Vec::with_capacity(HTTP_RESPONSE_BYTES),
            bridge_egress: None,
            written: 0,
            parser: ParsedHttpHead::new(),
            phase: HttpConnectionPhase::ReadHttp,
            bridge: BridgeSession { id: None, outbox: None, opening: false, inbound: None },
            last_progress_ms: 0,
            header_deadline_ms: HTTP_SLOWLORIS_MS,
        }
    }

    fn owned_bridge_head(keys: &[&str]) -> ParsedHttpHead {
        let mut head = ParsedHttpHead::new();
        head.method = Some("GET".into());
        head.path = Some("/bridge?token=bridge-token".into());
        for (name, value) in [("upgrade", "websocket"), ("connection", "Upgrade"), ("sec-websocket-version", "13")].into_iter().chain(keys.iter().copied().map(|value| ("sec-websocket-key", value))) {
            head.headers[head.header_count] = Some(OwnedHeader { name: name.into(), value: value.into() });
            head.header_count += 1;
        }
        head
    }

    pub(super) fn masked_client_frame(opcode: u8, payload: &[u8], fin: bool) -> Vec<u8> {
        assert!(payload.len() <= 125);
        let mask = [1u8, 2, 3, 4];
        let mut frame = vec![(if fin { 0x80 } else { 0 }) | opcode, 0x80 | payload.len() as u8];
        frame.extend_from_slice(&mask);
        frame.extend(payload.iter().enumerate().map(|(index, byte)| byte ^ mask[index % 4]));
        frame
    }
    //#endregion 🧵️OwnedHttpAuthority
}

/// 🌐️ HTTP handler integration tests use the owned in-process request driver; websocket upgrade
/// coverage below continues to bind the real `axum::Router` on an ephemeral socket.
#[cfg(test)]
mod long {
    use super::*;
    use crate::protocol::{InMemoryPromptRegistry, InMemoryResourceRegistry, InMemoryToolRegistry, McpServer, NullBackend, Tool, META_PROTOCOL_VERSION_KEY};
    use crate::workspace::GatewayBackends;
    use axum::body::Body;
    use axum::http::Request;

    #[derive(Clone)]
    struct HttpTestDriver {
        state: HttpState,
    }

    impl HttpTestDriver {
        async fn request(&self, request: Request<Body>) -> Response {
            let (parts, body) = request.into_parts();
            match (parts.method, parts.uri.path()) {
                (axum::http::Method::POST, "/mcp") => match axum::body::to_bytes(body, usize::MAX).await {
                    Ok(body) => handle_post(State(self.state.clone()), parts.headers, body).await,
                    Err(error) => (StatusCode::BAD_REQUEST, error.to_string()).into_response(),
                },
                (axum::http::Method::GET, "/mcp") => handle_get(State(self.state.clone()), parts.headers).await,
                (_, "/mcp") => StatusCode::METHOD_NOT_ALLOWED.into_response(),
                _ => StatusCode::NOT_FOUND.into_response(),
            }
        }
    }

    fn fresh_server() -> McpServer {
        McpServer::new(Box::new(InMemoryToolRegistry::new()), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)))
    }

    fn transport() -> HttpTransport {
        HttpTransport::new(HttpTransportOptions::new("test-token", "bridge-token"))
    }

    fn test_driver(server: McpServer) -> (HttpTestDriver, HttpEventPublisher) {
        let transport = transport();
        let events = Arc::new(Mutex::new(EventLog::default()));
        let state = HttpState { server: Arc::new(Mutex::new(server)), bearer_token: Arc::from(transport.options.bearer_token.as_str()), allowed_origins: Arc::new(transport.options.allowed_origins.clone()), events: events.clone() };
        (HttpTestDriver { state }, HttpEventPublisher { events })
    }

    fn post_request(body: serde_json::Value, headers: &[(&str, &str)]) -> Request<Body> {
        let mut builder = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer test-token");
        for (name, value) in headers {
            builder = builder.header(*name, *value);
        }
        builder.body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap()
    }

    async fn one_shot(driver: HttpTestDriver, request: Request<Body>) -> Response {
        driver.request(request).await
    }

    fn owned_head(method: &str, path: &str, headers: &[(&str, &str)], body_len: usize) -> ParsedHttpHead {
        let mut head = ParsedHttpHead::new();
        head.method = Some(method.to_string());
        head.path = Some(path.to_string());
        head.version = Some("HTTP/1.1".to_string());
        head.header_end = Some(0);
        head.content_length = body_len;
        for (name, value) in headers {
            head.headers[head.header_count] = Some(OwnedHeader { name: name.to_ascii_lowercase(), value: value.to_string() });
            head.header_count += 1;
        }
        head
    }

    fn owned_response_body(response: &OwnedHttpResponse) -> &[u8] {
        let split = response.bytes.windows(4).position(|window| window == b"\r\n\r\n").expect("owned response header terminator");
        &response.bytes[split + 4..]
    }

    #[tokio::test]
    async fn owned_health_dispatch_matches_the_current_axum_adapter() {
        let body = serde_json::to_vec(&serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" })).unwrap();
        let (owned_state, _) = test_driver(fresh_server());
        let head = owned_head("POST", "/mcp", &[("authorization", "Bearer test-token")], body.len());
        let owned = dispatch_owned_http(&owned_state.state, "bridge-token", &head, &body).unwrap();
        assert!(owned.bytes.starts_with(b"HTTP/1.1 200 OK\r\n"));

        let (axum_driver, _) = test_driver(fresh_server());
        let axum = one_shot(axum_driver, post_request(serde_json::from_slice(&body).unwrap(), &[])).await;
        assert_eq!(axum.status(), StatusCode::OK);
        let axum_body = axum::body::to_bytes(axum.into_body(), HTTP_RESPONSE_BYTES).await.unwrap();
        let owned_json: serde_json::Value = serde_json::from_slice(owned_response_body(&owned)).unwrap();
        let axum_json: serde_json::Value = serde_json::from_slice(&axum_body).unwrap();
        assert_eq!(owned_json, axum_json);
    }

    #[test]
    fn owned_bridge_handshake_matches_current_rfc_message_close_and_error_contracts() {
        let head = owned_head("GET", "/bridge?token=bridge-token", &[("connection", "Upgrade"), ("upgrade", "websocket"), ("sec-websocket-version", "13"), ("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==")], 0);
        let response = dispatch_owned_bridge_handshake("bridge-token", &head).unwrap();
        let text = String::from_utf8(response.bytes).unwrap();
        assert!(text.starts_with("HTTP/1.1 101 Switching Protocols\r\n"));
        assert!(text.contains("Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo="));

        let hello = super::quick::masked_client_frame(
            0x2,
            &crate::bridge::ShellToGateway::Hello {
                bridge_version: crate::bridge::BRIDGE_VERSION,
                shell_kind: crate::bridge::ShellKind::React,
                shell_session_id: "owned".into(),
                principal_actor: "agent:local".into(),
                flags: crate::bridge::BridgeFlags::NONE,
            }
            .encode(),
            true,
        );
        assert!(matches!(decode_client_websocket_frame(&hello).unwrap().unwrap().opcode, 0x2));
        let close = super::quick::masked_client_frame(0x8, &[], true);
        assert_eq!(decode_client_websocket_frame(&close).unwrap().unwrap().opcode, 0x8);
        let text_frame = super::quick::masked_client_frame(0x1, b"unsupported", true);
        assert!(matches!(decode_client_websocket_frame(&text_frame), Err(HttpTerminalReason::Unsupported)));
    }

    //#region 🔖️PostModern
    #[tokio::test]
    async fn modern_tools_list_over_http_returns_200_with_the_json_rpc_result() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
        let request = post_request(body, &[("MCP-Protocol-Version", "2026-07-28")]);
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["result"]["resultType"], "complete");
    }

    #[tokio::test]
    async fn legacy_initialize_over_http_returns_200_and_negotiates_legacy() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": { "name": "t", "version": "0" } } });
        let request = post_request(body, &[]);
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["result"]["protocolVersion"], "2025-11-25");
    }

    #[tokio::test]
    async fn a_notification_over_http_is_202_accepted_with_no_body() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "method": "notifications/cancelled" });
        let request = post_request(body, &[]);
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(bytes.is_empty());
    }
    //#endregion 🔖️PostModern

    //#region 🔖️ProtocolVersionHeader
    #[tokio::test]
    async fn missing_protocol_version_header_on_a_modern_request_is_400_header_mismatch() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
        let request = post_request(body, &[]);
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["error"]["code"], HEADER_MISMATCH);
    }

    #[tokio::test]
    async fn mismatched_protocol_version_header_and_body_is_400_header_mismatch() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
        let request = post_request(body, &[("MCP-Protocol-Version", "2025-11-25")]);
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["error"]["code"], HEADER_MISMATCH);
    }

    #[tokio::test]
    async fn unsupported_protocol_version_is_400_with_the_supported_list() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": { "_meta": { META_PROTOCOL_VERSION_KEY: "1999-01-01" } } });
        let request = post_request(body, &[("MCP-Protocol-Version", "1999-01-01")]);
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["error"]["code"], UNSUPPORTED_PROTOCOL_VERSION);
        assert_eq!(value["error"]["data"]["supported"], serde_json::json!(SUPPORTED_PROTOCOL_VERSIONS));
    }
    //#endregion 🔖️ProtocolVersionHeader

    //#region 🔖️Security
    #[tokio::test]
    async fn an_evil_origin_is_rejected_with_403() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
        let request =
            Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer test-token").header("origin", "https://evil.example").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn a_loopback_origin_is_accepted() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
        let request =
            Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer test-token").header("origin", "http://127.0.0.1:6300").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn missing_bearer_token_is_401() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
        let request = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn incorrect_bearer_token_is_401() {
        let (router, _events) = test_driver(fresh_server());
        let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
        let request = Request::builder().method("POST").uri("/mcp").header("content-type", "application/json").header("authorization", "Bearer wrong-token").body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap();
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    //#endregion 🔖️Security

    //#region 🔖️BothErasOverHttp
    #[tokio::test]
    async fn both_eras_are_served_over_the_same_http_endpoint_by_the_same_server() {
        let mut tools = InMemoryToolRegistry::new();
        tools.register(Tool::new("ping_tool", serde_json::json!({"type":"object"})), |_arguments| crate::protocol::CallToolResult::ok(vec![], None)).unwrap();
        let server = McpServer::new(Box::new(tools), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)));
        let (router, _events) = test_driver(server);

        let modern_body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "ping_tool", "arguments": {}, "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } } });
        let modern_request = post_request(modern_body, &[("MCP-Protocol-Version", "2026-07-28")]);
        let modern_response = one_shot(router.clone(), modern_request).await;
        assert_eq!(modern_response.status(), StatusCode::OK);

        let legacy_body = serde_json::json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": { "name": "ping_tool", "arguments": {} } });
        let legacy_request = post_request(legacy_body, &[]);
        let legacy_response = one_shot(router, legacy_request).await;
        assert_eq!(legacy_response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(legacy_response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["result"]["isError"], false);
    }
    //#endregion 🔖️BothErasOverHttp

    //#region 🔖️GetSseResumption
    #[tokio::test]
    async fn get_with_no_last_event_id_replays_every_buffered_notification() {
        let (router, events) = test_driver(fresh_server());
        events.push(JsonRpcNotification::new("notifications/tools/list_changed", None));
        events.push(JsonRpcNotification::new("notifications/resources/list_changed", None));

        let request = Request::builder().method("GET").uri("/mcp").header("authorization", "Bearer test-token").body(Body::empty()).unwrap();
        let response = one_shot(router, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get(axum::http::header::CONTENT_TYPE).unwrap(), "text/event-stream");
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(text.contains("id: 1"));
        assert!(text.contains("id: 2"));
        assert!(text.contains("notifications/tools/list_changed"));
    }

    #[tokio::test]
    async fn get_with_last_event_id_resumes_after_that_id_only() {
        let (router, events) = test_driver(fresh_server());
        events.push(JsonRpcNotification::new("notifications/tools/list_changed", None));
        let second_id = events.push(JsonRpcNotification::new("notifications/resources/list_changed", None));
        let _ = second_id;

        let request = Request::builder().method("GET").uri("/mcp").header("authorization", "Bearer test-token").header("Last-Event-ID", "1").body(Body::empty()).unwrap();
        let response = one_shot(router, request).await;
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(!text.contains("tools/list_changed"), "must not replay an event at-or-before Last-Event-ID");
        assert!(text.contains("resources/list_changed"), "must replay events after Last-Event-ID");
    }
    //#endregion 🔖️GetSseResumption

    //#region 🔖️BridgeOnTheMergedApp
    /// 🌉️ P1c acceptance: `/bridge` is mounted on the SAME app `/mcp` lives on — this test binds a
    /// real ephemeral socket to the ACTUAL `HttpTransport::router()` output (not a bridge-only
    /// router), connects a real `tokio-tungstenite` client, and drives the full scenario
    /// `📓️sol-P1c-packet.md`'s acceptance list names: `Hello`→`Welcome`, a `ShellState` publish, a
    /// pushed `ShellCommand` answered by a `ShellCommandResult`, a wrong-token rejection, and a bad-
    /// `Origin` rejection — all in one foreground `#[tokio::test]`, no background process left running.
    #[tokio::test]
    async fn bridge_is_live_on_the_same_merged_app_run_http_builds() {
        use crate::bridge::{BridgeFlags, ShellKind, ShellToGateway, BRIDGE_VERSION};
        use futures::{SinkExt, StreamExt};
        use tokio_tungstenite::tungstenite::client::IntoClientRequest;
        use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

        let transport = HttpTransport::new(HttpTransportOptions::new("mcp-bearer", "bridge-secret"));
        let (router, _events, bridge_handle) = transport.router(fresh_server());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server_task = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        // /mcp still answers on the SAME bound socket the bridge is about to connect to — proves
        // both endpoints really are one merged app, not two separate servers on the same port by
        // coincidence.
        let mcp_client = reqwest_free_post(addr, "/mcp", "Bearer mcp-bearer", &serde_json::json!({"jsonrpc":"2.0","id":1,"method":"ping"})).await;
        assert_eq!(mcp_client, 200);

        // Hello -> Welcome.
        let (mut socket, _response) = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge?token=bridge-secret")).await.expect("client connects with the correct bridge token");
        let hello = ShellToGateway::Hello { bridge_version: BRIDGE_VERSION, shell_kind: ShellKind::React, shell_session_id: "shell-live".into(), principal_actor: "agent:local".into(), flags: BridgeFlags::NONE };
        socket.send(TungsteniteMessage::Binary(hello.encode().into())).await.unwrap();
        let welcome_bytes = match socket.next().await.unwrap().unwrap() {
            TungsteniteMessage::Binary(bytes) => bytes,
            other => panic!("expected a binary Welcome frame, got {other:?}"),
        };
        assert!(matches!(crate::bridge::GatewayToShell::decode(&welcome_bytes).unwrap(), crate::bridge::GatewayToShell::Welcome { .. }));

        // ShellState publish becomes visible through BridgeHandle.
        let id = bridge_handle.connections().first().copied().expect("one live connection");
        let state_frame = ShellToGateway::ShellState { revision: 1, state: vec![7] };
        socket.send(TungsteniteMessage::Binary(state_frame.clone().encode().into())).await.unwrap();
        socket.send(TungsteniteMessage::Binary(ShellToGateway::Ping.encode().into())).await.unwrap();
        let _pong = socket.next().await.unwrap().unwrap();
        assert_eq!(bridge_handle.last_shell_state(id), Some(state_frame));

        // A pushed ShellCommand reaches the client, whose ShellCommandResult becomes visible.
        let pushed = crate::bridge::GatewayToShell::ShellCommand { seq: 1, command: vec![9] };
        assert!(bridge_handle.send_to(id, pushed.clone()));
        let received_bytes = match socket.next().await.unwrap().unwrap() {
            TungsteniteMessage::Binary(bytes) => bytes,
            other => panic!("expected a binary ShellCommand frame, got {other:?}"),
        };
        assert_eq!(crate::bridge::GatewayToShell::decode(&received_bytes).unwrap(), pushed);
        let result_frame = ShellToGateway::ShellCommandResult { in_reply_to: 1, ok: true, fault: None };
        socket.send(TungsteniteMessage::Binary(result_frame.encode().into())).await.unwrap();
        socket.send(TungsteniteMessage::Binary(ShellToGateway::Ping.encode().into())).await.unwrap();
        let _pong2 = socket.next().await.unwrap().unwrap();
        assert_eq!(bridge_handle.last_command_result(id), Some((1, true, None)));
        drop(socket);

        // Wrong token is rejected before the upgrade completes.
        let wrong_token = tokio_tungstenite::connect_async(format!("ws://{addr}/bridge?token=nope")).await;
        assert!(wrong_token.is_err(), "a mismatched bridge token must never complete the websocket handshake");

        // Bad Origin is rejected before the upgrade completes.
        let mut bad_origin_request = format!("ws://{addr}/bridge?token=bridge-secret").into_client_request().unwrap();
        bad_origin_request.headers_mut().insert("origin", "https://evil.example".parse().unwrap());
        let bad_origin = tokio_tungstenite::connect_async(bad_origin_request).await;
        assert!(bad_origin.is_err(), "a non-loopback Origin must never complete the websocket handshake");

        server_task.abort();
    }

    /// 🧰️ The smallest possible real HTTP/1.1 POST over a bound TCP socket — used ONLY by the
    /// merged-app test above, which already needs a real listener for the websocket half; avoids
    /// adding a `reqwest` dependency for one confirming assertion by speaking just enough HTTP/1.1 by
    /// hand (`Connection: close`, read to EOF, parse the status line) for a single fire-and-forget
    /// request/response.
    async fn reqwest_free_post(addr: SocketAddr, path: &str, bearer: &str, body: &serde_json::Value) -> u16 {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let body_bytes = serde_json::to_vec(body).unwrap();
        let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        let request = format!("POST {path} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: {bearer}\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n", body_bytes.len());
        stream.write_all(request.as_bytes()).await.unwrap();
        stream.write_all(&body_bytes).await.unwrap();
        let mut response = Vec::new();
        stream.read_to_end(&mut response).await.unwrap();
        let response_text = String::from_utf8_lossy(&response);
        let status_line = response_text.lines().next().unwrap_or("");
        status_line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0)
    }
    //#endregion 🔖️BridgeOnTheMergedApp
}
//#endregion 🧪️Tests
