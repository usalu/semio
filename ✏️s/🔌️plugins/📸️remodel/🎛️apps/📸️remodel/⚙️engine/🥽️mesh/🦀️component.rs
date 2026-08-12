//! 🕸️ Surface extraction and texturing: marching cubes, quadric simplification, hole filling, UV atlas and multi-view texture blending.
//!
//! This crate's one non-negotiable contract: [`mesh_pipeline_step`] always terminates in either
//! `Done` with [`WatertightReport::is_watertight`] `true`, or an explicit `Failed`. There is no
//! silent "good enough" mesh — [`close_voxel`] is a by-construction closed-2-manifold fallback
//! that the pipeline falls back to whenever repair/hole-filling leaves defects behind.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 📦️glue.rs for the wiring).
use crate::apps::remodel::engine::{camera as remodel_camera, dense as remodel_dense, images as remodel_image};

use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

// #region 🔖️TriMesh
/// ➕️ Adds two 3-vectors.
fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// ➖️ Subtracts two 3-vectors.
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// ✖️ Scales a 3-vector.
fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

/// 🔀️ Cross product of two 3-vectors.
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// 🔵️ Dot product of two 3-vectors.
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// 📏️ Euclidean norm of a 3-vector.
fn norm3(a: [f64; 3]) -> f64 {
    dot3(a, a).sqrt()
}

/// 🧭️ Unit-length copy of `a`, or `[0,0,0]` when `a` is (numerically) the zero vector.
fn normalize3(a: [f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-15 {
        [0.0, 0.0, 0.0]
    } else {
        scale3(a, 1.0 / n)
    }
}

/// 🎯️ Linear interpolation between two 3-points.
fn lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    add3(a, scale3(sub3(b, a), t))
}

/// 🧮️ Builds a dense `MatD` from row-major `f64` rows (`MatD` itself only exposes `zeros`/`set`,
/// unlike the generic `MatG<T>::from_rows`).
fn matd_from_rows(rows: &[Vec<f64>]) -> math::algebra::MatD {
    let r = rows.len();
    let c = rows.first().map_or(0, Vec::len);
    let mut m = math::algebra::MatD::zeros(r, c);
    for (row, values) in rows.iter().enumerate() {
        for (col, &v) in values.iter().enumerate() {
            m.set(row, col, v);
        }
    }
    m
}

/// 🔺️ An f64 vertex-soup triangle mesh: the workhorse representation every other region in this
/// crate reads from or writes into. No implicit topology — [`TriMesh::edge_map`] derives it on
/// demand from `triangles` alone, so iteration order (via `BTreeMap`) is always deterministic.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriMesh {
    pub positions: Vec<[f64; 3]>,
    pub triangles: Vec<[u32; 3]>,
}

/// 🗺️ Sorted undirected vertex pair → incident face indices, deterministic-order workhorse for
/// manifold checks, boundary detection and orientation flooding.
pub type EdgeMap = BTreeMap<(u32, u32), Vec<u32>>;

/// 🔗️ Canonical (sorted) undirected edge key for a vertex pair.
fn sorted_edge(a: u32, b: u32) -> (u32, u32) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

impl TriMesh {
    pub fn new() -> Self {
        Self { positions: Vec::new(), triangles: Vec::new() }
    }

    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// 🗺️ Builds the sorted-edge → incident-face-indices map from `triangles`.
    pub fn edge_map(&self) -> EdgeMap {
        let mut map: EdgeMap = BTreeMap::new();
        for (f, tri) in self.triangles.iter().enumerate() {
            for k in 0..3 {
                let key = sorted_edge(tri[k], tri[(k + 1) % 3]);
                map.entry(key).or_default().push(f as u32);
            }
        }
        map
    }

    /// 🔽️ Unnormalized area-weighted normal of triangle `f` (`|result| == 2 * area`).
    pub fn face_normal_unnormalized(&self, f: usize) -> [f64; 3] {
        let tri = self.triangles[f];
        let (a, b, c) = (self.positions[tri[0] as usize], self.positions[tri[1] as usize], self.positions[tri[2] as usize]);
        cross3(sub3(b, a), sub3(c, a))
    }

    pub fn face_normal(&self, f: usize) -> [f64; 3] {
        normalize3(self.face_normal_unnormalized(f))
    }

    pub fn face_area(&self, f: usize) -> f64 {
        norm3(self.face_normal_unnormalized(f)) * 0.5
    }

    pub fn face_centroid(&self, f: usize) -> [f64; 3] {
        let tri = self.triangles[f];
        let (a, b, c) = (self.positions[tri[0] as usize], self.positions[tri[1] as usize], self.positions[tri[2] as usize]);
        scale3(add3(add3(a, b), c), 1.0 / 3.0)
    }

    pub fn bbox(&self) -> ([f64; 3], [f64; 3]) {
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];
        for p in &self.positions {
            for k in 0..3 {
                lo[k] = lo[k].min(p[k]);
                hi[k] = hi[k].max(p[k]);
            }
        }
        if self.positions.is_empty() {
            lo = [0.0; 3];
            hi = [0.0; 3];
        }
        (lo, hi)
    }

    pub fn bbox_diagonal(&self) -> f64 {
        let (lo, hi) = self.bbox();
        norm3(sub3(hi, lo))
    }

    /// 🧮️ Signed volume via the divergence theorem, summing signed tetrahedron volumes from the
    /// origin over every triangle; positive for a consistently outward-wound closed mesh.
    pub fn signed_volume(&self) -> f64 {
        let mut total = 0.0;
        for tri in &self.triangles {
            let (a, b, c) = (self.positions[tri[0] as usize], self.positions[tri[1] as usize], self.positions[tri[2] as usize]);
            total += dot3(a, cross3(b, c)) / 6.0;
        }
        total
    }

    /// 🧭️ Area-weighted per-vertex normals, accumulated from face normals over each vertex's
    /// incident triangles.
    pub fn compute_vertex_normals(&self) -> Vec<[f64; 3]> {
        let mut normals = vec![[0.0; 3]; self.positions.len()];
        for (f, tri) in self.triangles.iter().enumerate() {
            let n = self.face_normal_unnormalized(f);
            for &v in tri {
                normals[v as usize] = add3(normals[v as usize], n);
            }
        }
        for n in &mut normals {
            *n = normalize3(*n);
        }
        normals
    }
}

/// 🧩️ Minimal union-find over `0..n`, path-halved on find, union by size — the shared workhorse
/// behind small-component removal, non-manifold fan grouping and connected-component counting.
struct DisjointSet {
    parent: Vec<u32>,
    size: Vec<u32>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self { parent: (0..n as u32).collect(), size: vec![1; n] }
    }

    fn find(&mut self, x: u32) -> u32 {
        let mut root = x;
        while self.parent[root as usize] != root {
            root = self.parent[root as usize];
        }
        let mut cur = x;
        while self.parent[cur as usize] != root {
            let next = self.parent[cur as usize];
            self.parent[cur as usize] = root;
            cur = next;
        }
        root
    }

    fn union(&mut self, a: u32, b: u32) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        let (big, small) = if self.size[ra as usize] >= self.size[rb as usize] { (ra, rb) } else { (rb, ra) };
        self.parent[small as usize] = big;
        self.size[big as usize] += self.size[small as usize];
    }
}
// #endregion 🔖️TriMesh

// #region 🔖️Topology
/// ⚠️ Why a [`TriMesh`] failed to become a [`HalfedgeTopology`]: it wasn't yet an oriented
/// 2-manifold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopologyError {
    NonManifoldEdge { a: u32, b: u32, face_count: usize },
    InconsistentOrientation { a: u32, b: u32 },
    NonManifoldVertex(u32),
    DegenerateTriangle(usize),
}

impl std::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonManifoldEdge { a, b, face_count } => write!(f, "edge ({a},{b}) has {face_count} incident faces"),
            Self::InconsistentOrientation { a, b } => write!(f, "edge ({a},{b}) is traversed the same direction by two faces"),
            Self::NonManifoldVertex(v) => write!(f, "vertex {v} has more than one incident fan"),
            Self::DegenerateTriangle(idx) => write!(f, "triangle {idx} is degenerate"),
        }
    }
}

impl std::error::Error for TopologyError {}

#[derive(Clone, Copy, Debug)]
struct Halfedge {
    origin: u32,
    twin: Option<u32>,
    next: u32,
}

/// 🕸️ Halfedge adjacency over an oriented 2-manifold [`TriMesh`], own lightweight structure built
/// fresh from `EdgeMap` checks each time (not a general-purpose editable mesh kernel — read-only,
/// exists purely to answer "who is my twin" and "walk this boundary").
pub struct HalfedgeTopology {
    halfedges: Vec<Halfedge>,
}

impl HalfedgeTopology {
    /// 🏗️ Builds halfedge adjacency, rejecting anything that isn't already an oriented 2-manifold:
    /// every edge has at most 2 incident faces, each directed edge appears at most once, and every
    /// vertex's incident faces form a single fan (no pinch points).
    pub fn build(mesh: &TriMesh) -> Result<Self, TopologyError> {
        for (idx, tri) in mesh.triangles.iter().enumerate() {
            if tri[0] == tri[1] || tri[1] == tri[2] || tri[0] == tri[2] {
                return Err(TopologyError::DegenerateTriangle(idx));
            }
        }
        let edges = mesh.edge_map();
        for (&(a, b), faces) in &edges {
            if faces.len() > 2 {
                return Err(TopologyError::NonManifoldEdge { a, b, face_count: faces.len() });
            }
        }
        let mut halfedges = Vec::with_capacity(mesh.triangles.len() * 3);
        for tri in &mesh.triangles {
            let base = halfedges.len() as u32;
            for (k, &origin) in tri.iter().enumerate() {
                halfedges.push(Halfedge { origin, twin: None, next: base + ((k as u32 + 1) % 3) });
            }
        }
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for (he_id, he) in halfedges.iter().enumerate() {
            let dest = halfedges[he.next as usize].origin;
            if directed.insert((he.origin, dest), he_id as u32).is_some() {
                return Err(TopologyError::InconsistentOrientation { a: he.origin.min(dest), b: he.origin.max(dest) });
            }
        }
        let twins: Vec<Option<u32>> = halfedges
            .iter()
            .map(|he| {
                let dest = halfedges[he.next as usize].origin;
                directed.get(&(dest, he.origin)).copied()
            })
            .collect();
        for (he, twin) in halfedges.iter_mut().zip(twins) {
            he.twin = twin;
        }
        let topology = Self { halfedges };
        topology.check_vertex_fans(mesh.vertex_count())?;
        Ok(topology)
    }

    fn destination(&self, he: u32) -> u32 {
        self.halfedges[self.halfedges[he as usize].next as usize].origin
    }

    /// 🌀️ Rejects pinch vertices: walks the fan around every vertex (via `twin`/`next`) from each
    /// outgoing halfedge and confirms every halfedge originating there is reached by a single walk.
    fn check_vertex_fans(&self, vertex_count: usize) -> Result<(), TopologyError> {
        let mut outgoing: Vec<Vec<u32>> = vec![Vec::new(); vertex_count];
        for (id, he) in self.halfedges.iter().enumerate() {
            outgoing[he.origin as usize].push(id as u32);
        }
        for (v, out) in outgoing.iter().enumerate() {
            if out.len() <= 1 {
                continue;
            }
            let start = *out.iter().find(|&&he| self.halfedges[he as usize].twin.is_none()).unwrap_or(&out[0]);
            let mut visited = HashSet::new();
            let mut cur = start;
            loop {
                visited.insert(cur);
                let prev_around = self.prev(cur);
                match self.halfedges[prev_around as usize].twin {
                    Some(t) if !visited.contains(&t) => cur = t,
                    _ => break,
                }
            }
            if visited.len() != out.len() {
                return Err(TopologyError::NonManifoldVertex(v as u32));
            }
        }
        Ok(())
    }

    fn prev(&self, he: u32) -> u32 {
        let mut cur = he;
        loop {
            let next = self.halfedges[cur as usize].next;
            if next == he {
                return cur;
            }
            cur = next;
        }
    }

    /// 🔁️ Every boundary loop (halfedges with no twin), each as an ordered vertex cycle; empty
    /// when the mesh is already closed.
    pub fn boundary_loops(&self) -> Vec<Vec<u32>> {
        let mut boundary_out: BTreeMap<u32, u32> = BTreeMap::new();
        for (id, he) in self.halfedges.iter().enumerate() {
            if he.twin.is_none() {
                boundary_out.insert(he.origin, id as u32);
            }
        }
        let mut visited = vec![false; self.halfedges.len()];
        let mut loops = Vec::new();
        for (&_start_vertex, &start_he) in &boundary_out {
            if visited[start_he as usize] {
                continue;
            }
            let mut verts = Vec::new();
            let mut he = start_he;
            loop {
                visited[he as usize] = true;
                verts.push(self.halfedges[he as usize].origin);
                let dest = self.destination(he);
                let Some(&next_he) = boundary_out.get(&dest) else { break };
                he = next_he;
                if he == start_he {
                    break;
                }
            }
            loops.push(verts);
        }
        loops
    }
}
// #endregion 🔖️Topology

// #region 🔖️MarchingCubes
type Lattice = (i32, i32, i32);

/// 🔑️ Global lattice-edge key: the sorted pair of integer corner coordinates an interpolated
/// vertex lies between. Generalizes classic marching cubes' `{corner, axis}` key to also cover the
/// face-diagonal edges the tetrahedral decomposition below introduces — any two cells that share a
/// lattice edge (axis-aligned or a shared face's diagonal) compute the *identical* key, so the
/// weld map merges them into one vertex regardless of iteration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct EdgeKey(Lattice, Lattice);

impl EdgeKey {
    fn new(a: Lattice, b: Lattice) -> Self {
        if a <= b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
}

/// ⚖️ One corner sample: its global lattice coordinate, world position and scalar field value.
#[derive(Clone, Copy)]
struct Corner {
    key: Lattice,
    pos: [f64; 3],
    val: f64,
}

fn lattice_add(c: Lattice, d: (i32, i32, i32)) -> Lattice {
    (c.0 + d.0, c.1 + d.1, c.2 + d.2)
}

/// 🧊️ The fixed 6-tetrahedron decomposition of a unit cube along its `c0`-`c6` space diagonal
/// (Doi & Koide 1991). Every cube in the lattice uses this *same* relative pattern, so any two
/// cubes sharing a face always assign identical global vertices to identical relative corner
/// slots on that shared face — which is exactly what makes the face-diagonal split agree between
/// neighbors and keeps extraction crack-free without any per-face ambiguity test.
fn cube_tets(corners: &[Corner; 8]) -> [[Corner; 4]; 6] {
    let (c0, c1, c2, c3, c4, c5, c6, c7) = (corners[0], corners[1], corners[2], corners[3], corners[4], corners[5], corners[6], corners[7]);
    [[c0, c1, c2, c6], [c0, c2, c3, c6], [c0, c3, c7, c6], [c0, c7, c4, c6], [c0, c4, c5, c6], [c0, c5, c1, c6]]
}

/// 🎯️ Weight of the requested triangle normal orientation relative to `odd`: `+1.0` when the lone
/// differing vertex is inside the surface (normal should point away from it), `-1.0` when it's
/// outside (normal should point toward it) — see `march_tet`'s doc for the full derivation.
fn orient_triangle(positions: &[[f64; 3]], tri: &mut [u32; 3], odd_pos: [f64; 3], desired_sign: f64) {
    let (a, b, c) = (positions[tri[0] as usize], positions[tri[1] as usize], positions[tri[2] as usize]);
    let normal = cross3(sub3(b, a), sub3(c, a));
    let centroid = scale3(add3(add3(a, b), c), 1.0 / 3.0);
    if dot3(normal, sub3(centroid, odd_pos)) * desired_sign < 0.0 {
        tri.swap(1, 2);
    }
}

/// 🪚️ Weld helper: returns the (possibly newly created) global vertex index for the isosurface
/// crossing point between two corners, canonicalizing argument order first so both directions of
/// traversal hash to the same key and interpolate to the bit-identical position.
fn weld_edge(weld: &mut HashMap<EdgeKey, u32>, positions: &mut Vec<[f64; 3]>, a: Corner, b: Corner, iso: f64) -> u32 {
    let (lo, hi) = if a.key <= b.key { (a, b) } else { (b, a) };
    let key = EdgeKey::new(lo.key, hi.key);
    if let Some(&idx) = weld.get(&key) {
        return idx;
    }
    let t = ((iso - lo.val) / (hi.val - lo.val)).clamp(0.0, 1.0);
    let pos = lerp3(lo.pos, hi.pos, t);
    let idx = positions.len() as u32;
    positions.push(pos);
    weld.insert(key, idx);
    idx
}

/// 🔺️ Marches one tetrahedron: 0, 1 or 2 triangles depending on how many of its 4 corners are on
/// the inside (`val < iso`) side. Tetrahedra have triangular faces only, so — unlike a cube face —
/// there is never an ambiguous case to resolve: the crossing-edge set alone fully determines the
/// topology. Triangle winding is fixed by a purely local rule (point the normal from the "odd"
/// vertex's side toward the other side), which independently reproduces the same globally
/// consistent orientation on every tet, so two triangles sharing a welded edge always end up
/// wound oppositely along it without any extra bookkeeping.
fn march_tet(corners: [Corner; 4], iso: f64, weld: &mut HashMap<EdgeKey, u32>, positions: &mut Vec<[f64; 3]>, tris: &mut Vec<[u32; 3]>) {
    let inside: [bool; 4] = std::array::from_fn(|i| corners[i].val < iso);
    let inside_count = inside.iter().filter(|&&b| b).count();
    if inside_count == 0 || inside_count == 4 {
        return;
    }
    let mut cross: Vec<(usize, usize, u32)> = Vec::with_capacity(4);
    for i in 0..4 {
        for j in (i + 1)..4 {
            if inside[i] != inside[j] {
                let idx = weld_edge(weld, positions, corners[i], corners[j], iso);
                cross.push((i, j, idx));
            }
        }
    }
    if inside_count == 1 || inside_count == 3 {
        let odd = if inside_count == 1 { inside.iter().position(|&b| b).expect("one inside corner") } else { inside.iter().position(|&b| !b).expect("one outside corner") };
        let pts: Vec<u32> = cross.iter().filter(|&&(i, j, _)| i == odd || j == odd).map(|&(_, _, idx)| idx).collect();
        let mut tri = [pts[0], pts[1], pts[2]];
        let desired_sign = if inside[odd] { 1.0 } else { -1.0 };
        orient_triangle(positions, &mut tri, corners[odd].pos, desired_sign);
        tris.push(tri);
    } else {
        let inside_idx: Vec<usize> = (0..4).filter(|&i| inside[i]).collect();
        let outside_idx: Vec<usize> = (0..4).filter(|&i| !inside[i]).collect();
        let (p0, p1) = (inside_idx[0], inside_idx[1]);
        let (q0, q1) = (outside_idx[0], outside_idx[1]);
        let find = |a: usize, b: usize| cross.iter().find(|&&(i, j, _)| (i == a && j == b) || (i == b && j == a)).expect("crossing edge").2;
        let mut quad = [find(p0, q0), find(p1, q0), find(p1, q1), find(p0, q1)];
        let quad_pos: Vec<[f64; 3]> = quad.iter().map(|&i| positions[i as usize]).collect();
        let normal = cross3(sub3(quad_pos[1], quad_pos[0]), sub3(quad_pos[2], quad_pos[0]));
        let inside_avg = scale3(add3(corners[p0].pos, corners[p1].pos), 0.5);
        let outside_avg = scale3(add3(corners[q0].pos, corners[q1].pos), 0.5);
        if dot3(normal, sub3(outside_avg, inside_avg)) < 0.0 {
            quad.reverse();
        }
        tris.push([quad[0], quad[1], quad[2]]);
        tris.push([quad[0], quad[2], quad[3]]);
    }
}

/// 🧊️ Crack-free marching-cubes extraction from a cross-block-sampled scalar field, dispatching
/// every cube to [`cube_tets`] + [`march_tet`]. `bounds_min`/`bounds_max` (inclusive) are the
/// voxel-corner coordinate range to search — `remodel_dense::TsdfVolume` does not currently expose
/// block enumeration, so the caller (who performed the integration and knows the reconstruction
/// volume's extent) supplies the search domain explicitly; this is the one deliberate signature
/// deviation from the plan's parameterless `extract_tsdf(vol, iso)`.
pub fn extract_tsdf(vol: &remodel_dense::TsdfVolume, iso: f64, bounds_min: [i32; 3], bounds_max: [i32; 3]) -> TriMesh {
    let sample = |c: Lattice| -> Option<Corner> {
        let (sdf, _weight) = vol.sample(c.0, c.1, c.2)?;
        let p = [(c.0 as f64 + 0.5) * vol.voxel_size, (c.1 as f64 + 0.5) * vol.voxel_size, (c.2 as f64 + 0.5) * vol.voxel_size];
        Some(Corner { key: c, pos: p, val: sdf })
    };
    let mut weld: HashMap<EdgeKey, u32> = HashMap::new();
    let mut positions: Vec<[f64; 3]> = Vec::new();
    let mut triangles: Vec<[u32; 3]> = Vec::new();
    for i in bounds_min[0]..bounds_max[0] {
        for j in bounds_min[1]..bounds_max[1] {
            for k in bounds_min[2]..bounds_max[2] {
                let offsets: [(i32, i32, i32); 8] = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0), (0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)];
                let mut corners = [Corner { key: (0, 0, 0), pos: [0.0; 3], val: 0.0 }; 8];
                let mut all_known = true;
                for (slot, off) in offsets.iter().enumerate() {
                    match sample(lattice_add((i, j, k), *off)) {
                        Some(c) => corners[slot] = c,
                        None => {
                            all_known = false;
                            break;
                        }
                    }
                }
                if !all_known {
                    continue;
                }
                for tet in cube_tets(&corners) {
                    march_tet(tet, iso, &mut weld, &mut positions, &mut triangles);
                }
            }
        }
    }
    TriMesh { positions, triangles }
}

/// 🧊️ A total (every cell defined), contiguous dense scalar field over an axis-aligned lattice —
/// the simple non-hashed counterpart to [`remodel_dense::TsdfVolume`] used by
/// [`extract_dense_grid`], primarily as the [`close_voxel`] fallback's occupancy field.
#[derive(Clone, Debug)]
pub struct DenseField {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub origin: [f64; 3],
    pub voxel: f64,
    pub values: Vec<f64>,
}

impl DenseField {
    pub fn new(nx: usize, ny: usize, nz: usize, origin: [f64; 3], voxel: f64, fill: f64) -> Self {
        Self { nx, ny, nz, origin, voxel, values: vec![fill; nx * ny * nz] }
    }

    fn index(&self, i: i32, j: i32, k: i32) -> Option<usize> {
        if i < 0 || j < 0 || k < 0 || i as usize >= self.nx || j as usize >= self.ny || k as usize >= self.nz {
            return None;
        }
        Some((k as usize * self.ny + j as usize) * self.nx + i as usize)
    }

    pub fn get(&self, i: i32, j: i32, k: i32) -> Option<f64> {
        self.index(i, j, k).map(|idx| self.values[idx])
    }

    pub fn set(&mut self, i: i32, j: i32, k: i32, value: f64) {
        if let Some(idx) = self.index(i, j, k) {
            self.values[idx] = value;
        }
    }

    pub fn world_corner(&self, i: i32, j: i32, k: i32) -> [f64; 3] {
        add3(self.origin, scale3([i as f64, j as f64, k as f64], self.voxel))
    }
}

/// 🧊️ Marching-cubes extraction over a total [`DenseField`] (every corner defined, so no
/// unknown-corner skipping is needed) — same crack-free tetrahedral kernel as [`extract_tsdf`],
/// reused verbatim rather than re-implemented, so the [`close_voxel`] fallback's correctness rests
/// on the identical, already-tested welding and orientation logic.
pub fn extract_dense_grid(field: &DenseField, iso: f64) -> TriMesh {
    let mut weld: HashMap<EdgeKey, u32> = HashMap::new();
    let mut positions: Vec<[f64; 3]> = Vec::new();
    let mut triangles: Vec<[u32; 3]> = Vec::new();
    for i in 0..(field.nx as i32 - 1) {
        for j in 0..(field.ny as i32 - 1) {
            for k in 0..(field.nz as i32 - 1) {
                let offsets: [(i32, i32, i32); 8] = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0), (0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)];
                let mut corners = [Corner { key: (0, 0, 0), pos: [0.0; 3], val: 0.0 }; 8];
                for (slot, off) in offsets.iter().enumerate() {
                    let c = lattice_add((i, j, k), *off);
                    let val = field.get(c.0, c.1, c.2).expect("interior cell corner is always in range");
                    corners[slot] = Corner { key: c, pos: field.world_corner(c.0, c.1, c.2), val };
                }
                for tet in cube_tets(&corners) {
                    march_tet(tet, iso, &mut weld, &mut positions, &mut triangles);
                }
            }
        }
    }
    TriMesh { positions, triangles }
}
// #endregion 🔖️MarchingCubes

// #region 🔖️Clean
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CleanStats {
    pub vertices_welded: usize,
    pub degenerate_triangles_removed: usize,
    pub zero_length_edges_collapsed: usize,
    pub small_components_removed: usize,
}

/// 🧹️ Welds exact-duplicate vertices, drops zero-area/duplicate triangles, collapses zero-length
/// edges, and removes connected components smaller than `min_component_faces` faces *and* under
/// `min_component_bbox_fraction` of the mesh's bounding-box diagonal (a component only drops when
/// both are true, so a single huge thin sliver-heavy component never gets removed by face count
/// alone).
pub fn clean_mesh(mesh: &mut TriMesh, min_component_faces: usize, min_component_bbox_fraction: f64) -> CleanStats {
    let vertices_welded = weld_duplicate_vertices(mesh);
    let mut degenerate_triangles_removed = remove_degenerate_and_duplicate_triangles(mesh);
    let zero_length_edges_collapsed = collapse_zero_length_edges(mesh);
    degenerate_triangles_removed += remove_degenerate_and_duplicate_triangles(mesh);
    let small_components_removed = remove_small_components(mesh, min_component_faces, min_component_bbox_fraction);
    CleanStats { vertices_welded, degenerate_triangles_removed, zero_length_edges_collapsed, small_components_removed }
}

fn weld_duplicate_vertices(mesh: &mut TriMesh) -> usize {
    let mut first_index: HashMap<[u64; 3], u32> = HashMap::new();
    let mut remap = vec![0u32; mesh.positions.len()];
    let mut new_positions = Vec::with_capacity(mesh.positions.len());
    let mut welded = 0usize;
    for (i, p) in mesh.positions.iter().enumerate() {
        let key = [p[0].to_bits(), p[1].to_bits(), p[2].to_bits()];
        match first_index.get(&key) {
            Some(&idx) => {
                remap[i] = idx;
                welded += 1;
            }
            None => {
                let idx = new_positions.len() as u32;
                new_positions.push(*p);
                first_index.insert(key, idx);
                remap[i] = idx;
            }
        }
    }
    mesh.positions = new_positions;
    for tri in &mut mesh.triangles {
        for v in tri.iter_mut() {
            *v = remap[*v as usize];
        }
    }
    welded
}

fn remove_degenerate_and_duplicate_triangles(mesh: &mut TriMesh) -> usize {
    let mut seen: HashSet<(u32, u32, u32)> = HashSet::new();
    let before = mesh.triangles.len();
    let mut kept = Vec::with_capacity(mesh.triangles.len());
    for (f, tri) in mesh.triangles.iter().enumerate() {
        if tri[0] == tri[1] || tri[1] == tri[2] || tri[0] == tri[2] {
            continue;
        }
        if mesh.face_area(f) < 1e-15 {
            continue;
        }
        let mut sorted = *tri;
        sorted.sort_unstable();
        if !seen.insert((sorted[0], sorted[1], sorted[2])) {
            continue;
        }
        kept.push(*tri);
    }
    mesh.triangles = kept;
    before - mesh.triangles.len()
}

fn collapse_zero_length_edges(mesh: &mut TriMesh) -> usize {
    let mut dsu = DisjointSet::new(mesh.positions.len());
    let mut collapsed = 0usize;
    for tri in &mesh.triangles {
        for k in 0..3 {
            let (a, b) = (tri[k], tri[(k + 1) % 3]);
            if norm3(sub3(mesh.positions[a as usize], mesh.positions[b as usize])) < 1e-12 {
                dsu.union(a, b);
                collapsed += 1;
            }
        }
    }
    if collapsed == 0 {
        return 0;
    }
    let mut remap = vec![0u32; mesh.positions.len()];
    let mut new_positions = Vec::new();
    let mut root_to_new: HashMap<u32, u32> = HashMap::new();
    for i in 0..mesh.positions.len() as u32 {
        let root = dsu.find(i);
        let idx = *root_to_new.entry(root).or_insert_with(|| {
            let idx = new_positions.len() as u32;
            new_positions.push(mesh.positions[root as usize]);
            idx
        });
        remap[i as usize] = idx;
    }
    mesh.positions = new_positions;
    for tri in &mut mesh.triangles {
        for v in tri.iter_mut() {
            *v = remap[*v as usize];
        }
    }
    remove_degenerate_and_duplicate_triangles(mesh);
    collapsed
}

fn remove_small_components(mesh: &mut TriMesh, min_faces: usize, min_bbox_fraction: f64) -> usize {
    if mesh.triangles.is_empty() {
        return 0;
    }
    let edges = mesh.edge_map();
    let mut dsu = DisjointSet::new(mesh.triangles.len());
    for faces in edges.values() {
        if faces.len() == 2 {
            dsu.union(faces[0], faces[1]);
        }
    }
    let mesh_diag = mesh.bbox_diagonal().max(1e-12);
    let mut groups: HashMap<u32, Vec<u32>> = HashMap::new();
    for f in 0..mesh.triangles.len() as u32 {
        groups.entry(dsu.find(f)).or_default().push(f);
    }
    let mut keep = vec![true; mesh.triangles.len()];
    let mut removed_components = 0usize;
    for faces in groups.values() {
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];
        for &f in faces {
            for &v in &mesh.triangles[f as usize] {
                let p = mesh.positions[v as usize];
                for k in 0..3 {
                    lo[k] = lo[k].min(p[k]);
                    hi[k] = hi[k].max(p[k]);
                }
            }
        }
        let comp_diag = norm3(sub3(hi, lo));
        if faces.len() < min_faces && comp_diag / mesh_diag < min_bbox_fraction {
            for &f in faces {
                keep[f as usize] = false;
            }
            removed_components += 1;
        }
    }
    mesh.triangles = mesh.triangles.iter().enumerate().filter(|&(f, _)| keep[f]).map(|(_, t)| *t).collect();
    removed_components
}

/// 🫧️ Alternating shrink (`lambda` > 0) / inflate (`mu` < 0, `|mu| > lambda`) umbrella-Laplacian
/// smoothing (Taubin 1995): applies uniform one-ring averaging with `lambda` then `mu` each
/// iteration, which — unlike plain Laplacian smoothing — does not shrink the mesh over many
/// iterations. Boundary vertices are smoothed too (no boundary pinning): callers that need crisp
/// boundaries should smooth before hole-filling.
pub fn taubin_smooth(mesh: &mut TriMesh, lambda: f64, mu: f64, iterations: usize) {
    if mesh.positions.is_empty() {
        return;
    }
    let mut neighbors: Vec<Vec<u32>> = vec![Vec::new(); mesh.positions.len()];
    for &(a, b) in mesh.edge_map().keys() {
        neighbors[a as usize].push(b);
        neighbors[b as usize].push(a);
    }
    let apply = |positions: &mut Vec<[f64; 3]>, factor: f64| {
        let mut next = positions.clone();
        for (v, nbrs) in neighbors.iter().enumerate() {
            if nbrs.is_empty() {
                continue;
            }
            let mut avg = [0.0; 3];
            for &n in nbrs {
                avg = add3(avg, positions[n as usize]);
            }
            avg = scale3(avg, 1.0 / nbrs.len() as f64);
            let laplacian = sub3(avg, positions[v]);
            next[v] = add3(positions[v], scale3(laplacian, factor));
        }
        *positions = next;
    };
    for _ in 0..iterations {
        apply(&mut mesh.positions, lambda);
        apply(&mut mesh.positions, mu);
    }
}
// #endregion 🔖️Clean

// #region 🔖️Repair
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RepairStats {
    pub non_manifold_edges_split: usize,
    pub non_manifold_vertices_split: usize,
    pub vertices_duplicated: usize,
}

/// 🩹️ Groups a set of faces (all touching some shared non-manifold feature) into fan-components by
/// connectivity through *other*, already-manifold (`<= 2`-face) edges — i.e. two faces are in the
/// same group iff there is a path between them that never crosses another non-manifold edge. This
/// is a topological stand-in for dihedral-angle fan grouping: it is deterministic, requires no
/// angle threshold tuning, and correctly separates genuinely-disjoint sheets (two spheres meeting
/// at a single edge or vertex) into distinct groups, which is exactly the property
/// `repair_non_manifold` needs.
/// 🩹️ For each distinct manifold-connectivity group among `seed_faces`, returns *every* face in the
/// whole mesh sharing that group's component *and* still referencing `a` or `b` — not just the
/// seed faces themselves. This distinction matters: a face can be non-manifold-edge-adjacent
/// (touching the flagged edge directly) while a same-fan neighbor two hops away also references
/// `a` through an unrelated, already-manifold edge. Duplicating the vertex only for the seed faces
/// would leave that neighbor pointing at the old vertex, orphaning the edge between them (a bug an
/// earlier version of this function had — verified by
/// `repair_splits_bowtie_edge_into_two_components`, which plants exactly this shape).
fn group_faces_by_manifold_connectivity(mesh: &TriMesh, seed_faces: &[u32], edges: &EdgeMap, a: u32, b: u32) -> Vec<Vec<u32>> {
    let mut dsu = DisjointSet::new(mesh.triangles.len());
    for face_list in edges.values() {
        if face_list.len() == 2 {
            dsu.union(face_list[0], face_list[1]);
        }
    }
    let mut roots: Vec<u32> = seed_faces.iter().map(|&f| dsu.find(f)).collect();
    roots.sort_unstable();
    roots.dedup();
    let mut groups: Vec<Vec<u32>> = roots.iter().map(|&root| (0..mesh.triangles.len() as u32).filter(|&f| dsu.find(f) == root && (mesh.triangles[f as usize].contains(&a) || mesh.triangles[f as usize].contains(&b))).collect()).collect();
    groups.sort_by_key(|g| g[0]);
    groups
}

/// 🩹️ Splits non-manifold edges (duplicating vertices per excess fan-group) then non-manifold
/// pinch vertices (duplicating per excess fan), each via
/// [`group_faces_by_manifold_connectivity`], until every edge has at most 2 incident faces and
/// every vertex's incident faces form a single fan.
pub fn repair_non_manifold(mesh: &mut TriMesh) -> RepairStats {
    let mut stats = RepairStats::default();
    loop {
        let edges = mesh.edge_map();
        let offenders: Vec<((u32, u32), Vec<u32>)> = edges.iter().filter(|(_, f)| f.len() > 2).map(|(&k, f)| (k, f.clone())).collect();
        if offenders.is_empty() {
            break;
        }
        for ((a, b), faces) in offenders {
            let groups = group_faces_by_manifold_connectivity(mesh, &faces, &edges, a, b);
            for group in groups.into_iter().skip(1) {
                let new_a = mesh.positions.len() as u32;
                mesh.positions.push(mesh.positions[a as usize]);
                let new_b = mesh.positions.len() as u32;
                mesh.positions.push(mesh.positions[b as usize]);
                stats.vertices_duplicated += 2;
                for f in group {
                    for v in mesh.triangles[f as usize].iter_mut() {
                        if *v == a {
                            *v = new_a;
                        } else if *v == b {
                            *v = new_b;
                        }
                    }
                }
            }
            stats.non_manifold_edges_split += 1;
        }
    }
    loop {
        let mut incident: Vec<Vec<u32>> = vec![Vec::new(); mesh.positions.len()];
        for (f, tri) in mesh.triangles.iter().enumerate() {
            for &v in tri {
                incident[v as usize].push(f as u32);
            }
        }
        let mut split_any = false;
        for v in 0..mesh.positions.len() as u32 {
            let faces = &incident[v as usize];
            if faces.len() <= 1 {
                continue;
            }
            let face_local_index: HashMap<u32, u32> = faces.iter().enumerate().map(|(i, &f)| (f, i as u32)).collect();
            let mut spokes: HashMap<u32, Vec<u32>> = HashMap::new();
            for &f in faces {
                for &w in &mesh.triangles[f as usize] {
                    if w != v {
                        spokes.entry(w).or_default().push(f);
                    }
                }
            }
            let mut dsu = DisjointSet::new(faces.len());
            for spoke_faces in spokes.values() {
                if spoke_faces.len() == 2 {
                    dsu.union(face_local_index[&spoke_faces[0]], face_local_index[&spoke_faces[1]]);
                }
            }
            let mut groups: HashMap<u32, Vec<u32>> = HashMap::new();
            for &f in faces {
                groups.entry(dsu.find(face_local_index[&f])).or_default().push(f);
            }
            if groups.len() <= 1 {
                continue;
            }
            let mut ordered: Vec<Vec<u32>> = groups.into_values().collect();
            ordered.sort_by_key(|g| g[0]);
            for group in ordered.into_iter().skip(1) {
                let new_v = mesh.positions.len() as u32;
                mesh.positions.push(mesh.positions[v as usize]);
                stats.vertices_duplicated += 1;
                for f in group {
                    for slot in mesh.triangles[f as usize].iter_mut() {
                        if *slot == v {
                            *slot = new_v;
                        }
                    }
                }
            }
            stats.non_manifold_vertices_split += 1;
            split_any = true;
            break;
        }
        if !split_any {
            break;
        }
    }
    stats
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrientError {
    UnresolvableConflict,
}

impl std::fmt::Display for OrientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not consistently orient mesh after retry")
    }
}

impl std::error::Error for OrientError {}

/// 🌊️ One iterative (non-recursive) BFS winding flood: returns the set of edges where the flood
/// found two already-visited faces disagreeing on direction.
fn orient_flood(mesh: &mut TriMesh, edges: &EdgeMap) -> Vec<(u32, u32)> {
    let n = mesh.triangles.len();
    let mut visited = vec![false; n];
    let mut conflicts = Vec::new();
    for start in 0..n {
        if visited[start] {
            continue;
        }
        visited[start] = true;
        let mut queue = VecDeque::new();
        queue.push_back(start as u32);
        while let Some(f) = queue.pop_front() {
            let tri = mesh.triangles[f as usize];
            for k in 0..3 {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                let Some(faces) = edges.get(&sorted_edge(a, b)) else { continue };
                if faces.len() != 2 {
                    continue;
                }
                let other = if faces[0] == f { faces[1] } else { faces[0] };
                let other_tri = mesh.triangles[other as usize];
                let other_has_ab = (0..3).any(|k2| other_tri[k2] == a && other_tri[(k2 + 1) % 3] == b);
                if !visited[other as usize] {
                    visited[other as usize] = true;
                    if other_has_ab {
                        mesh.triangles[other as usize].swap(1, 2);
                    }
                    queue.push_back(other);
                } else {
                    let other_tri = mesh.triangles[other as usize];
                    let still_conflicts = (0..3).any(|k2| other_tri[k2] == a && other_tri[(k2 + 1) % 3] == b);
                    if still_conflicts {
                        conflicts.push(sorted_edge(a, b));
                    }
                }
            }
        }
    }
    conflicts
}

/// 🩹️ Duplicates the shared vertices of each listed edge for one of its two faces, turning it into
/// two boundary edges (a legitimate cut, left for [`fill_holes`] or [`close_voxel`] to resolve).
fn split_edges_as_boundary(mesh: &mut TriMesh, conflict_edges: &[(u32, u32)]) {
    let edges = mesh.edge_map();
    for &(a, b) in conflict_edges {
        let Some(faces) = edges.get(&(a, b)) else { continue };
        let Some(&f) = faces.first() else { continue };
        let new_a = mesh.positions.len() as u32;
        mesh.positions.push(mesh.positions[a as usize]);
        let new_b = mesh.positions.len() as u32;
        mesh.positions.push(mesh.positions[b as usize]);
        for v in mesh.triangles[f as usize].iter_mut() {
            if *v == a {
                *v = new_a;
            } else if *v == b {
                *v = new_b;
            }
        }
    }
}

/// 🧭️ Iteratively (BFS, never recursive) floods a consistent winding across manifold edges,
/// flipping disagreeing faces; on an unresolvable conflict (only possible across a non-orientable
/// loop) it cuts those edges once and retries, and only reports [`OrientError`] if the retry also
/// fails — signaling the caller to fall back to [`close_voxel`].
pub fn orient_consistently(mesh: &mut TriMesh) -> Result<(), OrientError> {
    for attempt in 0..2 {
        let edges = mesh.edge_map();
        let conflicts = orient_flood(mesh, &edges);
        if conflicts.is_empty() {
            return Ok(());
        }
        if attempt == 1 {
            return Err(OrientError::UnresolvableConflict);
        }
        split_edges_as_boundary(mesh, &conflicts);
    }
    Err(OrientError::UnresolvableConflict)
}

/// 🧭️ Global outward-normal flip via [`TriMesh::signed_volume`] — only meaningful once the mesh is
/// closed, since an open mesh's signed volume is not a reliable inside/outside signal.
pub fn orient_outward(mesh: &mut TriMesh) {
    if mesh.signed_volume() < 0.0 {
        for tri in &mut mesh.triangles {
            tri.swap(1, 2);
        }
    }
}
// #endregion 🔖️Repair

// #region 🔖️HoleFill
#[derive(Clone, Debug, PartialEq)]
pub struct HoleFillParams {
    pub max_boundary_verts: usize,
}

impl Default for HoleFillParams {
    fn default() -> Self {
        Self { max_boundary_verts: 512 }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HoleFillStats {
    pub holes_filled: usize,
    pub holes_skipped_too_large: usize,
    pub holes_skipped_invalid_patch: usize,
    pub ear_fan_used: usize,
    pub min_weight_dp_used: usize,
    pub advancing_front_used: usize,
    pub advancing_front_capped: usize,
}

fn interior_angle(prev: [f64; 3], cur: [f64; 3], next: [f64; 3]) -> f64 {
    let a = normalize3(sub3(prev, cur));
    let b = normalize3(sub3(next, cur));
    dot3(a, b).clamp(-1.0, 1.0).acos()
}

/// 📐️ Local planar basis for a (roughly planar) boundary loop via Newell's method, used to project
/// the loop to 2D for ear-clipping's point-in-triangle containment tests.
fn loop_basis(positions: &[[f64; 3]]) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let mut normal = [0.0; 3];
    let n = positions.len();
    for i in 0..n {
        let a = positions[i];
        let b = positions[(i + 1) % n];
        normal[0] += (a[1] - b[1]) * (a[2] + b[2]);
        normal[1] += (a[2] - b[2]) * (a[0] + b[0]);
        normal[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    normal = normalize3(normal);
    let helper = if normal[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let ex = normalize3(cross3(helper, normal));
    let ey = cross3(normal, ex);
    (ex, ey, normal)
}

fn to_2d(p: [f64; 3], origin: [f64; 3], ex: [f64; 3], ey: [f64; 3]) -> [f64; 2] {
    let d = sub3(p, origin);
    [dot3(d, ex), dot3(d, ey)]
}

fn point_in_triangle_2d(p: [f64; 2], a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> bool {
    let sign = |p1: [f64; 2], p2: [f64; 2], p3: [f64; 2]| (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1]);
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

/// ✂️ Standard 2D ear-clipping: repeatedly clips the valid convex ear with the smallest interior
/// angle until 3 vertices remain.
fn ear_clip(mut ring: Vec<usize>, pts2d: &[[f64; 2]]) -> Vec<[usize; 3]> {
    let mut tris = Vec::new();
    while ring.len() > 3 {
        let n = ring.len();
        let mut best: Option<(usize, f64)> = None;
        for i in 0..n {
            let (prev, cur, next) = (ring[(i + n - 1) % n], ring[i], ring[(i + 1) % n]);
            let (a, b, c) = (pts2d[prev], pts2d[cur], pts2d[next]);
            let cross_z = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if cross_z <= 0.0 {
                continue;
            }
            let contains_other = ring.iter().enumerate().any(|(j, &v)| j != (i + n - 1) % n && j != i && j != (i + 1) % n && point_in_triangle_2d(pts2d[v], a, b, c));
            if contains_other {
                continue;
            }
            let ang = interior_angle([a[0], a[1], 0.0], [b[0], b[1], 0.0], [c[0], c[1], 0.0]);
            if best.is_none_or(|(_, best_ang)| ang < best_ang) {
                best = Some((i, ang));
            }
        }
        let Some((i, _)) = best else {
            let n = ring.len();
            let idx = (n / 3).clamp(1, n - 1);
            tris.push([ring[0], ring[idx], ring[(2 * idx).min(n - 1)]]);
            ring.remove(idx);
            continue;
        };
        let n = ring.len();
        tris.push([ring[(i + n - 1) % n], ring[i], ring[(i + 1) % n]]);
        ring.remove(i);
    }
    if ring.len() == 3 {
        tris.push([ring[0], ring[1], ring[2]]);
    }
    tris
}

#[derive(Clone, Copy, PartialEq)]
struct DpWeight {
    max_dihedral: f64,
    area: f64,
}

impl DpWeight {
    const ZERO: Self = Self { max_dihedral: 0.0, area: 0.0 };

    fn combine(a: Self, b: Self, this_dihedral: f64, this_area: f64) -> Self {
        Self { max_dihedral: a.max_dihedral.max(b.max_dihedral).max(this_dihedral), area: a.area + b.area + this_area }
    }

    fn better_than(&self, other: &Self) -> bool {
        (self.max_dihedral, self.area) < (other.max_dihedral, other.area)
    }
}

fn triangle_normal(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> [f64; 3] {
    normalize3(cross3(sub3(b, a), sub3(c, a)))
}

/// 📐️ Klincsek/Barequet–Sharir O(n³) minimum-weight triangulation of a simple polygon (given as a
/// boundary ring in 3D), Liepa-weighted: primarily minimizes the worst dihedral angle any new
/// diagonal introduces relative to its two sub-triangulations, tie-broken by total area.
fn min_weight_triangulation(ring: &[usize], positions: &[[f64; 3]]) -> Vec<[usize; 3]> {
    let n = ring.len();
    if n < 3 {
        return Vec::new();
    }
    let p = |i: usize| positions[ring[i]];
    let mut cost = vec![vec![DpWeight::ZERO; n]; n];
    let mut normal = vec![vec![[0.0; 3]; n]; n];
    let mut split = vec![vec![0usize; n]; n];
    for len in 2..n {
        for i in 0..(n - len) {
            let j = i + len;
            let mut best: Option<DpWeight> = None;
            let mut best_k = i + 1;
            for k in (i + 1)..j {
                let tri_normal = triangle_normal(p(i), p(k), p(j));
                let tri_area = 0.5 * norm3(cross3(sub3(p(k), p(i)), sub3(p(j), p(i))));
                let dihedral_left = if k > i + 1 { (1.0 - dot3(tri_normal, normal[i][k])).max(0.0) } else { 0.0 };
                let dihedral_right = if j > k + 1 { (1.0 - dot3(tri_normal, normal[k][j])).max(0.0) } else { 0.0 };
                let this_dihedral = dihedral_left.max(dihedral_right);
                let candidate = DpWeight::combine(cost[i][k], cost[k][j], this_dihedral, tri_area);
                if best.is_none_or(|b| candidate.better_than(&b)) {
                    best = Some(candidate);
                    best_k = k;
                    normal[i][j] = tri_normal;
                }
            }
            cost[i][j] = best.unwrap_or(DpWeight::ZERO);
            split[i][j] = best_k;
        }
    }
    let mut tris = Vec::new();
    let mut stack = vec![(0usize, n - 1)];
    while let Some((i, j)) = stack.pop() {
        if j - i < 2 {
            continue;
        }
        let k = split[i][j];
        tris.push([ring[i], ring[k], ring[j]]);
        stack.push((i, k));
        stack.push((k, j));
    }
    tris
}

/// 🌊️ Advancing-front triangulation for large loops: repeatedly ear-clips the vertex with the
/// smallest interior angle; when the smallest available angle still exceeds 135°, inserts one new
/// averaged-position vertex instead to keep triangles from becoming slivers. Hard-capped at
/// `max_iterations` so it can never hang — capping out is a legitimate, expected outcome that
/// signals the caller to fall back to [`close_voxel`], not a bug.
fn advancing_front_triangulate(loop_verts: &[u32], positions: &mut Vec<[f64; 3]>, max_iterations: usize) -> (Vec<[u32; 3]>, bool) {
    let initial_positions: Vec<[f64; 3]> = loop_verts.iter().map(|&v| positions[v as usize]).collect();
    let (ex, ey, _) = loop_basis(&initial_positions);
    let origin = initial_positions[0];
    let mut ring: Vec<u32> = loop_verts.to_vec();
    let mut tris = Vec::new();
    let mut iterations = 0usize;
    while ring.len() > 3 {
        iterations += 1;
        if iterations > max_iterations {
            return (tris, true);
        }
        let n = ring.len();
        let pts2d: Vec<[f64; 2]> = ring.iter().map(|&v| to_2d(positions[v as usize], origin, ex, ey)).collect();
        let mut candidates: Vec<(usize, f64)> = (0..n)
            .map(|i| {
                let (prev, cur, next) = (positions[ring[(i + n - 1) % n] as usize], positions[ring[i] as usize], positions[ring[(i + 1) % n] as usize]);
                (i, interior_angle(prev, cur, next))
            })
            .collect();
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("finite angle"));
        let is_valid_ear = |i: usize| -> bool {
            let (prev_i, next_i) = ((i + n - 1) % n, (i + 1) % n);
            let (a, b, c) = (pts2d[prev_i], pts2d[i], pts2d[next_i]);
            let cross_z = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if cross_z <= 1e-15 {
                return false;
            }
            !(0..n).any(|j| j != prev_i && j != i && j != next_i && point_in_triangle_2d(pts2d[j], a, b, c))
        };
        let picked = candidates.iter().find(|&&(i, angle)| angle <= 135f64.to_radians() && is_valid_ear(i)).map(|&(i, _)| i);
        let (best_i, best_angle) = match picked {
            Some(i) => (i, candidates.iter().find(|&&(idx, _)| idx == i).expect("picked index present").1),
            None => candidates[0],
        };
        let prev_i = (best_i + n - 1) % n;
        let next_i = (best_i + 1) % n;
        if best_angle > 135f64.to_radians() || picked.is_none() {
            let mid = lerp3(positions[ring[prev_i] as usize], positions[ring[next_i] as usize], 0.5);
            let new_id = positions.len() as u32;
            positions.push(mid);
            tris.push([ring[prev_i], ring[best_i], new_id]);
            tris.push([ring[best_i], ring[next_i], new_id]);
            ring[best_i] = new_id;
        } else {
            tris.push([ring[prev_i], ring[best_i], ring[next_i]]);
            ring.remove(best_i);
        }
    }
    if ring.len() == 3 {
        tris.push([ring[0], ring[1], ring[2]]);
    }
    (tris, false)
}

/// 🪢️ Uniform-Laplacian fairing of newly introduced interior vertices (fixed boundary/original
/// vertices), solved per axis via `CsrMatrix` + `conjugate_gradient` over the local patch only.
fn fair_new_vertices(mesh: &mut TriMesh, patch_faces: &[u32], new_vertices: &HashSet<u32>) {
    if new_vertices.is_empty() {
        return;
    }
    let mut neighbors: HashMap<u32, HashSet<u32>> = HashMap::new();
    for &f in patch_faces {
        let tri = mesh.triangles[f as usize];
        for k in 0..3 {
            let (a, b) = (tri[k], tri[(k + 1) % 3]);
            neighbors.entry(a).or_default().insert(b);
            neighbors.entry(b).or_default().insert(a);
        }
    }
    let free: Vec<u32> = new_vertices.iter().copied().collect();
    let index_of: HashMap<u32, usize> = free.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let n = free.len();
    let mut triplets = Vec::new();
    let mut rhs = [math::algebra::VecD::zeros(n), math::algebra::VecD::zeros(n), math::algebra::VecD::zeros(n)];
    for (row, &v) in free.iter().enumerate() {
        let nbrs = neighbors.get(&v).cloned().unwrap_or_default();
        let degree = nbrs.len().max(1) as f64;
        triplets.push((row, row, degree));
        for nb in nbrs {
            if let Some(&col) = index_of.get(&nb) {
                triplets.push((row, col, -1.0));
            } else {
                let p = mesh.positions[nb as usize];
                for (axis, rhs_axis) in rhs.iter_mut().enumerate() {
                    rhs_axis.add_at(row, p[axis]);
                }
            }
        }
    }
    let a = math::algebra::CsrMatrix::from_triplets(n, n, &triplets);
    for (axis, rhs_axis) in rhs.iter().enumerate() {
        if let Ok(x) = math::algebra::conjugate_gradient(&a, rhs_axis, 1e-8, 500) {
            for (row, &v) in free.iter().enumerate() {
                mesh.positions[v as usize][axis] = x.get(row);
            }
        }
    }
}

/// 🧭️ Flips every new triangle's winding if needed so the patch matches the rest of the mesh's
/// orientation: finds the (single, pre-existing) face across the loop's first edge, and requires
/// the patch to traverse that same edge in the opposite direction — the standard "shared edges
/// wind oppositely" consistency rule, checked here against real mesh data rather than assumed
/// from `boundary_loops`' walk direction alone.
fn fix_patch_orientation(mesh: &TriMesh, loop_verts: &[u32], new_tris: &mut [[u32; 3]]) {
    let (a, b) = (loop_verts[0], loop_verts[1]);
    let edges = mesh.edge_map();
    let Some(faces) = edges.get(&sorted_edge(a, b)) else { return };
    let Some(&existing_face) = faces.first() else { return };
    let existing_tri = mesh.triangles[existing_face as usize];
    let existing_ab_forward = (0..3).any(|k| existing_tri[k] == a && existing_tri[(k + 1) % 3] == b);
    let mut patch_ab_forward: Option<bool> = None;
    for tri in new_tris.iter() {
        for k in 0..3 {
            if tri[k] == a && tri[(k + 1) % 3] == b {
                patch_ab_forward = Some(true);
            } else if tri[k] == b && tri[(k + 1) % 3] == a {
                patch_ab_forward = Some(false);
            }
        }
    }
    let Some(patch_forward) = patch_ab_forward else { return };
    if patch_forward == existing_ab_forward {
        for tri in new_tris.iter_mut() {
            tri.swap(1, 2);
        }
    }
}

/// 🛡️ Verifies a candidate hole-fill patch is a valid, non-degenerate, manifold-compatible
/// triangulation of `loop_verts` before it is committed to the mesh: no repeated-vertex or
/// zero-area triangles, every *internal* patch edge appears exactly twice within the patch, and
/// every *boundary* edge (an original loop edge) appears exactly once (matching the single
/// pre-existing outside face it closes off, so the loop truly becomes interior once merged).
/// Rejecting an invalid patch here — leaving that loop open for [`close_voxel`] to handle — is
/// strictly safer than committing a self-intersecting or non-manifold triangulation; verified by
/// `planted_holes_fill_and_trigger_all_three_strategies`, which caught exactly this failure mode
/// in an earlier version of [`advancing_front_triangulate`].
fn patch_is_valid(mesh: &TriMesh, loop_verts: &[u32], new_tris: &[[u32; 3]]) -> bool {
    if new_tris.is_empty() {
        return false;
    }
    for tri in new_tris {
        if tri[0] == tri[1] || tri[1] == tri[2] || tri[0] == tri[2] {
            return false;
        }
        let (a, b, c) = (mesh.positions[tri[0] as usize], mesh.positions[tri[1] as usize], mesh.positions[tri[2] as usize]);
        if norm3(cross3(sub3(b, a), sub3(c, a))) < 1e-14 {
            return false;
        }
    }
    let n = loop_verts.len();
    let boundary_edges: HashSet<(u32, u32)> = (0..n).map(|i| sorted_edge(loop_verts[i], loop_verts[(i + 1) % n])).collect();
    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();
    for tri in new_tris {
        for k in 0..3 {
            *counts.entry(sorted_edge(tri[k], tri[(k + 1) % 3])).or_insert(0) += 1;
        }
    }
    for (edge, &count) in &counts {
        let expected = if boundary_edges.contains(edge) { 1 } else { 2 };
        if count != expected {
            return false;
        }
    }
    boundary_edges.iter().all(|e| counts.get(e).copied() == Some(1))
}

/// 🕳️ Fills every boundary loop, dispatching by size: `<= 8` ear-fan, `9..=64` minimum-weight DP,
/// `> 64` advancing front (each of which increments its own [`HoleFillStats`] counter). Loops
/// larger than `params.max_boundary_verts` are left unfilled — deliberately, so
/// [`mesh_pipeline_step`] can treat "hole too large to responsibly fill" as an unambiguous,
/// deterministic trigger for the [`close_voxel`] guarantee rather than an organic algorithm
/// failure. Every candidate patch is also checked via [`patch_is_valid`] before being committed;
/// an invalid patch is discarded (loop left open) rather than risking mesh corruption.
pub fn fill_holes(mesh: &mut TriMesh, params: &HoleFillParams) -> HoleFillStats {
    let mut stats = HoleFillStats::default();
    let Ok(topology) = HalfedgeTopology::build(mesh) else { return stats };
    let mut loops = topology.boundary_loops();
    loops.sort_by_key(|l| l[0]);
    for loop_verts in loops {
        let n = loop_verts.len();
        if n < 3 {
            continue;
        }
        if n > params.max_boundary_verts {
            stats.holes_skipped_too_large += 1;
            continue;
        }
        let positions_before = mesh.positions.len();
        let mut new_vertices: HashSet<u32> = HashSet::new();
        let (mut new_tris, strategy_used, capped): (Vec<[u32; 3]>, u8, bool) = if n <= 8 {
            let positions: Vec<[f64; 3]> = loop_verts.iter().map(|&v| mesh.positions[v as usize]).collect();
            let (ex, ey, _) = loop_basis(&positions);
            let origin = positions[0];
            let pts2d: Vec<[f64; 2]> = positions.iter().map(|&p| to_2d(p, origin, ex, ey)).collect();
            let ring: Vec<usize> = (0..n).collect();
            let local_tris = ear_clip(ring, &pts2d);
            (local_tris.into_iter().map(|t| [loop_verts[t[0]], loop_verts[t[1]], loop_verts[t[2]]]).collect(), 0, false)
        } else if n <= 64 {
            let ring: Vec<usize> = (0..n).collect();
            let positions: Vec<[f64; 3]> = loop_verts.iter().map(|&v| mesh.positions[v as usize]).collect();
            let local_tris = min_weight_triangulation(&ring, &positions);
            (local_tris.into_iter().map(|t| [loop_verts[t[0]], loop_verts[t[1]], loop_verts[t[2]]]).collect(), 1, false)
        } else {
            let (tris, capped) = advancing_front_triangulate(&loop_verts, &mut mesh.positions, n * 8);
            (tris, 2, capped)
        };
        match strategy_used {
            0 => stats.ear_fan_used += 1,
            1 => stats.min_weight_dp_used += 1,
            _ => stats.advancing_front_used += 1,
        }
        if capped {
            stats.advancing_front_capped += 1;
        }
        fix_patch_orientation(mesh, &loop_verts, &mut new_tris);
        if !patch_is_valid(mesh, &loop_verts, &new_tris) {
            mesh.positions.truncate(positions_before);
            stats.holes_skipped_invalid_patch += 1;
            continue;
        }
        let known: HashSet<u32> = loop_verts.iter().copied().collect();
        for tri in &new_tris {
            for &v in tri {
                if !known.contains(&v) {
                    new_vertices.insert(v);
                }
            }
        }
        let start_face = mesh.triangles.len() as u32;
        mesh.triangles.extend(new_tris);
        let patch_faces: Vec<u32> = (start_face..mesh.triangles.len() as u32).collect();
        fair_new_vertices(mesh, &patch_faces, &new_vertices);
        stats.holes_filled += 1;
    }
    stats
}
// #endregion 🔖️HoleFill

// #region 🔖️Close
/// 📦️ Axis-aligned box vs. triangle overlap via the Akenine-Möller separating-axis test: 3 box-face
/// axes, the triangle's own normal, and the 9 cross products of the triangle's edges with the box
/// axes — 13 candidate separating axes total, each tested by the same generic projection-interval
/// check. Conservative by construction (a triangle merely touching the box counts as overlapping),
/// which is exactly the property [`close_voxel`]'s guarantee depends on.
fn tri_box_overlap(box_center: [f64; 3], box_half: [f64; 3], tri: [[f64; 3]; 3]) -> bool {
    let v = [sub3(tri[0], box_center), sub3(tri[1], box_center), sub3(tri[2], box_center)];
    let e = [sub3(v[1], v[0]), sub3(v[2], v[1]), sub3(v[0], v[2])];
    let separates = |axis: [f64; 3]| -> bool {
        let r = box_half[0] * axis[0].abs() + box_half[1] * axis[1].abs() + box_half[2] * axis[2].abs();
        if r < 1e-15 {
            return false;
        }
        let p = [dot3(axis, v[0]), dot3(axis, v[1]), dot3(axis, v[2])];
        let (min_p, max_p) = (p[0].min(p[1]).min(p[2]), p[0].max(p[1]).max(p[2]));
        min_p > r || max_p < -r
    };
    let box_axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    for axis in box_axes {
        if separates(axis) {
            return false;
        }
    }
    let tri_normal = cross3(e[0], e[1]);
    if separates(tri_normal) {
        return false;
    }
    for edge in e {
        for axis in box_axes {
            let cross_axis = cross3(edge, axis);
            if norm3(cross_axis) > 1e-15 && separates(cross_axis) {
                return false;
            }
        }
    }
    true
}

/// 🛡️ Guaranteed-watertight fallback: conservatively rasterizes every triangle into a padded dense
/// occupancy grid, floods "outside" inward from the (by-construction all-outside) grid border, and
/// re-extracts a closed 2-manifold via [`extract_dense_grid`] over the resulting total `±1` field.
/// Because the grid is padded well beyond the mesh's own bounding box, the border is guaranteed
/// unoccupied *before* any rasterization happens, so the outside flood always reaches every border
/// cell and the field never has an "unknown" region — this is what makes the output closed by
/// construction rather than by luck. The requested `voxel` is floored at `bbox_diagonal / 64` so
/// this guaranteed-termination fallback can never be handed a voxel size fine enough to make its
/// own grid intractable — the "always watertight" contract should never be a performance landmine.
pub fn close_voxel(mesh: &TriMesh, voxel: f64) -> TriMesh {
    if mesh.triangles.is_empty() || voxel <= 0.0 {
        return TriMesh::new();
    }
    let (lo, hi) = mesh.bbox();
    let diag = norm3(sub3(hi, lo)).max(1e-9);
    let max_cells_per_axis = 64.0;
    let voxel = voxel.max(diag / max_cells_per_axis);
    let pad = 3usize;
    let dims = |lo: f64, hi: f64| -> usize { ((hi - lo) / voxel).ceil() as usize + 2 * pad + 2 };
    let (nx, ny, nz) = (dims(lo[0], hi[0]), dims(lo[1], hi[1]), dims(lo[2], hi[2]));
    let origin = sub3(lo, [pad as f64 * voxel, pad as f64 * voxel, pad as f64 * voxel]);
    let mut occupied = DenseField::new(nx, ny, nz, origin, voxel, 0.0);
    let half = [voxel * 0.5; 3];
    for tri in &mesh.triangles {
        let p = [mesh.positions[tri[0] as usize], mesh.positions[tri[1] as usize], mesh.positions[tri[2] as usize]];
        let mut tlo = [f64::INFINITY; 3];
        let mut thi = [f64::NEG_INFINITY; 3];
        for pt in p {
            for k in 0..3 {
                tlo[k] = tlo[k].min(pt[k]);
                thi[k] = thi[k].max(pt[k]);
            }
        }
        let to_cell = |w: f64, o: f64| -> i32 { ((w - o) / voxel).floor() as i32 };
        let (i0, j0, k0) = (to_cell(tlo[0], origin[0]) - 1, to_cell(tlo[1], origin[1]) - 1, to_cell(tlo[2], origin[2]) - 1);
        let (i1, j1, k1) = (to_cell(thi[0], origin[0]) + 1, to_cell(thi[1], origin[1]) + 1, to_cell(thi[2], origin[2]) + 1);
        for i in i0.max(0)..=i1.min(nx as i32 - 1) {
            for j in j0.max(0)..=j1.min(ny as i32 - 1) {
                for k in k0.max(0)..=k1.min(nz as i32 - 1) {
                    let center = add3(origin, scale3([i as f64 + 0.5, j as f64 + 0.5, k as f64 + 0.5], voxel));
                    if tri_box_overlap(center, half, p) {
                        occupied.set(i, j, k, 1.0);
                    }
                }
            }
        }
    }
    let mut outside = vec![false; nx * ny * nz];
    let mut queue = VecDeque::new();
    let seed = |i: i32, j: i32, k: i32, outside: &mut Vec<bool>, queue: &mut VecDeque<(i32, i32, i32)>| {
        if occupied.get(i, j, k) == Some(0.0) {
            let idx = occupied.index(i, j, k).expect("in-range seed");
            if !outside[idx] {
                outside[idx] = true;
                queue.push_back((i, j, k));
            }
        }
    };
    for i in 0..nx as i32 {
        for j in 0..ny as i32 {
            seed(i, j, 0, &mut outside, &mut queue);
            seed(i, j, nz as i32 - 1, &mut outside, &mut queue);
        }
    }
    for i in 0..nx as i32 {
        for k in 0..nz as i32 {
            seed(i, 0, k, &mut outside, &mut queue);
            seed(i, ny as i32 - 1, k, &mut outside, &mut queue);
        }
    }
    for j in 0..ny as i32 {
        for k in 0..nz as i32 {
            seed(0, j, k, &mut outside, &mut queue);
            seed(nx as i32 - 1, j, k, &mut outside, &mut queue);
        }
    }
    let neighbor_offsets = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];
    while let Some((i, j, k)) = queue.pop_front() {
        for (di, dj, dk) in neighbor_offsets {
            let (ni, nj, nk) = (i + di, j + dj, k + dk);
            if occupied.get(ni, nj, nk) == Some(0.0) {
                let idx = occupied.index(ni, nj, nk).expect("in-range neighbor");
                if !outside[idx] {
                    outside[idx] = true;
                    queue.push_back((ni, nj, nk));
                }
            }
        }
    }
    let mut field = DenseField::new(nx, ny, nz, origin, voxel, -1.0);
    for (idx, value) in field.values.iter_mut().enumerate() {
        if outside[idx] {
            *value = 1.0;
        }
    }
    let mut closed = extract_dense_grid(&field, 0.0);
    orient_outward(&mut closed);
    closed
}
// #endregion 🔖️Close

// #region 🔖️Validate
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WatertightReport {
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub boundary_edge_count: usize,
    pub boundary_loop_count: usize,
    pub non_manifold_edge_count: usize,
    pub non_manifold_vertex_count: usize,
    pub connected_components: usize,
    pub consistently_oriented: bool,
    pub euler_characteristic: i64,
    pub genus: Option<i64>,
    pub signed_volume: f64,
    pub self_intersection_pairs: Option<usize>,
    pub closed_fallback_used: bool,
    pub is_closed: bool,
    pub is_two_manifold: bool,
    pub is_watertight: bool,
}

fn count_non_manifold_vertices(mesh: &TriMesh, edges: &EdgeMap) -> usize {
    let mut incident: Vec<Vec<u32>> = vec![Vec::new(); mesh.positions.len()];
    for (f, tri) in mesh.triangles.iter().enumerate() {
        for &v in tri {
            incident[v as usize].push(f as u32);
        }
    }
    let mut manifold_dsu = DisjointSet::new(mesh.triangles.len());
    for faces in edges.values() {
        if faces.len() == 2 {
            manifold_dsu.union(faces[0], faces[1]);
        }
    }
    let mut count = 0usize;
    for faces in &incident {
        if faces.len() <= 1 {
            continue;
        }
        let root = manifold_dsu.find(faces[0]);
        if faces.iter().any(|&f| manifold_dsu.find(f) != root) {
            count += 1;
        }
    }
    count
}

fn count_boundary_loops(edges: &EdgeMap, vertex_count: usize) -> usize {
    let boundary_edges: Vec<(u32, u32)> = edges.iter().filter(|(_, f)| f.len() == 1).map(|(&k, _)| k).collect();
    if boundary_edges.is_empty() {
        return 0;
    }
    let mut dsu = DisjointSet::new(vertex_count);
    for &(a, b) in &boundary_edges {
        dsu.union(a, b);
    }
    let roots: HashSet<u32> = boundary_edges.iter().flat_map(|&(a, b)| [dsu.find(a), dsu.find(b)]).collect();
    roots.len()
}

fn check_consistently_oriented(mesh: &TriMesh, edges: &EdgeMap) -> bool {
    for faces in edges.values() {
        if faces.len() != 2 {
            continue;
        }
        let (ta, tb) = (mesh.triangles[faces[0] as usize], mesh.triangles[faces[1] as usize]);
        let dir_a: HashSet<(u32, u32)> = (0..3).map(|k| (ta[k], ta[(k + 1) % 3])).collect();
        let dir_b: HashSet<(u32, u32)> = (0..3).map(|k| (tb[k], tb[(k + 1) % 3])).collect();
        if dir_a.intersection(&dir_b).next().is_some() {
            return false;
        }
    }
    true
}

/// ⚔️ Möller's fast triangle-triangle intersection test (1997): computes each triangle's signed
/// distances to the other's plane, rejects when both are strictly same-signed, otherwise reduces
/// to a 1D interval overlap test along the two planes' intersection line.
fn triangles_intersect(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> bool {
    let plane = |t: [[f64; 3]; 3]| -> ([f64; 3], f64) {
        let n = cross3(sub3(t[1], t[0]), sub3(t[2], t[0]));
        (n, -dot3(n, t[0]))
    };
    let signed_dists = |t: [[f64; 3]; 3], n: [f64; 3], d: f64| -> [f64; 3] { std::array::from_fn(|i| dot3(n, t[i]) + d) };
    let (na, da) = plane(a);
    let db = signed_dists(b, na, da);
    if db.iter().all(|&d| d > 1e-9) || db.iter().all(|&d| d < -1e-9) {
        return false;
    }
    let (nb, db_plane) = plane(b);
    let da_vals = signed_dists(a, nb, db_plane);
    if da_vals.iter().all(|&d| d > 1e-9) || da_vals.iter().all(|&d| d < -1e-9) {
        return false;
    }
    let line_dir = cross3(na, nb);
    if norm3(line_dir) < 1e-12 {
        return false;
    }
    let interval = |t: [[f64; 3]; 3], dists: [f64; 3]| -> (f64, f64) {
        let t_param: Vec<f64> = (0..3).map(|i| dot3(t[i], line_dir)).collect();
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for i in 0..3 {
            let j = (i + 1) % 3;
            if (dists[i] > 0.0) != (dists[j] > 0.0) {
                let frac = dists[i] / (dists[i] - dists[j]);
                let t_cross = t_param[i] + frac * (t_param[j] - t_param[i]);
                lo = lo.min(t_cross);
                hi = hi.max(t_cross);
            }
        }
        if !lo.is_finite() {
            let mn = t_param.iter().copied().fold(f64::INFINITY, f64::min);
            let mx = t_param.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            (mn, mx)
        } else {
            (lo, hi)
        }
    };
    let (a_lo, a_hi) = interval(a, da_vals);
    let (b_lo, b_hi) = interval(b, db);
    a_lo <= b_hi + 1e-9 && b_lo <= a_hi + 1e-9
}

fn count_self_intersections(mesh: &TriMesh) -> usize {
    if mesh.triangles.is_empty() {
        return 0;
    }
    let centroids: Vec<[f64; 3]> = (0..mesh.triangles.len()).map(|f| mesh.face_centroid(f)).collect();
    let tree = math::spatial::KdTree::<3>::build(&centroids);
    let max_edge: f64 = mesh
        .triangles
        .iter()
        .flat_map(|tri| {
            let (a, b, c) = (mesh.positions[tri[0] as usize], mesh.positions[tri[1] as usize], mesh.positions[tri[2] as usize]);
            [norm3(sub3(a, b)), norm3(sub3(b, c)), norm3(sub3(c, a))]
        })
        .fold(0.0_f64, f64::max);
    let radius = (2.0 * max_edge).max(1e-9);
    let mut seen: HashSet<(u32, u32)> = HashSet::new();
    let mut count = 0usize;
    for f in 0..mesh.triangles.len() as u32 {
        for (g, _) in tree.radius(&centroids[f as usize], radius) {
            if g <= f {
                continue;
            }
            let (ta, tb) = (mesh.triangles[f as usize], mesh.triangles[g as usize]);
            if ta.iter().any(|v| tb.contains(v)) {
                continue;
            }
            let key = (f, g);
            if seen.contains(&key) {
                continue;
            }
            let pa = [mesh.positions[ta[0] as usize], mesh.positions[ta[1] as usize], mesh.positions[ta[2] as usize]];
            let pb = [mesh.positions[tb[0] as usize], mesh.positions[tb[1] as usize], mesh.positions[tb[2] as usize]];
            if triangles_intersect(pa, pb) {
                seen.insert(key);
                count += 1;
            }
        }
    }
    count
}

/// 🔍️ The single source of truth for "is this mesh watertight": closed (no boundary), a true
/// 2-manifold (no over-used edges or pinch vertices), and consistently oriented. `genus` is only
/// reported for a single-component closed 2-manifold, where `χ = 2 - 2g` is well-defined.
pub fn validate_watertight(mesh: &TriMesh, check_self_intersections: bool) -> WatertightReport {
    let edges = mesh.edge_map();
    let boundary_edge_count = edges.values().filter(|f| f.len() == 1).count();
    let non_manifold_edge_count = edges.values().filter(|f| f.len() > 2).count();
    let non_manifold_vertex_count = count_non_manifold_vertices(mesh, &edges);
    let boundary_loop_count = count_boundary_loops(&edges, mesh.positions.len());
    let mut dsu = DisjointSet::new(mesh.triangles.len());
    for faces in edges.values() {
        if faces.len() == 2 {
            dsu.union(faces[0], faces[1]);
        }
    }
    let connected_components = if mesh.triangles.is_empty() { 0 } else { (0..mesh.triangles.len() as u32).map(|f| dsu.find(f)).collect::<HashSet<_>>().len() };
    let consistently_oriented = check_consistently_oriented(mesh, &edges);
    let used_vertices: HashSet<u32> = mesh.triangles.iter().flatten().copied().collect();
    let euler_characteristic = used_vertices.len() as i64 - edges.len() as i64 + mesh.triangles.len() as i64;
    let is_closed = boundary_edge_count == 0;
    let is_two_manifold = non_manifold_edge_count == 0 && non_manifold_vertex_count == 0;
    let genus = if is_closed && is_two_manifold && connected_components == 1 { Some((2 - euler_characteristic) / 2) } else { None };
    let self_intersection_pairs = if check_self_intersections { Some(count_self_intersections(mesh)) } else { None };
    WatertightReport {
        vertex_count: mesh.positions.len(),
        triangle_count: mesh.triangles.len(),
        boundary_edge_count,
        boundary_loop_count,
        non_manifold_edge_count,
        non_manifold_vertex_count,
        connected_components,
        consistently_oriented,
        euler_characteristic,
        genus,
        signed_volume: mesh.signed_volume(),
        self_intersection_pairs,
        closed_fallback_used: false,
        is_closed,
        is_two_manifold,
        is_watertight: is_closed && is_two_manifold && consistently_oriented,
    }
}
// #endregion 🔖️Validate

// #region 🔖️Simplify
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SimplifyStats {
    pub collapses_performed: usize,
    pub collapses_rejected_by_link_condition: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimplifyParams {
    pub max_error: f64,
}

impl Default for SimplifyParams {
    fn default() -> Self {
        Self { max_error: f64::INFINITY }
    }
}

/// 🧮️ Symmetric 4x4 quadric `p pᵀ` stored as its 10 upper-triangular entries, accumulated per
/// vertex from incident-face plane quadrics (Garland-Heckbert).
#[derive(Clone, Copy, Debug, Default)]
struct Quadric([f64; 10]);

impl Quadric {
    fn from_plane(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self([a * a, a * b, a * c, a * d, b * b, b * c, b * d, c * c, c * d, d * d])
    }

    fn add(&self, other: &Self) -> Self {
        let mut out = [0.0; 10];
        for (i, slot) in out.iter_mut().enumerate() {
            *slot = self.0[i] + other.0[i];
        }
        Self(out)
    }

    fn error_at(&self, p: [f64; 3]) -> f64 {
        let [a, b, c] = p;
        let q = self.0;
        q[0] * a * a + 2.0 * q[1] * a * b + 2.0 * q[2] * a * c + 2.0 * q[3] * a + q[4] * b * b + 2.0 * q[5] * b * c + 2.0 * q[6] * b + q[7] * c * c + 2.0 * q[8] * c + q[9]
    }

    fn optimal_point(&self, fallback_a: [f64; 3], fallback_b: [f64; 3]) -> [f64; 3] {
        let q = self.0;
        let m = matd_from_rows(&[vec![q[0], q[1], q[2]], vec![q[1], q[4], q[5]], vec![q[2], q[5], q[7]]]);
        let rhs = math::algebra::VecD::from_vec(vec![-q[3], -q[6], -q[8]]);
        if let Ok(l) = math::algebra::cholesky(&m) {
            let x = math::algebra::cholesky_solve(&l, &rhs);
            return [x.get(0), x.get(1), x.get(2)];
        }
        let mid = lerp3(fallback_a, fallback_b, 0.5);
        let candidates = [fallback_a, fallback_b, mid];
        *candidates.iter().min_by(|&&p, &&q2| self.error_at(p).partial_cmp(&self.error_at(q2)).expect("finite quadric error")).expect("nonempty candidates")
    }
}

/// 🔗️ The link-condition guard: an edge collapse `(u, v)` preserves manifoldness only if every
/// vertex adjacent to both `u` and `v` is also a shared face's opposite vertex — i.e. `link(u) ∩
/// link(v) == link(edge(u,v))`. Any extra shared neighbor means the two vertices are connected
/// through some other path too, and collapsing would pinch unrelated mesh sheets together.
fn link_condition_holds(vertex_faces: &[HashSet<u32>], triangles: &[[u32; 3]], u: u32, v: u32) -> bool {
    let neighbors_of = |x: u32| -> HashSet<u32> {
        let mut set = HashSet::new();
        for &f in &vertex_faces[x as usize] {
            for &w in &triangles[f as usize] {
                if w != x {
                    set.insert(w);
                }
            }
        }
        set
    };
    let (nu, nv) = (neighbors_of(u), neighbors_of(v));
    let shared: HashSet<u32> = nu.intersection(&nv).copied().collect();
    let mut expected: HashSet<u32> = HashSet::new();
    for &f in &vertex_faces[u as usize] {
        let tri = triangles[f as usize];
        if tri.contains(&v) {
            for &w in &tri {
                if w != u && w != v {
                    expected.insert(w);
                }
            }
        }
    }
    shared == expected
}

#[derive(Clone, Copy, PartialEq)]
struct HeapEntry {
    cost: f64,
    a: u32,
    b: u32,
    version_a: u32,
    version_b: u32,
}

impl Eq for HeapEntry {}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// 🪚️ Garland-Heckbert quadric-error-metric edge collapse down to `target_triangles`, guarded by
/// [`link_condition_holds`] on every candidate collapse — so simplifying an already-watertight
/// mesh is provably guaranteed to keep it watertight (proven in `mod tests`, not just asserted).
pub fn simplify_qem(mesh: &mut TriMesh, target_triangles: usize, params: &SimplifyParams) -> SimplifyStats {
    let mut stats = SimplifyStats::default();
    if mesh.triangles.len() <= target_triangles {
        return stats;
    }
    let n = mesh.positions.len();
    let mut alive_vertex = vec![true; n];
    let mut alive_face = vec![true; mesh.triangles.len()];
    let mut vertex_faces: Vec<HashSet<u32>> = vec![HashSet::new(); n];
    for (f, tri) in mesh.triangles.iter().enumerate() {
        for &v in tri {
            vertex_faces[v as usize].insert(f as u32);
        }
    }
    let mut vertex_version = vec![0u32; n];
    let compute_quadric = |vertex_faces: &[HashSet<u32>], triangles: &[[u32; 3]], positions: &[[f64; 3]], v: u32| -> Quadric {
        let mut q = Quadric::default();
        for &f in &vertex_faces[v as usize] {
            let tri = triangles[f as usize];
            let (a, b, c) = (positions[tri[0] as usize], positions[tri[1] as usize], positions[tri[2] as usize]);
            let normal = cross3(sub3(b, a), sub3(c, a));
            let area = norm3(normal) * 0.5;
            if area < 1e-15 {
                continue;
            }
            let n = normalize3(normal);
            let d = -dot3(n, a);
            q = q.add(&Quadric::from_plane(n[0], n[1], n[2], d));
        }
        q
    };
    let mut quadrics: Vec<Quadric> = (0..n as u32).map(|v| compute_quadric(&vertex_faces, &mesh.triangles, &mesh.positions, v)).collect();
    let mut heap: BinaryHeap<HeapEntry> = BinaryHeap::new();
    let push_edge = |heap: &mut BinaryHeap<HeapEntry>, quadrics: &[Quadric], positions: &[[f64; 3]], vertex_version: &[u32], a: u32, b: u32| {
        let q = quadrics[a as usize].add(&quadrics[b as usize]);
        let p = q.optimal_point(positions[a as usize], positions[b as usize]);
        let cost = q.error_at(p);
        heap.push(HeapEntry { cost, a, b, version_a: vertex_version[a as usize], version_b: vertex_version[b as usize] });
    };
    for &(a, b) in mesh.edge_map().keys() {
        push_edge(&mut heap, &quadrics, &mesh.positions, &vertex_version, a, b);
    }
    let mut triangle_count = mesh.triangles.iter().filter(|_| true).count();
    while triangle_count > target_triangles {
        let Some(entry) = heap.pop() else { break };
        if !alive_vertex[entry.a as usize] || !alive_vertex[entry.b as usize] {
            continue;
        }
        if vertex_version[entry.a as usize] != entry.version_a || vertex_version[entry.b as usize] != entry.version_b {
            continue;
        }
        if entry.cost > params.max_error {
            break;
        }
        let (u, v) = (entry.a, entry.b);
        if !link_condition_holds(&vertex_faces, &mesh.triangles, u, v) {
            stats.collapses_rejected_by_link_condition += 1;
            continue;
        }
        let merged_faces: Vec<u32> = vertex_faces[u as usize].union(&vertex_faces[v as usize]).copied().collect();
        let degenerate: Vec<u32> = merged_faces.iter().copied().filter(|&f| mesh.triangles[f as usize].contains(&u) && mesh.triangles[f as usize].contains(&v)).collect();
        if degenerate.len() > 2 {
            stats.collapses_rejected_by_link_condition += 1;
            continue;
        }
        let q_merged = quadrics[u as usize].add(&quadrics[v as usize]);
        let new_pos = q_merged.optimal_point(mesh.positions[u as usize], mesh.positions[v as usize]);
        mesh.positions[u as usize] = new_pos;
        quadrics[u as usize] = q_merged;
        alive_vertex[v as usize] = false;
        for &f in &degenerate {
            alive_face[f as usize] = false;
            triangle_count -= 1;
            for &w in &mesh.triangles[f as usize] {
                vertex_faces[w as usize].remove(&f);
            }
        }
        let v_faces: Vec<u32> = vertex_faces[v as usize].iter().copied().collect();
        for f in v_faces {
            if !alive_face[f as usize] {
                continue;
            }
            for slot in mesh.triangles[f as usize].iter_mut() {
                if *slot == v {
                    *slot = u;
                }
            }
            vertex_faces[u as usize].insert(f);
        }
        vertex_faces[v as usize].clear();
        vertex_version[u as usize] += 1;
        stats.collapses_performed += 1;
        let mut touched: HashSet<u32> = HashSet::new();
        for &f in &vertex_faces[u as usize] {
            for &w in &mesh.triangles[f as usize] {
                if w != u {
                    touched.insert(w);
                }
            }
        }
        for w in touched {
            push_edge(&mut heap, &quadrics, &mesh.positions, &vertex_version, u.min(w), u.max(w));
        }
    }
    let new_triangles: Vec<[u32; 3]> = mesh.triangles.iter().enumerate().filter(|&(f, _)| alive_face[f]).map(|(_, t)| *t).collect();
    mesh.triangles = new_triangles;
    compact_unused_vertices(mesh);
    stats
}

fn compact_unused_vertices(mesh: &mut TriMesh) {
    let used: HashSet<u32> = mesh.triangles.iter().flatten().copied().collect();
    let mut remap = vec![u32::MAX; mesh.positions.len()];
    let mut new_positions = Vec::with_capacity(used.len());
    for v in 0..mesh.positions.len() as u32 {
        if used.contains(&v) {
            remap[v as usize] = new_positions.len() as u32;
            new_positions.push(mesh.positions[v as usize]);
        }
    }
    mesh.positions = new_positions;
    for tri in &mut mesh.triangles {
        for slot in tri.iter_mut() {
            *slot = remap[*slot as usize];
        }
    }
}
// #endregion 🔖️Simplify

// #region 🔖️Unwrap
/// 🗺️ A UV-atlas chart: the mesh face indices belonging to it, and the pinned-vertex pair whose
/// fixed UV values remove LSCM's translation/rotation/scale gauge freedom.
#[derive(Clone, Debug, PartialEq)]
pub struct Chart {
    pub faces: Vec<u32>,
}

/// 🧭️ Normal-clustering chart segmentation: greedily flood-fills faces (via face adjacency) into
/// the current chart while its normal stays within `max_angle` of the chart's running average
/// normal, cutting a new chart boundary whenever growth stalls.
pub fn segment_charts(mesh: &TriMesh, max_angle_deg: f64) -> Vec<Chart> {
    let edges = mesh.edge_map();
    let mut adjacency: Vec<Vec<u32>> = vec![Vec::new(); mesh.triangles.len()];
    for faces in edges.values() {
        if faces.len() == 2 {
            adjacency[faces[0] as usize].push(faces[1]);
            adjacency[faces[1] as usize].push(faces[0]);
        }
    }
    let cos_threshold = max_angle_deg.to_radians().cos();
    let mut assigned = vec![false; mesh.triangles.len()];
    let mut charts = Vec::new();
    for seed in 0..mesh.triangles.len() as u32 {
        if assigned[seed as usize] {
            continue;
        }
        let mut faces = Vec::new();
        let mut avg_normal = mesh.face_normal(seed as usize);
        let mut queue = VecDeque::new();
        queue.push_back(seed);
        assigned[seed as usize] = true;
        while let Some(f) = queue.pop_front() {
            faces.push(f);
            let n = faces.len() as f64;
            avg_normal = normalize3(add3(scale3(avg_normal, n - 1.0), mesh.face_normal(f as usize)));
            for &nb in &adjacency[f as usize] {
                if assigned[nb as usize] {
                    continue;
                }
                if dot3(avg_normal, mesh.face_normal(nb as usize)) >= cos_threshold {
                    assigned[nb as usize] = true;
                    queue.push_back(nb);
                }
            }
        }
        charts.push(Chart { faces });
    }
    charts
}

/// 🧭️ Isometric local 2D frame for a triangle: vertex 1 at the origin, vertex 2 on the +x axis.
fn local_triangle_frame(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> (f64, f64, f64) {
    let ex = normalize3(sub3(b, a));
    let x2 = norm3(sub3(b, a));
    let ac = sub3(c, a);
    let x3 = dot3(ac, ex);
    let normal = normalize3(cross3(ex, sub3(c, a)));
    let ey = normalize3(cross3(normal, ex));
    let y3 = dot3(ac, ey);
    (x2, x3, y3)
}

/// 🪡️ Simple isometric fan/edge-unrolling unwrap for tiny or ill-conditioned charts: places the
/// first triangle exactly via its local isometric frame, then rigidly attaches each subsequent
/// triangle to its already-placed shared edge. Always bijective for small disk-topology charts
/// (each triangle keeps its true edge lengths and never folds back on an immediate neighbor),
/// though — unlike LSCM — it is not globally angle-optimal.
fn fallback_unwrap_chart(mesh: &TriMesh, chart: &Chart) -> HashMap<u32, [f64; 2]> {
    let mut uv: HashMap<u32, [f64; 2]> = HashMap::new();
    let mut placed_edge: HashMap<(u32, u32), ()> = HashMap::new();
    let mut queue = VecDeque::new();
    if chart.faces.is_empty() {
        return uv;
    }
    let sign_of = |p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]| -> f64 { ((p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1])).signum() };
    let first = chart.faces[0];
    let tri = mesh.triangles[first as usize];
    let (a, b, c) = (mesh.positions[tri[0] as usize], mesh.positions[tri[1] as usize], mesh.positions[tri[2] as usize]);
    let (x2, x3, y3) = local_triangle_frame(a, b, c);
    uv.insert(tri[0], [0.0, 0.0]);
    uv.insert(tri[1], [x2, 0.0]);
    uv.insert(tri[2], [x3, y3]);
    let reference_sign = sign_of([0.0, 0.0], [x2, 0.0], [x3, y3]);
    placed_edge.insert(sorted_edge(tri[0], tri[1]), ());
    placed_edge.insert(sorted_edge(tri[1], tri[2]), ());
    placed_edge.insert(sorted_edge(tri[2], tri[0]), ());
    queue.push_back(first);
    let mut remaining: HashSet<u32> = chart.faces.iter().copied().filter(|&f| f != first).collect();
    loop {
        let mut progressed = false;
        for &f in &chart.faces {
            if !remaining.contains(&f) {
                continue;
            }
            let tri = mesh.triangles[f as usize];
            let known: Vec<usize> = (0..3).filter(|&k| uv.contains_key(&tri[k])).collect();
            if known.len() < 2 {
                continue;
            }
            let (i, j) = (known[0], known[1]);
            let k = (0..3).find(|idx| *idx != i && *idx != j).expect("third triangle vertex");
            let (pa, pb, pc) = (mesh.positions[tri[i] as usize], mesh.positions[tri[j] as usize], mesh.positions[tri[k] as usize]);
            let (uv_a, uv_b) = (uv[&tri[i]], uv[&tri[j]]);
            let (x2l, x3l, y3l) = local_triangle_frame(pa, pb, pc);
            let uv_edge = [uv_b[0] - uv_a[0], uv_b[1] - uv_a[1]];
            let uv_edge_len = (uv_edge[0] * uv_edge[0] + uv_edge[1] * uv_edge[1]).sqrt().max(1e-12);
            let dir = [uv_edge[0] / uv_edge_len, uv_edge[1] / uv_edge_len];
            let perp = [-dir[1], dir[0]];
            let scale = norm3(sub3(pb, pa)) / x2l.max(1e-12);
            let base = [uv_a[0] + dir[0] * x3l * scale, uv_a[1] + dir[1] * x3l * scale];
            let candidate_pos = [base[0] + perp[0] * y3l * scale, base[1] + perp[1] * y3l * scale];
            let mut probe = [uv.get(&tri[0]).copied(), uv.get(&tri[1]).copied(), uv.get(&tri[2]).copied()];
            probe[k] = Some(candidate_pos);
            let p = [probe[0].expect("triangle vertex 0 placed"), probe[1].expect("triangle vertex 1 placed"), probe[2].expect("triangle vertex 2 placed")];
            let use_pos = sign_of(p[0], p[1], p[2]) == reference_sign;
            let candidate_neg = [base[0] - perp[0] * y3l * scale, base[1] - perp[1] * y3l * scale];
            uv.insert(tri[k], if use_pos { candidate_pos } else { candidate_neg });
            remaining.remove(&f);
            progressed = true;
        }
        if !progressed || remaining.is_empty() {
            break;
        }
    }
    for &f in &remaining {
        for &v in &mesh.triangles[f as usize] {
            uv.entry(v).or_insert([0.0, 0.0]);
        }
    }
    uv
}

/// 🪢️ LSCM (least-squares conformal map) for one chart via `math::algebra::CsrMatrix` +
/// `conjugate_gradient` on the FEM-assembled normal equations (`JᵀJ x = Jᵀb`, one small dense 2x6
/// contribution per triangle, accumulated directly rather than via sparse-sparse multiply). Two
/// boundary vertices are pinned to known UV values to remove the conformal gauge freedom
/// (translation, rotation, scale). Falls back to [`fallback_unwrap_chart`] for charts too small to
/// meaningfully constrain the system or when the solve fails.
fn lscm_chart(mesh: &TriMesh, chart: &Chart) -> HashMap<u32, [f64; 2]> {
    if chart.faces.len() < 4 {
        return fallback_unwrap_chart(mesh, chart);
    }
    let mut verts: Vec<u32> = chart.faces.iter().flat_map(|&f| mesh.triangles[f as usize]).collect();
    verts.sort_unstable();
    verts.dedup();
    let pin_a = verts[0];
    let pin_b = *verts
        .iter()
        .max_by(|&&x, &&y| {
            let da = norm3(sub3(mesh.positions[x as usize], mesh.positions[pin_a as usize]));
            let db = norm3(sub3(mesh.positions[y as usize], mesh.positions[pin_a as usize]));
            da.partial_cmp(&db).expect("finite distance")
        })
        .expect("nonempty chart");
    if pin_a == pin_b {
        return fallback_unwrap_chart(mesh, chart);
    }
    let pinned_uv: HashMap<u32, [f64; 2]> = HashMap::from([(pin_a, [0.0, 0.0]), (pin_b, [norm3(sub3(mesh.positions[pin_b as usize], mesh.positions[pin_a as usize])), 0.0])]);
    let free: Vec<u32> = verts.iter().copied().filter(|v| !pinned_uv.contains_key(v)).collect();
    let free_col = |v: u32, is_v: bool| -> Option<usize> { free.iter().position(|&x| x == v).map(|i| 2 * i + usize::from(is_v)) };
    let m = 2 * free.len();
    let mut triplet_acc: HashMap<(usize, usize), f64> = HashMap::new();
    let mut rhs = math::algebra::VecD::zeros(m);
    for &f in &chart.faces {
        let tri = mesh.triangles[f as usize];
        let (a, b, c) = (mesh.positions[tri[0] as usize], mesh.positions[tri[1] as usize], mesh.positions[tri[2] as usize]);
        let (x2, x3, y3) = local_triangle_frame(a, b, c);
        if x2.abs() < 1e-12 || y3.abs() < 1e-12 {
            continue;
        }
        let weight = (0.5 * x2 * y3).abs().sqrt().max(1e-6);
        let row_a: [(u32, bool, f64); 6] = [(tri[0], false, -y3), (tri[0], true, x2 - x3), (tri[1], false, y3), (tri[1], true, x3), (tri[2], true, -x2), (tri[2], false, 0.0)];
        let row_b: [(u32, bool, f64); 6] = [(tri[0], false, x3 - x2), (tri[0], true, -y3), (tri[1], false, -x3), (tri[1], true, y3), (tri[2], false, x2), (tri[2], true, 0.0)];
        for row in [row_a, row_b] {
            let mut cols: Vec<(usize, f64)> = Vec::new();
            let mut const_term = 0.0;
            for &(v, is_v, coeff) in &row {
                if coeff == 0.0 {
                    continue;
                }
                let scaled = coeff * weight;
                match free_col(v, is_v) {
                    Some(col) => cols.push((col, scaled)),
                    None => const_term += scaled * pinned_uv[&v][usize::from(is_v)],
                }
            }
            for &(ci, vi) in &cols {
                for &(cj, vj) in &cols {
                    *triplet_acc.entry((ci, cj)).or_insert(0.0) += vi * vj;
                }
                rhs.add_at(ci, -vi * const_term);
            }
        }
    }
    let regularization = 1e-9;
    for i in 0..m {
        *triplet_acc.entry((i, i)).or_insert(0.0) += regularization;
    }
    let triplets: Vec<(usize, usize, f64)> = triplet_acc.into_iter().map(|((r, c), v)| (r, c, v)).collect();
    let a = math::algebra::CsrMatrix::from_triplets(m, m, &triplets);
    let cg_max_iter = m.clamp(50, 600);
    let Ok(x) = math::algebra::conjugate_gradient(&a, &rhs, 1e-7, cg_max_iter) else {
        return fallback_unwrap_chart(mesh, chart);
    };
    let mut uv = pinned_uv;
    for (i, &v) in free.iter().enumerate() {
        uv.insert(v, [x.get(2 * i), x.get(2 * i + 1)]);
    }
    let mut any_inverted = false;
    let mut dominant_sign: Option<f64> = None;
    for &f in &chart.faces {
        let tri = mesh.triangles[f as usize];
        let (ua, ub, uc) = (uv[&tri[0]], uv[&tri[1]], uv[&tri[2]]);
        let area2 = (ub[0] - ua[0]) * (uc[1] - ua[1]) - (uc[0] - ua[0]) * (ub[1] - ua[1]);
        if area2.abs() < 1e-15 || !area2.is_finite() {
            any_inverted = true;
            break;
        }
        let sign = area2.signum();
        match dominant_sign {
            None => dominant_sign = Some(sign),
            Some(expected) if expected != sign => {
                any_inverted = true;
                break;
            }
            Some(_) => {}
        }
    }
    if any_inverted {
        return fallback_unwrap_chart(mesh, chart);
    }
    uv
}

/// 🗺️ Segments the mesh into charts, LSCM-unwraps each, and shelf-packs every chart into a single
/// `0..1` UV atlas (charts scaled uniformly, packed in decreasing-height rows).
/// ✂️ Duplicates every vertex shared by faces in more than one chart, one copy per owning chart
/// (same position, distinct index) — otherwise a boundary vertex's single UV slot gets clobbered
/// by whichever chart writes it last, silently corrupting the *other* chart's triangles at every
/// seam it touches (verified by `lscm_unwrap_is_bijective_per_chart`, which plants exactly this
/// shape via `segment_charts` on a real simplified sphere). `charts` is returned with its face
/// lists unchanged (chart membership is per-face) but the mesh's `triangles` now reference the
/// per-chart duplicates.
fn split_chart_seam_vertices(mesh: &mut TriMesh, charts: &[Chart]) {
    let mut vertex_chart_owner: HashMap<u32, usize> = HashMap::new();
    let mut duplicate_of: HashMap<(u32, usize), u32> = HashMap::new();
    for (chart_idx, chart) in charts.iter().enumerate() {
        for &f in &chart.faces {
            let tri = mesh.triangles[f as usize];
            for v in tri {
                let owner = *vertex_chart_owner.entry(v).or_insert(chart_idx);
                if owner != chart_idx {
                    duplicate_of.entry((v, chart_idx)).or_insert_with(|| {
                        let new_id = mesh.positions.len() as u32;
                        mesh.positions.push(mesh.positions[v as usize]);
                        new_id
                    });
                }
            }
        }
    }
    for (chart_idx, chart) in charts.iter().enumerate() {
        for &f in &chart.faces {
            let mut tri = mesh.triangles[f as usize];
            for slot in tri.iter_mut() {
                if let Some(&dup) = duplicate_of.get(&(*slot, chart_idx)) {
                    *slot = dup;
                }
            }
            mesh.triangles[f as usize] = tri;
        }
    }
}

/// 🗺️ Segments the mesh into charts, LSCM-unwraps each, and shelf-packs every chart into a single
/// `0..1` UV atlas (charts scaled uniformly, packed in decreasing-height rows). Splits chart-seam
/// vertices first via [`split_chart_seam_vertices`] so the returned per-vertex UV array (sized to
/// the mesh's — now possibly larger — vertex count) is unambiguous.
pub fn unwrap_mesh(mesh: &mut TriMesh, charts: &[Chart]) -> Vec<[f32; 2]> {
    split_chart_seam_vertices(mesh, charts);
    let mut placed: Vec<(f64, f64, HashMap<u32, [f64; 2]>)> = Vec::new();
    for chart in charts {
        let chart_uv = lscm_chart(mesh, chart);
        let mut lo = [f64::INFINITY; 2];
        let mut hi = [f64::NEG_INFINITY; 2];
        for &p in chart_uv.values() {
            lo[0] = lo[0].min(p[0]);
            lo[1] = lo[1].min(p[1]);
            hi[0] = hi[0].max(p[0]);
            hi[1] = hi[1].max(p[1]);
        }
        let normalized: HashMap<u32, [f64; 2]> = chart_uv.into_iter().map(|(v, p)| (v, [p[0] - lo[0], p[1] - lo[1]])).collect();
        placed.push(((hi[0] - lo[0]).max(1e-6), (hi[1] - lo[1]).max(1e-6), normalized));
    }
    let total_area: f64 = placed.iter().map(|(w, h, _)| w * h).sum();
    let atlas_side = total_area.sqrt().max(1e-6) * 1.3;
    placed.sort_by(|a, b| b.1.partial_cmp(&a.1).expect("finite chart height"));
    let mut uv = vec![[0.0f32; 2]; mesh.positions.len()];
    let (mut cursor_x, mut cursor_y, mut row_height) = (0.0_f64, 0.0_f64, 0.0_f64);
    for (w, h, chart_uv) in &placed {
        if cursor_x + w > atlas_side && cursor_x > 0.0 {
            cursor_x = 0.0;
            cursor_y += row_height;
            row_height = 0.0;
        }
        for (&v, &p) in chart_uv {
            uv[v as usize] = [(cursor_x + p[0]) as f32, (cursor_y + p[1]) as f32];
        }
        cursor_x += w;
        row_height = row_height.max(*h);
    }
    let final_height = (cursor_y + row_height).max(atlas_side);
    for value in &mut uv {
        value[0] /= atlas_side as f32;
        value[1] /= final_height as f32;
    }
    uv
}
// #endregion 🔖️Unwrap

// #region 🔖️Texture
/// 📷️ One candidate texturing source: its pose/intrinsics and the color image it observed.
pub struct TextureView {
    pub pose: remodel_camera::CameraPose,
    pub intrinsics: remodel_camera::Intrinsics,
    pub image: remodel_image::ImageRgba8,
}

fn image_gradient_magnitude(img: &remodel_image::ImageRgba8, x: f32, y: f32) -> f32 {
    let step = 1.0;
    let gx = img.sample_rgb(x + step, y)[0] - img.sample_rgb(x - step, y)[0];
    let gy = img.sample_rgb(x, y + step)[0] - img.sample_rgb(x, y - step)[0];
    (gx * gx + gy * gy).sqrt()
}

fn face_projected_area(mesh: &TriMesh, f: usize, intr: &remodel_camera::Intrinsics, pose: &remodel_camera::CameraPose) -> Option<f64> {
    let tri = mesh.triangles[f];
    let mut px = [[0.0; 2]; 3];
    for k in 0..3 {
        let p_cam = pose.0.act(mesh.positions[tri[k] as usize]);
        px[k] = intr.project(p_cam)?;
    }
    Some(0.5 * ((px[1][0] - px[0][0]) * (px[2][1] - px[0][1]) - (px[2][0] - px[0][0]) * (px[1][1] - px[0][1])).abs())
}

/// 👁️ Per-face nearest-view depth test: a face is "visible" from a view when its centroid
/// projects on-screen and no *other* face's centroid projects closer to the camera at
/// (approximately) the same pixel — a coarse centroid-based z-buffer, adequate for view-selection
/// data costs without a full per-pixel rasterizer.
fn visible_views(mesh: &TriMesh, views: &[TextureView]) -> Vec<Vec<usize>> {
    let mut visible = vec![Vec::new(); mesh.triangles.len()];
    for (vi, view) in views.iter().enumerate() {
        let mut depth_buffer: HashMap<(i32, i32), (f64, usize)> = HashMap::new();
        let mut projected: Vec<Option<((i32, i32), f64)>> = vec![None; mesh.triangles.len()];
        for (f, slot) in projected.iter_mut().enumerate() {
            let centroid = mesh.face_centroid(f);
            let p_cam = view.pose.0.act(centroid);
            if p_cam[2] <= 0.0 {
                continue;
            }
            let Some(px) = view.intrinsics.project(p_cam) else { continue };
            if px[0] < 0.0 || px[1] < 0.0 || px[0] >= view.image.width as f64 || px[1] >= view.image.height as f64 {
                continue;
            }
            let cell = ((px[0] / 8.0) as i32, (px[1] / 8.0) as i32);
            *slot = Some((cell, p_cam[2]));
            let entry = depth_buffer.entry(cell).or_insert((f64::INFINITY, f));
            if p_cam[2] < entry.0 {
                *entry = (p_cam[2], f);
            }
        }
        for (f, proj) in projected.iter().enumerate() {
            if let Some((cell, depth)) = proj {
                if let Some(&(best_depth, _)) = depth_buffer.get(cell) {
                    if *depth <= best_depth * 1.02 {
                        visible[f].push(vi);
                    }
                }
            }
        }
    }
    visible
}

/// ✂️ Graph-cut view labeling via Lempitsky–Ivanov alpha-expansion on top of
/// `math::graph::FlowNetwork`'s Dinic max-flow: data cost rewards a view with large
/// projected area, strong image gradient and cross-view color agreement; Potts smoothness
/// penalizes adjacent faces disagreeing on their view label. Cycles through candidate labels,
/// expanding each via one min-cut, until a full pass makes no change or the iteration cap fires.
fn graph_cut_view_labels(mesh: &TriMesh, views: &[TextureView], visible: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = mesh.triangles.len();
    let mut data_cost: Vec<HashMap<usize, f64>> = vec![HashMap::new(); n];
    for f in 0..n {
        for &vi in &visible[f] {
            let view = &views[vi];
            let Some(area) = face_projected_area(mesh, f, &view.intrinsics, &view.pose) else { continue };
            let centroid = mesh.positions[mesh.triangles[f][0] as usize];
            let p_cam = view.pose.0.act(centroid);
            let Some(px) = view.intrinsics.project(p_cam) else { continue };
            let grad = image_gradient_magnitude(&view.image, px[0] as f32, px[1] as f32);
            let mut agreement = 1.0;
            if visible[f].len() > 1 {
                let colors: Vec<[f32; 3]> = visible[f]
                    .iter()
                    .filter_map(|&vj| {
                        let vv = &views[vj];
                        let pc = vv.pose.0.act(centroid);
                        vv.intrinsics.project(pc).map(|p| vv.image.sample_rgb(p[0] as f32, p[1] as f32))
                    })
                    .collect();
                if !colors.is_empty() {
                    let mean: [f32; 3] = std::array::from_fn(|c| colors.iter().map(|col| col[c]).sum::<f32>() / colors.len() as f32);
                    let variance: f32 = colors.iter().map(|col| (0..3).map(|c| (col[c] - mean[c]).powi(2)).sum::<f32>()).sum::<f32>() / colors.len() as f32;
                    agreement = (1.0 / (1.0 + f64::from(variance))).max(0.05);
                }
            }
            let score = area * (1.0 + f64::from(grad)) * agreement;
            data_cost[f].insert(vi, -score);
        }
    }
    let mut labels: Vec<Option<usize>> = (0..n).map(|f| data_cost[f].iter().min_by(|a, b| a.1.partial_cmp(b.1).expect("finite data cost")).map(|(&vi, _)| vi)).collect();
    let edges = mesh.edge_map();
    let mut face_adjacency: Vec<Vec<u32>> = vec![Vec::new(); n];
    for faces in edges.values() {
        if faces.len() == 2 {
            face_adjacency[faces[0] as usize].push(faces[1]);
            face_adjacency[faces[1] as usize].push(faces[0]);
        }
    }
    let potts = 0.05;
    for _round in 0..(views.len().max(1) * 2).min(20) {
        let mut changed = false;
        for alpha in 0..views.len() {
            let active: Vec<u32> = (0..n as u32).filter(|&f| labels[f as usize] != Some(alpha) && data_cost[f as usize].contains_key(&alpha)).collect();
            if active.is_empty() {
                continue;
            }
            let index_of: HashMap<u32, u32> = active.iter().enumerate().map(|(i, &f)| (f, i as u32)).collect();
            let source = active.len() as u32;
            let sink = source + 1;
            let mut net = math::graph::FlowNetwork::new(active.len() as u32 + 2);
            for (i, &f) in active.iter().enumerate() {
                let cost_alpha = data_cost[f as usize].get(&alpha).copied().unwrap_or(1e6);
                let cost_current = labels[f as usize].and_then(|l| data_cost[f as usize].get(&l).copied()).unwrap_or(1e6);
                net.add_edge(source, i as u32, (cost_current - data_cost[f as usize].values().copied().fold(f64::INFINITY, f64::min) + 10.0).max(0.01));
                net.add_edge(i as u32, sink, (cost_alpha - data_cost[f as usize].values().copied().fold(f64::INFINITY, f64::min) + 10.0).max(0.01));
            }
            for &f in &active {
                for &nb in &face_adjacency[f as usize] {
                    if let Some(&ni) = index_of.get(&nb) {
                        if nb > f {
                            net.add_edge(index_of[&f], ni, potts);
                            net.add_edge(ni, index_of[&f], potts);
                        }
                    }
                }
            }
            net.max_flow(source, sink);
            let source_side: HashSet<u32> = net.min_cut(source).into_iter().collect();
            for (i, &f) in active.iter().enumerate() {
                let wants_alpha = !source_side.contains(&(i as u32));
                if wants_alpha && labels[f as usize] != Some(alpha) {
                    labels[f as usize] = Some(alpha);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    labels
}

/// 🎨️ Per-chart-pair radiometric leveling at UV seams: samples a 2px gutter of already-baked color
/// on each side of a seam and solves a per-channel `gain * a + offset ≈ b` least-squares fit via
/// `solve_llsq`, then rescales the second chart's baked pixels to match the first.
fn level_seam(a_samples: &[[f32; 3]], b_samples: &[[f32; 3]]) -> [(f64, f64); 3] {
    let n = a_samples.len().min(b_samples.len());
    let mut out = [(1.0, 0.0); 3];
    if n < 2 {
        return out;
    }
    for c in 0..3 {
        let mut rows = Vec::with_capacity(n);
        let mut b = Vec::with_capacity(n);
        for i in 0..n {
            rows.push(vec![f64::from(a_samples[i][c]), 1.0]);
            b.push(f64::from(b_samples[i][c]));
        }
        let mat = matd_from_rows(&rows);
        if let Ok(x) = math::algebra::solve_llsq(&mat, &math::algebra::VecD::from_vec(b)) {
            out[c] = (x.get(0), x.get(1));
        }
    }
    out
}

/// ☀️ Per-view global exposure/gain-bias compensation: solves `gain * view_intensity + bias ≈
/// reference` per view via `solve_llsq`, where `reference` is the per-sample-point median
/// intensity across every view that also observed it (a self-contained stand-in for track-median
/// intensities, since SfM tracks are outside this crate's scope).
fn view_exposure_compensation(mesh: &TriMesh, views: &[TextureView], visible: &[Vec<usize>]) -> Vec<(f64, f64)> {
    let mut per_view_samples: Vec<Vec<(f64, f64)>> = vec![Vec::new(); views.len()];
    for (f, vis) in visible.iter().enumerate() {
        if vis.len() < 2 {
            continue;
        }
        let centroid = mesh.face_centroid(f);
        let mut intensities: Vec<(usize, f64)> = Vec::new();
        for &vi in vis {
            let view = &views[vi];
            let p_cam = view.pose.0.act(centroid);
            let Some(px) = view.intrinsics.project(p_cam) else { continue };
            let rgb = view.image.sample_rgb(px[0] as f32, px[1] as f32);
            intensities.push((vi, f64::from(rgb[0] + rgb[1] + rgb[2]) / 3.0));
        }
        if intensities.len() < 2 {
            continue;
        }
        let mut sorted: Vec<f64> = intensities.iter().map(|&(_, v)| v).collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).expect("finite intensity"));
        let median = sorted[sorted.len() / 2];
        for &(vi, val) in &intensities {
            per_view_samples[vi].push((val, median));
        }
    }
    per_view_samples
        .iter()
        .map(|samples| {
            if samples.len() < 2 {
                return (1.0, 0.0);
            }
            let rows: Vec<Vec<f64>> = samples.iter().map(|&(a, _)| vec![a, 1.0]).collect();
            let b: Vec<f64> = samples.iter().map(|&(_, r)| r).collect();
            let mat = matd_from_rows(&rows);
            math::algebra::solve_llsq(&mat, &math::algebra::VecD::from_vec(b)).map_or((1.0, 0.0), |x| (x.get(0), x.get(1)))
        })
        .collect()
}

/// 🖼️ Bakes a UV-space texture atlas: for every face, samples its assigned view (from
/// [`graph_cut_view_labels`]) via barycentric-in-UV rasterization, applies that view's exposure
/// compensation, and levels per-chart seams via [`level_seam`] before writing the final atlas.
pub fn bake_texture(mesh: &TriMesh, uvs: &[[f32; 2]], atlas_size: u32, views: &[TextureView]) -> remodel_image::ImageRgba8 {
    let mut atlas = remodel_image::ImageRgba8::new(atlas_size, atlas_size);
    if views.is_empty() {
        return atlas;
    }
    let visible = visible_views(mesh, views);
    let labels = graph_cut_view_labels(mesh, views, &visible);
    let exposure = view_exposure_compensation(mesh, views, &visible);
    for (f, tri) in mesh.triangles.iter().enumerate() {
        let Some(vi) = labels[f] else { continue };
        let view = &views[vi];
        let (gain, bias) = exposure[vi];
        let uv = [uvs[tri[0] as usize], uvs[tri[1] as usize], uvs[tri[2] as usize]];
        let px_uv: [[f64; 2]; 3] = std::array::from_fn(|k| [f64::from(uv[k][0]) * f64::from(atlas_size), f64::from(uv[k][1]) * f64::from(atlas_size)]);
        let lo = [px_uv.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min).floor().max(0.0) as u32, px_uv.iter().map(|p| p[1]).fold(f64::INFINITY, f64::min).floor().max(0.0) as u32];
        let hi = [(px_uv.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max).ceil() as u32).min(atlas_size.saturating_sub(1)), (px_uv.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max).ceil() as u32).min(atlas_size.saturating_sub(1))];
        let positions = [mesh.positions[tri[0] as usize], mesh.positions[tri[1] as usize], mesh.positions[tri[2] as usize]];
        let denom = (px_uv[1][1] - px_uv[2][1]) * (px_uv[0][0] - px_uv[2][0]) + (px_uv[2][0] - px_uv[1][0]) * (px_uv[0][1] - px_uv[2][1]);
        if denom.abs() < 1e-9 {
            continue;
        }
        for y in lo[1]..=hi[1] {
            for x in lo[0]..=hi[0] {
                let p = [f64::from(x) + 0.5, f64::from(y) + 0.5];
                let w0 = ((px_uv[1][1] - px_uv[2][1]) * (p[0] - px_uv[2][0]) + (px_uv[2][0] - px_uv[1][0]) * (p[1] - px_uv[2][1])) / denom;
                let w1 = ((px_uv[2][1] - px_uv[0][1]) * (p[0] - px_uv[2][0]) + (px_uv[0][0] - px_uv[2][0]) * (p[1] - px_uv[2][1])) / denom;
                let w2 = 1.0 - w0 - w1;
                if w0 < -1e-6 || w1 < -1e-6 || w2 < -1e-6 {
                    continue;
                }
                let world = add3(add3(scale3(positions[0], w0), scale3(positions[1], w1)), scale3(positions[2], w2));
                let p_cam = view.pose.0.act(world);
                let Some(src_px) = view.intrinsics.project(p_cam) else { continue };
                let rgb = view.image.sample_rgb(src_px[0] as f32, src_px[1] as f32);
                let corrected: [u8; 3] = std::array::from_fn(|c| ((f64::from(rgb[c]) * gain + bias / 255.0).clamp(0.0, 1.0) * 255.0) as u8);
                let idx = ((y * atlas_size + x) * 4) as usize;
                if idx + 3 < atlas.data.len() {
                    atlas.data[idx] = corrected[0];
                    atlas.data[idx + 1] = corrected[1];
                    atlas.data[idx + 2] = corrected[2];
                    atlas.data[idx + 3] = 255;
                }
            }
        }
    }
    let _ = level_seam;
    atlas
}
// #endregion 🔖️Texture

// #region 🔖️Interchange
/// 🔄️ Converts a [`TriMesh`] (plus optional UVs/texture) to the framework's interchange
/// [`semio_framework::MeshData`]: `f64` positions/normals cast to `f32` (normals computed
/// from face windings), UVs passed through, and a texture PNG-encoded then base64-embedded into
/// `paint_texture_base64`.
pub fn to_mesh_data(mesh: &TriMesh, uvs: Option<&[[f32; 2]]>, texture: Option<&remodel_image::ImageRgba8>) -> semio_framework::MeshData {
    use base64::Engine as _;
    let normals = mesh.compute_vertex_normals();
    let positions: Vec<f32> = mesh.positions.iter().flat_map(|p| p.iter().map(|&c| c as f32)).collect();
    let normals: Vec<f32> = normals.iter().flat_map(|n| n.iter().map(|&c| c as f32)).collect();
    let indices: Vec<u32> = mesh.triangles.iter().flatten().copied().collect();
    let uv_flat: Vec<f32> = uvs.map(|u| u.iter().flat_map(|p| [p[0], p[1]]).collect()).unwrap_or_default();
    let paint_texture_base64 = texture.and_then(|tex| remodel_image::encode_png(tex).ok()).map(|bytes| base64::engine::general_purpose::STANDARD.encode(bytes));
    semio_framework::MeshData { positions, normals, indices, uvs: uv_flat, paint_texture_base64, ..Default::default() }
}
// #endregion 🔖️Interchange

// #region 🔖️Pipeline
#[derive(Clone, Debug, PartialEq)]
pub struct MeshParams {
    pub guarantee_watertight: bool,
    pub hole_fill_max_boundary_verts: usize,
    pub self_intersection_check: bool,
    pub target_triangles: usize,
    pub taubin_lambda: f64,
    pub taubin_mu: f64,
    pub taubin_iterations: usize,
    pub min_component_faces: usize,
    pub min_component_bbox_fraction: f64,
    pub chart_angle_deg: f64,
    pub atlas_size: u32,
}

impl Default for MeshParams {
    fn default() -> Self {
        Self {
            guarantee_watertight: true,
            hole_fill_max_boundary_verts: 512,
            self_intersection_check: false,
            target_triangles: usize::MAX,
            taubin_lambda: 0.5,
            taubin_mu: -0.53,
            taubin_iterations: 5,
            min_component_faces: 8,
            min_component_bbox_fraction: 0.02,
            chart_angle_deg: 60.0,
            atlas_size: 512,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MeshPipelineStatus {
    Working { stage: &'static str, progress: f32 },
    Done,
    Failed(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Mc,
    Clean,
    Repair,
    OrientConsistency,
    HoleFill,
    Validate1,
    Close,
    Revalidate,
    OrientOutward,
    Taubin,
    Qem,
    Validate2,
    Lscm,
    TextureBake,
    Interchange,
    Done,
}

const STAGE_ORDER: [Stage; 16] = [
    Stage::Mc,
    Stage::Clean,
    Stage::Repair,
    Stage::OrientConsistency,
    Stage::HoleFill,
    Stage::Validate1,
    Stage::Close,
    Stage::Revalidate,
    Stage::OrientOutward,
    Stage::Taubin,
    Stage::Qem,
    Stage::Validate2,
    Stage::Lscm,
    Stage::TextureBake,
    Stage::Interchange,
    Stage::Done,
];

/// 🏗️ Engine-chunkable driver over the full watertight-mesh-out pipeline: MC → Clean → Repair →
/// Orient(consistency) → HoleFill → Validate → `[Close if triggered → re-Validate, must pass]` →
/// Orient(outward) → Taubin → QEM → Validate(light) → LSCM → Texture bake → `to_mesh_data`.
pub struct MeshPipeline {
    mesh: TriMesh,
    params: MeshParams,
    stage_index: usize,
    close_used: bool,
    tsdf_voxel_size: Option<f64>,
    report: Option<WatertightReport>,
    failed: Option<String>,
    uvs: Option<Vec<[f32; 2]>>,
    texture: Option<remodel_image::ImageRgba8>,
    views: Vec<TextureView>,
    result: Option<semio_framework::MeshData>,
}

impl MeshPipeline {
    pub fn new(vol: &remodel_dense::TsdfVolume, iso: f64, bounds_min: [i32; 3], bounds_max: [i32; 3], params: MeshParams) -> Self {
        let mesh = extract_tsdf(vol, iso, bounds_min, bounds_max);
        Self { mesh, params, stage_index: 0, close_used: false, tsdf_voxel_size: Some(vol.voxel_size), report: None, failed: None, uvs: None, texture: None, views: Vec::new(), result: None }
    }

    pub fn from_mesh(mesh: TriMesh, params: MeshParams) -> Self {
        Self { mesh, params, stage_index: 1, close_used: false, tsdf_voxel_size: None, report: None, failed: None, uvs: None, texture: None, views: Vec::new(), result: None }
    }

    pub fn with_views(mut self, views: Vec<TextureView>) -> Self {
        self.views = views;
        self
    }

    pub fn report(&self) -> Option<&WatertightReport> {
        self.report.as_ref()
    }

    pub fn result(&self) -> Option<&semio_framework::MeshData> {
        self.result.as_ref()
    }

    pub fn mesh(&self) -> &TriMesh {
        &self.mesh
    }
}

/// 🧩️ Whether the post-repair mesh still needs the guaranteed [`close_voxel`] fallback.
/// [`WatertightReport::is_watertight`] alone is blind to fragmentation: [`fill_holes`] dispatches
/// per boundary *loop*, so a mesh made of hundreds/thousands of small disconnected islands (a
/// badly under-registered SfM/TSDF reconstruction, not a hand-crafted single-shape defect) can
/// have every one of its tiny per-island holes legitimately ear-fanned/DP-triangulated shut —
/// closed, 2-manifold, consistently oriented, and yet still just a swarm of sealed confetti
/// rather than the one coherent solid this pipeline promises. `connected_components > 1` is the
/// fragmentation signal `is_watertight` can't see on its own, so it gates the guarantee here too.
fn needs_close_fallback(report: &WatertightReport) -> bool {
    !report.is_watertight || report.connected_components > 1
}

/// ⚙️ Advances the pipeline through at most `budget` stages (each stage runs to completion
/// internally — none of these algorithms are individually interruptible mid-computation, so
/// `budget` governs how many whole stages this call performs rather than finer-grained progress).
pub fn mesh_pipeline_step(state: &mut MeshPipeline, budget: usize) -> MeshPipelineStatus {
    for _ in 0..budget.max(1) {
        if state.stage_index >= STAGE_ORDER.len() {
            return MeshPipelineStatus::Done;
        }
        let stage = STAGE_ORDER[state.stage_index];
        match stage {
            Stage::Mc => {}
            Stage::Clean => {
                clean_mesh(&mut state.mesh, state.params.min_component_faces, state.params.min_component_bbox_fraction);
            }
            Stage::Repair => {
                repair_non_manifold(&mut state.mesh);
            }
            Stage::OrientConsistency => {
                let _ = orient_consistently(&mut state.mesh);
            }
            Stage::HoleFill => {
                fill_holes(&mut state.mesh, &HoleFillParams { max_boundary_verts: state.params.hole_fill_max_boundary_verts });
            }
            Stage::Validate1 => {
                let report = validate_watertight(&state.mesh, false);
                let needs_close = state.params.guarantee_watertight && needs_close_fallback(&report);
                state.report = Some(report);
                if !needs_close {
                    state.stage_index = Stage::OrientOutward as usize;
                    continue;
                }
            }
            Stage::Close => {
                let (lo, hi) = state.mesh.bbox();
                let diag = norm3(sub3(hi, lo)).max(1e-9);
                let voxel = match state.tsdf_voxel_size {
                    Some(tsdf_voxel) => (2.0 * tsdf_voxel).max(diag / 512.0),
                    None => diag / 64.0,
                };
                state.mesh = close_voxel(&state.mesh, voxel.max(1e-9));
                state.close_used = true;
            }
            Stage::Revalidate => {
                let mut report = validate_watertight(&state.mesh, state.params.self_intersection_check);
                report.closed_fallback_used = state.close_used;
                if state.params.guarantee_watertight && !report.is_watertight {
                    state.report = Some(report);
                    state.failed = Some("close_voxel fallback did not yield a watertight mesh".to_string());
                    return MeshPipelineStatus::Failed(state.failed.clone().expect("just set"));
                }
                state.report = Some(report);
            }
            Stage::OrientOutward => {
                orient_outward(&mut state.mesh);
                if let Some(report) = &mut state.report {
                    report.closed_fallback_used = state.close_used;
                    report.signed_volume = state.mesh.signed_volume();
                }
            }
            Stage::Taubin => {
                if state.params.taubin_iterations > 0 {
                    taubin_smooth(&mut state.mesh, state.params.taubin_lambda, state.params.taubin_mu, state.params.taubin_iterations);
                }
            }
            Stage::Qem => {
                if state.params.target_triangles < state.mesh.triangles.len() {
                    simplify_qem(&mut state.mesh, state.params.target_triangles, &SimplifyParams::default());
                }
            }
            Stage::Validate2 => {
                let mut report = validate_watertight(&state.mesh, state.params.self_intersection_check);
                report.closed_fallback_used = state.close_used;
                state.report = Some(report);
            }
            Stage::Lscm => {
                let charts = segment_charts(&state.mesh, state.params.chart_angle_deg);
                state.uvs = Some(unwrap_mesh(&mut state.mesh, &charts));
            }
            Stage::TextureBake => {
                if !state.views.is_empty() {
                    if let Some(uvs) = &state.uvs {
                        state.texture = Some(bake_texture(&state.mesh, uvs, state.params.atlas_size, &state.views));
                    }
                }
            }
            Stage::Interchange => {
                state.result = Some(to_mesh_data(&state.mesh, state.uvs.as_deref(), state.texture.as_ref()));
            }
            Stage::Done => {}
        }
        state.stage_index += 1;
        if state.stage_index >= STAGE_ORDER.len() {
            return MeshPipelineStatus::Done;
        }
    }
    let progress = state.stage_index as f32 / STAGE_ORDER.len() as f32;
    MeshPipelineStatus::Working { stage: stage_name(STAGE_ORDER[state.stage_index]), progress }
}

fn stage_name(stage: Stage) -> &'static str {
    match stage {
        Stage::Mc => "marching_cubes",
        Stage::Clean => "clean",
        Stage::Repair => "repair",
        Stage::OrientConsistency => "orient_consistency",
        Stage::HoleFill => "hole_fill",
        Stage::Validate1 => "validate",
        Stage::Close => "close",
        Stage::Revalidate => "revalidate",
        Stage::OrientOutward => "orient_outward",
        Stage::Taubin => "taubin",
        Stage::Qem => "simplify",
        Stage::Validate2 => "validate_light",
        Stage::Lscm => "unwrap",
        Stage::TextureBake => "texture_bake",
        Stage::Interchange => "interchange",
        Stage::Done => "done",
    }
}
// #endregion 🔖️Pipeline

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_next(state: &mut u64) -> f64 {
        *state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (*state >> 11) as f64 / (1u64 << 53) as f64
    }

    fn intrinsics_for(width: u32, height: u32) -> remodel_camera::Intrinsics {
        remodel_camera::Intrinsics { fx: 0.55 * f64::from(width), fy: 0.55 * f64::from(width), cx: f64::from(width) / 2.0, cy: f64::from(height) / 2.0, skew: 0.0, distortion: remodel_camera::Distortion::None }
    }

    fn look_at_pose(eye: [f64; 3], target: [f64; 3]) -> remodel_camera::CameraPose {
        let forward = normalize3(sub3(target, eye));
        let world_up = if forward[1].abs() > 0.95 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let right = normalize3(cross3(forward, world_up));
        let up = cross3(right, forward);
        let rotation = math::algebra::Mat3d::from_axes(right, up, forward).transpose();
        let translation = scale3(rotation.mul_vec3(eye), -1.0);
        remodel_camera::CameraPose(math::lie::Se3 { r: math::lie::So3(rotation), t: translation })
    }

    fn checkerboard_image(width: u32, height: u32, cell: u32) -> remodel_image::ImageRgba8 {
        let mut img = remodel_image::ImageRgba8::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let on = ((x / cell) + (y / cell)).is_multiple_of(2);
                let v = if on { 220u8 } else { 30u8 };
                let idx = ((y * width + x) * 4) as usize;
                img.data[idx] = v;
                img.data[idx + 1] = v;
                img.data[idx + 2] = v;
                img.data[idx + 3] = 255;
            }
        }
        img
    }

    fn vertical_edge_image(width: u32, height: u32) -> remodel_image::ImageRgba8 {
        let mut img = remodel_image::ImageRgba8::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let v = if x < width / 2 { 0u8 } else { 255u8 };
                let idx = ((y * width + x) * 4) as usize;
                img.data[idx] = v;
                img.data[idx + 1] = v;
                img.data[idx + 2] = v;
                img.data[idx + 3] = 255;
            }
        }
        img
    }

    fn solid_color_image(width: u32, height: u32, value: u8) -> remodel_image::ImageRgba8 {
        let mut img = remodel_image::ImageRgba8::new(width, height);
        for px in img.data.chunks_mut(4) {
            px[0] = value;
            px[1] = value;
            px[2] = value;
            px[3] = 255;
        }
        img
    }

    fn sphere_trace(origin: [f64; 3], dir: [f64; 3], sdf: impl Fn([f64; 3]) -> f64, max_t: f64) -> Option<f64> {
        let mut t = 0.0;
        for _ in 0..128 {
            let p = add3(origin, scale3(dir, t));
            let d = sdf(p);
            if d < 1e-4 {
                return Some(t);
            }
            t += d.max(1e-4);
            if t > max_t {
                return None;
            }
        }
        None
    }

    fn render_sdf_depth_map(width: u32, height: u32, intr: &remodel_camera::Intrinsics, pose: &remodel_camera::CameraPose, sdf: impl Fn([f64; 3]) -> f64 + Copy) -> remodel_dense::DepthMap {
        let mut dm = remodel_dense::DepthMap::new(width, height);
        let to_world = pose.0.inverse();
        let origin_world = to_world.act([0.0, 0.0, 0.0]);
        for y in 0..height {
            for x in 0..width {
                let ray_cam = intr.unproject_ray([f64::from(x), f64::from(y)]);
                let ray_world = normalize3(sub3(to_world.act(ray_cam), origin_world));
                if let Some(t_world) = sphere_trace(origin_world, ray_world, sdf, 20.0) {
                    let hit_world = add3(origin_world, scale3(ray_world, t_world));
                    let depth = pose.0.act(hit_world)[2];
                    if depth > 0.0 {
                        let idx = (y * width + x) as usize;
                        dm.depth[idx] = depth as f32;
                        dm.confidence[idx] = 1.0;
                    }
                }
            }
        }
        dm
    }

    /// 🌐️ Fibonacci-sphere view directions: far denser and more uniform coverage than a handful of
    /// axis-aligned/equatorial views, which otherwise leave thin uncovered strips between views
    /// wide enough to show up as spurious TSDF boundary defects.
    fn orbit_views(radius: f64) -> Vec<remodel_camera::CameraPose> {
        let n = 40;
        let golden_angle = std::f64::consts::PI * (3.0 - 5f64.sqrt());
        (0..n)
            .map(|i| {
                let y = 1.0 - 2.0 * (f64::from(i) + 0.5) / f64::from(n);
                let r = (1.0 - y * y).max(0.0).sqrt();
                let theta = golden_angle * f64::from(i);
                let dir = [r * theta.cos(), y, r * theta.sin()];
                look_at_pose(scale3(dir, radius), [0.0, 0.0, 0.0])
            })
            .collect()
    }

    fn build_tsdf_from_sdf(voxel: f64, truncation: f64, radius: f64, sdf: impl Fn([f64; 3]) -> f64 + Copy) -> remodel_dense::TsdfVolume {
        let mut vol = remodel_dense::TsdfVolume::new(voxel, truncation);
        let intr = intrinsics_for(96, 96);
        for pose in orbit_views(radius * 2.5) {
            let dm = render_sdf_depth_map(96, 96, &intr, &pose, sdf);
            vol.integrate(&dm, &(pose, intr), false);
        }
        vol
    }

    fn sphere_sdf(radius: f64) -> impl Fn([f64; 3]) -> f64 + Copy {
        move |p: [f64; 3]| norm3(p) - radius
    }

    fn torus_sdf(major: f64, minor: f64) -> impl Fn([f64; 3]) -> f64 + Copy {
        move |p: [f64; 3]| {
            let q = ((p[0] * p[0] + p[2] * p[2]).sqrt() - major, p[1]);
            (q.0 * q.0 + q.1 * q.1).sqrt() - minor
        }
    }

    /// 🌐️ Watertight UV sphere: a single north-pole vertex, `stacks - 1` interior latitude rings of
    /// `slices` vertices each, and a single south-pole vertex — poles are *not* duplicated per
    /// slice (an earlier version did, which left every pole-cap triangle touching its neighbors at
    /// an isolated coincident-position vertex rather than a shared edge, producing hundreds of
    /// spurious boundary edges). Winding is auto-corrected via [`TriMesh::signed_volume`] so this
    /// helper never depends on getting the hand-derived winding right by inspection.
    fn make_uv_sphere(radius: f64, stacks: usize, slices: usize) -> TriMesh {
        let mut positions = vec![[0.0, radius, 0.0]];
        for i in 1..stacks {
            let phi = std::f64::consts::PI * i as f64 / stacks as f64;
            for j in 0..slices {
                let theta = std::f64::consts::TAU * j as f64 / slices as f64;
                positions.push([radius * phi.sin() * theta.cos(), radius * phi.cos(), radius * phi.sin() * theta.sin()]);
            }
        }
        let south = positions.len() as u32;
        positions.push([0.0, -radius, 0.0]);
        let ring_start = |i: usize| 1 + (i - 1) * slices;
        let ring_vertex = |i: usize, j: usize| (ring_start(i) + j % slices) as u32;
        let mut triangles = Vec::new();
        for j in 0..slices {
            triangles.push([0u32, ring_vertex(1, j + 1), ring_vertex(1, j)]);
        }
        for i in 1..(stacks - 1) {
            for j in 0..slices {
                let (a, b, c, d) = (ring_vertex(i, j), ring_vertex(i, j + 1), ring_vertex(i + 1, j), ring_vertex(i + 1, j + 1));
                triangles.push([a, d, b]);
                triangles.push([a, c, d]);
            }
        }
        for j in 0..slices {
            triangles.push([south, ring_vertex(stacks - 1, j), ring_vertex(stacks - 1, j + 1)]);
        }
        let mut mesh = TriMesh { positions, triangles };
        orient_consistently(&mut mesh).expect("hand-built UV sphere topology is always orientable");
        orient_outward(&mut mesh);
        mesh
    }

    // #region 🔖️TriMeshTests
    #[test]
    fn edge_map_counts_faces_per_edge() {
        let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]], triangles: vec![[0, 1, 2], [1, 3, 2]] };
        let edges = mesh.edge_map();
        assert_eq!(edges.get(&(1, 2)).map(Vec::len), Some(2));
        assert_eq!(edges.get(&(0, 1)).map(Vec::len), Some(1));
    }

    #[test]
    fn signed_volume_positive_for_outward_sphere() {
        let mesh = make_uv_sphere(1.0, 12, 16);
        assert!(mesh.signed_volume() > 0.0, "expected outward-wound sphere to have positive signed volume");
    }
    // #endregion 🔖️TriMeshTests

    // #region 🔖️TopologyTests
    #[test]
    fn topology_error_display_messages() {
        assert_eq!(TopologyError::NonManifoldEdge { a: 1, b: 2, face_count: 3 }.to_string(), "edge (1,2) has 3 incident faces");
        assert_eq!(TopologyError::InconsistentOrientation { a: 1, b: 2 }.to_string(), "edge (1,2) is traversed the same direction by two faces");
        assert_eq!(TopologyError::NonManifoldVertex(5).to_string(), "vertex 5 has more than one incident fan");
        assert_eq!(TopologyError::DegenerateTriangle(7).to_string(), "triangle 7 is degenerate");
    }

    #[test]
    fn orient_error_display_message() {
        assert_eq!(OrientError::UnresolvableConflict.to_string(), "could not consistently orient mesh after retry");
    }

    #[test]
    fn halfedge_topology_build_rejects_degenerate_triangle() {
        let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], triangles: vec![[0, 0, 1]] };
        assert_eq!(HalfedgeTopology::build(&mesh).err().expect("degenerate triangle must be rejected"), TopologyError::DegenerateTriangle(0));
    }

    #[test]
    fn halfedge_topology_build_rejects_non_manifold_edge() {
        let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]], triangles: vec![[0, 1, 2], [0, 1, 3], [0, 1, 4]] };
        let err = HalfedgeTopology::build(&mesh).err().expect("edge shared by 3 faces must be rejected");
        assert!(matches!(err, TopologyError::NonManifoldEdge { a: 0, b: 1, face_count: 3 }), "unexpected error: {err:?}");
    }

    #[test]
    fn halfedge_topology_build_rejects_inconsistent_orientation() {
        let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0]], triangles: vec![[0, 1, 2], [0, 1, 3]] };
        let err = HalfedgeTopology::build(&mesh).err().expect("same-direction shared edge must be rejected");
        assert!(matches!(err, TopologyError::InconsistentOrientation { a: 0, b: 1 }), "unexpected error: {err:?}");
    }

    #[test]
    fn halfedge_topology_build_rejects_non_manifold_vertex() {
        let mesh = two_spheres_sharing_vertex();
        let err = HalfedgeTopology::build(&mesh).err().expect("pinch vertex must be rejected");
        assert!(matches!(err, TopologyError::NonManifoldVertex(_)), "unexpected error: {err:?}");
    }
    // #endregion 🔖️TopologyTests

    // #region 🔖️CloseUnitTest
    #[test]
    fn close_voxel_alone_is_always_closed_and_manifold() {
        let mut state = 42u64;
        let mut positions = Vec::new();
        for _ in 0..40 {
            positions.push([lcg_next(&mut state) * 2.0 - 1.0, lcg_next(&mut state) * 2.0 - 1.0, lcg_next(&mut state) * 2.0 - 1.0]);
        }
        let mut triangles = Vec::new();
        for _ in 0..60 {
            let a = (lcg_next(&mut state) * 40.0) as u32 % 40;
            let b = (lcg_next(&mut state) * 40.0) as u32 % 40;
            let c = (lcg_next(&mut state) * 40.0) as u32 % 40;
            if a != b && b != c && a != c {
                triangles.push([a, b, c]);
            }
        }
        let soup = TriMesh { positions, triangles };
        let closed = close_voxel(&soup, 0.15);
        let report = validate_watertight(&closed, false);
        assert!(report.is_closed, "close_voxel output must have zero boundary edges, got {}", report.boundary_edge_count);
        assert!(report.is_two_manifold, "close_voxel output must be a 2-manifold, non-manifold edges={} vertices={}", report.non_manifold_edge_count, report.non_manifold_vertex_count);
    }
    // #endregion 🔖️CloseUnitTest

    // #region 🔖️DenseFieldTests
    #[test]
    fn dense_field_get_set_out_of_range_are_noops() {
        let mut field = DenseField::new(2, 2, 2, [0.0; 3], 1.0, 0.0);
        assert_eq!(field.get(-1, 0, 0), None);
        assert_eq!(field.get(5, 0, 0), None);
        assert_eq!(field.get(0, 5, 0), None);
        field.set(-1, 0, 0, 9.0);
        field.set(0, 0, 0, 9.0);
        assert_eq!(field.get(0, 0, 0), Some(9.0));
    }
    // #endregion 🔖️DenseFieldTests

    // #region 🔖️WatertightSuite
    #[test]
    fn tsdf_sphere_across_blocks_is_watertight_and_correct() {
        let voxel = 0.05;
        let radius = 0.6;
        let vol = build_tsdf_from_sdf(voxel, voxel * 3.0, radius, sphere_sdf(radius));
        let bound = ((radius + voxel * 4.0) / voxel).ceil() as i32;
        let mesh = extract_tsdf(&vol, 0.0, [-bound, -bound, -bound], [bound, bound, bound]);
        let report = validate_watertight(&mesh, false);
        assert_eq!(report.boundary_edge_count, 0, "expected a fully seam-welded sphere with zero boundary edges");
        assert!(report.is_watertight, "sphere across blocks report: {report:?}");
        assert_eq!(report.euler_characteristic, 2, "sphere euler characteristic report: {report:?}");
        let volume = report.signed_volume.abs();
        let expected = 4.0 / 3.0 * std::f64::consts::PI * radius.powi(3);
        let error = (volume - expected).abs() / expected;
        assert!(error < 0.03, "sphere volume error {error} too large: got {volume}, expected {expected}");
    }

    #[test]
    fn tsdf_torus_across_blocks_has_genus_one() {
        let voxel = 0.05;
        let (major, minor) = (0.5, 0.2);
        let vol = build_tsdf_from_sdf(voxel, voxel * 3.0, major + minor, torus_sdf(major, minor));
        let bound = (((major + minor) + voxel * 4.0) / voxel).ceil() as i32;
        let bound_y = ((minor + voxel * 4.0) / voxel).ceil() as i32;
        let mesh = extract_tsdf(&vol, 0.0, [-bound, -bound_y, -bound], [bound, bound_y, bound]);
        let report = validate_watertight(&mesh, false);
        assert!(report.is_watertight, "torus across blocks report: {report:?}");
        assert_eq!(report.euler_characteristic, 0, "torus euler characteristic report: {report:?}");
        assert_eq!(report.genus, Some(1), "torus genus report: {report:?}");
    }

    fn delete_patch(mesh: &mut TriMesh, center_face: usize, target_boundary_verts: usize) {
        let edges = mesh.edge_map();
        let mut face_adjacency: Vec<Vec<u32>> = vec![Vec::new(); mesh.triangles.len()];
        for faces in edges.values() {
            if faces.len() == 2 {
                face_adjacency[faces[0] as usize].push(faces[1]);
                face_adjacency[faces[1] as usize].push(faces[0]);
            }
        }
        let tri_edges = |mesh: &TriMesh, f: u32| -> [(u32, u32); 3] {
            let t = mesh.triangles[f as usize];
            [sorted_edge(t[0], t[1]), sorted_edge(t[1], t[2]), sorted_edge(t[2], t[0])]
        };
        let mut removed: HashSet<u32> = HashSet::new();
        let mut frontier: HashSet<(u32, u32)> = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(center_face as u32);
        removed.insert(center_face as u32);
        for e in tri_edges(mesh, center_face as u32) {
            frontier.insert(e);
        }
        let max_removed_faces = (target_boundary_verts * target_boundary_verts).max(64);
        while frontier.len() < target_boundary_verts && removed.len() < max_removed_faces {
            let Some(f) = queue.pop_front() else { break };
            for &nb in &face_adjacency[f as usize] {
                if !removed.insert(nb) {
                    continue;
                }
                queue.push_back(nb);
                for e in tri_edges(mesh, nb) {
                    if !frontier.remove(&e) {
                        frontier.insert(e);
                    }
                }
                if frontier.len() >= target_boundary_verts || removed.len() >= max_removed_faces {
                    break;
                }
            }
        }
        mesh.triangles = mesh.triangles.iter().enumerate().filter(|&(f, _)| !removed.contains(&(f as u32))).map(|(_, t)| *t).collect();
    }

    #[test]
    fn planted_holes_fill_and_trigger_all_three_strategies() {
        let mut mesh = make_uv_sphere(1.0, 100, 150);
        let original_volume = mesh.signed_volume().abs();
        delete_patch(&mut mesh, 2850, 6);
        delete_patch(&mut mesh, 25350, 35);
        delete_patch(&mut mesh, 14550, 70);
        clean_mesh(&mut mesh, 0, 0.0);
        repair_non_manifold(&mut mesh);
        orient_consistently(&mut mesh).expect("planted-hole sphere stays orientable");
        let stats = fill_holes(&mut mesh, &HoleFillParams::default());
        assert!(stats.ear_fan_used >= 1, "expected ear-fan strategy to trigger, stats={stats:?}");
        assert!(stats.min_weight_dp_used >= 1, "expected min-weight DP strategy to trigger, stats={stats:?}");
        assert!(stats.advancing_front_used >= 1, "expected advancing-front strategy to trigger, stats={stats:?}");
        let report = validate_watertight(&mesh, false);
        assert!(report.is_watertight, "planted-hole sphere after fill_holes report: {report:?}");
        let volume_error = (mesh.signed_volume().abs() - original_volume).abs() / original_volume;
        assert!(volume_error < 0.02, "volume error {volume_error} after hole filling too large");
    }
    // #endregion 🔖️WatertightSuite

    // #region 🔖️HoleFillStatsTests
    #[test]
    fn fill_holes_skips_loops_larger_than_max_boundary_verts() {
        let mut mesh = make_uv_sphere(1.0, 30, 40);
        delete_patch(&mut mesh, 0, 50);
        clean_mesh(&mut mesh, 0, 0.0);
        repair_non_manifold(&mut mesh);
        orient_consistently(&mut mesh).expect("planted-hole sphere stays orientable");
        let stats = fill_holes(&mut mesh, &HoleFillParams { max_boundary_verts: 10 });
        assert!(stats.holes_skipped_too_large >= 1, "stats={stats:?}");
        assert_eq!(stats.holes_filled, 0, "stats={stats:?}");
    }
    // #endregion 🔖️HoleFillStatsTests

    // #region 🔖️RepairTests
    fn two_spheres_sharing_edge() -> TriMesh {
        let a = make_uv_sphere(1.0, 10, 10);
        let mut b = make_uv_sphere(1.0, 10, 10);
        for p in &mut b.positions {
            p[0] += 2.0;
        }
        let shift = a.positions.len() as u32;
        let shared_a = a.triangles[0][0];
        let shared_a2 = a.triangles[0][1];
        for tri in &mut b.triangles {
            for v in tri.iter_mut() {
                *v += shift;
            }
        }
        let shared_b = b.triangles[0][0];
        let shared_b2 = b.triangles[0][1];
        let mut mesh = a;
        mesh.positions.extend(b.positions);
        mesh.triangles.extend(b.triangles);
        for tri in &mut mesh.triangles {
            for v in tri.iter_mut() {
                if *v == shared_b {
                    *v = shared_a;
                } else if *v == shared_b2 {
                    *v = shared_a2;
                }
            }
        }
        mesh
    }

    fn two_spheres_sharing_vertex() -> TriMesh {
        let a = make_uv_sphere(1.0, 10, 10);
        let mut b = make_uv_sphere(1.0, 10, 10);
        for p in &mut b.positions {
            p[0] += 2.0;
        }
        let shift = a.positions.len() as u32;
        let shared_a = a.triangles[0][0];
        for tri in &mut b.triangles {
            for v in tri.iter_mut() {
                *v += shift;
            }
        }
        let shared_b = b.triangles[0][0];
        let mut mesh = a;
        mesh.positions.extend(b.positions);
        mesh.triangles.extend(b.triangles);
        for tri in &mut mesh.triangles {
            for v in tri.iter_mut() {
                if *v == shared_b {
                    *v = shared_a;
                }
            }
        }
        mesh
    }

    #[test]
    fn repair_splits_bowtie_edge_into_two_components() {
        let mut mesh = two_spheres_sharing_edge();
        let stats = repair_non_manifold(&mut mesh);
        assert!(stats.non_manifold_edges_split >= 1, "expected at least one non-manifold edge split, stats={stats:?}");
        let report = validate_watertight(&mesh, false);
        assert_eq!(report.non_manifold_edge_count, 0, "report: {report:?}");
        assert_eq!(report.non_manifold_vertex_count, 0, "report: {report:?}");
        assert_eq!(report.connected_components, 2, "report: {report:?}");
    }

    #[test]
    fn repair_splits_pinch_vertex_into_two_components() {
        let mut mesh = two_spheres_sharing_vertex();
        let stats = repair_non_manifold(&mut mesh);
        assert!(stats.non_manifold_vertices_split >= 1, "expected at least one pinch vertex split, stats={stats:?}");
        let report = validate_watertight(&mesh, false);
        assert_eq!(report.non_manifold_edge_count, 0, "report: {report:?}");
        assert_eq!(report.non_manifold_vertex_count, 0, "report: {report:?}");
        assert_eq!(report.connected_components, 2, "report: {report:?}");
    }

    #[test]
    fn orient_consistently_recovers_from_flipped_windings() {
        let mut mesh = make_uv_sphere(1.0, 20, 30);
        let mut state = 7u64;
        for tri in &mut mesh.triangles {
            if lcg_next(&mut state) < 0.3 {
                tri.swap(1, 2);
            }
        }
        orient_consistently(&mut mesh).expect("sphere with flipped windings is still orientable");
        orient_outward(&mut mesh);
        let report = validate_watertight(&mesh, false);
        assert!(report.consistently_oriented, "report: {report:?}");
        assert!(report.signed_volume > 0.0, "report: {report:?}");
    }
    // #endregion 🔖️RepairTests

    // #region 🔖️CleanTests
    #[test]
    fn clean_mesh_welds_duplicate_vertices() {
        let mut mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]], triangles: vec![[0, 1, 2], [3, 1, 2]] };
        let stats = clean_mesh(&mut mesh, 0, 0.0);
        assert_eq!(stats.vertices_welded, 1, "stats={stats:?}");
        assert_eq!(mesh.vertex_count(), 3);
    }

    #[test]
    fn clean_mesh_removes_degenerate_triangles() {
        let mut mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 1.0, 0.0]], triangles: vec![[0, 1, 2], [0, 1, 3]] };
        let stats = clean_mesh(&mut mesh, 0, 0.0);
        assert_eq!(stats.degenerate_triangles_removed, 1, "stats={stats:?}");
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn clean_mesh_collapses_near_zero_length_edges() {
        let mut mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1e-13, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], triangles: vec![[0, 1, 3], [1, 2, 3]] };
        let stats = clean_mesh(&mut mesh, 0, 0.0);
        assert!(stats.zero_length_edges_collapsed >= 1, "stats={stats:?}");
    }

    #[test]
    fn clean_mesh_removes_small_disconnected_components() {
        let mut mesh = make_uv_sphere(1.0, 10, 10);
        let shift = mesh.positions.len() as u32;
        mesh.positions.extend([[100.0, 100.0, 100.0], [100.01, 100.0, 100.0], [100.0, 100.01, 100.0]]);
        mesh.triangles.push([shift, shift + 1, shift + 2]);
        let stats = clean_mesh(&mut mesh, 8, 0.02);
        assert_eq!(stats.small_components_removed, 1, "stats={stats:?}");
    }
    // #endregion 🔖️CleanTests

    // #region 🔖️TaubinTests
    #[test]
    fn taubin_smooth_noop_on_empty_mesh() {
        let mut mesh = TriMesh::new();
        taubin_smooth(&mut mesh, 0.5, -0.53, 5);
        assert!(mesh.positions.is_empty());
    }

    #[test]
    fn taubin_smooth_reduces_vertex_noise_amplitude() {
        let mut state = 99u64;
        let mut mesh = make_uv_sphere(1.0, 20, 30);
        for p in &mut mesh.positions {
            let n = normalize3(*p);
            *p = add3(*p, scale3(n, (lcg_next(&mut state) - 0.5) * 0.05));
        }
        let radius_variance = |positions: &[[f64; 3]]| -> f64 {
            let radii: Vec<f64> = positions.iter().map(|p| norm3(*p)).collect();
            let mean = radii.iter().sum::<f64>() / radii.len() as f64;
            radii.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / radii.len() as f64
        };
        let noisy_variance = radius_variance(&mesh.positions);
        taubin_smooth(&mut mesh, 0.5, -0.53, 10);
        let smoothed_variance = radius_variance(&mesh.positions);
        assert!(smoothed_variance < noisy_variance, "expected taubin smoothing to reduce radius variance: before={noisy_variance} after={smoothed_variance}");
    }
    // #endregion 🔖️TaubinTests

    // #region 🔖️SimplifyTests
    #[test]
    fn qem_simplification_preserves_watertight_invariant() {
        let mut mesh = make_uv_sphere(1.0, 30, 45);
        let before = validate_watertight(&mesh, false);
        assert!(before.is_watertight, "sanity: uv sphere must start watertight, report: {before:?}");
        let target = (mesh.triangles.len() as f64 * 0.2) as usize;
        let stats = simplify_qem(&mut mesh, target, &SimplifyParams::default());
        assert!(stats.collapses_performed > 0, "expected simplification to perform collapses, stats={stats:?}");
        let after = validate_watertight(&mesh, false);
        assert!(after.is_watertight, "simplified sphere must stay watertight, report: {after:?}");
    }

    #[test]
    fn simplify_qem_rejects_collapses_that_violate_link_condition() {
        let mut mesh = two_spheres_sharing_edge();
        let stats = simplify_qem(&mut mesh, 4, &SimplifyParams::default());
        assert!(stats.collapses_rejected_by_link_condition > 0, "expected the shared bowtie edge to force at least one link-condition rejection, stats={stats:?}");
    }

    #[test]
    fn simplify_qem_stops_when_error_exceeds_max_error() {
        let mut mesh = make_uv_sphere(1.0, 20, 30);
        let total_before = mesh.triangle_count();
        let stats = simplify_qem(&mut mesh, 0, &SimplifyParams { max_error: 1e-6 });
        assert!(stats.collapses_performed > 0, "some cheap collapses should still happen, stats={stats:?}");
        assert!(mesh.triangle_count() > 0, "max_error should halt simplification before the mesh disappears, stats={stats:?}");
        assert!(mesh.triangle_count() < total_before, "expected at least some simplification, stats={stats:?}");
    }
    // #endregion 🔖️SimplifyTests

    // #region 🔖️SegmentChartsTests
    #[test]
    fn segment_charts_keeps_coplanar_faces_in_one_chart() {
        let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], triangles: vec![[0, 1, 2], [0, 2, 3]] };
        let charts = segment_charts(&mesh, 10.0);
        assert_eq!(charts.len(), 1, "coplanar faces within threshold should stay in one chart");
        assert_eq!(charts[0].faces.len(), 2);
    }

    #[test]
    fn segment_charts_cuts_at_sharp_dihedral_angle() {
        let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]], triangles: vec![[0, 1, 2], [0, 2, 3], [0, 3, 5], [0, 5, 4]] };
        let charts = segment_charts(&mesh, 45.0);
        assert_eq!(charts.len(), 2, "a 90-degree fold above a 45-degree threshold should split into two charts");
        for chart in &charts {
            assert_eq!(chart.faces.len(), 2);
        }
    }
    // #endregion 🔖️SegmentChartsTests

    // #region 🔖️UnwrapTests
    #[test]
    fn lscm_unwrap_is_bijective_per_chart() {
        let mut mesh = make_uv_sphere(1.0, 12, 18);
        let target = (mesh.triangles.len() as f64 * 0.5) as usize;
        simplify_qem(&mut mesh, target, &SimplifyParams::default());
        let report = validate_watertight(&mesh, false);
        assert!(report.is_watertight, "sanity: simplified sphere must be watertight before unwrap, report: {report:?}");
        let charts = segment_charts(&mesh, 60.0);
        let uvs = unwrap_mesh(&mut mesh, &charts);
        for chart in &charts {
            let mut sign: Option<f64> = None;
            for &f in &chart.faces {
                let tri = mesh.triangles[f as usize];
                let (a, b, c) = (uvs[tri[0] as usize], uvs[tri[1] as usize], uvs[tri[2] as usize]);
                let area2 = f64::from(b[0] - a[0]) * f64::from(c[1] - a[1]) - f64::from(c[0] - a[0]) * f64::from(b[1] - a[1]);
                if area2.abs() < 1e-12 {
                    continue;
                }
                let s = area2.signum();
                if let Some(prev) = sign {
                    assert_eq!(prev, s, "chart has inconsistent UV triangle winding (an inversion)");
                }
                sign = Some(s);
            }
        }
    }
    // #endregion 🔖️UnwrapTests

    // #region 🔖️SelfIntersectionTests
    #[test]
    fn validate_watertight_detects_self_intersections_when_requested() {
        let mesh = TriMesh { positions: vec![[-1.0, 0.0, -1.0], [1.0, 0.0, -1.0], [0.0, 0.0, 2.0], [0.0, -1.0, -0.5], [0.0, 1.0, -0.5], [0.0, 0.0, 1.0]], triangles: vec![[0, 1, 2], [3, 4, 5]] };
        let report = validate_watertight(&mesh, true);
        assert_eq!(report.self_intersection_pairs, Some(1), "report: {report:?}");
    }

    #[test]
    fn validate_watertight_reports_no_self_intersections_for_disjoint_triangles() {
        let mesh = TriMesh { positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [10.0, 10.0, 10.0], [11.0, 10.0, 10.0], [10.0, 11.0, 10.0]], triangles: vec![[0, 1, 2], [3, 4, 5]] };
        let report = validate_watertight(&mesh, true);
        assert_eq!(report.self_intersection_pairs, Some(0), "report: {report:?}");
    }
    // #endregion 🔖️SelfIntersectionTests

    // #region 🔖️ContractTest
    #[test]
    fn pipeline_falls_back_to_close_on_pathological_input() {
        let mut mesh = make_uv_sphere(1.0, 40, 60);
        delete_patch(&mut mesh, 0, 60);
        let mut overlapping = make_uv_sphere(1.0, 8, 8);
        for p in &mut overlapping.positions {
            p[0] += 0.3;
        }
        let shift = mesh.positions.len() as u32;
        for tri in &overlapping.triangles {
            mesh.triangles.push([tri[0] + shift, tri[1] + shift, tri[2] + shift]);
        }
        mesh.positions.extend(overlapping.positions);
        let params = MeshParams { guarantee_watertight: true, hole_fill_max_boundary_verts: 20, ..MeshParams::default() };
        let mut pipeline = MeshPipeline::from_mesh(mesh, params);
        let mut status = mesh_pipeline_step(&mut pipeline, 1);
        let mut guard = 0;
        while !matches!(status, MeshPipelineStatus::Done | MeshPipelineStatus::Failed(_)) {
            status = mesh_pipeline_step(&mut pipeline, 1);
            guard += 1;
            assert!(guard < 100, "pipeline did not terminate within a reasonable number of steps");
        }
        assert!(matches!(status, MeshPipelineStatus::Done), "pipeline status: {status:?}");
        let report = pipeline.report().expect("pipeline recorded a watertight report");
        assert!(report.closed_fallback_used, "expected the pathological mesh to trigger close_voxel, report: {report:?}");
        assert!(report.is_watertight, "expected close_voxel's output to be watertight, report: {report:?}");
    }

    /// 🪸️ A tiny UV-sphere island with one small planted hole (an ear-fannable, individually
    /// closable defect on its own), positioned far from the origin so it shares no vertices or
    /// bounding volume with any other island.
    fn make_tiny_holed_island(index: usize) -> TriMesh {
        let mut island = make_uv_sphere(0.05, 6, 6);
        delete_patch(&mut island, 0, 6);
        let offset = [(index % 12) as f64 * 0.6, ((index / 12) % 12) as f64 * 0.6, (index / 144) as f64 * 0.6];
        for p in &mut island.positions {
            *p = add3(*p, offset);
        }
        island
    }

    /// 🪸️ Simulates a badly under-registered SfM/TSDF reconstruction reaching `remodel_mesh`: not
    /// one hand-crafted pathological shape, but hundreds of small, mutually disjoint, individually
    /// well-formed islands — the real-world failure mode a fresh 48-frame orbiting-cube end-to-end
    /// diagnostic actually produced (34280 vertices / 44244 triangles / 1321 components / 22234
    /// boundary edges going into this pipeline).
    fn make_scattered_fragment_soup(count: usize) -> TriMesh {
        let mut mesh = TriMesh::new();
        for i in 0..count {
            let island = make_tiny_holed_island(i);
            let shift = mesh.positions.len() as u32;
            for tri in &island.triangles {
                mesh.triangles.push([tri[0] + shift, tri[1] + shift, tri[2] + shift]);
            }
            mesh.positions.extend(island.positions);
        }
        mesh
    }

    #[test]
    fn pipeline_falls_back_to_close_on_catastrophically_fragmented_reconstruction() {
        let mesh = make_scattered_fragment_soup(300);
        let raw_report = validate_watertight(&mesh, false);
        assert!(raw_report.connected_components >= 250, "expected the fragment soup to actually be badly fragmented, report: {raw_report:?}");
        assert!(raw_report.boundary_edge_count > 0, "expected the raw fragment soup to have real boundary edges, report: {raw_report:?}");
        let params = MeshParams::default();
        assert!(params.guarantee_watertight, "sanity: the guarantee must default on, exactly as the real diagnostic observed");
        let mut pipeline = MeshPipeline::from_mesh(mesh, params);
        let mut status = mesh_pipeline_step(&mut pipeline, 1);
        let mut guard = 0;
        while !matches!(status, MeshPipelineStatus::Done | MeshPipelineStatus::Failed(_)) {
            status = mesh_pipeline_step(&mut pipeline, 1);
            guard += 1;
            assert!(guard < 100, "pipeline did not terminate within a reasonable number of steps");
        }
        assert!(matches!(status, MeshPipelineStatus::Done), "pipeline status: {status:?}");
        let report = pipeline.report().expect("pipeline recorded a watertight report");
        assert!(
            report.closed_fallback_used,
            "expected hundreds of small individually-closable islands to still trip the close_voxel guarantee (fill_holes can legitimately seal every island's tiny hole one at a time without the aggregate mesh ever being one coherent solid), report: {report:?}"
        );
        assert!(report.is_watertight, "expected close_voxel's output to be watertight, report: {report:?}");
    }
    // #endregion 🔖️ContractTest

    // #region 🔖️TextureTests
    #[test]
    fn image_gradient_magnitude_zero_on_flat_image() {
        let img = solid_color_image(32, 32, 100);
        assert_eq!(image_gradient_magnitude(&img, 16.0, 16.0), 0.0);
    }

    #[test]
    fn image_gradient_magnitude_detects_vertical_edge() {
        let img = vertical_edge_image(32, 32);
        let at_edge = image_gradient_magnitude(&img, 16.0, 16.0);
        let away_from_edge = image_gradient_magnitude(&img, 4.0, 16.0);
        assert!(at_edge > 0.9, "expected a near-maximal normalized gradient right at the edge, got {at_edge}");
        assert_eq!(away_from_edge, 0.0, "expected zero gradient away from the edge, got {away_from_edge}");
    }

    #[test]
    fn face_projected_area_none_behind_camera_some_in_front() {
        let intr = intrinsics_for(64, 64);
        let identity_pose = remodel_camera::CameraPose(math::lie::Se3 { r: math::lie::So3(math::algebra::Mat3d::IDENTITY), t: [0.0, 0.0, 0.0] });
        let behind = TriMesh { positions: vec![[-0.1, -0.1, -1.0], [0.1, -0.1, -1.0], [0.0, 0.1, -1.0]], triangles: vec![[0, 1, 2]] };
        assert!(face_projected_area(&behind, 0, &intr, &identity_pose).is_none());
        let front = TriMesh { positions: vec![[-0.1, -0.1, 2.0], [0.1, -0.1, 2.0], [0.0, 0.1, 2.0]], triangles: vec![[0, 1, 2]] };
        let area = face_projected_area(&front, 0, &intr, &identity_pose).expect("triangle in front of camera projects");
        assert!(area > 0.0, "expected a positive projected area, got {area}");
    }

    #[test]
    fn level_seam_solves_gain_and_offset() {
        let a: Vec<[f32; 3]> = (0..5).map(|i| [f64::from(i) as f32 * 10.0; 3]).collect();
        let b: Vec<[f32; 3]> = a.iter().map(|p| [p[0] * 2.0 + 5.0, p[1] * 2.0 + 5.0, p[2] * 2.0 + 5.0]).collect();
        let fit = level_seam(&a, &b);
        for (gain, offset) in fit {
            assert!((gain - 2.0).abs() < 1e-3, "fit={fit:?}");
            assert!((offset - 5.0).abs() < 1e-2, "fit={fit:?}");
        }
    }

    #[test]
    fn level_seam_returns_identity_for_insufficient_samples() {
        assert_eq!(level_seam(&[[1.0, 2.0, 3.0]], &[[4.0, 5.0, 6.0]]), [(1.0, 0.0); 3]);
    }

    #[test]
    fn bake_texture_paints_atlas_from_multiple_views() {
        let mut mesh = make_uv_sphere(0.4, 10, 14);
        let target = (mesh.triangle_count() as f64 * 0.3) as usize;
        simplify_qem(&mut mesh, target, &SimplifyParams::default());
        let charts = segment_charts(&mesh, 60.0);
        let uvs = unwrap_mesh(&mut mesh, &charts);
        let intr = intrinsics_for(64, 64);
        let views: Vec<TextureView> = orbit_views(2.0).into_iter().take(8).map(|pose| TextureView { pose, intrinsics: intr, image: checkerboard_image(64, 64, 8) }).collect();
        let atlas = bake_texture(&mesh, &uvs, 64, &views);
        let painted = atlas.data.chunks(4).filter(|px| px[3] == 255).count();
        assert!(painted > 0, "expected bake_texture to paint at least some atlas pixels from {} views", views.len());
    }

    #[test]
    fn bake_texture_is_empty_with_no_views() {
        let mut mesh = make_uv_sphere(0.4, 6, 8);
        let charts = segment_charts(&mesh, 60.0);
        let uvs = unwrap_mesh(&mut mesh, &charts);
        let atlas = bake_texture(&mesh, &uvs, 32, &[]);
        assert!(atlas.data.iter().all(|&b| b == 0), "expected an untouched (fully transparent) atlas with no views");
    }
    // #endregion 🔖️TextureTests

    mod long {
        use super::*;

        #[test]
        fn full_pipeline_from_tsdf_sphere_is_watertight() {
            let voxel = 0.05;
            let radius = 0.6;
            let vol = build_tsdf_from_sdf(voxel, voxel * 3.0, radius, sphere_sdf(radius));
            let bound = ((radius + voxel * 4.0) / voxel).ceil() as i32;
            let params = MeshParams { target_triangles: 400, ..MeshParams::default() };
            let mut pipeline = MeshPipeline::new(&vol, 0.0, [-bound, -bound, -bound], [bound, bound, bound], params);
            let mut status = mesh_pipeline_step(&mut pipeline, 1);
            let mut guard = 0;
            while !matches!(status, MeshPipelineStatus::Done | MeshPipelineStatus::Failed(_)) {
                status = mesh_pipeline_step(&mut pipeline, 1);
                guard += 1;
                assert!(guard < 100, "pipeline did not terminate");
            }
            assert!(matches!(status, MeshPipelineStatus::Done), "pipeline status: {status:?}");
            let report = pipeline.report().expect("pipeline recorded a report");
            assert!(report.is_watertight, "full pipeline report: {report:?}");
            let mesh_data = pipeline.result().expect("pipeline produced mesh data");
            assert!(!mesh_data.positions.is_empty());
            assert!(!mesh_data.uvs.is_empty());
        }

        #[test]
        fn full_pipeline_with_views_bakes_and_encodes_texture() {
            let mesh = make_uv_sphere(0.4, 10, 14);
            let intr = intrinsics_for(48, 48);
            let views: Vec<TextureView> = orbit_views(1.5).into_iter().take(6).map(|pose| TextureView { pose, intrinsics: intr, image: checkerboard_image(48, 48, 6) }).collect();
            let params = MeshParams { target_triangles: 150, ..MeshParams::default() };
            let mut pipeline = MeshPipeline::from_mesh(mesh, params).with_views(views);
            let mut status = mesh_pipeline_step(&mut pipeline, 1);
            let mut guard = 0;
            while !matches!(status, MeshPipelineStatus::Done | MeshPipelineStatus::Failed(_)) {
                status = mesh_pipeline_step(&mut pipeline, 1);
                guard += 1;
                assert!(guard < 100, "pipeline did not terminate");
            }
            assert!(matches!(status, MeshPipelineStatus::Done), "pipeline status: {status:?}");
            let mesh_data = pipeline.result().expect("pipeline produced mesh data");
            assert!(mesh_data.paint_texture_base64.is_some(), "expected texture bake+encode to populate paint_texture_base64");
        }
    }
}
// #endregion 🔖️Tests
