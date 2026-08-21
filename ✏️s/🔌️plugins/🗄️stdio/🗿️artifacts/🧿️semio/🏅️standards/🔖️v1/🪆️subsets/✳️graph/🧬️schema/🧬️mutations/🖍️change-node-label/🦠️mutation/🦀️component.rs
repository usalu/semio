//! 🖍️ `change-node-label` — sets one node's `label` scalar field, addressed by BASE-state id.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeNodeLabel {
    pub id: GraphNodeId,
    pub new_label: String,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for ChangeNodeLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-label", kind: "change-node-label", record: "ChangedNodeLabel" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change node \"{}\" label to {}", self.id.value, self.new_label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
