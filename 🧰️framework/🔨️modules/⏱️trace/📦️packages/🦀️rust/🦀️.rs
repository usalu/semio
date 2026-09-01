//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️component.rs`. Zero dependencies (see that
//! file's module doc): nothing to wire at the glue layer beyond the `#[path]` re-export below — no
//! wasm-bindgen wrapper, this crate exposes plain Rust fns/types that compile unmodified on
//! `wasm32-unknown-unknown`/`wasm32-wasip2` as well as native.

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;
