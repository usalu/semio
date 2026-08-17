//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️component.rs`. This crate has no
//! wasm-bindgen surface of its own (the SSOT is plain serde data in, serde data out); a host-side
//! wasm binding is later-packet adoption work, not this packet's concern.

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
