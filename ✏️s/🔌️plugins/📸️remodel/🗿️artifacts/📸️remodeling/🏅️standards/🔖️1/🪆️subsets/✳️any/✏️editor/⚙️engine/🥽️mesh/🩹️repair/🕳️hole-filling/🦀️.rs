//! 🕳️ Hole filling of the mesh repair: the boundary loop triangulation by ear clipping, minimum weight and advancing front, with fairing and patch validation. See [`super`].

use super::*;

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
    let mut free: Vec<u32> = new_vertices.iter().copied().collect();
    free.sort_unstable();
    let index_of: HashMap<u32, usize> = free.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let n = free.len();
    let mut triplets = Vec::new();
    let mut rhs = [semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD::zeros(n), semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD::zeros(n), semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD::zeros(n)];
    for (row, &v) in free.iter().enumerate() {
        let mut nbrs: Vec<u32> = neighbors.get(&v).map(|set| set.iter().copied().collect()).unwrap_or_default();
        nbrs.sort_unstable();
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
    let a = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::CsrMatrix::from_triplets(n, n, &triplets);
    for (axis, rhs_axis) in rhs.iter().enumerate() {
        if let Ok(x) = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::conjugate_gradient(&a, rhs_axis, 1e-8, 500) {
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
