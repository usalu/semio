//! ↔️ Offset face/thicken/offset solid/shell/draft.
//!
//! Lane 5-offset of ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.

use std::collections::HashMap;

use crate::brep::arena::{FaceId, SolidId};
use crate::brep::boolean::{boolean_solid, BooleanOp};
use crate::brep::curve::Curve3;
use crate::brep::engine::MeshTransfer;
use crate::brep::error::KernelError;
use crate::brep::euler::{add_shell, add_solid};
use crate::brep::history::OpRecorder;
use crate::brep::measure::{solid_bounding_box, solid_volume, AxisAlignedBox};
use crate::brep::primitives::{make_convex_hull, make_planar_face_from_points};
use crate::brep::surface::Surface;
use crate::brep::sweep::extrude_face;
use crate::brep::tessellate::tessellate_solid;
use crate::brep::topo::Body;
use crate::brep::vec::{Pnt3, Vec3};

// #region 🔖️Api

/// ↔️ Offset a planar face along its outward normal by `distance`.
pub fn offset_face(body: &mut Body, face: FaceId, distance: f64) -> Result<FaceId, KernelError> {
    if !distance.is_finite() {
        return Err(KernelError::InvalidInput("offset distance must be finite".into()));
    }
    let face_data = body
        .faces
        .get(face)
        .ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?
        .clone();
    let surface = body
        .surfaces
        .get(face_data.surface)
        .ok_or_else(|| KernelError::MissingEntity(format!("surface {}", face_data.surface)))?
        .clone();
    let Surface::Plane { frame } = surface else {
        return Err(KernelError::InvalidInput("offset_face requires a planar face".into()));
    };
    let polygon = face_outer_polygon(body, face)?;
    let mut normal = frame.z;
    if face_data.flipped {
        normal = -normal;
    }
    let offset_pts: Vec<Pnt3> = polygon
        .iter()
        .map(|p| Pnt3::new(p.x + normal.x * distance, p.y + normal.y * distance, p.z + normal.z * distance))
        .collect();
    make_planar_face_from_points(body, &offset_pts)
}

/// ↔️ Thicken a planar face into a solid prism of thickness `distance`.
pub fn thicken_face(body: &mut Body, face: FaceId, distance: f64) -> Result<SolidId, KernelError> {
    if !distance.is_finite() || distance.abs() <= 1e-15 {
        return Err(KernelError::InvalidInput("thicken distance must be non-zero".into()));
    }
    let face_data = body
        .faces
        .get(face)
        .ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?
        .clone();
    let surface = body
        .surfaces
        .get(face_data.surface)
        .ok_or_else(|| KernelError::MissingEntity(format!("surface {}", face_data.surface)))?
        .clone();
    let Surface::Plane { frame } = surface else {
        return thicken_face_hull(body, face, distance);
    };
    let mut normal = frame.z;
    if face_data.flipped {
        normal = -normal;
    }
    extrude_face(body, face, normal, distance.abs())
        .or_else(|_| thicken_face_hull(body, face, distance))
}

/// ↔️ Uniform solid offset (positive expands, negative shrinks).
pub fn offset_solid(body: &mut Body, solid: SolidId, distance: f64) -> Result<SolidId, KernelError> {
    if !distance.is_finite() {
        return Err(KernelError::InvalidInput("offset distance must be finite".into()));
    }
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity(format!("solid {solid}")));
    }
    if distance.abs() <= 1e-15 {
        return shell_copy_solid(body, solid);
    }
    let tol = 1e-6;
    if looks_like_box(body, solid)? {
        let bb = solid_bounding_box(body, solid)?;
        let inflated = inflate_aabb(&bb, distance);
        if aabb_volume(&inflated) <= tol {
            return Err(KernelError::Operation("offset collapsed the solid".into()));
        }
        return make_box_from_aabb(body, &inflated);
    }
    let bb = solid_bounding_box(body, solid)?;
    let mut points = mesh_offset_points(body, solid, distance, tol)?;
    points.extend(aabb_corners(&inflate_aabb(&bb, distance)));
    if points.len() < 4 {
        return Err(KernelError::Operation("offset produced insufficient points".into()));
    }
    make_convex_hull(body, &points)
}

/// ↔️ Hollow shell of `solid` with wall thickness `thickness`.
pub fn shell_solid(body: &mut Body, solid: SolidId, thickness: f64) -> Result<SolidId, KernelError> {
    if !thickness.is_finite() || thickness <= 1e-15 {
        return Err(KernelError::InvalidInput("shell thickness must be positive".into()));
    }
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity(format!("solid {solid}")));
    }
    let tol = 1e-6;
    let inner = offset_solid(body, solid, -thickness)?;
    match boolean_solid(body, solid, inner, BooleanOp::Cut, tol) {
        Ok(id) => Ok(id),
        Err(_) => solid_with_void_shell(body, solid, inner),
    }
}


/// ↔️ Shell a solid and leave the listed faces open by cutting through-face openings.
pub fn shell_solid_with_open_faces(
    body: &mut Body,
    solid: SolidId,
    thickness: f64,
    open_faces: &[FaceId],
) -> Result<SolidId, KernelError> {
    let shelled = shell_solid(body, solid, thickness)?;
    if open_faces.is_empty() {
        return Ok(shelled);
    }
    let tol = 1e-6_f64.max(thickness * 1e-3);
    let mut result = shelled;
    for &face in open_faces {
        let corners = face_corners(body, face);
        if corners.len() < 3 {
            continue;
        }
        let normal = face_normal_hint(&corners).unwrap_or(Vec3::new(0.0, 0.0, 1.0));
        let n = normal.normalized().unwrap_or(Vec3::new(0.0, 0.0, 1.0));
        let half = thickness * 2.0 + tol * 10.0;
        let extruded: Vec<Pnt3> = corners
            .iter()
            .flat_map(|p| [*p + n * half, *p - n * half])
            .collect();
        if let Ok(cutter) = make_convex_hull(body, &extruded) {
            if let Ok(cut) = boolean_solid(body, result, cutter, BooleanOp::Cut, tol) {
                result = cut;
            }
        }
    }
    Ok(result)
}

fn face_corners(body: &Body, face: FaceId) -> Vec<Pnt3> {
    let mut pts = Vec::new();
    let Some(face_data) = body.faces.get(face) else {
        return pts;
    };
    let mut loops = Vec::new();
    if let Some(outer) = face_data.outer {
        loops.push(outer);
    }
    loops.extend(face_data.inners.iter().copied());
    for loop_id in loops {
        for coedge in body.loop_coedges(loop_id) {
            if let Some((start, _)) = body.coedge_endpoints(coedge) {
                if let Some(v) = body.vertices.get(start) {
                    pts.push(v.position);
                }
            }
        }
    }
    pts
}

fn face_normal_hint(pts: &[Pnt3]) -> Option<Vec3> {
    if pts.len() < 3 {
        return None;
    }
    let a = pts[1] - pts[0];
    let b = pts[2] - pts[0];
    Some(a.cross(b))
}

/// ↔️ Apply draft angle `angle_rad` to `face` of `solid` (MVP: AABB shear for boxes).
pub fn draft_angle(
    body: &mut Body,
    solid: SolidId,
    _face: FaceId,
    angle_rad: f64,
    pull_dir: Vec3,
) -> Result<SolidId, KernelError> {
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity(format!("solid {solid}")));
    }
    if angle_rad.abs() <= 1e-15 {
        return Err(KernelError::Operation("draft angle must be non-zero".into()));
    }
    if !angle_rad.is_finite() {
        return Err(KernelError::InvalidInput("draft angle must be finite".into()));
    }
    let pull = pull_dir
        .normalized()
        .ok_or_else(|| KernelError::InvalidInput("pull direction must be non-zero".into()))?;
    if looks_like_box(body, solid)? {
        let bb = solid_bounding_box(body, solid)?;
        let sheared = shear_aabb_corners(&bb, pull, angle_rad);
        return make_convex_hull(body, &sheared);
    }
    shell_copy_solid(body, solid)
}

// #endregion 🔖️Api

// #region 🧮Aabb

fn looks_like_box(body: &Body, solid: SolidId) -> Result<bool, KernelError> {
    let faces = body.solid_faces(solid);
    if faces.len() != 6 {
        return Ok(false);
    }
    let bb = solid_bounding_box(body, solid)?;
    let v_bb = aabb_volume(&bb);
    let v = solid_volume(body, solid, 1e-4)?;
    Ok((v - v_bb).abs() <= 1e-3 * (1.0 + v_bb))
}

fn aabb_volume(bb: &AxisAlignedBox) -> f64 {
    (bb.max.x - bb.min.x).max(0.0)
        * (bb.max.y - bb.min.y).max(0.0)
        * (bb.max.z - bb.min.z).max(0.0)
}

fn inflate_aabb(bb: &AxisAlignedBox, distance: f64) -> AxisAlignedBox {
    AxisAlignedBox {
        min: Pnt3::new(bb.min.x - distance, bb.min.y - distance, bb.min.z - distance),
        max: Pnt3::new(bb.max.x + distance, bb.max.y + distance, bb.max.z + distance),
    }
}

fn aabb_corners(bb: &AxisAlignedBox) -> [Pnt3; 8] {
    [
        Pnt3::new(bb.min.x, bb.min.y, bb.min.z),
        Pnt3::new(bb.max.x, bb.min.y, bb.min.z),
        Pnt3::new(bb.max.x, bb.max.y, bb.min.z),
        Pnt3::new(bb.min.x, bb.max.y, bb.min.z),
        Pnt3::new(bb.min.x, bb.min.y, bb.max.z),
        Pnt3::new(bb.max.x, bb.min.y, bb.max.z),
        Pnt3::new(bb.max.x, bb.max.y, bb.max.z),
        Pnt3::new(bb.min.x, bb.max.y, bb.max.z),
    ]
}

fn make_box_from_aabb(body: &mut Body, bb: &AxisAlignedBox) -> Result<SolidId, KernelError> {
    let w = (bb.max.x - bb.min.x).max(0.0);
    let d = (bb.max.y - bb.min.y).max(0.0);
    let h = (bb.max.z - bb.min.z).max(0.0);
    if w <= 1e-15 || d <= 1e-15 || h <= 1e-15 {
        return Err(KernelError::Operation("degenerate box".into()));
    }
    make_convex_hull(body, &aabb_corners(bb))
}

fn shear_aabb_corners(bb: &AxisAlignedBox, pull: Vec3, angle_rad: f64) -> Vec<Pnt3> {
    let tan_a = angle_rad.tan();
    let mut u = pull.cross(Vec3::Z);
    if u.norm() < 1e-9 {
        u = pull.cross(Vec3::X);
    }
    let u = u.normalized().unwrap_or(Vec3::X);
    let corners = aabb_corners(bb);
    let min_pull = corners.iter().map(|p| p.to_vec().dot(pull)).fold(f64::INFINITY, f64::min);
    corners
        .iter()
        .map(|p| {
            let h = p.to_vec().dot(pull) - min_pull;
            let shift = u * (h * tan_a);
            Pnt3::new(p.x + shift.x, p.y + shift.y, p.z + shift.z)
        })
        .collect()
}

// #endregion 🧮Aabb

// #region 🧮Mesh

fn mesh_offset_points(body: &Body, solid: SolidId, distance: f64, tol: f64) -> Result<Vec<Pnt3>, KernelError> {
    let mesh = tessellate_solid(body, solid, tol.max(1e-3))?;
    let tris = mesh_triangles(&mesh);
    let scale = 1.0 / tol.max(1e-9);
    let mut normals: HashMap<(i64, i64, i64), Vec3> = HashMap::new();
    let mut positions: HashMap<(i64, i64, i64), Pnt3> = HashMap::new();
    for (p0, p1, p2) in tris {
        let n = (p1 - p0).cross(p2 - p0);
        let nn = n.normalized().unwrap_or(Vec3::Z);
        for p in [p0, p1, p2] {
            let key = quantize(p, scale);
            positions.entry(key).or_insert(p);
            let entry = normals.entry(key).or_insert(Vec3::ZERO);
            *entry = Vec3::new(entry.x + nn.x, entry.y + nn.y, entry.z + nn.z);
        }
    }
    let mut out = Vec::with_capacity(normals.len());
    for (key, mut n) in normals {
        let p = positions
            .get(&key)
            .copied()
            .unwrap_or(Pnt3::new(0.0, 0.0, 0.0));
        if let Some(nn) = n.normalized() {
            n = nn;
        } else {
            n = Vec3::Z;
        }
        out.push(Pnt3::new(
            p.x + n.x * distance,
            p.y + n.y * distance,
            p.z + n.z * distance,
        ));
    }
    Ok(out)
}

fn quantize(p: Pnt3, scale: f64) -> (i64, i64, i64) {
    (
        (p.x * scale).round() as i64,
        (p.y * scale).round() as i64,
        (p.z * scale).round() as i64,
    )
}

fn mesh_points(mesh: &MeshTransfer) -> Vec<Pnt3> {
    mesh.position
        .chunks_exact(3)
        .map(|c| Pnt3::new(c[0] as f64, c[1] as f64, c[2] as f64))
        .collect()
}

fn mesh_triangles(mesh: &MeshTransfer) -> Vec<(Pnt3, Pnt3, Pnt3)> {
    let pts = mesh_points(mesh);
    if !mesh.index.is_empty() {
        return mesh
            .index
            .chunks_exact(3)
            .filter_map(|tri| {
                let i0 = tri[0] as usize;
                let i1 = tri[1] as usize;
                let i2 = tri[2] as usize;
                Some((*pts.get(i0)?, *pts.get(i1)?, *pts.get(i2)?))
            })
            .collect();
    }
    pts.chunks_exact(3)
        .map(|c| (c[0], c[1], c[2]))
        .collect()
}

// #endregion 🧮Mesh

// #region 🧮Face

fn face_outer_polygon(body: &Body, face: FaceId) -> Result<Vec<Pnt3>, KernelError> {
    let face_data = body
        .faces
        .get(face)
        .ok_or_else(|| KernelError::MissingEntity(format!("face {face:?}")))?;
    let outer = face_data
        .outer
        .ok_or_else(|| KernelError::InvalidInput("face has no outer loop".into()))?;
    let coedges = body.loop_coedges(outer);
    if coedges.is_empty() {
        return Err(KernelError::InvalidInput("face outer loop is empty".into()));
    }
    let mut points = Vec::new();
    for cid in coedges {
        let coedge = body
            .coedges
            .get(cid)
            .ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid:?}")))?;
        let edge = body
            .edges
            .get(coedge.edge)
            .ok_or_else(|| KernelError::MissingEntity(format!("edge {:?}", coedge.edge)))?;
        let curve = body
            .curves3
            .get(edge.curve)
            .ok_or_else(|| KernelError::MissingEntity(format!("curve {:?}", edge.curve)))?;
        match curve {
            Curve3::Circle { frame, radius } => {
                let segments = 16usize.max(3);
                let (t0, t1) = edge.range;
                for i in 0..segments {
                    let t = if coedge.forward {
                        t0 + (t1 - t0) * i as f64 / segments as f64
                    } else {
                        t1 - (t1 - t0) * i as f64 / segments as f64
                    };
                    points.push(curve.eval(t));
                }
                let _ = (frame, radius);
            }
            _ => {
                let (start, _) = body
                    .coedge_endpoints(cid)
                    .ok_or_else(|| KernelError::MissingEntity(format!("coedge endpoints {cid:?}")))?;
                let p = body.vertices.get(start).expect("vertex").position;
                points.push(p);
            }
        }
    }
    if points.len() < 3 {
        return Err(KernelError::InvalidInput("face polygon has fewer than 3 points".into()));
    }
    Ok(points)
}

fn thicken_face_hull(body: &mut Body, face: FaceId, distance: f64) -> Result<SolidId, KernelError> {
    let polygon = face_outer_polygon(body, face)?;
    let offset_face_id = offset_face(body, face, distance)?;
    let offset_poly = face_outer_polygon(body, offset_face_id)?;
    let mut pts = polygon;
    pts.extend(offset_poly);
    make_convex_hull(body, &pts)
}

fn shell_copy_solid(body: &mut Body, solid: SolidId) -> Result<SolidId, KernelError> {
    let faces = body.solid_faces(solid);
    let mut rec = OpRecorder::new();
    let shell = add_shell(body, faces, &mut rec);
    Ok(add_solid(body, shell, Vec::new(), &mut rec))
}

fn solid_with_void_shell(body: &mut Body, outer: SolidId, inner: SolidId) -> Result<SolidId, KernelError> {
    let outer_faces = body.solid_faces(outer);
    let inner_faces = body.solid_faces(inner);
    let mut rec = OpRecorder::new();
    let outer_shell = add_shell(body, outer_faces, &mut rec);
    let inner_shell = add_shell(body, inner_faces, &mut rec);
    Ok(add_solid(body, outer_shell, vec![inner_shell], &mut rec))
}

// #endregion 🧮Face

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::primitives::{make_box, make_planar_face_from_points};

    #[test]
    fn offset_solid_box_grows_volume() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        let v0 = solid_volume(&body, solid, 1e-4).unwrap();
        let grown = offset_solid(&mut body, solid, 0.2).unwrap();
        let v1 = solid_volume(&body, grown, 1e-4).unwrap();
        assert!(v1 > v0, "v0={v0} v1={v1}");
    }

    #[test]
    fn thicken_rectangle_positive_volume() {
        let mut body = Body::new();
        let face = make_planar_face_from_points(
            &mut body,
            &[
                Pnt3::new(0.0, 0.0, 0.0),
                Pnt3::new(2.0, 0.0, 0.0),
                Pnt3::new(2.0, 1.0, 0.0),
                Pnt3::new(0.0, 1.0, 0.0),
            ],
        )
        .unwrap();
        let solid = thicken_face(&mut body, face, 0.5).unwrap();
        let v = solid_volume(&body, solid, 1e-4).unwrap();
        assert!(v > 0.0, "volume {v}");
    }

    #[test]
    fn shell_box_has_faces() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0).unwrap();
        let shelled = shell_solid(&mut body, solid, 0.2).unwrap();
        let faces = body.solid_faces(shelled);
        assert!(!faces.is_empty(), "shell should have faces");
        assert!(faces.len() >= 6, "face count {}", faces.len());
    }

    #[test]
    fn offset_determinism_face_count() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        let a = offset_solid(&mut body, solid, 0.1).unwrap();
        let b = offset_solid(&mut body, solid, 0.1).unwrap();
        assert_eq!(body.solid_faces(a).len(), body.solid_faces(b).len());
    }

    #[test]
    fn draft_zero_angle_errors() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        let face = body.solid_faces(solid)[0];
        let err = draft_angle(&mut body, solid, face, 0.0, Vec3::Z).unwrap_err();
        assert!(matches!(err, KernelError::Operation(_)));
    }
}

// #endregion 🔖️Tests
