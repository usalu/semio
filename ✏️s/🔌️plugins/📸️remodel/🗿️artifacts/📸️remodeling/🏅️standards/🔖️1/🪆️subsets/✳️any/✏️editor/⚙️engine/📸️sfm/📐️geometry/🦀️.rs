//! 📐️ Multi-view geometry: two-view models and essential-matrix estimation, triangulation, PnP, feature tracks, global rotation/translation averaging, loop closure and pose priors.

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
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::SchurLmPreparation;
#[cfg(test)] use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::{schur_lm, SchurResult};

#[path = "🧮️matrix-helpers/🦀️.rs"]
mod matrix_helpers;
pub use matrix_helpers::*;

#[path = "🔀️two-view/🦀️.rs"]
mod two_view;
pub use two_view::*;

#[path = "📍️triangulation-and-pnp/🦀️.rs"]
mod triangulation_and_pnp;
pub use triangulation_and_pnp::*;

#[path = "🛤️tracks-global-and-loop-closure/🦀️.rs"]
mod tracks_global_and_loop_closure;
pub use tracks_global_and_loop_closure::*;

// #region 🔖️Error

/// ⚠️ Error type for fallible structure-from-motion operations: two-view geometry, triangulation, PnP and incremental reconstruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfmError {
    TooFewFrames,
    DegenerateGeometry,
    InsufficientMatches,
    TriangulationFailed,
    PnpFailed,
    Convergence,
}

impl std::fmt::Display for SfmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewFrames => write!(f, "fewer than the minimum required number of frames"),
            Self::DegenerateGeometry => write!(f, "two-view geometry is degenerate"),
            Self::InsufficientMatches => write!(f, "not enough correspondences to fit a model"),
            Self::TriangulationFailed => write!(f, "triangulation failed to produce a valid point"),
            Self::PnpFailed => write!(f, "perspective-n-point pose estimation failed"),
            Self::Convergence => write!(f, "nonlinear refinement failed to converge"),
        }
    }
}

impl std::error::Error for SfmError {}

// #endregion 🔖️Error
