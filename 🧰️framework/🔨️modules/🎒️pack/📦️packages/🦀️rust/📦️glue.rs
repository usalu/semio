//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️component.rs` files.
//!
//! The crate's `[lib] name` is `pack`: the container format keeps the one canonical name every
//! downstream caller already spells.

// 🔓️ R7: `async fn` in a public trait is our deliberate crate-wide shape (R3 answers the lint's
// underlying `Send` concern structurally — see `AsyncPackSource`/`RangeTransport`). Never take
// rustc's suggested `-> impl Future<...> + Send` fix; it re-imposes `Send` R3 forbids.
#![allow(async_fn_in_trait)]
#![allow(ambiguous_glob_reexports, unused_imports)]

// 📡️ Codec primitives, container identity and pack sources are owned by the replication module —
// the `.spk` container and the `.spr` record stream share one codec floor.
pub use protocol::codec;
pub use protocol::codec::ids;
pub use protocol::source;
/// 🌱️ Re-exported so a crate that already depends on `pack` (e.g. `🔺️mesh-engine`) can implement
/// `ToValue`/`FromValue` for its own types without taking a second, direct dependency on
/// `replication` — implementing a foreign trait for a local type is allowed by the orphan rule.
pub use protocol::value;

#[path = "../../📐️format/🦀️component.rs"]
pub mod format;

#[path = "../../🔤️json/🦀️component.rs"]
pub mod json;

#[path = "../../⏳️async/🦀️component.rs"]
pub mod async_;

#[path = "../../🌐️http/🦀️component.rs"]
pub mod http;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../../🔌️io/🦀️component.rs"]
pub mod io;

#[path = "../../🧪️testkit/🦀️component.rs"]
pub mod testkit;

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;

pub use protocol::codec::ids::*;
pub use protocol::codec::*;
pub use protocol::source::*;
