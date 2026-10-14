//! 🕸️ Surface extraction and texturing: marching cubes, quadric simplification, hole filling, UV atlas and multi-view texture blending.
//!
//! This crate's one non-negotiable contract: [`mesh_pipeline_step`] always terminates in either
//! `Done` with [`WatertightReport::is_watertight`] `true`, or an explicit `Failed`. There is no
//! silent "good enough" mesh — [`close_voxel`] is a by-construction closed-2-manifold fallback
//! that the pipeline falls back to whenever repair/hole-filling leaves defects behind.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).

use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_dense::editor::remodeling::engine::dense as remodeling_dense;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::SimplifyParams;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::TextureView;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::bake_texture;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::segment_charts;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::simplify_qem;
use semio_s_artifact_remodel_remodeling_mesh_atlas::editor::remodeling::engine::mesh::atlas::unwrap_mesh;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedCleanPreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedEdgePreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedHoleFillPreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedOrientPreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::BoundedRepairPreparation;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::HoleFillParams;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::WatertightReport;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::clean_mesh;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::close_voxel;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::fill_holes;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::orient_consistently;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::orient_outward;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::repair_non_manifold;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::taubin_smooth;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::triangles_intersect;
use semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::validate_watertight;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::TSDF_EXTRACTION_POINTS_PER_DISPATCH;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::TriMesh;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::TsdfExtractionPreparation;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::add3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::cross3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::dot3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::extract_tsdf;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::norm3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::normalize3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::scale3;
use semio_s_artifact_remodel_remodeling_mesh_surface::editor::remodeling::engine::mesh::surface::sub3;


// #region 🔖️Interchange

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum InterchangePhase {
    AllocateNormals,
    AccumulateNormals,
    NormalizeNormals,
    Positions,
    Normals,
    Indices,
    Uvs,
    Texture,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
struct InterchangePreparation {
    phase: InterchangePhase,
    cursor: usize,
    accumulated_normals: Vec<[f64; 3]>,
    mesh_data: semio_framework::MeshData,
    texture_png: Option<BoundedTexturePngPreparation>,
}

impl InterchangePreparation {
    fn new(mesh: &TriMesh) -> Self {
        Self {
            phase: InterchangePhase::AllocateNormals,
            cursor: 0,
            accumulated_normals: Vec::with_capacity(mesh.positions.len()),
            mesh_data: semio_framework::MeshData { positions: Vec::with_capacity(mesh.positions.len() * 3), normals: Vec::with_capacity(mesh.positions.len() * 3), indices: Vec::with_capacity(mesh.triangles.len() * 3), ..Default::default() },
            texture_png: None,
        }
    }

    fn advance(&mut self, mesh: &TriMesh, uvs: Option<&[[f32; 2]]>, texture: Option<&remodeling_image::ImageRgba8>, item_budget: usize) -> bool {
        let budget = item_budget.max(1);
        match self.phase {
            InterchangePhase::AllocateNormals => {
                let end = self.cursor.saturating_add(budget).min(mesh.positions.len());
                self.accumulated_normals.resize(end, [0.0; 3]);
                self.cursor = end;
                if end == mesh.positions.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::AccumulateNormals;
                }
            }
            InterchangePhase::AccumulateNormals => {
                let end = self.cursor.saturating_add(budget).min(mesh.triangles.len());
                for triangle_index in self.cursor..end {
                    let triangle = mesh.triangles[triangle_index];
                    let normal = mesh.face_normal_unnormalized(triangle_index);
                    for vertex in triangle {
                        for (accumulated, coordinate) in self.accumulated_normals[vertex as usize].iter_mut().zip(normal) {
                            *accumulated += coordinate;
                        }
                    }
                }
                self.cursor = end;
                if end == mesh.triangles.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::NormalizeNormals;
                }
            }
            InterchangePhase::NormalizeNormals => {
                let end = self.cursor.saturating_add(budget).min(self.accumulated_normals.len());
                for normal in &mut self.accumulated_normals[self.cursor..end] {
                    *normal = normalize3(*normal);
                }
                self.cursor = end;
                if end == self.accumulated_normals.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::Positions;
                }
            }
            InterchangePhase::Positions => {
                let end = self.cursor.saturating_add(budget).min(mesh.positions.len());
                for position in &mesh.positions[self.cursor..end] {
                    self.mesh_data.positions.extend(position.map(|coordinate| coordinate as f32));
                }
                self.cursor = end;
                if end == mesh.positions.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::Normals;
                }
            }
            InterchangePhase::Normals => {
                let end = self.cursor.saturating_add(budget).min(self.accumulated_normals.len());
                for normal in &self.accumulated_normals[self.cursor..end] {
                    self.mesh_data.normals.extend(normal.map(|coordinate| coordinate as f32));
                }
                self.cursor = end;
                if end == self.accumulated_normals.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::Indices;
                }
            }
            InterchangePhase::Indices => {
                let end = self.cursor.saturating_add(budget).min(mesh.triangles.len());
                for triangle in &mesh.triangles[self.cursor..end] {
                    self.mesh_data.indices.extend(*triangle);
                }
                self.cursor = end;
                if end == mesh.triangles.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::Uvs;
                }
            }
            InterchangePhase::Uvs => {
                let values = uvs.unwrap_or_default();
                let end = self.cursor.saturating_add(budget).min(values.len());
                for uv in &values[self.cursor..end] {
                    self.mesh_data.uvs.extend(*uv);
                }
                self.cursor = end;
                if end == values.len() {
                    self.phase = InterchangePhase::Texture;
                }
            }
            InterchangePhase::Texture => {
                let Some(texture) = texture else {
                    self.phase = InterchangePhase::Done;
                    return true;
                };
                let preparation = self.texture_png.get_or_insert_with(|| BoundedTexturePngPreparation::new(texture));
                if preparation.advance(texture) {
                    self.mesh_data.paint_texture_base64 = self.texture_png.take().map(BoundedTexturePngPreparation::finish);
                    self.phase = InterchangePhase::Done;
                }
            }
            InterchangePhase::Done => {}
        }
        self.phase == InterchangePhase::Done
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedTexturePngPhase {
    Header,
    Rows,
    Footer,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
struct BoundedTexturePngPreparation {
    phase: BoundedTexturePngPhase,
    row: u32,
    adler_a: u32,
    adler_b: u32,
    base64: String,
    carry: Vec<u8>,
}

impl BoundedTexturePngPreparation {
    fn new(image: &remodeling_image::ImageRgba8) -> Self {
        let encoded_len = (image.data.len() + image.height as usize * 6 + 96).div_ceil(3) * 4;
        Self { phase: BoundedTexturePngPhase::Header, row: 0, adler_a: 1, adler_b: 0, base64: String::with_capacity(encoded_len), carry: Vec::with_capacity(2) }
    }

    fn append_png_chunk(bytes: &mut Vec<u8>, kind: &[u8; 4], payload: &[u8]) {
        bytes.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        bytes.extend_from_slice(kind);
        bytes.extend_from_slice(payload);
        let mut crc = 0xffff_ffffu32;
        for byte in kind.iter().chain(payload) {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0xedb8_8320u32 & (0u32.wrapping_sub(crc & 1)));
            }
        }
        bytes.extend_from_slice(&(!crc).to_be_bytes());
    }

    fn append_stored_block(bytes: &mut Vec<u8>, final_block: bool, raw: &[u8]) {
        bytes.push(u8::from(final_block));
        let length = raw.len() as u16;
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes.extend_from_slice(&(!length).to_le_bytes());
        bytes.extend_from_slice(raw);
    }

    fn append_base64(&mut self, bytes: &[u8], final_chunk: bool) {
        let mut pending = std::mem::take(&mut self.carry);
        pending.extend_from_slice(bytes);
        let encoded_end = if final_chunk { pending.len() } else { pending.len() / 3 * 3 };
        if encoded_end > 0 {
            self.base64.push_str(&base64_codec::base64_standard_encode(&pending[..encoded_end]));
        }
        if !final_chunk {
            self.carry.extend_from_slice(&pending[encoded_end..]);
        }
    }

    fn advance(&mut self, image: &remodeling_image::ImageRgba8) -> bool {
        match self.phase {
            BoundedTexturePngPhase::Header => {
                let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
                let mut ihdr = Vec::with_capacity(13);
                ihdr.extend_from_slice(&image.width.to_be_bytes());
                ihdr.extend_from_slice(&image.height.to_be_bytes());
                ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
                Self::append_png_chunk(&mut bytes, b"IHDR", &ihdr);
                Self::append_png_chunk(&mut bytes, b"IDAT", &[0x78, 0x01]);
                self.append_base64(&bytes, false);
                self.phase = BoundedTexturePngPhase::Rows;
            }
            BoundedTexturePngPhase::Rows => {
                if self.row >= image.height {
                    self.phase = BoundedTexturePngPhase::Footer;
                    return false;
                }
                let start = self.row as usize * image.width as usize * 4;
                let end = start + image.width as usize * 4;
                let mut raw = Vec::with_capacity(end - start + 1);
                raw.push(0);
                raw.extend_from_slice(&image.data[start..end]);
                for byte in &raw {
                    self.adler_a = (self.adler_a + u32::from(*byte)) % 65_521;
                    self.adler_b = (self.adler_b + self.adler_a) % 65_521;
                }
                let mut deflate = Vec::with_capacity(raw.len() + 5);
                Self::append_stored_block(&mut deflate, false, &raw);
                let mut bytes = Vec::with_capacity(deflate.len() + 12);
                Self::append_png_chunk(&mut bytes, b"IDAT", &deflate);
                self.append_base64(&bytes, false);
                self.row += 1;
            }
            BoundedTexturePngPhase::Footer => {
                let mut deflate = Vec::with_capacity(9);
                Self::append_stored_block(&mut deflate, true, &[]);
                deflate.extend_from_slice(&((self.adler_b << 16) | self.adler_a).to_be_bytes());
                let mut bytes = Vec::with_capacity(33);
                Self::append_png_chunk(&mut bytes, b"IDAT", &deflate);
                Self::append_png_chunk(&mut bytes, b"IEND", &[]);
                self.append_base64(&bytes, true);
                self.phase = BoundedTexturePngPhase::Done;
            }
            BoundedTexturePngPhase::Done => {}
        }
        self.phase == BoundedTexturePngPhase::Done
    }

    fn finish(self) -> String {
        self.base64
    }
}

// #endregion 🔖️Interchange

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
struct BoundedValidationPreparation {
    phase: BoundedValidationPhase,
    edges: BoundedEdgePreparation,
    face_parent: Vec<u32>,
    vertex_parent: Vec<u32>,
    incident: Vec<Vec<u32>>,
    cursor: usize,
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
    boundary_edge_count: usize,
    non_manifold_edge_count: usize,
    non_manifold_vertex_count: usize,
    consistently_oriented: bool,
    signed_volume: f64,
    self_intersections: usize,
    check_self_intersections: bool,
}

impl BoundedValidationPreparation {
    fn new(check_self_intersections: bool) -> Self {
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

    fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {
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

    fn finish(self, mesh: &TriMesh) -> WatertightReport {
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

// #region 🔖️BoundedPostprocess

#[derive(semio_framework_value::RetireOwned)]
struct BoundedOutwardPreparation {
    cursor: usize,
    signed_volume: f64,
    flip: Option<bool>,
}

impl BoundedOutwardPreparation {
    fn new() -> Self {
        Self { cursor: 0, signed_volume: 0.0, flip: None }
    }

    fn advance(&mut self, mesh: &mut TriMesh, triangle_budget: usize) -> bool {
        let end = self.cursor.saturating_add(triangle_budget.max(1)).min(mesh.triangles.len());
        if let Some(flip) = self.flip {
            if flip {
                for triangle in &mut mesh.triangles[self.cursor..end] {
                    triangle.swap(1, 2);
                }
            }
        } else {
            for triangle in &mesh.triangles[self.cursor..end] {
                let [a, b, c] = triangle.map(|vertex| mesh.positions[vertex as usize]);
                self.signed_volume += dot3(a, cross3(b, c)) / 6.0;
            }
        }
        self.cursor = end;
        if end < mesh.triangles.len() {
            return false;
        }
        if self.flip.is_none() {
            self.flip = Some(self.signed_volume < 0.0);
            self.cursor = 0;
            return mesh.triangles.is_empty();
        }
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedTaubinPhase {
    AllocateNeighbors,
    Edges,
    Vertices,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
struct BoundedTaubinPreparation {
    phase: BoundedTaubinPhase,
    cursor: usize,
    neighbors: Vec<BTreeSet<u32>>,
    next: Vec<[f64; 3]>,
    pass: usize,
    passes: usize,
    lambda: f64,
    mu: f64,
}

impl BoundedTaubinPreparation {
    fn new(iterations: usize, lambda: f64, mu: f64) -> Self {
        Self { phase: BoundedTaubinPhase::AllocateNeighbors, cursor: 0, neighbors: Vec::new(), next: Vec::new(), pass: 0, passes: iterations.saturating_mul(2), lambda, mu }
    }

    fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {
        for _ in 0..item_budget.max(1) {
            match self.phase {
                BoundedTaubinPhase::AllocateNeighbors => {
                    if self.neighbors.len() < mesh.positions.len() {
                        self.neighbors.push(BTreeSet::new());
                    } else {
                        self.phase = BoundedTaubinPhase::Edges;
                    }
                }
                BoundedTaubinPhase::Edges => {
                    let Some(triangle) = mesh.triangles.get(self.cursor).copied() else {
                        self.cursor = 0;
                        self.next = Vec::with_capacity(mesh.positions.len());
                        self.phase = if self.passes == 0 { BoundedTaubinPhase::Done } else { BoundedTaubinPhase::Vertices };
                        continue;
                    };
                    for edge in 0..3 {
                        let a = triangle[edge];
                        let b = triangle[(edge + 1) % 3];
                        self.neighbors[a as usize].insert(b);
                        self.neighbors[b as usize].insert(a);
                    }
                    self.cursor += 1;
                }
                BoundedTaubinPhase::Vertices => {
                    if self.cursor < mesh.positions.len() {
                        let position = mesh.positions[self.cursor];
                        let neighbors = &self.neighbors[self.cursor];
                        if neighbors.is_empty() {
                            self.next.push(position);
                        } else {
                            let sum = neighbors.iter().fold([0.0; 3], |sum, &neighbor| add3(sum, mesh.positions[neighbor as usize]));
                            let average = scale3(sum, 1.0 / neighbors.len() as f64);
                            let factor = if self.pass.is_multiple_of(2) { self.lambda } else { self.mu };
                            self.next.push(add3(position, scale3(sub3(average, position), factor)));
                        }
                        self.cursor += 1;
                    } else {
                        mesh.positions = std::mem::take(&mut self.next);
                        self.pass += 1;
                        self.cursor = 0;
                        if self.pass == self.passes {
                            self.phase = BoundedTaubinPhase::Done;
                        } else {
                            self.next = Vec::with_capacity(mesh.positions.len());
                        }
                    }
                }
                BoundedTaubinPhase::Done => return true,
            }
        }
        self.phase == BoundedTaubinPhase::Done
    }
}

#[derive(semio_framework_value::RetireOwned)]
struct BoundedSimplifyPreparation {
    cursor: usize,
    target: usize,
    total: usize,
    triangles: Vec<[u32; 3]>,
}

impl BoundedSimplifyPreparation {
    fn new(mesh: &TriMesh, target: usize) -> Self {
        Self { cursor: 0, target: target.min(mesh.triangles.len()), total: mesh.triangles.len(), triangles: Vec::with_capacity(target.min(mesh.triangles.len())) }
    }

    fn advance(&mut self, mesh: &TriMesh, triangle_budget: usize) -> bool {
        let end = self.cursor.saturating_add(triangle_budget.max(1)).min(self.total);
        for index in self.cursor..end {
            if (index + 1).saturating_mul(self.target) / self.total.max(1) > index.saturating_mul(self.target) / self.total.max(1) {
                self.triangles.push(mesh.triangles[index]);
            }
        }
        self.cursor = end;
        end == self.total
    }

    fn finish(self, mesh: &mut TriMesh) {
        mesh.triangles = self.triangles;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedUnwrapPhase {
    Bounds,
    Vertices,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
struct BoundedUnwrapPreparation {
    phase: BoundedUnwrapPhase,
    cursor: usize,
    lower: [f64; 3],
    upper: [f64; 3],
    axes: [usize; 2],
    uvs: Vec<[f32; 2]>,
}

impl BoundedUnwrapPreparation {
    fn new(mesh: &TriMesh) -> Self {
        Self { phase: BoundedUnwrapPhase::Bounds, cursor: 0, lower: [f64::INFINITY; 3], upper: [f64::NEG_INFINITY; 3], axes: [0, 1], uvs: Vec::with_capacity(mesh.positions.len()) }
    }

    fn advance(&mut self, mesh: &TriMesh, vertex_budget: usize) -> bool {
        let end = self.cursor.saturating_add(vertex_budget.max(1)).min(mesh.positions.len());
        match self.phase {
            BoundedUnwrapPhase::Bounds => {
                for position in &mesh.positions[self.cursor..end] {
                    for (axis, coordinate) in position.iter().copied().enumerate() {
                        self.lower[axis] = self.lower[axis].min(coordinate);
                        self.upper[axis] = self.upper[axis].max(coordinate);
                    }
                }
                self.cursor = end;
                if end == mesh.positions.len() {
                    let mut spans = (0..3).map(|axis| (axis, self.upper[axis] - self.lower[axis])).collect::<Vec<_>>();
                    spans.sort_by(|a, b| b.1.total_cmp(&a.1));
                    self.axes = [spans[0].0, spans[1].0];
                    self.cursor = 0;
                    self.phase = BoundedUnwrapPhase::Vertices;
                }
            }
            BoundedUnwrapPhase::Vertices => {
                for position in &mesh.positions[self.cursor..end] {
                    self.uvs.push(std::array::from_fn(|component| {
                        let axis = self.axes[component];
                        ((position[axis] - self.lower[axis]) / (self.upper[axis] - self.lower[axis]).max(1e-12)) as f32
                    }));
                }
                self.cursor = end;
                if end == mesh.positions.len() {
                    self.phase = BoundedUnwrapPhase::Done;
                }
            }
            BoundedUnwrapPhase::Done => {}
        }
        self.phase == BoundedUnwrapPhase::Done
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedTexturePhase {
    Allocate,
    Faces,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
struct BoundedTexturePreparation {
    phase: BoundedTexturePhase,
    atlas_size: u32,
    data: Vec<u8>,
    face: usize,
    pixel: u32,
}

impl BoundedTexturePreparation {
    fn new(atlas_size: u32) -> Self {
        Self { phase: BoundedTexturePhase::Allocate, atlas_size, data: Vec::with_capacity(atlas_size as usize * atlas_size as usize * 4), face: 0, pixel: 0 }
    }

    fn advance(&mut self, mesh: &TriMesh, uvs: &[[f32; 2]], views: &[TextureView], item_budget: usize) -> bool {
        for _ in 0..item_budget.max(1) {
            match self.phase {
                BoundedTexturePhase::Allocate => {
                    let target = self.atlas_size as usize * self.atlas_size as usize * 4;
                    let end = self.data.len().saturating_add(4).min(target);
                    self.data.resize(end, 0);
                    if end == target {
                        self.phase = BoundedTexturePhase::Faces;
                    }
                }
                BoundedTexturePhase::Faces => {
                    let Some(triangle) = mesh.triangles.get(self.face).copied() else {
                        self.phase = BoundedTexturePhase::Done;
                        continue;
                    };
                    let atlas_pixels = self.atlas_size.saturating_mul(self.atlas_size);
                    if self.pixel >= atlas_pixels {
                        self.face += 1;
                        self.pixel = 0;
                        continue;
                    }
                    let x = self.pixel % self.atlas_size;
                    let y = self.pixel / self.atlas_size;
                    self.pixel += 1;
                    let uv = triangle.map(|vertex| uvs[vertex as usize]);
                    let p = [(f64::from(x) + 0.5) / f64::from(self.atlas_size), (f64::from(y) + 0.5) / f64::from(self.atlas_size)];
                    let denominator = f64::from((uv[1][1] - uv[2][1]) * (uv[0][0] - uv[2][0]) + (uv[2][0] - uv[1][0]) * (uv[0][1] - uv[2][1]));
                    if denominator.abs() < 1e-12 {
                        continue;
                    }
                    let w0 = (f64::from(uv[1][1] - uv[2][1]) * (p[0] - f64::from(uv[2][0])) + f64::from(uv[2][0] - uv[1][0]) * (p[1] - f64::from(uv[2][1]))) / denominator;
                    let w1 = (f64::from(uv[2][1] - uv[0][1]) * (p[0] - f64::from(uv[2][0])) + f64::from(uv[0][0] - uv[2][0]) * (p[1] - f64::from(uv[2][1]))) / denominator;
                    let w2 = 1.0 - w0 - w1;
                    if w0 < -1e-6 || w1 < -1e-6 || w2 < -1e-6 {
                        continue;
                    }
                    let positions = triangle.map(|vertex| mesh.positions[vertex as usize]);
                    let world = add3(add3(scale3(positions[0], w0), scale3(positions[1], w1)), scale3(positions[2], w2));
                    let sample = views.iter().find_map(|view| {
                        let camera = view.pose.0.act(world);
                        (camera[2] > 0.0)
                            .then(|| view.intrinsics.project(camera))
                            .flatten()
                            .and_then(|pixel| (pixel[0] >= 0.0 && pixel[1] >= 0.0 && pixel[0] < f64::from(view.image.width) && pixel[1] < f64::from(view.image.height)).then(|| view.image.sample_rgb(pixel[0] as f32, pixel[1] as f32)))
                    });
                    if let Some(rgb) = sample {
                        let index = ((y * self.atlas_size + x) * 4) as usize;
                        self.data[index] = (rgb[0].clamp(0.0, 1.0) * 255.0) as u8;
                        self.data[index + 1] = (rgb[1].clamp(0.0, 1.0) * 255.0) as u8;
                        self.data[index + 2] = (rgb[2].clamp(0.0, 1.0) * 255.0) as u8;
                        self.data[index + 3] = 255;
                    }
                }
                BoundedTexturePhase::Done => return true,
            }
        }
        self.phase == BoundedTexturePhase::Done
    }

    fn finish(self) -> remodeling_image::ImageRgba8 {
        remodeling_image::ImageRgba8 { width: self.atlas_size, height: self.atlas_size, data: self.data }
    }
}

// #endregion 🔖️BoundedPostprocess

// #region 🔖️Pipeline

#[derive(Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(semio_framework_value::RetireOwned)]
pub struct MeshPipeline {
    mesh: TriMesh,
    tsdf: Option<remodeling_dense::TsdfVolume>,
    extraction: Option<TsdfExtractionPreparation>,
    params: MeshParams,
    stage_index: usize,
    close_used: bool,
    tsdf_voxel_size: Option<f64>,
    report: Option<WatertightReport>,
    failed: Option<String>,
    uvs: Option<Vec<[f32; 2]>>,
    texture: Option<remodeling_image::ImageRgba8>,
    views: Vec<TextureView>,
    result: Option<semio_framework::MeshData>,
    interchange: Option<InterchangePreparation>,
    interactive: bool,
    cleaning: Option<BoundedCleanPreparation>,
    repairing: Option<BoundedRepairPreparation>,
    orienting: Option<BoundedOrientPreparation>,
    hole_filling: Option<BoundedHoleFillPreparation>,
    validating: Option<BoundedValidationPreparation>,
    outward: Option<BoundedOutwardPreparation>,
    smoothing: Option<BoundedTaubinPreparation>,
    simplifying: Option<BoundedSimplifyPreparation>,
    unwrapping: Option<BoundedUnwrapPreparation>,
    texturing: Option<BoundedTexturePreparation>,
}

impl MeshPipeline {
    #[cfg(test)]
    pub fn new(vol: &remodeling_dense::TsdfVolume, iso: f64, bounds_min: [i32; 3], bounds_max: [i32; 3], params: MeshParams) -> Self {
        let mesh = extract_tsdf(vol, iso, bounds_min, bounds_max);
        Self {
            mesh,
            tsdf: None,
            extraction: None,
            params,
            stage_index: 0,
            close_used: false,
            tsdf_voxel_size: Some(vol.voxel_size),
            report: None,
            failed: None,
            uvs: None,
            texture: None,
            views: Vec::new(),
            result: None,
            interchange: None,
            interactive: false,
            cleaning: None,
            repairing: None,
            orienting: None,
            hole_filling: None,
            validating: None,
            outward: None,
            smoothing: None,
            simplifying: None,
            unwrapping: None,
            texturing: None,
        }
    }

    pub fn new_bounded(vol: remodeling_dense::TsdfVolume, iso: f64, bounds_min: [i32; 3], bounds_max: [i32; 3], mut params: MeshParams) -> Self {
        let voxel_size = vol.voxel_size;
        params.atlas_size = params.atlas_size.min(64);
        params.taubin_iterations = params.taubin_iterations.min(4);
        params.hole_fill_max_boundary_verts = params.hole_fill_max_boundary_verts.min(128);
        Self {
            mesh: TriMesh::new(),
            tsdf: Some(vol),
            extraction: Some(TsdfExtractionPreparation::new(iso, bounds_min, bounds_max)),
            params,
            stage_index: 0,
            close_used: false,
            tsdf_voxel_size: Some(voxel_size),
            report: None,
            failed: None,
            uvs: None,
            texture: None,
            views: Vec::new(),
            result: None,
            interchange: None,
            interactive: true,
            cleaning: None,
            repairing: None,
            orienting: None,
            hole_filling: None,
            validating: None,
            outward: None,
            smoothing: None,
            simplifying: None,
            unwrapping: None,
            texturing: None,
        }
    }

    #[cfg(test)]
    pub fn from_mesh(mesh: TriMesh, params: MeshParams) -> Self {
        Self {
            mesh,
            tsdf: None,
            extraction: None,
            params,
            stage_index: 1,
            close_used: false,
            tsdf_voxel_size: None,
            report: None,
            failed: None,
            uvs: None,
            texture: None,
            views: Vec::new(),
            result: None,
            interchange: None,
            interactive: false,
            cleaning: None,
            repairing: None,
            orienting: None,
            hole_filling: None,
            validating: None,
            outward: None,
            smoothing: None,
            simplifying: None,
            unwrapping: None,
            texturing: None,
        }
    }

    /// 👁️ Extract only the observed surface and let the hole fill cap it; see
    /// [`TsdfExtractionPreparation::observed_only`].
    pub fn with_observed_only_surface(mut self) -> Self {
        self.extraction = self.extraction.map(TsdfExtractionPreparation::observed_only);
        self
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

/// ⚙️ Advances the pipeline through at most `budget` stage dispatches. Interactive pipelines
/// retain explicit extraction, topology, validation, smoothing, unwrap, texture, and interchange
/// cursors; a dispatch consumes only the fixed work allowance of the active cursor.
pub fn mesh_pipeline_step(state: &mut MeshPipeline, budget: usize) -> MeshPipelineStatus {
    for _ in 0..budget.max(1) {
        if state.interactive && (state.mesh.positions.len() > 512 || state.mesh.triangles.len() > 512) {
            let message = format!("interactive mesh envelope exceeded: {} vertices, {} triangles", state.mesh.positions.len(), state.mesh.triangles.len());
            state.failed = Some(message.clone());
            return MeshPipelineStatus::Failed(message);
        }
        if state.stage_index >= STAGE_ORDER.len() {
            return MeshPipelineStatus::Done;
        }
        let stage = STAGE_ORDER[state.stage_index];
        match stage {
            Stage::Mc => {
                if let (Some(extraction), Some(volume)) = (state.extraction.as_mut(), state.tsdf.as_ref()) {
                    let complete = extraction.advance(volume, TSDF_EXTRACTION_POINTS_PER_DISPATCH);
                    if extraction.exceeded() {
                        // 🧊️ The full-resolution net does not fit the element envelope: restart it one
                        // stride coarser (a reconstruction volume is 32 cells per axis, so the surface
                        // survives the coarsening) rather than refusing the reconstruction outright.
                        match extraction.coarsened() {
                            Some(coarser) => {
                                state.extraction = Some(coarser);
                                return MeshPipelineStatus::Working { stage: "marching_cubes", progress: 0.0 };
                            }
                            None => {
                                let message = "interactive mesh envelope exceeded during bounded TSDF extraction".to_string();
                                state.failed = Some(message.clone());
                                return MeshPipelineStatus::Failed(message);
                            }
                        }
                    }
                    if !complete {
                        return MeshPipelineStatus::Working { stage: "marching_cubes", progress: 0.0 };
                    }
                    state.mesh = state.extraction.take().and_then(TsdfExtractionPreparation::finish).expect("completed TSDF extraction");
                    state.tsdf = None;
                }
            }
            Stage::Clean => {
                if state.interactive {
                    let preparation = state.cleaning.get_or_insert_with(|| BoundedCleanPreparation::new(&state.mesh, state.params.min_component_faces, state.params.min_component_bbox_fraction, state.params.guarantee_watertight));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "clean", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.mesh = state.cleaning.take().expect("completed cleanup").finish();
                } else {
                    clean_mesh(&mut state.mesh, state.params.min_component_faces, state.params.min_component_bbox_fraction);
                }
            }
            Stage::Repair => {
                if state.interactive {
                    let preparation = state.repairing.get_or_insert_with(|| BoundedRepairPreparation::new(&state.mesh));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "repair", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.repairing.take().expect("completed repair").finish(&mut state.mesh);
                } else {
                    repair_non_manifold(&mut state.mesh);
                }
            }
            Stage::OrientConsistency => {
                if state.interactive {
                    let preparation = state.orienting.get_or_insert_with(BoundedOrientPreparation::new);
                    if !preparation.advance(&mut state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "orient_consistency", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.orienting = None;
                } else {
                    let _ = orient_consistently(&mut state.mesh);
                }
            }
            Stage::HoleFill => {
                if state.interactive {
                    let preparation = state.hole_filling.get_or_insert_with(|| BoundedHoleFillPreparation::new(state.params.hole_fill_max_boundary_verts));
                    let complete = preparation.advance(&mut state.mesh, 64);
                    if preparation.exceeded() {
                        let message = "interactive mesh envelope exceeded during bounded hole filling".to_string();
                        state.failed = Some(message.clone());
                        return MeshPipelineStatus::Failed(message);
                    }
                    if !complete {
                        return MeshPipelineStatus::Working { stage: "hole_fill", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.hole_filling = None;
                } else {
                    fill_holes(&mut state.mesh, &HoleFillParams { max_boundary_verts: state.params.hole_fill_max_boundary_verts });
                }
            }
            Stage::Validate1 => {
                let report = if state.interactive {
                    let preparation = state.validating.get_or_insert_with(|| BoundedValidationPreparation::new(false));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "validate", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.validating.take().expect("completed validation").finish(&state.mesh)
                } else {
                    validate_watertight(&state.mesh, false)
                };
                let needs_close = state.params.guarantee_watertight && needs_close_fallback(&report);
                state.report = Some(report);
                if !needs_close {
                    state.stage_index = Stage::OrientOutward as usize;
                    continue;
                }
            }
            Stage::Close => {
                if state.interactive {
                    // 🧾️ The refusal names what the validation saw, so a run's failure line says whether
                    // the net came out open (boundary loops), fragmented (components) or non-manifold.
                    let seen = state.report.as_ref().map_or_else(String::new, |report| {
                        format!(
                            " (validated {} vertices / {} triangles: {} boundary edges in {} loops, {} non-manifold edges, {} non-manifold vertices, {} components, oriented={})",
                            report.vertex_count, report.triangle_count, report.boundary_edge_count, report.boundary_loop_count, report.non_manifold_edge_count, report.non_manifold_vertex_count, report.connected_components, report.consistently_oriented
                        )
                    });
                    let message = format!("interactive mesh requires a bounded watertight input; voxel-close fallback is not admitted{seen}");
                    state.failed = Some(message.clone());
                    return MeshPipelineStatus::Failed(message);
                }
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
                let mut report = if state.interactive {
                    let check = state.params.self_intersection_check;
                    let preparation = state.validating.get_or_insert_with(|| BoundedValidationPreparation::new(check));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "revalidate", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.validating.take().expect("completed revalidation").finish(&state.mesh)
                } else {
                    validate_watertight(&state.mesh, state.params.self_intersection_check)
                };
                report.closed_fallback_used = state.close_used;
                if state.params.guarantee_watertight && !report.is_watertight {
                    state.report = Some(report);
                    state.failed = Some("close_voxel fallback did not yield a watertight mesh".to_string());
                    return MeshPipelineStatus::Failed(state.failed.clone().expect("just set"));
                }
                state.report = Some(report);
            }
            Stage::OrientOutward => {
                if state.interactive {
                    let preparation = state.outward.get_or_insert_with(BoundedOutwardPreparation::new);
                    if !preparation.advance(&mut state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "orient_outward", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.outward = None;
                } else {
                    orient_outward(&mut state.mesh);
                }
                if let Some(report) = &mut state.report {
                    report.closed_fallback_used = state.close_used;
                    report.signed_volume = if state.interactive { report.signed_volume.abs() } else { state.mesh.signed_volume() };
                }
            }
            Stage::Taubin => {
                if state.interactive {
                    let preparation = state.smoothing.get_or_insert_with(|| BoundedTaubinPreparation::new(state.params.taubin_iterations, state.params.taubin_lambda, state.params.taubin_mu));
                    if !preparation.advance(&mut state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "taubin", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.smoothing = None;
                } else if state.params.taubin_iterations > 0 {
                    taubin_smooth(&mut state.mesh, state.params.taubin_lambda, state.params.taubin_mu, state.params.taubin_iterations);
                }
            }
            Stage::Qem => {
                if state.interactive && state.params.target_triangles < state.mesh.triangles.len() {
                    let preparation = state.simplifying.get_or_insert_with(|| BoundedSimplifyPreparation::new(&state.mesh, state.params.target_triangles));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "simplify", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.simplifying.take().expect("completed simplification").finish(&mut state.mesh);
                } else if state.params.target_triangles < state.mesh.triangles.len() {
                    simplify_qem(&mut state.mesh, state.params.target_triangles, &SimplifyParams::default());
                }
            }
            Stage::Validate2 => {
                let mut report = if state.interactive {
                    let check = state.params.self_intersection_check;
                    let preparation = state.validating.get_or_insert_with(|| BoundedValidationPreparation::new(check));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "validate_light", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.validating.take().expect("completed light validation").finish(&state.mesh)
                } else {
                    validate_watertight(&state.mesh, state.params.self_intersection_check)
                };
                report.closed_fallback_used = state.close_used;
                if state.interactive && state.params.guarantee_watertight && !report.is_watertight {
                    let message = "bounded mesh postprocess did not preserve a watertight surface".to_string();
                    state.report = Some(report);
                    state.failed = Some(message.clone());
                    return MeshPipelineStatus::Failed(message);
                }
                state.report = Some(report);
            }
            Stage::Lscm => {
                if state.interactive {
                    let preparation = state.unwrapping.get_or_insert_with(|| BoundedUnwrapPreparation::new(&state.mesh));
                    if !preparation.advance(&state.mesh, 64) {
                        return MeshPipelineStatus::Working { stage: "unwrap", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                    state.uvs = state.unwrapping.take().map(|preparation| preparation.uvs);
                } else {
                    let charts = segment_charts(&state.mesh, state.params.chart_angle_deg);
                    state.uvs = Some(unwrap_mesh(&mut state.mesh, &charts));
                }
            }
            Stage::TextureBake => {
                if !state.views.is_empty() {
                    if let Some(uvs) = &state.uvs {
                        if state.interactive {
                            let preparation = state.texturing.get_or_insert_with(|| BoundedTexturePreparation::new(state.params.atlas_size));
                            if !preparation.advance(&state.mesh, uvs, &state.views, 64) {
                                return MeshPipelineStatus::Working { stage: "texture_bake", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                            }
                            state.texture = state.texturing.take().map(BoundedTexturePreparation::finish);
                        } else {
                            state.texture = Some(bake_texture(&state.mesh, uvs, state.params.atlas_size, &state.views));
                        }
                    }
                }
            }
            Stage::Interchange => {
                let preparation = state.interchange.get_or_insert_with(|| InterchangePreparation::new(&state.mesh));
                if state.interactive {
                    if !preparation.advance(&state.mesh, state.uvs.as_deref(), state.texture.as_ref(), 64) {
                        return MeshPipelineStatus::Working { stage: "interchange", progress: state.stage_index as f32 / STAGE_ORDER.len() as f32 };
                    }
                } else {
                    // 🧱️ A batch pipeline runs every stage whole per dispatch (see the non-interactive
                    // arms above); only the interactive worker is micro-stepped. Driving the
                    // interchange cursor 64 items per dispatch here made a batch pipeline's step
                    // count scale with the mesh size instead of the stage count.
                    while !preparation.advance(&state.mesh, state.uvs.as_deref(), state.texture.as_ref(), 64) {}
                }
                state.result = state.interchange.take().map(|preparation| preparation.mesh_data);
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
