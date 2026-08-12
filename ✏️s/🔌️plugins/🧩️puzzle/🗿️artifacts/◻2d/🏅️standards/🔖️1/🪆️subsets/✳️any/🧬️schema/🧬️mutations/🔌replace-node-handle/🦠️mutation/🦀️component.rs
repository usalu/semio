//! 🔌 Puzzle2d mutation — `ReplaceNodeHandle`: whole-value swap of one handle's presentation
//! fields (kind/angle/radius/color/icon/scale/visible/locked together, one property-panel gesture).
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::{Puzzle2dHandle, Puzzle2dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔌 `replace-node-handle` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-node-handle")]
pub struct ReplaceNodeHandle {
    pub node_id: String,
    pub handle_id: String,
    #[dsl(block)]
    pub new_handle: Puzzle2dHandle,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_node_handle(node_id: String, handle_id: String, new_handle: Puzzle2dHandle) -> Puzzle2dMutation {
    Puzzle2dMutation::ReplaceNodeHandle(ReplaceNodeHandle { node_id, handle_id, new_handle })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ReplaceNodeHandle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "node-handle", kind: "replace-node-handle", record: "ReplacedNodeHandle" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace handle \"{}\" on node \"{}\"", self.handle_id, self.node_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.clone(), self.handle_id.clone()]
    }
}
//#endregion 🔖️Mutation
