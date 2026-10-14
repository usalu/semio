//! 📐️ The semio geometry framework module: the first-party 2D vocabulary, the fixed-size render matrices, and the seeded Rng.
//!
//! Each domain is a `🦀️.rs` in the owner tree; this entry file is pure wiring.

pub use semio_framework_geometry_primitives::*;

#[path = "../../🎲️random/🦀️.rs"]
pub mod random;

pub use semio_framework_geometry_vector as vector;

pub use semio_framework_geometry_bulge as bulge;




pub use semio_framework_geometry_solids::{loops, mesh, placement, triangulation};

#[path = "../../🔪️section/🦀️.rs"]
pub mod section;

pub use semio_framework_geometry_skeleton as skeleton;

#[path = "../../🏠️roof/🦀️.rs"]
pub mod roof;

#[path = "../../💥️collision/🦀️.rs"]
pub mod collision;
