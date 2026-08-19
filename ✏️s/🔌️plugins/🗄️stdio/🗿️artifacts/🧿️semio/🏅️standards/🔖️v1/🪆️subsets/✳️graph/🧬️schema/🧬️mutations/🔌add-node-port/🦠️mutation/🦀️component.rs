//! 🔌 `add-node-port` — attaches one port to a node at a FINAL-state index within that node's
//! `ports` (an intrinsically ordered, anonymous collection nested one level inside `nodes` — mirrors
//! `✳️text`'s `add-mark` exactly).

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphPort, SemioGraphSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddNodePort {
    pub node_id: GraphNodeId,
    pub index: usize,
    pub port: SemioGraphPort,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for AddNodePort {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "node-port", kind: "add-node-port", record: "AddedNodePort" };

    async fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Add port to node \"{}\" at #{}", self.node_id.value, self.index)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
