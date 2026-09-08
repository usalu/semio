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

#[path = "../../🦀️.rs"]
mod component;
/// 🧠️ Repository-owned, instruction-fuelled core execution boundary. Native component lifting is
/// layered above this module; browser builds retain the platform WebAssembly boundary.
#[path = "../../../🧠️interpreter/🦀️.rs"]
pub mod interpreter;
pub use component::*;

/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): the one world's contract-parity
/// test (effect ↔ host-async import parity, plus the collapsed shape itself) — mounted here rather than inside `🦀️.rs` (other packets are live
/// in that file). The test uses a narrow owned WIT source inspector and adds no external parser to
/// the test graph.
#[cfg(test)]
#[path = "../../🪞️schema-parity/🧪️tests/🔬️unit/🦀️.rs"]
mod schema_parity;

pub use semio_framework_os_config::opening_config;
