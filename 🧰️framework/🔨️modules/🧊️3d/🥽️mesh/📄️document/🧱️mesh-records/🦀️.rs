//! 🧱️ Mesh records: the vertex, half-edge, face and transfer records, the halfedge mesh struct and their retirement owners. See [`super`].

use super::*;

//#region Types
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01).
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value")]
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

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value")]
pub struct HalfEdge {
    pub vertex: u32,
    pub twin: Option<u32>,
    pub next: u32,
    pub face: Option<u32>,
    #[cfg_attr(test, serde(default = "default_uv"))]
    #[value(default = "default_uv")]
    pub uv: [f32; 2],
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value")]
pub struct MeshFace {
    pub halfedge: u32,
    pub smooth: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub flipped: bool,
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value")]
pub struct MeshTransfer {
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub colors: Vec<f32>,
    #[value(default)]
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    pub attributes: HistoryFoldIndex<String,MeshAttribute>,
    #[value(default)]
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    pub materials: HistoryFoldIndex<String,semio_framework_value::DslValue>,
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
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value")]
pub struct HalfedgeMesh {
    pub vertices: Vec<MeshVertex>,
    pub halfedges: Vec<HalfEdge>,
    pub faces: Vec<MeshFace>,
    #[cfg_attr(test, serde(default))]
    #[value(default, with = "u32_membership_bridge")]
    pub uv_seams: HistoryFoldSet<u32>,
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    #[value(default)]
    pub attributes: HistoryFoldIndex<String,MeshAttribute>,
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    #[value(default)]
    pub materials: HistoryFoldIndex<String,semio_framework_value::DslValue>,
    #[cfg_attr(test, serde(default, with = "attribute_serde_bridge"))]
    #[value(default)]
    pub textures: HistoryFoldIndex<String,MeshTexture>,
}

impl semio_framework_value::retirement::RetireOwned for HalfedgeMesh {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.vertices),semio_framework_value::retirement::deferred(self.halfedges),semio_framework_value::retirement::deferred(self.faces),semio_framework_value::retirement::deferred(self.uv_seams),semio_framework_value::retirement::deferred(self.attributes),semio_framework_value::retirement::deferred(self.materials),semio_framework_value::retirement::deferred(self.textures)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.halfedges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.uv_seams),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.materials),semio_framework_value::retirement::deferred_birth_bytes_for(&self.textures)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshVertex {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.position),semio_framework_value::retirement::deferred(self.normal),semio_framework_value::retirement::deferred(self.halfedge)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.position),semio_framework_value::retirement::deferred_birth_bytes_for(&self.normal),semio_framework_value::retirement::deferred_birth_bytes_for(&self.halfedge)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for HalfEdge {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.vertex),semio_framework_value::retirement::deferred(self.twin),semio_framework_value::retirement::deferred(self.next),semio_framework_value::retirement::deferred(self.face),semio_framework_value::retirement::deferred(self.uv)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertex),semio_framework_value::retirement::deferred_birth_bytes_for(&self.twin),semio_framework_value::retirement::deferred_birth_bytes_for(&self.next),semio_framework_value::retirement::deferred_birth_bytes_for(&self.face),semio_framework_value::retirement::deferred_birth_bytes_for(&self.uv)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshFace {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.halfedge),semio_framework_value::retirement::deferred(self.smooth),semio_framework_value::retirement::deferred(self.flipped)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.halfedge),semio_framework_value::retirement::deferred_birth_bytes_for(&self.smooth),semio_framework_value::retirement::deferred_birth_bytes_for(&self.flipped)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshTransfer {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.positions),semio_framework_value::retirement::deferred(self.normals),semio_framework_value::retirement::deferred(self.colors),semio_framework_value::retirement::deferred(self.indices),semio_framework_value::retirement::deferred(self.uvs),semio_framework_value::retirement::deferred(self.face_ids),semio_framework_value::retirement::deferred(self.vertex_ids),semio_framework_value::retirement::deferred(self.edge_positions),semio_framework_value::retirement::deferred(self.edge_ids),semio_framework_value::retirement::deferred(self.edge_uvs),semio_framework_value::retirement::deferred(self.edge_is_seam),semio_framework_value::retirement::deferred(self.attributes),semio_framework_value::retirement::deferred(self.materials),semio_framework_value::retirement::deferred(self.textures)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.positions),semio_framework_value::retirement::deferred_birth_bytes_for(&self.normals),semio_framework_value::retirement::deferred_birth_bytes_for(&self.colors),semio_framework_value::retirement::deferred_birth_bytes_for(&self.indices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.uvs),semio_framework_value::retirement::deferred_birth_bytes_for(&self.face_ids),semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertex_ids),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_positions),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_ids),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_uvs),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_is_seam),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.materials),semio_framework_value::retirement::deferred_birth_bytes_for(&self.textures)])}
    fn controlled_retirement_supported()->bool {true}
}
//#endregion HalfedgeMesh
