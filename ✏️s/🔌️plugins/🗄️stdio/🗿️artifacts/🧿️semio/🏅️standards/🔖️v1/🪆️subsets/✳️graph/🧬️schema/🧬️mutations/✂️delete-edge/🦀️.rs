//! ✂️ `delete-edge` — removes an id-keyed edge; no cascade needed (edges don't own other entities).

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, create_edge};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, SemioGraphSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteEdge {
    pub id: GraphEdgeId,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for DeleteEdge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "edge", kind: "delete-edge", record: "DeletedEdge" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete edge \"{}\"", self.id.value)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
