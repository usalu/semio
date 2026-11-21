//! 🧮️ Constructs bounded rational arcs and interpolating curves and surfaces under retained grants.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::contract::Vec3 as EVec3;
use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Pnt3;

#[path = "🪡️grid-fit/🦀️.rs"]
mod grid_fit;
pub use grid_fit::{CoonsSurfaceBuild, CurveSplineBuild, GridSurfaceBuild};

#[path = "🌙️arc/🦀️.rs"]
mod arc;
pub use arc::ArcSplineBuild;


#[path = "〰️nurbs-cuboid/🦀️.rs"]
pub mod nurbs_cuboid;

#[path = "🪢️quadratic-arc/🦀️.rs"]
pub mod quadratic_arc;

#[cfg(test)]
#[path = "🧪️tests/🔬️funded-storage/🦀️.rs"]
mod funded_storage;
