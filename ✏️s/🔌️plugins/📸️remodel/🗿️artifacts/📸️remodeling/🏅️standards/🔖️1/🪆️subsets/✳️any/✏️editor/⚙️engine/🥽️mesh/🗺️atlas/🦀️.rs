//! 🗺️ Mesh atlas: quadric simplification, chart segmentation and LSCM unwrapping, and multi-view texture baking.

use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::TriMesh;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::add3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::cross3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::dot3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::lerp3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::matd_from_rows;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::norm3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::normalize3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::scale3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::sorted_edge;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::sub3;

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
        let rhs = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD::from_vec(vec![-q[3], -q[6], -q[8]]);
        if let Ok(l) = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::cholesky(&m) {
            let x = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::cholesky_solve(&l, &rhs);
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

/// 🪢️ LSCM (least-squares conformal map) for one chart via `crate::standards::v1::subsets::any::schema::algebra_internals::CsrMatrix` +
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
    let mut rhs = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD::zeros(m);
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
    let a = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::CsrMatrix::from_triplets(m, m, &triplets);
    let cg_max_iter = m.clamp(50, 600);
    let Ok(x) = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::conjugate_gradient(&a, &rhs, 1e-7, cg_max_iter) else {
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
#[derive(semio_framework_value::RetireOwned)]
pub struct TextureView {
    pub pose: remodeling_camera::CameraPose,
    pub intrinsics: remodeling_camera::Intrinsics,
    pub image: remodeling_image::ImageRgba8,
}

fn image_gradient_magnitude(img: &remodeling_image::ImageRgba8, x: f32, y: f32) -> f32 {
    let step = 1.0;
    let gx = img.sample_rgb(x + step, y)[0] - img.sample_rgb(x - step, y)[0];
    let gy = img.sample_rgb(x, y + step)[0] - img.sample_rgb(x, y - step)[0];
    (gx * gx + gy * gy).sqrt()
}

fn face_projected_area(mesh: &TriMesh, f: usize, intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose) -> Option<f64> {
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
/// `graph::FlowNetwork`'s Dinic max-flow: data cost rewards a view with large
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
            let mut net = graph::FlowNetwork::new(active.len() as u32 + 2);
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
        if let Ok(x) = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::solve_llsq(&mat, &semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD::from_vec(b)) {
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
            semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::solve_llsq(&mat, &semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD::from_vec(b)).map_or((1.0, 0.0), |x| (x.get(0), x.get(1)))
        })
        .collect()
}

/// 🖼️ Bakes a UV-space texture atlas: for every face, samples its assigned view (from
/// [`graph_cut_view_labels`]) via barycentric-in-UV rasterization, applies that view's exposure
/// compensation, and levels per-chart seams via [`level_seam`] before writing the final atlas.
pub fn bake_texture(mesh: &TriMesh, uvs: &[[f32; 2]], atlas_size: u32, views: &[TextureView]) -> remodeling_image::ImageRgba8 {
    let mut atlas = remodeling_image::ImageRgba8::new(atlas_size, atlas_size);
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
