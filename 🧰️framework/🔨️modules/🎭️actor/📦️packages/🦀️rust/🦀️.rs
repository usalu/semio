//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️.rs`. The `wasm_bindgen` [Design notes](README.md#source-notes-1).


// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, so this is narrowed to
// `not(target_env = "p2")` — the browser-only `KernelHost` bridge below has no meaning for the
// WASI component target and must not pull `wasm-bindgen`/the aliased async runtime into it.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "../../🔗️bindings/🦀️.rs"]
mod kernel_host;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use kernel_host::KernelHost;

pub use semio_framework_actor_failure::{ActorFailureCallKind,ActorFailureCallReference,ActorFailureOutcome,ActorFailureChunk,ActorFailureCallFailure};
