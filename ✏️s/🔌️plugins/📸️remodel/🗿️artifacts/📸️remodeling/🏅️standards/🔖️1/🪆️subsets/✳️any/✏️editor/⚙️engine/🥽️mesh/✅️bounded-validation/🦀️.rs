//! ✅️ Bounded validation of the mesh pipeline: the resumable watertightness validation preparation with its phases. See [`super`].

use super::*;

// #region 🔖️BoundedValidation
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedValidationPhase {
    Edges,
    AllocateFaces,
    AllocateVertices,
    AllocateIncident,
    Triangles,
    ScanEdges,
    ScanVertices,
    Components,
    BoundaryLoops,
    Intersections,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct BoundedValidationPreparation {
    pub(super) phase: BoundedValidationPhase,
    pub(super) edges: BoundedEdgePreparation,
    face_parent: Vec<u32>,
    vertex_parent: Vec<u32>,
    incident: Vec<Vec<u32>>,
    pub(super) cursor: usize,
    edge_key: Option<(u32, u32)>,
    incident_face_cursor: usize,
    incident_root: Option<u32>,
    incident_conflict: bool,
    boundary_vertex: Option<u32>,
    pair: (usize, usize),
    used_vertices: BTreeSet<u32>,
    component_roots: BTreeSet<u32>,
    boundary_roots: BTreeSet<u32>,
    boundary_vertices: BTreeSet<u32>,
    pub(super) boundary_edge_count: usize,
    pub(super) non_manifold_edge_count: usize,
    pub(super) non_manifold_vertex_count: usize,
    pub(super) consistently_oriented: bool,
    pub(super) signed_volume: f64,
    self_intersections: usize,
    check_self_intersections: bool,
}

impl BoundedValidationPreparation {
    pub(super) fn new(check_self_intersections: bool) -> Self {
        Self {
            phase: BoundedValidationPhase::Edges,
            edges: BoundedEdgePreparation::new(),
            face_parent: Vec::new(),
            vertex_parent: Vec::new(),
            incident: Vec::new(),
            cursor: 0,
            edge_key: None,
            incident_face_cursor: 0,
            incident_root: None,
            incident_conflict: false,
            boundary_vertex: None,
            pair: (0, 1),
            used_vertices: BTreeSet::new(),
            component_roots: BTreeSet::new(),
            boundary_roots: BTreeSet::new(),
            boundary_vertices: BTreeSet::new(),
            boundary_edge_count: 0,
            non_manifold_edge_count: 0,
            non_manifold_vertex_count: 0,
            consistently_oriented: true,
            signed_volume: 0.0,
            self_intersections: 0,
            check_self_intersections,
        }
    }

    fn root(parent: &mut [u32], value: u32) -> u32 {
        let mut root = value;
        while parent[root as usize] != root {
            root = parent[root as usize];
        }
        let mut cursor = value;
        while parent[cursor as usize] != cursor {
            let next = parent[cursor as usize];
            parent[cursor as usize] = root;
            cursor = next;
        }
        root
    }

    fn union(parent: &mut [u32], a: u32, b: u32) {
        let a = Self::root(parent, a);
        let b = Self::root(parent, b);
        if a != b {
            parent[b as usize] = a;
        }
    }

    pub(super) fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {
        for _ in 0..item_budget.max(1) {
            match self.phase {
                BoundedValidationPhase::Edges => {
                    if self.edges.advance(mesh, 1) {
                        self.phase = BoundedValidationPhase::AllocateFaces;
                    }
                }
                BoundedValidationPhase::AllocateFaces => {
                    if self.face_parent.len() < mesh.triangles.len() {
                        self.face_parent.push(self.face_parent.len() as u32);
                    } else {
                        self.phase = BoundedValidationPhase::AllocateVertices;
                    }
                }
                BoundedValidationPhase::AllocateVertices => {
                    if self.vertex_parent.len() < mesh.positions.len() {
                        self.vertex_parent.push(self.vertex_parent.len() as u32);
                    } else {
                        self.phase = BoundedValidationPhase::AllocateIncident;
                    }
                }
                BoundedValidationPhase::AllocateIncident => {
                    if self.incident.len() < mesh.positions.len() {
                        self.incident.push(Vec::new());
                    } else {
                        self.cursor = 0;
                        self.phase = BoundedValidationPhase::Triangles;
                    }
                }
                BoundedValidationPhase::Triangles => {
                    let Some(triangle) = mesh.triangles.get(self.cursor).copied() else {
                        self.cursor = 0;
                        self.edge_key = self.edges.records.keys().next().copied();
                        self.phase = BoundedValidationPhase::ScanEdges;
                        continue;
                    };
                    let [a, b, c] = triangle.map(|vertex| mesh.positions[vertex as usize]);
                    self.signed_volume += dot3(a, cross3(b, c)) / 6.0;
                    for vertex in triangle {
                        self.used_vertices.insert(vertex);
                        self.incident[vertex as usize].push(self.cursor as u32);
                    }
                    self.cursor += 1;
                }
                BoundedValidationPhase::ScanEdges => {
                    let Some(edge) = self.edge_key else {
                        self.cursor = 0;
                        self.phase = BoundedValidationPhase::ScanVertices;
                        continue;
                    };
                    let record = self.edges.records.get(&edge).expect("bounded validation edge");
                    match record.faces.len() {
                        1 => {
                            self.boundary_edge_count += 1;
                            self.boundary_vertices.extend([edge.0, edge.1]);
                            Self::union(&mut self.vertex_parent, edge.0, edge.1);
                        }
                        2 => {
                            Self::union(&mut self.face_parent, record.faces[0], record.faces[1]);
                            let a = record.directed[0];
                            let b = record.directed[1];
                            let other = mesh.triangles[record.faces[1] as usize];
                            if (0..3).any(|slot| other[slot] == a && other[(slot + 1) % 3] == b) {
                                self.consistently_oriented = false;
                            }
                        }
                        _ => self.non_manifold_edge_count += 1,
                    }
                    self.edge_key = self.edges.records.range((std::ops::Bound::Excluded(edge), std::ops::Bound::Unbounded)).next().map(|(next, _)| *next);
                }
                BoundedValidationPhase::ScanVertices => {
                    let Some(faces) = self.incident.get(self.cursor) else {
                        self.cursor = 0;
                        self.phase = BoundedValidationPhase::Components;
                        continue;
                    };
                    if faces.is_empty() {
                        self.cursor += 1;
                        continue;
                    }
                    if let Some(root) = self.incident_root {
                        if self.incident_face_cursor < faces.len() {
                            let face = faces[self.incident_face_cursor];
                            if Self::root(&mut self.face_parent, face) != root {
                                self.incident_conflict = true;
                            }
                            self.incident_face_cursor += 1;
                        }
                    } else {
                        self.incident_root = Some(Self::root(&mut self.face_parent, faces[0]));
                        self.incident_face_cursor = 1;
                    }
                    if self.incident_face_cursor >= faces.len() {
                        self.non_manifold_vertex_count += usize::from(self.incident_conflict);
                        self.cursor += 1;
                        self.incident_face_cursor = 0;
                        self.incident_root = None;
                        self.incident_conflict = false;
                    }
                }
                BoundedValidationPhase::Components => {
                    if self.cursor < self.face_parent.len() {
                        self.component_roots.insert(Self::root(&mut self.face_parent, self.cursor as u32));
                        self.cursor += 1;
                    } else {
                        self.cursor = 0;
                        self.boundary_vertex = self.boundary_vertices.iter().next().copied();
                        self.phase = BoundedValidationPhase::BoundaryLoops;
                    }
                }
                BoundedValidationPhase::BoundaryLoops => {
                    let Some(vertex) = self.boundary_vertex else {
                        self.phase = if self.check_self_intersections { BoundedValidationPhase::Intersections } else { BoundedValidationPhase::Done };
                        continue;
                    };
                    self.boundary_roots.insert(Self::root(&mut self.vertex_parent, vertex));
                    self.boundary_vertex = self.boundary_vertices.range((std::ops::Bound::Excluded(vertex), std::ops::Bound::Unbounded)).next().copied();
                }
                BoundedValidationPhase::Intersections => {
                    if self.pair.0 >= mesh.triangles.len() {
                        self.phase = BoundedValidationPhase::Done;
                        continue;
                    }
                    if self.pair.1 >= mesh.triangles.len() {
                        self.pair.0 += 1;
                        self.pair.1 = self.pair.0 + 1;
                        continue;
                    }
                    let a = mesh.triangles[self.pair.0];
                    let b = mesh.triangles[self.pair.1];
                    if !a.iter().any(|vertex| b.contains(vertex)) {
                        let positions_a = a.map(|vertex| mesh.positions[vertex as usize]);
                        let positions_b = b.map(|vertex| mesh.positions[vertex as usize]);
                        if triangles_intersect(positions_a, positions_b) {
                            self.self_intersections += 1;
                        }
                    }
                    self.pair.1 += 1;
                }
                BoundedValidationPhase::Done => return true,
            }
        }
        self.phase == BoundedValidationPhase::Done
    }

    pub(super) fn finish(self, mesh: &TriMesh) -> WatertightReport {
        let connected_components = if mesh.triangles.is_empty() { 0 } else { self.component_roots.len() };
        let is_closed = self.boundary_edge_count == 0;
        let is_two_manifold = self.non_manifold_edge_count == 0 && self.non_manifold_vertex_count == 0;
        let euler_characteristic = self.used_vertices.len() as i64 - self.edges.records.len() as i64 + mesh.triangles.len() as i64;
        WatertightReport {
            vertex_count: mesh.positions.len(),
            triangle_count: mesh.triangles.len(),
            boundary_edge_count: self.boundary_edge_count,
            boundary_loop_count: self.boundary_roots.len(),
            non_manifold_edge_count: self.non_manifold_edge_count,
            non_manifold_vertex_count: self.non_manifold_vertex_count,
            connected_components,
            consistently_oriented: self.consistently_oriented,
            euler_characteristic,
            genus: (is_closed && is_two_manifold && connected_components == 1).then_some((2 - euler_characteristic) / 2),
            signed_volume: self.signed_volume,
            self_intersection_pairs: self.check_self_intersections.then_some(self.self_intersections),
            closed_fallback_used: false,
            is_closed,
            is_two_manifold,
            is_watertight: is_closed && is_two_manifold && self.consistently_oriented,
        }
    }
}
// #endregion 🔖️BoundedValidation
