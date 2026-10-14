//! 🩹️ Mesh repair: cleaning, non-manifold repair, consistent orientation, hole filling, the closed voxel fallback and watertightness validation.

use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::DenseField;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::DisjointSet;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::EdgeMap;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::HalfedgeTopology;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::TriMesh;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::add3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::cross3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::dot3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::extract_dense_grid;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::lerp3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::norm3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::normalize3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::scale3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::sorted_edge;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::sub3;

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
    remap: Vec<u32>,
    positions: Vec<[f64; 3]>,
    triangles: Vec<[u32; 3]>,
    seen_triangles: BTreeSet<[u32; 3]>,
    minimum_component_faces: usize,
    minimum_component_bbox_fraction: f64,
    /// 🏝️ Keep only the component with the most faces. The interactive pipeline has no
    /// `close_voxel` re-voxelisation to merge islands with, and its watertight guarantee is one
    /// closed solid: everything a carved surface net emits besides the principal shell is a
    /// depth-noise bubble (an isolated negative voxel closes into an 8–12 face ball whose bounding
    /// box, at one voxel, is well above the 2 % fraction the size rule alone would drop).
    keep_largest_component: bool,
    /// Face-connectivity of the welded triangles, for the small-component pass.
    components: Option<DisjointSet>,
    /// First face seen on each undirected edge, so the second one unions with it.
    edge_owner: BTreeMap<(u32, u32), u32>,
    /// Per component root: face count and bounding box.
    component_extents: BTreeMap<u32, (usize, [f64; 3], [f64; 3])>,
    kept_triangles: Vec<[u32; 3]>,
    kept_positions: Vec<[f64; 3]>,
    kept_vertex_of: BTreeMap<u32, u32>,
}

impl BoundedCleanPreparation {
    fn new(mesh: &TriMesh, minimum_component_faces: usize, minimum_component_bbox_fraction: f64, keep_largest_component: bool) -> Self {
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

    fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {
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

    fn finish(self) -> TriMesh {
        TriMesh { positions: self.kept_positions, triangles: self.kept_triangles }
    }
}

#[derive(Clone, semio_framework_value::RetireOwned)]
struct BoundedEdgeRecord {
    directed: [u32; 2],
    faces: Vec<u32>,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct BoundedEdgePreparation {
    cursor: usize,
    records: BTreeMap<(u32, u32), BoundedEdgeRecord>,
}

impl BoundedEdgePreparation {
    fn new() -> Self {
        Self { cursor: 0, records: BTreeMap::new() }
    }

    fn advance(&mut self, mesh: &TriMesh, triangle_budget: usize) -> bool {
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
    edges: BoundedEdgePreparation,
    /// 🧩️ Face connectivity across manifold edges.
    components: Option<DisjointSet>,
    /// 🧭️ Union cursor: the edge key the next unit resumes after.
    union_cursor: Option<(u32, u32)>,
    /// 🧭️ Offenders (edges with more than two faces) in key order, and the next one to split.
    offenders: Vec<(u32, u32)>,
    offender_cursor: usize,
    /// 🩹️ Vertex copies appended and the rewritten triangles, applied at `finish`.
    added_positions: Vec<[f64; 3]>,
    triangles: Vec<[u32; 3]>,
}

impl BoundedRepairPreparation {
    fn new(mesh: &TriMesh) -> Self {
        Self { phase: BoundedRepairPhase::Edges, edges: BoundedEdgePreparation::new(), components: None, union_cursor: None, offenders: Vec::new(), offender_cursor: 0, added_positions: Vec::new(), triangles: mesh.triangles.clone() }
    }

    fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {
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

    fn finish(self, mesh: &mut TriMesh) {
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
    edges: BoundedEdgePreparation,
    visited: Vec<bool>,
    seed_cursor: usize,
    queue: VecDeque<u32>,
}

impl BoundedOrientPreparation {
    fn new() -> Self {
        Self { phase: BoundedOrientPhase::Edges, edges: BoundedEdgePreparation::new(), visited: Vec::new(), seed_cursor: 0, queue: VecDeque::new() }
    }

    fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {
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
    edges: BoundedEdgePreparation,
    boundary_key: Option<(u32, u32)>,
    remaining: BTreeSet<(u32, u32)>,
    current_loop: Vec<u32>,
    start: Option<u32>,
    current: Option<u32>,
    /// 🔁️ Closed boundary loops waiting to be filled: a walk that revisits a vertex (two holes
    /// pinching at it) splits off the loop it just closed, so no loop repeats a vertex.
    pending: Vec<Vec<u32>>,
    /// 🩹️ Undirected edges the fill added, so no two fill triangles and no fill triangle and mesh
    /// face share an edge more than a manifold allows.
    added: BTreeSet<(u32, u32)>,
    max_boundary_verts: usize,
    exceeded: bool,
}

impl BoundedHoleFillPreparation {
    fn new(max_boundary_verts: usize) -> Self {
        Self { phase: BoundedHoleFillPhase::Edges, edges: BoundedEdgePreparation::new(), boundary_key: None, remaining: BTreeSet::new(), current_loop: Vec::new(), start: None, current: None, pending: Vec::new(), added: BTreeSet::new(), max_boundary_verts, exceeded: false }
    }

    fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {
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

    fn exceeded(&self) -> bool {
        self.exceeded
    }
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
    // 🎲️ Sorted, not set order: the conjugate-gradient rounding depends on the unknown and
    // neighbour order, and a mesh must not differ between two runs of the same input.
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
