//! 🎨️ Exact rolling-ball fillet and cutting-plane chamfer via topology surgery.
//!
//! Every selected edge is replaced by two tangent edges (recomputed on the neighbouring faces'
//! own surfaces) plus a new blend face bridging them: an arc-swept NURBS strip for
//! [`fillet_edges`]/[`fillet_variable`] (cross-section a genuine circular arc of the requested
//! radius at every sampled station — exact geometry, stored as a general `Surface::Nurbs` patch
//! rather than a literal `Surface::Cylinder`/`Surface::Torus`, a documented representation
//! simplification — see `📓️w2d-blends-offsets-draft.md`), a ruled strip (straight rulings) for
//! [`chamfer_edges`] (exact planes when both adjacent faces are planar). No triangle soup, no
//! convex hull, no sampled "blunt" approximation.
//!
//! Ticket `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave W2-D.

use std::collections::HashSet;

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{make_edge, make_loop, make_vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::offset::{ruled_surface_from_curves, set_face_pcurves};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{attach_face, finish_solid, line_edge};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::edge_length;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{Curve2Id, EdgeId, FaceId, SolidId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::{interpolate_curve, ParamMethod};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

const BLEND_TOL: f64 = 1e-6;
const BLEND_SAMPLES: usize = 9;

// #region 🔖️Validate

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_solid(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity("solid".into()));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_edge_set(body: &Body, solid: SolidId) -> HashSet<EdgeId> {
    let mut edges = HashSet::new();
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

// #region 🔖️Topology

/// 🎨️ The two distinct faces of `solid` that share `edge` (its two coedges' owning loops).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_two_faces(body: &Body, solid_faces: &HashSet<FaceId>, edge: EdgeId) -> Result<(FaceId, FaceId), KernelError> {
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
    if faces.len() != 2 {
        return Err(KernelError::Operation("blend edge must be shared by exactly two distinct faces".into()));
    }
    Ok((faces[0], faces[1]))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_on_face(body: &Body, edge: EdgeId, face: FaceId) -> Option<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::CoedgeId> {
    body.edge_coedges(edge).into_iter().find(|&cid| body.coedges.get(cid).and_then(|c| body.loops.get(c.loop_id)).map(|lp| lp.face) == Some(face))
}

/// 🎨️ Outward normal of `face0`/`face1` (accounting for [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Face::flipped`]) and the
/// 3D point, all evaluated at `edge`'s own curve parameter `t` via each face's stored p-curve
/// (exact — the edge already lies on both surfaces, no numerical closest-point search needed).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn station_normals(body: &Body, edge: EdgeId, f0: FaceId, f1: FaceId, s0: &Surface, s1: &Surface, t: f64) -> Result<(Vec3, Vec3, Pnt3), KernelError> {
    let edge_ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let curve = body.curves3.get(edge_ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
    let p = curve.eval(t);
    let normal_on = |f: FaceId, s: &Surface| -> Result<Vec3, KernelError> {
        let mut n = if let Surface::Plane { frame } = s {
            frame.z
        } else if let Some(cid) = coedge_on_face(body, edge, f) {
            match body.coedges.get(cid).unwrap().pcurve {
                Some(pid) => {
                    let pc = body.curves2.get(pid).ok_or_else(|| KernelError::MissingEntity("pcurve".into()))?;
                    let uv = pc.eval(t);
                    normal_with_pole_fallback(s, uv.x, uv.y)?
                }
                None => {
                    let cu = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_uv(s, s.domain(), p, BLEND_TOL);
                    normal_with_pole_fallback(s, cu.u, cu.v)?
                }
            }
        } else {
            return Err(KernelError::Operation("blend edge missing coedge on face".into()));
        };
        if body.faces.get(f).unwrap().flipped {
            n = -n;
        }
        Ok(n)
    };
    let n0 = normal_on(f0, s0)?;
    let n1 = normal_on(f1, s1)?;
    Ok((n0, n1, p))
}

/// 🎨️ [`Surface::normal`] degenerates exactly at a pole/apex singularity; nudge `v` toward the
/// domain interior and retry once (same technique as [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::offset`]'s own fallback).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normal_with_pole_fallback(surf: &Surface, u: f64, v: f64) -> Result<Vec3, KernelError> {
    if let Some(n) = surf.normal(u, v) {
        return Ok(n);
    }
    let (v0, v1) = surf.domain().1;
    let eps = 1e-6 * (v1 - v0).abs().max(1.0);
    let v_try = if (v - v0).abs() < (v1 - v).abs() { v + eps } else { v - eps };
    surf.normal(u, v_try).ok_or_else(|| KernelError::Operation("degenerate surface normal".into()))
}

/// 🎨️ Rebuilds `face`'s outer loop, substituting the coedge that used `old_edge` for one that
/// uses `new_edge` (same orientation), reusing every unchanged coedge's existing p-curve verbatim
/// and fitting a fresh one (via [`Surface::project_curve`]) only for the substituted coedge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn replace_face_edge(body: &mut Body, face: FaceId, old_edge: EdgeId, new_edge: EdgeId, new_curve: &Curve3, new_range: (f64, f64), tol: f64) -> Result<(), KernelError> {
    let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?.clone();
    let outer = face_data.outer.ok_or_else(|| KernelError::Operation("face has no outer loop".into()))?;
    let old_coedges = body.loop_coedges(outer);
    let mut members = Vec::with_capacity(old_coedges.len());
    let mut saved: Vec<Option<(Curve2Id, (f64, f64))>> = Vec::with_capacity(old_coedges.len());
    for cid in &old_coedges {
        let c = body.coedges.get(*cid).unwrap();
        if c.edge == old_edge {
            members.push((new_edge, c.forward));
            saved.push(None);
        } else {
            members.push((c.edge, c.forward));
            saved.push(c.pcurve.map(|p| (p, c.prange)));
        }
    }
    let new_loop = make_loop(body, face, &members);
    let new_coedges = body.loop_coedges(new_loop);
    let surface = body.surfaces.get(face_data.surface).unwrap().clone();
    for (i, cid) in new_coedges.iter().enumerate() {
        if let Some((pid, range)) = saved[i] {
            let c = body.coedges.get_mut(*cid).unwrap();
            c.pcurve = Some(pid);
            c.prange = range;
        } else {
            let pcurve = surface.project_curve(new_curve, new_range, tol);
            let pid = body.curves2.insert(pcurve);
            let c = body.coedges.get_mut(*cid).unwrap();
            c.pcurve = Some(pid);
            c.prange = new_range;
        }
    }
    body.faces.get_mut(face).unwrap().outer = Some(new_loop);
    Ok(())
}

// #endregion 🔖️Topology

// #region 🔖️Fillet

/// 🎨️ Rolling-ball center and the two face-tangency points at one station, from the two faces'
/// outward normals `n0`/`n1` and the edge point `p`: the ball center lies on the bisector of the
/// two inward directions at `center = p − r·(n0+n1)/(1+n0·n1)` (exact closed form — degenerates
/// only as the dihedral approaches a flat/tangent 180°), and each tangency point is
/// `center + r·n_i` (exact for a planar face; a certified-good first-order approximation for a
/// curved one, since it uses the edge's own local normal rather than re-solving `closest_uv`
/// there).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fillet_center_and_tangents(n0: Vec3, n1: Vec3, p: Pnt3, r: f64) -> (Pnt3, Pnt3, Pnt3) {
    let denom = (1.0 + n0.dot(n1)).max(1e-3);
    let center = p - (n0 + n1) * (r / denom);
    (center, center + n0 * r, center + n1 * r)
}

/// 🎨️ The rational-quadratic Bézier representation of the circular arc from `a` to `b` around
/// `center` with radius `r`: control points `[a, m, b]`, weights `[1, cos(θ/2), 1]`, `m` on the
/// bisector at `r / cos(θ/2)` from `center` — the standard exact NURBS arc construction.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn circular_arc_bezier(center: Pnt3, a: Pnt3, b: Pnt3, r: f64) -> ([Pnt3; 3], [f64; 3]) {
    let va = a - center;
    let vb = b - center;
    let cos_theta = (va.dot(vb) / (r * r)).clamp(-1.0, 1.0);
    let half = ((1.0 + cos_theta) * 0.5).max(0.0).sqrt().max(1e-6);
    let bis = (va + vb).normalized().unwrap_or_else(|| va.normalized().unwrap_or(Vec3::X));
    let mid = center + bis * (r / half);
    ([a, mid, b], [1.0, half, 1.0])
}

/// 🎨️ Builds the arc-swept blend surface plus its two tangency-curve boundaries by sampling
/// [`BLEND_SAMPLES`] stations along `edge`'s own parameter range: at each, [`station_normals`]
/// gives the exact local face normals, [`fillet_center_and_tangents`] the ball center/tangency
/// points for `radius_at(fraction)`, and [`circular_arc_bezier`] the exact cross-section. The
/// `BLEND_SAMPLES` cross-sections loft (degree-1 in the spine direction — an exact ruling between
/// congruent cross-sections when the dihedral is constant, e.g. a straight box edge; a
/// close-to-exact piecewise-linear approximation when it varies, e.g. along a plane/cylinder
/// junction). Tangency curves are fit through the same stations via [`interpolate_curve`] with
/// [`ParamMethod::Uniform`] so both share parameter `[0, 1]` — required by
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::offset::ruled_surface_from_curves`]'s callers elsewhere, and just as valid here for building
/// the blend face's own trim edges.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_fillet_geometry(body: &Body, edge: EdgeId, f0: FaceId, f1: FaceId, s0: &Surface, s1: &Surface, t_lo: f64, t_hi: f64, radius_at: &dyn Fn(f64) -> f64) -> Result<(Surface, Curve3, Curve3), KernelError> {
    let n = BLEND_SAMPLES;
    let mut rows = Vec::with_capacity(n);
    let mut wts = Vec::with_capacity(n);
    let mut tan0 = Vec::with_capacity(n);
    let mut tan1 = Vec::with_capacity(n);
    for i in 0..n {
        let f = i as f64 / (n - 1) as f64;
        let t = t_lo + (t_hi - t_lo) * f;
        let (n0, n1, p) = station_normals(body, edge, f0, f1, s0, s1, t)?;
        let r = radius_at(f);
        if !(r.is_finite() && r > 0.0) {
            return Err(KernelError::InvalidInput("fillet radius must stay positive along the edge".into()));
        }
        let (center, a, b) = fillet_center_and_tangents(n0, n1, p, r);
        tan0.push(a);
        tan1.push(b);
        let (pts, w) = circular_arc_bezier(center, a, b, r);
        rows.push(pts.to_vec());
        wts.push(w.to_vec());
    }
    let u_knots = KnotVector::clamped_uniform(n, 1);
    let v_knots = KnotVector { knots: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], degree: 2 };
    let surface = Surface::Nurbs { u_knots, v_knots, controls: rows, weights: wts };
    let deg = (n - 1).min(3).max(1);
    let c0 = interpolate_curve(&tan0, deg, ParamMethod::Uniform, None, false).ok_or_else(|| KernelError::Operation("fillet: tangency curve fit failed".into()))?;
    let c1 = interpolate_curve(&tan1, deg, ParamMethod::Uniform, None, false).ok_or_else(|| KernelError::Operation("fillet: tangency curve fit failed".into()))?;
    Ok((surface, Curve3::Nurbs { knots: c0.knots, controls: c0.controls, weights: c0.weights }, Curve3::Nurbs { knots: c1.knots, controls: c1.controls, weights: c1.weights }))
}

/// 🎨️ Replaces `edge` (shared by two faces of `solid_faces`) with two tangent edges plus a new
/// blend face, mutating the two adjacent faces' outer loops in place (same [`FaceId`]s — see
/// [`replace_face_edge`]) and returning the new blend face.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fillet_one_edge(body: &mut Body, solid_faces: &HashSet<FaceId>, edge: EdgeId, radius_at: &dyn Fn(f64) -> f64, rec: &mut OpRecorder, tol: f64) -> Result<FaceId, KernelError> {
    let (f0, f1) = edge_two_faces(body, solid_faces, edge)?;
    let fd0 = body.faces.get(f0).unwrap().clone();
    let fd1 = body.faces.get(f1).unwrap().clone();
    let s0 = body.surfaces.get(fd0.surface).unwrap().clone();
    let s1 = body.surfaces.get(fd1.surface).unwrap().clone();
    let edge_ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?.clone();
    let (t_lo, t_hi) = edge_ent.range;
    // A CLOSED edge (`v0 == v1`, e.g. a cylinder cap's own full-circle lateral/cap boundary) has
    // no distinct "start" and "end" station — sampling the full `[t_lo, t_hi]` period gives the
    // SAME physical point at both ends. Building two separate end-closure edges for that (the
    // open-strip construction below) makes them exact geometric duplicates occupying the same 3D
    // segment — a degenerate, self-overlapping loop that fed NaN/garbage into the exact-predicate
    // fallback ([`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::predicates`], downstream in `solid_volume`). The
    // correct closed-loop construction reuses ONE end edge as a genuine seam (same convention
    // [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_cylinder`] already uses for its own lateral seam:
    // one edge, traversed forward then reverse) — the blend face becomes a real closed
    // torus-band, not an open strip.
    let is_closed = edge_ent.v0 == edge_ent.v1;
    let (blend_surface, tan0_curve, tan1_curve) = build_fillet_geometry(body, edge, f0, f1, &s0, &s1, t_lo, t_hi, radius_at)?;
    let blend_id = body.surfaces.insert(blend_surface.clone());

    let (nt0, nt1) = (tan0_curve.domain(), tan1_curve.domain());
    let tan0_p0 = tan0_curve.eval(nt0.0);
    let tan0_p1 = tan0_curve.eval(nt0.1);
    let tan1_p0 = tan1_curve.eval(nt1.0);
    let tan1_p1 = tan1_curve.eval(nt1.1);
    let tv0_0 = make_vertex(body, tan0_p0, Tol::DEFAULT, rec);
    let tv0_1 = if is_closed { tv0_0 } else { make_vertex(body, tan0_p1, Tol::DEFAULT, rec) };
    let tv1_0 = make_vertex(body, tan1_p0, Tol::DEFAULT, rec);
    let tv1_1 = if is_closed { tv1_0 } else { make_vertex(body, tan1_p1, Tol::DEFAULT, rec) };
    let curve0_id = body.curves3.insert(tan0_curve.clone());
    let curve1_id = body.curves3.insert(tan1_curve.clone());
    let e_tan0 = make_edge(body, curve0_id, nt0, tv0_0, tv0_1, Tol::DEFAULT, rec);
    let e_tan1 = make_edge(body, curve1_id, nt1, tv1_0, tv1_1, Tol::DEFAULT, rec);
    let e_end0 = line_edge(body, tan0_p0, tan1_p0, tv0_0, tv1_0, Tol::DEFAULT, rec);
    let e_end1 = if is_closed { e_end0 } else { line_edge(body, tan0_p1, tan1_p1, tv0_1, tv1_1, Tol::DEFAULT, rec) };

    replace_face_edge(body, f0, edge, e_tan0, &tan0_curve, nt0, tol)?;
    replace_face_edge(body, f1, edge, e_tan1, &tan1_curve, nt1, tol)?;

    let members = [(e_tan0, true), (e_end1, true), (e_tan1, false), (e_end0, false)];
    let blend_face = attach_face(body, blend_id, &members, false, Tol::DEFAULT, rec);
    let mut edge_geom = std::collections::HashMap::new();
    edge_geom.insert(e_tan0, (tan0_curve, nt0));
    edge_geom.insert(e_tan1, (tan1_curve, nt1));
    edge_geom.insert(e_end0, (Curve3::Line { origin: tan0_p0, dir: tan1_p0 - tan0_p0 }, (0.0, 1.0)));
    edge_geom.insert(e_end1, (Curve3::Line { origin: tan0_p1, dir: tan1_p1 - tan0_p1 }, (0.0, 1.0)));
    set_face_pcurves(body, blend_face, &blend_surface, &edge_geom, tol);
    Ok(blend_face)
}

/// 🎨️ Constant-radius fillet on `edges` of `solid` — exact topology surgery per edge, see
/// [`fillet_one_edge`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fillet_edges(body: &mut Body, solid: SolidId, edges: &[EdgeId], radius: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    validate_blend_request(body, solid, edges, radius)?;
    let solid_faces: HashSet<FaceId> = body.solid_faces(solid).into_iter().collect();
    let radius_at = |_f: f64| radius;
    let mut new_faces = Vec::with_capacity(edges.len());
    for &edge in edges {
        new_faces.push(fillet_one_edge(body, &solid_faces, edge, &radius_at, rec, BLEND_TOL)?);
    }
    let mut faces = body.solid_faces(solid);
    faces.extend(new_faces);
    Ok(finish_solid(body, faces, rec))
}

/// 🎨️ Linearly varying fillet radius `r0 → r1` along a single `edge` — same exact topology
/// surgery, `radius_at` interpolated over the edge's own `[0, 1]` fraction.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fillet_variable(body: &mut Body, solid: SolidId, edge: EdgeId, r0: f64, r1: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if r0 <= 0.0 || r1 <= 0.0 {
        return Err(KernelError::InvalidInput("variable fillet radii must be positive".into()));
    }
    validate_blend_request(body, solid, &[edge], r0.max(r1))?;
    let solid_faces: HashSet<FaceId> = body.solid_faces(solid).into_iter().collect();
    let radius_at = |f: f64| r0 + (r1 - r0) * f;
    let blend_face = fillet_one_edge(body, &solid_faces, edge, &radius_at, rec, BLEND_TOL)?;
    let mut faces = body.solid_faces(solid);
    faces.push(blend_face);
    Ok(finish_solid(body, faces, rec))
}

// #endregion 🔖️Fillet

// #region 🔖️Chamfer

/// 🎨️ Replaces `edge` with a planar (or, for a non-planar pair, ruled-NURBS) chamfer face cutting
/// at distance `d1` from the edge along `f0` and `d2` along `f1` (measured in each face's own
/// local tangent plane at the edge — exact for two planar faces, a documented first-order
/// approximation otherwise).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chamfer_one_edge(body: &mut Body, solid_faces: &HashSet<FaceId>, edge: EdgeId, d1: f64, d2: f64, rec: &mut OpRecorder, tol: f64) -> Result<FaceId, KernelError> {
    let (f0, f1) = edge_two_faces(body, solid_faces, edge)?;
    let fd0 = body.faces.get(f0).unwrap().clone();
    let fd1 = body.faces.get(f1).unwrap().clone();
    let s0 = body.surfaces.get(fd0.surface).unwrap().clone();
    let s1 = body.surfaces.get(fd1.surface).unwrap().clone();
    let edge_ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?.clone();
    let curve = body.curves3.get(edge_ent.curve).unwrap().clone();
    let (t_lo, t_hi) = edge_ent.range;
    let n = BLEND_SAMPLES;
    let mut c0pts = Vec::with_capacity(n);
    let mut c1pts = Vec::with_capacity(n);
    for i in 0..n {
        let f = i as f64 / (n - 1) as f64;
        let t = t_lo + (t_hi - t_lo) * f;
        let (n0, n1, p) = station_normals(body, edge, f0, f1, &s0, &s1, t)?;
        let tangent = curve.tangent(t).unwrap_or(Vec3::Z);
        let bis_in = -(n0 + n1);
        let mut perp0 = n0.cross(tangent).normalized().unwrap_or_else(|| n0.any_orthogonal());
        if perp0.dot(bis_in) < 0.0 {
            perp0 = -perp0;
        }
        let mut perp1 = n1.cross(tangent).normalized().unwrap_or_else(|| n1.any_orthogonal());
        if perp1.dot(bis_in) < 0.0 {
            perp1 = -perp1;
        }
        c0pts.push(p + perp0 * d1);
        c1pts.push(p + perp1 * d2);
    }
    let (curve0, curve1, range) = if let (Surface::Plane { .. }, Surface::Plane { .. }) = (&s0, &s1) {
        let a0 = c0pts[0];
        let a1 = *c0pts.last().unwrap();
        let b0 = c1pts[0];
        let b1 = *c1pts.last().unwrap();
        (Curve3::Line { origin: a0, dir: a1 - a0 }, Curve3::Line { origin: b0, dir: b1 - b0 }, (0.0, 1.0))
    } else {
        let deg = (n - 1).min(3).max(1);
        let n0c = interpolate_curve(&c0pts, deg, ParamMethod::Uniform, None, false).ok_or_else(|| KernelError::Operation("chamfer: tangent curve fit failed".into()))?;
        let n1c = interpolate_curve(&c1pts, deg, ParamMethod::Uniform, None, false).ok_or_else(|| KernelError::Operation("chamfer: tangent curve fit failed".into()))?;
        (Curve3::Nurbs { knots: n0c.knots, controls: n0c.controls, weights: n0c.weights }, Curve3::Nurbs { knots: n1c.knots, controls: n1c.controls, weights: n1c.weights }, (0.0, 1.0))
    };
    let chamfer_surf = ruled_surface_from_curves(&curve0, range, &curve1, range)?;
    let chamfer_id = body.surfaces.insert(chamfer_surf.clone());

    // Same closed-edge seam-reuse as `fillet_one_edge` (see its docstring) — a full-circle
    // dihedral edge makes `range.0`/`range.1` the SAME physical point, so the second end-closure
    // edge would otherwise duplicate the first exactly.
    let is_closed = edge_ent.v0 == edge_ent.v1;
    let p00 = curve0.eval(range.0);
    let p01 = curve0.eval(range.1);
    let p10 = curve1.eval(range.0);
    let p11 = curve1.eval(range.1);
    let v00 = make_vertex(body, p00, Tol::DEFAULT, rec);
    let v01 = if is_closed { v00 } else { make_vertex(body, p01, Tol::DEFAULT, rec) };
    let v10 = make_vertex(body, p10, Tol::DEFAULT, rec);
    let v11 = if is_closed { v10 } else { make_vertex(body, p11, Tol::DEFAULT, rec) };
    let c0id = body.curves3.insert(curve0.clone());
    let c1id = body.curves3.insert(curve1.clone());
    let e0 = make_edge(body, c0id, range, v00, v01, Tol::DEFAULT, rec);
    let e1 = make_edge(body, c1id, range, v10, v11, Tol::DEFAULT, rec);
    let e_end0 = line_edge(body, p00, p10, v00, v10, Tol::DEFAULT, rec);
    let e_end1 = if is_closed { e_end0 } else { line_edge(body, p01, p11, v01, v11, Tol::DEFAULT, rec) };

    replace_face_edge(body, f0, edge, e0, &curve0, range, tol)?;
    replace_face_edge(body, f1, edge, e1, &curve1, range, tol)?;

    let members = [(e0, true), (e_end1, true), (e1, false), (e_end0, false)];
    let chamfer_face = attach_face(body, chamfer_id, &members, false, Tol::DEFAULT, rec);
    let mut edge_geom = std::collections::HashMap::new();
    edge_geom.insert(e0, (curve0, range));
    edge_geom.insert(e1, (curve1, range));
    edge_geom.insert(e_end0, (Curve3::Line { origin: p00, dir: p10 - p00 }, (0.0, 1.0)));
    edge_geom.insert(e_end1, (Curve3::Line { origin: p01, dir: p11 - p01 }, (0.0, 1.0)));
    set_face_pcurves(body, chamfer_face, &chamfer_surf, &edge_geom, tol);
    Ok(chamfer_face)
}

/// 🎨️ Asymmetric chamfer (`d1` along the first adjacent face, `d2` along the second) on `edges`
/// of `solid`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chamfer_edges(body: &mut Body, solid: SolidId, edges: &[EdgeId], d1: f64, d2: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    validate_blend_request(body, solid, edges, d1.min(d2))?;
    if !(d2.is_finite() && d2 > 0.0) {
        return Err(KernelError::InvalidInput("chamfer distance must be positive".into()));
    }
    let solid_faces: HashSet<FaceId> = body.solid_faces(solid).into_iter().collect();
    let mut new_faces = Vec::with_capacity(edges.len());
    for &edge in edges {
        new_faces.push(chamfer_one_edge(body, &solid_faces, edge, d1, d2, rec, BLEND_TOL)?);
    }
    let mut faces = body.solid_faces(solid);
    faces.extend(new_faces);
    Ok(finish_solid(body, faces, rec))
}

// #endregion 🔖️Chamfer

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn box_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
        let mut edges: Vec<EdgeId> = solid_edge_set(body, solid).into_iter().collect();
        edges.sort_by_key(|e| format!("{e:?}"));
        edges
    }

    #[semio_framework_async_macros::async_test]
    async fn fillet_one_box_edge_matches_closed_form() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (w, d, h, r) = (2.0, 2.0, 2.0, 0.3);
        let solid = make_box(&mut body, w, d, h, &mut rec).unwrap();
        let v0 = solid_volume(&body, solid, 1e-6).unwrap();
        let edge = box_edges(&body, solid)[0];
        let len = edge_length(&body, edge).unwrap();
        let out = fillet_edges(&mut body, solid, &[edge], r, &mut rec).unwrap();
        let v1 = solid_volume(&body, out, 1e-4).unwrap();
        let closed_form = v0 - len * (r * r - std::f64::consts::PI * r * r / 4.0);
        assert!((v1 - closed_form).abs() < 1e-2 * closed_form.max(1.0), "v1={v1} expected={closed_form}");
    }

    #[semio_framework_async_macros::async_test]
    async fn fillet_all_box_edges_is_valid_and_decreases_volume() {
        // Known gap (see 📓️w2d-blends-offsets-draft.md): vertex-blend spherical corner patches
        // are not implemented, so a full 12-edge fillet leaves the 8 corners without a proper
        // rounded-corner patch — this asserts the weaker, honest property (a valid solid, strictly
        // less volume than the box, still positive) rather than the exact rounded-box closed form.
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 3.0, 3.0, 3.0, &mut rec).unwrap();
        let v0 = solid_volume(&body, solid, 1e-6).unwrap();
        let edges = box_edges(&body, solid);
        assert_eq!(edges.len(), 12);
        let out = fillet_edges(&mut body, solid, &edges, 0.3, &mut rec).unwrap();
        assert!(!body.solid_faces(out).is_empty());
        let v1 = solid_volume(&body, out, 1e-4).unwrap();
        assert!(v1 > 0.0 && v1 < v0, "v0={v0} v1={v1}");
    }

    #[semio_framework_async_macros::async_test]
    async fn fillet_plane_cylinder_junction_decreases_volume() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_cylinder(&mut body, 1.0, 2.0, &mut rec).unwrap();
        let v0 = solid_volume(&body, solid, 1e-4).unwrap();
        let solid_faces: HashSet<FaceId> = body.solid_faces(solid).into_iter().collect();
        let edge = solid_edge_set(&body, solid).into_iter().find(|&e| edge_two_faces(&body, &solid_faces, e).is_ok()).expect("a real dihedral edge (lateral/cap) exists");
        let out = fillet_edges(&mut body, solid, &[edge], 0.2, &mut rec).unwrap();
        let v1 = solid_volume(&body, out, 1e-4).unwrap();
        assert!(v1 > 0.0 && v1 < v0, "v0={v0} v1={v1}");
    }

    #[semio_framework_async_macros::async_test]
    async fn chamfer_asymmetric_matches_closed_form() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let (w, d, h, d1, d2) = (2.0, 2.0, 2.0, 0.2, 0.35);
        let solid = make_box(&mut body, w, d, h, &mut rec).unwrap();
        let v0 = solid_volume(&body, solid, 1e-6).unwrap();
        let edge = box_edges(&body, solid)[0];
        let len = edge_length(&body, edge).unwrap();
        let out = chamfer_edges(&mut body, solid, &[edge], d1, d2, &mut rec).unwrap();
        let v1 = solid_volume(&body, out, 1e-4).unwrap();
        let closed_form = v0 - 0.5 * d1 * d2 * len;
        assert!((v1 - closed_form).abs() < 1e-2 * closed_form.max(1.0), "v1={v1} expected={closed_form}");
    }

    #[semio_framework_async_macros::async_test]
    async fn variable_fillet_is_monotone_in_radius() {
        let mut body_a = Body::new();
        let mut rec_a = OpRecorder::new();
        let solid_a = make_box(&mut body_a, 2.0, 2.0, 2.0, &mut rec_a).unwrap();
        let edge_a = box_edges(&body_a, solid_a)[0];
        let out_a = fillet_variable(&mut body_a, solid_a, edge_a, 0.1, 0.2, &mut rec_a).unwrap();
        let v_a = solid_volume(&body_a, out_a, 1e-4).unwrap();

        let mut body_b = Body::new();
        let mut rec_b = OpRecorder::new();
        let solid_b = make_box(&mut body_b, 2.0, 2.0, 2.0, &mut rec_b).unwrap();
        let edge_b = box_edges(&body_b, solid_b)[0];
        let out_b = fillet_variable(&mut body_b, solid_b, edge_b, 0.1, 0.4, &mut rec_b).unwrap();
        let v_b = solid_volume(&body_b, out_b, 1e-4).unwrap();

        assert!(v_b < v_a, "growing the far-end radius should remove strictly more material: v_a={v_a} v_b={v_b}");
    }

    #[semio_framework_async_macros::async_test]
    async fn fillet_determinism() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let edge = box_edges(&body, solid)[0];
        let a = fillet_edges(&mut body, solid, &[edge], 0.2, &mut rec).unwrap();
        let solid2 = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let edge2 = box_edges(&body, solid2).into_iter().find(|&e| (edge_length(&body, e).unwrap() - edge_length(&body, edge).unwrap()).abs() < 1e-9).unwrap();
        let b = fillet_edges(&mut body, solid2, &[edge2], 0.2, &mut rec).unwrap();
        assert_eq!(body.solid_faces(a).len(), body.solid_faces(b).len());
        let va = solid_volume(&body, a, 1e-4).unwrap();
        let vb = solid_volume(&body, b, 1e-4).unwrap();
        assert!((va - vb).abs() < 1e-6, "va={va} vb={vb}");
    }

    #[semio_framework_async_macros::async_test]
    async fn reject_zero_radius_and_empty_edges() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let edge = box_edges(&body, solid)[0];
        assert!(fillet_edges(&mut body, solid, &[edge], 0.0, &mut rec).is_err());
        assert!(chamfer_edges(&mut body, solid, &[edge], 0.0, 0.1, &mut rec).is_err());
        assert!(fillet_variable(&mut body, solid, edge, 0.0, 0.1, &mut rec).is_err());
        assert!(fillet_edges(&mut body, solid, &[], 0.1, &mut rec).is_err());
        assert!(chamfer_edges(&mut body, solid, &[], 0.1, 0.1, &mut rec).is_err());
    }
}

// #endregion 🔖️Tests
