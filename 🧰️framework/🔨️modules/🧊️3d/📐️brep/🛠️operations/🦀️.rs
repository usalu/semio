//! 🏗️ Constructive boundary-representation operations of the 3D kernel: the checked euler editors, primitives, the
//! staged-operation contract, sewing and transforms. Booleans, sweeps and offsets/blends are their own crates above it.
//!
//! Crate cut of `semio-framework-3d` (wave E4, `📓️2026-10-14-wave-e4-3d-architect-bim.md`).
#[path = "🔺️euler/🦀️.rs"]
pub mod euler;
#[path = "🧱️primitives/🦀️.rs"]
pub mod primitives;
#[path = "📦️point-hull/🦀️.rs"]
pub mod point_hull;
#[path = "🗜️prism-boundary/🦀️.rs"]
pub mod prism_boundary;

#[path = "📐️circular-prism/🦀️.rs"]
pub mod circular_prism;

#[path = "✂️skew-box-clip/🦀️.rs"]
pub mod skew_box_clip;

#[path = "📐️conical-frustum/🦀️.rs"]
pub mod conical_frustum;
#[path = "⏱️staged/🦀️.rs"]
pub mod staged;
#[path = "🧵️sew/🦀️.rs"]
pub mod sew;
#[path = "🔁️transform/🦀️.rs"]
pub mod transform;

//#region 🧪️TestSupport
#[cfg(test)]
pub(crate) use semio_framework_3d_testing_support as test_support;
//#endregion 🧪️TestSupport
