//! 🎯️ Stdio transport of the server crate: The MCP server dispatcher and the stdio transport.

#[cfg(test)]
use axum::response::IntoResponse;
use std::io::BufRead;
use std::io::Write;
use std::sync::Arc;
use crate::mcp_server::McpServer;
use semio_os_mcp_consent::elicitation::{ElicitationChannel, ElicitationSlot, StdioLines, StdioRead};
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_wire::notify::{NotificationSink, NotificationSlot};
use semio_os_mcp_wire::protocol::{ClientFeatures, JsonRpcId, JsonRpcIncoming, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, METHOD_INITIALIZE, METHOD_NOTIFICATIONS_INITIALIZED, METHOD_PING, METHOD_SERVER_DISCOVER, PARSE_ERROR};
use std::sync::atomic::{AtomicBool, Ordering};

//#region 🔖️McpTransport

/// 🔌️ Drives one [`McpServer`] to completion over the synchronous stdio channel. The retained HTTP
/// owner has an explicit start/wait/control lifecycle because its pool turns must never block.
pub trait McpTransport {
    fn serve(&mut self, server: McpServer) -> Result<(), GatewayError>;
}

//#endregion 🔖️McpTransport

//#region 🔖️StdioTransport

/// 📻️ Newline-delimited JSON-RPC over `input`/`output`, with a THIRD writer (`log`) for every [Design notes](README.md#source-notes-26).
pub struct StdioTransport<L: Write> {
    pub lines: Arc<StdioLines>,
    pub log: L,
    pub elicitation: Option<ElicitationSlot>,
}

/// 📤️ The stdio connection's server→client notification lane — the SAME [`StdioLines`] owner the
/// serve loop and an in-flight `elicitation/create` write through, so a `notifications/progress`
/// emitted from inside a tool call reaches the client's stdin mid-call rather than after it
/// (`📓️m4-mcp-bridge-approval-binding.md` §5.3 built that single-owner channel; this rides it).
struct StdioNotificationSink {
    pub lines: Arc<StdioLines>,
}

impl NotificationSink for StdioNotificationSink {
    fn publish(&self, notification: JsonRpcNotification) {
        if let Ok(line) = serde_json::to_string(&notification) {
            let _ = self.lines.write_line(&line);
        }
    }
}

impl<L: Write> StdioTransport<L> {
    pub fn new(input: impl BufRead + Send + 'static, output: impl Write + Send + 'static, log: L) -> Self {
        Self { lines: Arc::new(StdioLines::new(Box::new(input), Box::new(output))), log, elicitation: None }
    }

    /// 📤️ Fills `slot` with this connection's notification sink, so an [`McpServer`] built with
    /// `publishing_notifications_into(slot)` pushes `resources/updated`, `resources/list_changed`
    /// and `progress` down the very descriptors this transport owns.
    #[must_use]
    pub fn publishing_notifications_into(self, slot: NotificationSlot) -> Self {
        let _ = slot.set(Arc::new(StdioNotificationSink { lines: self.lines.clone() }));
        self
    }

    /// 🙋 Publishes this connection's server→client request channel into `slot`, so the approval
    /// coordinator the tool registry already captured can ask the CLIENT's own human for a decision.
    /// `features` is the live client-capability mirror `McpServer` fills at `initialize`: elicitation
    /// is attempted only when the connected client advertised it.
    #[must_use]
    pub fn publishing_elicitation_into(mut self, slot: ElicitationSlot, features: Arc<ClientFeatures>) -> Self {
        let _ = slot.set(Arc::new(ElicitationChannel::new(self.lines.clone(), features)));
        self.elicitation = Some(slot);
        self
    }

    fn write_response(&mut self, response: &JsonRpcResponse) -> Result<(), GatewayError> {
        let line = serde_json::to_string(response).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        self.lines.write_line(&line)
    }

    fn log_line(&mut self, line: &str) {
        let _ = writeln!(self.log, "{line}");
    }
}

impl<L: Write> McpTransport for StdioTransport<L> {
    /// 🔁️ One line in → zero-or-one line out, until EOF (client closed stdin) or a hard io error. A [Design notes](README.md#source-notes-90).
    fn serve(&mut self, mut server: McpServer) -> Result<(), GatewayError> {
        loop {
            let Some(line) = self.lines.read_line()? else { return Ok(()) };
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
                        self.lines.write_line(&line)?;
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

//#region 🔖️StdioHandshake

const HANDSHAKE_POLL: std::time::Duration = std::time::Duration::from_millis(20);

/// 🤝️ The connection handshake (`initialize`, `server/discover`, `notifications/initialized`, `ping`), answered from a
/// bare [`McpServer`] while the real one is still being built. Every other line stays queued, in arrival order, for the real
/// server, which replays the answered handshake requests before it serves so its negotiated state is identical.
pub struct StdioHandshake {
    stop: Arc<AtomicBool>,
    answering: std::thread::JoinHandle<Vec<JsonRpcRequest>>,
}

impl StdioHandshake {
    fn finish(self) -> Vec<JsonRpcRequest> {
        self.stop.store(true, Ordering::Release);
        self.answering.join().unwrap_or_default()
    }
}

fn is_handshake_method(method: &str) -> bool {
    matches!(method, METHOD_INITIALIZE | METHOD_SERVER_DISCOVER | METHOD_NOTIFICATIONS_INITIALIZED | METHOD_PING)
}

fn answer_handshake(lines: Arc<StdioLines>, stop: Arc<AtomicBool>) -> Vec<JsonRpcRequest> {
    let mut bare = McpServer::with_defaults();
    let mut answered = Vec::new();
    while !stop.load(Ordering::Acquire) {
        let line = match lines.read_line_direct_within(HANDSHAKE_POLL) {
            Ok(StdioRead::Line(line)) => line,
            Ok(StdioRead::TimedOut) => continue,
            Ok(StdioRead::Eof) | Err(_) => break,
        };
        let Ok(JsonRpcIncoming::Single(request)) = serde_json::from_str::<JsonRpcIncoming>(line.trim()) else {
            lines.defer(line);
            break;
        };
        if !is_handshake_method(&request.method) {
            lines.defer(line);
            break;
        }
        if let Some(response) = bare.dispatch(&request) {
            if serde_json::to_string(&response).ok().is_none_or(|text| lines.write_line(&text).is_err()) {
                break;
            }
        }
        answered.push(request);
    }
    answered
}

impl<L: Write> StdioTransport<L> {
    /// 🤝️ Starts answering the handshake now, so a client's `initialize` does not wait for the server build that follows.
    pub fn answer_handshake_while_building(&self) -> StdioHandshake {
        let stop = Arc::new(AtomicBool::new(false));
        let answering = std::thread::Builder::new().name("semio-mcp-handshake".to_string()).spawn({
            let (lines, stop) = (self.lines.clone(), stop.clone());
            move || answer_handshake(lines, stop)
        }).expect("handshake thread spawns");
        StdioHandshake { stop, answering }
    }

    /// 🔁️ Serves `server` once built: replays the handshake already answered, then every queued and later line.
    pub fn serve_after_handshake(&mut self, mut server: McpServer, handshake: StdioHandshake) -> Result<(), GatewayError> {
        for request in handshake.finish() {
            let _ = server.dispatch(&request);
        }
        self.serve(server)
    }
}

//#endregion 🔖️StdioHandshake
