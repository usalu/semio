//! 🎨️ Rolling-ball fillet, variable fillet, chamfer (MVP convex-hull approximation).
//!
//! Lane 5-blend of ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.
//! Exact rolling-ball topology surgery is deferred; selected edges are blunted by sampling
//! inset / quarter-circle points in the adjacent-face bisector frame, then rebuilding via
//! [`crate::brep::primitives::make_convex_hull`].

use std::collections::HashSet;
use std::f64::consts::FRAC_PI_2;

use crate::brep::arena::{EdgeId, FaceId, SolidId, VertexId};
use crate::brep::error::KernelError;
use crate::brep::measure::{edge_length, solid_volume};
use crate::brep::primitives::{make_box, make_convex_hull};
use crate::brep::surface::Surface;
use crate::brep::topo::Body;
use crate::brep::vec::{Pnt3, Vec3};

// #region 🔖️Api

/// 🎨️ Constant-radius fillet on `edges` of `solid` (MVP hull of arc-sampled blunt points).
pub fn fillet_edges(
    body: &mut Body,
    solid: SolidId,
    edges: &[EdgeId],
    radius: f64,
) -> Result<SolidId, KernelError> {
    validate_blend_request(body, solid, edges, radius)?;
    let points = sample_blunt_points(body, solid, edges, BlendKind::Fillet { radius })?;
    make_convex_hull(body, &points)
}

/// 🎨️ Linearly varying fillet radius `r0→r1` along a single `edge` (MVP hull approximation).
pub fn fillet_variable(
    body: &mut Body,
    solid: SolidId,
    edge: EdgeId,
    r0: f64,
    r1: f64,
) -> Result<SolidId, KernelError> {
    if r0 <= 0.0 || r1 <= 0.0 {
        return Err(KernelError::InvalidInput(
            "variable fillet radii must be positive".into(),
        ));
    }
    validate_blend_request(body, solid, &[edge], r0.max(r1))?;
    let points = sample_blunt_points(
        body,
        solid,
        &[edge],
        BlendKind::Variable { r0, r1 },
    )?;
    make_convex_hull(body, &points)
}

/// 🎨️ Constant-distance chamfer on `edges` of `solid` (MVP hull of face-inset samples).
pub fn chamfer_edges(
    body: &mut Body,
    solid: SolidId,
    edges: &[EdgeId],
    distance: f64,
) -> Result<SolidId, KernelError> {
    validate_blend_request(body, solid, edges, distance)?;
    let points = sample_blunt_points(body, solid, edges, BlendKind::Chamfer { distance })?;
    make_convex_hull(body, &points)
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

fn validate_blend_request(
    body: &Body,
    solid: SolidId,
    edges: &[EdgeId],
    amount: f64,
) -> Result<(), KernelError> {
    require_solid(body, solid)?;
    if edges.is_empty() {
        return Err(KernelError::InvalidInput(
            "blend requires at least one edge".into(),
        ));
    }
    if !(amount.is_finite() && amount > 0.0) {
        return Err(KernelError::InvalidInput(
            "blend radius/distance must be positive".into(),
        ));
    }
    let solid_edges = solid_edge_set(body, solid);
    for &edge in edges {
        if !solid_edges.contains(&edge) {
            return Err(KernelError::MissingEntity(format!(
                "edge {edge:?} is not on solid"
            )));
        }
        let min_adj = min_adjacent_edge_length(body, edge)?;
        if amount >= min_adj {
            return Err(KernelError::InvalidInput(format!(
                "blend amount {amount} must be smaller than min adjacent edge length {min_adj}"
            )));
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
    let ent = body
        .edges
        .get(edge)
        .ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
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
        return Err(KernelError::InvalidInput(
            "edge has no measurable adjacent edges".into(),
        ));
    }
    Ok(min_len)
}

fn edge_adjacent_faces(
    body: &Body,
    solid: SolidId,
    edge: EdgeId,
) -> Result<(FaceId, FaceId), KernelError> {
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
        return Err(KernelError::Operation(
            "blend edge must be shared by two solid faces".into(),
        ));
    }
    Ok((faces[0], faces[1]))
}

fn face_outward_normal(body: &Body, face: FaceId) -> Result<Vec3, KernelError> {
    let face_ent = body
        .faces
        .get(face)
        .ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = body
        .surfaces
        .get(face_ent.surface)
        .ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
    let n = match surface {
        Surface::Plane { frame } => frame.z,
        other => other.normal(0.0, 0.0).ok_or_else(|| {
            KernelError::Operation("could not evaluate face normal for blend".into())
        })?,
    };
    let n = if face_ent.flipped { -n } else { n };
    n.normalized()
        .ok_or_else(|| KernelError::Operation("degenerate face normal".into()))
}

fn sample_blunt_points(
    body: &Body,
    solid: SolidId,
    edges: &[EdgeId],
    kind: BlendKind,
) -> Result<Vec<Pnt3>, KernelError> {
    let selected: HashSet<EdgeId> = edges.iter().copied().collect();
    let mut endpoint_verts: HashSet<VertexId> = HashSet::new();
    for &edge in edges {
        let ent = body
            .edges
            .get(edge)
            .ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
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

    for &edge in &selected {
        let ent = body
            .edges
            .get(edge)
            .ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
        let a = body
            .vertices
            .get(ent.v0)
            .ok_or_else(|| KernelError::MissingEntity("vertex".into()))?
            .position;
        let b = body
            .vertices
            .get(ent.v1)
            .ok_or_else(|| KernelError::MissingEntity("vertex".into()))?
            .position;
        let (f0, f1) = edge_adjacent_faces(body, solid, edge)?;
        let n0 = face_outward_normal(body, f0)?;
        let n1 = face_outward_normal(body, f1)?;

        for si in 0..EDGE_STATIONS {
            let t = if EDGE_STATIONS == 1 {
                0.5
            } else {
                si as f64 / (EDGE_STATIONS - 1) as f64
            };
            let p = a.lerp(b, t);
            let amount = match kind {
                BlendKind::Fillet { radius } => radius,
                BlendKind::Variable { r0, r1 } => r0 * (1.0 - t) + r1 * t,
                BlendKind::Chamfer { distance } => distance,
            };
            match kind {
                BlendKind::Chamfer { .. } => {
                    points.push(p - n0 * amount);
                    points.push(p - n1 * amount);
                }
                BlendKind::Fillet { .. } | BlendKind::Variable { .. } => {
                    // Quarter-circle in the plane spanned by the two outward normals:
                    // center sits inward from both faces; samples replace the sharp edge.
                    let center = p - n0 * amount - n1 * amount;
                    for ai in 0..ARC_SAMPLES {
                        let theta = if ARC_SAMPLES == 1 {
                            0.0
                        } else {
                            (ai as f64) * FRAC_PI_2 / (ARC_SAMPLES - 1) as f64
                        };
                        let pt = center + n0 * (amount * theta.cos()) + n1 * (amount * theta.sin());
                        points.push(pt);
                    }
                }
            }
        }
    }

    if points.len() < 4 {
        return Err(KernelError::Operation(
            "blend produced too few sample points for a solid hull".into(),
        ));
    }
    Ok(points)
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
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        let vol0 = solid_volume(&body, solid, 1e-6).unwrap();
        let edges = solid_edges(&body, solid);
        assert_eq!(edges.len(), 12);
        let out = chamfer_edges(&mut body, solid, &edges, 0.1).unwrap();
        let vol1 = solid_volume(&body, out, 1e-6).unwrap();
        assert!(
            vol1 < vol0 - 1e-6,
            "chamfered volume {vol1} should be < original {vol0}"
        );
    }

    #[test]
    fn fillet_one_edge_yields_solid() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0).unwrap();
        let edge = solid_edges(&body, solid)[0];
        let out = fillet_edges(&mut body, solid, &[edge], 0.2).unwrap();
        assert!(!body.solid_faces(out).is_empty());
        let vol = solid_volume(&body, out, 1e-6).unwrap();
        assert!(vol > 0.0 && vol.is_finite());
    }

    #[test]
    fn fillet_variable_runs() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 2.0, 2.0, 2.0).unwrap();
        let edge = solid_edges(&body, solid)[0];
        let out = fillet_variable(&mut body, solid, edge, 0.1, 0.3).unwrap();
        assert!(!body.solid_faces(out).is_empty());
    }

    #[test]
    fn reject_zero_radius() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        let edge = solid_edges(&body, solid)[0];
        assert!(fillet_edges(&mut body, solid, &[edge], 0.0).is_err());
        assert!(chamfer_edges(&mut body, solid, &[edge], 0.0).is_err());
        assert!(fillet_variable(&mut body, solid, edge, 0.0, 0.1).is_err());
    }

    #[test]
    fn reject_empty_edges() {
        let mut body = Body::new();
        let solid = make_box(&mut body, 1.0, 1.0, 1.0).unwrap();
        assert!(fillet_edges(&mut body, solid, &[], 0.1).is_err());
        assert!(chamfer_edges(&mut body, solid, &[], 0.1).is_err());
    }
}
// #endregion 🧪Tests
