//! ➖ `remove-edge-property` — detaches the property entry carrying `key` from an edge (keyed like
//! `set-edge-property`, so a history edit of an earlier row never re-targets it; the inverse restores the entry at its
//! BASE index, byte-exact).

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, SemioGraphSnapshot};

//#region 🔖️Payload
/// ➖ `remove-edge-property` payload — the edge and the key of the entry it detaches.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEdgeProperty {
    pub edge_id: GraphEdgeId,
    pub key: String,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for RemoveEdgeProperty {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "edge-property", kind: "remove-edge-property", record: "RemovedEdgeProperty" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove property \"{}\" from edge \"{}\"", self.key, self.edge_id.value), &format!("Eigenschaft \"{}\" aus Kante \"{}\" entfernen", self.key, self.edge_id.value))
    }
    fn target(&self) -> Vec<String> {
        vec![self.edge_id.value.clone(), self.key.clone()]
    }
}
//#endregion 🔖️Payload
