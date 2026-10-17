//! 🧮️ Linear algebra: dense/sparse matrices and vectors, exact elimination, decompositions and eigen solvers. The fixed-size render types `Vec3`/`Mat4` live in `semio-framework-geometry`.
//! Moved wholesale from `🧮️math/➕️algebra` per `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` wave M3d, whose sole-repo-wide-consumer census (verified true at the time) was outrun mid-ticket by wave M3a giving `➗️mathematical`'s `cas-internals` a second consumer of the generic `VecG`/`MatG` types. Wave FIXALG (same ticket) relocated `VecG`/`MatG` (and the `ExactElimination`/`Charpoly`/`Smith` inherent impls that must travel with them) to `semio_framework_number`'s `algebra` module — see that crate for the generic exact-linear-algebra types. Only `lll_reduce` below, a free function with no second consumer, stays here, built on `number::VecG`/`number::Rational`.

#[path = "🧱️dense-types/🦀️.rs"]
mod dense_types;
pub use dense_types::*;

#[path = "🔗️lattice-sparse-and-errors/🦀️.rs"]
mod lattice_sparse_and_errors;
pub use lattice_sparse_and_errors::*;

#[path = "📉️factorizations-and-eigen/🦀️.rs"]
mod factorizations_and_eigen;
pub use factorizations_and_eigen::*;

#[path = "🔁️iterative-and-svd/🦀️.rs"]
mod iterative_and_svd;
pub use iterative_and_svd::*;

#[path = "🌀️schur-and-roots/🦀️.rs"]
mod schur_and_roots;
pub use schur_and_roots::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️ExactTests
// 🚚 wave FIXALG: every test here but `lll_recovers_a_short_relation` moved to
// `semio_framework_number`'s `algebra` module tests, alongside the `MatG`/`VecG` types they exercise.
#[cfg(test)]
#[path = "🧪️tests/🔬️exact/🦀️.rs"]
mod exact_tests;
// #endregion 🔖️ExactTests
