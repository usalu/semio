//! 🖥️ Semio framework OS host — Shape V2 glue.

#[cfg(feature = "os-host-full")]
extern crate semio_framework_os_kernel as dsl;
#[cfg(feature = "os-host-full")]
extern crate semio_framework_os_kernel as protocol;
#[cfg(feature = "os-host-full")]
extern crate semio_framework_value_derive as value_derive;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

//#region 🔖️OsHostFull
// 🧬️ `workflow_kernel` is the private path-mount of `🔨️modules/🔁️workflow` — kept distinct from the
// public OS-layer `pub mod workflow` in host so that module can re-export the kernel vocabulary and
// layer media/registry helpers on top. Public spelling beside `space` is `workflow` only.
#[cfg(feature = "os-host-full")]
#[path = "../../../🔨️modules/🔁️workflow/🦀️.rs"]
mod workflow_kernel;

#[cfg(feature = "os-host-full")]
#[path = "../../../🔨️modules/🪐️space/🦀️.rs"]
pub mod space;

// 🌉️ Mirrors the target guard on the module this re-exports: `store::sync` is itself mounted
// `#[cfg(all(feature = "sync", not(all(target_arch = "wasm32", target_env = "p2"))))]` because the
// sync actor's transport is `tokio`/`tokio-tungstenite`, which a WASI-P2 guest never links. Without
// the same guard here the re-export is an unresolved import on `wasm32-wasip2`, and that single
// failure masks the rest of this crate's diagnostics.
#[cfg(all(feature = "os-host-full", not(all(target_arch = "wasm32", target_env = "p2"))))]
pub use store::sync as store_sync;
//#endregion 🔖️OsHostFull

#[path = "../../🦀️.rs"]
mod host_core;
pub use host_core::*;

// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the shared native
// kernel-activation facade — see `🎠️activation/🦀️.rs`'s own module doc for why it lives here rather
// than in `🎯️targets/🧊️wgpu`'s `ParallelRuntime`. Native-only, same reason `NativeKernelRuntime`
// itself is: real OS threads (`ShardExecutor`s + forwarders), never compiled for wasm32.
#[cfg(not(target_arch = "wasm32"))]
#[path = "../../🎠️activation/🦀️.rs"]
pub mod activation;
