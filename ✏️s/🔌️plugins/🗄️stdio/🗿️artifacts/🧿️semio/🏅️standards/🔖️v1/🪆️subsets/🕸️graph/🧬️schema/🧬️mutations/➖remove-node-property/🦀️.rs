//! ➖ `remove-node-property` — detaches the property entry carrying `key` from a node (keyed like
//! `set-node-property`, so a history edit of an earlier row never re-targets it; the inverse restores the entry at its
//! BASE index, byte-exact).

use crate::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Payload
/// ➖ `remove-node-property` payload — the node and the key of the entry it detaches.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveNodeProperty {
    pub node_id: GraphNodeId,
    pub key: String,
}

impl<Op> protocol::MutationKind<SemioGraphSnapshot, Op> for RemoveNodeProperty
where
    Op: protocol::Mutation<SemioGraphSnapshot, Diff = crate::schema::diff::SemioGraphDiff> + From<RemoveNodeProperty> + From<crate::schema::mutations::add_node_property::AddNodeProperty>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node-property", kind: "remove-node-property", record: "RemovedNodeProperty" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove property \"{}\" from node \"{}\"", self.key, self.node_id.value), &format!("Eigenschaft \"{}\" aus Knoten \"{}\" entfernen", self.key, self.node_id.value))
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone(), self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioGraphSnapshot, Op> for RemoveNodeProperty
where
    Op: protocol::ColdMutation<SemioGraphSnapshot, ReferenceDiff = crate::schema::diff::SemioGraphDiff> + From<RemoveNodeProperty> + From<crate::schema::mutations::add_node_property::AddNodeProperty>,
{
    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioGraphSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
}
//#endregion 🔖️Payload
