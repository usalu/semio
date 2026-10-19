//! 🔢️ (wedge/hex) with tet splitting and boundary faces, the element quality report, and the binary64 region
//! 🧬️ Meshes derived from a triangulation or a rectangle: structured quad grids, quadratic promotion, 3D extrusion
//! volume/surface records.

use std::collections::HashMap;
use semio_s_fem_mesh::TriMesh2;

// #region 🔖️QuadMesh2
/// 🔲️ A structured quad mesh: shared node positions plus quads as index quadruples into `points`.
#[derive(Clone, Debug, PartialEq)]
pub struct QuadMesh2 {
    pub points: Vec<[f64; 2]>,
    pub quads: Vec<[u32; 4]>,
}

/// 🔲️ An `nx` x `ny` structured grid of quads over an axis-aligned rectangle `[x0,x1] x [y0,y1]`,
/// row-major point numbering, each quad wound `[bottom-left, bottom-right, top-right, top-left]`.
pub fn quad_grid(x0: f64, y0: f64, x1: f64, y1: f64, nx: usize, ny: usize) -> QuadMesh2 {
    let mut points = Vec::with_capacity((nx + 1) * (ny + 1));
    for j in 0..=ny {
        for i in 0..=nx {
            let x = x0 + (x1 - x0) * (i as f64) / (nx as f64);
            let y = y0 + (y1 - y0) * (j as f64) / (ny as f64);
            points.push([x, y]);
        }
    }
    let index = |i: usize, j: usize| (j * (nx + 1) + i) as u32;
    let mut quads = Vec::with_capacity(nx * ny);
    for j in 0..ny {
        for i in 0..nx {
            quads.push([index(i, j), index(i + 1, j), index(i + 1, j + 1), index(i, j + 1)]);
        }
    }
    QuadMesh2 { points, quads }
}
// #endregion 🔖️QuadMesh2

// #region 🔖️Quadratic
/// 🔺️ A quadratic-promoted triangle mesh: shared node positions (originals first, then appended
/// mid-edge points) plus 6-node triangles.
#[derive(Clone, Debug, PartialEq)]
pub struct TriMesh2Quadratic {
    pub points: Vec<[f64; 2]>,
    pub tris6: Vec<[u32; 6]>,
}

/// 🔗️ Looks up (or creates, welding shared edges to exactly one mid-node) the mid-edge point index
/// for edge `(a,b)`, keyed by the sorted `(min,max)` index pair.
fn mid_index(a: u32, b: u32, points: &mut Vec<[f64; 2]>, edge_mid: &mut HashMap<(u32, u32), u32>) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(&idx) = edge_mid.get(&key) {
        return idx;
    }
    let pa = points[a as usize];
    let pb = points[b as usize];
    let idx = points.len() as u32;
    points.push([(pa[0] + pb[0]) * 0.5, (pa[1] + pb[1]) * 0.5]);
    edge_mid.insert(key, idx);
    idx
}

/// 🔺️ Promotes a linear `TriMesh2` to quadratic by inserting a mid-edge node per unique edge (shared
/// edges between adjacent triangles get exactly ONE mid-node, deduped by sorted `(min,max)` edge key).
/// Original points keep their indices unchanged; mid-edge points are appended after them. Each
/// triangle's 6 node indices follow `[n0,n1,n2, mid(n0,n1), mid(n1,n2), mid(n2,n0)]` — the standard
/// Tri6 convention (matches `elements2d.rs`'s `shape_tri6` node ordering, documented here since that
/// function may land concurrently with this module).
pub fn to_quadratic(mesh: &TriMesh2) -> TriMesh2Quadratic {
    let mut points = mesh.points.clone();
    let mut edge_mid: HashMap<(u32, u32), u32> = HashMap::new();
    let mut tris6 = Vec::with_capacity(mesh.tris.len());

    for tri in &mesh.tris {
        let [n0, n1, n2] = *tri;
        let m01 = mid_index(n0, n1, &mut points, &mut edge_mid);
        let m12 = mid_index(n1, n2, &mut points, &mut edge_mid);
        let m20 = mid_index(n2, n0, &mut points, &mut edge_mid);
        tris6.push([n0, n1, n2, m01, m12, m20]);
    }

    TriMesh2Quadratic { points, tris6 }
}
// #endregion 🔖️Quadratic

// #region 🔖️VolumeMesh
/// 🧱️ One volumetric cell — a linear wedge/hex prism or a tet, as index tuples into `VolumeMesh::points`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Wedge6([u32; 6]),
    Hex8([u32; 8]),
    Tet4([u32; 4]),
}

/// 🧱️ A volumetric mesh: shared node positions plus cells.
#[derive(Clone, Debug, PartialEq)]
pub struct VolumeMesh {
    pub points: Vec<[f64; 3]>,
    pub cells: Vec<Cell>,
}

/// 🧱️ Extrudes a flat `TriMesh2` (lying in the z=0 plane) along +z by `height`, split into `layers`
/// equal-height layers, producing one `Wedge6` per (triangle, layer) — node order
/// `[bottom0,bottom1,bottom2, top0,top1,top2]` (bottom face matches the triangle's own `[n0,n1,n2]`
/// winding, top face directly above).
pub fn extrude_tri_mesh(mesh: &TriMesh2, height: f64, layers: usize) -> VolumeMesh {
    let layers = layers.max(1);
    let n = mesh.points.len();
    let mut points = Vec::with_capacity(n * (layers + 1));
    for l in 0..=layers {
        let z = height * (l as f64) / (layers as f64);
        for p in &mesh.points {
            points.push([p[0], p[1], z]);
        }
    }
    let mut cells = Vec::with_capacity(mesh.tris.len() * layers);
    for l in 0..layers {
        let bottom_off = (l * n) as u32;
        let top_off = ((l + 1) * n) as u32;
        for tri in &mesh.tris {
            let [a, b, c] = *tri;
            cells.push(Cell::Wedge6([bottom_off + a, bottom_off + b, bottom_off + c, top_off + a, top_off + b, top_off + c]));
        }
    }
    VolumeMesh { points, cells }
}

/// 🧱️ Extrudes a flat `QuadMesh2` along +z by `height` into `layers` layers of `Hex8` cells — node
/// order `[bottom0,bottom1,bottom2,bottom3, top0,top1,top2,top3]` matching the quad's own winding.
pub fn extrude_quad_mesh(mesh: &QuadMesh2, height: f64, layers: usize) -> VolumeMesh {
    let layers = layers.max(1);
    let n = mesh.points.len();
    let mut points = Vec::with_capacity(n * (layers + 1));
    for l in 0..=layers {
        let z = height * (l as f64) / (layers as f64);
        for p in &mesh.points {
            points.push([p[0], p[1], z]);
        }
    }
    let mut cells = Vec::with_capacity(mesh.quads.len() * layers);
    for l in 0..layers {
        let bottom_off = (l * n) as u32;
        let top_off = ((l + 1) * n) as u32;
        for quad in &mesh.quads {
            let [a, b, c, d] = *quad;
            cells.push(Cell::Hex8([bottom_off + a, bottom_off + b, bottom_off + c, bottom_off + d, top_off + a, top_off + b, top_off + c, top_off + d]));
        }
    }
    VolumeMesh { points, cells }
}

/// ✂️ Splits a quad face `[a,b,c,d]` (in winding order, so `a`-`c` and `b`-`d` are the two diagonals)
/// into 2 triangles, choosing the diagonal FROM the corner with the smallest global point index. This
/// depends only on the face's own 4 global indices (not on cell/apex choice), so two cells sharing a
/// quad face always agree — the parity-consistency guarantee `split_to_tets` relies on.
fn split_quad_face(a: u32, b: u32, c: u32, d: u32) -> [[u32; 3]; 2] {
    let min = a.min(b).min(c).min(d);
    if min == a || min == c {
        [[a, b, c], [a, c, d]]
    } else {
        [[a, b, d], [b, c, d]]
    }
}

/// 🔺️ Fan-triangulates a convex cell's boundary from `apex` (one of the cell's own vertices): every
/// quad face is split via [`split_quad_face`], every triangular face passes through as-is, and every
/// resulting boundary triangle that does NOT already contain `apex` becomes a tet `(apex, t0, t1, t2)`
/// — the standard star/cone decomposition of a convex polyhedron, valid since convexity guarantees no
/// overlap/gaps. Faces touching `apex` need no explicit tet: their volume is degenerate (zero) from
/// `apex`'s own cone and is instead captured as internal faces of tets from adjacent, non-apex faces.
fn split_cell_to_tets(quad_faces: &[[u32; 4]], tri_faces: &[[u32; 3]], apex: u32) -> Vec<[u32; 4]> {
    let mut tets = Vec::new();
    for &[a, b, c, d] in quad_faces {
        for tri in split_quad_face(a, b, c, d) {
            if !tri.contains(&apex) {
                tets.push([apex, tri[0], tri[1], tri[2]]);
            }
        }
    }
    for &tri in tri_faces {
        if !tri.contains(&apex) {
            tets.push([apex, tri[0], tri[1], tri[2]]);
        }
    }
    tets
}

/// 🔪️ Splits every `Wedge6`/`Hex8` cell into `Tet4` cells (`Wedge6` → 3 tets, `Hex8` → 6 tets), using
/// the minimum-global-node-index apex + face-diagonal rule so adjacent cells split their SHARED quad
/// faces identically (see [`split_quad_face`]). `Tet4` cells in the input pass through unchanged.
pub fn split_to_tets(mesh: &VolumeMesh) -> VolumeMesh {
    let mut cells = Vec::with_capacity(mesh.cells.len());
    for cell in &mesh.cells {
        match cell {
            Cell::Tet4(t) => cells.push(Cell::Tet4(*t)),
            Cell::Wedge6(w) => {
                let [n0, n1, n2, n3, n4, n5] = *w;
                let apex = w.iter().copied().min().unwrap();
                let quads = [[n0, n1, n4, n3], [n1, n2, n5, n4], [n2, n0, n3, n5]];
                let tris = [[n0, n1, n2], [n3, n4, n5]];
                for tet in split_cell_to_tets(&quads, &tris, apex) {
                    cells.push(Cell::Tet4(tet));
                }
            }
            Cell::Hex8(h) => {
                let [n0, n1, n2, n3, n4, n5, n6, n7] = *h;
                let apex = h.iter().copied().min().unwrap();
                let quads = [[n0, n1, n2, n3], [n4, n5, n6, n7], [n0, n1, n5, n4], [n1, n2, n6, n5], [n2, n3, n7, n6], [n3, n0, n4, n7]];
                for tet in split_cell_to_tets(&quads, &[], apex) {
                    cells.push(Cell::Tet4(tet));
                }
            }
        }
    }
    VolumeMesh { points: mesh.points.clone(), cells }
}

/// 🧭️ The average of `mesh.points` at `idxs` — shared by `boundary_faces`'s per-tet and per-face
/// centroid computations.
fn point_centroid(mesh: &VolumeMesh, idxs: &[u32]) -> [f64; 3] {
    let mut c = [0.0; 3];
    for &i in idxs {
        let p = mesh.points[i as usize];
        for k in 0..3 {
            c[k] += p[k];
        }
    }
    let n = idxs.len() as f64;
    [c[0] / n, c[1] / n, c[2] / n]
}

/// 🧱️ Every triangular face belonging to EXACTLY ONE `Tet4` cell — the mesh's outer surface (call
/// [`split_to_tets`] first if `mesh` still has `Wedge6`/`Hex8` cells; those contribute no faces here).
/// Each returned triangle is independently wound so its `cross(edge0,edge1)` normal points AWAY from
/// its own tet's centroid (outward) — determined per-tet via a centroid side-test, so the result
/// doesn't depend on any input node-order convention. Used by `fem_3d`'s solid mesh preview/rendering.
pub fn boundary_faces(mesh: &VolumeMesh) -> Vec<[u32; 3]> {
    let mut counts: HashMap<[u32; 3], usize> = HashMap::new();
    let mut oriented: HashMap<[u32; 3], [u32; 3]> = HashMap::new();

    for cell in &mesh.cells {
        let Cell::Tet4(t) = cell else { continue };
        let [n0, n1, n2, n3] = *t;
        let tet_centroid = point_centroid(mesh, &[n0, n1, n2, n3]);

        for face in [[n0, n1, n2], [n0, n1, n3], [n0, n2, n3], [n1, n2, n3]] {
            let mut key = face;
            key.sort_unstable();
            *counts.entry(key).or_insert(0) += 1;

            let p = |i: u32| mesh.points[i as usize];
            let (a, b, c) = (p(face[0]), p(face[1]), p(face[2]));
            let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let normal = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            let face_centroid = point_centroid(mesh, &face);
            let to_tet = [tet_centroid[0] - face_centroid[0], tet_centroid[1] - face_centroid[1], tet_centroid[2] - face_centroid[2]];
            let dot = normal[0] * to_tet[0] + normal[1] * to_tet[1] + normal[2] * to_tet[2];
            let outward = if dot > 0.0 { [face[0], face[2], face[1]] } else { face };
            oriented.insert(key, outward);
        }
    }

    counts.into_iter().filter(|(_, count)| *count == 1).map(|(key, _)| oriented[&key]).collect()
}
// #endregion 🔖️VolumeMesh

// #region 🔖️Quality
/// 📊️ Cheap mesh sanity report — interior angle bounds (2D) and inverted-cell detection (3D).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QualityReport {
    pub min_angle_deg: f64,
    pub max_angle_deg: f64,
    pub min_jacobian_sign_positive: bool,
    pub element_count: usize,
}

/// 📐️ Interior angle at `p`, between the edges to `prev` and `next`, in degrees.
fn angle_at(prev: [f64; 2], p: [f64; 2], next: [f64; 2]) -> f64 {
    let v1 = [prev[0] - p[0], prev[1] - p[1]];
    let v2 = [next[0] - p[0], next[1] - p[1]];
    let dot = v1[0] * v2[0] + v1[1] * v2[1];
    let n1 = (v1[0] * v1[0] + v1[1] * v1[1]).sqrt();
    let n2 = (v2[0] * v2[0] + v2[1] * v2[1]).sqrt();
    let cos_a = (dot / (n1 * n2)).clamp(-1.0, 1.0);
    cos_a.acos().to_degrees()
}

/// 📊️ Min/max interior angle across all triangles; `min_jacobian_sign_positive` mirrors the 2D
/// analogue of the 3D check — true iff every triangle's signed area (shoelace, `[n0,n1,n2]` order) is
/// positive, i.e. consistently wound.
pub fn tri_mesh_quality(mesh: &TriMesh2) -> QualityReport {
    let mut min_angle = f64::INFINITY;
    let mut max_angle = f64::NEG_INFINITY;
    let mut all_positive = true;
    for tri in &mesh.tris {
        let p = [mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]];
        for i in 0..3 {
            let a = angle_at(p[(i + 2) % 3], p[i], p[(i + 1) % 3]);
            min_angle = min_angle.min(a);
            max_angle = max_angle.max(a);
        }
        let signed_area = 0.5 * ((p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[2][0] - p[0][0]) * (p[1][1] - p[0][1]));
        if signed_area <= 0.0 {
            all_positive = false;
        }
    }
    if mesh.tris.is_empty() {
        min_angle = 0.0;
        max_angle = 0.0;
    }
    QualityReport { min_angle_deg: min_angle, max_angle_deg: max_angle, min_jacobian_sign_positive: all_positive, element_count: mesh.tris.len() }
}

/// 🧮️ Signed tet volume via the scalar triple product of edge vectors from `p0`.
fn tet_signed_volume(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> f64 {
    let a = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let b = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let c = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];
    let cross = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    (cross[0] * c[0] + cross[1] * c[1] + cross[2] * c[2]) / 6.0
}

/// 🧮️ Signed volume of one cell, via a FIXED (purely local-index-based, not global-min-based like
/// [`split_to_tets`]) tet decomposition — so the sign faithfully reflects whether the cell's own
/// documented local node order (`extrude_tri_mesh`/`extrude_quad_mesh`'s convention) is right-handed,
/// independent of the cell's global point indices. Verified by hand against a unit right prism and a
/// unit cube (both give the expected positive volume for correctly-ordered nodes).
fn cell_signed_volume(points: &[[f64; 3]], cell: &Cell) -> f64 {
    let p = |i: u32| points[i as usize];
    match cell {
        Cell::Tet4([a, b, c, d]) => tet_signed_volume(p(*a), p(*b), p(*c), p(*d)),
        Cell::Wedge6(n) => tet_signed_volume(p(n[0]), p(n[1]), p(n[2]), p(n[3])) + tet_signed_volume(p(n[1]), p(n[2]), p(n[3]), p(n[4])) + tet_signed_volume(p(n[2]), p(n[3]), p(n[4]), p(n[5])),
        Cell::Hex8(n) => {
            tet_signed_volume(p(n[0]), p(n[4]), p(n[5]), p(n[6]))
                + tet_signed_volume(p(n[0]), p(n[4]), p(n[6]), p(n[7]))
                + tet_signed_volume(p(n[0]), p(n[1]), p(n[2]), p(n[6]))
                + tet_signed_volume(p(n[0]), p(n[1]), p(n[6]), p(n[5]))
                + tet_signed_volume(p(n[0]), p(n[2]), p(n[3]), p(n[7]))
                + tet_signed_volume(p(n[0]), p(n[2]), p(n[7]), p(n[6]))
        }
    }
}

/// 📊️ `min_jacobian_sign_positive` is true iff every cell's signed volume is positive — a negative
/// signed volume flags inverted/degenerate connectivity. Angle bounds are a 2D-only concept and are
/// left at `0.0` here.
pub fn volume_mesh_quality(mesh: &VolumeMesh) -> QualityReport {
    let all_positive = mesh.cells.iter().all(|cell| cell_signed_volume(&mesh.points, cell) > 0.0);
    QualityReport { min_angle_deg: 0.0, max_angle_deg: 0.0, min_jacobian_sign_positive: all_positive, element_count: mesh.cells.len() }
}
// #endregion 🔖️Quality

#[path = "../📐️surface/🧬️schema/🦀️.rs"]
pub mod surface_schema;

// #region 🔖️Tests
#[cfg(test)]
#[path = "../📐️surface/🧪️tests/🦀️.rs"]
mod surface_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
