//! 🔗️ Causal glue — wiring only. Domain lives at the owner `🦀️.rs` files.
//!
//! Envelopes, diffs, inverses and frontiers, with their binary and text codecs beside them.


#[cfg(test)]
use semio_framework_value_testing_allocator as _;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

#[path = "../../../🚪️io/💾️binary/🔗️causal/🦀️.rs"]
pub mod binary;

#[path = "../../../🚪️io/📝️text/🔗️causal/🦀️.rs"]
mod text;
