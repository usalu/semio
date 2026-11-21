//! ✂️ Curve/curve, curve/surface, and surface/surface intersection math. Split into local
//! submodules under this directory (one per intersection kind, plus an `intersection_numerics` numeric-plumbing
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
//!
//! Crate cut of `semio-framework-3d-brep-queries` (wave E4 source admission ceiling, `📓️2026-10-14-wave-e4-3d-architect-bim.md`).

// #region 🔖️Submodules

#[path = "📦️convex-box/🦀️.rs"]
pub mod convex_box;
#[path = "➰️curve-curve/🦀️.rs"]
pub mod curve_curve;
#[path = "➿️curve-surface/🦀️.rs"]
pub mod curve_surface;
#[path = "🤝️intersection-numerics/🦀️.rs"]
mod intersection_numerics;
#[path = "🏄️surface-surface/🦀️.rs"]
pub mod surface_surface;

// #endregion 🔖️Submodules

// #region 🔖️Reexports
pub use curve_curve::{intersect_curve_curve, intersect_line_line_unique, intersect_original_curve_curve, CurveCurveHit};
pub use curve_surface::{intersect_curve_surface, intersect_line_plane_unique, intersect_original_curve_plane, CurveSurfaceHit};

/// ⚖️ Allocation-free refusal from the bounded linear intersection equations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineIntersectionRefusal {
    InvalidTolerance,
    InvalidRange,
    InvalidSpline,
    ContactCapacity,
    DegenerateDirection,
    DegenerateConic,
    Infinite,
    NonFinite,
    UnresolvedContact,
}

impl LineIntersectionRefusal {
    pub const fn message(self) -> &'static str {
        match self {
            Self::InvalidTolerance => "tolerance must be positive and finite",
            Self::InvalidRange => "intersection trim must have finite ordered endpoints within the source domain",
            Self::InvalidSpline => "plane intersection requires a continuous spline with finite knots and positive finite weights",
            Self::ContactCapacity => "plane intersection has more than four distinct contacts",
            Self::DegenerateDirection => "zero-length line direction",
            Self::DegenerateConic => "conic intersection requires finite radii larger than tolerance",
            Self::Infinite => "infinitely many intersections",
            Self::NonFinite => "intersection requires finite arithmetic",
            Self::UnresolvedContact => "contact could not be isolated within the admitted precision",
        }
    }
}

impl From<LineIntersectionRefusal> for semio_framework_3d_brep_representation::error::IntersectError {
    fn from(error: LineIntersectionRefusal) -> Self {
        match error {
            LineIntersectionRefusal::Infinite => Self::Tangent,
            _ => Self::Degenerate(error.message().into()),
        }
    }
}
pub use surface_surface::{intersect_original_analytic_surfaces, intersect_plane_plane_curve, intersect_surface_surface, ExactSurfaceRefusal, IntCurve, IntCurveKind};
// #endregion 🔖️Reexports
