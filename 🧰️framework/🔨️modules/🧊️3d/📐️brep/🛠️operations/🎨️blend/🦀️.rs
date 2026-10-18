//! 🎨️ Exact ANALYTIC rolling-ball fillet and cutting-plane chamfer via topology surgery.
//!
//! Every selected edge is replaced by a blend patch carried on its own analytic support — a
//! [`Surface::Cylinder`] for a constant-radius fillet between two planes, a [`Surface::Torus`] for
//! one between a plane and a coaxial cylinder, a [`Surface::Cone`] for a variable-radius fillet or
//! for a chamfer of a cylinder's cap edge, a [`Surface::Plane`] for a chamfer between two planes —
//! and every vertex where ALL incident edges are blended gets its own corner patch: a
//! [`Surface::Sphere`] octant for a fillet, a planar triangle for a chamfer.
//!
//! Every trim curve is an ISO-LINE of the patch it bounds (a ruling, an arc, a straight cut line),
//! so every p-curve stored here is derived in closed form and VERIFIED against its own 3D curve,
//! never fitted — the one exception, a variable fillet's elliptical end trim on its cone, is
//! certified to `1e-8` and named as such at the call site. That is what turned a 12-edge box
//! fillet from minutes of `Surface::project_curve` sampling into sub-millisecond, and what makes
//! the rounded-box Minkowski closed form come out exactly.
//!
//! The surgery is non-destructive: the input solid is left intact and every vertex/edge/loop/face
//! of the result is freshly minted (curve and surface geometry is shared, being immutable), so a
//! graph that feeds one solid into several blend nodes keeps working and `validate_body` stays
//! silent over the whole body.
//!
//! Exactness preconditions, each an explicit `Err` rather than a silent approximation: a blended
//! edge must be CONVEX and shared by exactly two faces; a vertex a blend reaches must be trihedral
//! with either exactly one or all of its edges blended; the face closing a partially blended
//! vertex must be planar; a spherical corner needs one of its three face normals perpendicular to
//! the other two. See `📓️blend-kernel-2026-09-09.md`.
//!
//! Ticket `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave W2-D, rewritten onto analytic
//! supports in ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

use {semio_framework_3d_brep_construction::euler::{add_face, add_shell, add_solid, make_edge, make_vertex}, semio_framework_3d_brep_representation::topology::make_loop};
use crate::exact_pcurve;
use semio_framework_3d_brep_construction::staged::{drive_solid, Plan, StageOutput, StageProgress, StageStep, StagedOperation};
use semio_framework_3d_brep_queries::mass_properties::edge_length;
use semio_framework_3d_brep_representation::arena::{ArenaId, CoedgeId, EdgeId, FaceId, SolidId, SurfaceId, VertexId};
use semio_framework_3d_brep_curve::bspline::KnotVector;
use semio_framework_3d_brep_curve::curve_ops::{interpolate_curve, ParamMethod};
use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_representation::error::KernelError;
use semio_framework_3d_brep_surface::surface_ops::closest_uv;
use semio_framework_3d_brep_surface::{IsoDirection, Surface};
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::Body;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec2, Vec3};

/// 🎨️ Positional agreement two exactly-derived quantities must reach (a trim endpoint against the
/// curve it is meant to lie on, a closed-form p-curve against its own 3D curve). Three orders of
/// magnitude under [`Tol::DEFAULT`]'s `1e-7`, so anything accepted here also clears
/// `validate_body`'s same-parameter check with room to spare.
const BLEND_EPS: f64 = 1e-10;
/// 🎨️ The looser bound the ONE numerically fitted p-curve in this file (a variable fillet's
/// elliptical end trim on its cone) must still certify against.
const FIT_EPS: f64 = 1e-8;
/// 🎨️ Direction/parallelism threshold for the analytic preconditions (parallel axes, perpendicular
/// normals, degenerate dihedrals).
const DIR_EPS: f64 = 1e-9;

#[path = "✅️blend-validation-and-geometry/🦀️.rs"]
mod blend_validation_and_geometry;
use blend_validation_and_geometry::{curve_param_at, face_plane, line_line_point, line_plane_point, outward_normal_at, solid_edge_set, unit, validate_blend_request};

#[path = "📈️blend-pcurves/🦀️.rs"]
mod blend_pcurves;
use blend_pcurves::blend_pcurve;

#[path = "🩹️blend-patches/🦀️.rs"]
mod blend_patches;
use blend_patches::{BlendSpec, EndTrim, Patch, build_patch, edge_faces};
#[cfg(test)]
use blend_patches::edge_two_faces;

#[path = "🔺️blend-corners/🦀️.rs"]
mod blend_corners;
use blend_corners::{Corner, build_corner, conic_end_trim, corner_surface};

#[path = "⏱️blend-job/🦀️.rs"]
mod blend_job;
use blend_job::{Member, Minted, attach_with_pcurves};
pub use blend_job::BlendJob;

#[path = "🔪️face-surgery/🦀️.rs"]
mod face_surgery;
use face_surgery::{build_corner_face, build_patch_face, mint_carried_edge, rebuild_face, trim_patch};

// #region 🔖️Fillet

/// 🎨️ Constant-radius rolling-ball fillet on `edges` of `solid` — exact analytic topology surgery,
/// see this module's own docstring.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fillet_edges(body: &mut Body, solid: SolidId, edges: &[EdgeId], radius: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    drive_solid(&mut BlendJob::fillet(body, solid, edges, radius)?, body, rec)
}

/// 🎨️ Linearly varying fillet radius `r0 → r1` along a single `edge`: the ball centres still run
/// along a line, so the envelope is exactly a cone of revolution — see [`cone_fillet_patch`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fillet_variable(body: &mut Body, solid: SolidId, edge: EdgeId, r0: f64, r1: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    drive_solid(&mut BlendJob::variable_fillet(body, solid, edge, r0, r1)?, body, rec)
}

// #endregion 🔖️Fillet

// #region 🔖️Chamfer

/// 🎨️ Asymmetric chamfer (`d1` along the first adjacent face, `d2` along the second) on `edges` of
/// `solid` — a planar cut face between two planar faces, an exact cone frustum at a cylinder's cap.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chamfer_edges(body: &mut Body, solid: SolidId, edges: &[EdgeId], d1: f64, d2: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    drive_solid(&mut BlendJob::chamfer(body, solid, edges, d1, d2)?, body, rec)
}

// #endregion 🔖️Chamfer

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
