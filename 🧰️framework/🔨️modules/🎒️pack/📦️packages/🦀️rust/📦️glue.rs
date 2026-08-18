//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️component.rs` files.
//!
//! The crate's `[lib] name` is `pack`: the container format keeps the one canonical name every
//! downstream caller already spells.

#![allow(ambiguous_glob_reexports, unused_imports)]

// 📡️ Codec primitives, container identity and pack sources are owned by the replication module —
// the `.spk` container and the `.spr` record stream share one codec floor.
pub use protocol::codec;
pub use protocol::codec::ids;
pub use protocol::source;

#[path = "../../📐️format/🦀️component.rs"]
pub mod format;

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

pub use protocol::codec::*;
pub use protocol::codec::ids::*;
pub use protocol::source::*;
