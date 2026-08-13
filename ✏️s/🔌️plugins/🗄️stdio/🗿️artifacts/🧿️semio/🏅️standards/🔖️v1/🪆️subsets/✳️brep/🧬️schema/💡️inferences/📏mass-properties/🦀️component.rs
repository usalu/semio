//! 📏️ Divergence-theorem mass properties, axis-aligned bounds, and solid distance/classify
//! queries on `SemioBrepSnapshot`'s arena `Body`. `oracle` (below) is a closed-form ground
//! truth used only by tests — deliberately independent of this module's own algorithms.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/{📏️measure,🔮️oracle}/🦀️component.rs` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL2.

// 📏 Divergence-theorem mass properties, axis-aligned bounds, and solid distance queries on [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body`].

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{CoedgeId, EdgeId, FaceId, SolidId, VertexId};
use semio_framework_3d::brep::curve::Curve3;
use semio_framework_3d::brep::curve_ops;
use semio_framework_3d::brep::error::KernelError;
use semio_framework_3d::brep::surface::Surface;
use semio_framework_3d::brep::surface_ops;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use semio_framework_3d::brep::vec::{Pnt2, Pnt3, Vec3};

// #region 🔖️Types

/// 📦 Axis-aligned bounds in model space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisAlignedBox {
    pub min: Pnt3,
    pub max: Pnt3,
}

/// 🎯 Point-in-solid classification (ray parity until Wave 3 classify lands).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointSolidClassification {
    Inside,
    Outside,
    OnBoundary,
}

// #endregion 🔖️Types

// #region 🔖️Solid

/// 📐 Signed volume of `solid` via divergence theorem surface quadrature (`V = (1/3) ∫ P·n dA`).
pub fn solid_volume(body: &Body, solid: SolidId, chord_tol: f64) -> Result<f64, KernelError> {
    if let Some(v) = try_analytic_sphere_volume(body, solid) {
        return Ok(v);
    }
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut total = 0.0;
    for face in faces {
        total += face_volume_contribution(body, face, chord_tol)?;
    }
    Ok(total.abs())
}

/// 📐 Total outer surface area of `solid`.
pub fn solid_surface_area(body: &Body, solid: SolidId, chord_tol: f64) -> Result<f64, KernelError> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut total = 0.0;
    for face in faces {
        total += face_area(body, face, chord_tol)?;
    }
    Ok(total)
}

/// 📐 Center of mass of `solid` at uniform density (tetrahedral decomposition weighted by signed volume).
pub fn solid_center_of_mass(body: &Body, solid: SolidId, chord_tol: f64) -> Result<Pnt3, KernelError> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut vol = 0.0;
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut mz = 0.0;
    for face in faces {
        let (sv, cx, cy, cz) = face_volume_moments(body, face, chord_tol)?;
        vol += sv;
        mx += cx;
        my += cy;
        mz += cz;
    }
    if vol.abs() < 1e-15 {
        return Err(KernelError::InvalidInput("solid has zero volume".into()));
    }
    let denom = 4.0 * vol;
    Ok(Pnt3::new(mx / denom, my / denom, mz / denom))
}

/// 📦 Conservative axis-aligned bounding box of `solid` (vertices plus analytic surface expansion).
pub fn solid_bounding_box(body: &Body, solid: SolidId) -> Result<AxisAlignedBox, KernelError> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut min = Pnt3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = Pnt3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    let mut any = false;
    for face in faces {
        for p in face_sample_points(body, face)? {
            any = true;
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }
        if let Some(surface) = body.faces.get(face).and_then(|f| body.surfaces.get(f.surface)) {
            expand_bbox_for_surface(&mut min, &mut max, surface);
        }
    }
    if !any {
        return Err(KernelError::InvalidInput("solid has no geometry samples".into()));
    }
    Ok(AxisAlignedBox { min, max })
}

// #endregion 🔖️Solid

// #region 🔖️FaceEdge

/// 📐 Area of one face (`outer` minus `inner` loops).
pub fn face_area(body: &Body, face: FaceId, chord_tol: f64) -> Result<f64, KernelError> {
    let Some(face_ent) = body.faces.get(face) else {
        return Err(KernelError::MissingEntity("face".into()));
    };
    let mut area = 0.0;
    if let Some(outer) = face_ent.outer {
        area += loop_area(body, face, outer, chord_tol)?;
    }
    for &inner in &face_ent.inners {
        area -= loop_area(body, face, inner, chord_tol)?;
    }
    Ok(area.abs())
}

/// 📐 Arc length of an edge over its trimmed parameter range.
pub fn edge_length(body: &Body, edge: EdgeId) -> Result<f64, KernelError> {
    let edge_ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let curve = body.curves3.get(edge_ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
    Ok(curve_ops::arc_length(curve, edge_ent.range.0, edge_ent.range.1, 1e-9))
}

// #endregion 🔖️FaceEdge

// #region 🔖️Distance

/// 📏 Minimum distance between two closed solids.
pub fn distance_solid_solid(body: &Body, a: SolidId, b: SolidId) -> Result<f64, KernelError> {
    let bb_a = solid_bounding_box(body, a)?;
    let bb_b = solid_bounding_box(body, b)?;
    let separated = axis_aligned_box_distance(&bb_a, &bb_b);
    if separated > 1e-9 {
        return Ok(separated);
    }
    let mut best = f64::INFINITY;
    for face in body.solid_faces(a) {
        for p in face_sample_points(body, face)? {
            let (_, d) = closest_point_on_solid(body, b, p)?;
            if d > 1e-9 {
                best = best.min(d);
            }
        }
    }
    for face in body.solid_faces(b) {
        for p in face_sample_points(body, face)? {
            let (_, d) = closest_point_on_solid(body, a, p)?;
            if d > 1e-9 {
                best = best.min(d);
            }
        }
    }
    if !best.is_finite() {
        return Ok(0.0);
    }
    Ok(best)
}

/// 📏 Closest point on `solid` to `point` and the Euclidean distance.
pub fn closest_point_on_solid(body: &Body, solid: SolidId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut best_p = point;
    let mut best_d = f64::INFINITY;
    for face in faces {
        let (p, d) = closest_point_on_face(body, face, point)?;
        if d < best_d {
            best_d = d;
            best_p = p;
        }
    }
    Ok((best_p, best_d))
}

// #endregion 🔖️Distance

// #region 🔖️Classify

/// 🎯 Classifies `point` against `solid` via multi-ray parity (Wave 3: delegate to `classify` module).
pub fn classify_point_on_solid(body: &Body, solid: SolidId, point: Pnt3) -> Result<PointSolidClassification, KernelError> {
    const RAY_DIRS: [Vec3; 3] = [Vec3::X, Vec3::Y, Vec3::Z];
    let mut boundary_hits = 0;
    let mut parity_votes = 0;
    for dir in RAY_DIRS {
        match ray_hits_solid(body, solid, point, dir)? {
            RayHit::OnBoundary => boundary_hits += 1,
            RayHit::Inside => parity_votes += 1,
            RayHit::Outside => {}
        }
    }
    if boundary_hits > 0 {
        return Ok(PointSolidClassification::OnBoundary);
    }
    if parity_votes % 2 == 1 {
        Ok(PointSolidClassification::Inside)
    } else {
        Ok(PointSolidClassification::Outside)
    }
}

enum RayHit {
    Inside,
    Outside,
    OnBoundary,
}

fn ray_hits_solid(body: &Body, solid: SolidId, origin: Pnt3, dir: Vec3) -> Result<RayHit, KernelError> {
    let d = dir.normalized().unwrap_or(Vec3::X);
    let mut hits = 0usize;
    for face in body.solid_faces(solid) {
        if let Some(t) = ray_face_intersection(body, face, origin, d)? {
            if t < 1e-9 {
                return Ok(RayHit::OnBoundary);
            }
            hits += 1;
        }
    }
    Ok(if hits % 2 == 1 { RayHit::Inside } else { RayHit::Outside })
}

// #endregion 🔖️Classify

// #region 🔖️Quadrature

const GL5_NODES: [f64; 5] = [-0.906_179_845_938_664, -0.538_469_310_105_683_1, 0.0, 0.538_469_310_105_683_1, 0.906_179_845_938_664];
const GL5_WEIGHTS: [f64; 5] = [0.2369268850561891, 0.4786286704993665, 0.5688888888888889, 0.4786286704993665, 0.2369268850561891];

fn gauss_samples(chord_tol: f64) -> usize {
    ((1.0 / chord_tol.max(1e-6)).sqrt().ceil() as usize).clamp(4, 32)
}

fn parametric_volume_moments(surface: &Surface, flipped: bool, u0: f64, u1: f64, v0: f64, v1: f64, boundary: &[Pnt2], _samples: usize) -> (f64, f64, f64, f64) {
    let mut sv = 0.0;
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut mz = 0.0;
    integrate_parametric_face(u0, u1, v0, v1, |u, v| {
        if !point_in_uv_polygon(u, v, boundary) {
            return 0.0;
        }
        let d = surface.derivatives(u, v);
        let p = d.point;
        let cross = d.du.cross(d.dv);
        let mut weight = p.x * cross.x + p.y * cross.y + p.z * cross.z;
        if flipped {
            weight = -weight;
        }
        sv += weight;
        mx += weight * p.x;
        my += weight * p.y;
        mz += weight * p.z;
        weight
    });
    (sv, mx, my, mz)
}

fn integrate_parametric_face<F>(u0: f64, u1: f64, v0: f64, v1: f64, mut f: F) -> f64
where
    F: FnMut(f64, f64) -> f64,
{
    let hu = 0.5 * (u1 - u0);
    let hv = 0.5 * (v1 - v0);
    let mu = 0.5 * (u0 + u1);
    let mv = 0.5 * (v0 + v1);
    let mut sum = 0.0;
    for (&xu, &wu) in GL5_NODES.iter().zip(GL5_WEIGHTS.iter()) {
        for (&xv, &wv) in GL5_NODES.iter().zip(GL5_WEIGHTS.iter()) {
            sum += wu * wv * f(mu + hu * xu, mv + hv * xv);
        }
    }
    sum * hu * hv
}

// #endregion 🔖️Quadrature

// #region 🔖️Loops

fn axis_aligned_box_distance(a: &AxisAlignedBox, b: &AxisAlignedBox) -> f64 {
    let dx = gap_1d(a.min.x, a.max.x, b.min.x, b.max.x);
    let dy = gap_1d(a.min.y, a.max.y, b.min.y, b.max.y);
    let dz = gap_1d(a.min.z, a.max.z, b.min.z, b.max.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn gap_1d(a0: f64, a1: f64, b0: f64, b1: f64) -> f64 {
    if a1 < b0 {
        b0 - a1
    } else if b1 < a0 {
        a0 - b1
    } else {
        0.0
    }
}

fn loop_area(body: &Body, face: FaceId, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, _chord_tol: f64) -> Result<f64, KernelError> {
    let surface = face_surface(body, face)?;
    let flipped = body.faces.get(face).map(|f| f.flipped).unwrap_or(false);
    match surface {
        Surface::Plane { frame } => {
            let pts = loop_positions(body, loop_id)?;
            Ok(newell_area(&pts, outward_plane_normal(&frame, flipped)))
        }
        _ => {
            let (u0, u1, v0, v1) = loop_uv_bounds(body, face, loop_id, surface)?;
            let boundary = loop_uv_polygon(body, loop_id, surface)?;
            let area = integrate_parametric_face(u0, u1, v0, v1, |u, v| {
                if !point_in_uv_polygon(u, v, &boundary) {
                    return 0.0;
                }
                let d = surface.derivatives(u, v);
                let cross = d.du.cross(d.dv);
                match outward_normal(surface, u, v, flipped) {
                    Some(nn) => cross.dot(nn).abs(),
                    None => 0.0,
                }
            });
            Ok(area)
        }
    }
}

fn face_volume_contribution(body: &Body, face: FaceId, chord_tol: f64) -> Result<f64, KernelError> {
    let Some(face_ent) = body.faces.get(face) else {
        return Err(KernelError::MissingEntity("face".into()));
    };
    let mut vol = 0.0;
    if let Some(outer) = face_ent.outer {
        vol += loop_volume_contribution(body, face, outer, chord_tol)?;
    }
    for &inner in &face_ent.inners {
        vol -= loop_volume_contribution(body, face, inner, chord_tol)?;
    }
    Ok(vol)
}

fn face_volume_moments(body: &Body, face: FaceId, chord_tol: f64) -> Result<(f64, f64, f64, f64), KernelError> {
    let Some(face_ent) = body.faces.get(face) else {
        return Err(KernelError::MissingEntity("face".into()));
    };
    let mut sv = 0.0;
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut mz = 0.0;
    if let Some(outer) = face_ent.outer {
        let (a, b, c, d) = loop_volume_moments(body, face, outer, chord_tol)?;
        sv += a;
        mx += b;
        my += c;
        mz += d;
    }
    for &inner in &face_ent.inners {
        let (a, b, c, d) = loop_volume_moments(body, face, inner, chord_tol)?;
        sv -= a;
        mx -= b;
        my -= c;
        mz -= d;
    }
    Ok((sv, mx, my, mz))
}

fn loop_volume_contribution(body: &Body, face: FaceId, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, chord_tol: f64) -> Result<f64, KernelError> {
    let (sv, _, _, _) = loop_volume_moments(body, face, loop_id, chord_tol)?;
    Ok(sv / 6.0)
}

fn loop_volume_moments(body: &Body, face: FaceId, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, chord_tol: f64) -> Result<(f64, f64, f64, f64), KernelError> {
    let surface = face_surface(body, face)?;
    let flipped = body.faces.get(face).map(|f| f.flipped).unwrap_or(false);
    match surface {
        Surface::Plane { .. } => {
            let pts = loop_positions(body, loop_id)?;
            Ok(signed_tetra_sum(&pts))
        }
        _ => {
            let (u0, u1, v0, v1) = loop_uv_bounds(body, face, loop_id, surface)?;
            let boundary = loop_uv_polygon(body, loop_id, surface)?;
            let samples = gauss_samples(chord_tol);
            Ok(parametric_volume_moments(surface, flipped, u0, u1, v0, v1, &boundary, samples))
        }
    }
}

fn signed_tetra_sum(pts: &[Pnt3]) -> (f64, f64, f64, f64) {
    if pts.len() < 3 {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let p0 = pts[0];
    let mut sv = 0.0;
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut mz = 0.0;
    for i in 1..pts.len() - 1 {
        let a = p0.to_vec();
        let b = pts[i] - p0;
        let c = pts[i + 1] - p0;
        let tet = a.dot(b.cross(c));
        sv += tet;
        mx += tet * (p0.x + pts[i].x + pts[i + 1].x);
        my += tet * (p0.y + pts[i].y + pts[i + 1].y);
        mz += tet * (p0.z + pts[i].z + pts[i + 1].z);
    }
    (sv, mx, my, mz)
}

fn loop_positions(body: &Body, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId) -> Result<Vec<Pnt3>, KernelError> {
    let mut pts = Vec::new();
    for coedge in body.loop_coedges(loop_id) {
        let (v0, _) = body.coedge_endpoints(coedge).ok_or_else(|| KernelError::InvalidInput("open coedge".into()))?;
        let v = body.vertices.get(v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?;
        pts.push(v.position);
    }
    Ok(pts)
}

fn face_surface<'a>(body: &'a Body, face: FaceId) -> Result<&'a Surface, KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    body.surfaces.get(face_ent.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))
}

fn outward_plane_normal(frame: &semio_framework_3d::brep::mat::Frame3, flipped: bool) -> Vec3 {
    let mut n = frame.z;
    if flipped {
        n = -n;
    }
    n
}

fn outward_normal(surface: &Surface, u: f64, v: f64, flipped: bool) -> Option<Vec3> {
    let mut n = surface.normal(u, v)?;
    if flipped {
        n = -n;
    }
    Some(n)
}

fn newell_area(pts: &[Pnt3], normal: Vec3) -> f64 {
    if pts.len() < 3 {
        return 0.0;
    }
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        cx += pts[i].y * pts[j].z - pts[i].z * pts[j].y;
        cy += pts[i].z * pts[j].x - pts[i].x * pts[j].z;
        cz += pts[i].x * pts[j].y - pts[i].y * pts[j].x;
    }
    let area_vec = Vec3::new(cx, cy, cz);
    0.5 * area_vec.dot(normal).abs()
}

fn loop_uv_bounds(_body: &Body, _face: FaceId, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, surface: &Surface) -> Result<(f64, f64, f64, f64), KernelError> {
    let poly = loop_uv_polygon(_body, loop_id, surface)?;
    if poly.is_empty() {
        return Err(KernelError::InvalidInput("empty loop".into()));
    }
    let mut u0 = f64::INFINITY;
    let mut u1 = f64::NEG_INFINITY;
    let mut v0 = f64::INFINITY;
    let mut v1 = f64::NEG_INFINITY;
    for p in poly {
        u0 = u0.min(p.x);
        u1 = u1.max(p.x);
        v0 = v0.min(p.y);
        v1 = v1.max(p.y);
    }
    let pad_u = (u1 - u0).max(1e-6) * 0.02;
    let pad_v = (v1 - v0).max(1e-6) * 0.02;
    Ok((u0 - pad_u, u1 + pad_u, v0 - pad_v, v1 + pad_v))
}

fn loop_uv_polygon(body: &Body, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, surface: &Surface) -> Result<Vec<Pnt2>, KernelError> {
    let mut poly = Vec::new();
    let mut prev_u: Option<f64> = None;
    for coedge in body.loop_coedges(loop_id) {
        let (v0, _) = body.coedge_endpoints(coedge).ok_or_else(|| KernelError::InvalidInput("open coedge".into()))?;
        let v = body.vertices.get(v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?;
        let mut uv = surface_uv(surface, v.position);
        if surface.is_u_periodic() {
            if let Some(pu) = prev_u {
                uv.x = unwrap_u(pu, uv.x);
            }
            prev_u = Some(uv.x);
        }
        poly.push(uv);
    }
    Ok(poly)
}

fn unwrap_u(prev: f64, u: f64) -> f64 {
    let mut w = u;
    let pi = std::f64::consts::PI;
    while w - prev > pi {
        w -= std::f64::consts::TAU;
    }
    while w - prev < -pi {
        w += std::f64::consts::TAU;
    }
    w
}

fn surface_uv(surface: &Surface, p: Pnt3) -> Pnt2 {
    match surface {
        Surface::Plane { frame } => {
            let l = frame.to_local(p);
            Pnt2::new(l.x, l.y)
        }
        Surface::Cylinder { frame, radius: _ } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            Pnt2::new(u, l.z)
        }
        Surface::Cone { frame, half_angle } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let v = l.z / half_angle.tan().max(1e-15);
            Pnt2::new(u, v)
        }
        Surface::Sphere { frame, radius } => {
            let l = (p - frame.origin).normalized().unwrap_or(Vec3::Z);
            let v = l.z.clamp(-1.0, 1.0).asin();
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let _ = radius;
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
            Pnt2::new(dom.0.0, dom.1.0)
        }
    }
}

fn point_in_uv_polygon(u: f64, v: f64, poly: &[Pnt2]) -> bool {
    if poly.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let yi = poly[i].y;
        let yj = poly[j].y;
        if (yi > v) != (yj > v) {
            let xi = poly[i].x;
            let xj = poly[j].x;
            let x_int = (xj - xi) * (v - yi) / (yj - yi) + xi;
            if u < x_int {
                inside = !inside;
            }
        }
        j = i;
    }
    inside
}

// #endregion 🔖️Loops

// #region 🔖️Samples

fn face_sample_points(body: &Body, face: FaceId) -> Result<Vec<Pnt3>, KernelError> {
    let mut pts = Vec::new();
    for loop_id in body.face_loops(face) {
        for coedge in body.loop_coedges(loop_id) {
            let edge_ent = body.coedges.get(coedge).and_then(|c| body.edges.get(c.edge));
            if let Some(edge) = edge_ent {
                if let Some(curve) = body.curves3.get(edge.curve) {
                    let mid = 0.5 * (edge.range.0 + edge.range.1);
                    pts.push(curve.eval(mid));
                }
                for vid in [edge.v0, edge.v1] {
                    if let Some(v) = body.vertices.get(vid) {
                        pts.push(v.position);
                    }
                }
            }
        }
    }
    if pts.is_empty() {
        return Err(KernelError::InvalidInput("face has no samples".into()));
    }
    Ok(pts)
}

fn expand_bbox_for_surface(min: &mut Pnt3, max: &mut Pnt3, surface: &Surface) {
    match surface {
        Surface::Sphere { frame, radius } => {
            let c = frame.origin;
            let r = *radius;
            min.x = min.x.min(c.x - r);
            min.y = min.y.min(c.y - r);
            min.z = min.z.min(c.z - r);
            max.x = max.x.max(c.x + r);
            max.y = max.y.max(c.y + r);
            max.z = max.z.max(c.z + r);
        }
        Surface::Cylinder { frame, radius } => {
            let c = frame.origin;
            let r = *radius;
            min.x = min.x.min(c.x - r);
            min.y = min.y.min(c.y - r);
            max.x = max.x.max(c.x + r);
            max.y = max.y.max(c.y + r);
        }
        Surface::Torus { frame, major_radius, minor_radius } => {
            let c = frame.origin;
            let ext = major_radius + minor_radius;
            min.x = min.x.min(c.x - ext);
            min.y = min.y.min(c.y - ext);
            min.z = min.z.min(c.z - minor_radius);
            max.x = max.x.max(c.x + ext);
            max.y = max.y.max(c.y + ext);
            max.z = max.z.max(c.z + minor_radius);
        }
        _ => {}
    }
}

fn try_analytic_sphere_volume(body: &Body, solid: SolidId) -> Option<f64> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return None;
    }
    let mut origin: Option<Pnt3> = None;
    let mut radius: Option<f64> = None;
    for fid in faces {
        let face = body.faces.get(fid)?;
        let surf = body.surfaces.get(face.surface)?;
        let Surface::Sphere { frame, radius: r } = surf else {
            return None;
        };
        match (origin, radius) {
            (None, None) => {
                origin = Some(frame.origin);
                radius = Some(*r);
            }
            (Some(o), Some(r0)) if o.distance(frame.origin) < 1e-9 * r0.max(1.0) && (r0 - r).abs() < 1e-9 * r0.max(1.0) => {}
            _ => return None,
        }
    }
    let r = radius?;
    Some(4.0 / 3.0 * std::f64::consts::PI * r * r * r)
}

fn closest_point_on_face(body: &Body, face: FaceId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let surface = face_surface(body, face)?;
    match surface {
        Surface::Plane { .. } => closest_point_on_planar_face(body, face, point),
        _ => {
            let domain = surface.domain();
            let (u, v, d) = surface_ops::closest_point(surface, domain, point, 24);
            Ok((surface.eval(u, v), d))
        }
    }
}

fn closest_point_on_planar_face(body: &Body, face: FaceId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let surface = face_surface(body, face)?;
    let domain = surface.domain();
    let (u, v, d) = surface_ops::closest_point(surface, domain, point, 8);
    let p = surface.eval(u, v);
    if point_in_face_plane(body, face, p)? {
        return Ok((p, d));
    }
    let mut best_p = p;
    let mut best_d = f64::INFINITY;
    for loop_id in body.face_loops(face) {
        for coedge in body.loop_coedges(loop_id) {
            let co = body.coedges.get(coedge).ok_or_else(|| KernelError::MissingEntity("coedge".into()))?;
            let edge = body.edges.get(co.edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
            let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
            let (t, dist) = curve_ops::closest_point(curve, edge.range, point, 16);
            if dist < best_d {
                best_d = dist;
                best_p = curve.eval(t);
            }
        }
    }
    Ok((best_p, best_d))
}

fn ray_face_intersection(body: &Body, face: FaceId, origin: Pnt3, dir: Vec3) -> Result<Option<f64>, KernelError> {
    let surface = face_surface(body, face)?;
    let flipped = body.faces.get(face).map(|f| f.flipped).unwrap_or(false);
    match surface {
        Surface::Plane { frame } => {
            let n = outward_plane_normal(frame, flipped);
            let denom = n.dot(dir);
            if denom.abs() < 1e-12 {
                return Ok(None);
            }
            let t = n.dot(frame.origin - origin) / denom;
            if t < 0.0 {
                return Ok(None);
            }
            let hit = origin + dir * t;
            if point_in_face_plane(body, face, hit)? {
                Ok(Some(t))
            } else {
                Ok(None)
            }
        }
        _ => {
            let samples = 16;
            let domain = surface.domain();
            let (u_dom, v_dom) = domain;
            let mut best: Option<f64> = None;
            for i in 0..=samples {
                for j in 0..=samples {
                    let u = u_dom.0 + (u_dom.1 - u_dom.0) * (i as f64 / samples as f64);
                    let v = v_dom.0 + (v_dom.1 - v_dom.0) * (j as f64 / samples as f64);
                    let p = surface.eval(u, v);
                    let w = p - origin;
                    let cross = dir.cross(w);
                    let cross_norm = cross.norm();
                    if cross_norm < 1e-9 && w.dot(dir) > 0.0 {
                        let t = w.dot(dir);
                        best = Some(best.map_or(t, |b: f64| b.min(t)));
                    }
                }
            }
            Ok(best)
        }
    }
}

fn point_in_face_plane(body: &Body, face: FaceId, point: Pnt3) -> Result<bool, KernelError> {
    let surface = face_surface(body, face)?;
    let Surface::Plane { frame } = surface else {
        return Ok(true);
    };
    let uv = surface_uv(surface, point);
    let Some(outer) = body.faces.get(face).and_then(|f| f.outer) else {
        return Ok(false);
    };
    let boundary = loop_uv_polygon(body, outer, surface)?;
    if !point_in_uv_polygon(uv.x, uv.y, &boundary) {
        return Ok(false);
    }
    for inner in &body.faces.get(face).unwrap().inners {
        let hole = loop_uv_polygon(body, *inner, surface)?;
        if point_in_uv_polygon(uv.x, uv.y, &hole) {
            return Ok(false);
        }
    }
    let _ = frame;
    Ok(true)
}

// #endregion 🔖️Samples

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::ArenaId;
    use semio_framework_3d::brep::mat::Frame3;
    use semio_framework_3d::brep::tolerance::Tol;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::{Body, Coedge, Edge, Face, Loop, Shell, Solid, Vertex};
    use std::f64::consts::PI;

    fn null_coedge() -> CoedgeId {
        ArenaId::from_raw(0, 0)
    }

    fn insert_vertex(body: &mut Body, position: Pnt3) -> VertexId {
        let label = body.new_label();
        body.vertices.insert(Vertex { position, tol: Tol::DEFAULT, label })
    }

    fn insert_edge(body: &mut Body, curve: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId) -> EdgeId {
        let label = body.new_label();
        body.edges.insert(Edge { curve, range, v0, v1, tol: Tol::DEFAULT, label })
    }

    fn make_quad_loop(body: &mut Body, face: FaceId, corners: [Pnt3; 4]) -> crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId {
        let verts: Vec<_> = corners.iter().map(|&p| insert_vertex(body, p)).collect();
        let curves: Vec<_> = (0..4)
            .map(|i| {
                let a = corners[i];
                let b = corners[(i + 1) % 4];
                body.curves3.insert(Curve3::Line { origin: a, dir: b - a })
            })
            .collect();
        let edges: Vec<_> = (0..4).map(|i| insert_edge(body, curves[i], (0.0, 1.0), verts[i], verts[(i + 1) % 4])).collect();
        let loop_id = body.loops.insert(Loop { first: null_coedge(), face });
        let coedges: Vec<_> = edges
            .iter()
            .map(|&e| {
                body.coedges.insert(Coedge {
                    edge: e,
                    forward: true,
                    pcurve: None,
                    prange: (0.0, 1.0),
                    loop_id,
                    next: null_coedge(),
                    prev: null_coedge(),
                })
            })
            .collect();
        for i in 0..4 {
            let c = body.coedges.get_mut(coedges[i]).unwrap();
            c.next = coedges[(i + 1) % 4];
            c.prev = coedges[(i + 3) % 4];
        }
        body.loops.get_mut(loop_id).unwrap().first = coedges[0];
        loop_id
    }

    fn add_planar_face(body: &mut Body, frame: Frame3, corners: [Pnt3; 4], flipped: bool) -> FaceId {
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let label = body.new_label();
        let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped, tol: Tol::DEFAULT, label });
        let loop_id = make_quad_loop(body, face, corners);
        body.faces.get_mut(face).unwrap().outer = Some(loop_id);
        face
    }

    fn make_box_solid(body: &mut Body, origin: Pnt3, w: f64, d: f64, h: f64) -> SolidId {
        let o = origin;
        let z0 = Frame3::from_normal(o, -Vec3::Z).unwrap();
        let z1 = Frame3::from_normal(o + Vec3::new(0.0, 0.0, h), Vec3::Z).unwrap();
        let y0 = Frame3::from_normal(o, -Vec3::Y).unwrap();
        let y1 = Frame3::from_normal(o + Vec3::new(0.0, d, 0.0), Vec3::Y).unwrap();
        let x0 = Frame3::from_normal(o, -Vec3::X).unwrap();
        let x1 = Frame3::from_normal(o + Vec3::new(w, 0.0, 0.0), Vec3::X).unwrap();
        let f_bottom = add_planar_face(body, z0, [o, o + Vec3::new(w, 0.0, 0.0), o + Vec3::new(w, d, 0.0), o + Vec3::new(0.0, d, 0.0)], false);
        let f_top = add_planar_face(body, z1, [o + Vec3::new(0.0, 0.0, h), o + Vec3::new(w, 0.0, h), o + Vec3::new(w, d, h), o + Vec3::new(0.0, d, h)], false);
        let f_front = add_planar_face(body, y0, [o, o + Vec3::new(w, 0.0, 0.0), o + Vec3::new(w, 0.0, h), o + Vec3::new(0.0, 0.0, h)], false);
        let f_back = add_planar_face(body, y1, [o + Vec3::new(0.0, d, 0.0), o + Vec3::new(0.0, d, h), o + Vec3::new(w, d, h), o + Vec3::new(w, d, 0.0)], false);
        let f_left = add_planar_face(body, x0, [o, o + Vec3::new(0.0, 0.0, h), o + Vec3::new(0.0, d, h), o + Vec3::new(0.0, d, 0.0)], false);
        let f_right = add_planar_face(body, x1, [o + Vec3::new(w, 0.0, 0.0), o + Vec3::new(w, d, 0.0), o + Vec3::new(w, d, h), o + Vec3::new(w, 0.0, h)], false);
        let label = body.new_label();
        let shell = body.shells.insert(Shell { faces: vec![f_bottom, f_top, f_front, f_back, f_left, f_right], label });
        let solid_label = body.new_label();
        body.solids.insert(Solid { outer: shell, inners: vec![], label: solid_label })
    }

    fn make_uv_sphere(body: &mut Body, radius: f64, n_long: usize, n_lat: usize) -> SolidId {
        let frame = Frame3::WORLD;
        let surface = body.surfaces.insert(Surface::Sphere { frame, radius });
        let mut faces = Vec::new();
        for i in 0..n_lat {
            let v0 = -PI / 2.0 + PI * (i as f64) / n_lat as f64;
            let v1 = -PI / 2.0 + PI * ((i + 1) as f64) / n_lat as f64;
            for j in 0..n_long {
                let u0 = TAU * (j as f64) / n_long as f64;
                let u1 = TAU * ((j + 1) as f64) / n_long as f64;
                let corners = [sphere_corner(&frame, radius, u0, v0), sphere_corner(&frame, radius, u1, v0), sphere_corner(&frame, radius, u1, v1), sphere_corner(&frame, radius, u0, v1)];
                let label = body.new_label();
                let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: false, tol: Tol::DEFAULT, label });
                let loop_id = make_quad_loop(body, face, corners);
                body.faces.get_mut(face).unwrap().outer = Some(loop_id);
                faces.push(face);
            }
        }
        let label = body.new_label();
        let shell = body.shells.insert(Shell { faces, label });
        let solid_label = body.new_label();
        body.solids.insert(Solid { outer: shell, inners: vec![], label: solid_label })
    }

    fn sphere_corner(frame: &Frame3, radius: f64, u: f64, v: f64) -> Pnt3 {
        Surface::Sphere { frame: *frame, radius }.eval(u, v)
    }

    const TAU: f64 = 2.0 * PI;

    #[test]
    fn unit_box_volume_and_area() {
        let mut body = Body::new();
        let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        let vol = solid_volume(&body, solid, 0.1).unwrap();
        let area = solid_surface_area(&body, solid, 0.1).unwrap();
        assert!((vol - 1.0).abs() < 1e-9, "volume {vol}");
        assert!((area - 6.0).abs() < 1e-9, "area {area}");
    }

    #[test]
    fn box_mass_properties_and_bbox() {
        let mut body = Body::new();
        let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 2.0, 3.0, 4.0);
        let vol = solid_volume(&body, solid, 0.1).unwrap();
        assert!((vol - 24.0).abs() < 1e-8);
        let com = solid_center_of_mass(&body, solid, 0.1).unwrap();
        assert!((com.x - 1.0).abs() < 1e-8);
        assert!((com.y - 1.5).abs() < 1e-8);
        assert!((com.z - 2.0).abs() < 1e-8);
        let bb = solid_bounding_box(&body, solid).unwrap();
        assert!((bb.min.x - 0.0).abs() < 1e-9);
        assert!((bb.max.x - 2.0).abs() < 1e-9);
        assert!((bb.max.z - 4.0).abs() < 1e-9);
    }

    #[test]
    fn edge_length_on_unit_box() {
        let mut body = Body::new();
        let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        let face = body.solid_faces(solid)[0];
        let coedge = body.loop_coedges(body.faces.get(face).unwrap().outer.unwrap())[0];
        let edge = body.coedges.get(coedge).unwrap().edge;
        let len = edge_length(&body, edge).unwrap();
        assert!((len - 1.0).abs() < 1e-9);
    }

    #[test]
    fn sphere_volume_coarse_tessellation() {
        let mut body = Body::new();
        let r = 2.0;
        let solid = make_uv_sphere(&mut body, r, 12, 8);
        let vol = solid_volume(&body, solid, 0.15).unwrap();
        let expected = 4.0 / 3.0 * PI * r * r * r;
        assert!((vol - expected).abs() < 0.02 * expected, "vol {vol} expected {expected}");
    }

    #[test]
    fn distance_and_closest_point_between_boxes() {
        let mut body = Body::new();
        let a = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        let b = make_box_solid(&mut body, Pnt3::new(3.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        let d = distance_solid_solid(&body, a, b).unwrap();
        assert!((d - 2.0).abs() < 0.25, "distance {d}");
        let (cp, dist) = closest_point_on_solid(&body, b, Pnt3::new(0.5, 0.5, 0.5)).unwrap();
        assert!(dist > 1.5 && dist < 3.5, "dist {dist}");
        assert!(cp.x > 2.5);
    }

    #[test]
    fn classify_point_ray_parity_on_unit_box() {
        let mut body = Body::new();
        let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        assert_eq!(classify_point_on_solid(&body, solid, Pnt3::new(0.5, 0.5, 0.5)).unwrap(), PointSolidClassification::Inside);
        assert_eq!(classify_point_on_solid(&body, solid, Pnt3::new(2.0, 2.0, 2.0)).unwrap(), PointSolidClassification::Outside);
    }

    #[test]
    fn face_area_unit_square() {
        let mut body = Body::new();
        let frame = Frame3::WORLD;
        let face = add_planar_face(&mut body, frame, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)], false);
        let area = face_area(&body, face, 0.1).unwrap();
        assert!((area - 1.0).abs() < 1e-9);
    }

}

// #endregion 🔖️Tests

// #region 🔖️Oracle
pub mod oracle {
//! 🔮️ Ground truth used only by tests, kept deliberately independent from the kernel's own
//! algorithms (WFC-crate convention: a brute-force oracle catches bugs a self-referential test
//! never could). This module grows alongside the kernel — [`Sdf`] lands in Phase 0 with the
//! primitives it can already describe; mass-property, watertightness and shape-generator oracles
//! land in the phases that need them.

use semio_framework_3d::brep::mat::Trsf;
use semio_framework_3d::brep::vec::Pnt3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;

// #region 🔖️Sdf

/// 🔮️ A closed-form signed distance field: negative inside, zero on the boundary, positive
/// outside. Used to probe classification and Boolean results independently of the kernel's own
/// ray-casting/arrangement code.
#[derive(Clone, Debug, PartialEq)]
pub enum Sdf {
    /// 🔮️ Axis-aligned box of the given half-extents, centered at the origin before `placement`.
    Box {
        half_extents: Pnt3,
        placement: Trsf,
    },
    /// 🔮️ Sphere of the given radius, centered at the origin before `placement`.
    Sphere {
        radius: f64,
        placement: Trsf,
    },
    /// 🔮️ Cylinder of the given radius and half-height, axis along local `z`, centered at the
    /// origin before `placement`.
    Cylinder {
        radius: f64,
        half_height: f64,
        placement: Trsf,
    },
    /// 🔮️ Capped cone along local `z`, radius `radius` at `z = -half_height` tapering to apex at
    /// `z = +half_height`, centered at the origin before `placement`.
    Cone {
        radius: f64,
        half_height: f64,
        placement: Trsf,
    },
    /// 🔮️ Torus in the local `xy` plane, major circle radius `major_radius`, tube radius
    /// `minor_radius`, axis along local `z`, centered at the origin before `placement`.
    Torus {
        major_radius: f64,
        minor_radius: f64,
        placement: Trsf,
    },
    /// 🔮️ Boolean combination of two fields.
    Union(Box<Sdf>, Box<Sdf>),
    Intersect(Box<Sdf>, Box<Sdf>),
    Difference(Box<Sdf>, Box<Sdf>),
}

impl Sdf {
    /// 🔮️ Evaluates the field at a world-space point.
    pub fn eval(&self, p: Pnt3) -> f64 {
        match self {
            Sdf::Box { half_extents, placement } => {
                let local = placement.inverse().apply_point(p);
                let dx = local.x.abs() - half_extents.x;
                let dy = local.y.abs() - half_extents.y;
                let dz = local.z.abs() - half_extents.z;
                let outside = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2) + dz.max(0.0).powi(2)).sqrt();
                let inside = dx.max(dy).max(dz).min(0.0);
                outside + inside
            }
            Sdf::Sphere { radius, placement } => {
                let local = placement.inverse().apply_point(p);
                local.to_vec().norm() - radius
            }
            Sdf::Cylinder { radius, half_height, placement } => {
                let local = placement.inverse().apply_point(p);
                let radial = (local.x * local.x + local.y * local.y).sqrt() - radius;
                let axial = local.z.abs() - half_height;
                let outside = (radial.max(0.0).powi(2) + axial.max(0.0).powi(2)).sqrt();
                let inside = radial.max(axial).min(0.0);
                outside + inside
            }
            Sdf::Cone { radius, half_height, placement } => {
                let local = placement.inverse().apply_point(p);
                capped_cone_z(&local, *half_height, *radius, 0.0)
            }
            Sdf::Torus { major_radius, minor_radius, placement } => {
                let local = placement.inverse().apply_point(p);
                let qx = (local.x * local.x + local.y * local.y).sqrt() - major_radius;
                let qz = local.z;
                (qx * qx + qz * qz).sqrt() - minor_radius
            }
            Sdf::Union(a, b) => a.eval(p).min(b.eval(p)),
            Sdf::Intersect(a, b) => a.eval(p).max(b.eval(p)),
            Sdf::Difference(a, b) => a.eval(p).max(-b.eval(p)),
        }
    }
    /// 🔮️ `true` when `p` is inside (or on, within `tol`) the field's boundary.
    pub fn contains(&self, p: Pnt3, tol: f64) -> bool {
        self.eval(p) <= tol
    }
    pub fn union(self, other: Sdf) -> Sdf {
        Sdf::Union(Box::new(self), Box::new(other))
    }
    pub fn intersect(self, other: Sdf) -> Sdf {
        Sdf::Intersect(Box::new(self), Box::new(other))
    }
    pub fn difference(self, other: Sdf) -> Sdf {
        Sdf::Difference(Box::new(self), Box::new(other))
    }
}

/// 🔮️ Capped cone SDF along `z` with base radius `r1` at `z = -h` and `r2` at `z = +h`.
fn capped_cone_z(p: &Pnt3, h: f64, r1: f64, r2: f64) -> f64 {
    let qx = (p.x * p.x + p.y * p.y).sqrt();
    let k1_x = r2;
    let k1_z = h;
    let k2_x = r2 - r1;
    let k2_z = 2.0 * h;
    let cap_r = if p.z < 0.0 { r1 } else { r2 };
    let ca_x = qx - qx.min(cap_r);
    let ca_z = p.z.abs() - h;
    let dot_k1_q = k1_x * (k1_x - qx) + k1_z * (k1_z - p.z);
    let dot_k2_k2 = k2_x * k2_x + k2_z * k2_z;
    let t = (dot_k1_q / dot_k2_k2).clamp(0.0, 1.0);
    let cb_x = qx - k1_x + k2_x * t;
    let cb_z = p.z - k1_z + k2_z * t;
    let sign = if cb_x < 0.0 && ca_z < 0.0 { -1.0 } else { 1.0 };
    let ca_len = ca_x * ca_x + ca_z * ca_z;
    let cb_len = cb_x * cb_x + cb_z * cb_z;
    sign * ca_len.min(cb_len).sqrt()
}

// #endregion 🔖️Sdf

// #region 🔖️ClosedFormMass

/// 🔮️ Closed-form volume and surface area for analytic primitives (test oracle vs the sibling `super` mass-properties module).
pub struct ClosedFormMass;

impl ClosedFormMass {
    /// 🔮️ Volume of an axis-aligned box with the given half-extents.
    pub fn box_volume(half_extents: Pnt3) -> f64 {
        8.0 * half_extents.x * half_extents.y * half_extents.z
    }
    /// 🔮️ Total surface area of an axis-aligned box with the given half-extents.
    pub fn box_surface_area(half_extents: Pnt3) -> f64 {
        8.0 * (half_extents.x * half_extents.y + half_extents.y * half_extents.z + half_extents.x * half_extents.z)
    }
    /// 🔮️ Volume of a sphere with the given radius.
    pub fn sphere_volume(radius: f64) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3)
    }
    /// 🔮️ Surface area of a sphere with the given radius.
    pub fn sphere_surface_area(radius: f64) -> f64 {
        4.0 * std::f64::consts::PI * radius.powi(2)
    }
    /// 🔮️ Volume of a right circular cylinder (including caps) with radius and full height `2 * half_height`.
    pub fn cylinder_volume(radius: f64, half_height: f64) -> f64 {
        std::f64::consts::PI * radius.powi(2) * (2.0 * half_height)
    }
    /// 🔮️ Total surface area of a capped right circular cylinder.
    pub fn cylinder_surface_area(radius: f64, half_height: f64) -> f64 {
        2.0 * std::f64::consts::PI * radius * (radius + 2.0 * half_height)
    }
}

// #endregion 🔖️ClosedFormMass

// #region 🔖️Watertightness

/// 🔮️ Watertightness classification returned by the oracle checker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatertightnessVerdict {
    /// 🔮️ Every edge is shared by exactly two faces with consistent orientation.
    Watertight,
    /// 🔮️ At least one boundary edge remains (open shell or non-manifold rim).
    HasBoundaryEdges { count: usize },
    /// 🔮️ Topology not inspected yet (stub until sew/heal lanes wire real counts).
    NotChecked,
}

/// 🔮️ Summary of a watertightness probe for differential tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WatertightnessReport {
    pub verdict: WatertightnessVerdict,
}

/// 🔮️ Stub API: derives a verdict from a pre-counted boundary-edge tally supplied by future topo tests.
pub fn watertightness_from_boundary_edge_count(boundary_edges: usize) -> WatertightnessReport {
    let verdict = if boundary_edges == 0 {
        WatertightnessVerdict::Watertight
    } else {
        WatertightnessVerdict::HasBoundaryEdges { count: boundary_edges }
    };
    WatertightnessReport { verdict }
}

/// 🔮️ Count edges whose coedge valence is not exactly two (boundary or non-manifold).
pub fn count_boundary_edges(body: &Body) -> usize {
    let mut count = 0usize;
    for (edge_id, _) in body.edges.iter() {
        let valence = body.edge_coedges(edge_id).len();
        if valence != 2 {
            count += 1;
        }
    }
    count
}

/// 🔮️ Real watertightness probe from body topology (boundary/non-manifold edge valence).
pub fn watertightness_of_body(body: &Body) -> WatertightnessReport {
    watertightness_from_boundary_edge_count(count_boundary_edges(body))
}

/// 🔮️ Compatibility alias retained for older call sites; prefer [`watertightness_of_body`].
pub fn watertightness_stub_unchecked() -> WatertightnessReport {
    WatertightnessReport {
        verdict: WatertightnessVerdict::NotChecked,
    }
}

// #endregion 🔖️Watertightness

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_sdf_is_negative_inside_and_positive_outside() {
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement: Trsf::IDENTITY };
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(5.0, 0.0, 0.0)) > 0.0);
        assert!((b.eval(Pnt3::new(1.0, 0.0, 0.0))).abs() < 1e-9);
    }

    #[test]
    fn sphere_sdf_matches_analytic_distance() {
        let s = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        assert!((s.eval(Pnt3::new(5.0, 0.0, 0.0)) - 3.0).abs() < 1e-9);
        assert!((s.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-2.0)).abs() < 1e-9);
    }

    #[test]
    fn cylinder_sdf_is_correct_on_axis_and_cap() {
        let c = Sdf::Cylinder { radius: 1.0, half_height: 2.0, placement: Trsf::IDENTITY };
        assert!((c.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-1.0)).abs() < 1e-9);
        assert!((c.eval(Pnt3::new(0.0, 0.0, 5.0)) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn torus_sdf_is_negative_on_major_circle_and_positive_outside_tube() {
        let t = Sdf::Torus {
            major_radius: 2.0,
            minor_radius: 0.5,
            placement: Trsf::IDENTITY,
        };
        assert!(t.eval(Pnt3::new(2.0, 0.0, 0.0)) < 0.0);
        assert!((t.eval(Pnt3::new(2.5, 0.0, 0.0))).abs() < 1e-8);
        assert!(t.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
    }

    #[test]
    fn cone_sdf_is_negative_inside_taper_and_positive_outside() {
        let c = Sdf::Cone { radius: 1.0, half_height: 1.0, placement: Trsf::IDENTITY };
        assert!(c.eval(Pnt3::new(0.0, 0.0, -0.5)) < 0.0);
        assert!((c.eval(Pnt3::new(1.0, 0.0, -1.0))).abs() < 1e-8);
        assert!(c.eval(Pnt3::new(2.0, 0.0, 0.0)) > 0.0);
    }

    #[test]
    fn union_is_the_min_and_matches_containment_of_either_operand() {
        let a = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(semio_framework_3d::brep::vec::Vec3::new(-1.0, 0.0, 0.0)) };
        let b = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(semio_framework_3d::brep::vec::Vec3::new(1.0, 0.0, 0.0)) };
        let u = a.union(b);
        assert!(u.contains(Pnt3::new(-1.0, 0.0, 0.0), 1e-9));
        assert!(u.contains(Pnt3::new(1.0, 0.0, 0.0), 1e-9));
        assert!(!u.contains(Pnt3::new(5.0, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn difference_removes_the_second_operand() {
        let big = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        let small = Sdf::Sphere { radius: 1.0, placement: Trsf::IDENTITY };
        let d = big.difference(small);
        assert!(!d.contains(Pnt3::new(0.0, 0.0, 0.0), 1e-9));
        assert!(d.contains(Pnt3::new(1.5, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn placed_box_sdf_respects_transform() {
        let placement = Trsf::translation(semio_framework_3d::brep::vec::Vec3::new(10.0, 0.0, 0.0));
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement };
        assert!(b.eval(Pnt3::new(10.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
    }

    #[test]
    fn closed_form_mass_matches_textbook_box_sphere_cylinder() {
        let half = Pnt3::new(1.0, 2.0, 3.0);
        assert!((ClosedFormMass::box_volume(half) - 48.0).abs() < 1e-12);
        assert!((ClosedFormMass::box_surface_area(half) - 88.0).abs() < 1e-12);
        assert!((ClosedFormMass::sphere_volume(3.0) - 36.0 * std::f64::consts::PI).abs() < 1e-9);
        assert!((ClosedFormMass::sphere_surface_area(3.0) - 36.0 * std::f64::consts::PI).abs() < 1e-9);
        assert!((ClosedFormMass::cylinder_volume(2.0, 3.0) - 24.0 * std::f64::consts::PI).abs() < 1e-9);
        assert!((ClosedFormMass::cylinder_surface_area(2.0, 3.0) - 32.0 * std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn watertightness_stub_classifies_boundary_edge_count() {
        let tight = watertightness_from_boundary_edge_count(0);
        assert_eq!(tight.verdict, WatertightnessVerdict::Watertight);
        let open = watertightness_from_boundary_edge_count(3);
        assert_eq!(open.verdict, WatertightnessVerdict::HasBoundaryEdges { count: 3 });
        assert_eq!(watertightness_stub_unchecked().verdict, WatertightnessVerdict::NotChecked);
    }
}

    #[test]
    fn watertightness_of_box_is_watertight() {
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box;
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let _ = solid;
        let report = watertightness_of_body(&body);
        assert_eq!(report.verdict, WatertightnessVerdict::Watertight);
    }

// #endregion 🔖️Tests
}
// #endregion 🔖️Oracle
