//! 🔀️ Block2d mutation — `ChangeNodeKindVariant`: the node kind's optional `variant`.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔀️ `change-node-kind-variant` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-node-kind-variant")]
pub struct ChangeNodeKindVariant {
    pub new_variant: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_kind_variant(new_variant: Option<String>) -> Block2dMutation {
    Block2dMutation::ChangeNodeKindVariant(ChangeNodeKindVariant { new_variant })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeNodeKindVariant {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-kind", kind: "change-node-kind-variant", record: "ChangedNodeKindVariant" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change node kind variant to {:?}", self.new_variant)
    }
}
//#endregion 🔖️Mutation
