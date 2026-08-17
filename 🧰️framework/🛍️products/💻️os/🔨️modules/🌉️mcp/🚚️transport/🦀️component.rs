//! 🚚️ Transport-level framing over an arbitrary duplex byte stream — `StdioTransport` is P1a's only
//! implementation (newline-delimited JSON-RPC on stdin/stdout, blocking loop; no tokio/axum here —
//! those arrive with P1b's HTTP transport, §2.5 of this packet's brief). **All diagnostic output goes
//! to a SEPARATE log writer, never to the response stream** — a stray byte on stdout corrupts every
//! later line the client tries to parse as JSON-RPC (`luna-mcpspec-audit.md`'s stdio guidance).

use crate::errors::{GatewayError, GatewayErrorCode};
use crate::protocol::{JsonRpcId, JsonRpcIncoming, JsonRpcResponse, McpServer, PARSE_ERROR};
use std::io::{BufRead, Write};

//#region 🔖️McpTransport
/// 🔌️ Drives one [`McpServer`] connection to completion over whatever duplex channel a concrete
/// implementor speaks — stdio today, HTTP/SSE arrives with P1b.
pub trait McpTransport {
    fn serve(&mut self, server: &mut McpServer) -> Result<(), GatewayError>;
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
    fn serve(&mut self, server: &mut McpServer) -> Result<(), GatewayError> {
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

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::protocol::{InMemoryPromptRegistry, InMemoryResourceRegistry, InMemoryToolRegistry, NullBackend};
    use std::io::Cursor;

    fn fresh_server() -> McpServer {
        McpServer::new(Box::new(InMemoryToolRegistry::new()), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(NullBackend))
    }

    #[test]
    fn one_request_line_produces_exactly_one_response_line_on_stdout() {
        let input = Cursor::new(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\"}\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(&mut fresh_server()).unwrap();

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
        transport.serve(&mut fresh_server()).unwrap();

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
        transport.serve(&mut fresh_server()).unwrap();
        let output_text = String::from_utf8(output).unwrap();
        assert_eq!(output_text.lines().count(), 1);
    }

    #[test]
    fn a_notification_line_produces_no_output_line_at_all() {
        let input = Cursor::new(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\"}\n".to_vec());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        transport.serve(&mut fresh_server()).unwrap();
        assert!(output.is_empty());
    }

    #[test]
    fn eof_ends_the_loop_cleanly() {
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();
        let mut log = Vec::new();
        let mut transport = StdioTransport::new(input, &mut output, &mut log);
        assert!(transport.serve(&mut fresh_server()).is_ok());
        assert!(output.is_empty());
    }
}
//#endregion 🧪️Tests
