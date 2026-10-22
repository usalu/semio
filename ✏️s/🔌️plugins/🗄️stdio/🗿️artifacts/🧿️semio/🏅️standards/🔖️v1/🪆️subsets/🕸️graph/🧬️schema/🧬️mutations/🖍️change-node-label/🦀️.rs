//! 🖍️ `change-node-label` — sets one node's `label` scalar field, addressed by BASE-state id.

use crate::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeNodeLabel {
    pub id: GraphNodeId,
    pub new_label: String,
}

impl<Op> protocol::MutationKind<SemioGraphSnapshot, Op> for ChangeNodeLabel
where
    Op: protocol::Mutation<SemioGraphSnapshot, Diff = crate::schema::diff::SemioGraphDiff> + From<ChangeNodeLabel>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-label", kind: "change-node-label", record: "ChangedNodeLabel" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change node \"{}\" label to {}", self.id.value, self.new_label), &format!("Beschriftung von Knoten \"{}\" auf {} ändern", self.id.value, self.new_label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioGraphSnapshot, Op> for ChangeNodeLabel
where
    Op: protocol::ColdMutation<SemioGraphSnapshot, ReferenceDiff = crate::schema::diff::SemioGraphDiff> + From<ChangeNodeLabel>,
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
