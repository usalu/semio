//! 📐️ Triangle mesh of the mesh surface: the shared 3-vector helpers, the indexed triangle mesh with its edge map and the disjoint set. See [`super`].

use super::*;

// #region 🔖️TriMesh
/// ➕️ Adds two 3-vectors.
pub fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// ➖️ Subtracts two 3-vectors.
pub fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// ✖️ Scales a 3-vector.
pub fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

/// 🔀️ Cross product of two 3-vectors.
pub fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// 🔵️ Dot product of two 3-vectors.
pub fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// 📏️ Euclidean norm of a 3-vector.
pub fn norm3(a: [f64; 3]) -> f64 {
    dot3(a, a).sqrt()
}

/// 🧭️ Unit-length copy of `a`, or `[0,0,0]` when `a` is (numerically) the zero vector.
pub fn normalize3(a: [f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-15 {
        [0.0, 0.0, 0.0]
    } else {
        scale3(a, 1.0 / n)
    }
}

/// 🎯️ Linear interpolation between two 3-points.
pub fn lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    add3(a, scale3(sub3(b, a), t))
}

/// 🧮️ Builds a dense `MatD` from row-major `f64` rows (`MatD` itself only exposes `zeros`/`set`,
/// unlike the generic `MatG<T>::from_rows`).
pub fn matd_from_rows(rows: &[Vec<f64>]) -> semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::MatD {
    let r = rows.len();
    let c = rows.first().map_or(0, Vec::len);
    let mut m = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::MatD::zeros(r, c);
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
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct TriMesh {
    pub positions: Vec<[f64; 3]>,
    pub triangles: Vec<[u32; 3]>,
}

/// 🗺️ Sorted undirected vertex pair → incident face indices, deterministic-order workhorse for
/// manifold checks, boundary detection and orientation flooding.
pub type EdgeMap = BTreeMap<(u32, u32), Vec<u32>>;

/// 🔗️ Canonical (sorted) undirected edge key for a vertex pair.
pub fn sorted_edge(a: u32, b: u32) -> (u32, u32) {
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
#[derive(semio_framework_value::RetireOwned)]
pub struct DisjointSet {
    pub(super) parent: Vec<u32>,
    size: Vec<u32>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self { parent: (0..n as u32).collect(), size: vec![1; n] }
    }

    pub fn find(&mut self, x: u32) -> u32 {
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

    pub fn union(&mut self, a: u32, b: u32) {
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
