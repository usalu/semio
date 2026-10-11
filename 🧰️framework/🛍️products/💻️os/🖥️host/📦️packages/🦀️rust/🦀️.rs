//! 🖥️ Semio framework OS host — Shape V2 glue.

#[cfg(any(feature = "os-host-full", feature = "space-guest"))]
extern crate semio_framework_os_kernel as dsl;
#[cfg(any(feature = "os-host-full", feature = "space-guest"))]
extern crate semio_framework_os_kernel as protocol;
#[cfg(any(feature = "os-host-full", feature = "space-guest"))]
extern crate semio_framework_value_derive as value_derive;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

/// 🛂️ Binds a finite test caller identity authority, the one every authored host edit is admitted through.
#[cfg(test)]
macro_rules! test_identity {
    ($identity:ident) => {
        test_identity!($identity, 1 << 20);
    };
    ($identity:ident, $maximum:expr) => {
        let mut observer_function = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
        let observer: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut observer_function;
        let mut $identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new($maximum, observer).expect("finite test identity admission");
    };
}

#[path = "../../💾️persistence/🦀️.rs"]
pub mod persistence;

//#region 🔖️OsHostFull
#[cfg(any(feature = "os-host-full", feature = "space-guest"))]
use semio_framework_artifact_workflow_workflow as workflow_kernel;

#[cfg(any(feature = "os-host-full", feature = "space-guest"))]
#[path = "../../../🔨️modules/🪐️space/🦀️.rs"]
pub mod space;

// 🌉️ Mirrors the target guard on the module this re-exports: `store::sync` is itself mounted
// `#[cfg(all(feature = "sync", not(all(target_arch = "wasm32", target_env = "p2"))))]` because the
// sync actor's transport is `tokio`/`tokio-tungstenite`, which a WASI-P2 guest never links. Without
// the same guard here the re-export is an unresolved import on `wasm32-wasip2`, and that single
// failure masks the rest of this crate's diagnostics.
#[cfg(all(any(feature = "os-host-full", feature = "space-guest"), not(all(target_arch = "wasm32", target_env = "p2"))))]
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
