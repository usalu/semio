//! 🚚️ Transport-level framing over an arbitrary duplex channel — `StdioTransport` (P1a, newline- [Design notes](README.md#source-notes-1).

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
