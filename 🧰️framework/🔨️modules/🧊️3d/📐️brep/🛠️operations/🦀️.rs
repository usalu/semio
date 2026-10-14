//! 🏗️ Constructive boundary-representation operations of the 3D kernel: the checked euler editors, primitives, the
//! staged-operation contract, sewing and transforms. Booleans, sweeps and offsets/blends are their own crates above it.
//!
//! Crate cut of `semio-framework-3d` (wave E4, `📓️2026-10-14-wave-e4-3d-architect-bim.md`).
#[path = "🔺️euler/🦀️.rs"]
pub mod euler;
#[path = "🧱️primitives/🦀️.rs"]
pub mod primitives;
#[path = "⏱️staged/🦀️.rs"]
pub mod staged;
#[path = "🧵️sew/🦀️.rs"]
pub mod sew;
#[path = "🔁️transform/🦀️.rs"]
pub mod transform;

//#region 🧪️TestSupport
#[cfg(test)]
#[path = "../../🧪️tests/🧰️support/🦀️.rs"]
pub(crate) mod test_support;
//#endregion 🧪️TestSupport
