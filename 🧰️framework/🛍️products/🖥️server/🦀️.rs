//! 🖥️ Server product façade — the authoritative half of the same application model the os product
//! runs optimistically. The os owns a local replica; the server owns authority actor execution,
//! durable history, policy, projections and coordination. Neither imports the other: both speak
//! the `protocol` replication contract.
//!
//! An instance (hub, zentrale) is a value, not a fork: it registers modules against this product's
//! registries and runs the same host.

pub use crate::contract::*;
