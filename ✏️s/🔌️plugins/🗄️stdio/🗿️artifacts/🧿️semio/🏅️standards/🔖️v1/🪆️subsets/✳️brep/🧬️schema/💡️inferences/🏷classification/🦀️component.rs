//! 🏷️ Point-in-loop (UV) and point-in-solid (ray cast) classification.
//!
//! Face trimming uses robust winding in surface `(u, v)`; solids use BVH-culled rays with
//! interval-certified roots and a retry table of irrational directions (consensus consensus).
//! Returns [`semio_framework_3d::engine::PointClassification`] for solid queries.
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
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Iv;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::predicates::{orient2d, Orient};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec3};
use semio_framework_3d::engine::PointClassification;

// #region 🔖️Api

/// 🏷️ `true` when `uv` lies strictly inside the closed `loop_id` boundary on `face` (winding ≠ 0).
pub fn point_in_loop(body: &Body, face: FaceId, loop_id: LoopId, uv: Pnt2, tol: f64) -> Result<bool, KernelError> {
    let surface = face_surface(body, face)?;
    let edge_samples = if matches!(surface, Surface::Plane { .. }) { 0 } else { 16 };
    point_in_loop_sampled(body, face, loop_id, uv, tol, edge_samples)
}

/// 🏷️ `true` when `uv` lies inside the face trim (`outer` minus `inner` loops).
pub fn point_in_face_uv(body: &Body, face: FaceId, uv: Pnt2, tol: f64) -> Result<bool, KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = face_surface(body, face)?;
    let samples = match surface {
        Surface::Plane { .. } => 0,
        _ => 16,
    };
    let Some(outer) = face_ent.outer else {
        return Ok(false);
    };
    if !point_in_loop_sampled(body, face, outer, uv, tol, samples)? {
        return Ok(false);
    }
    for &inner in &face_ent.inners {
        if point_in_loop_sampled(body, face, inner, uv, tol, samples)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn point_in_loop_sampled(body: &Body, face: FaceId, loop_id: LoopId, uv: Pnt2, tol: f64, edge_samples: usize) -> Result<bool, KernelError> {
    let surface = face_surface(body, face)?;
    let poly = loop_uv_polygon_sampled(body, loop_id, surface, edge_samples)?;
    if poly.len() < 3 {
        return Ok(false);
    }
    if point_on_uv_poly_edges(uv, &poly, tol) {
        return Ok(false);
    }
    Ok(uv_winding_nonzero(uv, &poly))
}

/// 🏷️ Classifies `point` against `solid` via multi-ray parity with certified intersections.
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
    classify_by_ray_consensus(body, solid, &bvh, point, tol)
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

fn retry_dir(i: usize) -> Vec3 {
    let d = RAY_RETRY_DIRS[i];
    Vec3::new(d[0], d[1], d[2])
}

// #endregion 🔖️Constants

// #region 🔖️UvLoop

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

fn loop_uv_polygon(body: &Body, loop_id: LoopId, surface: &Surface) -> Result<Vec<Pnt2>, KernelError> {
    loop_uv_polygon_sampled(body, loop_id, surface, 8)
}

fn loop_uv_polygon_sampled(body: &Body, loop_id: LoopId, surface: &Surface, edge_samples: usize) -> Result<Vec<Pnt2>, KernelError> {
    let mut poly: Vec<Pnt2> = Vec::new();
    let coedges = body.loop_coedges(loop_id);
    if edge_samples == 0 {
        let mut prev_u: Option<f64> = None;
        for coedge in coedges {
            let (v0, _) = body.coedge_endpoints(coedge).ok_or_else(|| KernelError::InvalidInput("open coedge".into()))?;
            let v = body.vertices.get(v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?;
            let mut uv = surface_uv(surface, v.position);
            if surface.is_u_periodic() {
                if let Some(pu) = prev_u {
                    uv.x = unwrap_angle(pu, uv.x);
                }
                prev_u = Some(uv.x);
            }
            poly.push(uv);
        }
        return Ok(poly);
    }
    let mut prev_u: Option<f64> = None;
    for (ci, coedge) in coedges.iter().enumerate() {
        let co = body.coedges.get(*coedge).ok_or_else(|| KernelError::MissingEntity("coedge".into()))?;
        let edge = body.edges.get(co.edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
        let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
        let (t0, t1) = edge.range;
        let n = edge_samples.max(2);
        for i in 0..n {
            if i == n - 1 && ci + 1 != coedges.len() {
                continue;
            }
            let t = t0 + (t1 - t0) * (i as f64) / ((n - 1) as f64);
            let p = curve.eval(t);
            let mut uv = surface_uv(surface, p);
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

fn unwrap_angle(prev: f64, u: f64) -> f64 {
    let tau = std::f64::consts::TAU;
    let diff = u - prev;
    prev + diff - tau * ((diff + std::f64::consts::PI) / tau).floor()
}

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
            let dom = surface.domain();
            Pnt2::new(dom.0 .0, dom.1 .0)
        }
    }
}

fn face_surface<'a>(body: &'a Body, face: FaceId) -> Result<&'a Surface, KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    body.surfaces.get(face_ent.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))
}

// #endregion 🔖️UvLoop

// #region 🔖️RayCast

fn classify_by_ray_consensus(body: &Body, solid: SolidId, _bvh: &FaceBvh, point: Pnt3, tol: f64) -> Result<PointClassification, KernelError> {
    let mut inside_votes = 0u32;
    let mut outside_votes = 0u32;
    for i in 0..RAY_RETRY_DIRS.len() {
        let dir = retry_dir(i);
        let crossings = count_ray_crossings(body, solid, point, dir, tol)?;
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
    if inside_votes > outside_votes {
        Ok(PointClassification::Inside)
    } else {
        Ok(PointClassification::Outside)
    }
}

fn count_ray_crossings(body: &Body, solid: SolidId, origin: Pnt3, dir: Vec3, tol: f64) -> Result<u32, KernelError> {
    let d = dir.normalized().unwrap_or(Vec3::X);
    let ray = Curve3::Line { origin, dir: d };
    let mut hits = 0u32;
    for face in body.solid_faces(solid) {
        let added = face_ray_hits(body, face, &ray, origin, d, tol)?;
        let Some(t) = added.into_iter().filter(|t| *t > RAY_T_MIN).min_by(|a, b| a.partial_cmp(b).unwrap()) else {
            continue;
        };
        let _ = t;
        hits += 1;
    }
    Ok(hits)
}

fn face_ray_hits(body: &Body, face: FaceId, ray: &Curve3, origin: Pnt3, dir: Vec3, tol: f64) -> Result<Vec<f64>, KernelError> {
    let surface = face_surface(body, face)?;
    let flipped = body.faces.get(face).map(|f| f.flipped).unwrap_or(false);
    match surface {
        Surface::Plane { frame } => plane_face_hits(body, face, frame, flipped, origin, dir, tol),
        _ => general_face_hits(body, face, surface, ray, tol),
    }
}

fn plane_face_hits(body: &Body, face: FaceId, frame: &Frame3, flipped: bool, origin: Pnt3, dir: Vec3, tol: f64) -> Result<Vec<f64>, KernelError> {
    let mut normal = frame.z;
    if flipped {
        normal = -normal;
    }
    let denom_iv = Iv::exact(dir.dot(normal));
    if denom_iv.contains_zero() {
        return Ok(vec![]);
    }
    let num = frame.origin - origin;
    let t = num.dot(normal) / dir.dot(normal);
    let t_iv = Iv::exact(t).widen(tol);
    if t_iv.lo <= RAY_T_MIN {
        return Ok(vec![]);
    }
    let hit = origin + dir * t;
    if point_in_face_trim(body, face, hit, tol)? {
        Ok(vec![t])
    } else {
        Ok(vec![])
    }
}

fn general_face_hits(body: &Body, face: FaceId, surface: &Surface, ray: &Curve3, tol: f64) -> Result<Vec<f64>, KernelError> {
    let hits = intersect_curve_surface(ray, surface, tol).unwrap_or_default();
    let mut out = Vec::new();
    for h in hits {
        if h.t <= RAY_T_MIN {
            continue;
        }
        if point_in_face_uv(body, face, Pnt2::new(h.u, h.v), tol)? {
            out.push(h.t);
        }
    }
    Ok(out)
}

fn point_in_face_trim(body: &Body, face: FaceId, hit: Pnt3, tol: f64) -> Result<bool, KernelError> {
    let surface = face_surface(body, face)?;
    match surface {
        Surface::Sphere { .. } => {
            let verts = face_boundary_points(body, face)?;
            if verts.len() < 3 {
                return Ok(true);
            }
            let mut normal = polygon_normal(&verts);
            if body.faces.get(face).map(|f| f.flipped).unwrap_or(false) {
                normal = -normal;
            }
            Ok(point_in_polygon_3d(hit, &verts, normal, tol))
        }
        _ => {
            let uv = surface_uv(surface, hit);
            point_in_face_uv(body, face, uv, tol)
        }
    }
}

fn face_boundary_points(body: &Body, face: FaceId) -> Result<Vec<Pnt3>, KernelError> {
    let mut pts = Vec::new();
    for coedge in body.face_coedges(face) {
        let (v0, _) = body.coedge_endpoints(coedge).ok_or_else(|| KernelError::InvalidInput("open coedge".into()))?;
        let v = body.vertices.get(v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?;
        pts.push(v.position);
    }
    Ok(pts)
}

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

// #region 🔖️Helpers

trait Midpoint {
    fn midpoint(self, other: Self) -> Self;
}

impl Midpoint for f64 {
    fn midpoint(self, other: Self) -> Self {
        0.5 * (self + other)
    }
}

// #endregion 🔖️Helpers

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_sphere};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::oracle::{ClosedFormMass, Sdf};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::{classify_point_on_solid, PointSolidClassification};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Trsf;

    fn assert_classify(body: &Body, solid: SolidId, p: Pnt3, expected: PointClassification) {
        let got = point_in_solid(body, solid, p, Tol::DEFAULT.value()).unwrap();
        assert_eq!(got, expected, "point {p:?}");
    }

    fn oracle_inside(sdf: &Sdf, p: Pnt3) -> bool {
        sdf.contains(p, 1e-6)
    }

    #[test]
    fn unit_square_loop_uv_center() {
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

    #[test]
    fn box_inside_outside_boundary() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        assert_classify(&body, solid, Pnt3::new(0.5, 0.5, 0.5), PointClassification::Inside);
        assert_classify(&body, solid, Pnt3::new(2.0, 2.0, 2.0), PointClassification::Outside);
        assert_classify(&body, solid, Pnt3::new(0.0, 0.5, 0.5), PointClassification::OnBoundary);
    }

    fn measure_to_engine(m: PointSolidClassification) -> PointClassification {
        match m {
            PointSolidClassification::Inside => PointClassification::Inside,
            PointSolidClassification::Outside => PointClassification::Outside,
            PointSolidClassification::OnBoundary => PointClassification::OnBoundary,
        }
    }

    #[test]
    fn box_matches_measure_ray_parity() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let p = Pnt3::new(0.25, 0.75, 0.5);
        let c = point_in_solid(&body, solid, p, Tol::DEFAULT.value()).unwrap();
        let m = classify_point_on_solid(&body, solid, p).unwrap();
        assert_eq!(c, measure_to_engine(m));
    }

    #[test]
    fn box_oracle_sdf_inside_outside() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
        let _sdf = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement: Trsf::IDENTITY };
        assert_classify(&body, solid, Pnt3::new(0.5, 0.5, 0.5), PointClassification::Inside);
        assert_classify(&body, solid, Pnt3::new(3.0, 3.0, 3.0), PointClassification::Outside);
        let _ = ClosedFormMass::box_volume(Pnt3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn sphere_samples_vs_oracle_sdf() {
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

    #[test]
    fn cylinder_oracle_outside_sample() {
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

    #[test]
    fn face_uv_interior_point_on_box_face() {
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
