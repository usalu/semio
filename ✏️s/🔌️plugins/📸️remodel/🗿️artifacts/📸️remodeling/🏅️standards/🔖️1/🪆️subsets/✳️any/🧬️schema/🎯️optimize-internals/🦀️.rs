//! 🎯️ Nonlinear least squares and robust estimation: Gauss-Newton, Levenberg-Marquardt, Schur-complement bundle solvers, robust losses and RANSAC consensus.
//! Moved wholesale from `🧮️math/🎯️optimize` — 📸️remodeling is its sole repo-wide consumer (verified: symbol-level grep of every exported type/fn across the whole tree outside math and remodeling returned zero hits), per `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` wave M3d.

use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::cholesky;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::cholesky_solve;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::weighted_normal_equations;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::MatD;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD;
use geometry::random::Rng;
use std::collections::HashMap;

#[path = "🧮️problem-and-robust-losses/🦀️.rs"]
mod problem_and_robust_losses;
pub use problem_and_robust_losses::*;

#[path = "📐️newton-and-levenberg-marquardt/🦀️.rs"]
mod newton_and_levenberg_marquardt;
pub use newton_and_levenberg_marquardt::*;

#[path = "🕸️schur-bundle/🦀️.rs"]
mod schur_bundle;
pub use schur_bundle::*;

#[path = "🎲️consensus-and-scalar-search/🦀️.rs"]
mod consensus_and_scalar_search;
pub use consensus_and_scalar_search::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
