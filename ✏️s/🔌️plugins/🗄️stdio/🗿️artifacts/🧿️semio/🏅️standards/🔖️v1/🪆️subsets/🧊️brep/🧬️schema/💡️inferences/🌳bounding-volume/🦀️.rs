//! 🌳️ B-Rep entity bounding-volume-hierarchy query: brep-specific `FaceBvh`/`EdgeBvh`
//! adapters over a generic `spatial::Bvh<T>` (AABB overlap, nearest point, ray queries).
//! `spatial` (below) is kernel-agnostic and moved here only because this was its sole
//! consumer repo-wide.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/{📐️brep/🌳️bvh,🗺️spatial}/🦀️.rs` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL2.

// #region 🔖️Spatial
pub mod spatial {
    //! 🌳️ Generic bounding-volume-hierarchy spatial index over AABB-bounded items.
    //!
    //! Kernel-agnostic: works with any leaf payload (`semio_framework_3d::brep::kernel` instantiates
    //! `Bvh<FaceId>`/`Bvh<EdgeId>` internally) using only this file's own `engine::contract`'s `Vec3`/`Aabb`
    //! types, so this crate never depends on brep and stays reusable by other 3D kernels.

    use crate::standards::v1::subsets::brep::schema::engine::{Aabb, Vec3};

    // #region 🔖️AabbHelpers
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn aabb_union(a: &Aabb, b: &Aabb) -> Aabb {
        Aabb { min: [a.min[0].min(b.min[0]), a.min[1].min(b.min[1]), a.min[2].min(b.min[2])], max: [a.max[0].max(b.max[0]), a.max[1].max(b.max[1]), a.max[2].max(b.max[2])] }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn aabb_center(a: &Aabb) -> Vec3 {
        [(a.min[0] + a.max[0]) * 0.5, (a.min[1] + a.max[1]) * 0.5, (a.min[2] + a.max[2]) * 0.5]
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn aabb_overlaps(a: &Aabb, b: &Aabb) -> bool {
        a.min[0] <= b.max[0] && a.max[0] >= b.min[0] && a.min[1] <= b.max[1] && a.max[1] >= b.min[1] && a.min[2] <= b.max[2] && a.max[2] >= b.min[2]
    }

    /// 🎯️ Ray-AABB slab intersection test (`origin + t * dir`, `t >= 0`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn aabb_ray_hits(aabb: &Aabb, origin: Vec3, dir: Vec3) -> bool {
        aabb_ray_entry(aabb, origin, dir).is_some()
    }

    /// 🎯️ Ray-AABB slab intersection returning the clamped (`>= 0`) entry parameter, or `None` if
    /// the ray misses — the ordering key for [`Bvh::query_ray_ordered`] and the traversal guard
    /// for [`Bvh::query_nearest_exact`]'s branch-and-bound.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn aabb_ray_entry(aabb: &Aabb, origin: Vec3, dir: Vec3) -> Option<f64> {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;
        for axis in 0..3 {
            if dir[axis].abs() < 1e-12 {
                if origin[axis] < aabb.min[axis] || origin[axis] > aabb.max[axis] {
                    return None;
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
                return None;
            }
        }
        if tmax < 0.0 {
            None
        } else {
            Some(tmin.max(0.0))
        }
    }
    // #endregion 🔖️AabbHelpers

    // #region 🔖️Bvh
    enum Node<T> {
        Leaf { aabb: Aabb, item: T },
        Branch { aabb: Aabb, left: Box<Node<T>>, right: Box<Node<T>> },
    }

    impl<T> Node<T> {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn build(items: Vec<(Aabb, T)>) -> Self {
            Self { root: Self::build_node(items) }
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn query_point_nearest(&self, point: Vec3) -> Option<&T> {
            let mut best: Option<(&T, f64)> = None;
            Self::visit_nearest(self.root.as_ref(), point, &mut best);
            best.map(|(item, _)| item)
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn query_ray(&self, origin: Vec3, dir: Vec3) -> Vec<&T> {
            let mut hits = Vec::new();
            Self::visit_ray(self.root.as_ref(), origin, dir, &mut hits);
            hits
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn query_aabb_overlap(&self, query: &Aabb) -> Vec<&T> {
            let mut hits = Vec::new();
            Self::visit_overlap(self.root.as_ref(), query, &mut hits);
            hits
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

        /// 🎯️ Like [`Bvh::query_ray`], but sorted near-to-far by AABB entry parameter `t` — lets a
        /// caller stop at the first candidate whose *exact* intersection succeeds instead of
        /// resolving every leaf the ray's box crosses.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn query_ray_ordered(&self, origin: Vec3, dir: Vec3) -> Vec<(&T, f64)> {
            let mut hits = Vec::new();
            Self::visit_ray_ordered(self.root.as_ref(), origin, dir, &mut hits);
            hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            hits
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn visit_ray_ordered<'a>(node: Option<&'a Node<T>>, origin: Vec3, dir: Vec3, hits: &mut Vec<(&'a T, f64)>) {
            let Some(node) = node else { return };
            let Some(t) = aabb_ray_entry(node.aabb(), origin, dir) else { return };
            match node {
                Node::Leaf { item, .. } => hits.push((item, t)),
                Node::Branch { left, right, .. } => {
                    Self::visit_ray_ordered(Some(left), origin, dir, hits);
                    Self::visit_ray_ordered(Some(right), origin, dir, hits);
                }
            }
        }

        /// 🔍️ Branch-and-bound nearest search: `exact` computes a leaf item's TRUE distance (not
        /// just its AABB lower bound), and a subtree is pruned whenever its AABB lower bound
        /// already exceeds the best exact distance found so far — unlike
        /// [`Bvh::query_point_nearest`], this returns the genuinely closest item.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn query_nearest_exact<F: Fn(&T) -> f64>(&self, point: Vec3, exact: F) -> Option<&T> {
            let mut best: Option<(&T, f64)> = None;
            Self::visit_nearest_exact(self.root.as_ref(), point, &exact, &mut best);
            best.map(|(item, _)| item)
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn visit_nearest_exact<'a, F: Fn(&T) -> f64>(node: Option<&'a Node<T>>, point: Vec3, exact: &F, best: &mut Option<(&'a T, f64)>) {
            let Some(node) = node else { return };
            let lower_bound = aabb_point_distance_sq(node.aabb(), point).sqrt();
            if let Some((_, best_dist)) = best {
                if lower_bound > *best_dist {
                    return;
                }
            }
            match node {
                Node::Leaf { item, .. } => {
                    let d = exact(item);
                    let better = match best {
                        None => true,
                        Some((_, best_dist)) => d < *best_dist,
                    };
                    if better {
                        *best = Some((item, d));
                    }
                }
                Node::Branch { left, right, .. } => {
                    let left_lb = aabb_point_distance_sq(left.aabb(), point);
                    let right_lb = aabb_point_distance_sq(right.aabb(), point);
                    let (near, far) = if left_lb <= right_lb { (left, right) } else { (right, left) };
                    Self::visit_nearest_exact(Some(near), point, exact, best);
                    Self::visit_nearest_exact(Some(far), point, exact, best);
                }
            }
        }

        /// 🔁️ Recomputes every leaf and branch AABB in place (same tree topology) from a fresh
        /// `bounds` lookup — cheaper than [`Bvh::build`] when items moved slightly but membership
        /// didn't change enough to warrant a new median-split partition.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn refit<F: Fn(&T) -> Aabb>(&mut self, bounds: &F) {
            if let Some(node) = &mut self.root {
                Self::refit_node(node, bounds);
            }
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn refit_node<F: Fn(&T) -> Aabb>(node: &mut Node<T>, bounds: &F) -> Aabb {
            match node {
                Node::Leaf { aabb, item } => {
                    *aabb = bounds(item);
                    aabb.clone()
                }
                Node::Branch { aabb, left, right } => {
                    let left_aabb = Self::refit_node(left, bounds);
                    let right_aabb = Self::refit_node(right, bounds);
                    *aabb = aabb_union(&left_aabb, &right_aabb);
                    aabb.clone()
                }
            }
        }
    }
    // #endregion 🔖️Bvh

    // #region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️spatial-unit/🦀️.rs");
    // #endregion 🔖️Tests
}
// #endregion 🔖️Spatial

// 🌳 B-Rep entity BVH adapters over `spatial::Bvh` (ray / AABB / nearest by leaf bounds).

use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::closest_point_on_face;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::{EdgeId, FaceId, SolidId};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::Pnt3;
use crate::standards::v1::subsets::brep::schema::engine::{Aabb, Vec3};
use spatial::Bvh;

// #region 🔖️Bounds
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_from_point(p: Pnt3) -> Aabb {
    let v = p.to_array();
    Aabb { min: v, max: v }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_curve_segment(curve: &Curve3, t0: f64, t1: f64, samples: usize) -> Vec<Pnt3> {
    if samples <= 1 {
        return vec![curve.eval(t0), curve.eval(t1)];
    }
    let n = samples.max(2);
    (0..n).map(|i| curve.eval(t0 + (t1 - t0) * (i as f64) / ((n - 1) as f64))).collect()
}

/// 📦 Conservative world-space AABB for one edge's used curve segment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 📦 Conservative world-space AABB for one face from its loop vertices, edge curve samples
/// (trimmed boundary) plus, for non-planar surfaces, a coarse interior parametric grid over the
/// surface's own domain — a plane can't bulge past its boundary samples, but a saddle-shaped NURBS
/// patch's interior can, so boundary-only sampling under-bounds it. The grid samples the FULL
/// surface domain rather than a trim-clipped subset, which stays a valid (if slightly loose)
/// superset of the true trimmed region without needing this module to depend on UV triangulation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    if let Some(face_ent) = body.faces.get(face) {
        if let Some(surface) = body.surfaces.get(face_ent.surface) {
            if !matches!(surface, Surface::Plane { .. }) {
                vertices.extend(surface_interior_samples(surface));
            }
        }
    }
    let mut box_ = aabb_from_point(vertices[0]);
    for p in vertices.iter().skip(1) {
        box_ = aabb_extend(box_, *p);
    }
    Ok(box_)
}

/// 📦 A coarse `5x5` grid of world-space points over `surface`'s own parametric domain, used to
/// conservatively widen a face's AABB past what boundary sampling alone would capture.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn surface_interior_samples(surface: &Surface) -> Vec<Pnt3> {
    const GRID: usize = 5;
    let ((u0, u1), (v0, v1)) = surface.domain();
    let mut pts = Vec::with_capacity(GRID * GRID);
    for i in 0..GRID {
        let u = u0 + (u1 - u0) * (i as f64) / ((GRID - 1) as f64);
        for j in 0..GRID {
            let v = v0 + (v1 - v0) * (j as f64) / ((GRID - 1) as f64);
            pts.push(surface.eval(u, v));
        }
    }
    pts
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_solid(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_some() {
        Ok(())
    } else {
        Err(KernelError::MissingEntity(solid.to_string()))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn query_ray(&self, origin: Vec3, dir: Vec3) -> Vec<FaceId> {
        self.bvh.query_ray(origin, dir).into_iter().copied().collect()
    }

    /// 📦 Face ids whose leaf bounds overlap `query`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn query_aabb(&self, query: &Aabb) -> Vec<FaceId> {
        self.bvh.query_aabb_overlap(query).into_iter().copied().collect()
    }

    /// 📍 Face id whose leaf bound is nearest to `point` (AABB distance).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn query_nearest(&self, point: Vec3) -> Option<FaceId> {
        self.bvh.query_point_nearest(point).copied()
    }

    /// 🎯️ Face ids whose leaf bounds are crossed by the ray, ordered near-to-far by AABB entry.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn query_ray_ordered(&self, origin: Vec3, dir: Vec3) -> Vec<(FaceId, f64)> {
        self.bvh.query_ray_ordered(origin, dir).into_iter().map(|(f, t)| (*f, t)).collect()
    }

    /// 📍 The face genuinely closest to `point` (surface `closest_point` plus trim test, not just
    /// AABB distance) and its closest point and distance, via BVH branch-and-bound.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn closest_face(&self, body: &Body, point: Pnt3) -> Option<(FaceId, Pnt3, f64)> {
        let target = point.to_array();
        let face = *self.bvh.query_nearest_exact(target, |f: &FaceId| closest_point_on_face(body, *f, point).map_or(f64::INFINITY, |(_, d)| d))?;
        let (p, d) = closest_point_on_face(body, face, point).ok()?;
        Some((face, p, d))
    }

    /// 🔁️ Recomputes every face's AABB in place from the current `body` geometry.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn refit(&mut self, body: &Body) {
        self.bvh.refit(&|f: &FaceId| face_aabb(body, *f).unwrap_or(Aabb { min: [0.0; 3], max: [0.0; 3] }));
    }
}

impl EdgeBvh {
    /// 🎯️ Edge ids whose leaf bounds are crossed by the ray.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn query_ray(&self, origin: Vec3, dir: Vec3) -> Vec<EdgeId> {
        self.bvh.query_ray(origin, dir).into_iter().copied().collect()
    }

    /// 📦 Edge ids whose leaf bounds overlap `query`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn query_aabb(&self, query: &Aabb) -> Vec<EdgeId> {
        self.bvh.query_aabb_overlap(query).into_iter().copied().collect()
    }

    /// 📍 Edge id whose leaf bound is nearest to `point` (AABB distance).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn query_nearest(&self, point: Vec3) -> Option<EdgeId> {
        self.bvh.query_point_nearest(point).copied()
    }

    /// 🔁️ Recomputes every edge's AABB in place from the current `body` geometry.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn refit(&mut self, body: &Body) {
        self.bvh.refit(&|e: &EdgeId| edge_aabb(body, *e).unwrap_or(Aabb { min: [0.0; 3], max: [0.0; 3] }));
    }
}

/// 🌳 One solid's face spatial index, keyed by the [`SolidId`] it was built from — the unit a
/// future `Brep` engine wrapper caches per solid rather than rebuilding per query (audit §6.10).
pub struct SolidBvh {
    pub solid: SolidId,
    faces: FaceBvh,
}

impl SolidBvh {
    /// 🏗️ Builds the face index for `solid`.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn build(body: &Body, solid: SolidId) -> Result<Self, KernelError> {
        Ok(Self { solid, faces: build_face_bvh(body, solid)? })
    }

    /// 🌳 Borrows the underlying face index for direct ray/AABB/nearest queries.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn faces(&self) -> &FaceBvh {
        &self.faces
    }

    /// 📍 The face genuinely closest to `point` and its closest point and distance.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn closest_face(&self, body: &Body, point: Pnt3) -> Option<(FaceId, Pnt3, f64)> {
        self.faces.closest_face(body, point)
    }

    /// 🔁️ Recomputes every face's AABB in place from the current `body` geometry.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn refit(&mut self, body: &Body) {
        self.faces.refit(body);
    }
}
// #endregion 🔖️Index

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
