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

use crate::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::standards::v1::subsets::brep::schema::diff::offset::exact_pcurve;
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::edge_length;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, CoedgeId, EdgeId, FaceId, SolidId, SurfaceId, VertexId};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector;
use crate::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::{interpolate_curve, ParamMethod};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::{Curve2, Curve3};
use crate::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_uv;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::{IsoDirection, Surface};
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec2, Vec3};

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

// #region 🔖️Validate

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_solid(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity("solid".into()));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_edge_set(body: &Body, solid: SolidId) -> BTreeSet<EdgeId> {
    let mut edges = BTreeSet::new();
    for face in body.solid_faces(solid) {
        for coedge in body.face_coedges(face) {
            if let Some(c) = body.coedges.get(coedge) {
                edges.insert(c.edge);
            }
        }
    }
    edges
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn min_adjacent_edge_length(body: &Body, edge: EdgeId) -> Result<f64, KernelError> {
    let ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let mut min_len = f64::INFINITY;
    for vid in [ent.v0, ent.v1] {
        for adj in body.vertex_edges(vid) {
            if adj == edge {
                continue;
            }
            let len = edge_length(body, adj)?;
            if len > 0.0 {
                min_len = min_len.min(len);
            }
        }
    }
    if !min_len.is_finite() {
        return Err(KernelError::InvalidInput("edge has no measurable adjacent edges".into()));
    }
    Ok(min_len)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_blend_request(body: &Body, solid: SolidId, edges: &[EdgeId], amount: f64) -> Result<(), KernelError> {
    require_solid(body, solid)?;
    if edges.is_empty() {
        return Err(KernelError::InvalidInput("blend requires at least one edge".into()));
    }
    if !(amount.is_finite() && amount > 0.0) {
        return Err(KernelError::InvalidInput("blend radius/distance must be positive".into()));
    }
    let solid_edges = solid_edge_set(body, solid);
    for &edge in edges {
        if !solid_edges.contains(&edge) {
            return Err(KernelError::MissingEntity(format!("edge {edge:?} is not on solid")));
        }
        let min_adj = min_adjacent_edge_length(body, edge)?;
        if amount >= min_adj {
            return Err(KernelError::InvalidInput(format!("blend amount {amount} must be smaller than min adjacent edge length {min_adj}")));
        }
    }
    Ok(())
}

// #endregion 🔖️Validate

// #region 🔖️Geometry

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unit(v: Vec3, what: &str) -> Result<Vec3, KernelError> {
    v.normalized().ok_or_else(|| KernelError::Operation(format!("blend: {what} is degenerate")))
}

/// 🎨️ Where two COPLANAR, non-parallel lines meet — `None` when they are parallel or skew, which
/// is the precondition every corner construction here rejects rather than approximates.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn line_line_point(a0: Pnt3, da: Vec3, b0: Pnt3, db: Vec3) -> Option<Pnt3> {
    let n = da.cross(db);
    let nn = n.norm_sq();
    if nn <= DIR_EPS * DIR_EPS * da.norm_sq() * db.norm_sq() {
        return None;
    }
    let w = b0 - a0;
    let p = a0 + da * (w.cross(db).dot(n) / nn);
    let q = b0 + db * (w.cross(da).dot(n) / nn);
    if p.distance(q) > 1e-7 {
        return None;
    }
    Some(p)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn line_plane_point(origin: Pnt3, dir: Vec3, plane_point: Pnt3, plane_normal: Vec3) -> Option<Pnt3> {
    let den = dir.dot(plane_normal);
    if den.abs() <= DIR_EPS * dir.norm().max(1.0) {
        return None;
    }
    Some(origin + dir * ((plane_point - origin).dot(plane_normal) / den))
}

/// 🎨️ The curve parameter at `p`, shifted by whole periods into `range` for a periodic curve and
/// VERIFIED against the curve itself — so a point merely near the curve can never become a trim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn curve_param_at(curve: &Curve3, p: Pnt3, range: (f64, f64)) -> Option<f64> {
    let raw = match curve {
        Curve3::Line { origin, dir } => {
            let dd = dir.norm_sq();
            if dd <= 0.0 {
                return None;
            }
            (p - *origin).dot(*dir) / dd
        }
        Curve3::Circle { frame, .. } | Curve3::Ellipse { frame, .. } => {
            let local = frame.to_local(p);
            let mut t = local.y.atan2(local.x);
            let (lo, hi) = (range.0.min(range.1), range.0.max(range.1));
            while t < lo - 1e-12 {
                t += TAU;
            }
            while t > hi + 1e-12 {
                t -= TAU;
            }
            t
        }
        Curve3::Nurbs { .. } => return None,
    };
    if curve.eval(raw).distance(p) > 1e-7 {
        return None;
    }
    Some(raw)
}

/// 🎨️ A face's OUTWARD normal at `p`: the surface's own `du × dv`, negated when the face is
/// `flipped`. Constant over a planar face, radial over a cylindrical one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn outward_normal_at(body: &Body, face: FaceId, p: Pnt3) -> Result<Vec3, KernelError> {
    let fd = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = body.surfaces.get(fd.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
    let mut normal = match surface {
        Surface::Plane { frame } => frame.z,
        other => {
            let uv = closest_uv(other, other.domain(), p, BLEND_EPS);
            other.normal(uv.u, uv.v).ok_or_else(|| KernelError::Operation("blend: the face normal is degenerate at this edge".into()))?
        }
    };
    if fd.flipped {
        normal = -normal;
    }
    unit(normal, "face normal")
}

/// 🎨️ A planar face's `(point, outward normal)`, or `Err` — the precondition every corner and cap
/// trim states explicitly instead of guessing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_plane(body: &Body, face: FaceId) -> Result<(Pnt3, Vec3), KernelError> {
    let fd = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = body.surfaces.get(fd.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
    let Surface::Plane { frame } = surface else {
        return Err(KernelError::Operation("blend: this construction needs a planar face".into()));
    };
    Ok((frame.origin, if fd.flipped { -frame.z } else { frame.z }))
}

// #endregion 🔖️Geometry

// #region 🔖️Pcurve

/// 🎨️ The exact p-curve of a curve whose `(u, v)` image is a straight, AFFINELY parametrized line
/// — which nearly every boundary this file mints is: an iso-line of its own analytic patch (a
/// cylinder's ruling and its cross circle, a torus's tube arc and azimuth circle, a sphere's
/// meridian and equator, a cone's ruling) or any straight line on a plane. Both ends plus interior
/// probes (which is what a full-circle boundary's periodic unwrapping needs) are inverted through
/// the certified analytic [`closest_uv`], the affine map through them is formed, and the candidate
/// is VERIFIED against the real curve — so a boundary whose parameter is not affine in the
/// surface's (a circle on a plane, whose image is a conic) reports `None` instead of quietly
/// returning a wrong answer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn affine_uv_pcurve(surface: &Surface, curve: &Curve3, range: (f64, f64)) -> Option<Curve2> {
    let span = range.1 - range.0;
    if span.abs() <= 1e-15 {
        return None;
    }
    let probes = 8usize;
    let mut uvs: Vec<(f64, f64, bool)> = Vec::with_capacity(probes + 1);
    for i in 0..=probes {
        let t = range.0 + span * i as f64 / probes as f64;
        let raw = closest_uv(surface, surface.domain(), curve.eval(t), BLEND_EPS);
        uvs.push((raw.u, raw.v, surface.is_degenerate_uv(raw.u, raw.v)));
    }
    // 🧭️ A pole's `u` is meaningless (its inversion returns an arbitrary branch), and a boundary
    // ENDING at one — every meridian of a spherical corner does — would otherwise be fitted through
    // that arbitrary value and come out slanted. Carry `u` in from the nearest sample that has one,
    // in both directions, before anything is unwrapped or fitted.
    let first_good = uvs.iter().position(|entry| !entry.2)?;
    for i in (0..first_good).rev() {
        uvs[i].0 = uvs[i + 1].0;
    }
    for i in first_good + 1..uvs.len() {
        if uvs[i].2 {
            uvs[i].0 = uvs[i - 1].0;
        }
    }
    for i in 1..uvs.len() {
        let (pu, pv) = (uvs[i - 1].0, uvs[i - 1].1);
        if surface.is_u_periodic() {
            uvs[i].0 = unwrap_near(uvs[i].0, pu);
        }
        if surface.is_v_periodic() {
            uvs[i].1 = unwrap_near(uvs[i].1, pv);
        }
    }
    let (u0, v0, _) = uvs[0];
    let (u1, v1, _) = uvs[probes];
    let du = (u1 - u0) / span;
    let dv = (v1 - v0) / span;
    let candidate = Curve2::Line { origin: Pnt2::new(u0 - du * range.0, v0 - dv * range.0), dir: Vec2::new(du, dv) };
    if pcurve_deviation(surface, &candidate, curve, range) <= BLEND_EPS {
        Some(candidate)
    } else {
        None
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pcurve_deviation(surface: &Surface, pcurve: &Curve2, curve: &Curve3, range: (f64, f64)) -> f64 {
    let span = range.1 - range.0;
    (0..=32)
        .map(|i| {
            let t = range.0 + span * f64::from(i) / 32.0;
            let uv = pcurve.eval(t);
            surface.eval(uv.x, uv.y).distance(curve.eval(t))
        })
        .fold(0.0f64, f64::max)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_near(x: f64, near: f64) -> f64 {
    let mut y = x;
    while y - near > PI {
        y -= TAU;
    }
    while near - y > PI {
        y += TAU;
    }
    y
}

/// 🎨️ A curve that LIES IN a plane projects into that plane's `(u, v)` exactly, parameter for
/// parameter: a line maps to a line for any orientation, and a rational B-spline maps to the same
/// spline over its own control points' plane coordinates (every point of a rational curve is a
/// convex-weighted combination of its controls, so coplanar controls give a coplanar curve and the
/// projection commutes with the evaluation). Conics with an independent frame are left to
/// [`exact_pcurve`]'s own signed-radius ellipse shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn planar_pcurve(surface: &Surface, curve: &Curve3) -> Option<Curve2> {
    let Surface::Plane { frame } = surface else { return None };
    match curve {
        Curve3::Line { origin, dir } => {
            let local = frame.to_local(*origin);
            let direction = frame.to_local_vector(*dir);
            Some(Curve2::Line { origin: Pnt2::new(local.x, local.y), dir: Vec2::new(direction.x, direction.y) })
        }
        Curve3::Nurbs { knots, controls, weights } => {
            let mut planar = Vec::with_capacity(controls.len());
            for control in controls {
                let local = frame.to_local(*control);
                if local.z.abs() > 1e-9 {
                    return None;
                }
                planar.push(Pnt2::new(local.x, local.y));
            }
            Some(Curve2::Nurbs { knots: knots.clone(), controls: planar, weights: weights.clone() })
        }
        _ => None,
    }
}

/// 🎨️ Every p-curve this file stores. [`affine_uv_pcurve`] and [`planar_pcurve`] answer in closed
/// form; [`exact_pcurve`] adds the conic-on-a-plane shortcut; only then — and only for a curve no
/// closed form covers, which in practice is a variable fillet's elliptical end trim on its own cone
/// — is `Surface::project_curve`'s numeric fit used, and its result is CERTIFIED against the real
/// curve before it is accepted. A pair no route can answer is refused, never approximated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn blend_pcurve(surface: &Surface, curve: &Curve3, range: (f64, f64)) -> Result<Curve2, KernelError> {
    if let Some(pcurve) = affine_uv_pcurve(surface, curve, range) {
        return Ok(pcurve);
    }
    if let Some(pcurve) = planar_pcurve(surface, curve) {
        return Ok(pcurve);
    }
    if let Some(pcurve) = exact_pcurve(surface, curve, range) {
        if pcurve_deviation(surface, &pcurve, curve, range) <= BLEND_EPS {
            return Ok(pcurve);
        }
    }
    // 🧭️ The ONE interpolated p-curve in this engine, deliberately reachable only here: a variable
    // fillet's cap plane cuts its cone in a conic whose `(u, v)` image is transcendental in the
    // conic's own parameter, so no closed form exists. It is affordable exactly because the support
    // is analytic — its inversion is a closed form, unlike the NURBS patches whose sampled fits
    // this rewrite exists to eliminate.
    if !matches!((surface, curve), (Surface::Cone { .. }, Curve3::Nurbs { .. })) {
        return Err(KernelError::Operation(format!("blend: no closed-form p-curve exists for {} / {}", surface_kind(surface), curve_kind(curve))));
    }
    if let Some(interpolated) = interpolated_uv_pcurve(surface, curve, range) {
        return Ok(interpolated);
    }
    Err(KernelError::Operation(format!("blend: no p-curve within tolerance exists for {} / {}", surface_kind(surface), curve_kind(curve))))
}

/// 🎨️ A p-curve interpolated through exactly inverted `(u, v)` samples, parametrized by the 3D
/// curve's OWN parameter — which is what the same-parameter law requires and what
/// `Surface::project_curve`'s fit cannot give (it re-parametrizes centripetally, so its result is
/// only ever valid for a caller that re-parametrizes with it). Densified until the deviation
/// certifies, so the answer is exact to `1e-10` in practice and refused outright above `1e-8`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn interpolated_uv_pcurve(surface: &Surface, curve: &Curve3, range: (f64, f64)) -> Option<Curve2> {
    let span = range.1 - range.0;
    let mut best: Option<Curve2> = None;
    for count in [16usize, 32, 64, 128, 256] {
        let mut samples: Vec<Pnt3> = Vec::with_capacity(count + 1);
        let mut previous: Option<(f64, f64)> = None;
        for i in 0..=count {
            let t = range.0 + span * i as f64 / count as f64;
            let raw = closest_uv(surface, surface.domain(), curve.eval(t), BLEND_EPS);
            let (mut u, mut v) = (raw.u, raw.v);
            if let Some((pu, pv)) = previous {
                if surface.is_u_periodic() {
                    u = unwrap_near(u, pu);
                }
                if surface.is_v_periodic() {
                    v = unwrap_near(v, pv);
                }
            }
            previous = Some((u, v));
            samples.push(Pnt3::new(u, v, 0.0));
        }
        let fitted = interpolate_curve(&samples, 3, ParamMethod::Uniform, None, false)?;
        let knots = KnotVector::new(fitted.knots.knots.iter().map(|k| range.0 + span * k).collect(), fitted.knots.degree, fitted.controls.len())?;
        let candidate = Curve2::Nurbs { knots, controls: fitted.controls.iter().map(|p| Pnt2::new(p.x, p.y)).collect(), weights: fitted.weights };
        let deviation = pcurve_deviation(surface, &candidate, curve, range);
        if deviation <= BLEND_EPS {
            return Some(candidate);
        }
        if deviation <= FIT_EPS {
            best = Some(candidate);
        }
    }
    best
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn surface_kind(surface: &Surface) -> &'static str {
    match surface {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Nurbs { .. } => "nurbs surface",
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn curve_kind(curve: &Curve3) -> &'static str {
    match curve {
        Curve3::Line { .. } => "line",
        Curve3::Circle { .. } => "circle",
        Curve3::Ellipse { .. } => "ellipse",
        Curve3::Nurbs { .. } => "nurbs curve",
    }
}

// #endregion 🔖️Pcurve

// #region 🔖️Patch

/// 🎨️ What one blend call builds along every selected edge.
#[derive(Clone, Copy, Debug, PartialEq)]
enum BlendSpec {
    /// 🎨️ Rolling ball of one radius.
    Fillet(f64),
    /// 🎨️ Rolling ball whose radius runs linearly from the edge's start to its end.
    VariableFillet(f64, f64),
    /// 🎨️ Cutting plane at the first distance along `faces[0]` and the second along `faces[1]`.
    Chamfer(f64, f64),
}

impl BlendSpec {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fillet_radius(&self) -> Option<f64> {
        match self {
            BlendSpec::Fillet(r) => Some(*r),
            _ => None,
        }
    }
}

/// 🎨️ How one end of a patch is trimmed: at an iso-line of the run direction (every constant-radius
/// case — the cap plane there is perpendicular to the patch's own axis, so the trim IS a cross
/// section), or, for a variable-radius cone whose axis tilts away from the edge, at the exact conic
/// its cap plane cuts out of it.
#[derive(Clone, Debug)]
enum EndTrim {
    Iso(f64),
    Conic(Curve3),
}

/// 🎨️ One blend patch: its analytic support plus the parametric addresses of its four boundaries.
/// `cross` is the parameter held fixed along each face's TANGENCY curve (so that curve is
/// `isocurve(cross_dir, cross[i])`, parametrized by the run parameter); the patch's two END curves
/// run the other way, from `cross[0]` to `cross[1]`. Every analytic kind built here fits that
/// shape, which is what lets one assembly routine serve fillets and chamfers, cylinders, cones,
/// tori and planes alike.
struct Patch {
    edge: EdgeId,
    faces: [FaceId; 2],
    surface: Surface,
    cross_dir: IsoDirection,
    cross: [f64; 2],
    /// 🎨️ `true` for a band closed in the run direction (a blended circular cap edge), whose two
    /// "ends" are one seam curve traversed twice.
    closed: bool,
    /// 🎨️ The original edge's endpoint vertices in run order; both are the seam vertex for a
    /// closed band.
    end_vertex: [VertexId; 2],
    /// 🎨️ The run range each face's tangency curve keeps, after trimming. Identical for both faces
    /// except on a variable-radius cone.
    run: [(f64, f64); 2],
    ends: [EndTrim; 2],
}

impl Patch {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn run_dir(&self) -> IsoDirection {
        match self.cross_dir {
            IsoDirection::U => IsoDirection::V,
            IsoDirection::V => IsoDirection::U,
        }
    }
    /// 🎨️ The tangency curve on `faces[i]`, parametrized by the run parameter.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn tangency(&self, i: usize) -> Curve3 {
        self.surface.isocurve(self.cross_dir, self.cross[i])
    }
    /// 🎨️ The end curve at `slot` and its own parameter range, running from `faces[0]`'s tangency
    /// point to `faces[1]`'s.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn end_curve(&self, slot: usize) -> (Curve3, (f64, f64)) {
        match &self.ends[slot] {
            EndTrim::Iso(w) => (self.surface.isocurve(self.run_dir(), *w), (self.cross[0], self.cross[1])),
            EndTrim::Conic(curve) => (curve.clone(), (0.0, 1.0)),
        }
    }
    /// 🎨️ Whether the ring `[tangency 0, end@hi, tangency 1, end@lo]` walks the patch's `(u, v)`
    /// rectangle counter-clockwise — i.e. with `du × dv`, the outward normal of every support built
    /// here, toward the viewer. Swapping the roles of `u` and `v` reverses a rectangle's
    /// orientation, so the answer flips with `cross_dir`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn ring_is_ccw(&self) -> bool {
        match self.cross_dir {
            IsoDirection::U => self.cross[1] < self.cross[0],
            IsoDirection::V => self.cross[1] > self.cross[0],
        }
    }
}

/// 🎨️ The distinct solid faces that use `edge` — one for a seam a single face uses twice, two for an
/// ordinary manifold edge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_faces(body: &Body, solid_faces: &BTreeSet<FaceId>, edge: EdgeId) -> Vec<FaceId> {
    let mut faces = Vec::new();
    for cid in body.edge_coedges(edge) {
        if let Some(c) = body.coedges.get(cid) {
            if let Some(lp) = body.loops.get(c.loop_id) {
                if solid_faces.contains(&lp.face) && !faces.contains(&lp.face) {
                    faces.push(lp.face);
                }
            }
        }
    }
    faces
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_two_faces(body: &Body, solid_faces: &BTreeSet<FaceId>, edge: EdgeId) -> Result<(FaceId, FaceId), KernelError> {
    let faces = edge_faces(body, solid_faces, edge);
    if faces.len() != 2 {
        return Err(KernelError::Operation("blend edge must be shared by exactly two distinct faces".into()));
    }
    Ok((faces[0], faces[1]))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_on_face(body: &Body, edge: EdgeId, face: FaceId) -> Option<CoedgeId> {
    body.edge_coedges(edge).into_iter().find(|&cid| body.coedges.get(cid).and_then(|c| body.loops.get(c.loop_id)).map(|lp| lp.face) == Some(face))
}

/// 🎨️ The direction `edge` is traversed in as seen by `face`'s own loop.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn traversal_direction(body: &Body, edge: EdgeId, face: FaceId) -> Result<Vec3, KernelError> {
    let cid = coedge_on_face(body, edge, face).ok_or_else(|| KernelError::Operation("blend edge has no coedge on its own face".into()))?;
    let forward = body.coedges.get(cid).unwrap().forward;
    let ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let curve = body.curves3.get(ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
    // 🧭️ An edge runs from `v0` at `range.0` to `v1` at `range.1`, and a rebuilt edge's range can
    // DECREASE (`↔️offset`'s `rebuild_topology` reuses its source curve's own parametrization) — so
    // the direction `v0 → v1` is the curve's tangent only when the range increases.
    let sense = if ent.range.1 >= ent.range.0 { 1.0 } else { -1.0 };
    let tangent = curve.d1(0.5 * (ent.range.0 + ent.range.1)) * sense;
    unit(if forward { tangent } else { -tangent }, "edge traversal direction")
}

/// 🎨️ Builds the analytic patch for one selected edge, choosing the support from the pair of
/// adjacent surfaces and refusing — never approximating — anything else.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_patch(body: &Body, edge: EdgeId, solid_faces: &BTreeSet<FaceId>, spec: BlendSpec) -> Result<Patch, KernelError> {
    let (f0, f1) = edge_two_faces(body, solid_faces, edge)?;
    let ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?.clone();
    let curve = body.curves3.get(ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?.clone();
    let mid = curve.eval(0.5 * (ent.range.0 + ent.range.1));
    let m0 = outward_normal_at(body, f0, mid)?;
    let m1 = outward_normal_at(body, f1, mid)?;
    let s0 = body.surfaces.get(body.faces.get(f0).unwrap().surface).unwrap().clone();
    let s1 = body.surfaces.get(body.faces.get(f1).unwrap().surface).unwrap().clone();
    let base = Patch {
        edge,
        faces: [f0, f1],
        surface: Surface::Plane { frame: Frame3::WORLD },
        cross_dir: IsoDirection::U,
        cross: [0.0, 0.0],
        closed: ent.v0 == ent.v1,
        end_vertex: [ent.v0, ent.v1],
        run: [(0.0, 0.0); 2],
        ends: [EndTrim::Iso(0.0), EndTrim::Iso(0.0)],
    };
    match (&s0, &s1, &curve) {
        (Surface::Plane { .. }, Surface::Plane { .. }, Curve3::Line { origin, dir }) => {
            let along = traversal_direction(body, edge, f0)?;
            let axis = unit(m0.cross(m1), "dihedral axis")?;
            if axis.dot(along) <= 0.0 {
                return Err(KernelError::Operation("blend: only convex edges have a rolling-ball blend on this side".into()));
            }
            let point = *origin + *dir * ent.range.0;
            match spec {
                BlendSpec::Fillet(r) => cylinder_patch(base, point, axis, m0, m1, r),
                BlendSpec::VariableFillet(r0, r1) => cone_fillet_patch(base, &curve, ent.range, m0, m1, r0, r1),
                BlendSpec::Chamfer(d0, d1) => chamfer_plane_patch(base, point, axis, m0, m1, d0, d1),
            }
        }
        (Surface::Plane { .. }, Surface::Cylinder { .. }, Curve3::Circle { .. }) => cap_patch(body, base, 1, 0, spec),
        (Surface::Cylinder { .. }, Surface::Plane { .. }, Curve3::Circle { .. }) => cap_patch(body, base, 0, 1, spec),
        _ => Err(KernelError::Operation("blend: only plane/plane and plane/cylinder-cap edges have an exact analytic blend".into())),
    }
}

/// 🎨️ Constant-radius rolling ball between two planes: the ball centres run along the line at
/// distance `r` inside both, so the envelope is exactly the cylinder of radius `r` about that line
/// and the two tangency curves are its `u = const` rulings. `centre = p − r(m0+m1)/(1+m0·m1)` is
/// the closed form for "distance `r` from both planes" (each dot product collapses the fraction
/// back to `r`); the frame is pinned with `x = m0`, putting face 0's tangency at `u = 0` and face
/// 1's at the dihedral's own exterior angle.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cylinder_patch(base: Patch, point: Pnt3, axis: Vec3, m0: Vec3, m1: Vec3, r: f64) -> Result<Patch, KernelError> {
    let denom = 1.0 + m0.dot(m1);
    if denom <= 1e-6 {
        return Err(KernelError::Operation("blend: the dihedral angle is too flat for a rolling ball".into()));
    }
    let origin = point - (m0 + m1) * (r / denom);
    let frame = Frame3 { origin, x: m0, y: axis.cross(m0), z: axis };
    let theta = m1.dot(frame.y).atan2(m1.dot(frame.x));
    Ok(Patch { surface: Surface::Cylinder { frame, radius: r }, cross_dir: IsoDirection::U, cross: [0.0, theta], ..base })
}

/// 🎨️ Cutting-plane chamfer between two planes: the two cut lines sit at their distances INSIDE
/// each face (`m_i × traversal_i` is that in-face inward direction) and the chamfer face is the
/// plane through both. `x` runs from cut line 0 to cut line 1 — making them its `u = 0` and
/// `u = |A1 − A0|` iso-lines — and `y` is the edge direction, which makes `z = x × y` outward.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chamfer_plane_patch(base: Patch, point: Pnt3, axis: Vec3, m0: Vec3, m1: Vec3, d0: f64, d1: f64) -> Result<Patch, KernelError> {
    let a0 = point + unit(m0.cross(axis), "chamfer in-face direction")? * d0;
    let a1 = point + unit(m1.cross(-axis), "chamfer in-face direction")? * d1;
    let cross = a1.distance(a0);
    if cross <= BLEND_EPS {
        return Err(KernelError::Operation("blend: the chamfer distances collapse the cut face".into()));
    }
    let x = unit(a1 - a0, "chamfer cross direction")?;
    let frame = Frame3 { origin: a0, x, y: axis, z: x.cross(axis) };
    Ok(Patch { surface: Surface::Plane { frame }, cross_dir: IsoDirection::U, cross: [0.0, cross], ..base })
}

/// 🎨️ Variable-radius rolling ball between two planes: the centres still run along a LINE (both the
/// edge point and the radius are affine in the edge's parameter) with a radius growing linearly
/// along it, so the envelope is exactly the cone of revolution about that line with
/// `sin(half_angle) = dr/ds` — a canal surface of a line is a cone — and the tangency curves are
/// two of its rulings. `v` is distance from the apex, so both tangencies stay iso-lines; the ends,
/// whose cap planes are NOT perpendicular to this tilted axis, are the exact conics of §
/// [`conic_end_trim`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cone_fillet_patch(base: Patch, curve: &Curve3, range: (f64, f64), m0: Vec3, m1: Vec3, r0: f64, r1: f64) -> Result<Patch, KernelError> {
    let denom = 1.0 + m0.dot(m1);
    if denom <= 1e-6 {
        return Err(KernelError::Operation("blend: the dihedral angle is too flat for a rolling ball".into()));
    }
    let bisector = (m0 + m1) * (1.0 / denom);
    let c0 = curve.eval(range.0) - bisector * r0;
    let c1 = curve.eval(range.1) - bisector * r1;
    let span = c1.distance(c0);
    if span <= BLEND_EPS {
        return Err(KernelError::Operation("blend: the variable-radius spine collapses".into()));
    }
    let slope = (r1 - r0) / span;
    if slope.abs() >= 1.0 - DIR_EPS || slope.abs() <= DIR_EPS {
        return Err(KernelError::Operation("blend: this radius variation has no cone envelope — use a constant radius".into()));
    }
    let (spine, start_radius, base_center) = if slope > 0.0 { (unit(c1 - c0, "variable-fillet spine")?, r0, c0) } else { (unit(c0 - c1, "variable-fillet spine")?, r1, c1) };
    let half_angle = slope.abs().asin();
    let apex = base_center - spine * (start_radius / slope.abs());
    let radial0 = unit(m0 - spine * m0.dot(spine), "cone radial reference")?;
    let frame = Frame3 { origin: apex, x: radial0, y: spine.cross(radial0), z: spine };
    let radial1 = m1 - spine * m1.dot(spine);
    // 🧭️ The spine points along GROWING radius, which is the edge's own direction or its opposite
    // depending on which end carries the larger radius, so face 1's tangency ruling can sit on
    // either side of face 0's; the patch simply spans the shorter way between them, and
    // `ring_is_ccw` reads the sign back off `cross` so the ring still walks `(u, v)` positively.
    let theta = radial1.dot(frame.y).atan2(radial1.dot(frame.x));
    if !(theta.abs() > DIR_EPS && theta.abs() < PI - DIR_EPS) {
        return Err(KernelError::Operation("blend: the dihedral angle is too flat for a rolling ball".into()));
    }
    Ok(Patch { surface: Surface::Cone { frame, half_angle }, cross_dir: IsoDirection::U, cross: [0.0, theta], ..base })
}

/// 🎨️ The blend of a circular cap edge between a planar cap and its coaxial cylinder: the ball
/// centres run along a CIRCLE coaxial with the cylinder, so a constant-radius fillet's envelope is
/// exactly a torus (tube radius = the fillet radius, tube angle `v` running from the cylinder's
/// tangency at `v = 0` to the cap's at `v = π/2`) and a chamfer's is exactly a cone frustum (whose
/// two tangency circles are `v = const` iso-lines too). The band is closed in the azimuth, so its
/// two ends are one seam curve used twice — the same device `make_cylinder`'s own lateral face uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cap_patch(body: &Body, base: Patch, lateral_index: usize, plane_index: usize, spec: BlendSpec) -> Result<Patch, KernelError> {
    let lateral = base.faces[lateral_index];
    let plane_face = base.faces[plane_index];
    if !base.closed {
        return Err(KernelError::Operation("blend: a plane/cylinder edge must be the cylinder's own full cap circle".into()));
    }
    let ent = body.edges.get(base.edge).unwrap().clone();
    let Curve3::Circle { frame: circle_frame, radius: cap_radius } = body.curves3.get(ent.curve).unwrap().clone() else {
        return Err(KernelError::Operation("blend: expected a circular cap edge".into()));
    };
    let Surface::Cylinder { frame: cyl_frame, radius: cyl_radius } = body.surfaces.get(body.faces.get(lateral).unwrap().surface).unwrap().clone() else {
        return Err(KernelError::Operation("blend: expected a cylindrical face".into()));
    };
    if (cyl_radius - cap_radius).abs() > 1e-9 {
        return Err(KernelError::Operation("blend: the cap circle does not lie on its own cylinder".into()));
    }
    let (_, plane_normal) = face_plane(body, plane_face)?;
    if cyl_frame.z.cross(plane_normal).norm() > DIR_EPS {
        return Err(KernelError::Operation("blend: an exact cap blend needs the cap plane perpendicular to the cylinder axis".into()));
    }
    let seam_point = body.vertices.get(ent.v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position;
    let radial = unit(seam_point - circle_frame.origin, "cap seam radial")?;
    if outward_normal_at(body, lateral, seam_point)?.dot(radial) <= 0.0 {
        return Err(KernelError::Operation("blend: only convex cap edges have a blend on this side".into()));
    }
    let into_solid = -plane_normal;
    let (surface, cross) = match spec {
        BlendSpec::Fillet(r) => {
            if r >= cap_radius {
                return Err(KernelError::Operation("blend: the fillet radius exceeds the cap radius".into()));
            }
            let frame = Frame3 { origin: circle_frame.origin + into_solid * r, x: radial, y: plane_normal.cross(radial), z: plane_normal };
            (Surface::Torus { frame, major_radius: cap_radius - r, minor_radius: r }, [0.0, FRAC_PI_2])
        }
        BlendSpec::Chamfer(first, second) => {
            let (d_lateral, d_plane) = if lateral_index == 0 { (first, second) } else { (second, first) };
            if d_plane >= cap_radius {
                return Err(KernelError::Operation("blend: the chamfer distance exceeds the cap radius".into()));
            }
            let apex_offset = (cap_radius - d_plane) * d_lateral / d_plane;
            let frame = Frame3 { origin: circle_frame.origin - into_solid * apex_offset, x: radial, y: into_solid.cross(radial), z: into_solid };
            (Surface::Cone { frame, half_angle: (d_plane / d_lateral).atan() }, [apex_offset + d_lateral, apex_offset])
        }
        BlendSpec::VariableFillet(..) => return Err(KernelError::Operation("blend: a variable radius needs a straight edge between two planes".into())),
    };
    let cross = if lateral_index == 0 { cross } else { [cross[1], cross[0]] };
    Ok(Patch { surface, cross_dir: IsoDirection::V, cross, ..base })
}

// #endregion 🔖️Patch

// #region 🔖️Corner

/// 🎨️ Everything the surgery needs at one vertex a blend reaches: where each blended face's own
/// boundary now stops, the run parameter each incident patch's tangency curve is trimmed to on each
/// of its faces, the unblended face that receives the blend's end curve as a new boundary, and the
/// corner patch that closes a fully blended vertex.
struct Corner {
    points: BTreeMap<FaceId, Pnt3>,
    verts: BTreeMap<FaceId, VertexId>,
    runs: BTreeMap<(EdgeId, FaceId), f64>,
    cap: Option<FaceId>,
    corner_surface: Option<Surface>,
    corner_edges: Vec<EdgeId>,
}

/// 🎨️ Every solid edge and face incident to `vertex`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vertex_star(body: &Body, vertex: VertexId, solid_faces: &BTreeSet<FaceId>) -> (BTreeSet<EdgeId>, BTreeSet<FaceId>) {
    let mut edges = BTreeSet::new();
    let mut faces = BTreeSet::new();
    for &face in solid_faces {
        for cid in body.face_coedges(face) {
            let Some(c) = body.coedges.get(cid) else { continue };
            let Some(e) = body.edges.get(c.edge) else { continue };
            if e.v0 == vertex || e.v1 == vertex {
                edges.insert(c.edge);
                faces.insert(face);
            }
        }
    }
    (edges, faces)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tangency_param(patch: &Patch, i: usize, point: Pnt3) -> Result<f64, KernelError> {
    curve_param_at(&patch.tangency(i), point, (f64::NEG_INFINITY, f64::INFINITY)).ok_or_else(|| KernelError::Operation("blend: the corner point does not lie on the patch's own tangency curve".into()))
}

/// 🎨️ Classifies one vertex a blend reaches and derives its corner geometry.
///
/// * a CLOSED band's seam vertex splits into one vertex per adjacent face, joined by the band's own
///   seam curve (used twice by the band, exactly as a cylinder's lateral seam is);
/// * a vertex with exactly ONE blended edge keeps its two unblended edges, shortened to where the
///   tangency curves cross the third (cap) face's plane, and that cap face gains the blend's end
///   curve as a new boundary edge;
/// * a vertex with ALL THREE edges blended loses its three edges entirely: each face's two tangency
///   curves meet at one point and the three end curves bound a corner patch — a sphere octant for a
///   fillet (the ball resting in the corner touches all three faces, so those three points share
///   its centre and every end circle is a GREAT circle on it), a planar triangle for a chamfer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_corner(body: &Body, vertex: VertexId, incident: &[EdgeId], patches: &BTreeMap<EdgeId, Patch>, solid_faces: &BTreeSet<FaceId>, spec: BlendSpec) -> Result<Corner, KernelError> {
    let mut corner = Corner { points: BTreeMap::new(), verts: BTreeMap::new(), runs: BTreeMap::new(), cap: None, corner_surface: None, corner_edges: Vec::new() };
    if incident.len() == 1 && patches[&incident[0]].closed {
        let patch = &patches[&incident[0]];
        for i in 0..2 {
            corner.points.insert(patch.faces[i], patch.tangency(i).eval(0.0));
            corner.runs.insert((patch.edge, patch.faces[i]), 0.0);
        }
        return Ok(corner);
    }
    let (star_edges, star_faces) = vertex_star(body, vertex, solid_faces);
    if star_edges.len() != 3 || star_faces.len() != 3 {
        return Err(KernelError::Operation(format!("blend: vertex {vertex} is not trihedral ({} edges, {} faces) — no exact corner exists", star_edges.len(), star_faces.len())));
    }
    match incident.len() {
        1 => {
            let patch = &patches[&incident[0]];
            let cap = *star_faces.iter().find(|f| **f != patch.faces[0] && **f != patch.faces[1]).ok_or_else(|| KernelError::Operation("blend: no cap face at a partially blended vertex".into()))?;
            let (cap_point, cap_normal) = face_plane(body, cap)?;
            for i in 0..2 {
                let Curve3::Line { origin, dir } = patch.tangency(i) else {
                    return Err(KernelError::Operation("blend: a partially blended vertex needs straight tangency curves".into()));
                };
                let point = line_plane_point(origin, dir, cap_point, cap_normal).ok_or_else(|| KernelError::Operation("blend: the tangency curve runs parallel to its own cap face".into()))?;
                corner.runs.insert((patch.edge, patch.faces[i]), tangency_param(patch, i, point)?);
                corner.points.insert(patch.faces[i], point);
            }
            corner.cap = Some(cap);
            corner.corner_edges = vec![patch.edge];
        }
        3 => {
            let mut normals: BTreeMap<FaceId, Vec3> = BTreeMap::new();
            let vertex_position = body.vertices.get(vertex).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position;
            for &face in &star_faces {
                let owners: Vec<(EdgeId, usize)> = incident.iter().filter_map(|e| patches[e].faces.iter().position(|f| *f == face).map(|i| (*e, i))).collect();
                if owners.len() != 2 {
                    return Err(KernelError::Operation("blend: a fully blended vertex needs exactly two blended edges per face".into()));
                }
                let mut lines = Vec::with_capacity(2);
                for (edge, i) in &owners {
                    let Curve3::Line { origin, dir } = patches[edge].tangency(*i) else {
                        return Err(KernelError::Operation("blend: a fully blended vertex needs straight tangency curves".into()));
                    };
                    lines.push((origin, dir));
                }
                let point = line_line_point(lines[0].0, lines[0].1, lines[1].0, lines[1].1).ok_or_else(|| KernelError::Operation("blend: the two tangency curves on a face do not meet at the corner".into()))?;
                for (edge, i) in &owners {
                    corner.runs.insert((*edge, face), tangency_param(&patches[edge], *i, point)?);
                }
                normals.insert(face, outward_normal_at(body, face, vertex_position)?);
                corner.points.insert(face, point);
            }
            for &edge in incident {
                let patch = &patches[&edge];
                let a = corner.runs[&(edge, patch.faces[0])];
                let b = corner.runs[&(edge, patch.faces[1])];
                if (a - b).abs() > 1e-7 {
                    return Err(KernelError::Operation("blend: a corner patch needs its neighbours trimmed at one common station".into()));
                }
            }
            corner.corner_edges = incident.to_vec();
            corner.corner_surface = Some(corner_surface(&corner.points, &normals, spec)?);
        }
        _ => return Err(KernelError::Operation(format!("blend: vertex {vertex} has {} of its 3 edges blended — an exact corner needs either one or all", incident.len()))),
    }
    Ok(corner)
}

/// 🎨️ The corner patch closing a fully blended trihedral vertex: a sphere of the fillet's own
/// radius centred where the rolling ball rests against all three faces (so every end circle, having
/// that same centre and radius, is a GREAT circle on it), or the plane through the three chamfer
/// corner points. The sphere's frame is pinned with `z` on a face normal perpendicular to the other
/// two, which puts that face's corner at the pole and makes all three boundaries iso-lines: one
/// equator and two meridians.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn corner_surface(points: &BTreeMap<FaceId, Pnt3>, normals: &BTreeMap<FaceId, Vec3>, spec: BlendSpec) -> Result<Surface, KernelError> {
    let faces: Vec<FaceId> = points.keys().copied().collect();
    match spec.fillet_radius() {
        Some(r) => {
            for rotation in 0..3 {
                let (a, b, c) = (faces[rotation], faces[(rotation + 1) % 3], faces[(rotation + 2) % 3]);
                let (na, nb, nc) = (normals[&a], normals[&b], normals[&c]);
                if na.dot(nb).abs() > DIR_EPS || na.dot(nc).abs() > DIR_EPS {
                    continue;
                }
                let center = points[&a] - na * r;
                for face in &faces {
                    if (center + normals[face] * r).distance(points[face]) > 1e-7 {
                        return Err(KernelError::Operation("blend: the three corner points do not share one rolling-ball centre".into()));
                    }
                }
                return Ok(Surface::Sphere { frame: Frame3 { origin: center, x: nb, y: na.cross(nb), z: na }, radius: r });
            }
            Err(KernelError::Operation("blend: a spherical corner needs one face normal perpendicular to the other two".into()))
        }
        None => {
            let (p0, p1, p2) = (points[&faces[0]], points[&faces[1]], points[&faces[2]]);
            let normal = unit((p1 - p0).cross(p2 - p0), "chamfer corner normal")?;
            let outward = normals.values().fold(Vec3::ZERO, |acc, n| acc + *n);
            let normal = if normal.dot(outward) >= 0.0 { normal } else { -normal };
            Ok(Surface::Plane { frame: Frame3::from_normal(p0, normal).ok_or_else(|| KernelError::Operation("blend: degenerate chamfer corner plane".into()))? })
        }
    }
}

/// 🎨️ The exact conic a cap plane cuts out of a cone patch, as a rational quadratic Bézier from
/// `faces[0]`'s tangency ruling to `faces[1]`'s.
///
/// The cone's ruling direction at azimuth `u` is `D(u)`, an affine function of `(cos u, sin u)`, so
/// it is exactly a rational quadratic over the standard arc control net `C_i` with weights `w_i`.
/// The plane meets that ruling at `A + D(u)·c/(D(u)·n)` — a PERSPECTIVE image of `D`, which is
/// exactly the rational quadratic with control points `A + D_i·c/(D_i·n)` and weights `w_i(D_i·n)`
/// (substituting them back collapses the ratio to the same formula, so the identity is exact, not a
/// fit). Every control point lies in the cap plane, which is what makes the p-curve on the cap
/// exact too — only the p-curve on the CONE has no closed form, and that one is certified numerically.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn conic_end_trim(patch: &Patch, plane_point: Pnt3, plane_normal: Vec3) -> Result<Curve3, KernelError> {
    let Surface::Cone { frame, half_angle } = &patch.surface else {
        return Err(KernelError::Operation("blend: only a cone patch can be trimmed by an oblique cap plane".into()));
    };
    let (u0, u1) = (patch.cross[0], patch.cross[1]);
    let half = 0.5 * (u1 - u0);
    if !(half.abs() > 0.0 && half.abs() < FRAC_PI_2) {
        return Err(KernelError::Operation("blend: the blend's angular span needs a single conic arc".into()));
    }
    let mid = 0.5 * (u0 + u1);
    let cos_half = half.cos();
    let net = [(u0.cos(), u0.sin(), 1.0), (mid.cos() / cos_half, mid.sin() / cos_half, cos_half), (u1.cos(), u1.sin(), 1.0)];
    let tan_a = half_angle.tan();
    let mut controls = Vec::with_capacity(3);
    let mut weights = Vec::with_capacity(3);
    let mut offset = (plane_point - frame.origin).dot(plane_normal);
    let mut normal = plane_normal;
    let mut directions = Vec::with_capacity(3);
    for (cx, cy, _) in net {
        directions.push(frame.to_world_vector(Vec3::new(tan_a * cx, tan_a * cy, 1.0)));
    }
    if directions.iter().all(|d| d.dot(normal) < 0.0) {
        normal = -normal;
        offset = -offset;
    }
    for (i, (_, _, w)) in net.into_iter().enumerate() {
        let denominator = directions[i].dot(normal);
        if denominator <= DIR_EPS {
            return Err(KernelError::Operation("blend: a ruling of this blend never reaches its own cap plane".into()));
        }
        controls.push(frame.origin + directions[i] * (offset / denominator));
        weights.push(w * denominator);
    }
    let knots = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, 3).ok_or_else(|| KernelError::Operation("blend: conic knot vector".into()))?;
    Ok(Curve3::Nurbs { knots, controls, weights })
}

// #endregion 🔖️Corner

// #region 🔖️Surgery

/// 🎨️ One coedge of a face being built: the edge, its traversal sense, and the exact p-curve that
/// edge carries ON THIS face.
struct Member {
    edge: EdgeId,
    forward: bool,
    pcurve: Curve2,
    prange: (f64, f64),
}

/// 🎨️ Everything the assembly mints, keyed so both faces of a shared edge reach the same entity.
struct Minted {
    vertices: BTreeMap<VertexId, VertexId>,
    edges: BTreeMap<EdgeId, EdgeId>,
    ranges: BTreeMap<EdgeId, (f64, f64)>,
    tangency: BTreeMap<(EdgeId, usize), EdgeId>,
    ends: BTreeMap<(EdgeId, usize), EdgeId>,
}

/// 🎨️ Shifts each member's p-curve by whole periods so the ring is CONTINUOUS in the face's own
/// `(u, v)`, chaining every member's start onto its predecessor's end.
///
/// Each p-curve is derived independently, and an inversion on a periodic surface answers in
/// whichever branch its own `atan2` landed in — so a corner patch's three arcs could come back at
/// `u = 0`, `u = −π/2` and `u = 3π/2`, all three geometrically right and the polygon between them
/// three times too wide, winding backwards. A pole is exempt on purpose: two meridians meeting
/// there legitimately carry different `u`, and shifting one onto the other would collapse the lune
/// — the chain simply carries the arriving branch forward, which is what the departing arc's own
/// `u` is then measured against.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn align_pcurve_branches(surface: &Surface, members: &mut [Member]) {
    let (u_periodic, v_periodic) = (surface.is_u_periodic(), surface.is_v_periodic());
    if !u_periodic && !v_periodic {
        return;
    }
    let mut previous: Option<Pnt2> = None;
    for member in members.iter_mut() {
        let (start_at, end_at) = if member.forward { (member.prange.0, member.prange.1) } else { (member.prange.1, member.prange.0) };
        if let Some(previous) = previous {
            let start = member.pcurve.eval(start_at);
            let du = if u_periodic { ((previous.x - start.x) / TAU).round() * TAU } else { 0.0 };
            let dv = if v_periodic { ((previous.y - start.y) / TAU).round() * TAU } else { 0.0 };
            if du != 0.0 || dv != 0.0 {
                member.pcurve = member.pcurve.translated(Vec2::new(du, dv));
            }
        }
        previous = Some(member.pcurve.eval(end_at));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attach_with_pcurves(body: &mut Body, surface: SurfaceId, members: &mut [Member], flipped: bool, rec: &mut OpRecorder) -> FaceId {
    let support = body.surfaces.get(surface).expect("face surface").clone();
    align_pcurve_branches(&support, members);
    let pairs: Vec<(EdgeId, bool)> = members.iter().map(|m| (m.edge, m.forward)).collect();
    let outer = make_loop(body, FaceId::from_raw(0, 0), &pairs);
    let face = add_face(body, surface, Some(outer), vec![], flipped, Tol::DEFAULT, rec);
    body.loops.get_mut(outer).unwrap().face = face;
    for (cid, member) in body.loop_coedges(outer).into_iter().zip(members) {
        let pcurve = body.curves2.insert(member.pcurve.clone());
        let coedge = body.coedges.get_mut(cid).unwrap();
        coedge.pcurve = Some(pcurve);
        coedge.prange = member.prange;
    }
    face
}

/// 🎨️ The blend engine: builds every patch, classifies every vertex the blend reaches, trims each
/// patch to its corners, then rebuilds the whole solid — original faces with their boundaries
/// substituted and shortened, one patch face per selected edge, one corner face per fully blended
/// vertex.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn blend_edges(body: &mut Body, solid: SolidId, edges: &[EdgeId], spec: BlendSpec, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    let solid_faces: BTreeSet<FaceId> = body.solid_faces(solid).into_iter().collect();
    let selected: BTreeSet<EdgeId> = edges.iter().copied().collect();

    let mut patches: BTreeMap<EdgeId, Patch> = BTreeMap::new();
    for &edge in &selected {
        patches.insert(edge, build_patch(body, edge, &solid_faces, spec)?);
    }

    let mut incident: BTreeMap<VertexId, Vec<EdgeId>> = BTreeMap::new();
    for &edge in patches.keys() {
        let ent = body.edges.get(edge).unwrap();
        incident.entry(ent.v0).or_default().push(edge);
        if ent.v1 != ent.v0 {
            incident.entry(ent.v1).or_default().push(edge);
        }
    }

    let mut corners: BTreeMap<VertexId, Corner> = BTreeMap::new();
    for (&vertex, edges_here) in &incident {
        corners.insert(vertex, build_corner(body, vertex, edges_here, &patches, &solid_faces, spec)?);
    }

    trim_patches(body, &mut patches, &corners)?;

    let mut minted = Minted { vertices: BTreeMap::new(), edges: BTreeMap::new(), ranges: BTreeMap::new(), tangency: BTreeMap::new(), ends: BTreeMap::new() };
    for corner in corners.values_mut() {
        let points: Vec<(FaceId, Pnt3)> = corner.points.iter().map(|(f, p)| (*f, *p)).collect();
        for (face, point) in points {
            let id = make_vertex(body, point, Tol::DEFAULT, rec);
            corner.verts.insert(face, id);
        }
    }

    for &edge in &solid_edge_set(body, solid) {
        if selected.contains(&edge) {
            continue;
        }
        mint_carried_edge(body, edge, &corners, &solid_faces, &mut minted, rec)?;
    }

    for (&edge, patch) in &patches {
        for i in 0..2 {
            let curve_id = body.curves3.insert(patch.tangency(i));
            let v0 = corners[&patch.end_vertex[0]].verts[&patch.faces[i]];
            let v1 = corners[&patch.end_vertex[1]].verts[&patch.faces[i]];
            let fresh = make_edge(body, curve_id, patch.run[i], v0, v1, Tol::DEFAULT, rec);
            minted.tangency.insert((edge, i), fresh);
        }
        for slot in 0..2 {
            if patch.closed && slot == 1 {
                let seam = minted.ends[&(edge, 0)];
                minted.ends.insert((edge, 1), seam);
                continue;
            }
            let (curve, range) = patch.end_curve(slot);
            let curve_id = body.curves3.insert(curve);
            let v0 = corners[&patch.end_vertex[slot]].verts[&patch.faces[0]];
            let v1 = corners[&patch.end_vertex[slot]].verts[&patch.faces[1]];
            let fresh = make_edge(body, curve_id, range, v0, v1, Tol::DEFAULT, rec);
            minted.ends.insert((edge, slot), fresh);
        }
    }

    let mut faces: Vec<FaceId> = Vec::new();
    for &face in &solid_faces {
        faces.push(rebuild_face(body, face, &patches, &corners, &minted, rec)?);
    }
    for (&edge, patch) in &patches {
        faces.push(build_patch_face(body, edge, patch, &minted, rec)?);
    }
    for (&vertex, corner) in &corners {
        if corner.corner_surface.is_some() {
            faces.push(build_corner_face(body, vertex, corner, &patches, &minted, rec)?);
        }
    }

    let shell = add_shell(body, faces, rec);
    Ok(add_solid(body, shell, vec![], rec))
}

/// 🎨️ Orders each patch's two ends by its own run parameter and derives the end trims: an iso-line
/// wherever both faces stop at the same station (every constant-radius blend, whose cap plane is
/// perpendicular to its axis), the exact conic otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn trim_patches(body: &Body, patches: &mut BTreeMap<EdgeId, Patch>, corners: &BTreeMap<VertexId, Corner>) -> Result<(), KernelError> {
    for (&edge, patch) in patches.iter_mut() {
        if patch.closed {
            patch.run = [(0.0, TAU); 2];
            patch.ends = [EndTrim::Iso(0.0), EndTrim::Iso(TAU)];
            continue;
        }
        let faces = patch.faces;
        let mut ends = patch.end_vertex;
        let at = |ends: &[VertexId; 2], slot: usize, i: usize| corners[&ends[slot]].runs[&(edge, faces[i])];
        if at(&ends, 0, 0) > at(&ends, 1, 0) {
            ends.swap(0, 1);
        }
        let mut run = [(0.0, 0.0); 2];
        for (i, entry) in run.iter_mut().enumerate() {
            let (lo, hi) = (at(&ends, 0, i), at(&ends, 1, i));
            if hi - lo <= 1e-9 {
                return Err(KernelError::Operation("blend: the blend consumes its whole edge — reduce the radius".into()));
            }
            *entry = (lo, hi);
        }
        patch.end_vertex = ends;
        patch.run = run;
        for slot in 0..2 {
            let (a, b) = (at(&ends, slot, 0), at(&ends, slot, 1));
            let Some(cap) = corners[&ends[slot]].cap else {
                patch.ends[slot] = EndTrim::Iso(a);
                continue;
            };
            let (point, normal) = face_plane(body, cap)?;
            // 🧭️ Agreeing on ONE station is necessary but not sufficient: a variable fillet's cap
            // plane is symmetric about the dihedral's bisector, so both rulings meet it at the same
            // distance from the apex while the cross-section circle between them bulges straight
            // out of that plane. The trim is an iso-line only when the iso-line itself lies IN the
            // cap — which is exactly the constant-radius case, whose axis the cap is perpendicular to.
            patch.ends[slot] = if (a - b).abs() <= 1e-9 && iso_lies_in_plane(patch, a, point, normal) {
                EndTrim::Iso(a)
            } else {
                EndTrim::Conic(conic_end_trim(patch, point, normal)?)
            };
        }
    }
    Ok(())
}

/// 🎨️ Whether the patch's own cross-section at run parameter `w` lies IN the given plane — the
/// precondition for trimming that end with an iso-line instead of the plane's own conic section.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn iso_lies_in_plane(patch: &Patch, w: f64, plane_point: Pnt3, plane_normal: Vec3) -> bool {
    let curve = patch.surface.isocurve(patch.run_dir(), w);
    let (lo, hi) = (patch.cross[0], patch.cross[1]);
    (0..=8).all(|i| {
        let t = lo + (hi - lo) * f64::from(i) / 8.0;
        (curve.eval(t) - plane_point).dot(plane_normal).abs() <= 1e-9
    })
}

/// 🎨️ Carries one unblended edge over into the result, shortened wherever a corner pulled one of
/// its ends back, with fresh vertices so the input solid stays intact.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn mint_carried_edge(body: &mut Body, edge: EdgeId, corners: &BTreeMap<VertexId, Corner>, solid_faces: &BTreeSet<FaceId>, minted: &mut Minted, rec: &mut OpRecorder) -> Result<(), KernelError> {
    let ent = body.edges.get(edge).unwrap().clone();
    let curve = body.curves3.get(ent.curve).unwrap().clone();
    let originals = [ent.v0, ent.v1];
    let mut params = [ent.range.0, ent.range.1];
    let mut verts = originals;
    for slot in 0..2 {
        if let Some(corner) = corners.get(&originals[slot]) {
            let face = blended_face_of(body, edge, corner, solid_faces)?;
            let point = corner.points[&face];
            params[slot] = curve_param_at(&curve, point, ent.range).ok_or_else(|| KernelError::Operation("blend: a shortened endpoint is not on its own edge".into()))?;
            verts[slot] = corner.verts[&face];
            continue;
        }
        if !minted.vertices.contains_key(&originals[slot]) {
            let position = body.vertices.get(originals[slot]).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position;
            let fresh = make_vertex(body, position, Tol::DEFAULT, rec);
            minted.vertices.insert(originals[slot], fresh);
        }
        verts[slot] = minted.vertices[&originals[slot]];
    }
    let range = (params[0], params[1]);
    let fresh = make_edge(body, ent.curve, range, verts[0], verts[1], Tol::DEFAULT, rec);
    minted.edges.insert(edge, fresh);
    minted.ranges.insert(edge, range);
    Ok(())
}

/// 🎨️ The single blended face an unblended edge touches at a corner — the face whose own boundary
/// pulled that edge's endpoint back to the corner point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn blended_face_of(body: &Body, edge: EdgeId, corner: &Corner, solid_faces: &BTreeSet<FaceId>) -> Result<FaceId, KernelError> {
    let hits: Vec<FaceId> = edge_faces(body, solid_faces, edge).into_iter().filter(|f| corner.points.contains_key(f)).collect();
    match hits.len() {
        1 => Ok(hits[0]),
        _ => Err(KernelError::Operation("blend: an unblended edge at a blended vertex touches no single blended face".into())),
    }
}

/// 🎨️ Rebuilds one original face: every selected edge becomes that face's own tangency curve, every
/// other edge is carried over (its p-curve reused verbatim, its parameter range remapped where a
/// corner shortened it), and a partially blended vertex splices in the blend's end curve.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rebuild_face(body: &mut Body, face: FaceId, patches: &BTreeMap<EdgeId, Patch>, corners: &BTreeMap<VertexId, Corner>, minted: &Minted, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    let fd = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?.clone();
    let surface = body.surfaces.get(fd.surface).unwrap().clone();
    let outer = fd.outer.ok_or_else(|| KernelError::Operation("blend: face has no outer loop".into()))?;
    if !fd.inners.is_empty() {
        return Err(KernelError::Operation("blend: faces with holes are not supported".into()));
    }
    let ring = body.loop_coedges(outer);
    let mut members: Vec<Member> = Vec::with_capacity(ring.len() + 2);
    for &cid in &ring {
        let coedge = body.coedges.get(cid).unwrap().clone();
        let end_vertex = body.coedge_endpoints(cid).map(|(_, b)| b).ok_or_else(|| KernelError::Operation("blend: broken coedge".into()))?;
        if let Some(patch) = patches.get(&coedge.edge) {
            let i = patch.faces.iter().position(|f| *f == face).ok_or_else(|| KernelError::Operation("blend: a patch does not know its own face".into()))?;
            let curve = patch.tangency(i);
            let pcurve = blend_pcurve(&surface, &curve, patch.run[i])?;
            members.push(Member { edge: minted.tangency[&(patch.edge, i)], forward: !patch_tangency_forward(patch, i), pcurve, prange: patch.run[i] });
        } else {
            let range = minted.ranges[&coedge.edge];
            let original = body.edges.get(coedge.edge).map(|e| e.range).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
            let pcurve_id = coedge.pcurve.ok_or_else(|| KernelError::Operation("blend: the input solid has a coedge without a p-curve".into()))?;
            let pcurve = body.curves2.get(pcurve_id).ok_or_else(|| KernelError::MissingEntity("pcurve".into()))?.clone();
            members.push(Member { edge: minted.edges[&coedge.edge], forward: coedge.forward, pcurve, prange: remap_prange(coedge.prange, original, range) });
        }
        if let Some(corner) = corners.get(&end_vertex) {
            if corner.cap == Some(face) {
                let patch = &patches[&corner.corner_edges[0]];
                let slot = usize::from(patch.end_vertex[1] == end_vertex);
                let (curve, range) = patch.end_curve(slot);
                let pcurve = blend_pcurve(&surface, &curve, range)?;
                members.push(Member { edge: minted.ends[&(patch.edge, slot)], forward: !patch_end_forward(patch, slot), pcurve, prange: range });
            }
        }
    }
    Ok(attach_with_pcurves(body, fd.surface, &mut members, fd.flipped, rec))
}

/// 🎨️ A p-curve's own sub-range for a shortened edge — the same affine `s` the same-parameter law
/// maps `range` onto `prange` with.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn remap_prange(prange: (f64, f64), original: (f64, f64), trimmed: (f64, f64)) -> (f64, f64) {
    let span = original.1 - original.0;
    if span.abs() <= 1e-15 {
        return prange;
    }
    let at = |t: f64| prange.0 + (prange.1 - prange.0) * (t - original.0) / span;
    (at(trimmed.0), at(trimmed.1))
}

/// 🎨️ How the patch itself traverses its own tangency curve `i`: the ring is
/// `[tangency 0, end@hi, tangency 1, end@lo]` when that walks `(u, v)` counter-clockwise and its
/// reverse otherwise. Every ADJACENT face traverses the same edge the other way round, which is
/// exactly what a closed, coherently oriented shell means — so no face outside the patch needs its
/// own winding derivation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn patch_tangency_forward(patch: &Patch, i: usize) -> bool {
    patch.ring_is_ccw() == (i == 0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn patch_end_forward(patch: &Patch, slot: usize) -> bool {
    patch.ring_is_ccw() == (slot == 1)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_patch_face(body: &mut Body, edge: EdgeId, patch: &Patch, minted: &Minted, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    let surface = patch.surface.clone();
    let surface_id = body.surfaces.insert(surface.clone());
    let tangency = |i: usize| (minted.tangency[&(edge, i)], patch_tangency_forward(patch, i), patch.tangency(i), patch.run[i]);
    let end = |slot: usize| {
        let (curve, range) = patch.end_curve(slot);
        (minted.ends[&(edge, slot)], patch_end_forward(patch, slot), curve, range)
    };
    let entries = if patch.ring_is_ccw() { [tangency(0), end(1), tangency(1), end(0)] } else { [end(0), tangency(1), end(1), tangency(0)] };
    let mut members = Vec::with_capacity(4);
    for (edge_id, forward, curve, range) in entries {
        let pcurve = blend_pcurve(&surface, &curve, range)?;
        members.push(Member { edge: edge_id, forward, pcurve, prange: range });
    }
    Ok(attach_with_pcurves(body, surface_id, &mut members, false, rec))
}

/// 🎨️ Builds the corner face from the three end curves meeting at a fully blended vertex, chaining
/// them into a ring by their shared vertices — the traversal sense is forced by shell closure (the
/// opposite of each patch's own), so the ring's winding needs no independent derivation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_corner_face(body: &mut Body, vertex: VertexId, corner: &Corner, patches: &BTreeMap<EdgeId, Patch>, minted: &Minted, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    let surface = corner.corner_surface.clone().expect("corner face has a surface");
    let surface_id = body.surfaces.insert(surface.clone());
    let mut pending: Vec<(EdgeId, bool, Curve3, (f64, f64), VertexId, VertexId)> = Vec::new();
    for &edge in &corner.corner_edges {
        let patch = &patches[&edge];
        let slot = usize::from(patch.end_vertex[1] == vertex);
        let id = minted.ends[&(edge, slot)];
        let forward = !patch_end_forward(patch, slot);
        let ent = body.edges.get(id).unwrap();
        let (from, to) = if forward { (ent.v0, ent.v1) } else { (ent.v1, ent.v0) };
        let (curve, range) = patch.end_curve(slot);
        pending.push((id, forward, curve, range, from, to));
    }
    let mut ordered = vec![pending.remove(0)];
    while !pending.is_empty() {
        let tail = ordered.last().unwrap().5;
        let index = pending.iter().position(|entry| entry.4 == tail).ok_or_else(|| KernelError::Operation("blend: the corner's end curves do not form a ring".into()))?;
        ordered.push(pending.remove(index));
    }
    if ordered.first().unwrap().4 != ordered.last().unwrap().5 {
        return Err(KernelError::Operation("blend: the corner ring does not close".into()));
    }
    let mut members = Vec::with_capacity(ordered.len());
    for (id, forward, curve, range, _, _) in ordered {
        let pcurve = blend_pcurve(&surface, &curve, range)?;
        members.push(Member { edge: id, forward, pcurve, prange: range });
    }
    Ok(attach_with_pcurves(body, surface_id, &mut members, false, rec))
}

// #endregion 🔖️Surgery

// #region 🔖️Fillet

/// 🎨️ Constant-radius rolling-ball fillet on `edges` of `solid` — exact analytic topology surgery,
/// see this module's own docstring.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fillet_edges(body: &mut Body, solid: SolidId, edges: &[EdgeId], radius: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    validate_blend_request(body, solid, edges, radius)?;
    blend_edges(body, solid, edges, BlendSpec::Fillet(radius), rec)
}

/// 🎨️ Linearly varying fillet radius `r0 → r1` along a single `edge`: the ball centres still run
/// along a line, so the envelope is exactly a cone of revolution — see [`cone_fillet_patch`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fillet_variable(body: &mut Body, solid: SolidId, edge: EdgeId, r0: f64, r1: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if r0 <= 0.0 || r1 <= 0.0 {
        return Err(KernelError::InvalidInput("variable fillet radii must be positive".into()));
    }
    validate_blend_request(body, solid, &[edge], r0.max(r1))?;
    blend_edges(body, solid, &[edge], BlendSpec::VariableFillet(r0, r1), rec)
}

// #endregion 🔖️Fillet

// #region 🔖️Chamfer

/// 🎨️ Asymmetric chamfer (`d1` along the first adjacent face, `d2` along the second) on `edges` of
/// `solid` — a planar cut face between two planar faces, an exact cone frustum at a cylinder's cap.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chamfer_edges(body: &mut Body, solid: SolidId, edges: &[EdgeId], d1: f64, d2: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    validate_blend_request(body, solid, edges, d1.min(d2))?;
    if !(d2.is_finite() && d2 > 0.0) {
        return Err(KernelError::InvalidInput("chamfer distance must be positive".into()));
    }
    blend_edges(body, solid, edges, BlendSpec::Chamfer(d1, d2), rec)
}

// #endregion 🔖️Chamfer

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
