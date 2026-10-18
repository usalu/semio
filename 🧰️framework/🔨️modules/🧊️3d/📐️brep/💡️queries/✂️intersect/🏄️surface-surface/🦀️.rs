//! 🏄 Surface/surface intersection emitting [`IntCurve`]: a 3D curve plus a p-curve on *each*
//! support, an exactness tag, and the parameter domain they're all evaluable over.
//!
//! Exact analytic cases: plane/plane (line), plane/cylinder (circle/ellipse/one-or-two
//! lines/tangency), plane/cone (circle/ellipse — parabola/hyperbola fall to the general path,
//! see `📓️w2a-intersections.md`), plane/sphere, plane/torus (perpendicular-to-axis and
//! axis-containing sections only — oblique/Villarceau sections fall to the general path),
//! sphere/sphere, cylinder/cylinder (parallel axes → lines; equal-radius intersecting axes →
//! Steinmetz ellipses via the bisector-plane construction), and every coaxial pair among
//! cylinder/cone/sphere/torus (via [`coaxial_case`]'s meridian-profile intersection). Every other
//! pair traces via [`general_marching`]: quadtree seed finding, a tangent-predictor/Newton-corrector
//! march with loop-closure and domain-border termination, then a certified NURBS + p-curve fit.
//!
//! p-curves are built by [`build_pcurve`]: closed-form wherever the geometry actually admits one
//! (a plane's own `(u, v)` is always affine; an axisymmetric surface's `(u, v)` is affine in the
//! curve's own parameter exactly when the curve's plane/axis aligns with the surface's own axis),
//! and a certified-error NURBS fit (`sample_and_fit_pcurve`) otherwise — in which case `curve3`
//! can still be `Exact` while [`IntCurveKind`] is `Fitted`.
//!
//! See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`, majority-rewritten in
//! `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave 2 (W2-A) — see `📓️w2a-intersections.md`.
/// 🗺️ Parameter bounds and spatial bounds for a surface patch.
pub type SurfacePatchBounds = (f64, f64, f64, f64, (Pnt3, Pnt3));

/// 📍 Intersection point with parameters on both surfaces.
pub type SurfaceIntersectionSample = (Pnt3, f64, f64, f64, f64);

use semio_framework_3d_brep_representation::contract::ParamDomain;
use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_representation::error::IntersectError;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec2, Vec3};

// #region 🔖️Api

/// 🏄 Whether an [`IntCurve`] is closed-form on both supports, or a certified NURBS fit.
#[derive(Clone, Debug, PartialEq)]
pub enum IntCurveKind {
    Exact,
    Fitted { max_error: f64 },
}

/// 🏄 One continuous branch of a surface/surface intersection: the 3D curve, its p-curve on each
/// support (in that support's own `(u, v)`), the shared parameter domain all three are evaluable
/// over, and [`IntCurveKind`].
#[derive(Clone, Debug, PartialEq)]
pub struct IntCurve {
    pub curve3: Curve3,
    pub pcurve_a: Curve2,
    pub pcurve_b: Curve2,
    pub domain: ParamDomain,
    pub kind: IntCurveKind,
}

/// 🏄 Intersect two parametric surfaces within `tol`, producing exact-form branches wherever the
/// pair admits one and a certified NURBS trace otherwise.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn intersect_surface_surface(a: &Surface, b: &Surface, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    if !(tol.is_finite() && tol > 0.0) {
        return Err(IntersectError::Degenerate("tolerance must be positive and finite".into()));
    }
    match (a, b) {
        (Surface::Plane { frame: fa }, Surface::Plane { frame: fb }) => plane_plane(fa, fb, tol),
        (Surface::Plane { frame }, Surface::Cylinder { frame: cf, radius }) => plane_cylinder(frame, cf, *radius, tol),
        (Surface::Cylinder { frame: cf, radius }, Surface::Plane { frame }) => plane_cylinder(frame, cf, *radius, tol).map(swap_ab),
        (Surface::Plane { frame }, Surface::Cone { frame: cf, half_angle }) => plane_cone(frame, cf, *half_angle, tol),
        (Surface::Cone { frame: cf, half_angle }, Surface::Plane { frame }) => plane_cone(frame, cf, *half_angle, tol).map(swap_ab),
        (Surface::Plane { frame }, Surface::Sphere { frame: sf, radius }) => plane_sphere(frame, sf, *radius, tol),
        (Surface::Sphere { frame: sf, radius }, Surface::Plane { frame }) => plane_sphere(frame, sf, *radius, tol).map(swap_ab),
        (Surface::Plane { frame }, Surface::Torus { frame: tf, major_radius, minor_radius }) => Ok(plane_torus(frame, tf, *major_radius, *minor_radius, tol)),
        (Surface::Torus { frame: tf, major_radius, minor_radius }, Surface::Plane { frame }) => Ok(swap_ab(plane_torus(frame, tf, *major_radius, *minor_radius, tol))),
        (Surface::Sphere { frame: fa, radius: ra }, Surface::Sphere { frame: fb, radius: rb }) => sphere_sphere(fa, *ra, fb, *rb, tol),
        (Surface::Cylinder { .. }, Surface::Cylinder { .. }) => cylinder_cylinder(a, b, tol),
        (Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. }, Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. }) => {
            coaxial_case(a, b, tol).unwrap_or_else(|| Ok(general_marching(a, b, tol)))
        }
        _ => Ok(general_marching(a, b, tol)),
    }
}

/// 🏄 Swaps the two supports of every branch (used when the caller's `(a, b)` order is the mirror
/// of a case function written for a fixed `(plane, other)`/`(cylinder_a, cylinder_b)` order).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn swap_ab(curves: Vec<IntCurve>) -> Vec<IntCurve> {
    curves.into_iter().map(|c| IntCurve { curve3: c.curve3, pcurve_a: c.pcurve_b, pcurve_b: c.pcurve_a, domain: c.domain, kind: c.kind }).collect()
}

// #endregion 🔖️Api

#[path = "📈️pcurves-and-curve-finishing/🦀️.rs"]
mod pcurves_and_curve_finishing;
use pcurves_and_curve_finishing::finish_intcurve;

#[path = "✈️plane-pairs/🦀️.rs"]
mod plane_pairs;
use plane_pairs::{plane_cone, plane_cylinder, plane_plane, plane_sphere, plane_torus, sphere_sphere};

#[path = "🌀️conics-coaxial-and-cylinders/🦀️.rs"]
mod conics_coaxial_and_cylinders;
use conics_coaxial_and_cylinders::{coaxial_case, cylinder_cylinder};
#[cfg(test)]
use conics_coaxial_and_cylinders::steinmetz;

#[path = "🧭️general-marching/🦀️.rs"]
mod marching_tracer;
use marching_tracer::general_marching;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
