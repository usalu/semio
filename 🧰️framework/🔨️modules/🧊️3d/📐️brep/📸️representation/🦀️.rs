//! 📸️ Boundary-representation topology of the 3D kernel: generational arena ids, the `Body` topology with its seed,
//! history, reachability and compaction, kernel errors and the neutral kernel contract (`🔖️contract`). Polynomials,
//! vectors, tolerances, curves and surfaces are their own crates (`semio-framework-3d-brep-{polynomial,vector,tolerance,curve,surface}`).
//!
//! Crate cut of `semio-framework-3d` (wave E4, `📓️2026-10-14-wave-e4-3d-architect-bim.md`).

extern crate semio_framework_value_derive as value_derive;
#[path = "🏟️arena/🦀️.rs"]
pub mod arena;
#[path = "🔖️contract/🦀️.rs"]
pub mod contract;
#[path = "🚨️error/🦀️.rs"]
pub mod error;
#[path = "🕸️topology/🦀️.rs"]
pub mod topology;

//#region 🧪️TestSupport
#[cfg(test)]
#[path = "../../🧪️tests/🧰️support/🦀️.rs"]
pub(crate) mod test_support;
//#endregion 🧪️TestSupport
