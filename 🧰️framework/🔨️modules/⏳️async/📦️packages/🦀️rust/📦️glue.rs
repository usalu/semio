//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️component.rs`. This crate has no
//! wasm-bindgen wrapper yet: [`component::HostAsyncRuntime`] is a pure trait with no concrete
//! executor attached in this crate (the tokio-backed implementation is the sibling packet R2), so
//! there is nothing wasm-specific to wire at the glue layer today — this file has zero
//! `tokio`/`wasm_bindgen`/`web_sys`/`std::thread`, same as the owner file's purity contract.

// 🔕 async_fn_in_trait warns that callers can't assume Send on the returned future; R3 answers this
// structurally — every former dyn seam becomes a concrete enum so Send falls out at the spawn site.
// Never resolve this by adding `+ Send` to a trait method or by making it sync (R7).
#![allow(async_fn_in_trait)]

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
