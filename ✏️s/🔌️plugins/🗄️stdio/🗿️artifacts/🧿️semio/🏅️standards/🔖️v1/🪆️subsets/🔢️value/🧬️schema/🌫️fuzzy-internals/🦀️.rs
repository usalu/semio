//! 🌫️ Advanced fuzzy logic: type-1/IT2/IFS sets, multi-engine inference, neuro-fuzzy learning, clustering, multicriteria decision-making, and explainability.
//!
//! Moved verbatim from `🧰️framework/🔨️modules/🧮️math/🌫️fuzzy` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3e: Rust-only compute internals,
//! parked under `🔢️value` (its atomic operation — membership-degree evaluation, defuzzification — takes
//! a single scalar value) for lack of a better-evidenced home. Zero consumers anywhere in the repo at
//! move time, so NO `InferredField` is authored here (there is no snapshot shape to hang a genuine
//! `plan()`/`compute()` off without inventing one) — this is an honest "parked, not wired" placement.
//! If a plugin later owns fuzzy-rule-base content as authored data, THAT plugin should host the
//! snapshot and call into these compute internals, not stdio.
//!
//! Depends on the sibling `➕️algebra-internals` (`VecD`/`MatD` only — see that file's own doc comment
//! for why this duplicates rather than reuses `📸️remodel`'s copy of the same two types), not the
//! original `semio_framework_math::algebra`, which a CONCURRENT wave (M3d) dissolved out of the math
//! crate entirely before this file's dependency could be re-pointed at its new home.

#![allow(clippy::needless_range_loop, reason = "index-based numerics loops mirror the algebra_internals style for matrix and rule iteration")]
use semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD;
use semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::VecD;
use semio_framework_geometry::random::Rng;
use std::f64::consts::E;

#[path = "⚠️error-and-helpers/🦀️.rs"]
mod error_and_helpers;
pub use error_and_helpers::*;

#[path = "📈️membership-functions/🦀️.rs"]
mod membership_functions;
pub use membership_functions::*;

#[path = "🧮️fuzzy-sets/🦀️.rs"]
mod fuzzy_sets;
pub use fuzzy_sets::*;

#[path = "➗️norms-conorms-and-hedges/🦀️.rs"]
mod norms_conorms_and_hedges;
pub use norms_conorms_and_hedges::*;

#[path = "🔢️fuzzy-arithmetic-relations-and-possibility/🦀️.rs"]
mod fuzzy_arithmetic_relations_and_possibility;
pub use fuzzy_arithmetic_relations_and_possibility::*;

#[path = "🌐️universe-variables-and-rules/🦀️.rs"]
mod universe_variables_and_rules;
pub use universe_variables_and_rules::*;

#[path = "🎯️defuzzification-and-inference/🦀️.rs"]
mod defuzzification_and_inference;
pub use defuzzification_and_inference::*;

#[path = "🧠️adaptive-membership-and-anfis/🦀️.rs"]
mod adaptive_membership_and_anfis;
pub use adaptive_membership_and_anfis::*;

#[path = "📚️rule-learning-and-optimizers/🦀️.rs"]
mod rule_learning_and_optimizers;
pub use rule_learning_and_optimizers::*;

#[path = "🧩️clustering-and-decision-making/🦀️.rs"]
mod clustering_and_decision_making;
pub use clustering_and_decision_making::*;

#[path = "🛰️temporal-spatial-hybrid-and-fan/🦀️.rs"]
mod temporal_spatial_hybrid_and_fan;
pub use temporal_spatial_hybrid_and_fan::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
