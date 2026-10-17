//! 📍 `move-vertex` — absolute spatial reposition of ONE vertex inside a primitive's `positions` buffer, addressed by BASE-state index (`positions` only ever changes wholesale via `replace-primitive-geometry`, so an index stays valid for the lifetime of one geometry \"epoch\"). Present in the vocabulary because `computed-normals` (deliberately OMITTED from this subset's own inference facet, see its module doc comment) would have presumed raw per-vertex position edits are a first-class gesture; `move-vertex` is authored on its own independent merits — a real address (`mesh_id`+`primitive_id`+`vertex_index`) and one field (`new_point`) — matching taxonomy's `move` verb exactly, same shape as brep's own approved `move-vertex`.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveVertex {
    pub mesh_id: String,
    pub primitive_id: String,
    pub vertex_index: usize,
    pub new_point: SemioPoint3,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for MoveVertex
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<MoveVertex>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "vertex", kind: "move-vertex", record: "MovedVertex" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move vertex {} of primitive \"{}\" in mesh \"{}\" to ({}, {}, {})", self.vertex_index, self.primitive_id, self.mesh_id, self.new_point.x, self.new_point.y, self.new_point.z), &format!("Vertex {} von Primitiv \"{}\" in Netz \"{}\" nach ({}, {}, {}) verschieben", self.vertex_index, self.primitive_id, self.mesh_id, self.new_point.x, self.new_point.y, self.new_point.z))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{}:{}:{}", self.mesh_id, self.primitive_id, self.vertex_index)]
    }
}
//#endregion 🔖️Payload
