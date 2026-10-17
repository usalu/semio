//! 🧊️ Voxel closing and validation of the mesh repair: the by-construction closed fallback surface and the watertightness report with its self-intersection and manifold checks. See [`super`].

use super::*;

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
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
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
pub fn triangles_intersect(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> bool {
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
    let tree = semio_s_artifact_remodel_remodeling_spatial::standards::v1::subsets::any::schema::spatial_internals::KdTree::<3>::build(&centroids);
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
/// 2-manifold (no over-used edges or pinch vertices), and consistently oriented. `🐨️genus` is only
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
