//! 🔷️ Half-edge mesh kernel for low-poly editing. **Host authority:** `HalfedgeMesh` is a value
//! document/engine payload — not a process-global mesh store.

// 🔬️ `serde`/`serde_json` survive ONLY as a `#[cfg(test)]` differential oracle now that these
// types have their own first-party `ToValue`/`FromValue` codec — never a production dependency
// of this crate. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[cfg(test)]
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[path = "🛠️modeling/🦀️.rs"]
mod modeling;
pub use modeling::{MeshModelingJob, MeshModelingProgress, MeshModelingStep};

/// 🌉️ `HashSet<u32>` has no `ToValue`/`FromValue` blanket impl (`🌱️value/🔁️codec` only covers
/// `Vec`/`BTreeMap<String,_>`/`HashMap<K:ToString,_>`/`Option`/arrays) — `HalfedgeMesh::uv_seams`
/// names this bridge via `#[value(with = "u32_hashset_bridge")]`. Encodes as a `DslValue::Array`,
/// the same shape `serde_json` gives a `HashSet` by default.
mod u32_hashset_bridge {
    pub fn to_value(set: &super::HashSet<u32>) -> dsl_core::value::DslValue {
        dsl_core::value::DslValue::Array(set.iter().map(dsl_core::value::ToValue::to_value).collect())
    }
    pub fn from_value(value: dsl_core::value::DslValue) -> Result<super::HashSet<u32>, dsl_core::value::ValueError> {
        <Vec<u32> as dsl_core::value::FromValue>::from_value(value).map(|items| items.into_iter().collect())
    }
}

//#region Types

// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — newtype tuple struct, `#[value(transparent)]` forwards straight to the `[f32; 3]`
// array's own (blanket) `ToValue`/`FromValue`, matching serde's own newtype-struct default.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value", transparent)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    pub const ZERO: Self = Self([0.0, 0.0, 0.0]);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn x(self) -> f32 {
        self.0[0]
    }
    pub fn y(self) -> f32 {
        self.0[1]
    }
    pub fn z(self) -> f32 {
        self.0[2]
    }

    #[allow(clippy::should_implement_trait, reason = "renaming ripples through lowpoly/core, lowpoly/plugin, remodel/plugin (outside this crate); add/sub read better than +/- across this file's dense vector algebra")]
    pub fn add(self, o: Self) -> Self {
        Self([self.x() + o.x(), self.y() + o.y(), self.z() + o.z()])
    }

    #[allow(clippy::should_implement_trait, reason = "renaming ripples through lowpoly/core, lowpoly/plugin, remodel/plugin (outside this crate); add/sub read better than +/- across this file's dense vector algebra")]
    pub fn sub(self, o: Self) -> Self {
        Self([self.x() - o.x(), self.y() - o.y(), self.z() - o.z()])
    }

    pub fn scale(self, s: f32) -> Self {
        Self([self.x() * s, self.y() * s, self.z() * s])
    }

    pub fn dot(self, o: Self) -> f32 {
        self.x() * o.x() + self.y() * o.y() + self.z() * o.z()
    }

    pub fn cross(self, o: Self) -> Self {
        Self([self.y() * o.z() - self.z() * o.y(), self.z() * o.x() - self.x() * o.z(), self.x() * o.y() - self.y() * o.x()])
    }

    pub fn length(self) -> f32 {
        self.x().hypot(self.y()).hypot(self.z())
    }

    pub fn normalize(self) -> Self {
        let l = (self.x() as f64).hypot(self.y() as f64).hypot(self.z() as f64);
        if l == 0.0 {
            return Self::ZERO;
        }
        Self(self.0.map(|coordinate| (coordinate as f64 / l) as f32))
    }

    pub fn lerp(self, o: Self, t: f32) -> Self {
        self.add(o.sub(self).scale(t))
    }
}

// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `Vec3` above for the `transparent` newtype pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value", transparent)]
pub struct VertexId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value", transparent)]
pub struct HalfEdgeId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value", transparent)]
pub struct FaceId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value", transparent)]
pub struct EdgeId(pub u32);

// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub enum WeldMode {
    Center,
    First,
    ByDistance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub enum MirrorAxis {
    X,
    Y,
    Z,
}

//#region ⚠️ Errors
/// ⚠️ Half-edge mesh kernel operation failure.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `WeldMode` above.
#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub enum MeshKernelError {
    InvalidHandle,
    NonManifold,
    DegenerateOperation,
    EmptySelection,
    InvalidInput(String),
}

impl std::fmt::Display for MeshKernelError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHandle => formatter.write_str("invalid handle"),
            Self::NonManifold => formatter.write_str("mesh is non-manifold"),
            Self::DegenerateOperation => formatter.write_str("degenerate operation"),
            Self::EmptySelection => formatter.write_str("empty selection"),
            Self::InvalidInput(detail) => write!(formatter, "invalid input: {detail}"),
        }
    }
}

impl std::error::Error for MeshKernelError {}
//#endregion ⚠️ Errors

pub type MeshResult<T> = Result<T, MeshKernelError>;

// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01).
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value")]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: Option<[f32; 3]>,
    halfedge: Option<u32>,
}

// 🚫️async: E4 fn-pointer slot — serde's `#[serde(default = "...")]` calls this by path as a plain
// `fn() -> T`; this helper must remain synchronous.
fn default_uv() -> [f32; 2] {
    [0.0, 0.0]
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value")]
struct HalfEdge {
    vertex: u32,
    twin: Option<u32>,
    next: u32,
    face: Option<u32>,
    #[cfg_attr(test, serde(default = "default_uv"))]
    #[value(default = "default_uv")]
    uv: [f32; 2],
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value")]
struct MeshFace {
    halfedge: u32,
    smooth: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    flipped: bool,
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value")]
pub struct MeshTransfer {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub edge_positions: Vec<f32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub face_ids: Vec<u32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub vertex_ids: Vec<u32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub edge_ids: Vec<u32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub uvs: Vec<f32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub edge_uvs: Vec<f32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub edge_is_seam: Vec<u8>,
}

//#endregion Types

//#region HalfedgeMesh

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value")]
pub struct HalfedgeMesh {
    vertices: Vec<MeshVertex>,
    halfedges: Vec<HalfEdge>,
    faces: Vec<MeshFace>,
    #[cfg_attr(test, serde(default))]
    #[value(default, with = "u32_hashset_bridge")]
    uv_seams: HashSet<u32>,
}

impl HalfedgeMesh {
    pub fn empty() -> Self {
        Self { vertices: Vec::new(), halfedges: Vec::new(), faces: Vec::new(), uv_seams: HashSet::new() }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn halfedge_count(&self) -> usize {
        self.halfedges.len()
    }

    pub fn edge_count(&self) -> usize {
        self.halfedges.iter().enumerate().filter(|(index, edge)| edge.twin.is_none_or(|twin| *index < twin as usize)).count()
    }

    pub fn vertex_position(&self, id: VertexId) -> MeshResult<Vec3> {
        self.vertices.get(id.0 as usize).map(|v| Vec3(v.position)).ok_or(MeshKernelError::InvalidHandle)
    }

    pub fn set_vertex_position(&mut self, id: VertexId, pos: Vec3) -> MeshResult<()> {
        let v = self.vertices.get_mut(id.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        v.position = pos.0;
        Ok(())
    }

    pub fn face_vertex_ids(&self, face: FaceId) -> MeshResult<Vec<VertexId>> {
        let f = self.faces.get(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let mut out = Vec::new();
        let start = f.halfedge;
        let mut he = start;
        loop {
            let e = &self.halfedges[he as usize];
            out.push(VertexId(e.vertex));
            he = e.next;
            if he == start {
                break;
            }
        }
        if f.flipped {
            out.reverse();
        }
        Ok(out)
    }

    pub fn face_normal(&self, face: FaceId) -> MeshResult<Vec3> {
        let verts = self.face_vertex_ids(face)?;
        if verts.len() < 3 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let mut positions: Vec<Vec3> = Vec::with_capacity(verts.len());
        for v in &verts {
            positions.push(self.vertex_position(*v)?);
        }
        Ok(newell_normal(&positions).normalize())
    }

    pub fn edge_endpoints(&self, edge: EdgeId) -> MeshResult<(VertexId, VertexId)> {
        let he = self.halfedges.get(edge.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let v0 = VertexId(he.vertex);
        let next = &self.halfedges[he.next as usize];
        let v1 = VertexId(next.vertex);
        Ok((v0, v1))
    }

    pub fn face_halfedge_ids(&self, face: FaceId) -> MeshResult<Vec<u32>> {
        let f = self.faces.get(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let mut out = Vec::new();
        let start = f.halfedge;
        let mut he = start;
        loop {
            out.push(he);
            he = self.halfedges[he as usize].next;
            if he == start {
                break;
            }
        }
        Ok(out)
    }

    pub fn flip_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        for face in faces {
            let entry = self.faces.get_mut(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            entry.flipped = !entry.flipped;
        }
        self.recompute_normals()
    }

    pub fn from_indexed_triangles(positions: &[f32], indices: &[u32]) -> MeshResult<Self> {
        if !positions.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("positions length must be a multiple of 3".into()));
        }
        if !indices.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("indices length must be a multiple of 3".into()));
        }
        let verts: Vec<[f32; 3]> = positions.as_chunks::<3>().0.to_vec();
        let faces: Vec<Vec<u32>> = indices.as_chunks::<3>().0.iter().map(|tri| tri.to_vec()).collect();
        Self::from_faces(&verts, &faces)
    }

    /// Builds a halfedge mesh from a CAD solid tessellation that carries one B-Rep face id per triangle.
    ///
    /// Each B-Rep face is reconstructed independently: coplanar triangles of a simply-connected face merge
    /// into one n-gon; faces with holes keep their triangulation so openings are not filled. Call
    /// [`Self::weld_coincident_vertices`] afterwards so independently-tessellated seam vertices become shared.
    pub fn from_indexed_triangles_by_face_id(positions: &[f32], indices: &[u32], face_ids: &[u32]) -> MeshResult<Self> {
        if face_ids.is_empty() {
            return Self::from_indexed_triangles(positions, indices);
        }
        if !positions.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("positions length must be a multiple of 3".into()));
        }
        if !indices.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("indices length must be a multiple of 3".into()));
        }
        let tri_count = indices.len() / 3;
        if face_ids.len() != tri_count {
            return Err(MeshKernelError::InvalidInput(format!("face_ids length {} must equal triangle count {}", face_ids.len(), tri_count)));
        }
        let mut groups: HashMap<u32, Vec<u32>> = HashMap::new();
        for (triangle_index, &face_id) in face_ids.iter().enumerate() {
            let base = triangle_index * 3;
            let group = groups.entry(face_id).or_default();
            group.extend_from_slice(&indices[base..base + 3]);
        }
        let mut faces: Vec<Vec<u32>> = Vec::new();
        for group_indices in groups.values() {
            let mut face_mesh = Self::from_indexed_triangles(positions, group_indices)?;
            let _ = face_mesh.merge_coplanar_faces()?;
            let (_, group_faces) = face_mesh.polygon_soup();
            faces.extend(group_faces);
        }
        let verts: Vec<[f32; 3]> = positions.as_chunks::<3>().0.to_vec();
        Self::from_faces(&verts, &faces)
    }

    /// Builds a halfedge mesh from CAD face wire loops that share a global vertex buffer.
    ///
    /// Each entry is `(outer, holes)`. Faces without holes become one n-gon; faces with holes are
    /// triangulated via keyhole bridging so openings stay empty (never filled by a single outer n-gon).
    pub fn from_face_loops(positions: &[[f32; 3]], face_loops: &[(Vec<u32>, Vec<Vec<u32>>)]) -> MeshResult<Self> {
        let mut faces: Vec<Vec<u32>> = Vec::new();
        for (outer, holes) in face_loops {
            if outer.len() < 3 {
                continue;
            }
            if holes.is_empty() {
                faces.push(outer.clone());
                continue;
            }
            for tri in triangulate_indexed_polygon_with_holes(positions, outer, holes) {
                faces.push(tri.to_vec());
            }
        }
        Self::from_faces(positions, &faces)
    }

    pub fn from_faces(positions: &[[f32; 3]], faces: &[Vec<u32>]) -> MeshResult<Self> {
        let mut mesh = Self::empty();
        for p in positions {
            mesh.vertices.push(MeshVertex { position: *p, normal: None, halfedge: None });
        }
        let mut edge_map: HashMap<(u32, u32), u32> = HashMap::new();
        for face_verts in faces {
            if face_verts.len() < 3 {
                return Err(MeshKernelError::DegenerateOperation);
            }
            let face_id = mesh.faces.len() as u32;
            let mut face_hes = Vec::new();
            for i in 0..face_verts.len() {
                let v0 = face_verts[i];
                let v1 = face_verts[(i + 1) % face_verts.len()];
                if v0 as usize >= mesh.vertices.len() || v1 as usize >= mesh.vertices.len() {
                    return Err(MeshKernelError::InvalidInput("face index out of range".into()));
                }
                let he_id = mesh.halfedges.len() as u32;
                mesh.halfedges.push(HalfEdge { vertex: v0, twin: None, next: 0, face: Some(face_id), uv: [0.0, 0.0] });
                face_hes.push(he_id);
                if let Some(&twin_id) = edge_map.get(&(v1, v0)) {
                    mesh.halfedges[he_id as usize].twin = Some(twin_id);
                    mesh.halfedges[twin_id as usize].twin = Some(he_id);
                }
                edge_map.insert((v0, v1), he_id);
            }
            for i in 0..face_hes.len() {
                let next = face_hes[(i + 1) % face_hes.len()];
                mesh.halfedges[face_hes[i] as usize].next = next;
            }
            let start_he = face_hes[0];
            mesh.faces.push(MeshFace { halfedge: start_he, smooth: false, flipped: false });
            let v0 = mesh.halfedges[start_he as usize].vertex;
            mesh.vertices[v0 as usize].halfedge = Some(start_he);
        }
        mesh.recompute_normals()?;
        Ok(mesh)
    }

    fn add_vertex(&mut self, pos: [f32; 3]) -> VertexId {
        let id = self.vertices.len() as u32;
        self.vertices.push(MeshVertex { position: pos, normal: None, halfedge: None });
        VertexId(id)
    }

    fn rebuild_from_polygon_soup(&mut self, positions: &[[f32; 3]], faces: &[Vec<u32>]) -> MeshResult<()> {
        *self = Self::from_faces(positions, faces)?;
        Ok(())
    }

    fn polygon_soup(&self) -> (Vec<[f32; 3]>, Vec<Vec<u32>>) {
        let positions: Vec<[f32; 3]> = self.vertices.iter().map(|v| v.position).collect();
        let mut faces = Vec::new();
        for fi in 0..self.faces.len() {
            if let Ok(verts) = self.face_vertex_ids(FaceId(fi as u32)) {
                faces.push(verts.into_iter().map(|v| v.0).collect());
            }
        }
        (positions, faces)
    }
}

//#endregion HalfedgeMesh

//#region Primitives

impl HalfedgeMesh {
    pub fn box_prim(width: f32, height: f32, depth: f32) -> MeshResult<Self> {
        let hw = width * 0.5;
        let hh = height * 0.5;
        let hd = depth * 0.5;
        let positions = [[-hw, -hh, -hd], [hw, -hh, -hd], [hw, hh, -hd], [-hw, hh, -hd], [-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]];
        let faces = vec![vec![3, 2, 1, 0], vec![5, 6, 7, 4], vec![1, 5, 4, 0], vec![2, 6, 5, 1], vec![3, 7, 6, 2], vec![0, 4, 7, 3]];
        Self::from_faces(&positions, &faces)
    }

    pub fn plane_prim(width: f32, depth: f32) -> MeshResult<Self> {
        let hw = width * 0.5;
        let hd = depth * 0.5;
        Self::from_faces(&[[-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]], &[vec![0, 1, 2, 3]])
    }

    pub fn cylinder_prim(radius: f32, height: f32, segments: u32) -> MeshResult<Self> {
        if !radius.is_finite() || !height.is_finite() || radius <= 0.0 || height <= 0.0 || !(3..=100_000).contains(&segments) {
            return Err(MeshKernelError::InvalidInput("positive dimensions and 3..100000 segments required".into()));
        }
        let mut positions = Vec::with_capacity(segments as usize * 2);
        for y in [-height * 0.5, height * 0.5] {
            for i in 0..segments {
                let angle = i as f32 / segments as f32 * std::f32::consts::TAU;
                positions.push([radius * angle.cos(), y, radius * angle.sin()]);
            }
        }
        let mut faces = vec![(0..segments).collect(), (segments..segments * 2).rev().collect()];
        for i in 0..segments { let next = (i + 1) % segments; faces.push(vec![i, i + segments, next + segments, next]); }
        Self::from_faces(&positions, &faces)
    }

    pub fn cone_prim(radius: f32, height: f32, segments: u32) -> MeshResult<Self> {
        if !radius.is_finite() || !height.is_finite() || radius <= 0.0 || height <= 0.0 || !(3..=100_000).contains(&segments) {
            return Err(MeshKernelError::InvalidInput("positive dimensions and 3..100000 segments required".into()));
        }
        let mut positions = Vec::with_capacity(segments as usize + 1);
        for i in 0..segments {
            let angle = i as f32 / segments as f32 * std::f32::consts::TAU;
            positions.push([radius * angle.cos(), -height * 0.5, radius * angle.sin()]);
        }
        positions.push([0.0, height * 0.5, 0.0]);
        let mut faces = vec![(0..segments).collect()];
        for i in 0..segments { faces.push(vec![i, segments, (i + 1) % segments]); }
        Self::from_faces(&positions, &faces)
    }

    pub fn ico_sphere_prim(radius: f32, subdivisions: u32) -> MeshResult<Self> {
        if !radius.is_finite() || radius <= 0.0 || subdivisions > 5 { return Err(MeshKernelError::InvalidInput("positive finite radius and 0..5 subdivisions required".into())); }
        let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
        let mut positions = vec![[-1.0, t, 0.0], [1.0, t, 0.0], [-1.0, -t, 0.0], [1.0, -t, 0.0], [0.0, -1.0, t], [0.0, 1.0, t], [0.0, -1.0, -t], [0.0, 1.0, -t], [t, 0.0, -1.0], [t, 0.0, 1.0], [-t, 0.0, -1.0], [-t, 0.0, 1.0]];
        for p in &mut positions {
            let v = Vec3(*p).normalize();
            *p = v.0;
        }
        let mut faces = vec![
            vec![0, 11, 5],
            vec![0, 5, 1],
            vec![0, 1, 7],
            vec![0, 7, 10],
            vec![0, 10, 11],
            vec![1, 5, 9],
            vec![5, 11, 4],
            vec![11, 10, 2],
            vec![10, 7, 6],
            vec![7, 1, 8],
            vec![3, 9, 4],
            vec![3, 4, 2],
            vec![3, 2, 6],
            vec![3, 6, 8],
            vec![3, 8, 9],
            vec![4, 9, 5],
            vec![2, 4, 11],
            vec![6, 2, 10],
            vec![8, 6, 7],
            vec![9, 8, 1],
        ];
        for _ in 0..subdivisions {
            let mut new_faces = Vec::new();
            let mut midpoint_cache: HashMap<(u32, u32), u32> = HashMap::new();
            for face in &faces {
                let mut mids = Vec::new();
                for i in 0..face.len() {
                    let a = face[i];
                    let b = face[(i + 1) % face.len()];
                    let key = if a < b { (a, b) } else { (b, a) };
                    let mid = if let Some(&existing) = midpoint_cache.get(&key) {
                        existing
                    } else {
                        let pa = Vec3(positions[a as usize]);
                        let pb = Vec3(positions[b as usize]);
                        let m = pa.lerp(pb, 0.5).normalize();
                        let id = positions.len() as u32;
                        positions.push(m.0);
                        midpoint_cache.insert(key, id);
                        id
                    };
                    mids.push(mid);
                }
                for i in 0..face.len() {
                    let v = face[i];
                    let m0 = mids[i];
                    let m1 = mids[(i + face.len() - 1) % face.len()];
                    new_faces.push(vec![v, m0, m1]);
                }
                new_faces.push(mids);
            }
            faces = new_faces;
        }
        for point in &mut positions { *point = point.map(|coordinate| coordinate * radius); }
        Self::from_faces(&positions, &faces)
    }
}

//#endregion Primitives

//#region Transform

impl HalfedgeMesh {
    pub fn translate(&mut self, delta: Vec3) -> MeshResult<()> {
        for v in &mut self.vertices {
            v.position = Vec3(v.position).add(delta).0;
        }
        Ok(())
    }

    pub fn rotate(&mut self, axis: Vec3, angle_rad: f32) -> MeshResult<()> {
        let ax = axis.normalize();
        let (x, y, z) = (ax.x(), ax.y(), ax.z());
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        let t = 1.0 - c;
        for v in &mut self.vertices {
            let p = Vec3(v.position);
            let (px, py, pz) = (p.x(), p.y(), p.z());
            let rx = (t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz;
            let ry = (t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz;
            let rz = (t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz;
            v.position = [rx, ry, rz];
        }
        self.recompute_normals()
    }

    pub fn scale(&mut self, factor: Vec3) -> MeshResult<()> {
        let (fx, fy, fz) = (factor.x(), factor.y(), factor.z());
        for v in &mut self.vertices {
            v.position = [v.position[0] * fx, v.position[1] * fy, v.position[2] * fz];
        }
        self.recompute_normals()
    }

    pub fn move_vertices(&mut self, verts: &[VertexId], delta: Vec3) -> MeshResult<()> {
        if delta.0.iter().any(|value| !value.is_finite()) { return Err(MeshKernelError::DegenerateOperation); }
        self.map_vertices(verts, |point| std::array::from_fn(|axis| point[axis] + delta.0[axis] as f64))
    }

    pub fn rotate_vertices(&mut self, verts: &[VertexId], axis: Vec3, angle_rad: f32, pivot: Vec3) -> MeshResult<()> {
        if axis.0.iter().chain(pivot.0.iter()).any(|value| !value.is_finite()) || !angle_rad.is_finite() { return Err(MeshKernelError::DegenerateOperation); }
        let axis = axis.0.map(f64::from);
        let length = axis[0].hypot(axis[1]).hypot(axis[2]);
        if length == 0.0 { return Err(MeshKernelError::DegenerateOperation); }
        let [x, y, z] = axis.map(|value| value / length);
        let pivot = pivot.0.map(f64::from);
        let c = (angle_rad as f64).cos();
        let s = (angle_rad as f64).sin();
        let t = 1.0 - c;
        self.map_vertices(verts, |point| {
            let [px, py, pz] = std::array::from_fn::<_, 3, _>(|axis| point[axis] - pivot[axis]);
            let rx = (t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz;
            let ry = (t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz;
            let rz = (t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz;
            [pivot[0] + rx, pivot[1] + ry, pivot[2] + rz]
        })
    }

    pub fn scale_vertices(&mut self, verts: &[VertexId], factor: Vec3, pivot: Vec3) -> MeshResult<()> {
        if factor.0.iter().chain(pivot.0.iter()).any(|value| !value.is_finite()) || factor.0.contains(&0.0) { return Err(MeshKernelError::DegenerateOperation); }
        self.map_vertices(verts, |point| std::array::from_fn(|axis| pivot.0[axis] as f64 + (point[axis] - pivot.0[axis] as f64) * factor.0[axis] as f64))
    }

    fn map_vertices(&mut self, verts: &[VertexId], transform: impl Fn([f64; 3]) -> [f64; 3]) -> MeshResult<()> {
        if verts.is_empty() { return Err(MeshKernelError::EmptySelection); }
        let mut ids = verts.iter().map(|vertex| vertex.0 as usize).collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();
        let positions = ids.iter().map(|&id| {
            let point = self.vertices.get(id).ok_or(MeshKernelError::InvalidHandle)?.position;
            let result = transform(point.map(f64::from)).map(|value| value as f32);
            if result.iter().any(|value| !value.is_finite()) { return Err(MeshKernelError::DegenerateOperation); }
            Ok(result)
        }).collect::<MeshResult<Vec<_>>>()?;
        for (id, position) in ids.into_iter().zip(positions) { self.vertices[id].position = position; }
        self.recompute_normals()
    }

    /// 🌊 Moves the selection fully and nearby vertices with linear radial falloff.
    pub fn move_vertices_proportional(&mut self, verts: &[VertexId], delta: Vec3, pivot: Vec3, radius: f32) -> MeshResult<()> {
        if verts.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if !radius.is_finite() || radius <= 0.0 || delta.0.iter().chain(pivot.0.iter()).any(|value| !value.is_finite()) { return Err(MeshKernelError::DegenerateOperation); }
        for &vertex in verts { self.vertex_position(vertex)?; }
        let selected = verts.iter().copied().collect::<HashSet<_>>();
        let all = (0..self.vertex_count()).map(|id| VertexId(id as u32)).collect::<Vec<_>>();
        let weights = all.iter().map(|id| {
            if selected.contains(id) { 1.0 } else { (1.0 - self.vertex_position(*id).unwrap().sub(pivot).length() / radius).max(0.0) }
        }).collect::<Vec<_>>();
        let positions = self.vertices.iter().zip(weights).map(|(vertex, weight)| Vec3(vertex.position).add(delta.scale(weight)).0).collect::<Vec<_>>();
        if positions.iter().flatten().any(|value| !value.is_finite()) { return Err(MeshKernelError::DegenerateOperation); }
        for (vertex, position) in self.vertices.iter_mut().zip(positions) { vertex.position = position; }
        self.recompute_normals()
    }

    pub fn snap_vertices_to_grid(&mut self, verts: &[VertexId], grid: f32) -> MeshResult<()> {
        if !grid.is_finite() || grid <= 0.0 { return Err(MeshKernelError::InvalidInput("grid must be finite and positive".into())); }
        self.map_vertices(verts, |point| point.map(|coordinate| (coordinate / grid as f64).round() * grid as f64))
    }

}

//#endregion Transform

//#region Edit

impl HalfedgeMesh {
    pub fn extrude_faces(&mut self, faces: &[FaceId], distance: f32) -> MeshResult<()> {
        if faces.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if !distance.is_finite() || distance == 0.0 { return Err(MeshKernelError::DegenerateOperation); }
        let selected: HashSet<u32> = faces.iter().map(|face| face.0).collect();
        if selected.len() != faces.len() { return Err(MeshKernelError::InvalidInput("duplicate selected face".into())); }
        let mut normals = HashMap::<u32, Vec3>::new();
        let mut boundary = HashMap::<(u32, u32), (u32, u32, usize)>::new();
        for &face in faces {
            let vertices = self.face_vertex_ids(face)?;
            let normal = self.face_normal(face)?;
            for i in 0..vertices.len() {
                let a = vertices[i].0;
                let b = vertices[(i + 1) % vertices.len()].0;
                normals.entry(a).and_modify(|sum| *sum = sum.add(normal)).or_insert(normal);
                boundary.entry((a.min(b), a.max(b))).and_modify(|edge| edge.2 += 1).or_insert((a, b, 1));
            }
        }
        let (mut positions, original) = self.polygon_soup();
        let mut displaced = HashMap::new();
        let mut ids: Vec<u32> = normals.keys().copied().collect();
        ids.sort_unstable();
        for id in ids {
            let normal = normals[&id].normalize();
            if normal.length() < 1e-6 { return Err(MeshKernelError::DegenerateOperation); }
            displaced.insert(id, positions.len() as u32);
            positions.push(Vec3(positions[id as usize]).add(normal.scale(distance)).0);
        }
        let mut result = Vec::new();
        for (index, face) in original.iter().enumerate() {
            result.push(if selected.contains(&(index as u32)) { face.iter().map(|id| displaced[id]).collect() } else { face.clone() });
        }
        let mut edges: Vec<_> = boundary.into_values().filter(|edge| edge.2 == 1).collect();
        edges.sort_unstable();
        for (a, b, _) in edges { result.push(vec![a, b, displaced[&b], displaced[&a]]); }
        self.rebuild_from_polygon_soup(&positions, &result)
    }

    pub fn inset_faces(&mut self, faces: &[FaceId], amount: f32) -> MeshResult<()> {
        if faces.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if !amount.is_finite() || amount <= 0.0 { return Err(MeshKernelError::DegenerateOperation); }
        let selected: HashSet<u32> = faces.iter().map(|face| face.0).collect();
        if selected.len() != faces.len() { return Err(MeshKernelError::InvalidInput("duplicate selected face".into())); }
        let (mut positions, mut result) = self.polygon_soup();
        for &face in faces {
            let vertices = self.face_vertex_ids(face)?;
            let normal = self.face_normal(face)?;
            let mut inner = Vec::new();
            for i in 0..vertices.len() {
                let previous = self.vertex_position(vertices[(i + vertices.len() - 1) % vertices.len()])?;
                let current = self.vertex_position(vertices[i])?;
                let next = self.vertex_position(vertices[(i + 1) % vertices.len()])?;
                let a = normal.cross(current.sub(previous).normalize());
                let b = normal.cross(next.sub(current).normalize());
                let denominator = 1.0 + a.dot(b);
                if denominator <= 1e-6 { return Err(MeshKernelError::DegenerateOperation); }
                let offset = a.add(b).scale(amount / denominator);
                if offset.length() >= current.sub(previous).length().min(next.sub(current).length()) * 0.5 {
                    return Err(MeshKernelError::InvalidInput("inset exceeds local edge clearance".into()));
                }
                inner.push(positions.len() as u32);
                positions.push(current.add(offset).0);
            }
            result[face.0 as usize] = inner.clone();
            for i in 0..vertices.len() {
                let next = (i + 1) % vertices.len();
                result.push(vec![vertices[i].0, vertices[next].0, inner[next], inner[i]]);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &result)
    }

    /// 🪚 Bevels convex closed meshes with a segmented circular profile and mitered corners.
    pub fn bevel_edges(&mut self, edges: &[EdgeId], amount: f32, segments: u32) -> MeshResult<()> {
        self.bevel_edges_with_progress(edges, amount, segments, |_| true)
    }

    /// ⏳ Reports progress and cancels atomically when the callback returns false.
    pub fn bevel_edges_with_progress(&mut self, edges: &[EdgeId], amount: f32, segments: u32, progress: impl FnMut(f32) -> bool) -> MeshResult<()> {
        *self = self.bevel_job(edges, amount, segments)?.finish_with_progress(progress)?;
        Ok(())
    }

    /// ✂️ Splits connected quad strips, sharing cut vertices across adjacent faces.
    pub fn loop_cut(&mut self, edges: &[EdgeId], cuts: u32) -> MeshResult<()> {
        if edges.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if !(1..=256).contains(&cuts) { return Err(MeshKernelError::InvalidInput("cuts must be in 1..=256".into())); }
        let key = |a: u32, b: u32| (a.min(b), a.max(b));
        let (mut positions, faces) = self.polygon_soup();
        let mut incidence = HashMap::<(u32, u32), Vec<(usize, usize)>>::new();
        for (fi, face) in faces.iter().enumerate() {
            for i in 0..face.len() { incidence.entry(key(face[i], face[(i + 1) % face.len()])).or_default().push((fi, i)); }
        }
        let mut marked = HashSet::new();
        let mut pending = Vec::new();
        for &edge in edges {
            let (a, b) = self.edge_endpoints(edge)?;
            let edge = key(a.0, b.0);
            if !incidence[&edge].iter().any(|&(fi, _)| faces[fi].len() == 4) {
                return Err(MeshKernelError::InvalidInput("selected edges must touch a quad".into()));
            }
            if marked.insert(edge) { pending.push(edge); }
        }
        let mut cursor = 0;
        while cursor < pending.len() {
            let edge = pending[cursor];
            cursor += 1;
            if incidence[&edge].len() > 2 { return Err(MeshKernelError::InvalidInput("loop cut crosses a nonmanifold edge".into())); }
            for &(fi, i) in &incidence[&edge] {
                let face = &faces[fi];
                if face.len() == 4 {
                    let opposite = key(face[(i + 2) % 4], face[(i + 3) % 4]);
                    if marked.insert(opposite) { pending.push(opposite); }
                }
            }
        }
        let dimensions = |face: &[u32]| {
            let nx = if marked.contains(&key(face[0], face[1])) { cuts as usize + 1 } else { 1 };
            let ny = if marked.contains(&key(face[1], face[2])) { cuts as usize + 1 } else { 1 };
            (nx, ny)
        };
        let mut vertices = positions.len() as u64 + marked.len() as u64 * cuts as u64;
        let mut face_count = 0u64;
        let mut corner_count = 0u64;
        for face in &faces {
            if face.len() == 4 {
                let (nx, ny) = dimensions(face);
                vertices += ((nx - 1) * (ny - 1)) as u64;
                face_count += (nx * ny) as u64;
                corner_count += (nx * ny * 4) as u64;
            } else {
                face_count += 1;
                corner_count += face.len() as u64 + (0..face.len()).filter(|&i| marked.contains(&key(face[i], face[(i + 1) % face.len()]))).count() as u64 * cuts as u64;
            }
        }
        if vertices > 100_000 || face_count > 100_000 || corner_count > 600_000 {
            return Err(MeshKernelError::InvalidInput("loop cut exceeds mesh capacity".into()));
        }
        let mut cut_vertices = HashMap::new();
        pending.sort_unstable();
        for (a, b) in pending {
            let mut ids = Vec::new();
            let mut previous = positions[a as usize];
            for step in 1..=cuts {
                let t = step as f64 / (cuts + 1) as f64;
                let point = [0, 1, 2].map(|axis| (positions[a as usize][axis] as f64 * (1.0 - t) + positions[b as usize][axis] as f64 * t) as f32);
                if point.iter().any(|coordinate| !coordinate.is_finite()) || point == previous || point == positions[b as usize] { return Err(MeshKernelError::DegenerateOperation); }
                ids.push(positions.len() as u32);
                positions.push(point);
                previous = point;
            }
            cut_vertices.insert((a, b), ids);
        }
        let on_edge = |a: u32, b: u32, step: usize, segments: usize| {
            if step == 0 { a } else if step == segments { b } else { cut_vertices[&key(a, b)][if a < b { step - 1 } else { segments - step - 1 }] }
        };
        let mut result = Vec::with_capacity(face_count as usize);
        for face in &faces {
            if face.len() == 4 {
                let (nx, ny) = dimensions(face);
                let mut grid = vec![0; (nx + 1) * (ny + 1)];
                for y in 0..=ny {
                    for x in 0..=nx {
                        grid[y * (nx + 1) + x] = if y == 0 { on_edge(face[0], face[1], x, nx) }
                        else if y == ny { on_edge(face[3], face[2], x, nx) }
                        else if x == 0 { on_edge(face[0], face[3], y, ny) }
                        else if x == nx { on_edge(face[1], face[2], y, ny) }
                        else {
                            let (u, v) = (x as f64 / nx as f64, y as f64 / ny as f64);
                            let weights = [(1.0 - u) * (1.0 - v), u * (1.0 - v), u * v, (1.0 - u) * v];
                            let point = [0, 1, 2].map(|axis| face.iter().zip(weights).map(|(&id, weight)| positions[id as usize][axis] as f64 * weight).sum::<f64>() as f32);
                            if point.iter().any(|coordinate| !coordinate.is_finite()) { return Err(MeshKernelError::DegenerateOperation); }
                            let id = positions.len() as u32;
                            positions.push(point);
                            id
                        };
                    }
                }
                for y in 0..ny {
                    for x in 0..nx {
                        let i = y * (nx + 1) + x;
                        result.push(vec![grid[i], grid[i + 1], grid[i + nx + 2], grid[i + nx + 1]]);
                    }
                }
            } else {
                let mut boundary = Vec::new();
                for i in 0..face.len() {
                    let (a, b) = (face[i], face[(i + 1) % face.len()]);
                    boundary.push(a);
                    if marked.contains(&key(a, b)) { boundary.extend((1..=cuts as usize).map(|step| on_edge(a, b, step, cuts as usize + 1))); }
                }
                result.push(boundary);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &result)
    }

    /// ✂️ Splits a face by a projected line and reuses boundary intersections in neighboring polygons.
    pub fn knife_cut(&mut self, face: FaceId, cut_a: Vec3, cut_b: Vec3) -> MeshResult<()> {
        let ids = self.face_vertex_ids(face)?;
        if ids.len() < 3 { return Err(MeshKernelError::InvalidInput("knife face must have at least three vertices".into())); }
        if cut_a.0.iter().chain(&cut_b.0).any(|value| !value.is_finite()) { return Err(MeshKernelError::InvalidInput("knife points must be finite".into())); }
        let (mut positions, faces) = self.polygon_soup();
        let polygon = &faces[face.0 as usize];
        let points = ids.iter().map(|id| Vec3(self.vertices[id.0 as usize].position)).collect::<Vec<_>>();
        let tuple = |p: [f32; 3]| (p[0] as f64, p[1] as f64, p[2] as f64);
        let normal = tuple(newell_normal(&points).0);
        let direction = sub3(tuple(cut_b.0), tuple(cut_a.0));
        let plane = cross3(direction, normal);
        let length = length3(plane);
        if length == 0.0 || length <= length3(direction) * 1e-7 { return Err(MeshKernelError::InvalidInput("knife direction must project onto a nondegenerate face".into())); }
        let plane = (plane.0 / length, plane.1 / length, plane.2 / length);
        let origin = tuple(points[0].0);
        let scale = points.iter().map(|p| length3(sub3(tuple(p.0), origin))).fold(0.0f64, f64::max);
        let tolerance = scale * 1e-7;
        let distances = positions.iter().map(|p| {
            let d = dot3(sub3(tuple(*p), tuple(cut_a.0)), plane);
            if d.abs() <= tolerance { 0.0 } else { d }
        }).collect::<Vec<_>>();
        if !polygon.iter().any(|&id| distances[id as usize] > 0.0) || !polygon.iter().any(|&id| distances[id as usize] < 0.0) { return Err(MeshKernelError::InvalidInput("knife line must cross the face interior".into())); }
        let planar = points.iter().all(|p| dot3(sub3(tuple(p.0), origin), normal).abs() <= tolerance);
        let convex = (0..points.len()).all(|i| {
            let a = tuple(points[i].0);
            let b = tuple(points[(i + 1) % points.len()].0);
            let c = tuple(points[(i + 2) % points.len()].0);
            dot3(cross3(sub3(b, a), sub3(c, b)), normal) >= -scale * tolerance
        });
        let pieces = if planar && convex { vec![polygon.clone()] } else {
            triangulate_polygon(&points).into_iter().map(|triangle| triangle.into_iter().map(|id| polygon[id]).collect()).collect()
        };
        let key = |a: u32, b: u32| (a.min(b), a.max(b));
        let mut intersections = HashMap::<(u32, u32), u32>::new();
        let mut split = Vec::new();
        for piece in pieces {
            let positive = piece.iter().any(|&id| distances[id as usize] > 0.0);
            let negative = piece.iter().any(|&id| distances[id as usize] < 0.0);
            if !positive || !negative { split.push(piece); continue; }
            for sign in [1.0, -1.0] {
                let mut clipped = Vec::new();
                for i in 0..piece.len() {
                    let (a, b) = (piece[i], piece[(i + 1) % piece.len()]);
                    let (da, db) = (distances[a as usize], distances[b as usize]);
                    if da * sign >= 0.0 { clipped.push(a); }
                    if (da > 0.0 && db < 0.0) || (da < 0.0 && db > 0.0) {
                        let edge = key(a, b);
                        let id = if let Some(&id) = intersections.get(&edge) { id } else {
                            let (a, b) = (edge.0 as usize, edge.1 as usize);
                            let t = distances[a] / (distances[a] - distances[b]);
                            let point = [0, 1, 2].map(|axis| ((1.0 - t) * positions[a][axis] as f64 + t * positions[b][axis] as f64) as f32);
                            if point.iter().any(|value| !value.is_finite()) || point == positions[a] || point == positions[b] { return Err(MeshKernelError::InvalidInput("knife intersection collapses at this precision".into())); }
                            if positions.len() >= 100_000 { return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into())); }
                            let id = positions.len() as u32;
                            positions.push(point); intersections.insert(edge, id); id
                        };
                        clipped.push(id);
                    }
                }
                clipped.dedup();
                if clipped.first() == clipped.last() { clipped.pop(); }
                if clipped.len() < 3 { return Err(MeshKernelError::InvalidInput("knife cut produces a degenerate piece".into())); }
                split.push(clipped);
            }
        }
        let mut result = Vec::new();
        for (index, polygon) in faces.iter().enumerate() {
            if index == face.0 as usize { result.append(&mut split); continue; }
            let mut boundary = Vec::new();
            for i in 0..polygon.len() {
                let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
                boundary.push(a);
                if let Some(&id) = intersections.get(&key(a, b)) { boundary.push(id); }
            }
            result.push(boundary);
        }
        if result.len() > 100_000 || result.iter().map(Vec::len).sum::<usize>() > 600_000 { return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into())); }
        self.rebuild_from_polygon_soup(&positions, &result)
    }

    pub fn merge_vertices(&mut self, verts: &[VertexId], mode: WeldMode, threshold: f32) -> MeshResult<()> {
        let mut selected = Vec::new();
        for &vertex in verts { self.vertex_position(vertex)?; if !selected.contains(&vertex.0) { selected.push(vertex.0); } }
        if selected.len() < 2 { return Err(MeshKernelError::EmptySelection); }
        if mode == WeldMode::ByDistance && (!threshold.is_finite() || threshold < 0.0) { return Err(MeshKernelError::DegenerateOperation); }
        if selected.len().saturating_mul(selected.len()) > 4_000_000 { return Err(MeshKernelError::InvalidInput("merge work budget exceeded".into())); }
        let (mut positions, faces) = self.polygon_soup();
        let mut roots = (0..selected.len()).collect::<Vec<_>>();
        if mode == WeldMode::ByDistance {
            for i in 0..selected.len() {
                for j in 0..i {
                    if Vec3(positions[selected[i] as usize]).sub(Vec3(positions[selected[j] as usize])).length() <= threshold {
                        let mut a = i; while roots[a] != a { a = roots[a]; }
                        let mut b = j; while roots[b] != b { b = roots[b]; }
                        roots[a.max(b)] = a.min(b);
                    }
                }
            }
        } else { roots.fill(0); }
        let mut remap = (0..positions.len() as u32).collect::<Vec<_>>();
        for i in 0..selected.len() {
            let mut root = i; while roots[root] != root { root = roots[root]; }
            remap[selected[i] as usize] = selected[root];
        }
        if mode == WeldMode::Center {
            let mut center = [0.0f64; 3];
            for &id in &selected { for (axis, coordinate) in center.iter_mut().enumerate() { *coordinate += positions[id as usize][axis] as f64 / selected.len() as f64; } }
            positions[selected[0] as usize] = center.map(|value| value as f32);
        }
        let mut cleaned = Vec::new();
        for face in faces {
            let mut face = face.into_iter().map(|id| remap[id as usize]).collect::<Vec<_>>();
            face.dedup();
            if face.first() == face.last() { face.pop(); }
            if face.len() < 3 { continue; }
            if face.iter().copied().collect::<HashSet<_>>().len() != face.len() { return Err(MeshKernelError::NonManifold); }
            cleaned.push(face);
        }
        let mut result = Self::from_faces(&positions, &cleaned)?;
        result.drop_unreferenced_vertices()?;
        *self = result;
        Ok(())
    }

    pub fn dissolve_edges(&mut self, edges: &[EdgeId]) -> MeshResult<()> {
        if edges.is_empty() { return Err(MeshKernelError::EmptySelection); }
        let selected = edges.iter().map(|edge| self.edge_endpoints(*edge).map(|(a,b)| (a.0,b.0))).collect::<MeshResult<Vec<_>>>()?;
        let (positions, mut faces) = self.polygon_soup();
        let mut done = HashSet::new();
        for (a,b) in selected {
            if !done.insert((a.min(b), a.max(b))) { continue; }
            let ai = faces.iter().position(|face| find_edge_position(face,a,b).is_some()).ok_or(MeshKernelError::DegenerateOperation)?;
            let bi = faces.iter().position(|face| find_edge_position(face,b,a).is_some()).ok_or(MeshKernelError::DegenerateOperation)?;
            let merged = merge_face_loops(&faces[ai], &faces[bi]).ok_or(MeshKernelError::NonManifold)?;
            if merged.iter().copied().collect::<HashSet<_>>().len() != merged.len() { return Err(MeshKernelError::NonManifold); }
            faces[ai.min(bi)] = merged;
            faces.remove(ai.max(bi));
        }
        let mut result = Self::from_faces(&positions, &faces)?;
        result.drop_unreferenced_vertices()?;
        *self = result;
        Ok(())
    }

    /// 🧵 Dissolves planar vertex stars into their oriented outer boundary.
    pub fn dissolve_vertices(&mut self, verts: &[VertexId]) -> MeshResult<()> {
        if verts.is_empty() { return Err(MeshKernelError::EmptySelection); }
        for &vertex in verts { self.vertex_position(vertex)?; }
        let (positions, mut faces) = self.polygon_soup();
        let mut selected = verts.iter().map(|vertex| vertex.0).collect::<Vec<_>>();
        selected.sort_unstable(); selected.dedup();
        for vertex in selected {
            let incident = faces.iter().enumerate().filter_map(|(id, face)| face.contains(&vertex).then_some(id)).collect::<Vec<_>>();
            if incident.is_empty() { return Err(MeshKernelError::DegenerateOperation); }
            let points = faces[incident[0]].iter().map(|&id| Vec3(positions[id as usize])).collect::<Vec<_>>();
            let normal = newell_normal(&points).normalize();
            let origin = points[0];
            let extent = points.iter().map(|point| point.sub(origin).length()).fold(0.0f32, f32::max);
            if normal.length() == 0.0 || incident.iter().flat_map(|&id| faces[id].iter()).any(|&id| Vec3(positions[id as usize]).sub(origin).dot(normal).abs() > extent * 1e-5) { return Err(MeshKernelError::InvalidInput("vertex dissolution requires a planar star".into())); }
            let mut boundary = HashMap::new();
            for &id in &incident {
                let face = &faces[id];
                for i in 0..face.len() {
                    let (a,b) = (face[i], face[(i+1)%face.len()]);
                    if boundary.remove(&(b,a)).is_none() && boundary.insert((a,b),()).is_some() { return Err(MeshKernelError::NonManifold); }
                }
            }
            let mut next = HashMap::new();
            for &(a,b) in boundary.keys() { if next.insert(a,b).is_some() { return Err(MeshKernelError::NonManifold); } }
            let start = *next.keys().min().ok_or(MeshKernelError::DegenerateOperation)?;
            let mut merged = Vec::new();
            let mut cursor = start;
            loop {
                if cursor != vertex { merged.push(cursor); }
                cursor = next.remove(&cursor).ok_or(MeshKernelError::NonManifold)?;
                if cursor == start { break; }
                if merged.len() > boundary.len() { return Err(MeshKernelError::NonManifold); }
            }
            if !next.is_empty() || merged.len() < 3 { return Err(MeshKernelError::NonManifold); }
            let incident = incident.into_iter().collect::<HashSet<_>>();
            faces = faces.into_iter().enumerate().filter_map(|(id,face)| (!incident.contains(&id)).then_some(face)).collect();
            faces.push(merged);
        }
        let mut result = Self::from_faces(&positions, &faces)?;
        result.drop_unreferenced_vertices()?;
        *self = result;
        Ok(())
    }

    pub fn subdivide_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        if faces.is_empty() { return Err(MeshKernelError::EmptySelection); }
        let selected: HashSet<u32> = faces.iter().map(|face| face.0).collect();
        if selected.len() != faces.len() { return Err(MeshKernelError::InvalidInput("duplicate face selection".into())); }
        for &face in faces { self.face_vertex_ids(face)?; }
        let (mut positions, original) = self.polygon_soup();
        let mut result = Vec::new();
        for (index, face) in original.iter().enumerate() {
            if !selected.contains(&(index as u32)) { result.push(face.clone()); continue; }
            let points: Vec<_> = face.iter().map(|id| Vec3(positions[*id as usize])).collect();
            let normal = newell_normal(&points);
            let convex = (0..points.len()).all(|i| points[(i + 1) % points.len()].sub(points[i]).cross(points[(i + 2) % points.len()].sub(points[(i + 1) % points.len()])).dot(normal) > 0.0);
            let polygons = if convex { vec![face.clone()] } else { triangulate_polygon(&points).into_iter().map(|triangle| triangle.into_iter().map(|i| face[i]).collect()).collect() };
            for polygon in polygons {
                let center = polygon.iter().fold(Vec3::ZERO, |sum, id| sum.add(Vec3(positions[*id as usize]))).scale(1.0 / polygon.len() as f32);
                let center_id = positions.len() as u32;
                positions.push(center.0);
                for i in 0..polygon.len() { result.push(vec![polygon[i], polygon[(i + 1) % polygon.len()], center_id]); }
            }
        }
        self.rebuild_from_polygon_soup(&positions, &result)
    }

    pub fn triangulate(&mut self) -> MeshResult<()> {
        let (positions, face_list) = self.polygon_soup();
        let mut new_faces = Vec::new();
        for face in face_list {
            if face.len() <= 3 {
                new_faces.push(face);
                continue;
            }
            let face_positions: Vec<Vec3> = face.iter().map(|&vi| Vec3(positions[vi as usize])).collect();
            for tri in triangulate_polygon(&face_positions) {
                new_faces.push(vec![face[tri[0]], face[tri[1]], face[tri[2]]]);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &new_faces)
    }

    /// Merges every pair of adjacent faces whose normals are parallel and whose union is planar (within kernel
    /// tolerances) into a single n-gon, then drops resulting straight-pass-through (collinear) vertices. Returns
    /// the number of merges performed.
    pub fn merge_coplanar_faces(&mut self) -> MeshResult<usize> {
        let (positions, mut face_list) = self.polygon_soup();
        let mut merge_count = 0usize;
        loop {
            let mut merged_this_round = false;
            'search: for ai in 0..face_list.len() {
                let a_len = face_list[ai].len();
                for i in 0..a_len {
                    let u = face_list[ai][i];
                    let v = face_list[ai][(i + 1) % a_len];
                    let mut bi_found = None;
                    for (bi, f) in face_list.iter().enumerate() {
                        if bi != ai && find_edge_position(f, v, u).is_some() {
                            bi_found = Some(bi);
                            break;
                        }
                    }
                    let Some(bi) = bi_found else { continue };
                    if !faces_coplanar(&positions, &face_list[ai], &face_list[bi]) {
                        continue;
                    }
                    if let Some(merged) = merge_face_loops(&face_list[ai], &face_list[bi]) {
                        let (keep, remove) = if ai < bi { (ai, bi) } else { (bi, ai) };
                        face_list[keep] = merged;
                        face_list.remove(remove);
                        merge_count += 1;
                        merged_this_round = true;
                        break 'search;
                    }
                }
            }
            if !merged_this_round {
                break;
            }
        }
        let cleaned = collinear_cleanup(&positions, &face_list);
        self.rebuild_from_polygon_soup(&positions, &cleaned)?;
        Ok(merge_count)
    }

    /// Unifies every group of vertices at (nearly) the same position — as commonly produced by importers
    /// that tessellate adjacent source faces independently, leaving duplicate, non-shared vertex ids along
    /// shared boundaries — into a single vertex id per position, so the halfedge topology (twins, boundary
    /// detection) reflects the true geometric connectivity. Returns the number of vertices removed.
    pub fn weld_coincident_vertices(&mut self, precision: f32) -> MeshResult<usize> {
        let (positions, face_list) = self.polygon_soup();
        let scale = 1.0 / precision.max(1e-9);
        let mut groups: HashMap<(i64, i64, i64), u32> = HashMap::new();
        let mut compacted_positions: Vec<[f32; 3]> = Vec::new();
        let mut remap: Vec<u32> = Vec::with_capacity(positions.len());
        for p in &positions {
            let key = ((p[0] as f64 * scale as f64).round() as i64, (p[1] as f64 * scale as f64).round() as i64, (p[2] as f64 * scale as f64).round() as i64);
            let canonical = *groups.entry(key).or_insert_with(|| {
                compacted_positions.push(*p);
                (compacted_positions.len() - 1) as u32
            });
            remap.push(canonical);
        }
        let removed = positions.len() - compacted_positions.len();
        if removed == 0 {
            return Ok(0);
        }
        let new_faces: Vec<Vec<u32>> = face_list
            .into_iter()
            .map(|f| f.into_iter().map(|vi| remap[vi as usize]).collect::<Vec<u32>>())
            .filter(|f| {
                let mut unique = f.clone();
                unique.sort();
                unique.dedup();
                unique.len() >= 3
            })
            .collect();
        self.rebuild_from_polygon_soup(&compacted_positions, &new_faces)?;
        Ok(removed)
    }

    /// Flips faces so every undirected edge is traversed in opposite directions by its two incident faces.
    /// CAD imports often leave inconsistently oriented face wires; without this pass, halfedge twins are
    /// missing even though the undirected mesh is closed. Returns the number of faces flipped.
    pub fn orient_faces_consistently(&mut self) -> MeshResult<usize> {
        let (positions, mut face_list) = self.polygon_soup();
        if face_list.is_empty() {
            return Ok(0);
        }
        let mut edge_faces: HashMap<(u32, u32), Vec<(usize, bool)>> = HashMap::new();
        for (fi, face) in face_list.iter().enumerate() {
            let n = face.len();
            for i in 0..n {
                let a = face[i];
                let b = face[(i + 1) % n];
                let key = if a < b { (a, b) } else { (b, a) };
                let forward = a < b;
                edge_faces.entry(key).or_default().push((fi, forward));
            }
        }
        let mut adjacency: Vec<Vec<(usize, bool)>> = vec![Vec::new(); face_list.len()];
        for owners in edge_faces.values() {
            if owners.len() != 2 {
                continue;
            }
            let (a, a_forward) = owners[0];
            let (b, b_forward) = owners[1];
            // Same directed sense on a shared undirected edge ⇒ neighbor needs a relative flip.
            let needs_relative_flip = a_forward == b_forward;
            adjacency[a].push((b, needs_relative_flip));
            adjacency[b].push((a, needs_relative_flip));
        }
        let mut oriented = vec![false; face_list.len()];
        let mut flip = vec![false; face_list.len()];
        let mut flips = 0usize;
        for start in 0..face_list.len() {
            if oriented[start] {
                continue;
            }
            let mut stack = vec![start];
            oriented[start] = true;
            while let Some(fi) = stack.pop() {
                for &(neighbor, needs_relative_flip) in &adjacency[fi] {
                    let neighbor_flip = flip[fi] ^ needs_relative_flip;
                    if !oriented[neighbor] {
                        oriented[neighbor] = true;
                        flip[neighbor] = neighbor_flip;
                        if neighbor_flip {
                            flips += 1;
                        }
                        stack.push(neighbor);
                    }
                }
            }
        }
        for (fi, face) in face_list.iter_mut().enumerate() {
            if flip[fi] {
                face.reverse();
            }
        }
        self.rebuild_from_polygon_soup(&positions, &face_list)?;
        Ok(flips)
    }

    /// Finds every closed boundary loop (a chain of edges with no opposite face on the other side) in the
    /// current halfedge topology and caps each with a new n-gon face, so the mesh becomes watertight. Call
    /// `weld_coincident_vertices` first if the mesh may contain importer-duplicated boundary vertices, or
    /// this will also "cap" seams that are actually already shared with a differently-indexed neighbor.
    /// Returns the number of holes filled.
    pub fn fill_holes(&mut self) -> MeshResult<usize> {
        // Proper half-edge boundary walk: from a boundary half-edge (twin=None), the next boundary
        // half-edge of the SAME hole loop is found by rotating around its destination vertex via
        // next/twin jumps until another twin-less half-edge is hit. This correctly disambiguates separate
        // holes that happen to share a corner vertex (a vertex-only "next" map cannot).
        let he_count = self.halfedges.len() as u32;
        let mut visited: HashSet<u32> = HashSet::new();
        let mut new_loops: Vec<Vec<u32>> = Vec::new();
        for start in 0..he_count {
            if self.halfedges[start as usize].twin.is_some() || visited.contains(&start) {
                continue;
            }
            let mut loop_he_ids = vec![start];
            visited.insert(start);
            let mut current = start;
            let mut closed = false;
            loop {
                let mut probe = self.halfedges[current as usize].next;
                let mut guard = 0usize;
                let next_boundary = loop {
                    let Some(twin) = self.halfedges[probe as usize].twin else {
                        break Some(probe);
                    };
                    probe = self.halfedges[twin as usize].next;
                    guard += 1;
                    if guard > self.halfedges.len() + 4 {
                        break None;
                    }
                };
                let Some(next_he) = next_boundary else { break };
                if next_he == start {
                    closed = true;
                    break;
                }
                if visited.contains(&next_he) {
                    break;
                }
                visited.insert(next_he);
                loop_he_ids.push(next_he);
                current = next_he;
            }
            if closed && loop_he_ids.len() >= 3 {
                new_loops.push(loop_he_ids.iter().map(|&he| self.halfedges[he as usize].vertex).collect());
            }
        }
        if new_loops.is_empty() {
            return Ok(0);
        }
        let filled = new_loops.len();
        let (positions, face_list) = self.polygon_soup();
        let mut all_faces = face_list;
        for mut loop_verts in new_loops {
            // Boundary loops are traced in the "hole" direction (following existing faces' own winding
            // around the missing area); the cap face must have the opposite winding to be consistent.
            loop_verts.reverse();
            all_faces.push(loop_verts);
        }
        self.rebuild_from_polygon_soup(&positions, &all_faces)?;
        Ok(filled)
    }

    /// 🪞 Reflects geometry on one side of the axis plane and welds its seam.
    pub fn mirror(&mut self, axis: MirrorAxis, weld_threshold: f32) -> MeshResult<()> {
        if !weld_threshold.is_finite() || weld_threshold < 0.0 { return Err(MeshKernelError::DegenerateOperation); }
        let axis = match axis { MirrorAxis::X => 0, MirrorAxis::Y => 1, MirrorAxis::Z => 2 };
        let (mut positions, faces) = self.polygon_soup();
        let tolerance = weld_threshold * 0.5;
        if positions.iter().any(|point| point[axis] > tolerance) && positions.iter().any(|point| point[axis] < -tolerance) { return Err(MeshKernelError::InvalidInput("mirror geometry must lie on one side of its plane".into())); }
        let count = positions.len();
        let mut reflected = Vec::with_capacity(count);
        for id in 0..count {
            if positions[id][axis].abs() <= tolerance {
                positions[id][axis] = 0.0;
                reflected.push(id as u32);
            } else {
                let mut point = positions[id]; point[axis] = -point[axis];
                reflected.push(positions.len() as u32); positions.push(point);
            }
        }
        let mut result_faces = Vec::new();
        for face in faces {
            if face.iter().all(|&id| positions[id as usize][axis] == 0.0) { continue; }
            let mirrored = face.iter().rev().map(|&id| reflected[id as usize]).collect();
            result_faces.push(face); result_faces.push(mirrored);
        }
        let mut result = Self::from_faces(&positions, &result_faces)?;
        result.drop_unreferenced_vertices()?;
        *self = result;
        Ok(())
    }

    pub fn decimate(&mut self, target_ratio: f32) -> MeshResult<()> {
        self.decimate_with_progress(target_ratio, |_| true)
    }

    /// 🪶 Collapses shortest safe edges, preserving winding and manifold edge incidence.
    pub fn decimate_with_progress(&mut self, target_ratio: f32, progress: impl FnMut(f32) -> bool) -> MeshResult<()> {
        *self = self.decimate_job(target_ratio)?.finish_with_progress(progress)?;
        Ok(())
    }

    /// 🧹️ Compacts away vertices no longer referenced by any face, so `vertex_count()` reflects the mesh's
    /// actual remaining complexity after operations (like [`Self::decimate`]) that remap vertex ids away
    /// without themselves shrinking the position buffer.
    fn drop_unreferenced_vertices(&mut self) -> MeshResult<()> {
        let (positions, face_list) = self.polygon_soup();
        let mut used = vec![false; positions.len()];
        for face in &face_list {
            for &vi in face {
                used[vi as usize] = true;
            }
        }
        if used.iter().all(|&u| u) {
            return Ok(());
        }
        let mut remap = vec![0u32; positions.len()];
        let mut compacted = Vec::new();
        for (i, &keep) in used.iter().enumerate() {
            if keep {
                remap[i] = compacted.len() as u32;
                compacted.push(positions[i]);
            }
        }
        let new_faces: Vec<Vec<u32>> = face_list.into_iter().map(|f| f.into_iter().map(|vi| remap[vi as usize]).collect()).collect();
        self.rebuild_from_polygon_soup(&compacted, &new_faces)
    }

    pub fn set_shading(&mut self, faces: &[FaceId], smooth: bool) -> MeshResult<()> {
        if faces.is_empty() { return Err(MeshKernelError::EmptySelection); }
        for &face in faces { self.face_vertex_ids(face)?; }
        for &face in faces { self.faces[face.0 as usize].smooth = smooth; }
        self.recompute_normals()
    }

    pub fn recompute_normals(&mut self) -> MeshResult<()> {
        let mut flat = vec![None; self.vertex_count()];
        let mut smooth_sums = vec![Vec3::ZERO; self.vertex_count()];
        for fi in 0..self.faces.len() {
            let normal = self.face_normal(FaceId(fi as u32))?;
            for vertex in self.face_vertex_ids(FaceId(fi as u32))? {
                if flat[vertex.0 as usize].is_none() { flat[vertex.0 as usize] = Some(normal); }
                if self.faces[fi].smooth { smooth_sums[vertex.0 as usize] = smooth_sums[vertex.0 as usize].add(normal); }
            }
        }
        for (id,vertex) in self.vertices.iter_mut().enumerate() {
            vertex.normal = if smooth_sums[id].length() > 0.0 { Some(smooth_sums[id].normalize().0) } else { flat[id].map(|normal| normal.0) };
        }
        Ok(())
    }

}

//#endregion Edit

//#region Uv

fn cot_angle(a: Vec3, b: Vec3, c: Vec3) -> f32 {
    let ab = b.sub(a);
    let ac = c.sub(a);
    let cross_len = ab.cross(ac).length();
    if cross_len < 1e-8 {
        return 0.0;
    }
    ab.dot(ac) / cross_len
}

fn solve_lscm_1d(n: usize, triplets: &[(usize, usize, f64)], pin_a: usize, pin_b: usize, val_a: f64, val_b: f64) -> Vec<f64> {
    let free: Vec<usize> = (0..n).filter(|&i| i != pin_a && i != pin_b).collect();
    let m = free.len();
    if m == 0 {
        let mut out = vec![0.0; n];
        out[pin_a] = val_a;
        out[pin_b] = val_b;
        return out;
    }
    let mut a = vec![0.0f64; m * m];
    let mut b = vec![0.0f64; m];
    let idx = |v: usize| -> Option<usize> {
        if v == pin_a || v == pin_b {
            None
        } else {
            free.iter().position(|&x| x == v)
        }
    };
    for &(i, j, w) in triplets {
        if i == j {
            if let Some(ii) = idx(i) {
                a[ii * m + ii] += w;
            }
        } else {
            if let Some(ii) = idx(i) {
                if let Some(jj) = idx(j) {
                    a[ii * m + jj] -= w;
                } else if j == pin_a {
                    b[ii] += w * val_a;
                } else if j == pin_b {
                    b[ii] += w * val_b;
                }
            }
            if let Some(jj) = idx(j) {
                if let Some(ii) = idx(i) {
                    a[jj * m + ii] -= w;
                } else if i == pin_a {
                    b[jj] += w * val_a;
                } else if i == pin_b {
                    b[jj] += w * val_b;
                }
            }
        }
    }
    for row in 0..m {
        let mut pivot = row;
        for r in (row + 1)..m {
            if a[r * m + row].abs() > a[pivot * m + row].abs() {
                pivot = r;
            }
        }
        if a[pivot * m + row].abs() < 1e-12 {
            continue;
        }
        if pivot != row {
            for c in 0..m {
                a.swap(row * m + c, pivot * m + c);
            }
            b.swap(row, pivot);
        }
        let div = a[row * m + row];
        for c in row..m {
            a[row * m + c] /= div;
        }
        b[row] /= div;
        for r in 0..m {
            if r == row {
                continue;
            }
            let factor = a[r * m + row];
            if factor.abs() < 1e-12 {
                continue;
            }
            for c in row..m {
                a[r * m + c] -= factor * a[row * m + c];
            }
            b[r] -= factor * b[row];
        }
    }
    let mut out = vec![0.0; n];
    out[pin_a] = val_a;
    out[pin_b] = val_b;
    for (fi, &vi) in free.iter().enumerate() {
        out[vi] = b[fi];
    }
    out
}

impl HalfedgeMesh {
    pub fn mark_uv_seam(&mut self, edges: &[EdgeId], seam: bool) {
        for &edge in edges {
            self.uv_seams.insert(edge.0);
            if !seam {
                self.uv_seams.remove(&edge.0);
            }
        }
    }

    pub fn is_uv_seam(&self, edge: EdgeId) -> bool {
        self.uv_seams.contains(&edge.0)
    }

    fn uv_island_faces(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.faces.len()];
        let mut islands = Vec::new();
        for start in 0..self.faces.len() {
            if visited[start] {
                continue;
            }
            let mut stack = vec![start];
            let mut island = Vec::new();
            visited[start] = true;
            while let Some(fi) = stack.pop() {
                island.push(fi);
                let hes = self.face_halfedge_ids(FaceId(fi as u32)).unwrap_or_default();
                for he_id in hes {
                    let he = &self.halfedges[he_id as usize];
                    if self.uv_seams.contains(&he_id) {
                        continue;
                    }
                    if let Some(twin_id) = he.twin {
                        let twin = &self.halfedges[twin_id as usize];
                        if let Some(adj) = twin.face {
                            let adj = adj as usize;
                            if !visited[adj] {
                                visited[adj] = true;
                                stack.push(adj);
                            }
                        }
                    }
                }
            }
            if !island.is_empty() {
                islands.push(island);
            }
        }
        islands
    }

    fn solve_island_uv(&self, island_faces: &[usize]) -> HashMap<u32, [f32; 2]> {
        let mut vert_set: HashSet<u32> = HashSet::new();
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        for &fi in island_faces {
            let verts = self.face_vertex_ids(FaceId(fi as u32)).unwrap_or_default();
            if verts.len() < 3 {
                continue;
            }
            let face_positions: Vec<Vec3> = verts.iter().map(|v| Vec3(self.vertices[v.0 as usize].position)).collect();
            for tri in triangulate_polygon(&face_positions) {
                triangles.push([verts[tri[0]].0, verts[tri[1]].0, verts[tri[2]].0]);
            }
            for v in verts {
                vert_set.insert(v.0);
            }
        }
        let verts: Vec<u32> = vert_set.into_iter().collect();
        let n = verts.len();
        if n < 3 {
            return HashMap::new();
        }
        let index: HashMap<u32, usize> = verts.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let pos = |vid: u32| Vec3(self.vertices[vid as usize].position);
        let mut triplets: Vec<(usize, usize, f64)> = Vec::new();
        for tri in &triangles {
            let (a, b, c) = (tri[0], tri[1], tri[2]);
            let ia = index[&a];
            let ib = index[&b];
            let ic = index[&c];
            let pa = pos(a);
            let pb = pos(b);
            let pc = pos(c);
            let cot_a = cot_angle(pb, pa, pc) as f64;
            let cot_b = cot_angle(pa, pb, pc) as f64;
            let cot_c = cot_angle(pa, pc, pb) as f64;
            let pairs = [(ia, ib, cot_c), (ib, ia, cot_c), (ib, ic, cot_a), (ic, ib, cot_a), (ia, ic, cot_b), (ic, ia, cot_b)];
            for (i, j, w) in pairs {
                if w.abs() < 1e-12 {
                    continue;
                }
                triplets.push((i, i, w));
                triplets.push((i, j, -w));
            }
        }
        let pin_a = 0;
        let mut pin_b = 1;
        let mut max_dist = 0.0f32;
        let p0 = pos(verts[pin_a]);
        for (i, &v) in verts.iter().enumerate().skip(1) {
            let d = p0.sub(pos(v)).length();
            if d > max_dist {
                max_dist = d;
                pin_b = i;
            }
        }
        let u = solve_lscm_1d(n, &triplets, pin_a, pin_b, 0.0, 1.0);
        let v = solve_lscm_1d(n, &triplets, pin_a, pin_b, 0.0, 0.0);
        verts.into_iter().enumerate().map(|(i, vid)| (vid, [u[i] as f32, v[i] as f32])).collect()
    }

    fn pack_island_uvs(&self, islands: &[Vec<usize>]) -> HashMap<u32, [f32; 2]> {
        let mut packed = HashMap::new();
        let mut shelf_y = 0.0f32;
        let mut shelf_height = 0.0f32;
        let mut shelf_x = 0.0f32;
        const PAD: f32 = 0.01;
        for island in islands {
            let local = self.solve_island_uv(island);
            if local.is_empty() {
                continue;
            }
            let mut min_u = f32::INFINITY;
            let mut min_v = f32::INFINITY;
            let mut max_u = f32::NEG_INFINITY;
            let mut max_v = f32::NEG_INFINITY;
            for uv in local.values() {
                min_u = min_u.min(uv[0]);
                min_v = min_v.min(uv[1]);
                max_u = max_u.max(uv[0]);
                max_v = max_v.max(uv[1]);
            }
            let w = (max_u - min_u).max(1e-4);
            let h = (max_v - min_v).max(1e-4);
            if shelf_x + w + PAD > 1.0 {
                shelf_x = 0.0;
                shelf_y += shelf_height + PAD;
                shelf_height = 0.0;
            }
            shelf_height = shelf_height.max(h);
            let scale = (w.max(h)).min(0.45);
            for (vid, uv) in local {
                let nu = shelf_x + (uv[0] - min_u) / w * scale;
                let nv = shelf_y + (uv[1] - min_v) / h * scale;
                packed.insert(vid, [nu, nv]);
            }
            shelf_x += scale + PAD;
        }
        packed
    }

    pub fn unwrap_uv(&mut self) -> MeshResult<()> {
        let islands = self.uv_island_faces();
        let packed = self.pack_island_uvs(&islands);
        for fi in 0..self.faces.len() {
            let hes = self.face_halfedge_ids(FaceId(fi as u32))?;
            for he_id in hes {
                let he = &mut self.halfedges[he_id as usize];
                if let Some(uv) = packed.get(&he.vertex) {
                    he.uv = *uv;
                }
            }
        }
        Ok(())
    }
}

//#endregion Uv

//#region Polygon

/// 🧭️ Unit Newell normal evaluated relative to the first corner in double precision.
fn newell_normal(positions: &[Vec3]) -> Vec3 {
    let n = positions.len();
    let Some(origin) = positions.first() else { return Vec3::ZERO; };
    let mut nx = 0.0f64;
    let mut ny = 0.0f64;
    let mut nz = 0.0f64;
    for i in 0..n {
        let a = positions[i];
        let b = positions[(i + 1) % n];
        let (ax, ay, az) = (a.x() as f64 - origin.x() as f64, a.y() as f64 - origin.y() as f64, a.z() as f64 - origin.z() as f64);
        let (bx, by, bz) = (b.x() as f64 - origin.x() as f64, b.y() as f64 - origin.y() as f64, b.z() as f64 - origin.z() as f64);
        nx += (ay - by) * (az + bz);
        ny += (az - bz) * (ax + bx);
        nz += (ax - bx) * (ay + by);
    }
    let length = nx.hypot(ny).hypot(nz);
    if length == 0.0 { return Vec3::ZERO; }
    Vec3::new((nx / length) as f32, (ny / length) as f32, (nz / length) as f32)
}

type Vec3f64 = (f64, f64, f64);

fn sub3(a: Vec3f64, b: Vec3f64) -> Vec3f64 {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn dot3(a: Vec3f64, b: Vec3f64) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn cross3(a: Vec3f64, b: Vec3f64) -> Vec3f64 {
    (a.1 * b.2 - a.2 * b.1, a.2 * b.0 - a.0 * b.2, a.0 * b.1 - a.1 * b.0)
}

fn length3(a: Vec3f64) -> f64 {
    dot3(a, a).sqrt()
}

fn normalize3(a: Vec3f64) -> Vec3f64 {
    let l = length3(a);
    if l < 1e-12 {
        return (0.0, 0.0, 0.0);
    }
    (a.0 / l, a.1 / l, a.2 / l)
}

/// Least-parallel-world-axis projection basis for a plane with the given unit normal.
fn plane_basis(normal: Vec3f64) -> (Vec3f64, Vec3f64) {
    let (nx, ny, nz) = normal;
    let reference: Vec3f64 = if nx.abs() < ny.abs() && nx.abs() < nz.abs() {
        (1.0, 0.0, 0.0)
    } else if ny.abs() < nz.abs() {
        (0.0, 1.0, 0.0)
    } else {
        (0.0, 0.0, 1.0)
    };
    let axis_u = normalize3(cross3(normal, reference));
    let axis_v = cross3(normal, axis_u);
    (axis_u, axis_v)
}

fn cross2(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn point_in_triangle(p: (f64, f64), a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
    let d1 = cross2(a, b, p);
    let d2 = cross2(b, c, p);
    let d3 = cross2(c, a, p);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

/// Triangulates a planar polygon that may contain holes. Holes are bridged into the outer loop with a
/// keyhole (doubled bridge edge) then ear-clipped, so the result covers only the solid region.
fn triangulate_indexed_polygon_with_holes(positions: &[[f32; 3]], outer: &[u32], holes: &[Vec<u32>]) -> Vec<[u32; 3]> {
    if holes.is_empty() {
        let pts: Vec<Vec3> = outer.iter().map(|&i| Vec3(positions[i as usize])).collect();
        return triangulate_polygon(&pts).into_iter().map(|[a, b, c]| [outer[a], outer[b], outer[c]]).collect();
    }
    let mut combined: Vec<u32> = outer.to_vec();
    let mut remaining: Vec<Vec<u32>> = holes.to_vec();
    while let Some((hole_index, outer_pos, hole_pos)) = find_closest_bridge(&combined, &remaining, positions) {
        let hole = remaining.remove(hole_index);
        let mut spliced = Vec::with_capacity(combined.len() + hole.len() + 2);
        spliced.extend_from_slice(&combined[..=outer_pos]);
        spliced.push(hole[hole_pos]);
        for k in 1..hole.len() {
            spliced.push(hole[(hole_pos + k) % hole.len()]);
        }
        spliced.push(hole[hole_pos]);
        spliced.push(combined[outer_pos]);
        spliced.extend_from_slice(&combined[outer_pos + 1..]);
        combined = spliced;
    }
    let pts: Vec<Vec3> = combined.iter().map(|&i| Vec3(positions[i as usize])).collect();
    triangulate_polygon(&pts).into_iter().map(|[a, b, c]| [combined[a], combined[b], combined[c]]).collect()
}

fn find_closest_bridge(outer: &[u32], holes: &[Vec<u32>], positions: &[[f32; 3]]) -> Option<(usize, usize, usize)> {
    let mut best: Option<(f32, usize, usize, usize)> = None;
    for (hi, hole) in holes.iter().enumerate() {
        for (oi, &ov) in outer.iter().enumerate() {
            let operation = Vec3(positions[ov as usize]);
            for (hpi, &hv) in hole.iter().enumerate() {
                let hp = Vec3(positions[hv as usize]);
                let d = operation.sub(hp).length();
                if best.is_none_or(|(bd, _, _, _)| d < bd) {
                    best = Some((d, hi, oi, hpi));
                }
            }
        }
    }
    best.map(|(_, hi, oi, hpi)| (hi, oi, hpi))
}

/// Deterministic ear-clipping triangulation of a simple polygon (convex or concave), given ordered 3D corner
/// positions. Falls back to a fan (previous behavior) whenever the polygon is degenerate (zero-area / collinear)
/// or clipping stalls, so it never returns fewer than `n - 2` triangles or panics.
fn triangulate_polygon(positions: &[Vec3]) -> Vec<[usize; 3]> {
    let n = positions.len();
    if n < 3 {
        return Vec::new();
    }
    if n == 3 {
        return vec![[0, 1, 2]];
    }
    let fan = |from: usize| -> Vec<[usize; 3]> {
        let _ = from;
        (1..n - 1).map(|i| [0, i, i + 1]).collect()
    };
    let mut points_f64: Vec<Vec3f64> = Vec::with_capacity(positions.len());
    for p in positions {
        points_f64.push((p.x() as f64, p.y() as f64, p.z() as f64));
    }
    let normal = newell_normal(positions);
    if normal.length() < 1e-8 {
        return fan(0);
    }
    let normal_f64 = normalize3((normal.x() as f64, normal.y() as f64, normal.z() as f64));
    let (axis_u, axis_v) = plane_basis(normal_f64);
    let origin = points_f64[0];
    let mut projected: Vec<(f64, f64)> = Vec::with_capacity(points_f64.len());
    for &p in &points_f64 {
        let local = sub3(p, origin);
        projected.push((dot3(local, axis_u), dot3(local, axis_v)));
    }
    let scale = projected.iter().fold(0.0f64, |scale, &(x, y)| scale.max(x.abs()).max(y.abs()));
    if scale == 0.0 { return fan(0); }
    for point in &mut projected { point.0 /= scale; point.1 /= scale; }
    let mut signed_area2 = 0.0f64;
    for i in 0..n {
        let a = projected[i];
        let b = projected[(i + 1) % n];
        signed_area2 += a.0 * b.1 - b.0 * a.1;
    }
    if signed_area2.abs() < 1e-14 {
        return fan(0);
    }
    let ccw = signed_area2 > 0.0;

    let mut indices: Vec<usize> = (0..n).collect();
    let mut triangles = Vec::with_capacity(n - 2);
    let mut guard = 0usize;
    let guard_limit = n * n + 16;
    while indices.len() > 3 {
        guard += 1;
        if guard > guard_limit {
            for i in 1..indices.len() - 1 {
                triangles.push([indices[0], indices[i], indices[i + 1]]);
            }
            return triangles;
        }
        let m = indices.len();
        let mut ear_found = false;
        for i in 0..m {
            let prev = indices[(i + m - 1) % m];
            let curr = indices[i];
            let next = indices[(i + 1) % m];
            let a = projected[prev];
            let b = projected[curr];
            let c = projected[next];
            let cross = cross2(a, b, c);
            let is_convex = if ccw { cross > 1e-14 } else { cross < -1e-14 };
            if !is_convex {
                continue;
            }
            let mut contains_other = false;
            for &k in &indices {
                if k == prev || k == curr || k == next {
                    continue;
                }
                if point_in_triangle(projected[k], a, b, c) {
                    contains_other = true;
                    break;
                }
            }
            if contains_other {
                continue;
            }
            triangles.push([prev, curr, next]);
            indices.remove(i);
            ear_found = true;
            break;
        }
        if !ear_found {
            for i in 1..indices.len() - 1 {
                triangles.push([indices[0], indices[i], indices[i + 1]]);
            }
            return triangles;
        }
    }
    triangles.push([indices[0], indices[1], indices[2]]);
    triangles
}

fn find_edge_position(loop_verts: &[u32], from: u32, to: u32) -> Option<usize> {
    let n = loop_verts.len();
    (0..n).find(|&i| loop_verts[i] == from && loop_verts[(i + 1) % n] == to)
}

/// Merges two face loops that share exactly one boundary edge (in opposite winding, as guaranteed by a
/// consistently-oriented manifold) into a single n-gon loop. Returns `None` if the faces do not share exactly
/// one edge, or if splicing would produce a loop with a repeated vertex (non-simple / holed result).
fn merge_face_loops(a: &[u32], b: &[u32]) -> Option<Vec<u32>> {
    let n = a.len();
    let m = b.len();
    if n < 3 || m < 3 {
        return None;
    }
    let mut shared_a_pos = None;
    for i in 0..n {
        let u = a[i];
        let v = a[(i + 1) % n];
        if find_edge_position(b, v, u).is_some() {
            shared_a_pos = Some(i);
            break;
        }
    }
    let a_pos = shared_a_pos?;
    let u = a[a_pos];
    let v = a[(a_pos + 1) % n];
    let b_pos = find_edge_position(b, v, u)?;
    for i in 0..n {
        if i == a_pos {
            continue;
        }
        let uu = a[i];
        let vv = a[(i + 1) % n];
        if find_edge_position(b, vv, uu).is_some() {
            return None;
        }
    }
    let a_rot: Vec<u32> = (0..n).map(|k| a[(a_pos + k) % n]).collect();
    let b_rot: Vec<u32> = (0..m).map(|k| b[(b_pos + k) % m]).collect();
    let mut merged = Vec::with_capacity(n + m - 2);
    merged.push(a_rot[0]);
    merged.extend_from_slice(&b_rot[2..]);
    merged.extend_from_slice(&a_rot[1..]);
    let mut seen = HashSet::new();
    for &vid in &merged {
        if !seen.insert(vid) {
            return None;
        }
    }
    Some(merged)
}

const COPLANAR_NORMAL_DOT_MIN: f32 = 1.0 - 1e-4;
const COPLANAR_DISTANCE_REL_TOL: f32 = 1e-4;

fn faces_coplanar(positions: &[[f32; 3]], a: &[u32], b: &[u32]) -> bool {
    let pos = |vi: u32| Vec3(positions[vi as usize]);
    let a_pts: Vec<Vec3> = a.iter().map(|&vi| pos(vi)).collect();
    let b_pts: Vec<Vec3> = b.iter().map(|&vi| pos(vi)).collect();
    let na = newell_normal(&a_pts);
    let la = na.length();
    if la < 1e-10 {
        return false;
    }
    let na_n = na.scale(1.0 / la);
    let nb = newell_normal(&b_pts);
    let lb = nb.length();
    if lb < 1e-10 {
        return false;
    }
    let nb_n = nb.scale(1.0 / lb);
    if na_n.dot(nb_n) < COPLANAR_NORMAL_DOT_MIN {
        return false;
    }
    let origin = a_pts[0];
    let mut min = a_pts[0];
    let mut max = a_pts[0];
    for &p in a_pts.iter().chain(b_pts.iter()) {
        min = Vec3::new(min.x().min(p.x()), min.y().min(p.y()), min.z().min(p.z()));
        max = Vec3::new(max.x().max(p.x()), max.y().max(p.y()), max.z().max(p.z()));
    }
    let diag = max.sub(min).length();
    let tol = (COPLANAR_DISTANCE_REL_TOL * diag).max(1e-6);
    for &p in b_pts.iter() {
        if p.sub(origin).dot(na_n).abs() > tol {
            return false;
        }
    }
    true
}

/// Drops vertices that are a straight (~180°) pass-through in *every* face loop that references them, i.e. whose
/// loop-neighbors are identical across all incident faces. Turns merged coplanar-face borders into clean n-gon
/// corners instead of chains of collinear vertices left over from the original triangulation.
fn collinear_cleanup(positions: &[[f32; 3]], face_list: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let pos = |vi: u32| Vec3(positions[vi as usize]);
    let mut neighbor_pairs: HashMap<u32, HashSet<(u32, u32)>> = HashMap::new();
    for face in face_list {
        let n = face.len();
        for i in 0..n {
            let prev = face[(i + n - 1) % n];
            let curr = face[i];
            let next = face[(i + 1) % n];
            neighbor_pairs.entry(curr).or_default().insert((prev, next));
        }
    }
    let mut removable: HashSet<u32> = HashSet::new();
    for (&vid, pairs) in &neighbor_pairs {
        if pairs.len() != 1 {
            continue;
        }
        let Some(&(prev, next)) = pairs.iter().next() else { continue };
        if prev == next || prev == vid || next == vid {
            continue;
        }
        let d1 = pos(vid).sub(pos(prev));
        let d2 = pos(next).sub(pos(vid));
        if d1.length() < 1e-9 || d2.length() < 1e-9 {
            continue;
        }
        if d1.normalize().dot(d2.normalize()) > 1.0 - 1e-4 {
            removable.insert(vid);
        }
    }
    face_list.iter().map(|face| face.iter().copied().filter(|v| !removable.contains(v)).collect::<Vec<u32>>()).filter(|face: &Vec<u32>| face.len() >= 3).collect()
}

//#endregion Polygon

//#region Export

impl HalfedgeMesh {
    pub fn tessellate(&self) -> MeshResult<MeshTransfer> {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        let mut edge_positions = Vec::new();
        let mut face_ids = Vec::new();
        let mut vertex_ids = Vec::new();
        let mut edge_ids = Vec::new();
        let mut uvs = Vec::new();
        let mut edge_uvs = Vec::new();
        let mut edge_is_seam = Vec::new();
        let mut edge_seen: HashMap<(u32, u32), bool> = HashMap::new();

        for fi in 0..self.faces.len() {
            let face = &self.faces[fi];
            let smooth = face.smooth;
            let topology_hes = self.face_halfedge_ids(FaceId(fi as u32))?;
            let topology_verts: Vec<VertexId> = topology_hes.iter().map(|halfedge| VertexId(self.halfedges[*halfedge as usize].vertex)).collect();
            let mut hes = topology_hes.clone();
            if face.flipped {
                hes.reverse();
            }
            let verts = self.face_vertex_ids(FaceId(fi as u32))?;
            if verts.len() < 3 {
                continue;
            }
            let face_normal = self.face_normal(FaceId(fi as u32))?;
            let base = positions.len() as u32 / 3;

            let push_corner = |he_id: u32, positions: &mut Vec<f32>, normals: &mut Vec<f32>, vertex_ids: &mut Vec<u32>, uvs: &mut Vec<f32>, normal: Vec3| {
                let he = &self.halfedges[he_id as usize];
                let vert = &self.vertices[he.vertex as usize];
                let n = if smooth { vert.normal.map_or(normal, Vec3) } else { normal };
                positions.extend_from_slice(&vert.position);
                normals.extend_from_slice(&n.0);
                vertex_ids.push(he.vertex);
                uvs.push(he.uv[0]);
                uvs.push(he.uv[1]);
            };

            let face_positions: Vec<Vec3> = verts.iter().map(|v| Vec3(self.vertices[v.0 as usize].position)).collect();
            let triangles = triangulate_polygon(&face_positions);

            if smooth {
                for &he_id in &hes {
                    push_corner(he_id, &mut positions, &mut normals, &mut vertex_ids, &mut uvs, face_normal);
                }
                for tri in &triangles {
                    indices.push(base + tri[0] as u32);
                    indices.push(base + tri[1] as u32);
                    indices.push(base + tri[2] as u32);
                    face_ids.push(fi as u32);
                }
            } else {
                for tri in &triangles {
                    for &local in tri {
                        push_corner(hes[local], &mut positions, &mut normals, &mut vertex_ids, &mut uvs, face_normal);
                    }
                    let tri_base = (positions.len() / 3 - 3) as u32;
                    indices.push(tri_base);
                    indices.push(tri_base + 1);
                    indices.push(tri_base + 2);
                    face_ids.push(fi as u32);
                }
            }

            for i in 0..topology_verts.len() {
                let v0 = topology_verts[i].0;
                let v1 = topology_verts[(i + 1) % topology_verts.len()].0;
                let key = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                if edge_seen.contains_key(&key) {
                    continue;
                }
                edge_seen.insert(key, true);
                let p0 = self.vertices[v0 as usize].position;
                let p1 = self.vertices[v1 as usize].position;
                edge_positions.extend_from_slice(&p0);
                edge_positions.extend_from_slice(&p1);
                let he_id = topology_hes[i];
                let he = &self.halfedges[he_id as usize];
                let he_next = &self.halfedges[he.next as usize];
                edge_ids.push(he_id);
                edge_uvs.push(he.uv[0]);
                edge_uvs.push(he.uv[1]);
                edge_uvs.push(he_next.uv[0]);
                edge_uvs.push(he_next.uv[1]);
                edge_is_seam.push(if self.uv_seams.contains(&he_id) { 1 } else { 0 });
            }
        }

        Ok(MeshTransfer { positions, normals, indices, edge_positions, face_ids, vertex_ids, edge_ids, uvs, edge_uvs, edge_is_seam })
    }

    pub fn to_obj(&self) -> MeshResult<String> {
        let mut out = String::from("# kernel_3d_mesh OBJ export\n");
        for v in &self.vertices {
            out.push_str(&format!("v {} {} {}\n", v.position[0], v.position[1], v.position[2]));
        }
        let mut vt_written = false;
        for he in &self.halfedges {
            if he.uv[0] != 0.0 || he.uv[1] != 0.0 {
                vt_written = true;
                break;
            }
        }
        if vt_written {
            for he in &self.halfedges {
                out.push_str(&format!("vt {} {}\n", he.uv[0], he.uv[1]));
            }
        }
        for fi in 0..self.faces.len() {
            let mut hes = self.face_halfedge_ids(FaceId(fi as u32))?;
            if self.faces[fi].flipped {
                hes.reverse();
            }
            out.push('f');
            for he_id in hes {
                let he = &self.halfedges[he_id as usize];
                if vt_written {
                    out.push_str(&format!(" {}/{}", he.vertex + 1, he_id as usize + 1));
                } else {
                    out.push_str(&format!(" {}", he.vertex + 1));
                }
            }
            out.push('\n');
        }
        Ok(out)
    }

    pub fn to_json(&self) -> MeshResult<String> {
        Ok(pack::json::to_json_string(self))
    }

    pub fn from_json(json: &str) -> MeshResult<Self> {
        pack::json::from_json_str(json).map_err(|e| MeshKernelError::InvalidInput(e.to_string()))
    }
}

//#endregion Export

//#region Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️modeling/🦀️.rs"]
mod modeling_tests;
