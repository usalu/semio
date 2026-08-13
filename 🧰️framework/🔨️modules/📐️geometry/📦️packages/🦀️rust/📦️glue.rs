//! 📐️ The semio geometry framework module: the kurbo-backed 2D vocabulary, the fixed-size render matrices, and the seeded Rng.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

#[path = "../../⚙️engine/🦀️component.rs"]
mod engine;
pub use engine::*;

#[path = "../../🎲️random/🦀️component.rs"]
pub mod random;
