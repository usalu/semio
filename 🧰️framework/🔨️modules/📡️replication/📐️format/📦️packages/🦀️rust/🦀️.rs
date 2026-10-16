//! 📐️ Format glue — wiring only. Domain lives at the owner `🦀️.rs` files.
//!
//! The `.spr` record file format with the slice helpers that ship a byte-exact record range.


#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

#[path = "../../../🦀️.rs"]
mod slice;
pub use slice::*;
