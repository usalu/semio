//! 🎯️ The MCP server dispatcher and the stdio transport.

#[path = "../🧭️protocol/🎯️mcp-server/🦀️.rs"]
pub mod mcp_server;

#[cfg(test)]
#[path = "../🧭️protocol/🧪️tests/🔬️long/🦀️.rs"]
pub mod protocol_long;

#[cfg(test)]
#[path = "../🧭️protocol/🧪️tests/🔬️quick/🦀️.rs"]
pub mod protocol_quick;

#[cfg(test)]
#[path = "../🧭️protocol/🧪️tests/🔬️tests-support/🦀️.rs"]
pub mod protocol_tests_support;

#[path = "../🚚️transport/🎯️stdio-transport/🦀️.rs"]
pub mod stdio_transport;
