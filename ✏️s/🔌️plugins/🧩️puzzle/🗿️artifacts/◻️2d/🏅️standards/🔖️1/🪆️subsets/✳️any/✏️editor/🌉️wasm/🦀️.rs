//! 🎲️ Puzzle board (2d) rendering surface — WebGPU canvas session wrapping `puzzle_2d_engine`'s

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::wasm::bridge;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use bridge::*;
