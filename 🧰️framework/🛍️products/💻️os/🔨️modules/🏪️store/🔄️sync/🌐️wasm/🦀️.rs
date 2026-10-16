//! 🌐️ The browser wasm32 Store sync actor.
//!
//! 🧭️ wasm32-unknown-unknown only. Depends on `os-store-sync`; the browser `ArtifactHost` composes its `ArtifactHostCore` with the page's socket dialer.

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🎭️actor/🦀️.rs"]
pub mod wasm_actor;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use wasm_actor::{ArtifactChannels, ArtifactHost};
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;
