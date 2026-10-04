//! 🔷️ Half-edge mesh kernel for low-poly editing. **Host authority:** `HalfedgeMesh` is a value
//! document/engine payload — not a process-global mesh store.

// 🔬️ `serde`/`serde_json` survive ONLY as a `#[cfg(test)]` differential oracle now that these
// types have their own first-party `ToValue`/`FromValue` codec — never a production dependency
// of this crate. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[cfg(test)]
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

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

#[cfg(test)]
mod attribute_serde_bridge {
    use protocol::value::{FromValue, ToValue};
    use serde::{Deserialize, Serialize};
    pub fn serialize<T:ToValue,S: serde::Serializer>(value: &T, serializer: S) -> Result<S::Ok,S::Error> {
        serde_json::Value::from(value.to_value()).serialize(serializer)
    }
    pub fn deserialize<'de,T:FromValue,D:serde::Deserializer<'de>>(deserializer:D)->Result<T,D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        T::from_value(protocol::value::DslValue::from(value)).map_err(serde::de::Error::custom)
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

    pub fn scale(self, s: f32) -> Self { Self(self.0.map(|value|value*s)) }

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

pub use semio_framework_mesh_engine::{MeshAttribute, MeshAttributeDomain, MeshAttributeSemantic, MeshAttributeInterpolation, MeshTexture};

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
    pub halfedge: Option<u32>,
}

// 🚫️async: E4 fn-pointer slot — serde's `#[serde(default = "...")]` calls this by path as a plain
// `fn() -> T`; this helper must remain synchronous.
fn default_uv() -> [f32; 2] {
    [0.0, 0.0]
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value")]
pub struct HalfEdge {
    pub vertex: u32,
    pub twin: Option<u32>,
    pub next: u32,
    pub face: Option<u32>,
    #[cfg_attr(test, serde(default = "default_uv"))]
    #[value(default = "default_uv")]
    pub uv: [f32; 2],
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value")]
pub struct MeshFace {
    pub halfedge: u32,
    pub smooth: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub flipped: bool,
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::protocol::value")]
pub struct MeshTransfer {
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub colors: Vec<f32>,
    #[value(default)]
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    pub attributes: std::collections::BTreeMap<String,MeshAttribute>,
    #[value(default)]
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    pub materials: std::collections::BTreeMap<String,protocol::value::DslValue>,
    #[value(default)]
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    pub textures: std::collections::BTreeMap<String,MeshTexture>,
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
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    #[value(default)]
    attributes: BTreeMap<String, MeshAttribute>,
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    #[value(default)]
    materials: BTreeMap<String,protocol::value::DslValue>,
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    #[value(default)]
    textures: BTreeMap<String,MeshTexture>,
}

impl protocol::value::retirement::RetireOwned for Vec3 {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::retirement::RetireOwned::retirement(self.0)}
}
impl protocol::value::retirement::RetireOwned for HalfedgeMesh {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.vertices,self.halfedges,self.faces,self.uv_seams,self.attributes,self.materials,self.textures]}
}
impl protocol::value::retirement::RetireOwned for MeshVertex {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.position,self.normal,self.halfedge]}
}
impl protocol::value::retirement::RetireOwned for HalfEdge {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.vertex,self.twin,self.next,self.face,self.uv]}
}
impl protocol::value::retirement::RetireOwned for MeshFace {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.halfedge,self.smooth,self.flipped]}
}
impl protocol::value::retirement::RetireOwned for MeshTransfer {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.positions,self.normals,self.colors,self.indices,self.uvs,self.face_ids,self.vertex_ids,self.edge_positions,self.edge_ids,self.edge_uvs,self.edge_is_seam,self.attributes,self.materials,self.textures]}
}

impl HalfedgeMesh {
    /// 🧩️ Transfer all seven kernel owners directly to an individually typed persisted model.
    pub fn into_owned_parts(self)->(Vec<MeshVertex>,Vec<HalfEdge>,Vec<MeshFace>,HashSet<u32>,BTreeMap<String,MeshAttribute>,BTreeMap<String,protocol::value::DslValue>,BTreeMap<String,MeshTexture>){(self.vertices,self.halfedges,self.faces,self.uv_seams,self.attributes,self.materials,self.textures)}
    /// 🥽️ Editable literal topology enters the kernel without a serialized mesh document.
    pub fn from_owned_parts(vertices:Vec<MeshVertex>,halfedges:Vec<HalfEdge>,faces:Vec<MeshFace>,uv_seams:HashSet<u32>,attributes:BTreeMap<String,MeshAttribute>,materials:BTreeMap<String,protocol::value::DslValue>,textures:BTreeMap<String,MeshTexture>)->Self{Self{vertices,halfedges,faces,uv_seams,attributes,materials,textures}}
    pub fn empty() -> Self {
        Self { vertices: Vec::new(), halfedges: Vec::new(), faces: Vec::new(), uv_seams: HashSet::new(), attributes: BTreeMap::new(), materials:BTreeMap::new(),textures:BTreeMap::new() }
    }

    /// 🏷️ Authored channels are read from the same mesh value consumed by retained jobs.
    pub fn attributes(&self) -> &BTreeMap<String, MeshAttribute> { &self.attributes }

    /// 🎨️ Material references and texture bytes belong to this same owned mesh payload.
    pub fn materials(&self)->&BTreeMap<String,protocol::value::DslValue> { &self.materials }
    /// 🖼️ Authored texture records are retained without external resource ownership.
    pub fn textures(&self)->&BTreeMap<String,MeshTexture> { &self.textures }

    /// 📦️ Admits material and texture records before dependent face channels.
    pub fn set_surface_assets(&mut self,materials:BTreeMap<String,protocol::value::DslValue>,textures:BTreeMap<String,MeshTexture>)->MeshResult<()> {
        semio_framework_mesh_engine::validate_mesh_surface_assets(&materials,&textures).map_err(MeshKernelError::InvalidInput)?;
        if self.attributes.values().filter(|attribute|attribute.semantic==MeshAttributeSemantic::Material).flat_map(|attribute|&attribute.values).any(|value|value.as_str().is_none_or(|name|!materials.contains_key(name))) { return Err(MeshKernelError::InvalidInput("undefined mesh face material".into())); }
        self.materials=materials;self.textures=textures;
        Ok(())
    }

    /// ✅️ Admits a channel atomically after checking its domain and interpolation shape.
    pub fn set_attribute(&mut self, name: String, attribute: MeshAttribute) -> MeshResult<()> {
        if !self.attributes.contains_key(&name) && self.attributes.len()>=64 {return Err(MeshKernelError::InvalidInput("mesh attribute declaration limit exceeded".into()));}
        semio_framework_mesh_engine::validate_mesh_attribute(&name,&attribute,self.vertex_count(),self.face_count(),self.halfedge_count(),&self.materials).map_err(MeshKernelError::InvalidInput)?;
        self.attributes.insert(name, attribute);
        Ok(())
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

    /// 🔄 Drains selected winding work through the shared retained orientation owner.
    pub fn flip_faces(&mut self,faces:&[FaceId])->MeshResult<()> {*self=self.flip_faces_job(faces)?.finish_with_progress(|_|true)?;Ok(())}

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
        self.require_topology_source_remap("topology reconstruction")?;
        let mut mesh=Self::from_faces(positions,faces)?;mesh.materials=std::mem::take(&mut self.materials);mesh.textures=std::mem::take(&mut self.textures);*self=mesh;
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
    /// 🧊 Drains the canonical retained box constructor.
    pub fn box_prim(width:f32,height:f32,depth:f32)->MeshResult<Self> {Self::box_primitive_job(width,height,depth)?.finish_with_progress(|_|true)}
    /// 🔳 Drains the canonical retained plane constructor.
    pub fn plane_prim(width:f32,depth:f32)->MeshResult<Self> {Self::plane_primitive_job(width,depth)?.finish_with_progress(|_|true)}
    /// 🥫 Drains the canonical retained cylinder constructor.
    pub fn cylinder_prim(radius:f32,height:f32,segments:u32)->MeshResult<Self> {Self::cylinder_primitive_job(radius,height,segments)?.finish_with_progress(|_|true)}
    /// 📐 Drains the canonical retained cone constructor.
    pub fn cone_prim(radius:f32,height:f32,segments:u32)->MeshResult<Self> {Self::cone_primitive_job(radius,height,segments)?.finish_with_progress(|_|true)}
    /// 🌐 Drains the canonical retained spherical subdivision constructor.
    pub fn ico_sphere_prim(radius:f32,subdivisions:u32)->MeshResult<Self> {Self::sphere_primitive_job(radius,subdivisions)?.finish_with_progress(|_|true)}

}

//#endregion Primitives

//#region Transform

impl HalfedgeMesh {
    pub fn translate(&mut self, delta: Vec3) -> MeshResult<()> {
        let mesh=self.translate_job(delta)?.finish_with_progress(|_|true)?;*self=mesh;Ok(())
    }

    pub fn rotate(&mut self, axis: Vec3, angle_rad: f32) -> MeshResult<()> {
        let mesh=self.rotate_job(axis,angle_rad)?.finish_with_progress(|_|true)?;*self=mesh;Ok(())
    }

    pub fn scale(&mut self, factor: Vec3) -> MeshResult<()> {
        let mesh=self.scale_job(factor,false)?.finish_with_progress(|_|true)?;*self=mesh;Ok(())
    }

    pub fn move_vertices(&mut self, verts: &[VertexId], delta: Vec3) -> MeshResult<()> {
        if delta.0.iter().any(|value| !value.is_finite()) { return Err(MeshKernelError::DegenerateOperation); }
        self.map_vertices(verts, None, |point| std::array::from_fn(|axis| point[axis] + delta.0[axis] as f64))
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
        self.map_vertices(verts, Some([[t*x*x+c,t*x*y-s*z,t*x*z+s*y],[t*x*y+s*z,t*y*y+c,t*y*z-s*x],[t*x*z-s*y,t*y*z+s*x,t*z*z+c]]), |point| {
            let [px, py, pz] = std::array::from_fn::<_, 3, _>(|axis| point[axis] - pivot[axis]);
            let rx = (t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz;
            let ry = (t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz;
            let rz = (t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz;
            [pivot[0] + rx, pivot[1] + ry, pivot[2] + rz]
        })
    }

    pub fn scale_vertices(&mut self, verts: &[VertexId], factor: Vec3, pivot: Vec3) -> MeshResult<()> {
        if factor.0.iter().chain(pivot.0.iter()).any(|value| !value.is_finite()) || factor.0.contains(&0.0) { return Err(MeshKernelError::DegenerateOperation); }
        self.map_vertices(verts, Some([[1.0/factor.x() as f64,0.0,0.0],[0.0,1.0/factor.y() as f64,0.0],[0.0,0.0,1.0/factor.z() as f64]]), |point| std::array::from_fn(|axis| pivot.0[axis] as f64 + (point[axis] - pivot.0[axis] as f64) * factor.0[axis] as f64))
    }

    fn map_vertices(&mut self, verts: &[VertexId], normal_matrix:Option<[[f64;3];3]>, transform: impl Fn([f64; 3]) -> [f64; 3]) -> MeshResult<()> {
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
        let mut normal_channels=std::collections::BTreeMap::new();
        if let Some(matrix)=normal_matrix {
            for (name,attribute) in &self.attributes {
                if attribute.semantic!=MeshAttributeSemantic::Normal {continue;}
                let mut values=Vec::new();let mut indices=Vec::new();let mut samples=std::collections::HashMap::new();
                for domain in 0..attribute.domain_len() {
                    let vertex=if attribute.domain==MeshAttributeDomain::Corner {self.halfedges[domain].vertex as usize}else {domain};
                    let affected=ids.binary_search(&vertex).is_ok();let source=attribute.indices.as_ref().map_or(domain as u32,|indices|indices[domain]);
                    let sample=if let Some(sample)=samples.get(&(source,affected)) {*sample}else {
                        let original=attribute.values[source as usize].as_array().unwrap();let mut normal=[0,1,2].map(|axis|original[axis].as_f64().unwrap());
                        if affected {normal=[0,1,2].map(|axis|matrix[axis][0]*normal[0]+matrix[axis][1]*normal[1]+matrix[axis][2]*normal[2]);let length=normal[0].hypot(normal[1]).hypot(normal[2]);normal=normal.map(|value|value/length);if normal.iter().any(|value|!value.is_finite() || !(*value as f32).is_finite()) {return Err(MeshKernelError::DegenerateOperation);}}
                        let sample=values.len() as u32;values.push(protocol::value::DslValue::Array(normal.into_iter().map(protocol::value::DslValue::float).collect()));samples.insert((source,affected),sample);sample
                    };indices.push(sample);
                }
                normal_channels.insert(name.clone(),MeshAttribute {domain:attribute.domain,semantic:attribute.semantic,interpolation:attribute.interpolation,values,indices:Some(indices)});
            }
        }
        for (id, position) in ids.into_iter().zip(positions) { self.vertices[id].position = position; }
        self.attributes.extend(normal_channels);
        self.recompute_normals()
    }

    /// 🌊 Moves the selection fully and nearby vertices with linear radial falloff.
    pub fn move_vertices_proportional(&mut self,verts:&[VertexId],delta:Vec3,pivot:Vec3,radius:f32)->MeshResult<()> {
        *self=self.move_proportional_job(verts.to_vec(),delta,pivot,radius)?.finish_with_progress(|_|true)?;Ok(())
    }

    /// 🧲️ Applies selected grid rounding through the retained transform owner.
    pub fn snap_vertices_to_grid(&mut self,verts:&[VertexId],grid:f32)->MeshResult<()> {
        *self=self.snap_vertices_job(verts.to_vec(),grid)?.finish_with_progress(|_|true)?;Ok(())
    }

}

//#endregion Transform

//#region Edit

impl HalfedgeMesh {
    pub fn extrude_faces(&mut self, faces: &[FaceId], distance: f32) -> MeshResult<()> {
        *self=self.extrude_faces_job(faces,distance)?.finish_with_progress(|_|true)?;Ok(())
    }

    pub fn inset_faces(&mut self, faces: &[FaceId], amount: f32) -> MeshResult<()> {
        *self=self.inset_faces_job(faces,amount)?.finish_with_progress(|_|true)?;Ok(())
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
    pub fn loop_cut(&mut self,edges:&[EdgeId],cuts:u32)->MeshResult<()> {
        *self=self.loop_cut_job(edges,cuts)?.finish_with_progress(|_|true)?;Ok(())
    }

    /// ✂️ Splits a face by a projected line and reuses boundary intersections in neighboring polygons.
    pub fn knife_cut(&mut self,face:FaceId,cut_a:Vec3,cut_b:Vec3)->MeshResult<()> {
        *self=self.knife_cut_job(face,cut_a,cut_b)?.finish_with_progress(|_|true)?;Ok(())
    }

    pub fn merge_vertices(&mut self, verts: &[VertexId], mode: WeldMode, threshold: f32) -> MeshResult<()> {
        *self = self.merge_vertices_job(verts,mode,threshold)?.finish_with_progress(|_| true)?;
        Ok(())
    }

    /// ✂️ Drains the retained explicit-edge merge without changing the source on refusal.
    pub fn dissolve_edges(&mut self,edges:&[EdgeId])->MeshResult<()> {*self=self.dissolve_edges_job(edges)?.finish_with_progress(|_|true)?;Ok(())}

    /// 🧵 Drains retained planar-star dissolution, preserving source ownership on refusal.
    pub fn dissolve_vertices(&mut self,vertices:&[VertexId])->MeshResult<()> {*self=self.dissolve_vertices_job(vertices)?.finish_with_progress(|_|true)?;Ok(())}

    fn require_topology_source_remap(&self,operation:&str)->MeshResult<()> {if let Some((name,_))=self.attributes.first_key_value() {return Err(MeshKernelError::InvalidInput(format!("mesh attribute '{name}' requires declared source remapping for {operation}")));}if !self.uv_seams.is_empty() {return Err(MeshKernelError::InvalidInput(format!("mesh UV seams require declared source remapping for {operation}")));}Ok(())}

    /// 🧹️ Preserves source channels while removing selected face domains.
    pub fn delete_faces(&mut self,faces:&[FaceId])->MeshResult<()> {*self=self.delete_faces_job(faces)?.finish_with_progress(|_|true)?;Ok(())}

    pub fn subdivide_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        *self = self.subdivide_faces_job(faces)?.finish_with_progress(|_| true)?;
        Ok(())
    }

    pub fn triangulate(&mut self)->MeshResult<()> {
        *self=self.triangulate_job()?.finish_with_progress(|_|true)?;Ok(())
    }

    pub fn merge_coplanar_faces(&mut self) -> MeshResult<usize> {
        let mut job=self.merge_coplanar_faces_job()?;loop {match job.step(256)? {MeshModelingStep::Done(mesh)=>{let count=job.coplanar_merge_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh coplanar merge cancelled".into()))}}
    }

    /// Unifies every group of vertices at (nearly) the same position — as commonly produced by importers
    /// that tessellate adjacent source faces independently, leaving duplicate, non-shared vertex ids along
    /// shared boundaries — into a single vertex id per position, so the halfedge topology (twins, boundary
    /// detection) reflects the true geometric connectivity. Returns the number of vertices removed.
    pub fn weld_coincident_vertices(&mut self,precision:f32)->MeshResult<usize> {
        let mut job=self.weld_coincident_vertices_job(precision)?;loop {match job.step(256)? {MeshModelingStep::Done(mesh)=>{let count=job.welded_vertex_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh weld cancelled".into()))}}
    }

    /// Flips faces so every undirected edge is traversed in opposite directions by its two incident faces.
    /// CAD imports often leave inconsistently oriented face wires; without this pass, halfedge twins are
    /// missing even though the undirected mesh is closed. Returns the number of faces flipped.
    pub fn orient_faces_consistently(&mut self) -> MeshResult<usize> {
        let mut job=self.orient_faces_job()?;loop {match job.step(256)? {MeshModelingStep::Done(mesh)=>{let count=job.orientation_flip_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh orientation cancelled".into()))}}
    }

    /// Finds every closed boundary loop (a chain of edges with no opposite face on the other side) in the
    /// current halfedge topology and caps each with a new n-gon face, so the mesh becomes watertight. Call
    /// `weld_coincident_vertices` first if the mesh may contain importer-duplicated boundary vertices, or
    /// this will also "cap" seams that are actually already shared with a differently-indexed neighbor.
    /// Returns the number of holes filled.
    pub fn fill_holes(&mut self) -> MeshResult<usize> {
        let mut job=self.fill_holes_job()?;loop {match job.step(256)? {MeshModelingStep::Done(mesh)=>{let count=job.filled_hole_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh hole filling cancelled".into()))}}
    }

    /// 🪞 Reflects geometry on one side of the axis plane and welds its seam.
    pub fn mirror(&mut self, axis: MirrorAxis, weld_threshold: f32) -> MeshResult<()> {
        *self = self.mirror_job(axis,weld_threshold)?.finish_with_progress(|_| true)?;
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

fn find_edge_position(vertices:&[u32],from:u32,to:u32)->Option<usize> {(0..vertices.len()).find(|&index|vertices[index]==from && vertices[(index+1)%vertices.len()]==to)}

fn merge_face_loops(a:&[u32],b:&[u32])->Option<Vec<u32>> {
    if a.len()<3 || b.len()<3 {return None;}
    let ai=(0..a.len()).find(|&index|find_edge_position(b,a[(index+1)%a.len()],a[index]).is_some())?;let bi=find_edge_position(b,a[(ai+1)%a.len()],a[ai])?;
    if (0..a.len()).any(|index|index!=ai && find_edge_position(b,a[(index+1)%a.len()],a[index]).is_some()) {return None;}
    let mut merged=vec![a[ai]];merged.extend((2..b.len()).map(|index|b[(bi+index)%b.len()]));merged.extend((1..a.len()).map(|index|a[(ai+index)%a.len()]));let mut seen=HashSet::new();if merged.iter().any(|&id|!seen.insert(id)) {None}else {Some(merged)}
}

//#endregion Polygon

//#region Export

/// ⏱️ Retained corner, ear-test and preview emission cursor; each unit visits at most one corner.
pub struct MeshTessellationJob {
    mesh: HalfedgeMesh,
    output: Option<MeshTransfer>,
    face: usize,
    phase: u8,
    cursor: usize,
    start: u32,
    next_he: u32,
    hes: Vec<u32>,
    points: Vec<Vec3>,
    normal_sum: [f64; 3],
    normal: Vec3,
    basis: (Vec3f64, Vec3f64),
    projected: Vec<(f64, f64)>,
    scale: f64,
    area: f64,
    links: Vec<(usize, usize)>,
    head: usize,
    remaining: usize,
    ear: usize,
    probe: usize,
    candidates: usize,
    base: u32,
    edge_seen: HashSet<(u32, u32)>,
    done: usize,
    cancelled: bool,
    maximum_preview_bytes: usize,
    selected_faces: Option<HashSet<u32>>,
    corner_ids:Vec<u32>,
    attribute_name:Option<String>,
    attribute_indices:Vec<u32>,
    attribute_cursor:usize,
    metadata_done:bool,
}

/// 🧵️ Tessellation yields retain ownership; completed buffers transfer once.
pub enum MeshTessellationStep { Working(MeshModelingProgress), Done(MeshTransfer), Cancelled(MeshModelingProgress) }

impl MeshTessellationJob {
    pub fn new(mesh: HalfedgeMesh) -> Self {
        Self { mesh, output: Some(MeshTransfer { colors:Vec::new(),attributes:Default::default(),materials:Default::default(),textures:Default::default(),positions: Vec::new(), normals: Vec::new(), indices: Vec::new(), edge_positions: Vec::new(), face_ids: Vec::new(), vertex_ids: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), edge_uvs: Vec::new(), edge_is_seam: Vec::new() }), face: 0, phase: 0, cursor: 0, start: 0, next_he: 0, hes: Vec::new(), points: Vec::new(), normal_sum: [0.0; 3], normal: Vec3::ZERO, basis: ((0.0,0.0,0.0),(0.0,0.0,0.0)), projected: Vec::new(), scale: 0.0, area: 0.0, links: Vec::new(), head: 0, remaining: 0, ear: 0, probe: 0, candidates: 0, base: 0, edge_seen: HashSet::new(), done: 0, cancelled: false, maximum_preview_bytes: usize::MAX, selected_faces: None,corner_ids:Vec::new(),attribute_name:None,attribute_indices:Vec::new(),attribute_cursor:0,metadata_done:false }
    }
    pub fn with_preview_capacity(mesh: HalfedgeMesh, maximum_preview_bytes: usize) -> Self { let mut job = Self::new(mesh); job.maximum_preview_bytes = maximum_preview_bytes; job }
    pub(super) fn selected(mesh: HalfedgeMesh, faces: HashSet<u32>) -> Self { let mut job = Self::new(mesh); job.selected_faces = Some(faces); job }
    /// 🧹️ Transfers the existing tessellation payload to its shared typed retirement authority.
    pub fn into_retirement(self)->Box<dyn protocol::value::ErasedSnapshotRetirement> {
        protocol::value::retirement::owned_retirement(self)
    }
    pub fn source(&self) -> &HalfedgeMesh { &self.mesh }
    pub(super) fn into_source(self)->HalfedgeMesh { self.mesh }
    pub fn progress(&self) -> MeshModelingProgress { MeshModelingProgress { units_done: self.done, units_total: self.done.saturating_add(self.mesh.face_count().saturating_sub(self.face)), phase: if self.face==self.mesh.face_count() {"tessellate-attributes"} else {"tessellate"} } }
    pub fn cancel(&mut self) { self.cancelled = true; self.output = None; }
    fn halfedge(&self, local: usize) -> u32 { self.hes[if self.mesh.faces[self.face].flipped { self.hes.len() - 1 - local } else { local }] }
    fn corner_uv(&self,id:u32)->[f64;2] {
        let edge=&self.mesh.halfedges[id as usize];
        self.mesh.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Uv).and_then(|attribute|attribute.value_at(match attribute.domain {MeshAttributeDomain::Corner=>id as usize,MeshAttributeDomain::Face=>self.face,MeshAttributeDomain::Vertex=>edge.vertex as usize,MeshAttributeDomain::Edge=>id as usize})).and_then(protocol::value::DslValue::as_array).map_or(edge.uv.map(f64::from),|value|[0,1].map(|axis|value[axis].as_f64().unwrap()))
    }
    fn corner(&mut self, local: usize) {
        let he = &self.mesh.halfedges[self.halfedge(local) as usize];
        let vertex = &self.mesh.vertices[he.vertex as usize];
        let sample=|semantic|self.mesh.attributes.values().find(|attribute|attribute.semantic==semantic).and_then(|attribute|attribute.value_at(match attribute.domain {MeshAttributeDomain::Corner=>self.halfedge(local) as usize,MeshAttributeDomain::Face=>self.face,MeshAttributeDomain::Vertex=>he.vertex as usize,MeshAttributeDomain::Edge=>self.halfedge(local) as usize})).and_then(protocol::value::DslValue::as_array);
        let normal=sample(MeshAttributeSemantic::Normal).map(|value|Vec3([0,1,2].map(|axis|value[axis].as_f64().unwrap() as f32))).unwrap_or_else(||if self.mesh.faces[self.face].smooth {vertex.normal.map(Vec3).unwrap_or(self.normal)}else {self.normal});
        let uv=sample(MeshAttributeSemantic::Uv).map(|value|[0,1].map(|axis|value[axis].as_f64().unwrap() as f32)).unwrap_or(he.uv);
        let color=sample(MeshAttributeSemantic::Color).map(|value|[0,1,2,3].map(|axis|value[axis].as_f64().unwrap() as f32));
        self.corner_ids.push(self.halfedge(local));
        let out = self.output.as_mut().unwrap();
        out.positions.extend_from_slice(&vertex.position); out.normals.extend_from_slice(&normal.0); out.vertex_ids.push(he.vertex); out.uvs.extend_from_slice(&uv);if let Some(color)=color {out.colors.extend_from_slice(&color);}
    }
    fn triangle(&mut self, triangle: [usize; 3]) {
        let indices = if self.mesh.faces[self.face].smooth { triangle.map(|local| self.base + local as u32) } else {
            let base = self.output.as_ref().unwrap().positions.len() as u32 / 3;
            for local in triangle { self.corner(local); }
            [base, base + 1, base + 2]
        };
        let out = self.output.as_mut().unwrap(); out.indices.extend_from_slice(&indices); out.face_ids.push(self.face as u32);
    }
    fn advance(&mut self) -> MeshResult<()> {
        let n = self.hes.len();
        match self.phase {
            0 => {
                if self.selected_faces.as_ref().is_some_and(|faces| !faces.contains(&(self.face as u32))) { self.face += 1; return Ok(()); }
                self.start = self.mesh.faces[self.face].halfedge; self.next_he = self.start; self.cursor = 0; self.phase = 1;
            }
            1 => {
                self.hes.push(self.next_he); self.next_he = self.mesh.halfedges[self.next_he as usize].next;
                if self.hes.len() > self.mesh.halfedges.len() { return Err(MeshKernelError::InvalidHandle); }
                if self.next_he == self.start { self.phase = 2; self.cursor = 0; }
            }
            2 => {
                self.points.push(Vec3(self.mesh.vertices[self.mesh.halfedges[self.halfedge(self.cursor) as usize].vertex as usize].position));
                self.cursor += 1; if self.cursor == n { self.phase = 3; self.cursor = 0; }
            }
            3 => {
                let origin = self.points[0].0.map(f64::from);
                let a = self.points[self.cursor].0.map(f64::from); let b = self.points[(self.cursor + 1) % n].0.map(f64::from);
                let a = [a[0]-origin[0],a[1]-origin[1],a[2]-origin[2]]; let b = [b[0]-origin[0],b[1]-origin[1],b[2]-origin[2]];
                for (axis, (u,v)) in [(1,2),(2,0),(0,1)].into_iter().enumerate() { self.normal_sum[axis] += (a[u]-b[u])*(a[v]+b[v]); }
                self.cursor += 1;
                if self.cursor == n {
                    let length = self.normal_sum[0].hypot(self.normal_sum[1]).hypot(self.normal_sum[2]);
                    self.normal = if length == 0.0 { Vec3::ZERO } else { Vec3(self.normal_sum.map(|value| (value/length) as f32)) };
                    self.basis = plane_basis(normalize3((self.normal.x() as f64,self.normal.y() as f64,self.normal.z() as f64)));
                    self.normal = self.normal.normalize();
                    self.phase = 4; self.cursor = 0;
                }
            }
            4 => {
                let origin = self.points[0]; let point = self.points[self.cursor]; let local = sub3((point.x() as f64,point.y() as f64,point.z() as f64),(origin.x() as f64,origin.y() as f64,origin.z() as f64));
                let projected = (dot3(local,self.basis.0),dot3(local,self.basis.1)); self.scale = self.scale.max(projected.0.abs()).max(projected.1.abs()); self.projected.push(projected);
                self.links.push(((self.cursor+n-1)%n,(self.cursor+1)%n)); self.cursor += 1;
                if self.cursor == n { self.phase = 5; self.cursor = 0; self.remaining = n; }
            }
            5 => {
                if self.scale != 0.0 { self.projected[self.cursor].0 /= self.scale; self.projected[self.cursor].1 /= self.scale; }
                self.cursor += 1; if self.cursor == n { self.phase = 6; self.cursor = 0; }
            }
            6 => {
                let a = self.projected[self.cursor]; let b = self.projected[(self.cursor+1)%n]; self.area += a.0*b.1-b.0*a.1;
                self.cursor += 1;
                if self.cursor == n { self.phase = 7; self.cursor = 0; self.base = self.output.as_ref().unwrap().positions.len() as u32 / 3; }
            }
            7 => {
                if self.mesh.faces[self.face].smooth { self.corner(self.cursor); self.cursor += 1; }
                if !self.mesh.faces[self.face].smooth || self.cursor == n { self.phase = if n == 3 || self.normal.length() < 1e-8 || self.scale == 0.0 || self.area.abs() < 1e-14 { 10 } else { 8 }; self.ear = self.head; self.cursor = self.links[self.head].1; }
            }
            8 => {
                if self.remaining == 3 { self.triangle([self.head,self.links[self.head].1,self.links[self.head].0]); self.phase = 11; self.cursor = 0; }
                else {
                    let (prev,next) = self.links[self.ear]; let cross = cross2(self.projected[prev],self.projected[self.ear],self.projected[next]);
                    if if self.area > 0.0 { cross > 1e-14 } else { cross < -1e-14 } { self.probe = self.head; self.phase = 9; }
                    else { self.reject_ear(); }
                }
            }
            9 => {
                let (prev,next) = self.links[self.ear]; let k = self.probe;
                if k != prev && k != self.ear && k != next && point_in_triangle(self.projected[k],self.projected[prev],self.projected[self.ear],self.projected[next]) { self.reject_ear(); }
                else {
                    self.probe = self.links[k].1;
                    if self.probe == self.head {
                        self.triangle([prev,self.ear,next]); self.links[prev].1 = next; self.links[next].0 = prev;
                        if self.ear == self.head { self.head = next; }
                        self.remaining -= 1; self.ear = self.head; self.candidates = 0; self.phase = 8;
                    }
                }
            }
            10 => {
                let next = self.links[self.cursor].1; self.triangle([self.head,self.cursor,next]); self.cursor = next;
                if self.links[self.cursor].1 == self.head { self.phase = 11; self.cursor = 0; }
            }
            11 => {
                let he_id = self.hes[self.cursor]; let he = &self.mesh.halfedges[he_id as usize]; let next = &self.mesh.halfedges[he.next as usize];
                let key = (he.vertex.min(next.vertex),he.vertex.max(next.vertex));
                if self.edge_seen.insert(key) {
                    let (a,b)=(self.corner_uv(he_id),self.corner_uv(he.next));
                    let seam=self.mesh.uv_seams.contains(&he_id) || he.twin.is_some_and(|twin|self.mesh.uv_seams.contains(&twin) || a!=self.corner_uv(self.mesh.halfedges[twin as usize].next) || b!=self.corner_uv(twin));
                    let out = self.output.as_mut().unwrap(); out.edge_positions.extend_from_slice(&self.mesh.vertices[he.vertex as usize].position); out.edge_positions.extend_from_slice(&self.mesh.vertices[next.vertex as usize].position); out.edge_ids.push(he_id); out.edge_uvs.extend_from_slice(&a.map(|value|value as f32)); out.edge_uvs.extend_from_slice(&b.map(|value|value as f32)); out.edge_is_seam.push(u8::from(seam));
                }
                self.cursor += 1;
                if self.cursor == n {
                    self.face += 1; self.phase = 0; self.hes.clear(); self.points.clear(); self.projected.clear(); self.links.clear(); self.normal_sum = [0.0;3]; self.scale = 0.0; self.area = 0.0; self.head = 0; self.candidates = 0;
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }
    fn advance_attributes(&mut self)->MeshResult<()> {
        if self.selected_faces.is_some() {self.metadata_done=true;return Ok(());}
        let next=if let Some(name)=&self.attribute_name {self.mesh.attributes.range((std::ops::Bound::Included(name.clone()),std::ops::Bound::Unbounded)).next()}else {self.mesh.attributes.first_key_value()};
        let Some((name,attribute))=next else {let out=self.output.as_mut().unwrap();out.materials=std::mem::take(&mut self.mesh.materials);out.textures=std::mem::take(&mut self.mesh.textures);self.metadata_done=true;return Ok(());};
        if self.attribute_name.is_none() {self.attribute_name=Some(name.clone());}
        let out=self.output.as_ref().unwrap();
        let (domain,ids)=match attribute.domain {MeshAttributeDomain::Vertex=>(MeshAttributeDomain::Vertex,&out.vertex_ids),MeshAttributeDomain::Corner=>(MeshAttributeDomain::Vertex,&self.corner_ids),MeshAttributeDomain::Face=>(MeshAttributeDomain::Face,&out.face_ids),MeshAttributeDomain::Edge=>(MeshAttributeDomain::Edge,&out.edge_ids)};
        if self.attribute_cursor<ids.len() {let id=ids[self.attribute_cursor] as usize;let sample=attribute.indices.as_ref().map_or(id as u32,|indices|indices[id]);self.attribute_indices.push(sample);self.attribute_cursor+=1;}
        else {let name=self.attribute_name.take().unwrap();let mut attribute=self.mesh.attributes.remove(&name).unwrap();attribute.domain=domain;attribute.indices=Some(std::mem::take(&mut self.attribute_indices));self.output.as_mut().unwrap().attributes.insert(name,attribute);self.attribute_cursor=0;}
        Ok(())
    }
    fn reject_ear(&mut self) {
        self.candidates += 1; self.ear = self.links[self.ear].1;
        self.phase = if self.candidates == self.remaining { self.cursor = self.links[self.head].1; 10 } else { 8 };
    }
    pub fn step(&mut self, budget: usize) -> MeshResult<MeshTessellationStep> {
        if self.cancelled { return Ok(MeshTessellationStep::Cancelled(self.progress())); }
        if self.output.is_none() { return Err(MeshKernelError::InvalidInput("tessellation job is retired".into())); }
        for _ in 0..budget {
            if self.metadata_done {return Ok(MeshTessellationStep::Done(self.output.take().unwrap()));}
            let next = self.done.checked_add(1).ok_or_else(|| MeshKernelError::InvalidInput("tessellation progress overflow".into()))?; if self.face==self.mesh.face_count() {self.advance_attributes()?;}else {self.advance()?;} self.done = next;
            let out = self.output.as_ref().unwrap();
            let scalars = out.colors.len().saturating_add(out.attributes.values().map(|attribute|attribute.indices.as_ref().map_or(0,Vec::len)).sum::<usize>()).saturating_add(out.positions.len()).saturating_add(out.normals.len()).saturating_add(out.indices.len()).saturating_add(out.edge_positions.len()).saturating_add(out.face_ids.len()).saturating_add(out.vertex_ids.len()).saturating_add(out.edge_ids.len()).saturating_add(out.uvs.len()).saturating_add(out.edge_uvs.len());
            if scalars.saturating_mul(4).saturating_add(out.edge_is_seam.len()) > self.maximum_preview_bytes { return Err(MeshKernelError::InvalidInput(format!("mesh preview exceeds {} bytes",self.maximum_preview_bytes))); }
        }
        if budget > 0 && self.metadata_done { Ok(MeshTessellationStep::Done(self.output.take().unwrap())) } else { Ok(MeshTessellationStep::Working(self.progress())) }
    }
}

impl HalfedgeMesh {
    /// 🪢️ One boundary link for retained traversal without collecting a polygon.
    pub fn face_boundary(&self, face: FaceId) -> MeshResult<(EdgeId, bool)> { let face = self.faces.get(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?; Ok((EdgeId(face.halfedge), face.flipped)) }
    /// 🔗️ Constant-time corner identity and successor for a boundary cursor.
    pub fn boundary_corner(&self, edge: EdgeId) -> MeshResult<(VertexId, EdgeId)> { let edge = self.halfedges.get(edge.0 as usize).ok_or(MeshKernelError::InvalidHandle)?; Ok((VertexId(edge.vertex), EdgeId(edge.next))) }
    pub fn tessellate(&self) -> MeshResult<MeshTransfer> {
        let mut job=MeshTessellationJob::new(self.clone());
        loop {match job.step(4096)? {MeshTessellationStep::Done(transfer)=>return Ok(transfer),MeshTessellationStep::Working(_)=>{},MeshTessellationStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh tessellation cancelled".into()))}}
    }

    /// 🧵 Reads one authored UV sample through its canonical corner identity.
    pub fn corner_uv(&self,corner:EdgeId)->MeshResult<[f32;2]> {let edge=self.halfedges.get(corner.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;if let Some(attribute)=self.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Uv) {let id=match attribute.domain {MeshAttributeDomain::Vertex=>edge.vertex,MeshAttributeDomain::Face=>edge.face.ok_or(MeshKernelError::InvalidHandle)?,_=>corner.0};let value=attribute.value_at(id as usize).and_then(protocol::value::DslValue::as_array).ok_or(MeshKernelError::InvalidHandle)?;return Ok([value[0].as_f64().unwrap()as f32,value[1].as_f64().unwrap()as f32]);}Ok(edge.uv)}

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
        Ok(semio_framework_pack_json::to_json_string(self))
    }

    pub fn from_json(json: &str) -> MeshResult<Self> {
        semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| MeshKernelError::InvalidInput(e.to_string()))
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

protocol::value::artifact_retire_leaf!(VertexId,EdgeId,FaceId);
impl protocol::value::retirement::RetireOwned for MeshTessellationJob {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.mesh,self.output,self.hes,self.points,self.projected,self.links,self.edge_seen,self.selected_faces,self.corner_ids,self.attribute_name,self.attribute_indices]}
}
