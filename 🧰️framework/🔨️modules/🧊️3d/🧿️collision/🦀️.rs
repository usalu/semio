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
//!
//! 🕳️ [`distance_to_surface`] and [`clip_behind_triangle`] are the two primitives a penetration-DEPTH measure is built from
//! (how far one surface dives into another solid): the nearest-surface distance of a point, BVH best-first, and the part of a
//! triangle that lies behind another triangle's outward face inside its prism.

use crate::rigid::{Isometry3, Point3, Vector3};

//#region 🔖️Aabb3
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`; `Point3` (`🌀️rigid`) already covered.
#[derive(Clone, Copy, Debug, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
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

    /// 📏️ Squared distance from `point` to this box (0 inside).
    fn distance_squared(self, point: Point3) -> f32 {
        let gap = |value: f32, min: f32, max: f32| if value < min { min - value } else if value > max { value - max } else { 0.0 };
        let (dx, dy, dz) = (gap(point.x, self.min.x, self.max.x), gap(point.y, self.min.y, self.max.y), gap(point.z, self.min.z, self.max.z));
        dx * dx + dy * dy + dz * dz
    }

    /// 🔦️ Slab test of the ray `origin + t·direction`, t ≥ 0, given the component-wise inverse direction.
    fn ray_hits(self, origin: Point3, inverse: Vector3) -> bool {
        let slab = |origin: f32, inverse: f32, min: f32, max: f32| {
            let (near, far) = ((min - origin) * inverse, (max - origin) * inverse);
            (near.min(far), near.max(far))
        };
        let (x0, x1) = slab(origin.x, inverse.x, self.min.x, self.max.x);
        let (y0, y1) = slab(origin.y, inverse.y, self.min.y, self.max.y);
        let (z0, z1) = slab(origin.z, inverse.z, self.min.z, self.max.z);
        let (enter, exit) = (x0.max(y0).max(z0), x1.min(y1).min(z1));
        exit >= enter.max(0.0)
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

    /// 🎯️ Smallest distance from `point` to any triangle under this node, pruning subtrees whose bounds are no closer
    /// than the best distance found so far (`best` is a squared distance and only ever shrinks).
    fn nearest(&self, mesh: &TriMesh, point: Point3, best: &mut f32) {
        if self.aabb().distance_squared(point) >= *best {
            return;
        }
        match self {
            Self::Leaf { triangle, .. } => {
                let [a, b, c] = mesh.triangle_at(*triangle);
                let closest = closest_point_on_triangle(point, a, b, c);
                let offset = point - closest;
                *best = best.min(offset.dot(offset));
            }
            Self::Branch { left, right, .. } => {
                let (near, far) = if left.aabb().distance_squared(point) <= right.aabb().distance_squared(point) { (left, right) } else { (right, left) };
                near.nearest(mesh, point, best);
                far.nearest(mesh, point, best);
            }
        }
    }

    /// 🔦️ Counts the triangles under this node the ray `origin + t·direction` (t > 0) crosses; `None` as soon as one
    /// crossing grazes an edge, a vertex or the origin, where parity is not decided by the count.
    fn ray_crossings(&self, mesh: &TriMesh, origin: Point3, direction: Vector3, inverse: Vector3, count: &mut u32) -> Option<()> {
        if !self.aabb().ray_hits(origin, inverse) {
            return Some(());
        }
        match self {
            Self::Leaf { triangle, .. } => {
                let [a, b, c] = mesh.triangle_at(*triangle);
                match ray_triangle(origin, direction, a, b, c) {
                    RayTriangle::Miss => {}
                    RayTriangle::Cross => *count += 1,
                    RayTriangle::Graze => return None,
                }
                Some(())
            }
            Self::Branch { left, right, .. } => {
                left.ray_crossings(mesh, origin, direction, inverse, count)?;
                right.ray_crossings(mesh, origin, direction, inverse, count)
            }
        }
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
    /// 🧭️ `true` when the triangles wind INWARD (negative enclosed signed volume): [`Self::world_triangle_outward`] then
    /// reverses them, so outward-facing queries hold for either winding an asset ships with.
    inward: bool,
}

/// 🧭️ Rewinds `triangles` so every pair sharing an edge traverses it in opposite directions — one consistent orientation
/// per connected component — by a breadth-first walk over shared edges from each component's first triangle. The
/// winding-number test ([`contains_point`]) and the outward sign both need it, and asset indices do not always keep it:
/// a tetrahedron with one face wound the other way read points outside its base as inside. Non-manifold edges (more than
/// two triangles) keep their triangles' given winding.
fn orient_consistently(triangles: &mut [[u32; 3]]) {
    let mut by_edge: std::collections::HashMap<(u32, u32), Vec<usize>> = std::collections::HashMap::with_capacity(triangles.len() * 3);
    for (index, triangle) in triangles.iter().enumerate() {
        for (from, to) in [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])] {
            by_edge.entry((from.min(to), from.max(to))).or_default().push(index);
        }
    }
    let directed = |triangle: [u32; 3], from: u32, to: u32| [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])].contains(&(from, to));
    let mut visited = vec![false; triangles.len()];
    let mut queue = std::collections::VecDeque::new();
    for seed in 0..triangles.len() {
        if visited[seed] {
            continue;
        }
        visited[seed] = true;
        queue.push_back(seed);
        while let Some(current) = queue.pop_front() {
            let triangle = triangles[current];
            for (from, to) in [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])] {
                let Some(neighbours) = by_edge.get(&(from.min(to), from.max(to))).filter(|neighbours| neighbours.len() == 2) else { continue };
                let neighbour = if neighbours[0] == current { neighbours[1] } else { neighbours[0] };
                if visited[neighbour] {
                    continue;
                }
                // A consistently wound neighbour traverses the shared edge the other way round.
                if directed(triangles[neighbour], from, to) {
                    let [a, b, c] = triangles[neighbour];
                    triangles[neighbour] = [a, c, b];
                }
                visited[neighbour] = true;
                queue.push_back(neighbour);
            }
        }
    }
}

impl TriMesh {
    pub fn new(vertices: Vec<Point3>, mut triangles: Vec<[u32; 3]>) -> Self {
        orient_consistently(&mut triangles);
        let leaves: Vec<(Aabb3, u32)> = triangles
            .iter()
            .enumerate()
            .map(|(index, triangle)| {
                let [a, b, c] = triangle.map(|vertex| vertices[vertex as usize]);
                (Aabb3::of_triangle(a, b, c), index as u32)
            })
            .collect();
        let bvh = (!leaves.is_empty()).then(|| BvhNode::build(leaves));
        let signed_volume: f32 = triangles.iter().map(|triangle| {
            let [a, b, c] = triangle.map(|vertex| vertices[vertex as usize].coords());
            a.dot(b.cross(c))
        }).sum();
        Self { vertices, triangles, bvh, inward: signed_volume < 0.0 }
    }

    fn triangle_at(&self, index: u32) -> [Point3; 3] {
        self.triangles[index as usize].map(|vertex| self.vertices[vertex as usize])
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// 📍️ Vertex `index` placed at `pose` (in `pose`'s parent frame).
    pub fn world_vertex(&self, pose: Isometry3, index: usize) -> Point3 {
        pose.transform_point(self.vertices[index])
    }

    /// 🔺️ Triangle `index` placed at `pose` (in `pose`'s parent frame).
    pub fn world_triangle(&self, pose: Isometry3, index: usize) -> [Point3; 3] {
        self.triangle_at(index as u32).map(|vertex| pose.transform_point(vertex))
    }

    /// 🔺️ Triangle `index` placed at `pose`, wound counter-clockwise seen from OUTSIDE the solid whatever winding the mesh
    /// was built with — the orientation [`clip_behind_triangle`] and an inward offset along the face normal expect.
    pub fn world_triangle_outward(&self, pose: Isometry3, index: usize) -> [Point3; 3] {
        let [a, b, c] = self.world_triangle(pose, index);
        if self.inward { [a, c, b] } else { [a, b, c] }
    }

    /// 🎯️ Indices of the triangles whose bounds meet `triangle` (given in `pose`'s parent frame) grown by `margin` —
    /// the BVH candidates for a triangle-pair query. `out` is cleared first.
    pub fn triangles_near(&self, pose: Isometry3, triangle: [Point3; 3], margin: f32, out: &mut Vec<u32>) {
        out.clear();
        let Some(bvh) = self.bvh.as_ref() else { return };
        let inverse = pose.inverse();
        let [a, b, c] = triangle.map(|vertex| inverse.transform_point(vertex));
        let bounds = Aabb3::of_triangle(a, b, c);
        let grow = Vector3::new(margin, margin, margin);
        bvh.collect_overlaps(Aabb3 { min: Point3::from_coords(bounds.min.coords() - grow), max: bounds.max + grow }, out);
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

//#region 🔖️Depth
/// 📍️ The point of triangle `(a, b, c)` closest to `point` (Ericson, *Real-Time Collision Detection* §5.1.5); degenerate
/// triangles fall back to their edges through the same Voronoi-region tests.
fn closest_point_on_triangle(point: Point3, a: Point3, b: Point3, c: Point3) -> Point3 {
    let (ab, ac, ap) = (b - a, c - a, point - a);
    let (d1, d2) = (ab.dot(ap), ac.dot(ap));
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }
    let bp = point - b;
    let (d3, d4) = (ab.dot(bp), ac.dot(bp));
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let denominator = d1 - d3;
        return if denominator.abs() < f32::MIN_POSITIVE { a } else { a + ab * (d1 / denominator) };
    }
    let cp = point - c;
    let (d5, d6) = (ab.dot(cp), ac.dot(cp));
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let denominator = d2 - d6;
        return if denominator.abs() < f32::MIN_POSITIVE { a } else { a + ac * (d2 / denominator) };
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let denominator = (d4 - d3) + (d5 - d6);
        return if denominator.abs() < f32::MIN_POSITIVE { b } else { b + (c - b) * ((d4 - d3) / denominator) };
    }
    let total = va + vb + vc;
    if total.abs() < f32::MIN_POSITIVE {
        return a;
    }
    a + ab * (vb / total) + ac * (vc / total)
}

/// 📏️ Distance from `point` (in `pose`'s parent frame) to the nearest point on `mesh`'s surface, whether `point` lies
/// inside or outside — BVH best-first with bound pruning. `f32::INFINITY` for a mesh without triangles.
pub fn distance_to_surface(pose: Isometry3, mesh: &TriMesh, point: Point3) -> f32 {
    let Some(bvh) = mesh.bvh.as_ref() else { return f32::INFINITY };
    let local = pose.inverse().transform_point(point);
    let mut best = f32::INFINITY;
    bvh.nearest(mesh, local, &mut best);
    best.sqrt()
}

/// ✂️ A convex polygon of at most 8 vertices, the result of clipping one triangle by four half-spaces.
#[derive(Clone, Copy, Debug)]
pub struct ClippedPolygon {
    vertices: [Point3; 8],
    len: usize,
}

impl ClippedPolygon {
    pub fn points(&self) -> &[Point3] {
        &self.vertices[..self.len]
    }
}

/// ✂️ The part of triangle `a` that lies BEHIND triangle `b`'s outward face (counter-clockwise winding seen from
/// outside) and inside `b`'s infinite prism — every point of `a` whose projection onto `b`'s plane falls in `b` and that
/// sits on `b`'s inner side. Its vertices are where `a`'s surface dives deepest under that face, so a penetration-depth
/// measure only has to evaluate them. Empty when `b` is degenerate or `a` stays in front of / beside it.
pub fn clip_behind_triangle(a: [Point3; 3], b: [Point3; 3]) -> ClippedPolygon {
    let empty = ClippedPolygon { vertices: [Point3::new(0.0, 0.0, 0.0); 8], len: 0 };
    let normal = (b[1] - b[0]).cross(b[2] - b[0]);
    if normal.dot(normal) < TRI_EPS * TRI_EPS {
        return empty;
    }
    let mut current = ClippedPolygon { vertices: [a[0]; 8], len: 3 };
    current.vertices[1] = a[1];
    current.vertices[2] = a[2];
    // Each plane keeps `direction · (p - origin) >= 0`.
    let planes = [(b[0], -normal), (b[0], normal.cross(b[1] - b[0])), (b[1], normal.cross(b[2] - b[1])), (b[2], normal.cross(b[0] - b[2]))];
    for (origin, direction) in planes {
        let mut next = ClippedPolygon { vertices: [Point3::new(0.0, 0.0, 0.0); 8], len: 0 };
        let points = current.points();
        for index in 0..points.len() {
            let (from, to) = (points[index], points[(index + 1) % points.len()]);
            let (side_from, side_to) = (direction.dot(from - origin), direction.dot(to - origin));
            if side_from >= 0.0 && next.len < 8 {
                next.vertices[next.len] = from;
                next.len += 1;
            }
            if (side_from >= 0.0) != (side_to >= 0.0) && next.len < 8 {
                let t = side_from / (side_from - side_to);
                next.vertices[next.len] = from + (to - from) * t;
                next.len += 1;
            }
        }
        if next.len == 0 {
            return empty;
        }
        current = next;
    }
    current
}
//#endregion 🔖️Depth

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

enum RayTriangle {
    Miss,
    Cross,
    Graze,
}

/// 🔦️ Möller–Trumbore ray/triangle crossing for t > 0, relative to the triangle's own scale: a hit whose barycentrics come
/// within `RAY_GRAZE` of an edge, a ray running in the triangle's plane over it, or a crossing at the origin grazes.
fn ray_triangle(origin: Point3, direction: Vector3, a: Point3, b: Point3, c: Point3) -> RayTriangle {
    const RAY_GRAZE: f32 = 1e-4;
    let (ab, ac) = (b - a, c - a);
    let p = direction.cross(ac);
    let determinant = ab.dot(p);
    let (scale, extent) = (ab.norm() * ac.norm(), ab.norm().max(ac.norm()));
    if scale <= 0.0 {
        return RayTriangle::Miss;
    }
    if determinant.abs() <= RAY_GRAZE * scale {
        // 🪞️ Parallel to the plane: only a ray running inside the plane can touch the triangle, and that is ambiguous.
        let normal = ab.cross(ac);
        let height = normal.dot(origin - a).abs() / normal.norm().max(f32::MIN_POSITIVE);
        return if height <= RAY_GRAZE * extent { RayTriangle::Graze } else { RayTriangle::Miss };
    }
    let inverse = 1.0 / determinant;
    let offset = origin - a;
    let u = offset.dot(p) * inverse;
    if u < -RAY_GRAZE || u > 1.0 + RAY_GRAZE {
        return RayTriangle::Miss;
    }
    let q = offset.cross(ab);
    let v = direction.dot(q) * inverse;
    if v < -RAY_GRAZE || u + v > 1.0 + RAY_GRAZE {
        return RayTriangle::Miss;
    }
    let t = ac.dot(q) * inverse;
    if t < -RAY_GRAZE * extent {
        return RayTriangle::Miss;
    }
    if t <= RAY_GRAZE * extent || u <= RAY_GRAZE || v <= RAY_GRAZE || u + v >= 1.0 - RAY_GRAZE {
        return RayTriangle::Graze;
    }
    RayTriangle::Cross
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

/// 🔦️ Directions [`contains_point_fast`] casts, skewed off every axis and diagonal so axis-aligned asset geometry never
/// lines up with them.
const CONTAINMENT_RAYS: [[f32; 3]; 3] = [[0.5773, 0.3016, 0.7588], [-0.4271, 0.8193, -0.3826], [0.2129, -0.6644, -0.7164]];

/// 📍️ The same point-in-solid answer as [`contains_point`] for a closed, consistently wound mesh, in O(log triangles): the
/// parity of BVH ray crossings. A ray that grazes an edge, a vertex or its own origin decides nothing and the next direction
/// is cast; when every direction grazes the exact winding number answers.
pub fn contains_point_fast(pose: Isometry3, mesh: &TriMesh, point: Point3) -> bool {
    let Some(bvh) = mesh.bvh.as_ref() else { return false };
    let local = pose.inverse().transform_point(point);
    for [x, y, z] in CONTAINMENT_RAYS {
        let direction = Vector3::new(x, y, z);
        let inverse = Vector3::new(1.0 / x, 1.0 / y, 1.0 / z);
        let mut crossings = 0;
        if bvh.ray_crossings(mesh, local, direction, inverse, &mut crossings).is_some() {
            return crossings % 2 == 1;
        }
    }
    contains_point(pose, mesh, point)
}
//#endregion 🔖️Queries

#[cfg(test)]
#[path = "🧪️tests/🧿️collision/🦀️.rs"]
mod tests;
