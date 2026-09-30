//! ⏪️ Package glue — wiring only. Domain lives at owner `🦀️.rs`; pure and target-neutral, so the
//! owner file compiles unmodified on native, `wasm32-unknown-unknown` and `wasm32-wasip2`.

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;
