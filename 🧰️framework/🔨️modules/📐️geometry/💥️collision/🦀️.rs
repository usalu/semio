//! 💥️ Domain-neutral collision of double-precision triangle meshes: AABB, median-split BVH, exact triangle/triangle contact and distance, and the hard/clearance mesh clash.
//!
//! Units are the caller's (metres for the BIM domain); everything is pure and deterministic (no hashing, no randomness, ties break by index).
//! [`clash`] first collects every intersecting triangle pair through a simultaneous BVH descent and measures the *penetration extent* as the smallest axis extent of the AABB of all contact points (the minimum translation distance for axis-aligned boxes, a conservative proxy otherwise).
//! Without crossing surfaces it tests containment with a parity ray (meshes must be closed for that step), and last the minimum distance by branch-and-bound for the clearance check.
//! Compared against `parry3d` (triangle contact, triangle distance, box pairs) in `🧪️tests/💥️collision-oracles` and against the language-agnostic `🧫️fixtures/💥️collision/🔣️.json`.

use crate::mesh::TriMesh;
use crate::vector::{add3, cross3, dot3, length3, scale3, sub3, Xyz};

const EPS: f64 = 1e-9;
const PLANE_EPS: f64 = 1e-10;
const LEAF: usize = 4;
const MIN_NORMAL: f64 = 1e-18;
const RAY_SHIFT: Xyz = [0.0, 2.718281828e-7, 3.141592653e-7];

/// 📦️ Axis-aligned bounding box.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Aabb {
    pub min: Xyz,
    pub max: Xyz,
}

impl Aabb {
    /// 🎯️ Box around one point.
    pub fn point(p: Xyz) -> Self {
        Self { min: p, max: p }
    }

    /// 🧮️ Box around points; `None` when empty.
    pub fn from_points(points: &[Xyz]) -> Option<Self> {
        let (first, rest) = points.split_first()?;
        Some(rest.iter().fold(Self::point(*first), |b, p| b.including(*p)))
    }

    /// ➕️ Box grown to contain `p`.
    pub fn including(&self, p: Xyz) -> Self {
        Self { min: [self.min[0].min(p[0]), self.min[1].min(p[1]), self.min[2].min(p[2])], max: [self.max[0].max(p[0]), self.max[1].max(p[1]), self.max[2].max(p[2])] }
    }

    /// 🔗️ Smallest box containing both.
    pub fn union(&self, other: &Self) -> Self {
        self.including(other.min).including(other.max)
    }

    /// 🤝️ `true` when the boxes, each grown by `margin`, share a point.
    pub fn overlaps(&self, other: &Self, margin: f64) -> bool {
        (0..3).all(|k| self.min[k] <= other.max[k] + margin && other.min[k] <= self.max[k] + margin)
    }

    /// 📏️ Euclidean gap between the boxes; `0` when they overlap.
    pub fn distance(&self, other: &Self) -> f64 {
        (0..3).map(|k| (self.min[k] - other.max[k]).max(other.min[k] - self.max[k]).max(0.0).powi(2)).sum::<f64>().sqrt()
    }

    /// 🎯️ Centre of the box.
    pub fn centre(&self) -> Xyz {
        scale3(add3(self.min, self.max), 0.5)
    }

    /// 📐️ Edge lengths per axis.
    pub fn extents(&self) -> Xyz {
        sub3(self.max, self.min)
    }

    /// 🪶️ Smallest edge length.
    pub fn min_extent(&self) -> f64 {
        let e = self.extents();
        e[0].min(e[1]).min(e[2])
    }

    fn reach(&self) -> f64 {
        let e = self.extents();
        e[0] + e[1] + e[2]
    }
}

/// 📦️ Bounds of all vertices of a mesh; `None` when it has none.
pub fn mesh_aabb(mesh: &TriMesh) -> Option<Aabb> {
    mesh.bounds().map(|(min, max)| Aabb { min, max })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    aabb: Aabb,
    left: u32,
    right: u32,
    start: u32,
    count: u32,
}

impl Node {
    fn leaf(&self) -> bool {
        self.count > 0
    }
}

/// 🌳️ Median-split bounding volume hierarchy over the triangles of one [`TriMesh`] (leaves hold at most four triangles); build it once per mesh and reuse it for every pair.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bvh {
    nodes: Vec<Node>,
    order: Vec<u32>,
}

fn split(nodes: &mut Vec<Node>, order: &mut [u32], boxes: &[Aabb], centres: &[Xyz], start: usize, end: usize) -> u32 {
    let aabb = order[start..end].iter().fold(boxes[order[start] as usize], |b, &i| b.union(&boxes[i as usize]));
    let index = nodes.len();
    nodes.push(Node { aabb, left: 0, right: 0, start: start as u32, count: (end - start) as u32 });
    if end - start <= LEAF {
        return index as u32;
    }
    let spread = Aabb::from_points(&order[start..end].iter().map(|&i| centres[i as usize]).collect::<Vec<_>>()).unwrap().extents();
    let axis = if spread[0] >= spread[1] && spread[0] >= spread[2] { 0 } else if spread[1] >= spread[2] { 1 } else { 2 };
    order[start..end].sort_unstable_by(|&i, &j| centres[i as usize][axis].total_cmp(&centres[j as usize][axis]).then(i.cmp(&j)));
    let mid = (start + end) / 2;
    let left = split(nodes, order, boxes, centres, start, mid);
    let right = split(nodes, order, boxes, centres, mid, end);
    nodes[index].left = left;
    nodes[index].right = right;
    nodes[index].count = 0;
    index as u32
}

impl Bvh {
    /// 🏗️ Builds the hierarchy; an empty mesh gives an empty hierarchy.
    pub fn build(mesh: &TriMesh) -> Self {
        let boxes: Vec<Aabb> = (0..mesh.triangle_count()).map(|i| Aabb::from_points(&mesh.triangle(i)).unwrap()).collect();
        let centres: Vec<Xyz> = boxes.iter().map(Aabb::centre).collect();
        let mut order: Vec<u32> = (0..boxes.len() as u32).collect();
        let mut nodes = Vec::new();
        if !boxes.is_empty() {
            split(&mut nodes, &mut order, &boxes, &centres, 0, boxes.len());
        }
        Self { nodes, order }
    }

    /// 📦️ Bounds of all triangles; `None` when empty.
    pub fn bounds(&self) -> Option<Aabb> {
        self.nodes.first().map(|n| n.aabb)
    }

    /// 🔢️ Number of triangles indexed.
    pub fn triangle_count(&self) -> usize {
        self.order.len()
    }

    fn members(&self, node: &Node) -> &[u32] {
        &self.order[node.start as usize..(node.start + node.count) as usize]
    }
}

fn normal(t: [Xyz; 3]) -> Xyz {
    cross3(sub3(t[1], t[0]), sub3(t[2], t[0]))
}

fn degenerate(t: [Xyz; 3]) -> bool {
    !(length3(normal(t)) > MIN_NORMAL)
}

fn unit(v: Xyz) -> Xyz {
    scale3(v, 1.0 / length3(v))
}

fn snap(x: f64) -> f64 {
    if x.abs() <= PLANE_EPS {
        0.0
    } else {
        x
    }
}

fn lerp(a: Xyz, b: Xyz, s: f64) -> Xyz {
    add3(a, scale3(sub3(b, a), s))
}

fn plane_section(t: [Xyz; 3], d: [f64; 3]) -> Vec<Xyz> {
    let mut out: Vec<Xyz> = (0..3).filter(|&i| d[i] == 0.0).map(|i| t[i]).collect();
    for i in 0..3 {
        let j = (i + 1) % 3;
        if d[i] * d[j] < 0.0 {
            out.push(lerp(t[i], t[j], d[i] / (d[i] - d[j])));
        }
    }
    out
}

fn clip(polygon: &[Xyz], origin: Xyz, inward: Xyz) -> Vec<Xyz> {
    let side = |x: Xyz| dot3(inward, sub3(x, origin));
    let mut out = Vec::new();
    for i in 0..polygon.len() {
        let (c, d) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        let (sc, sd) = (side(c), side(d));
        let (c_in, d_in) = (sc >= -PLANE_EPS, sd >= -PLANE_EPS);
        if c_in {
            out.push(c);
        }
        if c_in != d_in {
            out.push(lerp(c, d, (sc / (sc - sd)).clamp(0.0, 1.0)));
        }
    }
    out
}

fn coplanar_overlap(a: [Xyz; 3], b: [Xyz; 3], n: Xyz) -> Option<Vec<Xyz>> {
    let b = if dot3(n, normal(b)) < 0.0 { [b[0], b[2], b[1]] } else { b };
    let mut polygon = a.to_vec();
    for i in 0..3 {
        let (p, q) = (b[i], b[(i + 1) % 3]);
        polygon = clip(&polygon, p, unit(cross3(n, sub3(q, p))));
        if polygon.is_empty() {
            return None;
        }
    }
    Some(polygon)
}

fn interval(points: &[Xyz], dir: Xyz) -> ((f64, Xyz), (f64, Xyz)) {
    let mut ends = points.iter().map(|&p| (dot3(dir, p), p)).collect::<Vec<_>>();
    ends.sort_by(|x, y| x.0.total_cmp(&y.0));
    (ends[0], *ends.last().unwrap())
}

/// 🎯️ All points of the contact between two triangles, or `None` when they do not touch or either is degenerate (twice the area at most `1e-18`).
///
/// Non-coplanar triangles touch in a segment (two points, equal for a point touch); coplanar overlapping triangles touch in a convex polygon whose vertices are returned (one to six points).
/// Planes closer than `1e-10` count as coincident.
pub fn triangle_contact(a: [Xyz; 3], b: [Xyz; 3]) -> Option<Vec<Xyz>> {
    if degenerate(a) || degenerate(b) {
        return None;
    }
    let (na, nb) = (unit(normal(a)), unit(normal(b)));
    let db = b.map(|p| snap(dot3(na, sub3(p, a[0]))));
    let da = a.map(|p| snap(dot3(nb, sub3(p, b[0]))));
    let apart = |d: [f64; 3]| d.iter().all(|&x| x > 0.0) || d.iter().all(|&x| x < 0.0);
    if apart(db) || apart(da) {
        return None;
    }
    if db.iter().all(|&x| x == 0.0) || da.iter().all(|&x| x == 0.0) {
        return coplanar_overlap(a, b, na);
    }
    let (sa, sb) = (plane_section(a, da), plane_section(b, db));
    let line = cross3(na, nb);
    let dir = if length3(line) > 1e-12 {
        unit(line)
    } else {
        let ends = interval(&sb, [1.0, 0.0, 0.0]);
        let span = sub3(ends.1 .1, ends.0 .1);
        if length3(span) > 0.0 {
            unit(span)
        } else {
            [1.0, 0.0, 0.0]
        }
    };
    let ((ta0, pa0), (ta1, pa1)) = interval(&sa, dir);
    let ((tb0, pb0), (tb1, pb1)) = interval(&sb, dir);
    let (lo, lo_p) = if ta0 >= tb0 { (ta0, pa0) } else { (tb0, pb0) };
    let (hi, hi_p) = if ta1 <= tb1 { (ta1, pa1) } else { (tb1, pb1) };
    if lo > hi + PLANE_EPS {
        return None;
    }
    if lo > hi {
        let mid = lerp(lo_p, hi_p, 0.5);
        return Some(vec![mid, mid]);
    }
    Some(vec![lo_p, hi_p])
}

fn farthest_pair(points: &[Xyz]) -> [Xyz; 2] {
    let mut best = (-1.0, [points[0], points[0]]);
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let d = sub3(points[i], points[j]);
            let d2 = dot3(d, d);
            if d2 > best.0 {
                best = (d2, [points[i], points[j]]);
            }
        }
    }
    best.1
}

/// 🔺️ Intersection of two triangles as a segment `[p, q]` (equal points for a point touch), or `None` when they are apart or either is degenerate.
///
/// Convention for coplanar overlapping triangles: the overlap polygon's extreme segment, i.e. its two mutually farthest vertices (first pair wins ties).
/// Use [`triangle_contact`] for the full polygon.
pub fn triangles_intersect(a: [Xyz; 3], b: [Xyz; 3]) -> Option<[Xyz; 2]> {
    triangle_contact(a, b).map(|points| farthest_pair(&points))
}

fn closest_on_segment(p: Xyz, a: Xyz, b: Xyz) -> Xyz {
    let ab = sub3(b, a);
    let l = dot3(ab, ab);
    if l <= 0.0 {
        return a;
    }
    lerp(a, b, (dot3(sub3(p, a), ab) / l).clamp(0.0, 1.0))
}

fn closest_on_triangle(p: Xyz, t: [Xyz; 3]) -> Xyz {
    let n = normal(t);
    let nn = dot3(n, n);
    if nn > 1e-36 {
        let q = sub3(p, scale3(n, dot3(sub3(p, t[0]), n) / nn));
        if (0..3).all(|i| dot3(cross3(sub3(t[(i + 1) % 3], t[i]), sub3(q, t[i])), n) >= 0.0) {
            return q;
        }
    }
    let mut best = (f64::INFINITY, t[0]);
    for i in 0..3 {
        let q = closest_on_segment(p, t[i], t[(i + 1) % 3]);
        let d = sub3(p, q);
        if dot3(d, d) < best.0 {
            best = (dot3(d, d), q);
        }
    }
    best.1
}

fn closest_between_segments(p1: Xyz, q1: Xyz, p2: Xyz, q2: Xyz) -> (Xyz, Xyz) {
    let (d1, d2, r) = (sub3(q1, p1), sub3(q2, p2), sub3(p1, p2));
    let (a, e, f) = (dot3(d1, d1), dot3(d2, d2), dot3(d2, r));
    let tiny = 1e-36;
    let (s, t);
    if a <= tiny && e <= tiny {
        return (p1, p2);
    }
    if a <= tiny {
        s = 0.0;
        t = (f / e).clamp(0.0, 1.0);
    } else {
        let c = dot3(d1, r);
        if e <= tiny {
            t = 0.0;
            s = (-c / a).clamp(0.0, 1.0);
        } else {
            let b = dot3(d1, d2);
            let denom = a * e - b * b;
            let s0 = if denom > 0.0 { ((b * f - c * e) / denom).clamp(0.0, 1.0) } else { 0.0 };
            let t0 = (b * s0 + f) / e;
            if t0 < 0.0 {
                t = 0.0;
                s = (-c / a).clamp(0.0, 1.0);
            } else if t0 > 1.0 {
                t = 1.0;
                s = ((b - c) / a).clamp(0.0, 1.0);
            } else {
                t = t0;
                s = s0;
            }
        }
    }
    (lerp(p1, q1, s), lerp(p2, q2, t))
}

/// 📏️ Minimum distance between two triangles and the closest point on each (`0` with the contact-segment midpoint twice when they intersect).
///
/// Disjoint triangles attain the minimum at a vertex-face or an edge-edge pair, so exactly those twelve candidates are compared; degenerate triangles degrade to segments and points.
pub fn triangle_distance(a: [Xyz; 3], b: [Xyz; 3]) -> (f64, Xyz, Xyz) {
    if let Some(s) = triangles_intersect(a, b) {
        let m = lerp(s[0], s[1], 0.5);
        return (0.0, m, m);
    }
    let mut best = (f64::INFINITY, a[0], b[0]);
    let mut offer = |p: Xyz, q: Xyz| {
        let d = sub3(p, q);
        if dot3(d, d) < best.0 {
            best = (dot3(d, d), p, q);
        }
    };
    for p in a {
        offer(p, closest_on_triangle(p, b));
    }
    for q in b {
        offer(closest_on_triangle(q, a), q);
    }
    for i in 0..3 {
        for j in 0..3 {
            let (p, q) = closest_between_segments(a[i], a[(i + 1) % 3], b[j], b[(j + 1) % 3]);
            offer(p, q);
        }
    }
    (best.0.sqrt(), best.1, best.2)
}

/// 🏷️ Kind of a mesh clash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClashKind {
    Hard,
    Clearance,
}

/// 💥️ One clash between two meshes: `distance` is `-(penetration extent)` for [`ClashKind::Hard`] and the positive gap for [`ClashKind::Clearance`]; `pairs` counts intersecting triangle pairs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeshClash {
    pub kind: ClashKind,
    pub distance: f64,
    pub point: Xyz,
    pub bounds: Aabb,
    pub pairs: usize,
}

struct Cancelled;

fn partner(a: &Node, b: &Node) -> bool {
    !a.leaf() && (b.leaf() || a.aabb.reach() >= b.aabb.reach())
}

fn crossings(a: &Bvh, am: &TriMesh, b: &Bvh, bm: &TriMesh, cancel: &dyn Fn() -> bool) -> Result<(usize, Option<Aabb>), Cancelled> {
    let mut stack = vec![(0u32, 0u32)];
    let (mut pairs, mut bounds) = (0, None::<Aabb>);
    while let Some((i, j)) = stack.pop() {
        if cancel() {
            return Err(Cancelled);
        }
        let (na, nb) = (&a.nodes[i as usize], &b.nodes[j as usize]);
        if !na.aabb.overlaps(&nb.aabb, EPS) {
            continue;
        }
        if na.leaf() && nb.leaf() {
            for &x in a.members(na) {
                for &y in b.members(nb) {
                    if let Some(points) = triangle_contact(am.triangle(x as usize), bm.triangle(y as usize)) {
                        pairs += 1;
                        for p in points {
                            bounds = Some(bounds.map_or(Aabb::point(p), |c| c.including(p)));
                        }
                    }
                }
            }
        } else if partner(na, nb) {
            stack.push((na.right, j));
            stack.push((na.left, j));
        } else {
            stack.push((i, nb.right));
            stack.push((i, nb.left));
        }
    }
    Ok((pairs, bounds))
}

fn inside(p: Xyz, mesh: &TriMesh, bvh: &Bvh, cancel: &dyn Fn() -> bool) -> Result<bool, Cancelled> {
    let o = add3(p, RAY_SHIFT);
    let orient = |u: Xyz, v: Xyz| (v[1] - u[1]) * (o[2] - u[2]) - (v[2] - u[2]) * (o[1] - u[1]);
    let mut stack = if bvh.nodes.is_empty() { vec![] } else { vec![0u32] };
    let mut hits = 0usize;
    while let Some(i) = stack.pop() {
        if cancel() {
            return Err(Cancelled);
        }
        let node = &bvh.nodes[i as usize];
        if node.aabb.max[0] < p[0] || o[1] < node.aabb.min[1] || o[1] > node.aabb.max[1] || o[2] < node.aabb.min[2] || o[2] > node.aabb.max[2] {
            continue;
        }
        if !node.leaf() {
            stack.push(node.right);
            stack.push(node.left);
            continue;
        }
        for &t in bvh.members(node) {
            let [a, b, c] = mesh.triangle(t as usize);
            let (w0, w1, w2) = (orient(b, c), orient(c, a), orient(a, b));
            let area = w0 + w1 + w2;
            let same = (w0 > 0.0 && w1 > 0.0 && w2 > 0.0) || (w0 < 0.0 && w1 < 0.0 && w2 < 0.0);
            if same && (w0 * a[0] + w1 * b[0] + w2 * c[0]) / area > p[0] {
                hits += 1;
            }
        }
    }
    Ok(hits % 2 == 1)
}

fn embedded(mesh: &TriMesh, other: &TriMesh, other_bvh: &Bvh, mut bounds: Aabb, cancel: &dyn Fn() -> bool) -> Result<Aabb, Cancelled> {
    let Some(frame) = other_bvh.bounds() else { return Ok(bounds) };
    for &p in &mesh.positions {
        let spot = Aabb::point(p);
        if !bounds.overlaps(&spot, 0.0) && frame.overlaps(&spot, 0.0) && inside(p, other, other_bvh, cancel)? {
            bounds = bounds.including(p);
        }
    }
    Ok(bounds)
}

fn containment(a: &Bvh, am: &TriMesh, b: &Bvh, bm: &TriMesh, cancel: &dyn Fn() -> bool) -> Result<Option<MeshClash>, Cancelled> {
    for (inner, inner_bvh, outer, outer_bvh) in [(am, a, bm, b), (bm, b, am, a)] {
        let Some(bounds) = inner_bvh.bounds() else { continue };
        let Some(outer_bounds) = outer_bvh.bounds() else { continue };
        let probe = inner.triangle(inner_bvh.order[0] as usize)[0];
        if outer_bounds.overlaps(&Aabb::point(probe), 0.0) && inside(probe, outer, outer_bvh, cancel)? {
            return Ok(Some(MeshClash { kind: ClashKind::Hard, distance: -bounds.min_extent(), point: bounds.centre(), bounds, pairs: 0 }));
        }
    }
    Ok(None)
}

fn nearest(a: &Bvh, am: &TriMesh, b: &Bvh, bm: &TriMesh, limit: f64, cancel: &dyn Fn() -> bool) -> Result<Option<(f64, Xyz, Xyz)>, Cancelled> {
    let mut stack = vec![(0u32, 0u32, a.nodes[0].aabb.distance(&b.nodes[0].aabb))];
    let mut best = limit;
    let mut found = None;
    while let Some((i, j, gap)) = stack.pop() {
        if cancel() {
            return Err(Cancelled);
        }
        if gap >= best {
            continue;
        }
        let (na, nb) = (&a.nodes[i as usize], &b.nodes[j as usize]);
        if na.leaf() && nb.leaf() {
            for &x in a.members(na) {
                for &y in b.members(nb) {
                    let (ta, tb) = (am.triangle(x as usize), bm.triangle(y as usize));
                    if Aabb::from_points(&ta).unwrap().distance(&Aabb::from_points(&tb).unwrap()) >= best {
                        continue;
                    }
                    let (d, p, q) = triangle_distance(ta, tb);
                    if d < best {
                        best = d;
                        found = Some((d, p, q));
                    }
                }
            }
            continue;
        }
        let pairs = if partner(na, nb) { [(na.left, j), (na.right, j)] } else { [(i, nb.left), (i, nb.right)] };
        let mut scored = pairs.map(|(x, y)| (x, y, a.nodes[x as usize].aabb.distance(&b.nodes[y as usize].aabb)));
        if scored[0].2 < scored[1].2 {
            scored.swap(0, 1);
        }
        stack.extend(scored);
    }
    Ok(found)
}

/// 💥️ Clash between two meshes with their prebuilt hierarchies; `None` when there is none or when `cancel` returned `true`.
///
/// 1. Every intersecting triangle pair is collected. The intersection set is the contact points of all pairs plus every vertex of either mesh that lies strictly inside the other (so a wall sunk into a slab keeps its embedded end; without it the crossing loop alone is flat). The AABB of that set gives the penetration extent (its smallest axis extent). Hard iff `pairs > 0` and extent `> max(tolerance, 1e-9)` with `distance = -extent`, `point` the box centre and `bounds` that box; touching surfaces within the tolerance yield `None`, also for the clearance check (their distance is `0`).
/// 2. Without crossing surfaces a +x parity ray from one vertex tests whether either closed mesh lies inside the other; a contained mesh is Hard with its own smallest AABB extent, centre and bounds.
/// 3. When `clearance > 0`, the branch-and-bound minimum distance `d` with `0 < d < clearance` is a [`ClashKind::Clearance`] with `distance = d`, `point` the midpoint of the closest points and `bounds` their AABB; `d >= clearance` is no clash.
///
/// `cancel` is polled once per BVH node visit; a cancelled run returns `None`, which a caller cannot tell from "no clash", so callers must check their own cancel flag afterwards.
pub fn clash(a: &Bvh, a_mesh: &TriMesh, b: &Bvh, b_mesh: &TriMesh, tolerance: f64, clearance: f64, cancel: &dyn Fn() -> bool) -> Option<MeshClash> {
    if a.nodes.is_empty() || b.nodes.is_empty() {
        return None;
    }
    let run = || -> Result<Option<MeshClash>, Cancelled> {
        let (pairs, contact) = crossings(a, a_mesh, b, b_mesh, cancel)?;
        if let Some(bounds) = contact {
            let bounds = embedded(b_mesh, a_mesh, a, embedded(a_mesh, b_mesh, b, bounds, cancel)?, cancel)?;
            let extent = bounds.min_extent();
            let hard = extent > tolerance.max(EPS);
            return Ok(hard.then(|| MeshClash { kind: ClashKind::Hard, distance: -extent, point: bounds.centre(), bounds, pairs }));
        }
        if let Some(inner) = containment(a, a_mesh, b, b_mesh, cancel)? {
            return Ok(Some(inner));
        }
        if clearance > 0.0 {
            if let Some((d, p, q)) = nearest(a, a_mesh, b, b_mesh, clearance, cancel)? {
                if d > 0.0 {
                    return Ok(Some(MeshClash { kind: ClashKind::Clearance, distance: d, point: lerp(p, q, 0.5), bounds: Aabb::point(p).including(q), pairs: 0 }));
                }
            }
        }
        Ok(None)
    };
    run().ok().flatten()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
