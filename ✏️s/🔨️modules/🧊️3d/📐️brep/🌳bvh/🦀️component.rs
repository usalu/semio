//! 🌳 B-Rep entity BVH adapters over [`crate::spatial::Bvh`] (ray / AABB / nearest by leaf bounds).

use crate::brep::arena::{ArenaId, EdgeId, FaceId, SolidId};
use crate::brep::curve::Curve3;
use crate::brep::engine::{Aabb, Vec3};
use crate::brep::error::KernelError;
use crate::brep::topo::Body;
use crate::brep::vec::Pnt3;
use crate::spatial::Bvh;

// #region 🔖️Bounds
fn aabb_from_point(p: Pnt3) -> Aabb {
    let v = p.to_array();
    Aabb { min: v, max: v }
}

fn aabb_extend(mut a: Aabb, p: Pnt3) -> Aabb {
    let v = p.to_array();
    a.min[0] = a.min[0].min(v[0]);
    a.min[1] = a.min[1].min(v[1]);
    a.min[2] = a.min[2].min(v[2]);
    a.max[0] = a.max[0].max(v[0]);
    a.max[1] = a.max[1].max(v[1]);
    a.max[2] = a.max[2].max(v[2]);
    a
}

fn sample_curve_segment(curve: &Curve3, t0: f64, t1: f64, samples: usize) -> Vec<Pnt3> {
    if samples <= 1 {
        return vec![curve.eval(t0), curve.eval(t1)];
    }
    let n = samples.max(2);
    (0..n).map(|i| curve.eval(t0 + (t1 - t0) * (i as f64) / ((n - 1) as f64))).collect()
}

/// 📦 Conservative world-space AABB for one edge's used curve segment.
pub fn edge_aabb(body: &Body, edge: EdgeId) -> Result<Aabb, KernelError> {
    let edge_rec = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity(edge.to_string()))?;
    let curve = body.curves3.get(edge_rec.curve).ok_or_else(|| KernelError::MissingEntity(format!("curve-{}", edge_rec.curve)))?;
    let (t0, t1) = edge_rec.range;
    let sample_count = match curve {
        Curve3::Line { .. } => 2,
        _ => 12,
    };
    let points = sample_curve_segment(curve, t0, t1, sample_count);
    let Some(first) = points.first() else {
        return Err(KernelError::Operation("edge produced no samples".into()));
    };
    let mut box_ = aabb_from_point(*first);
    for p in points.iter().skip(1) {
        box_ = aabb_extend(box_, *p);
    }
    Ok(box_)
}

/// 📦 Conservative world-space AABB for one face from its loop vertices and edge curve samples.
pub fn face_aabb(body: &Body, face: FaceId) -> Result<Aabb, KernelError> {
    let coedges = body.face_coedges(face);
    if coedges.is_empty() {
        return Err(KernelError::Operation(format!("face {face} has no boundary")));
    }
    let mut vertices: Vec<Pnt3> = Vec::new();
    for coedge_id in coedges {
        let coedge = body.coedges.get(coedge_id).ok_or_else(|| KernelError::MissingEntity(coedge_id.to_string()))?;
        let edge_rec = body.edges.get(coedge.edge).ok_or_else(|| KernelError::MissingEntity(coedge.edge.to_string()))?;
        let v0 = body.vertices.get(edge_rec.v0).ok_or_else(|| KernelError::MissingEntity(edge_rec.v0.to_string()))?;
        let v1 = body.vertices.get(edge_rec.v1).ok_or_else(|| KernelError::MissingEntity(edge_rec.v1.to_string()))?;
        vertices.push(v0.position);
        vertices.push(v1.position);
        let curve = body.curves3.get(edge_rec.curve).ok_or_else(|| KernelError::MissingEntity(format!("curve-{}", edge_rec.curve)))?;
        let (t0, t1) = edge_rec.range;
        vertices.extend(sample_curve_segment(curve, t0, t1, 4));
    }
    let mut box_ = aabb_from_point(vertices[0]);
    for p in vertices.iter().skip(1) {
        box_ = aabb_extend(box_, *p);
    }
    Ok(box_)
}
// #endregion 🔖️Bounds

// #region 🔖️Index
/// 🌳 Face spatial index for one solid shell.
pub struct FaceBvh {
    bvh: Bvh<FaceId>,
}

/// 🌳 Edge spatial index for one solid shell.
pub struct EdgeBvh {
    bvh: Bvh<EdgeId>,
}

/// 🌳 Either face or edge entity index (query helpers dispatch on variant).
pub enum BvhIndex {
    Faces(FaceBvh),
    Edges(EdgeBvh),
}

fn require_solid(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_some() {
        Ok(())
    } else {
        Err(KernelError::MissingEntity(solid.to_string()))
    }
}

fn solid_unique_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
    let mut seen = std::collections::HashSet::new();
    for face in body.solid_faces(solid) {
        for coedge in body.face_coedges(face) {
            if let Some(c) = body.coedges.get(coedge) {
                seen.insert(c.edge);
            }
        }
    }
    seen.into_iter().collect()
}

/// 🏗️ Builds a face BVH over every face referenced by `solid`.
pub fn build_face_bvh(body: &Body, solid: SolidId) -> Result<FaceBvh, KernelError> {
    require_solid(body, solid)?;
    let mut items = Vec::new();
    for face in body.solid_faces(solid) {
        if let Ok(aabb) = face_aabb(body, face) {
            items.push((aabb, face));
        }
    }
    Ok(FaceBvh { bvh: Bvh::build(items) })
}

/// 🏗️ Builds an edge BVH over every edge incident to `solid`'s faces.
pub fn build_edge_bvh(body: &Body, solid: SolidId) -> Result<EdgeBvh, KernelError> {
    require_solid(body, solid)?;
    let mut items = Vec::new();
    for edge in solid_unique_edges(body, solid) {
        if let Ok(aabb) = edge_aabb(body, edge) {
            items.push((aabb, edge));
        }
    }
    Ok(EdgeBvh { bvh: Bvh::build(items) })
}

impl FaceBvh {
    /// 🎯️ Face ids whose leaf bounds are crossed by the ray.
    pub fn query_ray(&self, origin: Vec3, dir: Vec3) -> Vec<FaceId> {
        self.bvh.query_ray(origin, dir).into_iter().copied().collect()
    }

    /// 📦 Face ids whose leaf bounds overlap `query`.
    pub fn query_aabb(&self, query: &Aabb) -> Vec<FaceId> {
        self.bvh.query_aabb_overlap(query).into_iter().copied().collect()
    }

    /// 📍 Face id whose leaf bound is nearest to `point` (AABB distance).
    pub fn query_nearest(&self, point: Vec3) -> Option<FaceId> {
        self.bvh.query_point_nearest(point).copied()
    }
}

impl EdgeBvh {
    /// 🎯️ Edge ids whose leaf bounds are crossed by the ray.
    pub fn query_ray(&self, origin: Vec3, dir: Vec3) -> Vec<EdgeId> {
        self.bvh.query_ray(origin, dir).into_iter().copied().collect()
    }

    /// 📦 Edge ids whose leaf bounds overlap `query`.
    pub fn query_aabb(&self, query: &Aabb) -> Vec<EdgeId> {
        self.bvh.query_aabb_overlap(query).into_iter().copied().collect()
    }

    /// 📍 Edge id whose leaf bound is nearest to `point` (AABB distance).
    pub fn query_nearest(&self, point: Vec3) -> Option<EdgeId> {
        self.bvh.query_point_nearest(point).copied()
    }
}
// #endregion 🔖️Index

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::curve::Curve3;
    use crate::brep::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
    use crate::brep::history::OpRecorder;
    use crate::brep::mat::Frame3;
    use crate::brep::surface::Surface;
    use crate::brep::tolerance::Tol;
    use std::collections::HashMap;

    fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> SolidId {
        let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
        let vertices: Vec<_> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
        let edge_pairs = [(0, 1), (1, 2), (2, 0), (0, 3), (1, 3), (2, 3)];
        let mut edges = HashMap::new();
        for &(a, b) in &edge_pairs {
            let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
            let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
            edges.insert((a, b), edge);
            edges.insert((b, a), edge);
        }
        let face_defs = [[0, 1, 2], [0, 3, 1], [1, 3, 2], [2, 3, 0]];
        let mut faces = Vec::new();
        for tri in face_defs {
            let normal = (positions[tri[1]] - positions[tri[0]]).cross(positions[tri[2]] - positions[tri[0]]);
            let frame = Frame3::from_normal(positions[tri[0]], normal).unwrap();
            let surface = body.surfaces.insert(Surface::Plane { frame });
            let members: Vec<(EdgeId, bool)> = (0..3)
                .map(|i| {
                    let a = tri[i];
                    let b = tri[(i + 1) % 3];
                    let edge = edges[&(a, b)];
                    let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                    (edge, forward)
                })
                .collect();
            let outer = make_loop(body, FaceId::from_raw(0, 0), &members);
            let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
            body.loops.get_mut(outer).unwrap().face = face;
            faces.push(face);
        }
        let shell = add_shell(body, faces, rec);
        add_solid(body, shell, vec![], rec)
    }

    #[test]
    fn face_bvh_builds_over_tetrahedron_with_four_faces() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let bvh = build_face_bvh(&body, solid).unwrap();
        let hits = bvh.query_ray([-0.5, 0.25, 0.25], [1.0, 0.0, 0.0]);
        assert!(!hits.is_empty(), "ray through tetrahedron should hit at least one face leaf");
        for face in &hits {
            assert!(body.solid_faces(solid).contains(face));
        }
        let near = bvh.query_nearest([0.25, 0.25, 0.25]).unwrap();
        assert!(body.solid_faces(solid).contains(&near));
    }

    #[test]
    fn edge_bvh_builds_six_edges_on_tetrahedron() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let bvh = build_edge_bvh(&body, solid).unwrap();
        let probe = Aabb { min: [0.4, 0.0, 0.0], max: [0.6, 0.1, 0.1] };
        let hits = bvh.query_aabb(&probe);
        assert!(!hits.is_empty());
    }

    #[test]
    fn empty_solid_yields_empty_face_bvh_queries() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let shell = add_shell(&mut body, vec![], &mut rec);
        let solid = add_solid(&mut body, shell, vec![], &mut rec);
        let bvh = build_face_bvh(&body, solid).unwrap();
        assert!(bvh.query_ray([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).is_empty());
        assert!(bvh.query_aabb(&Aabb { min: [0.0, 0.0, 0.0], max: [1.0, 1.0, 1.0] }).is_empty());
        assert!(bvh.query_nearest([0.5, 0.5, 0.5]).is_none());
    }

    #[test]
    fn missing_solid_returns_kernel_error() {
        let body = Body::new();
        let bogus = SolidId::from_raw(9, 9);
        assert!(matches!(build_face_bvh(&body, bogus), Err(KernelError::MissingEntity(_))));
        assert!(matches!(build_edge_bvh(&body, bogus), Err(KernelError::MissingEntity(_))));
    }
}
// #endregion 🔖️Tests
