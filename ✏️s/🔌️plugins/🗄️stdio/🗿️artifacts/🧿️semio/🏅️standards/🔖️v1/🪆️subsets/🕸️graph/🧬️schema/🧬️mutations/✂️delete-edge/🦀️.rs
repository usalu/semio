//! ✂️ `delete-edge` — removes an id-keyed edge; no cascade needed (edges don't own other entities).

use crate::schema::snapshot::{GraphEdgeId, SemioGraphSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteEdge {
    pub id: GraphEdgeId,
}

impl<Op> protocol::MutationKind<SemioGraphSnapshot, Op> for DeleteEdge
where
    Op: protocol::Mutation<SemioGraphSnapshot, Diff = crate::schema::diff::SemioGraphDiff> + From<DeleteEdge> + From<crate::schema::mutations::create_edge::CreateEdge>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "edge", kind: "delete-edge", record: "DeletedEdge" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete edge \"{}\"", self.id.value), &format!("Kante \"{}\" löschen", self.id.value))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioGraphSnapshot, Op> for DeleteEdge
where
    Op: protocol::ColdMutation<SemioGraphSnapshot, ReferenceDiff = crate::schema::diff::SemioGraphDiff> + From<DeleteEdge> + From<crate::schema::mutations::create_edge::CreateEdge>,
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
