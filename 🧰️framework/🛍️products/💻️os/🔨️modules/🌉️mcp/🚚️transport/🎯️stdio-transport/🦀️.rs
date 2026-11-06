//! 🎯️ Stdio transport of the server crate: The MCP server dispatcher and the stdio transport.

#[cfg(test)]
use axum::response::IntoResponse;
use std::io::BufRead;
use std::io::Write;
use std::sync::Arc;
use crate::mcp_server::McpServer;
use semio_os_mcp_consent::elicitation::{ElicitationChannel, ElicitationSlot, StdioLines};
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_wire::notify::{NotificationSink, NotificationSlot};
use semio_os_mcp_wire::protocol::{ClientFeatures, JsonRpcId, JsonRpcIncoming, JsonRpcNotification, JsonRpcResponse, PARSE_ERROR};

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
