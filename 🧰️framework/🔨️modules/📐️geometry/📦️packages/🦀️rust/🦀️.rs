//! 📐️ The semio geometry framework module: the first-party 2D vocabulary, the fixed-size render matrices, and the seeded Rng.
//!
//! Each domain is a `🦀️.rs` in the owner tree; this entry file is pure wiring.

#[path = "../../⚙️engine/🦀️.rs"]
mod engine;
pub use engine::*;

#[path = "../../🎲️random/🦀️.rs"]
pub mod random;

#[path = "../../➗️vector/🦀️.rs"]
pub mod vector;

#[path = "../../🌙️bulge/🦀️.rs"]
pub mod bulge;

#[path = "../../➰️loops/🦀️.rs"]
pub mod loops;

#[path = "../../🔺️triangulation/🦀️.rs"]
pub mod triangulation;

#[path = "../../🧭️placement/🦀️.rs"]
pub mod placement;

#[path = "../../🕸️mesh/🦀️.rs"]
pub mod mesh;

#[path = "../../🔪️section/🦀️.rs"]
pub mod section;

#[path = "../../🦴️skeleton/🦀️.rs"]
pub mod skeleton;

#[path = "../../🏠️roof/🦀️.rs"]
pub mod roof;

#[path = "../../💥️collision/🦀️.rs"]
pub mod collision;
