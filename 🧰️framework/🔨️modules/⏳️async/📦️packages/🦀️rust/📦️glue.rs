//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️component.rs`. This crate has no
//! wasm-bindgen wrapper yet: [`component::HostAsyncRuntime`] is a pure trait with no concrete
//! executor attached in this crate (the tokio-backed implementation is the sibling packet R2), so
//! there is nothing wasm-specific to wire at the glue layer today — this file has zero
//! `tokio`/`wasm_bindgen`/`web_sys`/`std::thread`, same as the owner file's purity contract.

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
