//! 🧱️ Protocol_tests_support of the server crate: The MCP server dispatcher and the stdio transport.

use semio_framework_os_mcp_wire::protocol::{JsonRpcId, JsonRpcRequest};

pub fn request_with(id: i64, method: &str, params: serde_json::Value) -> JsonRpcRequest {
    JsonRpcRequest { jsonrpc: "2.0".to_string(), id: Some(JsonRpcId::Number(id)), method: method.to_string(), params: Some(params) }
}
