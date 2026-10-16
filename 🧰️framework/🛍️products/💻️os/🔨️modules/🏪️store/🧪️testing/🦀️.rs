//! 🧪️ Companion fixtures of the Store: the demo snapshot, its mutation families and the Store test prelude.
//!
//! 🧭️ This crate sits below the kernel facade: the kernel and the companion test crates depend on it, it depends only on the Store crates. The test doubles that need the Store engine live in `semio-framework-os-store-testing-doubles`.
#![allow(unused_imports, dead_code, ambiguous_glob_reexports, async_fn_in_trait, clippy::all)]

#[path = "🧬️mutations/🧮️demo/🧬️mutations/🦀️.rs"]
pub mod demo;
#[path = "🧬️mutations/🪤️lossy/🧬️mutations/🦀️.rs"]
pub mod lossy;
#[path = "🧬️mutations/🚦️severity/🧬️mutations/🦀️.rs"]
pub mod severity;
#[path = "🧬️mutations/⏱️timestamped/🧬️mutations/🦀️.rs"]
pub mod timestamped;
#[path = "🧬️mutations/🛂️validated/🧬️mutations/🦀️.rs"]
pub mod validated;

#[path = "🧭️prelude/🦀️.rs"]
pub mod prelude;

#[path = "🧮️snapshot/🦀️.rs"]
mod snapshot;
pub use snapshot::*;
#[path = "🧵️operations/🦀️.rs"]
mod operations;
pub use operations::*;
