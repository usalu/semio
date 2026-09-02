//! 🧿️ Triangle-mesh collision queries: a BVH-accelerated shape-vs-shape overlap test and a
//! winding-number point-containment test — the framework-owned replacement for
//! `parry3d::shape::{SharedShape,TriMesh,TriMeshFlags}` plus `parry3d::query::intersection_test`.
//! Correctness pinned against `parry3d` (kept only as a `[dev-dependencies]` oracle on this
//! crate) across a deterministic mesh-pair corpus; see the `🧪️Parry3dOracle` test region.
//!
//! Input meshes are expected pre-oriented (outward-facing, consistent winding) — the caller's
//! contract under `parry3d::shape::TriMeshFlags::ORIENTED`, which this module also assumes rather
//! than re-derives: [`contains_point`]'s winding-number sum only comes out near `0`/`±1` when the
//! triangle winding is already consistent.

use crate::rigid::{Isometry3, Point3, Vector3};

//#region 🔖️Aabb3
#[derive(Clone, Copy, Debug)]
struct Aabb3 {
    min: Point3,
    max: Point3,
}

impl Aabb3 {
    fn of_triangle(a: Point3, b: Point3, c: Point3) -> Self {
        Self { min: a.inf(b).inf(c), max: a.sup(b).sup(c) }
    }

    fn union(self, other: Self) -> Self {
        Self { min: self.min.inf(other.min), max: self.max.sup(other.max) }
    }

    fn overlaps(self, other: Self) -> bool {
        self.min.x <= other.max.x && other.min.x <= self.max.x && self.min.y <= other.max.y && other.min.y <= self.max.y && self.min.z <= other.max.z && other.min.z <= self.max.z
    }

    fn center(self) -> Point3 {
        Point3::new((self.min.x + self.max.x) * 0.5, (self.min.y + self.max.y) * 0.5, (self.min.z + self.max.z) * 0.5)
    }

    fn longest_axis(self) -> usize {
        let extent = [self.max.x - self.min.x, self.max.y - self.min.y, self.max.z - self.min.z];
        if extent[0] >= extent[1] && extent[0] >= extent[2] {
            0
        } else if extent[1] >= extent[2] {
            1
        } else {
            2
        }
    }

    fn axis(self, index: usize) -> f32 {
        match index {
            0 => self.center().x,
            1 => self.center().y,
            _ => self.center().z,
        }
    }
}
//#endregion 🔖️Aabb3

//#region 🔖️Bvh
enum BvhNode {
    Leaf { aabb: Aabb3, triangle: u32 },
    Branch { aabb: Aabb3, left: Box<BvhNode>, right: Box<BvhNode> },
}

impl BvhNode {
    fn aabb(&self) -> Aabb3 {
        match self {
            Self::Leaf { aabb, .. } | Self::Branch { aabb, .. } => *aabb,
        }
    }

    /// 🏗️ Recursive median split on the longest axis of the enclosing bounds — a standard,
    /// balanced-by-construction BVH build; `items` is always non-empty by construction here.
    fn build(mut items: Vec<(Aabb3, u32)>) -> Self {
        if items.len() == 1 {
            let (aabb, triangle) = items[0];
            return Self::Leaf { aabb, triangle };
        }
        let bounds = items.iter().map(|(aabb, _)| *aabb).reduce(Aabb3::union).expect("non-empty by construction");
        let axis = bounds.longest_axis();
        items.sort_by(|(a, _), (b, _)| a.axis(axis).partial_cmp(&b.axis(axis)).unwrap_or(std::cmp::Ordering::Equal));
        let mid = items.len() / 2;
        let right_items = items.split_off(mid);
        Self::Branch { aabb: bounds, left: Box::new(Self::build(items)), right: Box::new(Self::build(right_items)) }
    }

    fn collect_overlaps(&self, query: Aabb3, out: &mut Vec<u32>) {
        if !self.aabb().overlaps(query) {
            return;
        }
        match self {
            Self::Leaf { triangle, .. } => out.push(*triangle),
            Self::Branch { left, right, .. } => {
                left.collect_overlaps(query, out);
                right.collect_overlaps(query, out);
            }
        }
    }
}
//#endregion 🔖️Bvh

//#region 🔖️TriMesh
/// 🔺️ An immutable indexed triangle mesh plus its precomputed BVH — the framework replacement
/// for `parry3d::shape::TriMesh` wrapped in `parry3d::shape::SharedShape`.
pub struct TriMesh {
    vertices: Vec<Point3>,
    triangles: Vec<[u32; 3]>,
    bvh: Option<BvhNode>,
}

impl TriMesh {
    pub fn new(vertices: Vec<Point3>, triangles: Vec<[u32; 3]>) -> Self {
        let leaves: Vec<(Aabb3, u32)> = triangles
            .iter()
            .enumerate()
            .map(|(index, triangle)| {
                let [a, b, c] = triangle.map(|vertex| vertices[vertex as usize]);
                (Aabb3::of_triangle(a, b, c), index as u32)
            })
            .collect();
        let bvh = (!leaves.is_empty()).then(|| BvhNode::build(leaves));
        Self { vertices, triangles, bvh }
    }

    fn triangle_at(&self, index: u32) -> [Point3; 3] {
        self.triangles[index as usize].map(|vertex| self.vertices[vertex as usize])
    }
}
//#endregion 🔖️TriMesh

//#region 🔖️TriangleTriangle
const TRI_EPS: f32 = 1e-6;

fn triangle_plane(tri: [Point3; 3]) -> (Vector3, f32) {
    let normal = (tri[1] - tri[0]).cross(tri[2] - tri[0]);
    let d = -normal.dot(tri[0].coords());
    (normal, d)
}

fn signed_distances(tri: [Point3; 3], normal: Vector3, d: f32) -> [f32; 3] {
    tri.map(|vertex| vertex.coords().dot(normal) + d)
}

fn sign(value: f32) -> i32 {
    if value.abs() < TRI_EPS {
        0
    } else if value > 0.0 {
        1
    } else {
        -1
    }
}

/// 📏️ The `[min,max]` parameter interval where `tri`'s boundary crosses the plane whose signed
/// distances at `tri`'s vertices are `dist`, projected onto `proj`. `None` when the triangle only
/// grazes the plane (no genuine crossing) — a degenerate, measure-zero contact.
fn crossing_interval(tri: [Point3; 3], proj: impl Fn(Point3) -> f32, dist: [f32; 3]) -> Option<(f32, f32)> {
    let mut hits: Vec<f32> = Vec::with_capacity(2);
    for (a, b) in [(0, 1), (1, 2), (2, 0)] {
        let (da, db) = (dist[a], dist[b]);
        if (da > 0.0 && db < 0.0) || (da < 0.0 && db > 0.0) {
            let t = da / (da - db);
            hits.push(proj(tri[a]) + t * (proj(tri[b]) - proj(tri[a])));
        } else if da.abs() < TRI_EPS {
            hits.push(proj(tri[a]));
        }
    }
    if hits.len() < 2 {
        return None;
    }
    let (mut lo, mut hi) = (hits[0], hits[0]);
    for &value in &hits[1..] {
        lo = lo.min(value);
        hi = hi.max(value);
    }
    Some((lo, hi))
}

/// 📐️ Signed area x2 of the 2D triangle `(a,b,c)`.
fn cross2(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn point_in_or_on_triangle_2d(p: [f32; 2], tri: [[f32; 2]; 3]) -> bool {
    let d0 = cross2(tri[0], tri[1], p);
    let d1 = cross2(tri[1], tri[2], p);
    let d2 = cross2(tri[2], tri[0], p);
    let has_neg = d0 < -TRI_EPS || d1 < -TRI_EPS || d2 < -TRI_EPS;
    let has_pos = d0 > TRI_EPS || d1 > TRI_EPS || d2 > TRI_EPS;
    !(has_neg && has_pos)
}

fn segments_intersect_2d(a0: [f32; 2], a1: [f32; 2], b0: [f32; 2], b1: [f32; 2]) -> bool {
    let d1 = cross2(b0, b1, a0);
    let d2 = cross2(b0, b1, a1);
    let d3 = cross2(a0, a1, b0);
    let d4 = cross2(a0, a1, b1);
    ((d1 > TRI_EPS && d2 < -TRI_EPS) || (d1 < -TRI_EPS && d2 > TRI_EPS)) && ((d3 > TRI_EPS && d4 < -TRI_EPS) || (d3 < -TRI_EPS && d4 > TRI_EPS))
}

/// 🪙️ Coplanar triangle-triangle overlap via 2D projection onto the dominant plane of `normal`.
fn coplanar_triangle_intersect(t1: [Point3; 3], t2: [Point3; 3], normal: Vector3) -> bool {
    let axes = [normal.x.abs(), normal.y.abs(), normal.z.abs()];
    let drop_axis = if axes[0] >= axes[1] && axes[0] >= axes[2] {
        0
    } else if axes[1] >= axes[2] {
        1
    } else {
        2
    };
    let project = |p: Point3| -> [f32; 2] {
        match drop_axis {
            0 => [p.y, p.z],
            1 => [p.x, p.z],
            _ => [p.x, p.y],
        }
    };
    let (p1, p2) = (t1.map(project), t2.map(project));
    for i in 0..3 {
        for j in 0..3 {
            if segments_intersect_2d(p1[i], p1[(i + 1) % 3], p2[j], p2[(j + 1) % 3]) {
                return true;
            }
        }
    }
    p1.iter().any(|&p| point_in_or_on_triangle_2d(p, p2)) || p2.iter().any(|&p| point_in_or_on_triangle_2d(p, p1))
}

/// 🔺️🔺️ Möller (1997) fast triangle-triangle intersection test, extended with an explicit
/// coplanar branch (2D segment/point-containment overlap on the dominant projection plane).
/// Touching (shared vertex/edge/face, or exactly grazing) counts as intersecting, matching
/// `parry3d::query::intersection_test`'s closed-interval convention.
fn triangle_triangle_intersect(t1: [Point3; 3], t2: [Point3; 3]) -> bool {
    let (n2, d2) = triangle_plane(t2);
    let du = signed_distances(t1, n2, d2);
    let du_signs = du.map(sign);
    if du_signs[0] != 0 && du_signs[0] == du_signs[1] && du_signs[0] == du_signs[2] {
        return false;
    }

    let (n1, d1) = triangle_plane(t1);
    let dv = signed_distances(t2, n1, d1);
    let dv_signs = dv.map(sign);
    if dv_signs[0] != 0 && dv_signs[0] == dv_signs[1] && dv_signs[0] == dv_signs[2] {
        return false;
    }

    let dir = n1.cross(n2);
    // 🛡️ Scale-invariant parallel check: `n1`/`n2` carry each triangle's raw area (unnormalized),
    // so comparing `|dir|` against a bare `TRI_EPS` would misclassify small triangles as
    // coplanar; comparing against `|n1|²·|n2|²` compares `sin(angle-between-planes)` instead.
    if dir.dot(dir) < TRI_EPS * TRI_EPS * n1.dot(n1).max(1e-12) * n2.dot(n2).max(1e-12) {
        return coplanar_triangle_intersect(t1, t2, n1);
    }

    let axes = [dir.x.abs(), dir.y.abs(), dir.z.abs()];
    let axis = if axes[0] >= axes[1] && axes[0] >= axes[2] {
        0
    } else if axes[1] >= axes[2] {
        1
    } else {
        2
    };
    let proj = |p: Point3| -> f32 {
        match axis {
            0 => p.x,
            1 => p.y,
            _ => p.z,
        }
    };

    let (Some((min1, max1)), Some((min2, max2))) = (crossing_interval(t1, proj, du), crossing_interval(t2, proj, dv)) else {
        return false;
    };
    max1 >= min2 && max2 >= min1
}
//#endregion 🔖️TriangleTriangle

//#region 🔖️Queries
/// 🎯️ Does `mesh_a` (placed at `pose_a`) overlap `mesh_b` (placed at `pose_b`)? BVH-pruned:
/// `mesh_b`'s triangles are transformed into `mesh_a`'s local frame once, then each is tested
/// only against the candidates `mesh_a`'s BVH returns for that triangle's local AABB.
pub fn intersection_test(pose_a: Isometry3, mesh_a: &TriMesh, pose_b: Isometry3, mesh_b: &TriMesh) -> bool {
    let Some(bvh) = mesh_a.bvh.as_ref() else { return false };
    let relative = pose_a.inverse().compose(pose_b);
    let mut candidates = Vec::new();
    for triangle_b in &mesh_b.triangles {
        let b_local = triangle_b.map(|vertex| relative.transform_point(mesh_b.vertices[vertex as usize]));
        let query_aabb = Aabb3::of_triangle(b_local[0], b_local[1], b_local[2]);
        candidates.clear();
        bvh.collect_overlaps(query_aabb, &mut candidates);
        for &triangle_a_index in &candidates {
            if triangle_triangle_intersect(mesh_a.triangle_at(triangle_a_index), b_local) {
                return true;
            }
        }
    }
    false
}

fn solid_angle(a: Vector3, b: Vector3, c: Vector3) -> f32 {
    let (al, bl, cl) = (a.norm(), b.norm(), c.norm());
    if al < 1e-12 || bl < 1e-12 || cl < 1e-12 {
        return 0.0;
    }
    let numerator = a.dot(b.cross(c));
    let denominator = al * bl * cl + a.dot(b) * cl + b.dot(c) * al + c.dot(a) * bl;
    2.0 * numerator.atan2(denominator)
}

/// 📍️ Generalized winding number point-in-solid test — the framework replacement for
/// `parry3d::shape::SharedShape::contains_point`. `point` is given in `pose`'s parent frame.
/// Requires a closed, consistently outward-oriented mesh (see the module docstring); the sum of
/// per-triangle signed solid angles comes out near `±4π` inside and near `0` outside.
pub fn contains_point(pose: Isometry3, mesh: &TriMesh, point: Point3) -> bool {
    let local = pose.inverse().transform_point(point);
    let mut sum = 0.0f32;
    for triangle in &mesh.triangles {
        let [a, b, c] = triangle.map(|vertex| mesh.vertices[vertex as usize] - local);
        sum += solid_angle(a, b, c);
    }
    (sum / (4.0 * std::f32::consts::PI)).abs() > 0.5
}
//#endregion 🔖️Queries

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
