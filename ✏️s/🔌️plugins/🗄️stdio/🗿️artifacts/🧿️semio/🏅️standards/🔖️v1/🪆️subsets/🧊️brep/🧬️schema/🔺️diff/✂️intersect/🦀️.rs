//! ✂️ Curve/curve, curve/surface, and surface/surface intersection math. Split into local
//! submodules under this directory (one per intersection kind, plus a `shared` numeric-plumbing
//! module) rather than the old single-file layout, once the surface/surface rewrite (exact p-curves
//! on both supports, coaxial meridian-profile solving, certified general marching) made one file
//! unwieldy — still "one compute subdir, not a 1:1 file mapping" per the `🔺️euler`/imprint
//! precedent, just mounted as real child files instead of inline `mod` blocks.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/{✂️int-cc,✂️int-cs,✂️int-ss}/🦀️.rs` in
//! ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL2, folded
//! into one file in `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`, and rewritten/split back
//! into files in `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave 2 (W2-A) — see
//! `📓️w2a-intersections.md` for the case table, p-curve conventions, and open items.

// #region 🔖️Submodules

#[path = "🤝️shared/🦀️.rs"]
mod shared;
#[path = "➰️curve-curve/🦀️.rs"]
pub mod curve_curve;
#[path = "➿️curve-surface/🦀️.rs"]
pub mod curve_surface;
#[path = "🏄️surface-surface/🦀️.rs"]
pub mod surface_surface;

// #endregion 🔖️Submodules

// #region 🔖️Reexports
pub use curve_curve::{intersect_curve_curve, CurveCurveHit};
pub use curve_surface::{intersect_curve_surface, CurveSurfaceHit};
pub use surface_surface::{intersect_surface_surface, IntCurve, IntCurveKind};
// #endregion 🔖️Reexports
