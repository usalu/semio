//! 🔁️ The retained HTTP transport state machine: limits, rings, head parsing, connection phases and the per-turn drivers.

use semio_framework_os_mcp_bridge::session::{BridgeEncodedLease, BridgeHandle, BridgeOutboxReceiver, ShellConnectionId};
use semio_framework_os_mcp_bridge_codec::bridge::{
    BRIDGE_VERSION,
    BridgeRefusal,
    GatewayToShell,
    ShellDecodeFault,
    ShellDecodeStep,
    ShellFrameKind,
    ShellMaterializeStep,
    ShellToGateway,
    ShellToGatewayDecodeCursor,
    ShellToGatewayMaterializeCursor,
};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_server::mcp_server::McpServer;
use std::io::Write;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use super::http_authority::ConnectionTurn;
use super::http_dispatch::{dispatch_owned_http, find_crlf_bounded, is_http_token_byte};
use super::http_surface::{EventLog, HttpConnectionKey, HttpState, HttpTerminalConnection, HttpTerminalReason, HttpTransportOptions};
use super::websocket_codec::{ClientWebSocketFrame, decode_client_websocket_frame, encode_server_websocket_frame, websocket_server_header};

//#region 🔖️HttpTransport
pub(crate) const HTTP_CONNECTION_CAPACITY: usize = 64;

const HTTP_HEADER_CAPACITY: usize = 64;

pub(crate) const HTTP_HEADER_BYTES: usize = 65_536;

pub(crate) const HTTP_REQUEST_LINE_BYTES: usize = 4_096;

const HTTP_HEADER_NAME_BYTES: usize = 64;

const HTTP_HEADER_VALUE_BYTES: usize = 8_192;

const HTTP_PATH_BYTES: usize = 2_048;

pub(crate) const HTTP_REQUEST_BYTES: usize = 1_048_576;

pub(crate) const HTTP_RESPONSE_BYTES: usize = 1_048_576;

pub(crate) const HTTP_IO_PAGE_BYTES: usize = 16_384;

pub(crate) const HTTP_SLOWLORIS_MS: u64 = 15_000;

/// ⏳️ How long an upgraded `/bridge` socket may stay open without its `Hello` before the gateway
/// closes it: every accepted connection ends in a bounded time, answered or not.
pub const BRIDGE_OPENING_DEADLINE_MS: u64 = 10_000;

/// 🚫️ What a connection accepted while every slot is taken is answered with before it is closed: a
/// typed refusal, never a silent wait in the kernel's accept queue.
const HTTP_CAPACITY_REFUSAL: &[u8] = b"HTTP/1.1 503 Service Unavailable\r\nContent-Type: application/json\r\nContent-Length: 60\r\nConnection: close\r\n\r\n{\"schema\":\"semio.mcp.transport-refusal/v1\",\"reason\":\"capacity\"}";

pub(super) const HTTP_READINESS_POLL_MS: u64 = 4;

pub(super) const HTTP_RETRY_MS: u64 = 1;

pub(crate) const WEBSOCKET_FRAME_BYTES: usize = 1_048_576;

pub(crate) struct FixedOwnerRing<T, const N: usize> {
    pub slots: Box<[Option<T>]>,
    pub(crate) head: usize,
    pub len: usize,
}

pub(crate) struct FixedByteCredits {
    pub used: usize,
    pub(super) cap: usize,
}

impl FixedByteCredits {
    pub(crate) fn new(cap: usize) -> Self {
        Self { used: 0, cap }
    }

    pub(super) fn remaining(&self) -> usize {
        self.cap.saturating_sub(self.used)
    }

    pub(crate) fn try_acquire<T>(&mut self, bytes: usize, owner: T) -> Result<T, T> {
        if bytes > self.remaining() {
            return Err(owner);
        }
        self.used += bytes;
        Ok(owner)
    }

    pub(crate) fn release(&mut self, bytes: usize) {
        self.used = self.used.saturating_sub(bytes);
    }
}

impl<T, const N: usize> FixedOwnerRing<T, N> {
    pub(crate) fn new() -> Self {
        Self { slots: semio_framework_async_slots::boxed_slots(N, || None), head: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn try_push(&mut self, owner: T) -> Result<(), T> {
        if self.len == N {
            return Err(owner);
        }
        let index = (self.head + self.len) % N;
        self.slots[index] = Some(owner);
        self.len += 1;
        Ok(())
    }

    pub(crate) fn pop_front(&mut self) -> Option<T> {
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
pub(crate) struct OwnedHeader {
    pub name: String,
    pub value: String,
}

pub(crate) struct ParsedHttpHead {
    pub method: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
    pub(crate) headers: [Option<OwnedHeader>; HTTP_HEADER_CAPACITY],
    pub(crate) header_count: usize,
    pub cursor: usize,
    pub(crate) header_end: Option<usize>,
    pub(crate) content_length: usize,
}

impl ParsedHttpHead {
    pub(crate) fn new() -> Self {
        Self { method: None, path: None, version: None, headers: std::array::from_fn(|_| None), header_count: 0, cursor: 0, header_end: None, content_length: 0 }
    }

    pub(crate) fn header(&self, name: &str) -> Option<&str> {
        self.headers[..self.header_count].iter().flatten().find(|header| header.name.eq_ignore_ascii_case(name)).map(|header| header.value.as_str())
    }

    pub(super) fn header_occurrences(&self, name: &str) -> usize {
        self.headers[..self.header_count].iter().flatten().filter(|header| header.name.eq_ignore_ascii_case(name)).count()
    }

    pub(super) fn request_end(&self) -> Option<usize> {
        self.header_end.and_then(|end| end.checked_add(self.content_length))
    }
}

#[derive(Clone, Copy)]
pub(crate) enum HttpAfterWrite {
    Close,
    Upgrade,
    WebSocket,
    WebSocketClose,
}

#[derive(Clone, Copy)]
pub(crate) enum HttpConnectionPhase {
    ReadHttp,
    ParseHttp,
    DispatchHttp,
    Write(HttpAfterWrite),
    ReadWebSocket,
    ParseWebSocket,
    DrainBridgeOutbox,
}

pub(crate) struct BridgeSession {
    pub id: Option<ShellConnectionId>,
    pub(crate) outbox: Option<BridgeOutboxReceiver>,
    pub(crate) opening: bool,
    pub(crate) inbound: Option<BridgeInboundCursor>,
}

enum BridgeInboundPhase {
    Decode(ShellToGatewayDecodeCursor),
    Materialize(ShellToGatewayMaterializeCursor),
    Ready(Option<ShellToGateway>),
}

struct BridgeInboundCursor {
    pub generation: u64,
    pub(crate) frame: ClientWebSocketFrame,
    pub phase: BridgeInboundPhase,
}

pub(super) struct BridgeWriteCursor {
    pub(crate) header: [u8; 10],
    pub(super) header_len: usize,
    pub(crate) frame: BridgeEncodedLease,
    pub(crate) written: usize,
    credited: bool,
}

impl BridgeWriteCursor {
    fn total_len(&self) -> usize {
        self.header_len + self.frame.len()
    }

    pub(super) fn remaining(&self) -> usize {
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

pub(crate) struct HttpConnection {
    pub key: HttpConnectionKey,
    pub(crate) peer: SocketAddr,
    pub(crate) stream: TcpStream,
    pub(crate) ingress: Vec<u8>,
    pub(crate) egress: Vec<u8>,
    pub(crate) bridge_egress: Option<BridgeWriteCursor>,
    pub(crate) written: usize,
    pub(crate) parser: ParsedHttpHead,
    pub phase: HttpConnectionPhase,
    pub bridge: BridgeSession,
    pub(crate) last_progress_ms: u64,
    pub(crate) header_deadline_ms: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum HttpTransportMode {
    Running,
    Closing,
    Closed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum HttpTerminalPolicy {
    Handback,
    Close,
}

pub(crate) enum HttpTurn {
    MoreWork,
    PollReadiness,
    Parked,
    Complete(Result<(), GatewayError>),
}

pub(crate) struct HttpTransportState {
    pub listener: Option<TcpListener>,
    pub(crate) connections: Box<[Option<HttpConnection>]>,
    pub terminal: FixedOwnerRing<HttpTerminalConnection, HTTP_CONNECTION_CAPACITY>,
    pub(crate) terminal_policy: HttpTerminalPolicy,
    pub mode: HttpTransportMode,
    terminal_result: Option<GatewayError>,
    pub(crate) run_generation: u64,
    next_connection_generation: u64,
    pub(crate) io_cursor: usize,
    pub(crate) request_credits: FixedByteCredits,
    pub(crate) response_credits: FixedByteCredits,
    pub(super) http: HttpState,
    pub bridge: BridgeHandle,
}

impl HttpTransportState {
    pub(crate) fn new(listener: TcpListener, server: Option<McpServer>, options: &HttpTransportOptions, events: Arc<Mutex<EventLog>>, bridge: BridgeHandle) -> Self {
        Self {
            listener: Some(listener),
            connections: semio_framework_async_slots::boxed_slots(HTTP_CONNECTION_CAPACITY, || None),
            terminal: FixedOwnerRing::new(),
            terminal_policy: HttpTerminalPolicy::Handback,
            mode: HttpTransportMode::Running,
            terminal_result: None,
            run_generation: 1,
            next_connection_generation: 1,
            io_cursor: 0,
            request_credits: FixedByteCredits::new(HTTP_CONNECTION_CAPACITY * HTTP_REQUEST_BYTES),
            response_credits: FixedByteCredits::new(HTTP_CONNECTION_CAPACITY * HTTP_RESPONSE_BYTES),
            http: HttpState { server: server.map(|server| Arc::new(Mutex::new(server))), admission: options.admission.clone(), allowed_origins: Arc::new(options.allowed_origins.clone()), events },
            bridge,
        }
    }

    fn active_connections(&self) -> usize {
        self.connections.iter().filter(|slot| slot.is_some()).count()
    }

    pub(crate) fn drive_one(&mut self, now_ms: u64) -> HttpTurn {
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
            return self.refuse_one_at_capacity();
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

    /// 🚫️ At capacity the next waiting connection is still accepted — and answered with
    /// [`HTTP_CAPACITY_REFUSAL`] and closed — so a client learns why instead of waiting on a socket the
    /// gateway will never read. Nothing waiting: the run parks until a slot frees.
    fn refuse_one_at_capacity(&mut self) -> HttpTurn {
        let Some(listener) = self.listener.as_ref() else { return HttpTurn::Parked };
        match listener.accept() {
            Ok((mut stream, _peer)) => {
                let _ = stream.set_nonblocking(true);
                let _ = stream.write(HTTP_CAPACITY_REFUSAL);
                let _ = stream.shutdown(Shutdown::Both);
                HttpTurn::MoreWork
            }
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => HttpTurn::MoreWork,
            Err(_) => HttpTurn::Parked,
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
        if now_ms >= connection.header_deadline_ms && (matches!(connection.phase, HttpConnectionPhase::ReadHttp | HttpConnectionPhase::ParseHttp) || (connection.bridge.opening && matches!(connection.phase, HttpConnectionPhase::ReadWebSocket | HttpConnectionPhase::ParseWebSocket))) {
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

    pub(crate) fn terminalize(&mut self, mut connection: HttpConnection, reason: HttpTerminalReason) {
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
        let response = match dispatch_owned_http(&self.http, &connection.parser, &connection.ingress[..request_end]) {
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
        if response.upgrade {
            connection.header_deadline_ms = connection.last_progress_ms.saturating_add(BRIDGE_OPENING_DEADLINE_MS);
        }
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

    /// 💬️ A human turn typed at the agent is recorded exactly like every other shell→gateway
    /// observation: onto its own connection, for `semio://ui/agent-messages` to drain.
    ///
    /// 🛑️ A cancel for a running tool call — `BridgeHandle::record` routes it to the one
    /// process-wide job registry rather than to this connection's own state.
    ///
    /// 🗿️ The shell's reply to a `GatewayToShell::AppCommand` — the artifact route's own
    /// response frame. It used to terminate the connection as `Unsupported` (nothing on the
    /// gateway had ever sent an `AppCommand`, so nothing could answer one); `🐚️channel`'s
    /// `ShellArtifactChannel` is the consumer that makes it a real reply.
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
            connection.bridge.inbound = Some(BridgeInboundCursor { generation: connection.key.generation, phase: BridgeInboundPhase::Decode(ShellToGatewayDecodeCursor::new(frame.payload_len)), frame });
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
                    ShellDecodeStep::Pending => return ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket),
                    ShellDecodeStep::Fault(ShellDecodeFault::Capacity) => return ConnectionTurn::Terminal(HttpTerminalReason::Capacity),
                    ShellDecodeStep::Fault(ShellDecodeFault::Malformed) => return ConnectionTurn::Terminal(HttpTerminalReason::Malformed),
                    ShellDecodeStep::Complete(frame) => {
                        let kind = frame.kind();
                        if opening && kind != ShellFrameKind::Hello {
                            return ConnectionTurn::Terminal(HttpTerminalReason::Malformed);
                        }
                        if !opening && kind == ShellFrameKind::Hello {
                            return ConnectionTurn::Terminal(HttpTerminalReason::Unsupported);
                        }
                        inbound.phase = BridgeInboundPhase::Materialize(ShellToGatewayMaterializeCursor::new(frame));
                        return ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket);
                    }
                }
            }
            BridgeInboundPhase::Materialize(materializer) => match materializer.step() {
                ShellMaterializeStep::Pending => return ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket),
                ShellMaterializeStep::Fault(ShellDecodeFault::Capacity) => return ConnectionTurn::Terminal(HttpTerminalReason::Capacity),
                ShellMaterializeStep::Fault(ShellDecodeFault::Malformed) => return ConnectionTurn::Terminal(HttpTerminalReason::Malformed),
                ShellMaterializeStep::Complete(message) => {
                    inbound.phase = BridgeInboundPhase::Ready(Some(message));
                    return ConnectionTurn::Keep(HttpConnectionPhase::ParseWebSocket);
                }
            },
            BridgeInboundPhase::Ready(message) => message.take().expect("ready bridge message disappeared"),
        };
        let consumed = connection.bridge.inbound.as_ref().expect("bridge inbound cursor disappeared").frame.consumed;
        match message {
            ShellToGateway::Hello { bridge_version, .. } if connection.bridge.opening && bridge_version != BRIDGE_VERSION => {
                self.consume_websocket_ingress(connection, consumed);
                let refused = GatewayToShell::Refused { reason: BridgeRefusal::Version, gateway_version: BRIDGE_VERSION };
                self.queue_websocket_payload(connection, 0x2, &refused.encode(), HttpAfterWrite::WebSocketClose)
            }
            ShellToGateway::Hello { flags, .. } if connection.bridge.opening => {
                self.consume_websocket_ingress(connection, consumed);
                let (id, outbox) = self.bridge.register();
                self.bridge.record_hello(id, flags);
                connection.bridge.id = Some(id);
                connection.bridge.outbox = Some(outbox);
                connection.bridge.opening = false;
                let welcome = GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: id.to_string(), principal: "agent:local".to_string() };
                self.queue_websocket_payload(connection, 0x2, &welcome.encode(), HttpAfterWrite::WebSocket)
            }
            ShellToGateway::Ping => {
                self.consume_websocket_ingress(connection, consumed);
                self.queue_websocket_payload(connection, 0x2, &GatewayToShell::Pong.encode(), HttpAfterWrite::WebSocket)
            }
            ShellToGateway::Bye => {
                self.consume_websocket_ingress(connection, consumed);
                self.queue_websocket_payload(connection, 0x8, &[], HttpAfterWrite::WebSocketClose)
            }
            message @ (ShellToGateway::ShellState { .. }
            | ShellToGateway::ShellStatePatch { .. }
            | ShellToGateway::Instances { .. }
            | ShellToGateway::ShellCommandResult { .. }
            | ShellToGateway::Approval { .. }
            | ShellToGateway::AgentMessage { .. }
            | ShellToGateway::AgentCancel { .. }
            | ShellToGateway::AppFrames { .. }) => {
                let Some(id) = connection.bridge.id else { return ConnectionTurn::Terminal(HttpTerminalReason::Malformed) };
                self.consume_websocket_ingress(connection, consumed);
                self.bridge.record(id, message);
                ConnectionTurn::Keep(HttpConnectionPhase::DrainBridgeOutbox)
            }
            ShellToGateway::Hello { .. } => ConnectionTurn::Terminal(HttpTerminalReason::Unsupported),
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

    fn queue_encoded_websocket_payload(&mut self, connection: &mut HttpConnection, opcode: u8, frame: BridgeEncodedLease, after: HttpAfterWrite) -> ConnectionTurn {
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
//#endregion 🔖️HttpTransport
