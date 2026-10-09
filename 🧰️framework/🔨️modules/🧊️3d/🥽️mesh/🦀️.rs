//! 🔷️ Half-edge mesh kernel for low-poly editing. **Host authority:** `HalfedgeMesh` is a value
//! document/engine payload — not a process-global mesh store.

// 🔬️ `serde`/`serde_json` survive ONLY as a `#[cfg(test)]` differential oracle now that these
// types have their own first-party `ToValue`/`FromValue` codec — never a production dependency
// of this crate. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[cfg(test)]
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
#[cfg(test)]
use std::collections::HashSet;

#[path = "🛠️modeling/🦀️.rs"]
mod modeling;
pub use modeling::{MeshModelingJob, MeshModelingProgress, MeshModelingStep};

#[path = "🔎️quality/🦀️.rs"]
mod quality;
pub use quality::{analyze_polygon_soup, MeshBounds, MeshMassProperties, MeshQualityReport, ScalarStats};

#[path = "🎨️surface/🦀️.rs"]
mod surface;
pub use surface::MeshSurfaceJob;

/// 🌉️ Encodes original ordered UV membership as the declared array of vertex ids.
mod u32_membership_bridge {
    pub fn to_value(set: &super::HistoryFoldSet<u32>) -> dsl_core::value::DslValue {
        let mut ids: Vec<&u32> = set.iter().collect();
        ids.sort_unstable();
        dsl_core::value::DslValue::Array(ids.into_iter().map(dsl_core::value::ToValue::to_value).collect())
    }
    pub fn from_value(value: dsl_core::value::DslValue) -> Result<super::HistoryFoldSet<u32>, dsl_core::value::ValueError> {
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

pub use semio_framework_mesh_engine::{HistoryFoldIndex,HistoryFoldSet, MeshAttribute, MeshAttributeDomain, MeshAttributeSemantic, MeshAttributeInterpolation, MeshTexture};

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub enum MirrorAxis {
    X,
    Y,
    Z,
}

//#region ⚠️ Errors
/// ⚠️ Half-edge mesh kernel operation failure.
#[derive(Debug, Clone, PartialEq)]
pub enum MeshKernelError {
    InvalidHandle,
    NonManifold,
    DegenerateOperation,
    EmptySelection,
    InvalidInput(String),
    Retained(protocol::value::ValueError),
}

impl std::fmt::Display for MeshKernelError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHandle => formatter.write_str("invalid handle"),
            Self::NonManifold => formatter.write_str("mesh is non-manifold"),
            Self::DegenerateOperation => formatter.write_str("degenerate operation"),
            Self::EmptySelection => formatter.write_str("empty selection"),
            Self::InvalidInput(detail) => write!(formatter, "invalid input: {detail}"),
            Self::Retained(error) => std::fmt::Display::fmt(error,formatter),
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
    pub attributes: HistoryFoldIndex<String,MeshAttribute>,
    #[value(default)]
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    pub materials: HistoryFoldIndex<String,protocol::value::DslValue>,
    #[value(default)]
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    pub textures: HistoryFoldIndex<String,MeshTexture>,
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
    #[value(default, with = "u32_membership_bridge")]
    uv_seams: HistoryFoldSet<u32>,
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    #[value(default)]
    attributes: HistoryFoldIndex<String,MeshAttribute>,
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    #[value(default)]
    materials: HistoryFoldIndex<String,protocol::value::DslValue>,
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    #[value(default)]
    textures: HistoryFoldIndex<String,MeshTexture>,
}

protocol::value::artifact_retire_leaf!(Vec3);
impl protocol::value::retirement::RetireOwned for HalfedgeMesh {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::retirement::sequence(vec![protocol::value::retirement::deferred(self.vertices),protocol::value::retirement::deferred(self.halfedges),protocol::value::retirement::deferred(self.faces),protocol::value::retirement::deferred(self.uv_seams),protocol::value::retirement::deferred(self.attributes),protocol::value::retirement::deferred(self.materials),protocol::value::retirement::deferred(self.textures)])}
    fn retirement_birth_bytes(&self)->Option<usize> {protocol::value::retirement::sequence_birth_bytes(&[protocol::value::retirement::deferred_birth_bytes_for(&self.vertices),protocol::value::retirement::deferred_birth_bytes_for(&self.halfedges),protocol::value::retirement::deferred_birth_bytes_for(&self.faces),protocol::value::retirement::deferred_birth_bytes_for(&self.uv_seams),protocol::value::retirement::deferred_birth_bytes_for(&self.attributes),protocol::value::retirement::deferred_birth_bytes_for(&self.materials),protocol::value::retirement::deferred_birth_bytes_for(&self.textures)])}
    fn controlled_retirement_supported()->bool {true}
}
impl protocol::value::retirement::RetireOwned for MeshVertex {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::retirement::sequence(vec![protocol::value::retirement::deferred(self.position),protocol::value::retirement::deferred(self.normal),protocol::value::retirement::deferred(self.halfedge)])}
    fn retirement_birth_bytes(&self)->Option<usize> {protocol::value::retirement::sequence_birth_bytes(&[protocol::value::retirement::deferred_birth_bytes_for(&self.position),protocol::value::retirement::deferred_birth_bytes_for(&self.normal),protocol::value::retirement::deferred_birth_bytes_for(&self.halfedge)])}
    fn controlled_retirement_supported()->bool {true}
}
impl protocol::value::retirement::RetireOwned for HalfEdge {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::retirement::sequence(vec![protocol::value::retirement::deferred(self.vertex),protocol::value::retirement::deferred(self.twin),protocol::value::retirement::deferred(self.next),protocol::value::retirement::deferred(self.face),protocol::value::retirement::deferred(self.uv)])}
    fn retirement_birth_bytes(&self)->Option<usize> {protocol::value::retirement::sequence_birth_bytes(&[protocol::value::retirement::deferred_birth_bytes_for(&self.vertex),protocol::value::retirement::deferred_birth_bytes_for(&self.twin),protocol::value::retirement::deferred_birth_bytes_for(&self.next),protocol::value::retirement::deferred_birth_bytes_for(&self.face),protocol::value::retirement::deferred_birth_bytes_for(&self.uv)])}
    fn controlled_retirement_supported()->bool {true}
}
impl protocol::value::retirement::RetireOwned for MeshFace {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::retirement::sequence(vec![protocol::value::retirement::deferred(self.halfedge),protocol::value::retirement::deferred(self.smooth),protocol::value::retirement::deferred(self.flipped)])}
    fn retirement_birth_bytes(&self)->Option<usize> {protocol::value::retirement::sequence_birth_bytes(&[protocol::value::retirement::deferred_birth_bytes_for(&self.halfedge),protocol::value::retirement::deferred_birth_bytes_for(&self.smooth),protocol::value::retirement::deferred_birth_bytes_for(&self.flipped)])}
    fn controlled_retirement_supported()->bool {true}
}
impl protocol::value::retirement::RetireOwned for MeshTransfer {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::retirement::sequence(vec![protocol::value::retirement::deferred(self.positions),protocol::value::retirement::deferred(self.normals),protocol::value::retirement::deferred(self.colors),protocol::value::retirement::deferred(self.indices),protocol::value::retirement::deferred(self.uvs),protocol::value::retirement::deferred(self.face_ids),protocol::value::retirement::deferred(self.vertex_ids),protocol::value::retirement::deferred(self.edge_positions),protocol::value::retirement::deferred(self.edge_ids),protocol::value::retirement::deferred(self.edge_uvs),protocol::value::retirement::deferred(self.edge_is_seam),protocol::value::retirement::deferred(self.attributes),protocol::value::retirement::deferred(self.materials),protocol::value::retirement::deferred(self.textures)])}
    fn retirement_birth_bytes(&self)->Option<usize> {protocol::value::retirement::sequence_birth_bytes(&[protocol::value::retirement::deferred_birth_bytes_for(&self.positions),protocol::value::retirement::deferred_birth_bytes_for(&self.normals),protocol::value::retirement::deferred_birth_bytes_for(&self.colors),protocol::value::retirement::deferred_birth_bytes_for(&self.indices),protocol::value::retirement::deferred_birth_bytes_for(&self.uvs),protocol::value::retirement::deferred_birth_bytes_for(&self.face_ids),protocol::value::retirement::deferred_birth_bytes_for(&self.vertex_ids),protocol::value::retirement::deferred_birth_bytes_for(&self.edge_positions),protocol::value::retirement::deferred_birth_bytes_for(&self.edge_ids),protocol::value::retirement::deferred_birth_bytes_for(&self.edge_uvs),protocol::value::retirement::deferred_birth_bytes_for(&self.edge_is_seam),protocol::value::retirement::deferred_birth_bytes_for(&self.attributes),protocol::value::retirement::deferred_birth_bytes_for(&self.materials),protocol::value::retirement::deferred_birth_bytes_for(&self.textures)])}
    fn controlled_retirement_supported()->bool {true}
}

impl HalfedgeMesh {
    /// 🧩️ Transfer all seven kernel owners directly to an individually typed persisted model.
    pub fn into_owned_parts(self)->(Vec<MeshVertex>,Vec<HalfEdge>,Vec<MeshFace>,HistoryFoldSet<u32>,HistoryFoldIndex<String,MeshAttribute>,HistoryFoldIndex<String,protocol::value::DslValue>,HistoryFoldIndex<String,MeshTexture>){(self.vertices,self.halfedges,self.faces,self.uv_seams,self.attributes,self.materials,self.textures)}
    /// 🥽️ Editable literal topology enters the kernel without a serialized mesh document.
    pub fn from_owned_parts(vertices:Vec<MeshVertex>,halfedges:Vec<HalfEdge>,faces:Vec<MeshFace>,uv_seams:HistoryFoldSet<u32>,attributes:HistoryFoldIndex<String,MeshAttribute>,materials:HistoryFoldIndex<String,protocol::value::DslValue>,textures:HistoryFoldIndex<String,MeshTexture>)->Self{Self{vertices,halfedges,faces,uv_seams,attributes,materials,textures}}
    pub fn empty() -> Self {
        Self { vertices: Vec::new(), halfedges: Vec::new(), faces: Vec::new(), uv_seams: HistoryFoldSet::new(), attributes: HistoryFoldIndex::new(), materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new() }
    }

    /// 🏷️ Authored channels are read from the same mesh value consumed by retained jobs.
    pub fn attributes(&self) -> &HistoryFoldIndex<String,MeshAttribute> { &self.attributes }

    /// 🎨️ Material references and texture bytes belong to this same owned mesh payload.
    pub fn materials(&self)->&HistoryFoldIndex<String,protocol::value::DslValue> { &self.materials }
    /// 🖼️ Authored texture records are retained without external resource ownership.
    pub fn textures(&self)->&HistoryFoldIndex<String,MeshTexture> { &self.textures }

    /// 📦️ Admits material and texture records before dependent face channels.
    pub fn set_surface_assets(&mut self,materials:HistoryFoldIndex<String,protocol::value::DslValue>,textures:HistoryFoldIndex<String,MeshTexture>)->MeshResult<()> {
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

    /// 📍️ Every vertex position in id order.
    pub fn positions(&self) -> Vec<[f32; 3]> {
        self.vertices.iter().map(|vertex| vertex.position).collect()
    }

    /// 🔷️ Every face as its wound vertex-index loop in id order, flipped faces already reversed.
    pub fn polygons(&self) -> Vec<Vec<u32>> {
        (0..self.faces.len() as u32).filter_map(|face| self.face_vertex_ids(FaceId(face)).ok()).map(|loop_ids| loop_ids.into_iter().map(|vertex| vertex.0).collect()).collect()
    }

    /// 🔗️ One canonical id per undirected edge (the lower half-edge of a twin pair), in id order.
    pub fn edge_ids(&self) -> Vec<EdgeId> {
        self.halfedges.iter().enumerate().filter(|(index, edge)| edge.twin.is_none_or(|twin| *index < twin as usize)).map(|(index, _)| EdgeId(index as u32)).collect()
    }

    /// 🧭️ The stored unit normal of one vertex.
    pub fn vertex_normal(&self, id: VertexId) -> MeshResult<Vec3> {
        self.vertices.get(id.0 as usize).ok_or(MeshKernelError::InvalidHandle)?.normal.map(Vec3).ok_or(MeshKernelError::DegenerateOperation)
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
        let mut job=self.merge_coplanar_faces_job()?;loop {match job.step(256,protocol::value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut protocol::value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.coplanar_merge_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh coplanar merge cancelled".into()))}}
    }

    /// Unifies every group of vertices at (nearly) the same position — as commonly produced by importers
    /// that tessellate adjacent source faces independently, leaving duplicate, non-shared vertex ids along
    /// shared boundaries — into a single vertex id per position, so the halfedge topology (twins, boundary
    /// detection) reflects the true geometric connectivity. Returns the number of vertices removed.
    pub fn weld_coincident_vertices(&mut self,precision:f32)->MeshResult<usize> {
        let mut job=self.weld_coincident_vertices_job(precision)?;loop {match job.step(256,protocol::value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut protocol::value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.welded_vertex_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh weld cancelled".into()))}}
    }

    /// Flips faces so every undirected edge is traversed in opposite directions by its two incident faces.
    /// CAD imports often leave inconsistently oriented face wires; without this pass, halfedge twins are
    /// missing even though the undirected mesh is closed. Returns the number of faces flipped.
    pub fn orient_faces_consistently(&mut self) -> MeshResult<usize> {
        let mut job=self.orient_faces_job()?;loop {match job.step(256,protocol::value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut protocol::value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.orientation_flip_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh orientation cancelled".into()))}}
    }

    /// Finds every closed boundary loop (a chain of edges with no opposite face on the other side) in the
    /// current halfedge topology and caps each with a new n-gon face, so the mesh becomes watertight. Call
    /// `weld_coincident_vertices` first if the mesh may contain importer-duplicated boundary vertices, or
    /// this will also "cap" seams that are actually already shared with a differently-indexed neighbor.
    /// Returns the number of holes filled.
    pub fn fill_holes(&mut self) -> MeshResult<usize> {
        let mut job=self.fill_holes_job()?;loop {match job.step(256,protocol::value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut protocol::value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.filled_hole_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh hole filling cancelled".into()))}}
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
        *self = self.set_shading_job(faces, smooth)?.finish_with_progress()?;
        Ok(())
    }

    pub fn recompute_normals(&mut self) -> MeshResult<()> {
        let mut flat = vec![None; self.vertex_count()];
        let mut sums = vec![Vec3::ZERO; self.vertex_count()];
        self.accumulate_vertex_normals(0..self.faces.len(), &mut flat, &mut sums)?;
        self.apply_vertex_normals(0..self.vertices.len(), &flat, &sums);
        Ok(())
    }

}

//#endregion Edit

//#region Uv

impl HalfedgeMesh {
    /// 🪡️ Marks or clears the seams on the edges (an edge and its twin are one seam); handles that name no edge are ignored.
    pub fn mark_uv_seam(&mut self, edges: &[EdgeId], seam: bool) {
        for &edge in edges {
            if (edge.0 as usize) < self.halfedges.len() {
                self.set_seam_pair(edge, seam);
            }
        }
    }

    pub fn is_uv_seam(&self, edge: EdgeId) -> bool {
        (edge.0 as usize) < self.halfedges.len() && self.is_seam_pair(edge.0)
    }

    /// 🗺️ Unwraps the surface along its seams into packed texture coordinates; see [`Self::unwrap_uv_job`].
    pub fn unwrap_uv(&mut self) -> MeshResult<()> {
        *self = self.unwrap_uv_job()?.finish_with_progress()?;
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
    let mut merged=vec![a[ai]];merged.extend((2..b.len()).map(|index|b[(bi+index)%b.len()]));merged.extend((1..a.len()).map(|index|a[(ai+index)%a.len()]));let mut seen=HistoryFoldSet::new();if merged.iter().any(|&id|!seen.insert(id)) {None}else {Some(merged)}
}

//#endregion Polygon

//#region Export

#[path="🧩️tessellation/🎟️reservation/🦀️.rs"]
mod tessellation_buffer_reservation;

#[path="🧩️tessellation/🎨️metadata/🦀️.rs"]
mod tessellation_metadata_capture;

#[path="🧩️tessellation/📤️corner/🦀️.rs"]
mod tessellation_corner_emission;

#[path="🧩️tessellation/📤️edge/🦀️.rs"]
mod tessellation_edge_emission;

#[path="🧩️tessellation/📤️triangle/🦀️.rs"]
mod tessellation_triangle_emission;

#[path="🧩️tessellation/🎨️projection/🦀️.rs"]
mod tessellation_attribute_projection;

#[path="🧩️tessellation/🧭️face/🦀️.rs"]
mod tessellation_face_preparation;

#[path="🧩️tessellation/⏱️normal/🦀️.rs"]
mod tessellation_normal_work;

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
    edge_seen: HistoryFoldSet<(u32, u32)>,
    done: usize,
    cancelled: bool,
    maximum_preview_bytes: usize,
    selected_faces: Option<HistoryFoldSet<u32>>,
    corner_ids:Vec<u32>,
    attribute_name:Option<String>,
    attribute_indices:Vec<u32>,
    attribute_cursor:usize,
    metadata_done:bool,
    buffer_reservation:u8,
    buffer_receipt:tessellation_buffer_reservation::MeshBufferReceipt,
    metadata_cursor:usize,
    semantic_slots:[Option<usize>;3],
    metadata_receipt:tessellation_metadata_capture::MeshMetadataReceipt,
    corner_receipt:tessellation_corner_emission::MeshCornerReceipt,
    edge_pending:Option<usize>,
    edge_receipt:tessellation_edge_emission::MeshEdgeReceipt,
    triangle_receipt:tessellation_triangle_emission::MeshTriangleReceipt,
    attribute_slot:usize,
    attribute_stage:u8,
    attribute_pending:Option<(String,MeshAttribute)>,
    attribute_original_indices:Option<protocol::value::retirement::controlled::ControlledRetirement<Option<Vec<u32>>>>,
    attribute_receipt:tessellation_attribute_projection::MeshAttributeReceipt,
    face_receipt:tessellation_face_preparation::MeshFaceReceipt,
    triangle_pending:Option<tessellation_normal_work::TriangleFrame>,
    normal_metadata_phase:u8,
    normal_output_bytes:usize,
    normal_receipt:tessellation_normal_work::MeshNormalReceipt,
}

/// 🧵️ Tessellation yields retain ownership; completed buffers transfer once.
pub enum MeshTessellationStep { Working(MeshModelingProgress), Done(MeshTransfer), Cancelled(MeshModelingProgress) }

impl MeshTessellationJob {
    pub fn new(mesh: HalfedgeMesh) -> Self {
        Self { mesh, output: Some(MeshTransfer { colors:Vec::new(),attributes:Default::default(),materials:Default::default(),textures:Default::default(),positions: Vec::new(), normals: Vec::new(), indices: Vec::new(), edge_positions: Vec::new(), face_ids: Vec::new(), vertex_ids: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), edge_uvs: Vec::new(), edge_is_seam: Vec::new() }), face: 0, phase: 0, cursor: 0, start: 0, next_he: 0, hes: Vec::new(), points: Vec::new(), normal_sum: [0.0; 3], normal: Vec3::ZERO, basis: ((0.0,0.0,0.0),(0.0,0.0,0.0)), projected: Vec::new(), scale: 0.0, area: 0.0, links: Vec::new(), head: 0, remaining: 0, ear: 0, probe: 0, candidates: 0, base: 0, edge_seen: HistoryFoldSet::new(), done: 0, cancelled: false, maximum_preview_bytes: usize::MAX, selected_faces: None,corner_ids:Vec::new(),attribute_name:None,attribute_indices:Vec::new(),attribute_cursor:0,metadata_done:false,buffer_reservation:0,buffer_receipt:Default::default(),metadata_cursor:0,semantic_slots:[None;3],metadata_receipt:Default::default(),corner_receipt:Default::default(),edge_pending:None,edge_receipt:Default::default(),triangle_receipt:Default::default(),attribute_slot:0,attribute_stage:0,attribute_pending:None,attribute_original_indices:None,attribute_receipt:Default::default(),face_receipt:Default::default(),triangle_pending:None,normal_metadata_phase:0,normal_output_bytes:0,normal_receipt:Default::default() }
    }
    pub fn with_preview_capacity(mesh: HalfedgeMesh, maximum_preview_bytes: usize) -> Self { let mut job = Self::new(mesh); job.maximum_preview_bytes = maximum_preview_bytes; job }
    pub(super) fn selected(mesh: HalfedgeMesh, faces: HistoryFoldSet<u32>) -> Self { let mut job = Self::new(mesh); job.selected_faces = Some(faces); job }
    /// 🧹️ Transfers the existing tessellation payload to its shared typed retirement authority.
    pub fn retirement_birth_bytes(&self)->usize {protocol::value::retirement::owned_retirement_birth_bytes::<Self>()}
    pub fn into_retirement(self,grant:protocol::value::retained_clone::RetainedCloneGrant)->Result<(Box<dyn protocol::value::ErasedSnapshotRetirement>,protocol::value::retained_clone::RetainedCloneProgress),(protocol::value::ValueError,Self)> {
        protocol::value::retirement::admit_owned_retirement(self,grant)
    }
    pub fn source(&self) -> &HalfedgeMesh { &self.mesh }
    pub(super) fn take_source(&mut self)->HalfedgeMesh { std::mem::take(&mut self.mesh) }
    pub fn progress(&self) -> MeshModelingProgress { MeshModelingProgress { units_done: self.done, units_total: self.done.saturating_add(self.mesh.face_count().saturating_sub(self.face)), phase: if self.face==self.mesh.face_count() {"tessellate-attributes"} else {"tessellate"} } }
    pub fn cancel(&mut self) { self.cancelled = true; }
    fn halfedge(&self, local: usize) -> u32 { self.hes[if self.mesh.faces[self.face].flipped { self.hes.len() - 1 - local } else { local }] }
    fn corner_uv(&self,id:u32)->[f64;2] {
        let edge=&self.mesh.halfedges[id as usize];
        self.mesh.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Uv).and_then(|attribute|attribute.value_at(match attribute.domain {MeshAttributeDomain::Corner=>id as usize,MeshAttributeDomain::Face=>self.face,MeshAttributeDomain::Vertex=>edge.vertex as usize,MeshAttributeDomain::Edge=>id as usize})).and_then(protocol::value::DslValue::as_array).map_or(edge.uv.map(f64::from),|value|[0,1].map(|axis|value[axis].as_f64().unwrap()))
    }
    fn corner(&mut self, local: usize) {
        let data=self.corner_data(local,[self.mesh.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Normal),self.mesh.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Uv),self.mesh.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Color)]);
        self.append_corner_data(data);
    }
    fn triangle(&mut self, triangle: [usize; 3]) {
        let indices = if self.mesh.faces[self.face].smooth { triangle.map(|local| self.base + local as u32) } else {
            let base = self.output.as_ref().unwrap().positions.len() as u32 / 3;
            for local in triangle { self.corner(local); }
            [base, base + 1, base + 2]
        };
        self.append_triangle_indices(indices);
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
                    let normal = if length == 0.0 { Vec3::ZERO } else { Vec3(self.normal_sum.map(|value| (value/length) as f32)) };
                    self.basis = plane_basis(normalize3((normal.x() as f64,normal.y() as f64,normal.z() as f64)));
                    self.normal = normal.normalize();
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
            8..=10 => {if let Some((finish,locals))=self.choose_triangle(){self.triangle(locals);self.finish_triangle(finish,locals);}}
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

}

impl HalfedgeMesh {
    /// 🪢️ One boundary link for retained traversal without collecting a polygon.
    pub fn face_boundary(&self, face: FaceId) -> MeshResult<(EdgeId, bool)> { let face = self.faces.get(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?; Ok((EdgeId(face.halfedge), face.flipped)) }
    /// 🔗️ Constant-time corner identity and successor for a boundary cursor.
    pub fn boundary_corner(&self, edge: EdgeId) -> MeshResult<(VertexId, EdgeId)> { let edge = self.halfedges.get(edge.0 as usize).ok_or(MeshKernelError::InvalidHandle)?; Ok((VertexId(edge.vertex), EdgeId(edge.next))) }
    /// 🥽️ The indexed polygon source of this mesh with authored channels re-keyed to canonical corner and edge identities; reconstructing it with `polygon_source_job` yields an equal mesh.
    pub fn polygon_source(&self) -> MeshResult<semio_framework_mesh_engine::PolygonMeshSource> {
        let mut corner_ids = Vec::new();
        let mut edge_ids = Vec::new();
        for face in 0..self.face_count() {
            let (start, flipped) = self.face_boundary(FaceId(face as u32))?;
            let mut halfedges = Vec::new();
            let mut next = start;
            loop {
                halfedges.push(next.0);
                next = self.boundary_corner(next)?.1;
                if next == start {
                    break;
                }
            }
            let count = halfedges.len();
            for cursor in 0..count {
                let index = if flipped { count - 1 - cursor } else { cursor };
                corner_ids.push(halfedges[index]);
                edge_ids.push(halfedges[if flipped { (index + count - 1) % count } else { index }]);
            }
        }
        let mut attributes = self.attributes.clone();
        for attribute in attributes.slot_values_mut() {
            let ids = match attribute.domain {
                MeshAttributeDomain::Corner => Some(&corner_ids),
                MeshAttributeDomain::Edge => Some(&edge_ids),
                _ => None,
            };
            if let Some(ids) = ids {
                if attribute.indices.is_some() || ids.iter().enumerate().any(|(index, id)| index != *id as usize) {
                    attribute.indices = Some(ids.iter().map(|id| attribute.indices.as_ref().map_or(*id, |indices| indices[*id as usize])).collect());
                }
            }
        }
        Ok(semio_framework_mesh_engine::PolygonMeshSource { vertices: self.positions(), faces: self.polygons(), attributes, materials: self.materials.clone(), textures: self.textures.clone() })
    }

    pub fn tessellate(&self) -> MeshResult<MeshTransfer> {
        let mut job=MeshTessellationJob::new(self.clone());
        loop {let grant=protocol::value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:128,maximum_capacity_bytes:job.next_normal_capacity_byte_demand(128).map_err(MeshKernelError::Retained)?,maximum_release_bytes:job.next_normal_release_byte_demand().map_err(MeshKernelError::Retained)?,maximum_depth:job.next_normal_depth_demand().map_err(MeshKernelError::Retained)?};match job.step(grant).map_err(MeshKernelError::Retained)?.0 {MeshTessellationStep::Done(transfer)=>return Ok(transfer),MeshTessellationStep::Working(_)=>{},MeshTessellationStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh tessellation cancelled".into()))}}
    }

    /// 🧵 Reads one authored UV sample through its canonical corner identity.
    pub fn corner_uv(&self,corner:EdgeId)->MeshResult<[f32;2]> {let edge=self.halfedges.get(corner.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;if let Some(attribute)=self.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Uv) {let id=match attribute.domain {MeshAttributeDomain::Vertex=>edge.vertex,MeshAttributeDomain::Face=>edge.face.ok_or(MeshKernelError::InvalidHandle)?,_=>corner.0};let value=attribute.value_at(id as usize).and_then(protocol::value::DslValue::as_array).ok_or(MeshKernelError::InvalidHandle)?;return Ok([value[0].as_f64().unwrap()as f32,value[1].as_f64().unwrap()as f32]);}Ok(edge.uv)}

    pub fn to_obj(&self) -> MeshResult<String> {
        let mut export = MeshObjExport::new();
        loop {
            if let Some(text) = export.step(self, 1)? {
                return Ok(text);
            }
        }
    }

    pub fn to_json(&self) -> MeshResult<String> {
        Ok(semio_framework_pack_json::to_json_string(self))
    }

    pub fn from_json(json: &str) -> MeshResult<Self> {
        semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| MeshKernelError::InvalidInput(e.to_string()))
    }
}

/// 📜️ Incremental OBJ writer: vertices, texture coordinates and faces are written in units of 256 elements so a host can stay inside an interactive step ceiling.
pub struct MeshObjExport {
    out: String,
    phase: u8,
    cursor: usize,
    has_uv: bool,
    units: usize,
}

impl Default for MeshObjExport {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshObjExport {
    pub fn new() -> Self {
        Self { out: String::from("# kernel_3d_mesh OBJ export\n"), phase: 0, cursor: 0, has_uv: false, units: 0 }
    }

    /// 📊 Completed units and the units the mesh needs in total (an upper bound until the texture scan finished).
    pub fn progress(&self, mesh: &HalfedgeMesh) -> (usize, usize) {
        let total = (mesh.vertices.len() + 2 * mesh.halfedges.len() + mesh.faces.len()).div_ceil(256) + 4;
        (self.units, total.max(self.units))
    }

    /// ⏱️ Writes at most `budget` units of 256 elements; the finished text once everything is written.
    pub fn step(&mut self, mesh: &HalfedgeMesh, budget: usize) -> MeshResult<Option<String>> {
        for _ in 0..budget.max(1) {
            self.units += 1;
            match self.phase {
                0 => {
                    let end = (self.cursor + 256).min(mesh.vertices.len());
                    for vertex in &mesh.vertices[self.cursor..end] {
                        self.out.push_str(&format!("v {} {} {}\n", vertex.position[0], vertex.position[1], vertex.position[2]));
                    }
                    self.cursor = end;
                    if self.cursor == mesh.vertices.len() {
                        self.phase = 1;
                        self.cursor = 0;
                    }
                }
                1 => {
                    let end = (self.cursor + 256).min(mesh.halfedges.len());
                    self.has_uv = mesh.halfedges[self.cursor..end].iter().any(|halfedge| halfedge.uv[0] != 0.0 || halfedge.uv[1] != 0.0);
                    if self.has_uv || end == mesh.halfedges.len() {
                        self.phase = if self.has_uv { 2 } else { 3 };
                        self.cursor = 0;
                    } else {
                        self.cursor = end;
                    }
                }
                2 => {
                    let end = (self.cursor + 256).min(mesh.halfedges.len());
                    for halfedge in &mesh.halfedges[self.cursor..end] {
                        self.out.push_str(&format!("vt {} {}\n", halfedge.uv[0], halfedge.uv[1]));
                    }
                    self.cursor = end;
                    if self.cursor == mesh.halfedges.len() {
                        self.phase = 3;
                        self.cursor = 0;
                    }
                }
                _ => {
                    let end = (self.cursor + 256).min(mesh.faces.len());
                    for face in self.cursor..end {
                        let mut halfedges = mesh.face_halfedge_ids(FaceId(face as u32))?;
                        if mesh.faces[face].flipped {
                            halfedges.reverse();
                        }
                        self.out.push('f');
                        for id in halfedges {
                            let vertex = mesh.halfedges[id as usize].vertex;
                            if self.has_uv {
                                self.out.push_str(&format!(" {}/{}", vertex + 1, id as usize + 1));
                            } else {
                                self.out.push_str(&format!(" {}", vertex + 1));
                            }
                        }
                        self.out.push('\n');
                    }
                    self.cursor = end;
                    if self.cursor == mesh.faces.len() {
                        return Ok(Some(std::mem::take(&mut self.out)));
                    }
                }
            }
        }
        Ok(None)
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

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/🎟️reservation/🦀️.rs"]
mod tessellation_reservation_tests;

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/🎨️metadata/🦀️.rs"]
mod tessellation_metadata_tests;

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/📤️corner/🦀️.rs"]
mod tessellation_corner_tests;

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/📤️edge/🦀️.rs"]
mod tessellation_edge_tests;

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/📤️triangle/🦀️.rs"]
mod tessellation_triangle_tests;

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/🎨️projection/🦀️.rs"]
mod tessellation_projection_tests;

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/🧭️face/🦀️.rs"]
mod tessellation_face_tests;

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/⏱️normal/🦀️.rs"]
mod tessellation_normal_tests;

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/⏱️polygons/🦀️.rs"]
mod tessellation_polygon_tests;

#[cfg(test)]
#[path="🧪️tests/🔬️tessellation/🎨️channels/🦀️.rs"]
mod tessellation_channel_tests;

protocol::value::artifact_retire_leaf!(VertexId,EdgeId,FaceId);
impl protocol::value::retirement::RetireOwned for MeshTessellationJob {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::retirement::sequence(vec![protocol::value::retirement::deferred(self.mesh),protocol::value::retirement::deferred(self.output),protocol::value::retirement::deferred(self.hes),protocol::value::retirement::deferred(self.points),protocol::value::retirement::deferred(self.projected),protocol::value::retirement::deferred(self.links),protocol::value::retirement::deferred(self.edge_seen),protocol::value::retirement::deferred(self.selected_faces),protocol::value::retirement::deferred(self.corner_ids),protocol::value::retirement::deferred(self.attribute_name),protocol::value::retirement::deferred(self.attribute_indices),protocol::value::retirement::deferred(self.attribute_pending),protocol::value::retirement::deferred(self.attribute_original_indices)])}
    fn retirement_birth_bytes(&self)->Option<usize> {protocol::value::retirement::sequence_birth_bytes(&[protocol::value::retirement::deferred_birth_bytes_for(&self.mesh),protocol::value::retirement::deferred_birth_bytes_for(&self.output),protocol::value::retirement::deferred_birth_bytes_for(&self.hes),protocol::value::retirement::deferred_birth_bytes_for(&self.points),protocol::value::retirement::deferred_birth_bytes_for(&self.projected),protocol::value::retirement::deferred_birth_bytes_for(&self.links),protocol::value::retirement::deferred_birth_bytes_for(&self.edge_seen),protocol::value::retirement::deferred_birth_bytes_for(&self.selected_faces),protocol::value::retirement::deferred_birth_bytes_for(&self.corner_ids),protocol::value::retirement::deferred_birth_bytes_for(&self.attribute_name),protocol::value::retirement::deferred_birth_bytes_for(&self.attribute_indices),protocol::value::retirement::deferred_birth_bytes_for(&self.attribute_pending),protocol::value::retirement::deferred_birth_bytes_for(&self.attribute_original_indices)])}
    fn controlled_retirement_supported()->bool {true}
}
