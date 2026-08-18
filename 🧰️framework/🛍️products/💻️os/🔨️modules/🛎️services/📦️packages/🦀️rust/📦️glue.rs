//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️component.rs`. `tokio` is confined to
//! that owner file (and this glue file never imports it directly); there is no wasm-bindgen surface
//! here because this crate has no wasm target — it is a native host-process crate by construction
//! (tokio's multi-thread runtime never targets wasm32).

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
