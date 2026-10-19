//! 🧭️ Face samples and closest point: the face sample points, the surface bounding box expansion, the analytic sphere volume and the closest point on a face, its plane and its boundary. See [`super`].

use super::*;

// #region 🔖️Samples
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn face_sample_points(body: &Body, face: FaceId) -> Result<Vec<Pnt3>, KernelError> {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn expand_bbox_for_surface(min: &mut Pnt3, max: &mut Pnt3, surface: &Surface) {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn try_analytic_sphere_volume(body: &Body, solid: SolidId) -> Option<f64> {
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

/// 📏 Closest point on one face's TRIMMED surface to `point`, with the trim test applied — public
/// so `bounding-volume`'s `FaceBvh::closest_face`/`SolidBvh::closest_face` can reuse the exact
/// per-face distance behind their BVH branch-and-bound (one implementation, not a second copy).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn closest_point_on_face(body: &Body, face: FaceId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let surface = face_surface(body, face)?;
    match surface {
        Surface::Plane { .. } => closest_point_on_planar_face(body, face, point),
        _ => {
            let domain = surface.domain();
            let closest = surface_ops::closest_uv(surface, domain, point, 1e-9);
            if crate::classification::point_in_face_uv_closure(body, face, Pnt2::new(closest.u, closest.v), 1e-9)? {
                Ok((closest.point, closest.distance))
            } else {
                closest_point_on_face_boundary(body, face, point)
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn closest_point_on_planar_face(body: &Body, face: FaceId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let surface = face_surface(body, face)?;
    let domain = surface.domain();
    let closest = surface_ops::closest_uv(surface, domain, point, 1e-9);
    let (d, p) = (closest.distance, closest.point);
    if point_in_face_plane(body, face, p)? {
        return Ok((p, d));
    }
    closest_point_on_face_boundary(body, face, point)
}

/// 🎯️ Projects onto the actual edge ranges of a face's outer and inner trim loops.
fn closest_point_on_face_boundary(body: &Body, face: FaceId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let mut best_p = point;
    let mut best_d = f64::INFINITY;
    for loop_id in body.face_loops(face) {
        for coedge in body.loop_coedges(loop_id) {
            let co = body.coedges.get(coedge).ok_or_else(|| KernelError::MissingEntity("coedge".into()))?;
            let edge = body.edges.get(co.edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
            let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
            let closest = curve_ops::closest_parameter(curve, edge.range, point, 1e-9);
            if closest.distance < best_d {
                best_d = closest.distance;
                best_p = closest.point;
            }
        }
    }
    if best_d.is_finite() { Ok((best_p, best_d)) } else { Err(KernelError::MissingEntity("face has no projectable trim boundary".into())) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_face_plane(body: &Body, face: FaceId, point: Pnt3) -> Result<bool, KernelError> {
    let surface = face_surface(body, face)?;
    let Surface::Plane { frame } = surface else {
        return Ok(true);
    };
    let uv = surface_uv(surface, point);
    let Some(outer) = body.faces.get(face).and_then(|f| f.outer) else {
        return Ok(false);
    };
    const CLASSIFY_CHORD_TOL: f64 = 1e-4;
    let boundary = loop_uv_polygon(body, outer, surface, CLASSIFY_CHORD_TOL)?;
    if !point_in_uv_polygon(uv.x, uv.y, &boundary) {
        return Ok(false);
    }
    for inner in &body.faces.get(face).unwrap().inners {
        let hole = loop_uv_polygon(body, *inner, surface, CLASSIFY_CHORD_TOL)?;
        if point_in_uv_polygon(uv.x, uv.y, &hole) {
            return Ok(false);
        }
    }
    let _ = frame;
    Ok(true)
}
// #endregion 🔖️Samples
