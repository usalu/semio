//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️component.rs.

// 🧯️ `extern crate self as` binds only at the crate root, so it lives here rather than in the domain
// file: `statechart!`-expanded code names this crate `fsm::…` both here and in consumer crates.
#[allow(unused_extern_crates)]
extern crate self as fsm;

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;

