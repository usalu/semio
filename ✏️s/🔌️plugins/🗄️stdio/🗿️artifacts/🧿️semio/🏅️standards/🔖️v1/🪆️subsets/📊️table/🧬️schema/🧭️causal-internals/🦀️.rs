//! 🔀️ Causal inference: DAG/CPDAG/PAG models, d-separation, PC-stable/GES/LiNGAM/FCI discovery, backdoor/frontdoor/ID-algorithm identification, linear-Gaussian and discrete SCMs with interventional and counterfactual queries, and potential-outcome effect estimators.
//!
//! Variable indices are shared across [`CausalDag`]/[`Cpdag`] and `crate::schema::tabular_internals::Table`
//! columns throughout this crate: a DAG built over `n` named variables is expected to pair with a
//! table whose column `i` holds observations of variable `i`, for every `i`.
//!
//! Moved verbatim from `🧰️framework/🔨️modules/🧮️math/🔗️causal` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3c: Rust-only compute internals
//! backing `📊️table`'s inferences (no TS twin — this is algorithm, not boundary vocabulary). Depends
//! on sibling `📋️tabular-internals`/`🎲️probability-internals`/`📊️statistics-internals` and framework
//! `semio_framework_geometry::random` + (as of wave M3e, re-pointed after a CONCURRENT wave M3d
//! dissolved `semio_framework_math::algebra` entirely) sibling-subset
//! `🔢️value/🧬️schema/➕️algebra-internals` for `MatD`/`VecD` — see that file's own doc comment.

use semio_s_artifact_stdio_semio_table_probability::schema::probability_internals::Continuous;
use std::collections::{BTreeSet, HashMap, HashSet};

#[path = "⚠️errors-and-dag-model/🦀️.rs"]
mod errors_and_dag_model;
pub use errors_and_dag_model::*;

#[path = "🧬️cpdag-and-meek-rules/🦀️.rs"]
mod cpdag_and_meek_rules;
pub use cpdag_and_meek_rules::*;

#[path = "🧱️d-separation-and-ci-tests/🦀️.rs"]
mod d_separation_and_ci_tests;
pub use d_separation_and_ci_tests::*;

#[path = "🧮️discovery-helpers-and-pc-stable/🦀️.rs"]
mod discovery_helpers_and_pc_stable;
pub use discovery_helpers_and_pc_stable::*;

#[path = "📈️score-based-and-lingam-discovery/🦀️.rs"]
mod score_based_and_lingam_discovery;
pub use score_based_and_lingam_discovery::*;

#[path = "🪟️identification/🦀️.rs"]
mod identification;
pub use identification::*;

#[path = "📐️linear-gaussian-scm/🦀️.rs"]
mod linear_gaussian_scm;
pub use linear_gaussian_scm::*;

#[path = "🎲️discrete-scm/🦀️.rs"]
mod discrete_scm;
pub use discrete_scm::*;

#[path = "📊️effect-estimation/🦀️.rs"]
mod effect_estimation;
pub use effect_estimation::*;

#[path = "❓️what-if-query/🦀️.rs"]
mod what_if_query;
pub use what_if_query::*;

// #region 🔖️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️UnitTests
