//! ➖ `remove-node-property` — detaches one property entry from a node, addressed by BASE-state
//! `{node_id, index}` — exactly mirrors `remove-node-port` on `properties`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveNodeProperty {
    pub node_id: GraphNodeId,
    pub index: usize,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for RemoveNodeProperty {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node-property", kind: "remove-node-property", record: "RemovedNodeProperty" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove property #{} from node \"{}\"", self.index, self.node_id.value)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
