//! ⏱️ Bounded preparations of the mesh repair: the resumable cleaning, non-manifold edge repair, orientation and hole filling preparations with their phases and statistics. See [`super`].

use super::*;

// #region 🔖️Clean
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CleanStats {
    pub vertices_welded: usize,
    pub degenerate_triangles_removed: usize,
    pub zero_length_edges_collapsed: usize,
    pub small_components_removed: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedCleanPhase {
    Vertices,
    Triangles,
    /// 🧩️ Union the welded triangles that share a manifold edge, one triangle per unit.
    ComponentUnion,
    /// 🧩️ Accumulate each component's face count and bounding box, one triangle per unit.
    ComponentMeasure,
    /// 🧩️ Drop the components that are both too small and too tight, one triangle per unit.
    ComponentFilter,
    /// 🧩️ Renumber the surviving triangles onto the vertices they still use, one triangle per unit.
    Compact,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct BoundedCleanPreparation {
    phase: BoundedCleanPhase,
    cursor: usize,
    vertex_of: BTreeMap<[u64; 3], u32>,
    pub(super) remap: Vec<u32>,
    pub(super) positions: Vec<[f64; 3]>,
    pub(super) triangles: Vec<[u32; 3]>,
    seen_triangles: BTreeSet<[u32; 3]>,
    minimum_component_faces: usize,
    minimum_component_bbox_fraction: f64,
    /// 🏝️ Keep only the component with the most faces. The interactive pipeline has no
    /// `close_voxel` re-voxelisation to merge islands with, and its watertight guarantee is one
    /// closed solid: everything a carved surface net emits besides the principal shell is a
    /// depth-noise bubble (an isolated negative voxel closes into an 8–12 face ball whose bounding
    /// box, at one voxel, is well above the 2 % fraction the size rule alone would drop).
    keep_largest_component: bool,
    /// 🔌️ Face-connectivity of the welded triangles, for the small-component pass.
    components: Option<DisjointSet>,
    /// 🔗️ First face seen on each undirected edge, so the second one unions with it.
    edge_owner: BTreeMap<(u32, u32), u32>,
    /// 🔩️ Per component root: face count and bounding box.
    component_extents: BTreeMap<u32, (usize, [f64; 3], [f64; 3])>,
    kept_triangles: Vec<[u32; 3]>,
    kept_positions: Vec<[f64; 3]>,
    kept_vertex_of: BTreeMap<u32, u32>,
}

impl BoundedCleanPreparation {
    pub fn new(mesh: &TriMesh, minimum_component_faces: usize, minimum_component_bbox_fraction: f64, keep_largest_component: bool) -> Self {
        Self {
            phase: BoundedCleanPhase::Vertices,
            cursor: 0,
            vertex_of: BTreeMap::new(),
            remap: Vec::with_capacity(mesh.positions.len()),
            positions: Vec::with_capacity(mesh.positions.len()),
            triangles: Vec::with_capacity(mesh.triangles.len()),
            seen_triangles: BTreeSet::new(),
            minimum_component_faces,
            minimum_component_bbox_fraction,
            keep_largest_component,
            components: None,
            edge_owner: BTreeMap::new(),
            component_extents: BTreeMap::new(),
            kept_triangles: Vec::new(),
            kept_positions: Vec::new(),
            kept_vertex_of: BTreeMap::new(),
        }
    }

    /// 📏️ Bounding-box diagonal of the welded working mesh, the scale the component test is
    /// relative to.
    fn working_diagonal(&self) -> f64 {
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];
        for position in &self.positions {
            for axis in 0..3 {
                lo[axis] = lo[axis].min(position[axis]);
                hi[axis] = hi[axis].max(position[axis]);
            }
        }
        if self.positions.is_empty() {
            return 1e-12;
        }
        norm3(sub3(hi, lo)).max(1e-12)
    }

    pub fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {
        let budget = item_budget.max(1);
        match self.phase {
            BoundedCleanPhase::Vertices => {
                let end = self.cursor.saturating_add(budget).min(mesh.positions.len());
                for &position in &mesh.positions[self.cursor..end] {
                    let key = position.map(f64::to_bits);
                    let vertex = if let Some(&vertex) = self.vertex_of.get(&key) {
                        vertex
                    } else {
                        let vertex = self.positions.len() as u32;
                        self.positions.push(position);
                        self.vertex_of.insert(key, vertex);
                        vertex
                    };
                    self.remap.push(vertex);
                }
                self.cursor = end;
                if end == mesh.positions.len() {
                    self.cursor = 0;
                    self.phase = BoundedCleanPhase::Triangles;
                }
            }
            BoundedCleanPhase::Triangles => {
                let end = self.cursor.saturating_add(budget).min(mesh.triangles.len());
                for triangle in &mesh.triangles[self.cursor..end] {
                    let remapped = triangle.map(|vertex| self.remap[vertex as usize]);
                    if remapped[0] == remapped[1] || remapped[1] == remapped[2] || remapped[2] == remapped[0] {
                        continue;
                    }
                    let a = self.positions[remapped[0] as usize];
                    let b = self.positions[remapped[1] as usize];
                    let c = self.positions[remapped[2] as usize];
                    if norm3(cross3(sub3(b, a), sub3(c, a))) <= 1e-15 {
                        continue;
                    }
                    let mut canonical = remapped;
                    canonical.sort_unstable();
                    if self.seen_triangles.insert(canonical) {
                        self.triangles.push(remapped);
                    }
                }
                self.cursor = end;
                if end == mesh.triangles.len() {
                    self.cursor = 0;
                    self.components = Some(DisjointSet::new(self.triangles.len()));
                    self.phase = BoundedCleanPhase::ComponentUnion;
                }
            }
            BoundedCleanPhase::ComponentUnion => {
                let end = self.cursor.saturating_add(budget).min(self.triangles.len());
                for face in self.cursor..end {
                    let triangle = self.triangles[face];
                    for corner in 0..3 {
                        let (a, b) = (triangle[corner], triangle[(corner + 1) % 3]);
                        let edge = (a.min(b), a.max(b));
                        match self.edge_owner.get(&edge).copied() {
                            Some(owner) => {
                                if let Some(components) = self.components.as_mut() {
                                    components.union(owner, face as u32);
                                }
                            }
                            None => {
                                self.edge_owner.insert(edge, face as u32);
                            }
                        }
                    }
                }
                self.cursor = end;
                if end == self.triangles.len() {
                    self.cursor = 0;
                    self.edge_owner = BTreeMap::new();
                    self.phase = BoundedCleanPhase::ComponentMeasure;
                }
            }
            BoundedCleanPhase::ComponentMeasure => {
                let end = self.cursor.saturating_add(budget).min(self.triangles.len());
                for face in self.cursor..end {
                    let triangle = self.triangles[face];
                    let Some(root) = self.components.as_mut().map(|components| components.find(face as u32)) else { continue };
                    let entry = self.component_extents.entry(root).or_insert((0, [f64::INFINITY; 3], [f64::NEG_INFINITY; 3]));
                    entry.0 += 1;
                    for vertex in triangle {
                        let position = self.positions[vertex as usize];
                        for axis in 0..3 {
                            entry.1[axis] = entry.1[axis].min(position[axis]);
                            entry.2[axis] = entry.2[axis].max(position[axis]);
                        }
                    }
                }
                self.cursor = end;
                if end == self.triangles.len() {
                    self.cursor = 0;
                    self.phase = BoundedCleanPhase::ComponentFilter;
                }
            }
            BoundedCleanPhase::ComponentFilter => {
                let diagonal = self.working_diagonal();
                let principal = self.keep_largest_component.then(|| self.component_extents.iter().max_by_key(|(root, (faces, _, _))| (*faces, std::cmp::Reverse(**root))).map(|(root, _)| *root)).flatten();
                let end = self.cursor.saturating_add(budget).min(self.triangles.len());
                for face in self.cursor..end {
                    let triangle = self.triangles[face];
                    let Some(root) = self.components.as_mut().map(|components| components.find(face as u32)) else { continue };
                    let Some(&(faces, lo, hi)) = self.component_extents.get(&root) else { continue };
                    if principal.is_some_and(|principal| principal != root) {
                        continue;
                    }
                    // 🧩️ Same rule as the batch `remove_small_components`: a component only drops
                    // when it is BOTH under the face floor and tighter than the bbox fraction.
                    if faces < self.minimum_component_faces && norm3(sub3(hi, lo)) / diagonal < self.minimum_component_bbox_fraction {
                        continue;
                    }
                    self.kept_triangles.push(triangle);
                }
                self.cursor = end;
                if end == self.triangles.len() {
                    self.cursor = 0;
                    self.components = None;
                    self.component_extents = BTreeMap::new();
                    self.phase = BoundedCleanPhase::Compact;
                }
            }
            BoundedCleanPhase::Compact => {
                let end = self.cursor.saturating_add(budget).min(self.kept_triangles.len());
                for face in self.cursor..end {
                    let triangle = self.kept_triangles[face];
                    self.kept_triangles[face] = triangle.map(|vertex| match self.kept_vertex_of.get(&vertex).copied() {
                        Some(renumbered) => renumbered,
                        None => {
                            let renumbered = self.kept_positions.len() as u32;
                            self.kept_positions.push(self.positions[vertex as usize]);
                            self.kept_vertex_of.insert(vertex, renumbered);
                            renumbered
                        }
                    });
                }
                self.cursor = end;
                if end == self.kept_triangles.len() {
                    self.phase = BoundedCleanPhase::Done;
                }
            }
            BoundedCleanPhase::Done => {}
        }
        self.phase == BoundedCleanPhase::Done
    }

    pub fn finish(self) -> TriMesh {
        TriMesh { positions: self.kept_positions, triangles: self.kept_triangles }
    }
}

#[derive(Clone, semio_framework_value::RetireOwned)]
pub struct BoundedEdgeRecord {
    pub directed: [u32; 2],
    pub faces: Vec<u32>,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct BoundedEdgePreparation {
    cursor: usize,
    pub records: BTreeMap<(u32, u32), BoundedEdgeRecord>,
}

impl BoundedEdgePreparation {
    pub fn new() -> Self {
        Self { cursor: 0, records: BTreeMap::new() }
    }

    pub fn advance(&mut self, mesh: &TriMesh, triangle_budget: usize) -> bool {
        let end = self.cursor.saturating_add(triangle_budget.max(1)).min(mesh.triangles.len());
        for face in self.cursor..end {
            let triangle = mesh.triangles[face];
            for edge in 0..3 {
                let directed = [triangle[edge], triangle[(edge + 1) % 3]];
                self.records.entry(sorted_edge(directed[0], directed[1])).or_insert_with(|| BoundedEdgeRecord { directed, faces: Vec::with_capacity(2) }).faces.push(face as u32);
            }
        }
        self.cursor = end;
        end == mesh.triangles.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedRepairPhase {
    Edges,
    /// 🧩️ Union the faces across every 2-face edge, one edge record per unit.
    Union,
    /// 🩹️ Split every edge with more than two faces, one offender per unit.
    Split,
    Done,
}

/// 🩹️ Resumable counterpart of [`repair_non_manifold`]'s edge pass: the faces around a
/// non-manifold edge are grouped by their manifold connectivity (a union-find across the 2-face
/// edges, [`group_faces_by_manifold_connectivity`]'s rule) and every group but the first gets its
/// own copy of the edge's two vertices. Dropping the surplus faces instead — what this preparation
/// used to do — only moved the defect: the hole fill that followed closed the two holes it left and
/// put a face back on the same edge, so a net with one such edge failed validation with exactly
/// one non-manifold edge, every time.
#[derive(semio_framework_value::RetireOwned)]
pub struct BoundedRepairPreparation {
    phase: BoundedRepairPhase,
    pub(super) edges: BoundedEdgePreparation,
    /// 🧩️ Face connectivity across manifold edges.
    components: Option<DisjointSet>,
    /// 🧭️ Union cursor: the edge key the next unit resumes after.
    union_cursor: Option<(u32, u32)>,
    /// 🧭️ Offenders (edges with more than two faces) in key order, and the next one to split.
    pub(super) offenders: Vec<(u32, u32)>,
    offender_cursor: usize,
    /// 🩹️ Vertex copies appended and the rewritten triangles, applied at `finish`.
    added_positions: Vec<[f64; 3]>,
    pub(super) triangles: Vec<[u32; 3]>,
}

impl BoundedRepairPreparation {
    pub fn new(mesh: &TriMesh) -> Self {
        Self { phase: BoundedRepairPhase::Edges, edges: BoundedEdgePreparation::new(), components: None, union_cursor: None, offenders: Vec::new(), offender_cursor: 0, added_positions: Vec::new(), triangles: mesh.triangles.clone() }
    }

    pub fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {
        match self.phase {
            BoundedRepairPhase::Edges => {
                if self.edges.advance(mesh, item_budget) {
                    self.components = Some(DisjointSet::new(mesh.triangles.len()));
                    self.offenders = self.edges.records.iter().filter(|(_, record)| record.faces.len() > 2).map(|(key, _)| *key).collect();
                    self.phase = BoundedRepairPhase::Union;
                }
            }
            BoundedRepairPhase::Union => {
                let components = self.components.as_mut().expect("repair components");
                let mut visited = 0usize;
                let range = match self.union_cursor {
                    Some(key) => self.edges.records.range((std::ops::Bound::Excluded(key), std::ops::Bound::Unbounded)),
                    None => self.edges.records.range(..),
                };
                let mut last = None;
                for (key, record) in range {
                    if record.faces.len() == 2 {
                        components.union(record.faces[0], record.faces[1]);
                    }
                    last = Some(*key);
                    visited += 1;
                    if visited >= item_budget.max(1) {
                        break;
                    }
                }
                match last {
                    Some(key) => self.union_cursor = Some(key),
                    None => self.phase = if self.offenders.is_empty() { BoundedRepairPhase::Done } else { BoundedRepairPhase::Split },
                }
                if visited < item_budget.max(1) {
                    self.phase = if self.offenders.is_empty() { BoundedRepairPhase::Done } else { BoundedRepairPhase::Split };
                }
            }
            BoundedRepairPhase::Split => {
                let components = self.components.as_mut().expect("repair components");
                for _ in 0..item_budget.max(1) {
                    let Some(&(a, b)) = self.offenders.get(self.offender_cursor) else {
                        self.phase = BoundedRepairPhase::Done;
                        break;
                    };
                    self.offender_cursor += 1;
                    let Some(record) = self.edges.records.get(&(a, b)) else { continue };
                    let mut roots: Vec<u32> = record.faces.iter().map(|&face| components.find(face)).collect();
                    roots.sort_unstable();
                    roots.dedup();
                    for root in roots.into_iter().skip(1) {
                        let new_a = (mesh.positions.len() + self.added_positions.len()) as u32;
                        self.added_positions.push(mesh.positions[a as usize]);
                        let new_b = (mesh.positions.len() + self.added_positions.len()) as u32;
                        self.added_positions.push(mesh.positions[b as usize]);
                        // Every face of that fan group touching `a` or `b` moves onto the copies,
                        // not only the faces on the offending edge (the same rule as the batch pass).
                        for face in 0..self.triangles.len() {
                            if components.find(face as u32) != root {
                                continue;
                            }
                            let mut triangle = self.triangles[face];
                            let mut touched = false;
                            for vertex in triangle.iter_mut() {
                                if *vertex == a {
                                    *vertex = new_a;
                                    touched = true;
                                } else if *vertex == b {
                                    *vertex = new_b;
                                    touched = true;
                                }
                            }
                            if touched {
                                self.triangles[face] = triangle;
                            }
                        }
                    }
                }
            }
            BoundedRepairPhase::Done => {}
        }
        self.phase == BoundedRepairPhase::Done
    }

    pub fn finish(self, mesh: &mut TriMesh) {
        mesh.positions.extend(self.added_positions);
        mesh.triangles = self.triangles;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedOrientPhase {
    Edges,
    AllocateVisited,
    Flood,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct BoundedOrientPreparation {
    phase: BoundedOrientPhase,
    pub(super) edges: BoundedEdgePreparation,
    pub(super) visited: Vec<bool>,
    seed_cursor: usize,
    pub(super) queue: VecDeque<u32>,
}

impl BoundedOrientPreparation {
    pub fn new() -> Self {
        Self { phase: BoundedOrientPhase::Edges, edges: BoundedEdgePreparation::new(), visited: Vec::new(), seed_cursor: 0, queue: VecDeque::new() }
    }

    pub fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {
        let budget = item_budget.max(1);
        match self.phase {
            BoundedOrientPhase::Edges => {
                if self.edges.advance(mesh, budget) {
                    self.phase = BoundedOrientPhase::AllocateVisited;
                }
            }
            BoundedOrientPhase::AllocateVisited => {
                let end = self.visited.len().saturating_add(budget).min(mesh.triangles.len());
                self.visited.resize(end, false);
                if end == mesh.triangles.len() {
                    self.phase = BoundedOrientPhase::Flood;
                }
            }
            BoundedOrientPhase::Flood => {
                for _ in 0..budget {
                    if self.queue.is_empty() {
                        while self.seed_cursor < self.visited.len() && self.visited[self.seed_cursor] {
                            self.seed_cursor += 1;
                        }
                        if self.seed_cursor == self.visited.len() {
                            self.phase = BoundedOrientPhase::Done;
                            break;
                        }
                        self.visited[self.seed_cursor] = true;
                        self.queue.push_back(self.seed_cursor as u32);
                        self.seed_cursor += 1;
                    }
                    let Some(face) = self.queue.pop_front() else { continue };
                    let triangle = mesh.triangles[face as usize];
                    for edge in 0..3 {
                        let a = triangle[edge];
                        let b = triangle[(edge + 1) % 3];
                        let Some(record) = self.edges.records.get(&sorted_edge(a, b)) else { continue };
                        if record.faces.len() != 2 {
                            continue;
                        }
                        let other = if record.faces[0] == face { record.faces[1] } else { record.faces[0] };
                        if self.visited[other as usize] {
                            continue;
                        }
                        self.visited[other as usize] = true;
                        let other_triangle = mesh.triangles[other as usize];
                        if (0..3).any(|slot| other_triangle[slot] == a && other_triangle[(slot + 1) % 3] == b) {
                            mesh.triangles[other as usize].swap(1, 2);
                        }
                        self.queue.push_back(other);
                    }
                }
            }
            BoundedOrientPhase::Done => {}
        }
        self.phase == BoundedOrientPhase::Done
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedHoleFillPhase {
    Edges,
    Boundary,
    Walk,
    Fill,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct BoundedHoleFillPreparation {
    phase: BoundedHoleFillPhase,
    pub(super) edges: BoundedEdgePreparation,
    boundary_key: Option<(u32, u32)>,
    remaining: BTreeSet<(u32, u32)>,
    current_loop: Vec<u32>,
    pub(super) start: Option<u32>,
    current: Option<u32>,
    /// 🔁️ Closed boundary loops waiting to be filled: a walk that revisits a vertex (two holes
    /// pinching at it) splits off the loop it just closed, so no loop repeats a vertex.
    pending: Vec<Vec<u32>>,
    /// 🩹️ Undirected edges the fill added, so no two fill triangles and no fill triangle and mesh
    /// face share an edge more than a manifold allows.
    added: BTreeSet<(u32, u32)>,
    pub(super) max_boundary_verts: usize,
    exceeded: bool,
}

impl BoundedHoleFillPreparation {
    pub fn new(max_boundary_verts: usize) -> Self {
        Self { phase: BoundedHoleFillPhase::Edges, edges: BoundedEdgePreparation::new(), boundary_key: None, remaining: BTreeSet::new(), current_loop: Vec::new(), start: None, current: None, pending: Vec::new(), added: BTreeSet::new(), max_boundary_verts, exceeded: false }
    }

    pub fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {
        for _ in 0..item_budget.max(1) {
            match self.phase {
                BoundedHoleFillPhase::Edges => {
                    if self.edges.advance(mesh, 1) {
                        self.boundary_key = self.edges.records.keys().next().copied();
                        self.phase = if self.boundary_key.is_some() { BoundedHoleFillPhase::Boundary } else { BoundedHoleFillPhase::Done };
                    }
                }
                BoundedHoleFillPhase::Boundary => {
                    let Some(key) = self.boundary_key else {
                        self.phase = BoundedHoleFillPhase::Walk;
                        continue;
                    };
                    let record = self.edges.records.get(&key).expect("bounded boundary edge");
                    if record.faces.len() == 1 {
                        self.remaining.insert((record.directed[0], record.directed[1]));
                    }
                    self.boundary_key = self.edges.records.range((std::ops::Bound::Excluded(key), std::ops::Bound::Unbounded)).next().map(|(next, _)| *next);
                }
                BoundedHoleFillPhase::Walk => {
                    if self.current.is_none() {
                        let Some((from, to)) = self.remaining.pop_first() else {
                            self.phase = if self.pending.is_empty() { BoundedHoleFillPhase::Done } else { BoundedHoleFillPhase::Fill };
                            continue;
                        };
                        self.current_loop = vec![from, to];
                        self.start = Some(from);
                        self.current = Some(to);
                        continue;
                    }
                    let current = self.current.expect("active boundary walk");
                    let next = self.remaining.range((std::ops::Bound::Included((current, 0)), std::ops::Bound::Included((current, u32::MAX)))).next().copied();
                    let Some(edge) = next else {
                        self.current = None;
                        self.current_loop.clear();
                        continue;
                    };
                    self.remaining.remove(&edge);
                    let next = edge.1;
                    if Some(next) == self.start {
                        self.current = None;
                        let closed = std::mem::take(&mut self.current_loop);
                        if closed.len() >= 3 && closed.len() <= self.max_boundary_verts {
                            self.pending.push(closed);
                            self.phase = BoundedHoleFillPhase::Fill;
                        }
                    } else if let Some(position) = self.current_loop.iter().position(|&vertex| vertex == next) {
                        // 📌️ A pinch: the walk came back to a vertex it already passed, closing the
                        // loop `next … current`; it is filled on its own and the walk continues.
                        let closed = self.current_loop.split_off(position + 1);
                        let mut closed_loop = vec![next];
                        closed_loop.extend(closed);
                        if closed_loop.len() >= 3 {
                            self.pending.push(closed_loop);
                        }
                        self.current = Some(next);
                    } else if self.current_loop.len() < self.max_boundary_verts {
                        self.current_loop.push(next);
                        self.current = Some(next);
                    } else {
                        self.current = None;
                        self.current_loop.clear();
                    }
                }
                BoundedHoleFillPhase::Fill => {
                    // ✂️ One ear per unit: the ear whose closing diagonal is shortest among those
                    // that add no edge the mesh (or an earlier fill triangle) already has — a fan
                    // from one vertex re-used existing edges and left non-manifold caps. Faces run
                    // against the loop's direction, which is the boundary faces' own winding.
                    let Some(polygon) = self.pending.last_mut() else {
                        self.phase = if self.current.is_none() && self.remaining.is_empty() { BoundedHoleFillPhase::Done } else { BoundedHoleFillPhase::Walk };
                        continue;
                    };
                    if mesh.triangles.len() >= 512 {
                        self.exceeded = true;
                        self.phase = BoundedHoleFillPhase::Done;
                        continue;
                    }
                    let n = polygon.len();
                    if n == 3 {
                        mesh.triangles.push([polygon[2], polygon[1], polygon[0]]);
                        self.pending.pop();
                        continue;
                    }
                    let taken = |a: u32, b: u32, records: &BTreeMap<(u32, u32), BoundedEdgeRecord>, added: &BTreeSet<(u32, u32)>| records.contains_key(&sorted_edge(a, b)) || added.contains(&sorted_edge(a, b));
                    let mut best: Option<(f64, usize)> = None;
                    for i in 0..n {
                        let (previous, next) = (polygon[(i + n - 1) % n], polygon[(i + 1) % n]);
                        if taken(previous, next, &self.edges.records, &self.added) {
                            continue;
                        }
                        let length = norm3(sub3(mesh.positions[previous as usize], mesh.positions[next as usize]));
                        if best.is_none_or(|(shortest, _)| length < shortest) {
                            best = Some((length, i));
                        }
                    }
                    let Some((_, ear)) = best else {
                        // No ear closes without duplicating an edge: leave this hole open rather
                        // than make the mesh non-manifold; validation reports it.
                        self.pending.pop();
                        continue;
                    };
                    let (previous, vertex, next) = (polygon[(ear + n - 1) % n], polygon[ear], polygon[(ear + 1) % n]);
                    mesh.triangles.push([next, vertex, previous]);
                    self.added.insert(sorted_edge(previous, next));
                    polygon.remove(ear);
                }
                BoundedHoleFillPhase::Done => return true,
            }
        }
        self.phase == BoundedHoleFillPhase::Done
    }

    pub fn exceeded(&self) -> bool {
        self.exceeded
    }
}
// #endregion 🔖️Clean
