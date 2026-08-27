//! 🖥️ Plugin host — Shape V2 glue.
// 👶️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (host-dedyn), ruling R7: `GuestRuntime`'s async
// methods are plain AFIT (`async fn` in a public trait) — the lint's real concern (callers cannot
// assume the returned future is `Send`) is answered STRUCTURALLY by R3: every dyn seam here is a
// concrete enum (`GuestRuntimes`), so the future's concrete type is known at each call site and
// `Send` falls out of the compiler's own analysis, never a bound on the trait method itself.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../../🦀️component.rs"]
mod component;
/// 🧠️ Repository-owned, instruction-fuelled core execution boundary. Native component lifting is
/// layered above this module; browser builds retain the platform WebAssembly boundary.
#[path = "../../../🧠️interpreter/🦀️component.rs"]
pub mod interpreter;
pub use component::*;

/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): the one world's contract-parity
/// test (effect ↔ host-async import parity, plus the collapsed shape itself) — mounted here rather than inside `🦀️component.rs` (other packets are live
/// in that file). The test uses a narrow owned WIT source inspector and adds no external parser to
/// the test graph.
#[cfg(test)]
#[path = "../../🧪️schema-parity/🦀️component.rs"]
mod schema_parity;

/// 🎚️ Mounts the OS config schema and every direct semantic mutation leaf. `#[path = "."]` keeps
/// Rust's synthetic module names from changing the base directory for emoji-named source folders.
#[path = "."]
pub mod opening_config {
    #[path = "../../../../../🎚️config/🧬️schema/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "."]
    pub mod mutations {
        #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🦀️component.rs"]
        pub mod set_default_app;
        #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🦀️component.rs"]
        pub mod clear_default_app;
        #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️component.rs"]
        pub mod change_merge_policy;
        #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🦀️component.rs"]
        pub mod sign_in;
        #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🦀️component.rs"]
        pub mod sign_out;
    }
}
