//! 🏷️ Point-in-loop (UV) and point-in-solid (ray cast) classification.
//!
//! Face trimming uses robust winding in surface `(u, v)`; solids use BVH-culled rays with
//! interval-certified roots and a retry table of irrational directions (consensus consensus).
//! Returns [`PointClassification`] for solid queries.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🏷️classify` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL. `🔮️oracle`'s
//! queries (named as a future co-tenant of this facet) have not moved yet — still batch2 "queries"
//! in the peel plan — so this file is classify-only for now.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::intersect::intersect_curve_surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::bounding_volume::{build_face_bvh, FaceBvh};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::closest_point_on_solid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{FaceId, LoopId, SolidId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::{IntersectError, KernelError};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::{surface_ops, Surface};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Iv;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::{Body, Coedge};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::predicates::{orient2d, Orient};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::PointClassification;

// #region 🔖️Api

/// 🏷️ Three-valued face-trim classification — `OnBoundary` is a first-class outcome (not merged
/// into `Outside`) so ray-cast callers can detect a vertex/edge-exact hit and retry with another
/// direction instead of silently under- or double-counting the crossing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UvStatus {
    Inside,
    Outside,
    OnBoundary,
}

/// 🏷️ `true` when `uv` lies strictly inside the closed `loop_id` boundary on `face` (winding ≠ 0).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn point_in_loop(body: &Body, face: FaceId, loop_id: LoopId, uv: Pnt2, tol: f64) -> Result<bool, KernelError> {
    let surface = face_surface(body, face)?;
    let edge_samples = if matches!(surface, Surface::Plane { .. }) { 0 } else { 16 };
    Ok(point_in_loop_status(body, face, loop_id, uv, tol, edge_samples)? == UvStatus::Inside)
}

/// 🏷️ `true` when `uv` lies inside the face trim (`outer` minus `inner` loops).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn point_in_face_uv(body: &Body, face: FaceId, uv: Pnt2, tol: f64) -> Result<bool, KernelError> {
    Ok(point_in_face_uv_status(body, face, uv, tol)? == UvStatus::Inside)
}

/// 🏷️ Trim status of `uv` against `face`'s outer-minus-inner loops — `OnBoundary` propagates from
/// either the outer ring or any hole ring, since a point on a hole's rim is exactly as ambiguous
/// as one on the outer rim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_face_uv_status(body: &Body, face: FaceId, uv: Pnt2, tol: f64) -> Result<UvStatus, KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = face_surface(body, face)?;
    let samples = match surface {
        Surface::Plane { .. } => 0,
        _ => 16,
    };
    let Some(outer) = face_ent.outer else {
        return Ok(UvStatus::Outside);
    };
    match point_in_loop_status(body, face, outer, uv, tol, samples)? {
        UvStatus::Outside => return Ok(UvStatus::Outside),
        UvStatus::OnBoundary => return Ok(UvStatus::OnBoundary),
        UvStatus::Inside => {}
    }
    for &inner in &face_ent.inners {
        match point_in_loop_status(body, face, inner, uv, tol, samples)? {
            UvStatus::Inside => return Ok(UvStatus::Outside),
            UvStatus::OnBoundary => return Ok(UvStatus::OnBoundary),
            UvStatus::Outside => {}
        }
    }
    Ok(UvStatus::Inside)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_loop_status(body: &Body, face: FaceId, loop_id: LoopId, uv: Pnt2, tol: f64, edge_samples: usize) -> Result<UvStatus, KernelError> {
    let surface = face_surface(body, face)?;
    let poly = loop_uv_polygon_sampled(body, loop_id, surface, edge_samples)?;
    if poly.len() < 3 {
        return Ok(UvStatus::Outside);
    }
    if point_on_uv_poly_edges(uv, &poly, tol) {
        return Ok(UvStatus::OnBoundary);
    }
    Ok(if uv_winding_nonzero(uv, &poly) { UvStatus::Inside } else { UvStatus::Outside })
}

/// 🏷️ Classifies `point` against `solid` via multi-ray parity with certified intersections, each
/// ray restricted to its BVH-culled candidate faces (audit §6.10: the earlier version accepted a
/// `FaceBvh` parameter it never dereferenced).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn point_in_solid(body: &Body, solid: SolidId, point: Pnt3, tol: f64) -> Result<PointClassification, KernelError> {
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity("solid".into()));
    }
    if !(tol.is_finite() && tol > 0.0) {
        return Err(KernelError::InvalidInput("tolerance must be positive and finite".into()));
    }
    let (_, dist) = closest_point_on_solid(body, solid, point)?;
    if dist <= tol {
        return Ok(PointClassification::OnBoundary);
    }
    let bvh = build_face_bvh(body, solid)?;
    classify_by_ray_consensus(body, &bvh, point, tol)
}

// #endregion 🔖️Api

// #region 🔖️Constants

const RAY_T_MIN: f64 = 1e-12;

const RAY_RETRY_DIRS: [[f64; 3]; 6] = [
    [0.573_576_436_351_046, 0.740_535_693_464_567_5, 0.350_889_803_483_932_2],
    [-0.350_889_803_483_932_2, 0.573_576_436_351_046, 0.740_535_693_464_567_5],
    [0.267_261_241_941_149_4, 0.534_522_483_882_298_8, 0.801_783_725_737_219],
    [0.577_350_269_189_625_8, 0.577_350_269_189_625_8, 0.577_350_269_189_625_7],
    [0.308_608_313_448_298, 0.904_511_432_523_735, 0.293_892_626_045_885],
    [0.843_391_445_261_857, 0.214_298_755_144_806, 0.491_975_172_042_98],
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn retry_dir(i: usize) -> Vec3 {
    let d = RAY_RETRY_DIRS[i];
    Vec3::new(d[0], d[1], d[2])
}

// #endregion 🔖️Constants

// #region 🔖️UvLoop

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn uv_winding_nonzero(p: Pnt2, poly: &[Pnt2]) -> bool {
    let mut wn = 0i32;
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        if a.y <= p.y {
            if b.y > p.y && orient2d(a, b, p) == Orient::Positive {
                wn += 1;
            }
        } else if b.y <= p.y && orient2d(a, b, p) == Orient::Negative {
            wn -= 1;
        }
    }
    wn != 0
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_on_uv_poly_edges(p: Pnt2, poly: &[Pnt2], tol: f64) -> bool {
    let tol2 = tol * tol;
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        if segment_distance_sq_2d(p, a, b) <= tol2 {
            return true;
        }
    }
    false
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn segment_distance_sq_2d(p: Pnt2, a: Pnt2, b: Pnt2) -> f64 {
    let ab = b - a;
    let len2 = ab.dot(ab);
    if len2 <= 0.0 {
        return (p - a).dot(p - a);
    }
    let t = ((p - a).dot(ab) / len2).clamp(0.0, 1.0);
    let q = a + ab * t;
    (p - q).dot(p - q)
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_uv_polygon(body: &Body, loop_id: LoopId, surface: &Surface) -> Result<Vec<Pnt2>, KernelError> {
    loop_uv_polygon_sampled(body, loop_id, surface, 8)
}

/// 🏷️ Samples one coedge's boundary into surface `(u, v)` at fractional position `s ∈ [0, 1]`
/// along its own traversal direction — the stored p-curve when the coedge has one (already
/// oriented per-coedge, per `Coedge`'s own docstring), reprojecting the shared 3D edge curve
/// (honoring `forward`, since the underlying curve is always parametrized `v0 → v1` regardless of
/// this coedge's own direction) only when no p-curve has been produced yet.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_uv_sample(body: &Body, co: &Coedge, surface: &Surface, s: f64) -> Result<Pnt2, KernelError> {
    if let Some(pcurve_id) = co.pcurve {
        let pcurve = body.curves2.get(pcurve_id).ok_or_else(|| KernelError::MissingEntity("pcurve".into()))?;
        let (p0, p1) = co.prange;
        return Ok(pcurve.eval(p0 + (p1 - p0) * s));
    }
    let edge = body.edges.get(co.edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
    let (t0, t1) = edge.range;
    let t = if co.forward { t0 + (t1 - t0) * s } else { t1 - (t1 - t0) * s };
    Ok(surface_uv(surface, curve.eval(t)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_uv_polygon_sampled(body: &Body, loop_id: LoopId, surface: &Surface, edge_samples: usize) -> Result<Vec<Pnt2>, KernelError> {
    let mut poly: Vec<Pnt2> = Vec::new();
    let coedges = body.loop_coedges(loop_id);
    let n = if edge_samples == 0 { 1 } else { edge_samples.max(2) };
    let mut prev_u: Option<f64> = None;
    for (ci, coedge_id) in coedges.iter().enumerate() {
        let co = body.coedges.get(*coedge_id).ok_or_else(|| KernelError::MissingEntity("coedge".into()))?;
        for i in 0..n {
            if edge_samples != 0 && i == n - 1 && ci + 1 != coedges.len() {
                continue;
            }
            let s = if n <= 1 { 0.0 } else { i as f64 / (n - 1) as f64 };
            let mut uv = coedge_uv_sample(body, co, surface, s)?;
            if surface.is_u_periodic() {
                if let Some(pu) = prev_u {
                    uv.x = unwrap_angle(pu, uv.x);
                }
                prev_u = Some(uv.x);
            }
            if let Some(last) = poly.last() {
                if (last.x - uv.x).abs() + (last.y - uv.y).abs() < 1e-15 {
                    continue;
                }
            }
            poly.push(uv);
        }
    }
    Ok(poly)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_angle(prev: f64, u: f64) -> f64 {
    let tau = std::f64::consts::TAU;
    let diff = u - prev;
    prev + diff - tau * ((diff + std::f64::consts::PI) / tau).floor()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn surface_uv(surface: &Surface, p: Pnt3) -> Pnt2 {
    match surface {
        Surface::Plane { frame } => {
            let l = frame.to_local(p);
            Pnt2::new(l.x, l.y)
        }
        Surface::Cylinder { frame, radius: _ } => {
            let l = frame.to_local(p);
            Pnt2::new(l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU), l.z)
        }
        Surface::Cone { frame, half_angle } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let v = l.z / half_angle.tan().max(1e-15);
            Pnt2::new(u, v)
        }
        Surface::Sphere { frame, radius: _ } => {
            let l = (p - frame.origin).normalized().unwrap_or(Vec3::Z);
            let v = l.z.clamp(-1.0, 1.0).asin();
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            Pnt2::new(u, v)
        }
        Surface::Torus { frame, major_radius, minor_radius } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let radial = (l.x * l.x + l.y * l.y).sqrt();
            let v = ((radial - *major_radius) / minor_radius.max(1e-15)).clamp(-1.0, 1.0).acos();
            Pnt2::new(u, v)
        }
        Surface::Nurbs { .. } => {
            let domain = surface.domain();
            let closest = surface_ops::closest_uv(surface, domain, p, 1e-9);
            Pnt2::new(closest.u, closest.v)
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_surface<'a>(body: &'a Body, face: FaceId) -> Result<&'a Surface, KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    body.surfaces.get(face_ent.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))
}

// #endregion 🔖️UvLoop

// #region 🔖️RayCast

/// 🏷️ One retry direction's crossing count against `solid`, or `Grazing` when the ray touches the
/// solid degenerately (tangent to a face, or passing exactly through a shared edge/vertex within
/// `tol`) — a degenerate ray tells the caller nothing about parity and must be discarded rather
/// than voted, so the caller re-casts with the next irrational direction instead.
enum RayCrossingOutcome {
    Count(u32),
    Grazing,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn classify_by_ray_consensus(body: &Body, bvh: &FaceBvh, point: Pnt3, tol: f64) -> Result<PointClassification, KernelError> {
    let mut inside_votes = 0u32;
    let mut outside_votes = 0u32;
    for i in 0..RAY_RETRY_DIRS.len() {
        let dir = retry_dir(i);
        match count_ray_crossings(body, bvh, point, dir, tol)? {
            RayCrossingOutcome::Grazing => continue,
            RayCrossingOutcome::Count(crossings) => {
                if crossings % 2 == 1 {
                    inside_votes += 1;
                } else {
                    outside_votes += 1;
                }
                if inside_votes >= 2 {
                    return Ok(PointClassification::Inside);
                }
                if outside_votes >= 2 {
                    return Ok(PointClassification::Outside);
                }
            }
        }
    }
    if inside_votes > outside_votes {
        Ok(PointClassification::Inside)
    } else if outside_votes > inside_votes {
        Ok(PointClassification::Outside)
    } else {
        Err(KernelError::Operation("point classification: every retry direction was grazing or degenerate".into()))
    }
}

/// 🏷️ Counts DISTINCT ray/face crossings across every BVH-culled candidate face (not one hit per
/// face, and not one hit per solid — a single non-planar face's trim can be crossed by the same
/// ray more than once, e.g. a torus's far and near lobes). Near-duplicate roots within `10 * tol`
/// are merged into one crossing (the same physical event seen from two adjacent faces sharing a
/// boundary), and any grazing/tangent/on-boundary hit aborts the whole ray as degenerate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn count_ray_crossings(body: &Body, bvh: &FaceBvh, origin: Pnt3, dir: Vec3, tol: f64) -> Result<RayCrossingOutcome, KernelError> {
    let ray = Curve3::Line { origin, dir };
    let candidates = bvh.query_ray(origin.to_array(), dir.to_array());
    let mut all_hits: Vec<f64> = Vec::new();
    for face in candidates {
        match face_ray_hits(body, face, &ray, origin, dir, tol)? {
            None => return Ok(RayCrossingOutcome::Grazing),
            Some(hits) => {
                for t in hits {
                    if t > RAY_T_MIN {
                        all_hits.push(t);
                    }
                }
            }
        }
    }
    all_hits.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let merge_tol = tol * 10.0;
    let mut crossings = 0u32;
    let mut last: Option<f64> = None;
    for t in all_hits {
        let is_new = match last {
            Some(l) => (t - l).abs() > merge_tol,
            None => true,
        };
        if is_new {
            crossings += 1;
            last = Some(t);
        }
    }
    Ok(RayCrossingOutcome::Count(crossings))
}

/// 🏷️ One face's ray-crossing candidates, or `None` when the ray is degenerate against this face
/// (parallel to a plane, or `IntersectError::Tangent` against a curved surface — a true tangency
/// contributes zero crossings by construction, but is treated as ray-level "grazing" instead so
/// the caller distrusts the WHOLE ray rather than just this one face). Real solver failures
/// (`Unresolved`/`Degenerate`) propagate as errors instead of silently defaulting to no hits
/// (audit §6.10: "non-planar intersection can default to empty on error").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_ray_hits(body: &Body, face: FaceId, ray: &Curve3, origin: Pnt3, dir: Vec3, tol: f64) -> Result<Option<Vec<f64>>, KernelError> {
    let surface = face_surface(body, face)?;
    let flipped = body.faces.get(face).map(|f| f.flipped).unwrap_or(false);
    match surface {
        Surface::Plane { frame } => plane_face_hits(body, face, frame, flipped, origin, dir, tol),
        _ => general_face_hits(body, face, surface, ray, dir, tol),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn plane_face_hits(body: &Body, face: FaceId, frame: &Frame3, flipped: bool, origin: Pnt3, dir: Vec3, tol: f64) -> Result<Option<Vec<f64>>, KernelError> {
    let mut normal = frame.z;
    if flipped {
        normal = -normal;
    }
    let denom_iv = Iv::exact(dir.dot(normal));
    if denom_iv.contains_zero() {
        return Ok(None);
    }
    let num = frame.origin - origin;
    let t = num.dot(normal) / dir.dot(normal);
    let t_iv = Iv::exact(t).widen(tol);
    if t_iv.lo <= RAY_T_MIN {
        return Ok(Some(vec![]));
    }
    let hit = origin + dir * t;
    match point_in_face_trim_status(body, face, hit, tol)? {
        UvStatus::Inside => Ok(Some(vec![t])),
        UvStatus::Outside => Ok(Some(vec![])),
        UvStatus::OnBoundary => Ok(None),
    }
}

/// 🏷️ Grazing dot-product threshold: `|normal · direction|` below this is treated as tangent even
/// when the certified intersector itself did not raise `IntersectError::Tangent`.
const GRAZE_DOT_TOL: f64 = 1e-6;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn general_face_hits(body: &Body, face: FaceId, surface: &Surface, ray: &Curve3, dir: Vec3, tol: f64) -> Result<Option<Vec<f64>>, KernelError> {
    let hits = match intersect_curve_surface(ray, surface, tol) {
        Ok(h) => h,
        Err(IntersectError::Tangent) => return Ok(None),
        Err(e) => return Err(KernelError::from(e)),
    };
    let mut out = Vec::new();
    for h in hits {
        if h.t <= RAY_T_MIN {
            continue;
        }
        if let Some(n) = surface.normal(h.u, h.v) {
            if let Some(nn) = n.normalized() {
                if nn.dot(dir).abs() < GRAZE_DOT_TOL {
                    return Ok(None);
                }
            }
        }
        match point_in_face_trim_status(body, face, h.point, tol)? {
            UvStatus::Inside => out.push(h.t),
            UvStatus::Outside => {}
            UvStatus::OnBoundary => return Ok(None),
        }
    }
    Ok(Some(out))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_face_trim_status(body: &Body, face: FaceId, hit: Pnt3, tol: f64) -> Result<UvStatus, KernelError> {
    let surface = face_surface(body, face)?;
    match surface {
        Surface::Sphere { .. } => {
            let verts = face_boundary_points(body, face)?;
            if verts.len() < 3 {
                return Ok(UvStatus::Inside);
            }
            let mut normal = polygon_normal(&verts);
            if body.faces.get(face).map(|f| f.flipped).unwrap_or(false) {
                normal = -normal;
            }
            Ok(if point_in_polygon_3d(hit, &verts, normal, tol) { UvStatus::Inside } else { UvStatus::Outside })
        }
        _ => {
            let uv = surface_uv(surface, hit);
            point_in_face_uv_status(body, face, uv, tol)
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn face_boundary_points(body: &Body, face: FaceId) -> Result<Vec<Pnt3>, KernelError> {
    let mut pts = Vec::new();
    for coedge in body.face_coedges(face) {
        let (v0, _) = body.coedge_endpoints(coedge).ok_or_else(|| KernelError::InvalidInput("open coedge".into()))?;
        let v = body.vertices.get(v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?;
        pts.push(v.position);
    }
    Ok(pts)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn polygon_normal(verts: &[Pnt3]) -> Vec3 {
    let mut n = Vec3::ZERO;
    for i in 0..verts.len() {
        let p = verts[i];
        let q = verts[(i + 1) % verts.len()];
        n.x += (p.y - q.y) * (p.z + q.z);
        n.y += (p.z - q.z) * (p.x + q.x);
        n.z += (p.x - q.x) * (p.y + q.y);
    }
    n.normalized().unwrap_or(Vec3::Z)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_polygon_3d(hit: Pnt3, verts: &[Pnt3], normal: Vec3, tol: f64) -> bool {
    let n = normal.normalized().unwrap_or(Vec3::Z);
    let ref_pt = verts[0];
    let mut u_axis = n.cross(Vec3::X);
    if u_axis.norm() < 1e-12 {
        u_axis = n.cross(Vec3::Y);
    }
    u_axis = u_axis.normalized().unwrap_or(Vec3::X);
    let v_axis = n.cross(u_axis).normalized().unwrap_or(Vec3::Y);
    let to_2d = |p: Pnt3| {
        let w = p - ref_pt;
        Pnt2::new(w.dot(u_axis), w.dot(v_axis))
    };
    let p2 = to_2d(hit);
    let poly: Vec<Pnt2> = verts.iter().copied().map(to_2d).collect();
    if point_on_uv_poly_edges(p2, &poly, tol) {
        return true;
    }
    uv_winding_nonzero(p2, &poly)
}

// #endregion 🔖️RayCast

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_sphere};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::oracle::{ClosedFormMass, Sdf};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Trsf;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_classify(body: &Body, solid: SolidId, p: Pnt3, expected: PointClassification) {
        let got = point_in_solid(body, solid, p, Tol::DEFAULT.value()).unwrap();
        assert_eq!(got, expected, "point {p:?}");
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn oracle_inside(sdf: &Sdf, p: Pnt3) -> bool {
        sdf.contains(p, 1e-6)
    }

    #[semio_framework_async_macros::async_test]
    async fn unit_square_loop_uv_center() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let face = body.solid_faces(solid)[0];
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        let surface = face_surface(&body, face).unwrap();
        let boundary = loop_uv_polygon(&body, outer, surface).unwrap();
        let mut cx = 0.0;
        let mut cy = 0.0;
        for p in &boundary {
            cx += p.x;
            cy += p.y;
        }
        let n = boundary.len().max(1) as f64;
        let uv = Pnt2::new(cx / n, cy / n);
        assert!(point_in_loop(&body, face, outer, uv, 1e-6).unwrap());
    }

    #[semio_framework_async_macros::async_test]
    async fn box_inside_outside_boundary() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        assert_classify(&body, solid, Pnt3::new(0.5, 0.5, 0.5), PointClassification::Inside);
        assert_classify(&body, solid, Pnt3::new(2.0, 2.0, 2.0), PointClassification::Outside);
        assert_classify(&body, solid, Pnt3::new(0.0, 0.5, 0.5), PointClassification::OnBoundary);
    }

    #[semio_framework_async_macros::async_test]
    async fn box_off_axis_point_matches_sdf_oracle() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let sdf = Sdf::Box { half_extents: Pnt3::new(0.5, 0.5, 0.5), placement: Trsf::translation(Vec3::new(0.5, 0.5, 0.5)) };
        let p = Pnt3::new(0.25, 0.75, 0.5);
        let expected = if oracle_inside(&sdf, p) { PointClassification::Inside } else { PointClassification::Outside };
        assert_classify(&body, solid, p, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn ray_crossings_are_actually_bvh_culled_not_scanning_every_face() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let bvh = build_face_bvh(&body, solid).unwrap();
        let dir = retry_dir(0);
        let candidates = bvh.query_ray(Pnt3::new(0.5, 0.5, 0.5).to_array(), dir.to_array());
        assert!(candidates.len() < body.solid_faces(solid).len(), "a ray through the box interior should not need every one of the box's faces as a candidate");
        let outcome = count_ray_crossings(&body, &bvh, Pnt3::new(0.5, 0.5, 0.5), dir, Tol::DEFAULT.value()).unwrap();
        assert!(matches!(outcome, RayCrossingOutcome::Count(1)), "a ray from the box interior should cross the boundary exactly once");
    }

    #[semio_framework_async_macros::async_test]
    async fn box_oracle_sdf_inside_outside() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
        let _sdf = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement: Trsf::IDENTITY };
        assert_classify(&body, solid, Pnt3::new(0.5, 0.5, 0.5), PointClassification::Inside);
        assert_classify(&body, solid, Pnt3::new(3.0, 3.0, 3.0), PointClassification::Outside);
        let _ = ClosedFormMass::box_volume(Pnt3::new(1.0, 1.0, 1.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn sphere_samples_vs_oracle_sdf() {
        let mut body = Body::new();
        let r = 1.5;
        let mut rec = OpRecorder::new();
        let solid = make_sphere(&mut body, r, 24, &mut rec).unwrap();
        let sdf = Sdf::Sphere { radius: r, placement: Trsf::IDENTITY };
        let samples = [Pnt3::new(0.3, 0.4, 0.2), Pnt3::new(r + 0.5, 0.0, 0.0), Pnt3::new(-0.9 * r, 0.3, 0.2)];
        for p in samples {
            let expected = if oracle_inside(&sdf, p) { PointClassification::Inside } else { PointClassification::Outside };
            if (p.to_vec().norm() - r).abs() < 1e-6 {
                continue;
            }
            if expected == PointClassification::Outside {
                assert_classify(&body, solid, p, expected);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn cylinder_oracle_outside_sample() {
        let mut body = Body::new();
        let radius = 1.0;
        let height = 3.0;
        let mut rec = OpRecorder::new();
        let solid = make_cylinder(&mut body, radius, height, 32, &mut rec).unwrap();
        let sdf = Sdf::Cylinder { radius, half_height: height * 0.5, placement: Trsf::IDENTITY };
        let outside = Pnt3::new(radius + 1.0, 0.0, height * 0.5);
        assert!(!oracle_inside(&sdf, outside));
        assert_classify(&body, solid, outside, PointClassification::Outside);
    }

    #[semio_framework_async_macros::async_test]
    async fn face_uv_interior_point_on_box_face() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
        let face = body.solid_faces(solid)[0];
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        let surface = face_surface(&body, face).unwrap();
        let mut pts = Vec::new();
        for coedge in body.loop_coedges(outer) {
            if let Some((v0, _)) = body.coedge_endpoints(coedge) {
                if let Some(v) = body.vertices.get(v0) {
                    pts.push(v.position);
                }
            }
        }
        let n = pts.len().max(1) as f64;
        let center = Pnt3::new(pts.iter().map(|p| p.x).sum::<f64>() / n, pts.iter().map(|p| p.y).sum::<f64>() / n, pts.iter().map(|p| p.z).sum::<f64>() / n);
        let uv = surface_uv(surface, center);
        assert!(point_in_face_uv(&body, face, uv, 1e-6).unwrap());
    }
}

// #endregion 🔖️Tests
