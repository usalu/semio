//! 📍 `move-vertex` — absolute spatial reposition of a vertex (taxonomy's `move` verb — FINAL-state
//! absolute position, not a relative offset). SMO approved this verb explicitly.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveVertex {
    pub vertex_id: String,
    pub new_point: SemioPoint3,
}

impl<Op> protocol::MutationKind<SemioBrepSnapshot, Op> for MoveVertex
where
    Op: protocol::Mutation<SemioBrepSnapshot, Diff = crate::schema::diff::SemioBrepDiff> + From<MoveVertex>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "vertex", kind: "move-vertex", record: "MovedVertex" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move vertex \"{}\" to ({}, {}, {})", self.vertex_id, self.new_point.x, self.new_point.y, self.new_point.z), &format!("Vertex \"{}\" nach ({}, {}, {}) verschieben", self.vertex_id, self.new_point.x, self.new_point.y, self.new_point.z))
    }
    fn target(&self) -> Vec<String> {
        vec![self.vertex_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioBrepSnapshot, Op> for MoveVertex
where
    Op: protocol::ColdMutation<SemioBrepSnapshot, ReferenceDiff = crate::schema::diff::SemioBrepDiff> + From<MoveVertex>,
{
    fn diff(&self, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioBrepSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
