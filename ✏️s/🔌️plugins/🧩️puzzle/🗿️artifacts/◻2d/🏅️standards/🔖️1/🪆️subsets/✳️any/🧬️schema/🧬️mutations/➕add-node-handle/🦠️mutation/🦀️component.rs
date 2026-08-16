//! ➕ Puzzle2d mutation — `AddNodeHandle`: attaches a new rim port to a node.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::{Puzzle2dHandle, Puzzle2dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕ `add-node-handle` payload — owner node id + new handle payload at an optional FINAL-state
/// `index` (`None` appends). A duplicate `handle.id` on the same node is a no-op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-node-handle")]
pub struct AddNodeHandle {
    pub node_id: String,
    #[dsl(block)]
    pub handle: Puzzle2dHandle,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_node_handle(node_id: String, handle: Puzzle2dHandle, index: Option<usize>) -> Puzzle2dMutation {
    Puzzle2dMutation::AddNodeHandle(AddNodeHandle { node_id, handle, index })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for AddNodeHandle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "node-handle", kind: "add-node-handle", record: "AddedNodeHandle" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add handle \"{}\" to node \"{}\"", self.handle.id, self.node_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.clone(), self.handle.id.clone()]
    }
}
//#endregion 🔖️Mutation
