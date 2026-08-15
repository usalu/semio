//! 🎨️ Rolling-ball fillet, variable fillet, chamfer (MVP convex-hull approximation).
//!
//! Lane 5-blend of ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.
//! Exact rolling-ball topology surgery is deferred; selected edges are blunted by sampling
//! inset / quarter-circle strips in the adjacent-face frame, stitched via triangle soup
//! (convex hull only if soup construction fails).
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🎨️blend` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL.

use std::collections::HashSet;
use std::f64::consts::FRAC_PI_2;

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_convex_hull, solid_from_triangle_soup};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::{edge_length, solid_volume};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{EdgeId, FaceId, SolidId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// #region 🔖️Api

/// 🎨️ Constant-radius fillet on `edges` of `solid` (MVP arc-strip triangle soup). `rec` is threaded
/// through to [`solid_from_blend_samples`] so the sample solid's provenance escapes this call.
pub fn fillet_edges(body: &mut Body, solid: SolidId, edges: &[EdgeId], radius: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    validate_blend_request(body, solid, edges, radius)?;
    let (points, tris) = sample_blunt_geometry(body, solid, edges, BlendKind::Fillet { radius })?;
    solid_from_blend_samples(body, &points, &tris, rec)
}

/// 🎨️ Linearly varying fillet radius `r0→r1` along a single `edge` (MVP arc-strip triangle soup).
pub fn fillet_variable(body: &mut Body, solid: SolidId, edge: EdgeId, r0: f64, r1: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if r0 <= 0.0 || r1 <= 0.0 {
        return Err(KernelError::InvalidInput("variable fillet radii must be positive".into()));
    }
    validate_blend_request(body, solid, &[edge], r0.max(r1))?;
    let (points, tris) = sample_blunt_geometry(body, solid, &[edge], BlendKind::Variable { r0, r1 })?;
    solid_from_blend_samples(body, &points, &tris, rec)
}

/// 🎨️ Constant-distance chamfer on `edges` of `solid` (MVP inset-strip triangle soup).
pub fn chamfer_edges(body: &mut Body, solid: SolidId, edges: &[EdgeId], distance: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    validate_blend_request(body, solid, edges, distance)?;
    let (points, tris) = sample_blunt_geometry(body, solid, edges, BlendKind::Chamfer { distance })?;
    solid_from_blend_samples(body, &points, &tris, rec)
}

// #endregion 🔖️Api

// #region 🔧Helpers

#[derive(Clone, Copy)]
enum BlendKind {
    Fillet { radius: f64 },
    Variable { r0: f64, r1: f64 },
    Chamfer { distance: f64 },
}

const EDGE_STATIONS: usize = 5;
const ARC_SAMPLES: usize = 5;

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

fn require_solid(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_none() {
        return Err(KernelError::MissingEntity("solid".into()));
    }
    Ok(())
}

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

fn solid_vertex_ids(body: &Body, solid: SolidId) -> HashSet<VertexId> {
    let mut verts = HashSet::new();
    for edge in solid_edge_set(body, solid) {
        if let Some(e) = body.edges.get(edge) {
            verts.insert(e.v0);
            verts.insert(e.v1);
        }
    }
    verts
}

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

fn edge_adjacent_faces(body: &Body, solid: SolidId, edge: EdgeId) -> Result<(FaceId, FaceId), KernelError> {
    let solid_faces: HashSet<FaceId> = body.solid_faces(solid).into_iter().collect();
    let mut faces = Vec::new();
    for coedge in body.edge_coedges(edge) {
        let Some(c) = body.coedges.get(coedge) else {
            continue;
        };
        let Some(lp) = body.loops.get(c.loop_id) else {
            continue;
        };
        if solid_faces.contains(&lp.face) && !faces.contains(&lp.face) {
            faces.push(lp.face);
        }
    }
    if faces.len() < 2 {
        return Err(KernelError::Operation("blend edge must be shared by two solid faces".into()));
    }
    Ok((faces[0], faces[1]))
}

fn face_outward_normal(body: &Body, face: FaceId) -> Result<Vec3, KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = body.surfaces.get(face_ent.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
    let n = match surface {
        Surface::Plane { frame } => frame.z,
        other => other.normal(0.0, 0.0).ok_or_else(|| KernelError::Operation("could not evaluate face normal for blend".into()))?,
    };
    let n = if face_ent.flipped { -n } else { n };
    n.normalized().ok_or_else(|| KernelError::Operation("degenerate face normal".into()))
}

fn solid_from_blend_samples(body: &mut Body, points: &[Pnt3], tris: &[[Pnt3; 3]], rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if !tris.is_empty() {
        if let Ok(id) = solid_from_triangle_soup(body, tris, rec) {
            return Ok(id);
        }
    }
    make_convex_hull(body, points, rec)
}

fn sample_blunt_geometry(body: &Body, solid: SolidId, edges: &[EdgeId], kind: BlendKind) -> Result<(Vec<Pnt3>, Vec<[Pnt3; 3]>), KernelError> {
    let selected: HashSet<EdgeId> = edges.iter().copied().collect();
    let mut endpoint_verts: HashSet<VertexId> = HashSet::new();
    for &edge in edges {
        let ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
        endpoint_verts.insert(ent.v0);
        endpoint_verts.insert(ent.v1);
    }

    let mut points = Vec::new();
    for vid in solid_vertex_ids(body, solid) {
        if endpoint_verts.contains(&vid) {
            continue;
        }
        if let Some(v) = body.vertices.get(vid) {
            points.push(v.position);
        }
    }

    let mut tris: Vec<[Pnt3; 3]> = Vec::new();
    for &edge in &selected {
        let ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
        let a = body.vertices.get(ent.v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position;
        let b = body.vertices.get(ent.v1).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position;
        let (f0, f1) = edge_adjacent_faces(body, solid, edge)?;
        let n0 = face_outward_normal(body, f0)?;
        let n1 = face_outward_normal(body, f1)?;

        let mut stations: Vec<Vec<Pnt3>> = Vec::with_capacity(EDGE_STATIONS);
        for si in 0..EDGE_STATIONS {
            let t = if EDGE_STATIONS == 1 { 0.5 } else { si as f64 / (EDGE_STATIONS - 1) as f64 };
            let p = a.lerp(b, t);
            let amount = match kind {
                BlendKind::Fillet { radius } => radius,
                BlendKind::Variable { r0, r1 } => r0 * (1.0 - t) + r1 * t,
                BlendKind::Chamfer { distance } => distance,
            };
            let mut ring = Vec::new();
            match kind {
                BlendKind::Chamfer { .. } => {
                    let p0 = p - n0 * amount;
                    let p1 = p - n1 * amount;
                    ring.push(p0);
                    ring.push(p1);
                    points.push(p0);
                    points.push(p1);
                }
                BlendKind::Fillet { .. } | BlendKind::Variable { .. } => {
                    let center = p - n0 * amount - n1 * amount;
                    for ai in 0..ARC_SAMPLES {
                        let theta = if ARC_SAMPLES == 1 { 0.0 } else { (ai as f64) * FRAC_PI_2 / (ARC_SAMPLES - 1) as f64 };
                        let pt = center + n0 * (amount * theta.cos()) + n1 * (amount * theta.sin());
                        ring.push(pt);
                        points.push(pt);
                    }
                }
            }
            stations.push(ring);
        }

        for si in 0..stations.len().saturating_sub(1) {
            let a_ring = &stations[si];
            let b_ring = &stations[si + 1];
            let n = a_ring.len().min(b_ring.len());
            for i in 0..n.saturating_sub(1) {
                let a0 = a_ring[i];
                let a1 = a_ring[i + 1];
                let b0 = b_ring[i];
                let b1 = b_ring[i + 1];
                tris.push([a0, b0, a1]);
                tris.push([a1, b0, b1]);
            }
        }
    }

    if points.len() < 4 {
        return Err(KernelError::Operation("blend produced too few sample points for a solid".into()));
    }
    Ok((points, tris))
}

// #endregion 🔧Helpers

// #region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn solid_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
        let mut edges: Vec<EdgeId> = solid_edge_set(body, solid).into_iter().collect();
        edges.sort_by_key(|e| format!("{e:?}"));
        edges
    }

    #[test]
    fn chamfer_all_box_edges_reduces_volume() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let vol0 = solid_volume(&body, solid, 1e-6).unwrap();
        let edges = solid_edges(&body, solid);
        assert_eq!(edges.len(), 12);
        let out = chamfer_edges(&mut body, solid, &edges, 0.1, &mut rec).unwrap();
        let vol1 = solid_volume(&body, out, 1e-6).unwrap();
        assert!(vol1 < vol0 - 1e-6, "chamfered volume {vol1} should be < original {vol0}");
    }

    #[test]
    fn fillet_one_edge_yields_solid() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
        let edge = solid_edges(&body, solid)[0];
        let out = fillet_edges(&mut body, solid, &[edge], 0.2, &mut rec).unwrap();
        assert!(!body.solid_faces(out).is_empty());
        let vol = solid_volume(&body, out, 1e-6).unwrap();
        assert!(vol > 0.0 && vol.is_finite());
    }

    #[test]
    fn fillet_variable_runs() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
        let edge = solid_edges(&body, solid)[0];
        let out = fillet_variable(&mut body, solid, edge, 0.1, 0.3, &mut rec).unwrap();
        assert!(!body.solid_faces(out).is_empty());
    }

    #[test]
    fn reject_zero_radius() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let edge = solid_edges(&body, solid)[0];
        assert!(fillet_edges(&mut body, solid, &[edge], 0.0, &mut rec).is_err());
        assert!(chamfer_edges(&mut body, solid, &[edge], 0.0, &mut rec).is_err());
        assert!(fillet_variable(&mut body, solid, edge, 0.0, 0.1, &mut rec).is_err());
    }

    #[test]
    fn reject_empty_edges() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        assert!(fillet_edges(&mut body, solid, &[], 0.1, &mut rec).is_err());
        assert!(chamfer_edges(&mut body, solid, &[], 0.1, &mut rec).is_err());
    }
}
// #endregion 🧪Tests
