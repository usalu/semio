//! 📮️ The Streamable HTTP and bridge websocket transport.

#[cfg(test)]
#[path = "../🧵️bridge/🧪️tests/🔬️long/🦀️.rs"]
pub mod bridge_long;

#[cfg(test)]
#[path = "../🧵️bridge/🧪️tests/🔬️server/🦀️.rs"]
pub mod bridge_server;

#[path = "../🚚️transport/🦀️.rs"]
pub mod transport;

#[cfg(test)]
#[path = "../🚚️transport/🧪️tests/🔬️long/🦀️.rs"]
pub mod transport_long;

#[cfg(test)]
#[path = "../🚚️transport/🧪️tests/🔬️quick/🦀️.rs"]
pub mod transport_quick;
