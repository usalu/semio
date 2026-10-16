//! 🧰️ Shared test fixtures of the wgpu renderer test bins: worlds, shells and apps that laws of several bins build from.
//!
//! 🧭️ Behind `testkit`: a bin lists this crate with `features = ["testkit"]`; each module holds what one layer of the renderer exports for tests.

#![cfg(any(test, feature = "testkit"))]

#[path = "🐚️shell/🦀️.rs"]
pub mod shell;
