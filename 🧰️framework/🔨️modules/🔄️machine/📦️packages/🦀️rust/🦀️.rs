//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️component.rs.

// 🧯️ `extern crate self as` binds only at the crate root, so it lives here rather than in the domain
// file: `statechart!`-expanded code names this crate `machine::…` both here and in consumer crates.
#[allow(unused_extern_crates)]
extern crate self as machine;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;
