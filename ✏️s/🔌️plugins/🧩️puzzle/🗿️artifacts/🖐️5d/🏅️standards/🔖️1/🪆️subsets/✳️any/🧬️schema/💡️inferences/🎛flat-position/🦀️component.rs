//! 🎛 `flat-position` — one named inference: absolute flatten pose (plane + center) per part.
//! The type carrying the pose (`FlattenPose`) and the compute itself (`flatten_snapshot`) are
//! already owned by `⚙️engine/📐️flatten` (maps parts/grips/fasteners onto the 3d object/vortex/
//! attraction graph and runs puzzle3d's own solver) — this leaf re-exports them under the slug's
//! own name so `📦️glue.rs` has a `flat_position` mount matching puzzle3d's own shape, without
//! duplicating either the type or the math.

pub use crate::artifacts::puzzle3d::engine::geometry::flatten::{FlattenPlane, FlattenPose};
pub use crate::artifacts::puzzle5d::standards::v1::engine::flatten::{flatten_snapshot, flatten_snapshot_inplace};
