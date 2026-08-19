//! 🗺️ Spatial acceleration structures: const-generic k-d trees, hashed voxel grids and point octrees for nearest-neighbour and range queries.
//! Moved wholesale from `🧮️math/🗺️spatial` — 📸️remodel is its sole repo-wide consumer (verified: symbol-level grep of every exported type/fn across the whole tree outside math and remodel returned zero hits), per `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` wave M3d.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// #region 🔖️KdTree
const KD_NONE: u32 = u32::MAX;

async fn dist_sq<const D: usize>(a: &[f64; D], b: &[f64; D]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y) * (x - y)).sum()
}

#[derive(Clone, Copy, Debug)]
struct KdNode<const D: usize> {
    point: [f64; D],
    id: u32,
    axis: usize,
    left: u32,
    right: u32,
}

#[derive(Clone, Copy, Debug)]
struct KdHeapEntry {
    dist: f64,
    id: u32,
}

impl PartialEq for KdHeapEntry {
    async fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for KdHeapEntry {}

impl PartialOrd for KdHeapEntry {
    async fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KdHeapEntry {
    async fn cmp(&self, other: &Self) -> Ordering {
        self.dist.total_cmp(&other.dist).then(self.id.cmp(&other.id))
    }
}

/// 🌲️ Balanced k-d tree over `[f64; D]` points with `u32` payload indices, built by iterative median splits (quickselect on `axis = depth % D`) into a flat node vector with children linked by index; all queries run iteratively with an explicit stack and bounding-plane pruning. <https://en.wikipedia.org/wiki/K-d_tree>
#[derive(Clone, Debug)]
pub struct KdTree<const D: usize> {
    nodes: Vec<KdNode<D>>,
}

impl<const D: usize> KdTree<D> {
    /// 🏗️ Builds a balanced tree from point copies, remembering each point's original slice index as its payload; empty input yields an empty tree.
    pub async fn build(points: &[[f64; D]]) -> Self {
        if D == 0 || points.is_empty() {
            return Self { nodes: Vec::new() };
        }
        let mut items: Vec<([f64; D], u32)> = points.iter().enumerate().map(|(i, &p)| (p, i as u32)).collect();
        let mut nodes: Vec<KdNode<D>> = Vec::with_capacity(points.len());
        let mut stack: Vec<(usize, usize, usize, u32, bool)> = vec![(0, items.len(), 0, KD_NONE, false)];
        while let Some((lo, hi, depth, parent, is_left)) = stack.pop() {
            if lo >= hi {
                continue;
            }
            let axis = depth % D;
            let mid = lo + (hi - lo) / 2;
            items[lo..hi].select_nth_unstable_by(mid - lo, |a, b| a.0[axis].total_cmp(&b.0[axis]).then(a.1.cmp(&b.1)));
            let idx = nodes.len() as u32;
            let (point, id) = items[mid];
            nodes.push(KdNode { point, id, axis, left: KD_NONE, right: KD_NONE });
            if parent != KD_NONE {
                let slot = &mut nodes[parent as usize];
                if is_left {
                    slot.left = idx;
                } else {
                    slot.right = idx;
                }
            }
            stack.push((lo, mid, depth + 1, idx, true));
            stack.push((mid + 1, hi, depth + 1, idx, false));
        }
        Self { nodes }
    }

    /// 🎯️ Nearest neighbour of `q` as `(payload index, squared distance)`; ties on distance resolve to the smallest index. `None` on an empty tree.
    pub async fn nearest(&self, q: &[f64; D]) -> Option<(u32, f64)> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut best_id = self.nodes[0].id;
        let mut best_d = f64::INFINITY;
        let mut stack: Vec<(u32, f64)> = vec![(0, 0.0)];
        while let Some((idx, bound)) = stack.pop() {
            if bound > best_d {
                continue;
            }
            let node = &self.nodes[idx as usize];
            let d = dist_sq(&node.point, q);
            if d < best_d || (d == best_d && node.id < best_id) {
                best_d = d;
                best_id = node.id;
            }
            let delta = q[node.axis] - node.point[node.axis];
            let (near, far) = if delta < 0.0 { (node.left, node.right) } else { (node.right, node.left) };
            if far != KD_NONE {
                stack.push((far, delta * delta));
            }
            if near != KD_NONE {
                stack.push((near, 0.0));
            }
        }
        Some((best_id, best_d))
    }

    /// 🎯️ The `k` nearest neighbours of `q`, ascending by `(squared distance, payload index)`, found with a bounded max-heap; returns fewer than `k` entries only when the tree holds fewer points.
    pub async fn k_nearest(&self, q: &[f64; D], k: usize) -> Vec<(u32, f64)> {
        if k == 0 || self.nodes.is_empty() {
            return Vec::new();
        }
        let mut heap: BinaryHeap<KdHeapEntry> = BinaryHeap::with_capacity(k + 1);
        let mut stack: Vec<(u32, f64)> = vec![(0, 0.0)];
        while let Some((idx, bound)) = stack.pop() {
            let worst = if heap.len() == k { heap.peek().map_or(f64::INFINITY, |e| e.dist) } else { f64::INFINITY };
            if bound > worst {
                continue;
            }
            let node = &self.nodes[idx as usize];
            let entry = KdHeapEntry { dist: dist_sq(&node.point, q), id: node.id };
            if heap.len() < k {
                heap.push(entry);
            } else if let Some(&top) = heap.peek() {
                if entry.cmp(&top) == Ordering::Less {
                    heap.pop();
                    heap.push(entry);
                }
            }
            let delta = q[node.axis] - node.point[node.axis];
            let (near, far) = if delta < 0.0 { (node.left, node.right) } else { (node.right, node.left) };
            if far != KD_NONE {
                stack.push((far, delta * delta));
            }
            if near != KD_NONE {
                stack.push((near, 0.0));
            }
        }
        let mut out: Vec<(u32, f64)> = heap.into_iter().map(|e| (e.id, e.dist)).collect();
        out.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
        out
    }

    /// ⭕️ All points within euclidean distance `r` of `q` as `(payload index, squared distance)`, sorted by payload index for determinism; negative `r` yields no hits.
    pub async fn radius(&self, q: &[f64; D], r: f64) -> Vec<(u32, f64)> {
        let mut out = Vec::new();
        if self.nodes.is_empty() || r < 0.0 {
            return out;
        }
        let r_sq = r * r;
        let mut stack: Vec<u32> = vec![0];
        while let Some(idx) = stack.pop() {
            let node = &self.nodes[idx as usize];
            let d = dist_sq(&node.point, q);
            if d <= r_sq {
                out.push((node.id, d));
            }
            let delta = q[node.axis] - node.point[node.axis];
            let (near, far) = if delta < 0.0 { (node.left, node.right) } else { (node.right, node.left) };
            if far != KD_NONE && delta * delta <= r_sq {
                stack.push(far);
            }
            if near != KD_NONE {
                stack.push(near);
            }
        }
        out.sort_unstable_by_key(|e| e.0);
        out
    }

    /// 📦️ Calls `f` with the payload index of every point inside the closed axis-aligned box `[min, max]`, pruning subtrees by the splitting plane.
    pub async fn for_each_in_aabb(&self, min: &[f64; D], max: &[f64; D], mut f: impl FnMut(u32)) {
        if self.nodes.is_empty() {
            return;
        }
        let mut stack: Vec<u32> = vec![0];
        while let Some(idx) = stack.pop() {
            let node = &self.nodes[idx as usize];
            if node.point.iter().zip(min.iter().zip(max.iter())).all(|(p, (lo, hi))| (*lo..=*hi).contains(p)) {
                f(node.id);
            }
            let axis = node.axis;
            if node.left != KD_NONE && node.point[axis] >= min[axis] {
                stack.push(node.left);
            }
            if node.right != KD_NONE && node.point[axis] <= max[axis] {
                stack.push(node.right);
            }
        }
    }
}
// #endregion 🔖️KdTree

// #region 🔖️VoxelGrid
/// 🧊️ Uniform hashed voxel grid over 3D points: buckets `u32` ids by integer cell, with 27-cell neighbourhood gathering for broad-phase proximity. <https://en.wikipedia.org/wiki/Bin_(computational_geometry)>
#[derive(Clone, Debug)]
pub struct VoxelGrid3 {
    cell: f64,
    map: HashMap<(i32, i32, i32), Vec<u32>>,
}

impl VoxelGrid3 {
    /// 🧊️ Creates an empty grid with the given positive cell edge length.
    pub async fn new(cell: f64) -> Self {
        assert!(cell > 0.0, "voxel cell size must be positive");
        Self { cell, map: HashMap::new() }
    }

    /// 🧭️ Integer cell containing `p`, via floor division that stays correct for negative coordinates.
    pub async fn cell_of(&self, p: [f64; 3]) -> (i32, i32, i32) {
        ((p[0] / self.cell).floor() as i32, (p[1] / self.cell).floor() as i32, (p[2] / self.cell).floor() as i32)
    }

    /// ➕️ Buckets `id` into the cell containing `p`.
    pub async fn insert(&mut self, p: [f64; 3], id: u32) {
        let key = self.cell_of(p);
        self.map.entry(key).or_default().push(id);
    }

    /// 🔍️ All ids bucketed in the 27 cells surrounding (and including) the cell of `p`, sorted ascending for determinism.
    pub async fn neighbors27(&self, p: [f64; 3]) -> Vec<u32> {
        let (cx, cy, cz) = self.cell_of(p);
        let mut out = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if let Some(ids) = self.map.get(&(cx + dx, cy + dy, cz + dz)) {
                        out.extend_from_slice(ids);
                    }
                }
            }
        }
        out.sort_unstable();
        out
    }
}

/// 🖼️ Uniform hashed 2D grid for image-space bucketing: buckets `u32` ids by integer cell, with 9-cell neighbourhood gathering.
#[derive(Clone, Debug)]
pub struct Grid2 {
    cell: f64,
    map: HashMap<(i32, i32), Vec<u32>>,
}

impl Grid2 {
    /// 🖼️ Creates an empty grid with the given positive cell edge length.
    pub async fn new(cell: f64) -> Self {
        assert!(cell > 0.0, "grid cell size must be positive");
        Self { cell, map: HashMap::new() }
    }

    /// 🧭️ Integer cell containing `p`, via floor division that stays correct for negative coordinates.
    pub async fn cell_of(&self, p: [f64; 2]) -> (i32, i32) {
        ((p[0] / self.cell).floor() as i32, (p[1] / self.cell).floor() as i32)
    }

    /// ➕️ Buckets `id` into the cell containing `p`.
    pub async fn insert(&mut self, p: [f64; 2], id: u32) {
        let key = self.cell_of(p);
        self.map.entry(key).or_default().push(id);
    }

    /// 🔍️ All ids bucketed in the 9 cells surrounding (and including) the cell of `p`, sorted ascending for determinism.
    pub async fn neighbors9(&self, p: [f64; 2]) -> Vec<u32> {
        let (cx, cy) = self.cell_of(p);
        let mut out = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(ids) = self.map.get(&(cx + dx, cy + dy)) {
                    out.extend_from_slice(ids);
                }
            }
        }
        out.sort_unstable();
        out
    }
}
// #endregion 🔖️VoxelGrid

// #region 🔖️Octree
const MORTON_BITS: u32 = 21;
const MORTON_COORD_MAX: i64 = (1 << MORTON_BITS) - 1;

async fn morton3_spread(v: u32) -> u64 {
    let mut x = u64::from(v) & 0x1F_FFFF;
    x = (x | (x << 32)) & 0x001F_0000_0000_FFFF;
    x = (x | (x << 16)) & 0x001F_0000_FF00_00FF;
    x = (x | (x << 8)) & 0x100F_00F0_0F00_F00F;
    x = (x | (x << 4)) & 0x10C3_0C30_C30C_30C3;
    x = (x | (x << 2)) & 0x1249_2492_4924_9249;
    x
}

async fn morton3_compact(v: u64) -> u32 {
    let mut x = v & 0x1249_2492_4924_9249;
    x = (x | (x >> 2)) & 0x10C3_0C30_C30C_30C3;
    x = (x | (x >> 4)) & 0x100F_00F0_0F00_F00F;
    x = (x | (x >> 8)) & 0x001F_0000_FF00_00FF;
    x = (x | (x >> 16)) & 0x001F_0000_0000_FFFF;
    x = (x | (x >> 32)) & 0x1F_FFFF;
    x as u32
}

/// 🧬️ Interleaves the low 21 bits of `x`, `y`, `z` into a 63-bit Morton code (`x` in bit 0, `y` in bit 1, `z` in bit 2 of each triple). <https://en.wikipedia.org/wiki/Z-order_curve>
pub async fn morton3_encode(x: u32, y: u32, z: u32) -> u64 {
    morton3_spread(x) | (morton3_spread(y) << 1) | (morton3_spread(z) << 2)
}

/// 🧬️ Recovers the 21-bit `(x, y, z)` coordinates interleaved by [`morton3_encode`].
pub async fn morton3_decode(code: u64) -> (u32, u32, u32) {
    (morton3_compact(code), morton3_compact(code >> 1), morton3_compact(code >> 2))
}

async fn octree_grid_coord(origin: f64, size: f64, cells: f64, v: f64) -> i64 {
    (((v - origin) / size) * cells).floor() as i64
}

/// 🐙️ Linear point octree: points hashed to 63-bit Morton codes on a `2^max_depth` grid over the computed bounding cube, stored as a code-sorted array for iterative octant range queries and voxel downsampling. <https://en.wikipedia.org/wiki/Octree>
#[derive(Clone, Debug)]
pub struct PointOctree {
    origin: [f64; 3],
    size: f64,
    max_depth: u32,
    entries: Vec<(u64, u32)>,
    points: Vec<[f64; 3]>,
}

impl PointOctree {
    /// 🏗️ Builds the octree over the axis-aligned bounding cube of `points` (`max_depth` clamped to 21, degenerate clouds get a unit cube); point ids are their original slice indices.
    pub async fn build(points: &[[f64; 3]], max_depth: u32) -> Self {
        let max_depth = max_depth.min(MORTON_BITS);
        if points.is_empty() {
            return Self { origin: [0.0; 3], size: 1.0, max_depth, entries: Vec::new(), points: Vec::new() };
        }
        let mut origin = points[0];
        let mut hi = points[0];
        for p in points {
            for a in 0..3 {
                origin[a] = origin[a].min(p[a]);
                hi[a] = hi[a].max(p[a]);
            }
        }
        let extent = (hi[0] - origin[0]).max(hi[1] - origin[1]).max(hi[2] - origin[2]);
        let size = if extent > 0.0 { extent } else { 1.0 };
        let cells = (1u64 << max_depth) as f64;
        let coord_max = (1i64 << max_depth) - 1;
        let mut entries: Vec<(u64, u32)> = points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let cx = octree_grid_coord(origin[0], size, cells, p[0]).clamp(0, coord_max) as u32;
                let cy = octree_grid_coord(origin[1], size, cells, p[1]).clamp(0, coord_max) as u32;
                let cz = octree_grid_coord(origin[2], size, cells, p[2]).clamp(0, coord_max) as u32;
                (morton3_encode(cx, cy, cz), i as u32)
            })
            .collect();
        entries.sort_unstable();
        Self { origin, size, max_depth, entries, points: points.to_vec() }
    }

    /// 🧊️ Voxel-downsample primitive: per occupied cell of edge length `cell` (anchored at the cube origin), the point count and centroid, sorted by the cell's Morton code for determinism.
    pub async fn downsample(&self, cell: f64) -> Vec<(usize, [f64; 3])> {
        if cell <= 0.0 || self.points.is_empty() {
            return Vec::new();
        }
        let mut cells: HashMap<u64, (usize, [f64; 3])> = HashMap::new();
        for p in &self.points {
            let cx = (((p[0] - self.origin[0]) / cell).floor() as i64).clamp(0, MORTON_COORD_MAX) as u32;
            let cy = (((p[1] - self.origin[1]) / cell).floor() as i64).clamp(0, MORTON_COORD_MAX) as u32;
            let cz = (((p[2] - self.origin[2]) / cell).floor() as i64).clamp(0, MORTON_COORD_MAX) as u32;
            let slot = cells.entry(morton3_encode(cx, cy, cz)).or_insert((0, [0.0; 3]));
            slot.0 += 1;
            for (acc, v) in slot.1.iter_mut().zip(p.iter()) {
                *acc += v;
            }
        }
        let mut keyed: Vec<(u64, usize, [f64; 3])> = cells.into_iter().map(|(code, (count, sum))| (code, count, sum)).collect();
        keyed.sort_unstable_by_key(|e| e.0);
        keyed
            .into_iter()
            .map(|(_, count, sum)| {
                let inv = 1.0 / count as f64;
                (count, [sum[0] * inv, sum[1] * inv, sum[2] * inv])
            })
            .collect()
    }

    /// 📦️ Ids of all points inside the closed box `[min, max]`, sorted ascending; iterative octant descent over Morton-code intervals with exact per-point filtering at the leaves.
    pub async fn range(&self, min: [f64; 3], max: [f64; 3]) -> Vec<u32> {
        let mut out = Vec::new();
        if self.entries.is_empty() {
            return out;
        }
        let cells = (1u64 << self.max_depth) as f64;
        let coord_max = (1i64 << self.max_depth) - 1;
        let mut lo_c = [0i64; 3];
        let mut hi_c = [0i64; 3];
        for a in 0..3 {
            lo_c[a] = octree_grid_coord(self.origin[a], self.size, cells, min[a]).clamp(0, coord_max);
            hi_c[a] = octree_grid_coord(self.origin[a], self.size, cells, max[a]).clamp(0, coord_max);
        }
        let mut stack: Vec<(u32, u64, u64, u64)> = vec![(0, 0, 0, 0)];
        while let Some((level, cx, cy, cz)) = stack.pop() {
            let shift = self.max_depth - level;
            let span = 1i64 << shift;
            let base = [(cx << shift) as i64, (cy << shift) as i64, (cz << shift) as i64];
            if (0..3).any(|a| base[a] > hi_c[a] || base[a] + span - 1 < lo_c[a]) {
                continue;
            }
            let code_lo = morton3_encode(cx as u32, cy as u32, cz as u32) << (3 * shift);
            let code_hi = code_lo + (1u64 << (3 * shift));
            let start = self.entries.partition_point(|e| e.0 < code_lo);
            let end = start + self.entries[start..].partition_point(|e| e.0 < code_hi);
            if start == end {
                continue;
            }
            if level == self.max_depth {
                for &(_, id) in &self.entries[start..end] {
                    let p = self.points[id as usize];
                    if (0..3).all(|a| (min[a]..=max[a]).contains(&p[a])) {
                        out.push(id);
                    }
                }
            } else {
                for oct in 0..8u64 {
                    stack.push((level + 1, (cx << 1) | (oct & 1), (cy << 1) | ((oct >> 1) & 1), (cz << 1) | (oct >> 2)));
                }
            }
        }
        out.sort_unstable();
        out
    }
}
// #endregion 🔖️Octree

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    async fn lcg_next(state: &mut u64) -> u64 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *state
    }

    async fn rand_unit(state: &mut u64) -> f64 {
        (lcg_next(state) >> 11) as f64 / (1u64 << 53) as f64
    }

    async fn rand_range(state: &mut u64, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * rand_unit(state)
    }

    async fn make_cloud<const D: usize>(n: usize, seed: u64) -> Vec<[f64; D]> {
        let mut state = seed;
        let mut pts: Vec<[f64; D]> = (0..n)
            .map(|_| {
                let mut p = [0.0; D];
                for v in p.iter_mut() {
                    *v = rand_range(&mut state, -50.0, 50.0);
                }
                p
            })
            .collect();
        for i in 0..n / 10 {
            let src = pts[(i * 13 + 7) % n];
            pts[i] = src;
        }
        pts
    }

    async fn brute_all<const D: usize>(pts: &[[f64; D]], q: &[f64; D]) -> Vec<(u32, f64)> {
        let mut all: Vec<(u32, f64)> = pts.iter().enumerate().map(|(i, p)| (i as u32, dist_sq(p, q))).collect();
        all.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
        all
    }

    async fn make_query<const D: usize>(state: &mut u64, qi: usize) -> [f64; D] {
        let mut q = [0.0; D];
        for v in q.iter_mut() {
            *v = rand_range(state, -80.0, 80.0);
        }
        if qi.is_multiple_of(5) {
            for v in q.iter_mut() {
                *v += 300.0;
            }
        }
        q
    }

    async fn check_kd_nearest_parity<const D: usize>(seed: u64) {
        let pts = make_cloud::<D>(2000, seed);
        let tree = KdTree::build(&pts);
        let mut state = seed ^ 0xABCD;
        for qi in 0..50 {
            let q = make_query::<D>(&mut state, qi);
            let expected = brute_all(&pts, &q);
            assert_eq!(tree.nearest(&q), Some(expected[0]));
            for &k in &[1usize, 7, 64, 2500] {
                let got = tree.k_nearest(&q, k);
                let want: Vec<(u32, f64)> = expected.iter().copied().take(k).collect();
                assert_eq!(got, want);
            }
        }
    }

    #[test]
    async fn kd_nearest_and_k_nearest_match_brute_force_d2() {
        check_kd_nearest_parity::<2>(11);
    }

    #[test]
    async fn kd_nearest_and_k_nearest_match_brute_force_d3() {
        check_kd_nearest_parity::<3>(23);
    }

    async fn check_kd_radius_parity<const D: usize>(seed: u64) {
        let pts = make_cloud::<D>(2000, seed);
        let tree = KdTree::build(&pts);
        let mut state = seed ^ 0x5150;
        for qi in 0..30 {
            let q = make_query::<D>(&mut state, qi);
            for &r in &[0.0f64, 5.0, 20.0, 60.0] {
                let got = tree.radius(&q, r);
                let mut want: Vec<(u32, f64)> = brute_all(&pts, &q).into_iter().filter(|e| e.1 <= r * r).collect();
                want.sort_unstable_by_key(|e| e.0);
                assert_eq!(got, want);
            }
            assert!(tree.radius(&q, -1.0).is_empty());
        }
    }

    #[test]
    async fn kd_radius_matches_brute_force_d2() {
        check_kd_radius_parity::<2>(31);
    }

    #[test]
    async fn kd_radius_matches_brute_force_d3() {
        check_kd_radius_parity::<3>(41);
    }

    async fn check_kd_aabb_parity<const D: usize>(seed: u64) {
        let pts = make_cloud::<D>(2000, seed);
        let tree = KdTree::build(&pts);
        let mut state = seed ^ 0xBEEF;
        for _ in 0..30 {
            let mut lo = [0.0; D];
            let mut hi = [0.0; D];
            for a in 0..D {
                let x = rand_range(&mut state, -60.0, 60.0);
                let y = rand_range(&mut state, -60.0, 60.0);
                lo[a] = x.min(y);
                hi[a] = x.max(y);
            }
            let mut got = Vec::new();
            tree.for_each_in_aabb(&lo, &hi, |id| got.push(id));
            got.sort_unstable();
            let want: Vec<u32> = pts.iter().enumerate().filter(|(_, p)| (0..D).all(|a| (lo[a]..=hi[a]).contains(&p[a]))).map(|(i, _)| i as u32).collect();
            assert_eq!(got, want);
        }
    }

    #[test]
    async fn kd_aabb_visits_match_brute_force_d2() {
        check_kd_aabb_parity::<2>(51);
    }

    #[test]
    async fn kd_aabb_visits_match_brute_force_d3() {
        check_kd_aabb_parity::<3>(61);
    }

    #[test]
    async fn kd_empty_tree_is_safe() {
        let tree = KdTree::<3>::build(&[]);
        let q = [0.0; 3];
        assert_eq!(tree.nearest(&q), None);
        assert!(tree.k_nearest(&q, 5).is_empty());
        assert!(tree.radius(&q, 10.0).is_empty());
        let mut visited = 0;
        tree.for_each_in_aabb(&[-1.0; 3], &[1.0; 3], |_| visited += 1);
        assert_eq!(visited, 0);
        let full = KdTree::<2>::build(&[[1.0, 2.0]]);
        assert!(full.k_nearest(&[0.0, 0.0], 0).is_empty());
    }

    #[test]
    async fn voxel_grid3_cell_of_floors_negative_coords() {
        let grid = VoxelGrid3::new(2.5);
        assert_eq!(grid.cell_of([-0.1, 0.0, 2.5]), (-1, 0, 1));
        assert_eq!(grid.cell_of([-2.5, -2.6, 4.9]), (-1, -2, 1));
    }

    #[test]
    async fn voxel_grid3_neighbors27_matches_brute_force() {
        let cell = 2.5;
        let mut grid = VoxelGrid3::new(cell);
        let mut state = 42u64;
        let mut pts: Vec<[f64; 3]> = (0..300).map(|_| [rand_range(&mut state, -12.0, 12.0), rand_range(&mut state, -12.0, 12.0), rand_range(&mut state, -12.0, 12.0)]).collect();
        for k in -2i32..=2 {
            for l in -2i32..=2 {
                for m in -2i32..=2 {
                    pts.push([f64::from(k) * cell, f64::from(l) * cell, f64::from(m) * cell]);
                }
            }
        }
        for (i, p) in pts.iter().enumerate() {
            grid.insert(*p, i as u32);
        }
        let mut queries: Vec<[f64; 3]> = (0..40).map(|_| [rand_range(&mut state, -13.0, 13.0), rand_range(&mut state, -13.0, 13.0), rand_range(&mut state, -13.0, 13.0)]).collect();
        queries.extend_from_slice(&pts[300..330]);
        for q in &queries {
            let qc = grid.cell_of(*q);
            let want: Vec<u32> = pts
                .iter()
                .enumerate()
                .filter(|(_, p)| {
                    let c = grid.cell_of(**p);
                    (c.0 - qc.0).abs() <= 1 && (c.1 - qc.1).abs() <= 1 && (c.2 - qc.2).abs() <= 1
                })
                .map(|(i, _)| i as u32)
                .collect();
            assert_eq!(grid.neighbors27(*q), want);
        }
    }

    #[test]
    async fn grid2_neighbors9_matches_brute_force() {
        let cell = 4.0;
        let mut grid = Grid2::new(cell);
        let mut state = 77u64;
        let mut pts: Vec<[f64; 2]> = (0..200).map(|_| [rand_range(&mut state, -20.0, 20.0), rand_range(&mut state, -20.0, 20.0)]).collect();
        for k in -3i32..=3 {
            for l in -3i32..=3 {
                pts.push([f64::from(k) * cell, f64::from(l) * cell]);
            }
        }
        for (i, p) in pts.iter().enumerate() {
            grid.insert(*p, i as u32);
        }
        let queries: Vec<[f64; 2]> = (0..40).map(|_| [rand_range(&mut state, -22.0, 22.0), rand_range(&mut state, -22.0, 22.0)]).collect();
        for q in queries.iter().chain(pts[200..220].iter()) {
            let qc = grid.cell_of(*q);
            let want: Vec<u32> = pts
                .iter()
                .enumerate()
                .filter(|(_, p)| {
                    let c = grid.cell_of(**p);
                    (c.0 - qc.0).abs() <= 1 && (c.1 - qc.1).abs() <= 1
                })
                .map(|(i, _)| i as u32)
                .collect();
            assert_eq!(grid.neighbors9(*q), want);
        }
    }

    #[test]
    async fn morton3_round_trips() {
        assert_eq!(morton3_encode(0, 0, 0), 0);
        assert_eq!(morton3_encode(1, 0, 0), 1);
        assert_eq!(morton3_encode(0, 1, 0), 2);
        assert_eq!(morton3_encode(0, 0, 1), 4);
        let mut state = 5u64;
        for _ in 0..200 {
            let x = (lcg_next(&mut state) & 0x1F_FFFF) as u32;
            let y = (lcg_next(&mut state) & 0x1F_FFFF) as u32;
            let z = (lcg_next(&mut state) & 0x1F_FFFF) as u32;
            assert_eq!(morton3_decode(morton3_encode(x, y, z)), (x, y, z));
        }
    }

    #[test]
    async fn octree_downsample_preserves_counts_and_centroids() {
        let pts = make_cloud::<3>(2000, 99);
        let tree = PointOctree::build(&pts, 8);
        let cell = 7.0;
        let ds = tree.downsample(cell);
        let total: usize = ds.iter().map(|e| e.0).sum();
        assert_eq!(total, pts.len());
        let mut origin = pts[0];
        for p in &pts {
            for a in 0..3 {
                origin[a] = origin[a].min(p[a]);
            }
        }
        let mut want: HashMap<u64, (usize, [f64; 3])> = HashMap::new();
        for p in &pts {
            let cx = (((p[0] - origin[0]) / cell).floor() as i64).clamp(0, (1 << 21) - 1) as u32;
            let cy = (((p[1] - origin[1]) / cell).floor() as i64).clamp(0, (1 << 21) - 1) as u32;
            let cz = (((p[2] - origin[2]) / cell).floor() as i64).clamp(0, (1 << 21) - 1) as u32;
            let slot = want.entry(morton3_encode(cx, cy, cz)).or_insert((0, [0.0; 3]));
            slot.0 += 1;
            for (acc, v) in slot.1.iter_mut().zip(p.iter()) {
                *acc += v;
            }
        }
        let mut want: Vec<(u64, usize, [f64; 3])> = want.into_iter().map(|(code, (count, sum))| (code, count, sum)).collect();
        want.sort_unstable_by_key(|e| e.0);
        assert_eq!(ds.len(), want.len());
        for (got, (_, count, sum)) in ds.iter().zip(want.iter()) {
            assert_eq!(got.0, *count);
            for (g, s) in got.1.iter().zip(sum.iter()) {
                assert!((g - s / *count as f64).abs() < 1e-9);
            }
        }
    }

    #[test]
    async fn octree_range_matches_brute_force() {
        let pts = make_cloud::<3>(2000, 7);
        for &depth in &[0u32, 4, 8, 30] {
            let tree = PointOctree::build(&pts, depth);
            let mut state = 1234u64 ^ u64::from(depth);
            for _ in 0..25 {
                let mut mn = [0.0; 3];
                let mut mx = [0.0; 3];
                for a in 0..3 {
                    let x = rand_range(&mut state, -60.0, 60.0);
                    let y = rand_range(&mut state, -60.0, 60.0);
                    mn[a] = x.min(y);
                    mx[a] = x.max(y);
                }
                let got = tree.range(mn, mx);
                let want: Vec<u32> = pts.iter().enumerate().filter(|(_, p)| (0..3).all(|a| (mn[a]..=mx[a]).contains(&p[a]))).map(|(i, _)| i as u32).collect();
                assert_eq!(got, want);
            }
            let all = tree.range([-1000.0; 3], [1000.0; 3]);
            assert_eq!(all, (0..pts.len() as u32).collect::<Vec<u32>>());
            assert!(tree.range([500.0; 3], [600.0; 3]).is_empty());
        }
    }

    #[test]
    async fn octree_empty_build_is_safe() {
        let tree = PointOctree::build(&[], 8);
        assert!(tree.range([-1.0; 3], [1.0; 3]).is_empty());
        assert!(tree.downsample(1.0).is_empty());
        let flat = PointOctree::build(&[[2.0, 2.0, 2.0], [2.0, 2.0, 2.0]], 8);
        assert_eq!(flat.range([1.0; 3], [3.0; 3]), vec![0, 1]);
        assert_eq!(flat.downsample(1.0), vec![(2, [2.0, 2.0, 2.0])]);
    }
}
// #endregion 🔖️Tests
