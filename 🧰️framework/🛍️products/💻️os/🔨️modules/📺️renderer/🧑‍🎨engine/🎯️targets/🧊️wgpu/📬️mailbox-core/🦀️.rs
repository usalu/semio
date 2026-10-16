//! 📬️ Target-neutral bounded completion queue and interaction checkout ledger.
//!
//! 🧭️ The only module of the wasm32-wasip2 build: every target including wasm32-wasip2, no dependencies. The module file keeps its place in the target tree; this crate root mounts it and re-exports its surface.

#[path = "../📮️runtime-mailbox-core/🦀️.rs"]
mod runtime_mailbox_core;
pub use runtime_mailbox_core::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
