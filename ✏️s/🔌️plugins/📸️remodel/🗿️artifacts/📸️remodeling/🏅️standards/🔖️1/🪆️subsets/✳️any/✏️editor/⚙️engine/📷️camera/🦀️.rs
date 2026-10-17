//! 📷️ Camera models and calibration: pinhole, Brown-Conrady and fisheye distortion, rolling shutter, rigs, planar and self-calibration.

use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::MatD;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::levenberg_marquardt;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::LeastSquaresProblem;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::LmConfig;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::RobustLoss;

use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::pseudo_inverse;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::svd_nullvector;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::vec3d_cross;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::vec3d_length;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::Mat3d;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::So3;

#[path = "🔭️intrinsics/🦀️.rs"]
mod intrinsics;
pub use intrinsics::*;

#[path = "🧭️poses-and-rigs/🦀️.rs"]
mod poses_and_rigs;
pub use poses_and_rigs::*;

#[path = "📐️calibration/🦀️.rs"]
mod calibration;
pub use calibration::*;

// #region 🔖️Error
/// ⚠️ Error type for fallible camera-geometry operations: homography estimation, planar calibration, and self-calibration solves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraError {
    TooFewViews,
    DegenerateHomography,
    SingularSystem,
    Convergence,
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewViews => write!(f, "fewer than the minimum required number of calibration views"),
            Self::DegenerateHomography => write!(f, "homography estimation is degenerate"),
            Self::SingularSystem => write!(f, "linear system is singular or has no valid solution"),
            Self::Convergence => write!(f, "nonlinear refinement failed to converge"),
        }
    }
}

impl std::error::Error for CameraError {}
// #endregion 🔖️Error

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
