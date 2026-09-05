//! 🔌 `add-node-port` — attaches one port to a node at a FINAL-state index within that node's
//! `ports` (an intrinsically ordered, anonymous collection nested one level inside `nodes` — mirrors
//! `🔤️text`'s `add-mark` exactly).

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, remove_node_port};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphPort, SemioGraphSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddNodePort {
    pub node_id: GraphNodeId,
    pub index: usize,
    pub port: SemioGraphPort,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for AddNodePort {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "node-port", kind: "add-node-port", record: "AddedNodePort" };

    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add port to node \"{}\" at #{}", self.node_id.value, self.index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.value.clone(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
