//! 🏷️ Block2d mutation — `ChangeNodeKindLabel`: the node kind's display `label`.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ `change-node-kind-label` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-node-kind-label")]
pub struct ChangeNodeKindLabel {
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_kind_label(new_label: String) -> Block2dMutation {
    Block2dMutation::ChangeNodeKindLabel(ChangeNodeKindLabel { new_label })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeNodeKindLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-kind", kind: "change-node-kind-label", record: "ChangedNodeKindLabel" };

    fn diff(&self, base: &Block2dSnapshot) -> Block2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change node kind label to \"{}\"", self.new_label)
    }
}
//#endregion 🔖️Mutation
