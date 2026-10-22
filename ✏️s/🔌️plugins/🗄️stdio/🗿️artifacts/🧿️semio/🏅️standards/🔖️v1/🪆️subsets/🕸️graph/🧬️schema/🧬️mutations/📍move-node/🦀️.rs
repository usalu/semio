//! 📍 `move-node` — absolute spatial reposition of a node (taxonomy's `move` verb — FINAL-state
//! absolute position, not a relative offset).

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use crate::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveNode {
    pub id: GraphNodeId,
    pub new_position: SemioPoint2,
}

impl<Op> protocol::MutationKind<SemioGraphSnapshot, Op> for MoveNode
where
    Op: protocol::Mutation<SemioGraphSnapshot, Diff = crate::schema::diff::SemioGraphDiff> + From<MoveNode>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move node \"{}\" to ({}, {})", self.id.value, self.new_position.x, self.new_position.y), &format!("Knoten \"{}\" nach ({}, {}) verschieben", self.id.value, self.new_position.x, self.new_position.y))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioGraphSnapshot, Op> for MoveNode
where
    Op: protocol::ColdMutation<SemioGraphSnapshot, ReferenceDiff = crate::schema::diff::SemioGraphDiff> + From<MoveNode>,
{
    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioGraphSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
