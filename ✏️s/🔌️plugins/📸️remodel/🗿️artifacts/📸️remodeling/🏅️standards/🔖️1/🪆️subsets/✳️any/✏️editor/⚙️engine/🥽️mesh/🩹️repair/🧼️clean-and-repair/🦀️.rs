//! 🧼️ Cleaning and repair of the mesh: the vertex welding, degenerate and small component removal, Taubin smoothing, non-manifold repair and consistent outward orientation. See [`super`].

use super::*;

// #region 🔖️Clean
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
