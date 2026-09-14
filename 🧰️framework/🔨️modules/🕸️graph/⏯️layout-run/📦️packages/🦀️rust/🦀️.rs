//! ⏯️ Package glue — wiring only. Domain lives at owner `🦀️.rs`; pure and target-neutral, so the owner file
//! compiles unmodified on native, `wasm32-unknown-unknown` and `wasm32-wasip2`.

extern crate semio_framework_os_kernel as dsl;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;
