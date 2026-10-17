//! 🏃️ Videogrammetry: multi-frame tracking, 3D trajectories, kinematics, vibration and modal analysis, camera sync, stabilization and non-rigid capture.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).
use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_feature::editor::remodeling::engine::feature as remodeling_feature;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;

use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3;
use remodeling_camera::{CameraPose, Intrinsics};
use remodeling_image::{ImageGray, Pyramid};

use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::solve_llsq;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::MatD;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::levenberg_marquardt;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::numeric_jacobian;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::LeastSquaresProblem;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::LmConfig;
use semio_s_artifact_remodel_remodeling_spatial::standards::v1::subsets::any::schema::spatial_internals::KdTree;
use remodeling_feature::{forward_backward_prune, klt_track, shi_tomasi_grid};

#[path = "🎯️tracking-and-trajectories/🦀️.rs"]
mod tracking_and_trajectories;
pub use tracking_and_trajectories::*;

#[path = "🎼️kinematics-modal-and-sync/🦀️.rs"]
mod kinematics_modal_and_sync;
pub use kinematics_modal_and_sync::*;

#[path = "🪞️camera-motion-compensation/🦀️.rs"]
mod camera_motion_compensation;
pub use camera_motion_compensation::*;

#[path = "🫧️non-rigid-and-pose/🦀️.rs"]
mod non_rigid_and_pose;
pub use non_rigid_and_pose::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
