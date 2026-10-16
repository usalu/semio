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

#[path = "🧾️http-surface/🦀️.rs"]
mod http_surface;
#[path = "🔁️http-transport-state/🦀️.rs"]
mod http_transport_state;
#[path = "🪪️http-authority/🦀️.rs"]
mod http_authority;
#[path = "📬️http-dispatch/🦀️.rs"]
mod http_dispatch;
#[path = "🔌️websocket-codec/🦀️.rs"]
mod websocket_codec;
#[path = "🛡️origin-and-handlers/🦀️.rs"]
mod origin_and_handlers;

pub use http_surface::{
    HEADER_MISMATCH,
    HttpConnectionKey,
    HttpEventPublisher,
    HttpTerminalConnection,
    HttpTerminalReason,
    HttpTransport,
    HttpTransportCompletion,
    HttpTransportOptions,
    HttpTransportRun,
};
pub use http_transport_state::BRIDGE_OPENING_DEADLINE_MS;
#[cfg(test)]
pub(crate) use http_authority::generation_is_current;
#[cfg(test)]
pub(crate) use http_dispatch::{OwnedHttpResponse, dispatch_owned_bridge_handshake, dispatch_owned_http, find_crlf_bounded};
#[cfg(test)]
pub(crate) use http_surface::{EventLog, HttpAdmission, HttpState};
#[cfg(test)]
pub(crate) use http_transport_state::{
    BridgeSession,
    FixedByteCredits,
    FixedOwnerRing,
    HTTP_CONNECTION_CAPACITY,
    HTTP_HEADER_BYTES,
    HTTP_IO_PAGE_BYTES,
    HTTP_REQUEST_BYTES,
    HTTP_REQUEST_LINE_BYTES,
    HTTP_RESPONSE_BYTES,
    HTTP_SLOWLORIS_MS,
    HttpAfterWrite,
    HttpConnection,
    HttpConnectionPhase,
    HttpTerminalPolicy,
    HttpTransportMode,
    HttpTransportState,
    HttpTurn,
    OwnedHeader,
    ParsedHttpHead,
    WEBSOCKET_FRAME_BYTES,
};
#[cfg(test)]
pub(crate) use origin_and_handlers::{constant_time_eq, handle_get, handle_post, is_loopback_or_null, origin_allowed};
#[cfg(test)]
pub(crate) use websocket_codec::{decode_client_websocket_frame, encode_server_websocket_frame, websocket_accept, websocket_key_nonce};

//#region 🧪️Tests
//#endregion 🧪️Tests
