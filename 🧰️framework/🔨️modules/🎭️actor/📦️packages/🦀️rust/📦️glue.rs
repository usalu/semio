//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️component.rs`. The `wasm_bindgen`
//! `KernelHost` wrapper lives at owner `🔗️bindings/🦀️.rs`, behind `#[cfg(target_arch = "wasm32")]`,
//! so the pure crate core never sees `wasm_bindgen`/`web_sys`, and this file stays pure
//! declaration/wiring (no `struct`/`impl` of its own) — keeping its taxonomy package role
//! classified as thin delegation rather than implementation, so it does not compete with
//! `🦀️component.rs` for the crate's one canonical implementation slot. It passes only byte buffers
//! (pack-encoded `Envelope`/`TurnResult`/`Decision`) — no typed value ever crosses the wasm boundary.

// 🚫️ `async_fn_in_trait` is allowed crate-wide per R3/R7 (ticket
// MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME): Send-ness is obtained structurally (every former `dyn`
// seam becomes a concrete enum, so the compiler derives `Send` at each spawn site), never by taking
// rustc's `-> impl Future + Send` suggestion, which would wrongly impose `Send` on this crate's
// guest-side (`?Send`) transports.
#![allow(async_fn_in_trait)]

#[cfg(target_arch = "wasm32")]
extern crate semio_framework_async as wasm_bindgen_futures;

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;

#[cfg(target_arch = "wasm32")]
#[path = "../../🔗️bindings/🦀️.rs"]
mod kernel_host;
#[cfg(target_arch = "wasm32")]
pub use kernel_host::KernelHost;
