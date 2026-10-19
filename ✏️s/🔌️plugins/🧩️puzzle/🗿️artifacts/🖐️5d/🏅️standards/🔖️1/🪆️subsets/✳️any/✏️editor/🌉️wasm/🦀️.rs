//! 🌐️ Original-caller browser lifecycle and DSL parser, mounted through the current extracted bridge.

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use semio_s_artifact_puzzle_5d_host_admission_tests::editor::puzzle5d::wasm::bridge;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use bridge::*;
