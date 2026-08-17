//! 🌳️ B-Rep entity bounding-volume-hierarchy query: brep-specific `FaceBvh`/`EdgeBvh`
//! adapters over a generic `spatial::Bvh<T>` (AABB overlap, nearest point, ray queries).
//! `spatial` (below) is kernel-agnostic and moved here only because this was its sole
//! consumer repo-wide.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/{📐️brep/🌳️bvh,🗺️spatial}/🦀️component.rs` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL2.

// #region 🔖️Spatial
pub mod spatial {
    //! 🌳️ Generic bounding-volume-hierarchy spatial index over AABB-bounded items.
    //!
    //! Kernel-agnostic: works with any leaf payload (`semio_framework_3d::brep::kernel` instantiates
    //! `Bvh<FaceId>`/`Bvh<EdgeId>` internally) using only [`semio_framework_3d::engine`]'s `Vec3`/`Aabb`
    //! types, so this crate never depends on brep and stays reusable by other 3D kernels.

    use semio_framework_3d::engine::{Aabb, Vec3};

    // #region 🔖️AabbHelpers
    fn aabb_union(a: &Aabb, b: &Aabb) -> Aabb {
        Aabb { min: [a.min[0].min(b.min[0]), a.min[1].min(b.min[1]), a.min[2].min(b.min[2])], max: [a.max[0].max(b.max[0]), a.max[1].max(b.max[1]), a.max[2].max(b.max[2])] }
    }

    fn aabb_center(a: &Aabb) -> Vec3 {
        [(a.min[0] + a.max[0]) * 0.5, (a.min[1] + a.max[1]) * 0.5, (a.min[2] + a.max[2]) * 0.5]
    }

    fn aabb_longest_axis(a: &Aabb) -> usize {
        let extents = [a.max[0] - a.min[0], a.max[1] - a.min[1], a.max[2] - a.min[2]];
        if extents[0] >= extents[1] && extents[0] >= extents[2] {
            0
        } else if extents[1] >= extents[2] {
            1
        } else {
            2
        }
    }

    /// 📏️ Squared distance from `point` to the closest point on `aabb` (0 if inside).
    fn aabb_point_distance_sq(aabb: &Aabb, point: Vec3) -> f64 {
        let mut d = 0.0;
        for ((&v, &lo), &hi) in point.iter().zip(aabb.min.iter()).zip(aabb.max.iter()) {
            if v < lo {
                d += (lo - v).powi(2);
            } else if v > hi {
                d += (v - hi).powi(2);
            }
        }
        d
    }

    fn aabb_overlaps(a: &Aabb, b: &Aabb) -> bool {
        a.min[0] <= b.max[0] && a.max[0] >= b.min[0] && a.min[1] <= b.max[1] && a.max[1] >= b.min[1] && a.min[2] <= b.max[2] && a.max[2] >= b.min[2]
    }

    /// 🎯️ Ray-AABB slab intersection test (`origin + t * dir`, `t >= 0`).
    fn aabb_ray_hits(aabb: &Aabb, origin: Vec3, dir: Vec3) -> bool {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;
        for axis in 0..3 {
            if dir[axis].abs() < 1e-12 {
                if origin[axis] < aabb.min[axis] || origin[axis] > aabb.max[axis] {
                    return false;
                }
                continue;
            }
            let inv = 1.0 / dir[axis];
            let mut t0 = (aabb.min[axis] - origin[axis]) * inv;
            let mut t1 = (aabb.max[axis] - origin[axis]) * inv;
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }
            tmin = tmin.max(t0);
            tmax = tmax.min(t1);
            if tmin > tmax {
                return false;
            }
        }
        tmax >= 0.0
    }
    // #endregion 🔖️AabbHelpers

    // #region 🔖️Bvh
    enum Node<T> {
        Leaf { aabb: Aabb, item: T },
        Branch { aabb: Aabb, left: Box<Node<T>>, right: Box<Node<T>> },
    }

    impl<T> Node<T> {
        fn aabb(&self) -> &Aabb {
            match self {
                Node::Leaf { aabb, .. } => aabb,
                Node::Branch { aabb, .. } => aabb,
            }
        }
    }

    /// 🌳️ Bounding-volume hierarchy over `(Aabb, T)` items, built once via median split.
    pub struct Bvh<T> {
        root: Option<Node<T>>,
    }

    impl<T> Bvh<T> {
        /// 🏗️ Builds a BVH from AABB-bounded items via recursive median split on the longest axis.
        pub fn build(items: Vec<(Aabb, T)>) -> Self {
            Self { root: Self::build_node(items) }
        }

        fn build_node(mut items: Vec<(Aabb, T)>) -> Option<Node<T>> {
            if items.is_empty() {
                return None;
            }
            if items.len() == 1 {
                // 🛡️ len == 1 was just checked above, so `next()` structurally cannot be `None`.
                let (aabb, item) = items.into_iter().next().expect("checked len == 1");
                return Some(Node::Leaf { aabb, item });
            }
            // 🛡️ items.is_empty() and items.len() == 1 were both ruled out above, so len >= 2 and `reduce` over a non-empty iterator cannot be `None`.
            let bounds = items.iter().map(|(aabb, _)| aabb.clone()).reduce(|a, b| aabb_union(&a, &b)).expect("checked non-empty");
            let axis = aabb_longest_axis(&bounds);
            items.sort_by(|(a, _), (b, _)| aabb_center(a)[axis].partial_cmp(&aabb_center(b)[axis]).unwrap_or(std::cmp::Ordering::Equal));
            let mid = items.len() / 2;
            let right_items = items.split_off(mid);
            // 🛡️ len >= 2 here implies mid = len/2 is in 1..=len-1, so both the retained `items` (left, len == mid) and `right_items` (len - mid) are non-empty — `build_node` only returns `None` for an empty input.
            let left = Self::build_node(items).expect("checked non-empty left partition");
            let right = Self::build_node(right_items).expect("checked non-empty right partition");
            Some(Node::Branch { aabb: bounds, left: Box::new(left), right: Box::new(right) })
        }

        /// 🔍️ Returns the item whose leaf AABB is nearest to `point` (by AABB distance, not exact
        /// item-surface distance) — callers refine among close candidates via the exact kernel
        /// query; this only narrows the candidate set.
        pub fn query_point_nearest(&self, point: Vec3) -> Option<&T> {
            let mut best: Option<(&T, f64)> = None;
            Self::visit_nearest(self.root.as_ref(), point, &mut best);
            best.map(|(item, _)| item)
        }

        fn visit_nearest<'a>(node: Option<&'a Node<T>>, point: Vec3, best: &mut Option<(&'a T, f64)>) {
            let Some(node) = node else { return };
            let bound_dist = aabb_point_distance_sq(node.aabb(), point);
            if let Some((_, best_dist)) = best {
                if bound_dist > *best_dist {
                    return;
                }
            }
            match node {
                Node::Leaf { item, .. } => {
                    let is_better = match best {
                        None => true,
                        Some((_, best_dist)) => bound_dist < *best_dist,
                    };
                    if is_better {
                        *best = Some((item, bound_dist));
                    }
                }
                Node::Branch { left, right, .. } => {
                    Self::visit_nearest(Some(left), point, best);
                    Self::visit_nearest(Some(right), point, best);
                }
            }
        }

        /// 🎯️ Returns all items whose leaf AABB is crossed by the ray `origin + t * dir` (`t >= 0`).
        pub fn query_ray(&self, origin: Vec3, dir: Vec3) -> Vec<&T> {
            let mut hits = Vec::new();
            Self::visit_ray(self.root.as_ref(), origin, dir, &mut hits);
            hits
        }

        fn visit_ray<'a>(node: Option<&'a Node<T>>, origin: Vec3, dir: Vec3, hits: &mut Vec<&'a T>) {
            let Some(node) = node else { return };
            if !aabb_ray_hits(node.aabb(), origin, dir) {
                return;
            }
            match node {
                Node::Leaf { item, .. } => hits.push(item),
                Node::Branch { left, right, .. } => {
                    Self::visit_ray(Some(left), origin, dir, hits);
                    Self::visit_ray(Some(right), origin, dir, hits);
                }
            }
        }

        /// 📦️ Returns all items whose leaf AABB overlaps `query`.
        pub fn query_aabb_overlap(&self, query: &Aabb) -> Vec<&T> {
            let mut hits = Vec::new();
            Self::visit_overlap(self.root.as_ref(), query, &mut hits);
            hits
        }

        fn visit_overlap<'a>(node: Option<&'a Node<T>>, query: &Aabb, hits: &mut Vec<&'a T>) {
            let Some(node) = node else { return };
            if !aabb_overlaps(node.aabb(), query) {
                return;
            }
            match node {
                Node::Leaf { item, .. } => hits.push(item),
                Node::Branch { left, right, .. } => {
                    Self::visit_overlap(Some(left), query, hits);
                    Self::visit_overlap(Some(right), query, hits);
                }
            }
        }
    }
    // #endregion 🔖️Bvh

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn aabb(min: Vec3, max: Vec3) -> Aabb {
            Aabb { min, max }
        }

        #[test]
        fn empty_bvh_returns_no_matches() {
            let bvh: Bvh<u32> = Bvh::build(Vec::new());
            assert_eq!(bvh.query_point_nearest([0.0, 0.0, 0.0]), None);
            assert!(bvh.query_ray([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).is_empty());
            assert!(bvh.query_aabb_overlap(&aabb([0.0, 0.0, 0.0], [1.0, 1.0, 1.0])).is_empty());
        }

        #[test]
        fn nearest_point_finds_closest_leaf() {
            let items = vec![(aabb([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), "near"), (aabb([10.0, 10.0, 10.0], [11.0, 11.0, 11.0]), "far")];
            let bvh = Bvh::build(items);
            assert_eq!(bvh.query_point_nearest([0.5, 0.5, 0.5]), Some(&"near"));
            assert_eq!(bvh.query_point_nearest([10.5, 10.5, 10.5]), Some(&"far"));
        }

        #[test]
        fn ray_hits_only_crossed_leaves() {
            let items = vec![(aabb([0.0, -1.0, -1.0], [1.0, 1.0, 1.0]), "hit"), (aabb([0.0, 10.0, 10.0], [1.0, 11.0, 11.0]), "miss")];
            let bvh = Bvh::build(items);
            let hits = bvh.query_ray([-5.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
            assert_eq!(hits, vec![&"hit"]);
        }

        #[test]
        fn aabb_overlap_finds_intersecting_leaves() {
            let items = vec![(aabb([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), "overlap"), (aabb([5.0, 5.0, 5.0], [6.0, 6.0, 6.0]), "disjoint")];
            let bvh = Bvh::build(items);
            let mut hits = bvh.query_aabb_overlap(&aabb([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]));
            hits.sort();
            assert_eq!(hits, vec![&"overlap"]);
        }

        #[test]
        fn many_leaves_build_and_query_correctly() {
            let items: Vec<(Aabb, usize)> = (0..200).map(|i| (aabb([i as f64, 0.0, 0.0], [i as f64 + 0.5, 0.5, 0.5]), i)).collect();
            let bvh = Bvh::build(items);
            assert_eq!(bvh.query_point_nearest([100.2, 0.2, 0.2]), Some(&100));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Spatial

// 🌳 B-Rep entity BVH adapters over `spatial::Bvh` (ray / AABB / nearest by leaf bounds).

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{EdgeId, FaceId, SolidId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Pnt3;
use semio_framework_3d::engine::{Aabb, Vec3};
use spatial::Bvh;

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
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
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
