//! ➕ `add-edge-property` — attaches one keyed property entry to an edge at a FINAL-state index within that edge's
//! `properties` (the edge twin of `add-node-property`, reusing `🔢️value`'s `SemioValueEntry`).

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, SemioGraphSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;

//#region 🔖️Payload
/// ➕ `add-edge-property` payload — the edge, the index the entry lands at and the entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddEdgeProperty {
    pub edge_id: GraphEdgeId,
    pub index: usize,
    pub property: SemioValueEntry,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for AddEdgeProperty {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "edge-property", kind: "add-edge-property", record: "AddedEdgeProperty" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add property \"{}\" to edge \"{}\"", self.property.key, self.edge_id.value), &format!("Eigenschaft \"{}\" zu Kante \"{}\" hinzufügen", self.property.key, self.edge_id.value))
    }
    fn target(&self) -> Vec<String> {
        vec![self.edge_id.value.clone()]
    }
}
//#endregion 🔖️Payload
