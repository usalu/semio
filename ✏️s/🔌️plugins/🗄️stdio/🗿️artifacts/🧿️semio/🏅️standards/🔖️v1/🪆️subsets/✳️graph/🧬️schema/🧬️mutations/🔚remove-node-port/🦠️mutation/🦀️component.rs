//! 🔚 `remove-node-port` — detaches one port from a node, addressed by BASE-state
//! `{node_id, index}`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveNodePort {
    pub node_id: GraphNodeId,
    pub index: usize,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for RemoveNodePort {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node-port", kind: "remove-node-port", record: "RemovedNodePort" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove port #{} from node \"{}\"", self.index, self.node_id.value)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
