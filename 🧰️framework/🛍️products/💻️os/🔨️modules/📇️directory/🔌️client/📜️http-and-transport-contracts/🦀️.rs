//! 📜️ HTTP and transport contracts of the directory client: the document HTTP port topic and caps, the installed service records, the document HTTP port records, the HTTP method and response, the transport errors, the platform marker traits and the directory transport and WS connection traits. See [`super`].

use super::*;

/// 🧩 Manifest topic carrying owner-authored document HTTP schemas and routes.
pub const DOCUMENT_HTTP_PORT_TOPIC: &str = "semio.os.document-http-port/v1";

pub const DOCUMENT_HTTP_REQUEST_MAX_BYTES: usize = 64 * 1024;

pub const DOCUMENT_HTTP_RESPONSE_MAX_BYTES: usize = 64 * 1024;

/// 🎯 One finite owner-authored service step over the installed document HTTP declaration.
pub enum InstalledServiceTurnV1 {
    Idle,
    WaitUntil(u64),
    Call { action: String, payload: DslValue },
    Terminal,
}

/// 🧩 Owner behavior receives opaque declared answers while the host retains transport authority.
pub trait InstalledServiceDriverV1: Send {
    fn intend(&mut self, action: &str, payload: DslValue) -> Result<(), DocumentHttpPortCodeV1>;
    fn turn(&mut self, now_ms: u64) -> InstalledServiceTurnV1;
    fn complete(&mut self, action: &str, request: &DslValue, result: Result<DslValue, DocumentHttpPortCodeV1>);
    fn status(&self) -> DslValue;
    fn terminal(&self) -> bool;
}

/// 📦 Explicit native composition supplied by the executable that installs the owner.
#[derive(Clone, Copy)]
pub struct InstalledServiceContributionV1 {
    pub owner: &'static str,
    pub service_id: &'static str,
    pub create: fn(DocumentScope, bool) -> Box<dyn InstalledServiceDriverV1>,
}

/// 📣 Neutral ephemeral status; its payload remains owned by the installed service.
#[derive(Clone, Debug, PartialEq)]
pub struct InstalledServiceStatusV1 {
    pub owner: String,
    pub service_id: String,
    pub payload: DslValue,
}

/// 🛡 Closed transport outcomes without server body disclosure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentHttpPortCodeV1 {
    Invalid, Bounds, Cancelled, Denied, Transport, NotFound, Conflict, Gone, Capacity, Unavailable,
}

impl DocumentHttpPortCodeV1 {
    pub fn from_status(status: u16) -> Self {
        match status { 400 => Self::Invalid, 429 => Self::Capacity, 401 | 403 => Self::Denied, 404 => Self::NotFound, 409 => Self::Conflict, 410 => Self::Gone, 413 => Self::Bounds, 503 => Self::Unavailable, _ => Self::Transport }
    }
}

/// 📜 Closed owner declaration transported by the existing plugin manifest contribution.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentHttpPortDeclarationV1 {
    pub schema: String,
    pub owner: String,
    pub service_id: String,
    pub operations: Vec<DocumentHttpOperationV1>,
}

/// 📖 One named operation and its published request/response schemas.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentHttpOperationV1 {
    pub action: String,
    pub method: String,
    pub route: Vec<String>,
    pub send_body: bool,
    pub cursor_field: Option<String>,
    pub request_max_bytes: usize,
    pub response_max_bytes: usize,
    pub input_schema: String,
    pub output_schema: String,
}

/// 📦 An admitted request; path segments are encoded independently inside the exact document scope.
pub struct DocumentHttpRequestV1 {
    pub method: HttpMethod,
    pub segments: Vec<String>,
    pub after: Option<u64>,
    pub body: Option<Vec<u8>>,
    pub request_max_bytes: usize,
    pub response_max_bytes: usize,
}

//#region 🔖️Transport
/// 📨️ One HTTP verb `DirectoryClient` issues against the hub REST surface: queries are `GET`,
/// commands are `POST`. The hub has no resource-deletion verb (CQRS, never CRUD).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
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
    /// 💰️ The transport's per-package byte budget is spent until its next refill turn: nothing is wrong with the hub, and
    /// the same request succeeds once the budget refills, so a caller waits instead of reporting a fault.
    BudgetExhausted,
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(detail) => write!(formatter, "transport io: {detail}"),
            Self::Cancelled => formatter.write_str("cancelled"),
            Self::DeadlineExceeded => formatter.write_str("deadline exceeded"),
            Self::BudgetExhausted => formatter.write_str("network byte budget exhausted until its next refill"),
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
/// (`🏃️run/🦀️.rs`) also gates a directory request, rather than each path inventing its
/// own bolted-on cancellation.
// 🔀️ dedyn-fw-os-misc, O1/R11(b): `DirectoryTransport` is already generic (never `dyn`, see
// `DirectoryClient<T: DirectoryTransport>` below) — `open_ws` is a trait method that RETURNS a
// runtime-chosen `DirectoryWsConnection` implementation, so the associated type `Ws` pushes that
// choice to the implementor exactly per the `ResourceResolver` precedent, replacing
// `Box<dyn DirectoryWsConnection>`.
pub trait DirectoryTransport: DirectoryTransportPlatform {
    type Ws: DirectoryWsConnection;
    async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError>;
    /// 🪢️ One GET that names the exact media type it accepts — a binary route such as the canonical checkpoint
    /// pair, whose hub refuses any other `Accept` — answered with the raw body bytes.
    async fn get_accepting(&self, ctx: &OperationContext, url: &str, bearer: Option<&str>, accept: &str) -> Result<HttpResponse, TransportError>;
    fn issue_socket_grant(&self, ctx: &OperationContext, url: &str, bearer: &str, body: &[u8], timeout_ms: u64) -> Result<HttpResponse, TransportError>;
    fn open_ws(&self, ctx: &OperationContext, url: &str, protocols: &[String], timeout_ms: u64) -> Result<Self::Ws, TransportError>;
}

/// 🔌️ One open `/directory/socket/v1` connection. Sequential by construction (`DirectoryStream` never
/// calls `send_text`/`try_recv_text` concurrently), so a single stream/sink object suffices — no
/// split halves needed the way `🏪️store/🔄️sync`'s bidirectional relay requires.
pub trait DirectoryWsConnection: DirectoryConnectionPlatform {
    fn send_text(&mut self, text: String) -> Result<(), TransportError>;
    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), TransportError>;
    /// 📭️ Nonblocking receive: `Pending` yields the worker immediately; `Closed` reconnects.
    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError>;
    fn close(&mut self);
}

/// 📬️ One nonblocking WebSocket receive observation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectoryWsPoll {
    Text(String),
    Pending,
    Closed(Option<u16>),
}
//#endregion 🔖️Transport
