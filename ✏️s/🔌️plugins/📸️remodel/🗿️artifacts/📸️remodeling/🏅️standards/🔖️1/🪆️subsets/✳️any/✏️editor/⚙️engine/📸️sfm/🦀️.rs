//! 🏗️ Structure from motion: two-view geometry, triangulation, PnP, incremental and global reconstruction, bundle adjustment, loop closure and pose priors.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).

use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_feature::editor::remodeling::engine::feature as remodeling_feature;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Sim3;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::So3;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::RobustLoss;
use remodeling_camera::{CameraPose, Distortion, Intrinsics};
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::jacobi_eigen_symmetric;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::poly_roots_companion;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::real_eigenvalues;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::svd;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::svd_nullvector;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::vec3d_length;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::vec3d_normalize;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::vec3d_sub;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::Mat3d;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::MatD;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::umeyama;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::lo_ransac;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::numeric_jacobian;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::ransac;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::BipartiteResiduals;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::LeastSquaresProblem;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::LmConfig;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::MinimalSolver;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::RansacConfig;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::RansacScoring;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::ResidualTerm;
use geometry::random::{normal, Rng};
use remodeling_camera::reproject;
use remodeling_feature::{match_brute, Descriptor256, Keypoint, Match};
use remodeling_image::ImageGray;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::SchurLmPreparation;
#[cfg(test)] use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::{schur_lm, SchurResult};
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::FeatureTracks;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::INTERACTIVE_POSE_REFINE_ITERATIONS;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::P3pSolver;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::SfmError;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::TwoViewModel;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::TwoViewResult;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::camera_center;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::cross3;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::decompose_essential;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::dot3;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential_five_point;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential_five_point_bounded;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential_with_limit;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential_with_threshold;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::mat3d_to_array;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::pnp_ransac;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::pnp_ransac_bounded;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::polish_essential;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::scale3;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::triangulate_and_validate;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::triangulate_normalized_pair;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::triangulation_angle;

#[path = "🛰️synthetic-fixtures-and-priors/🦀️.rs"]
mod synthetic_fixtures_and_priors;
pub use synthetic_fixtures_and_priors::*;

#[path = "🧱️incremental-state/🦀️.rs"]
mod incremental_state;
pub use incremental_state::*;

#[path = "🧭️seed-and-registration/🦀️.rs"]
mod seed_and_registration;

#[path = "🔩️bundle-and-snapshot/🦀️.rs"]
mod bundle_and_snapshot;

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
