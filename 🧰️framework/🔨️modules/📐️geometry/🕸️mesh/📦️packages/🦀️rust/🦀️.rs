//! 🕸️ Original polygon loops, triangulation, placement and owned solid construction.
pub use semio_framework_geometry_primitives::*;
pub use semio_framework_geometry_vector as vector;
pub use semio_framework_geometry_bulge as bulge;
#[path="../../../➰️loops/🦀️.rs"]
pub mod loops;
#[path="../../../🔺️triangulation/🦀️.rs"]
pub mod triangulation;
#[path="../../../🧭️placement/🦀️.rs"]
pub mod placement;
#[path="../../🦀️.rs"]
pub mod mesh;
