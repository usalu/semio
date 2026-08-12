//! ➖ Puzzle2d mutation — `RemoveNodeHandle`: detaches a rim port from a node (captures cascade —
//! any edge whose `source`/`target` referenced this handle is severed too).
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➖ `remove-node-handle` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-node-handle")]
pub struct RemoveNodeHandle {
    pub node_id: String,
    pub handle_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_node_handle(node_id: String, handle_id: String) -> Puzzle2dMutation {
    Puzzle2dMutation::RemoveNodeHandle(RemoveNodeHandle { node_id, handle_id })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for RemoveNodeHandle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node-handle", kind: "remove-node-handle", record: "RemovedNodeHandle" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove handle \"{}\" from node \"{}\"", self.handle_id, self.node_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.clone(), self.handle_id.clone()]
    }
}
//#endregion 🔖️Mutation
