//! 🔗 `create-edge` — brings a new id-keyed edge into existence. Edges are ID-KEYED ENTITIES, not
//! relationships: `source`/`target` are ordinary data fields on this entity, addressed/mutated via
//! `create`/`delete`, never `connect`/`disconnect` (see the snapshot facet's module doc comment for
//! the full ruling).

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, delete_edge};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateEdge {
    pub id: GraphEdgeId,
    pub source: GraphNodeId,
    pub target: GraphNodeId,
    #[value(default)]
    pub kind: String,
    #[value(default)]
    pub label: String,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for CreateEdge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "edge", kind: "create-edge", record: "CreatedEdge" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create edge \"{}\" ({} -> {})", self.id.value, self.source.value, self.target.value)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
