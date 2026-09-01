//! 🖼️ Block2d mutation — `ChangeNodeKindIcon`: the node kind's optional `icon`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ `change-node-kind-icon` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-node-kind-icon")]
pub struct ChangeNodeKindIcon {
    pub new_icon: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_kind_icon(new_icon: Option<String>) -> Block2dMutation {
    Block2dMutation::ChangeNodeKindIcon(ChangeNodeKindIcon { new_icon })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeNodeKindIcon {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-kind", kind: "change-node-kind-icon", record: "ChangedNodeKindIcon" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change node kind icon to {:?}", self.new_icon)
    }
}
//#endregion 🔖️Mutation
