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
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Vec<MeshVertex>,Vec<HalfEdge>,Vec<MeshFace>,HistoryFoldSet<u32>,HistoryFoldIndex<String,MeshAttribute>,HistoryFoldIndex<String,semio_framework_value::DslValue>,HistoryFoldIndex<String,MeshTexture>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.vertices,self.halfedges,self.faces,self.uv_seams,self.attributes,self.materials,self.textures)}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshVertex {
    type RetirementCursor=semio_framework_value::retirement_fields_type!([f32; 3],Option<[f32; 3]>,Option<u32>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.position,self.normal,self.halfedge)}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for HalfEdge {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(u32,Option<u32>,u32,Option<u32>,[f32; 2]);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.vertex,self.twin,self.next,self.face,self.uv)}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshFace {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(u32,bool,bool);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.halfedge,self.smooth,self.flipped)}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshTransfer {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Vec<f32>,Vec<f32>,Vec<f32>,Vec<u32>,Vec<f32>,Vec<u32>,Vec<u32>,Vec<f32>,Vec<u32>,Vec<f32>,Vec<u8>,HistoryFoldIndex<String,MeshAttribute>,HistoryFoldIndex<String,semio_framework_value::DslValue>,HistoryFoldIndex<String,MeshTexture>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.positions,self.normals,self.colors,self.indices,self.uvs,self.face_ids,self.vertex_ids,self.edge_positions,self.edge_ids,self.edge_uvs,self.edge_is_seam,self.attributes,self.materials,self.textures)}
    fn controlled_retirement_supported()->bool {true}
}
//#endregion HalfedgeMesh
