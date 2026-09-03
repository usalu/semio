//! 🔀️ Block2d mutation — `ChangeNodeKindVariant`: the node kind's optional `variant`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 🔀️ `change-node-kind-variant` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-node-kind-variant")]
pub struct ChangeNodeKindVariant {
    pub new_variant: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_node_kind_variant(new_variant: Option<String>) -> Block2dMutation {
    Block2dMutation::ChangeNodeKindVariant(ChangeNodeKindVariant { new_variant })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeNodeKindVariant {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-kind", kind: "change-node-kind-variant", record: "ChangedNodeKindVariant" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change node kind variant to {:?}", self.new_variant)
    }
}
//#endregion 🔖️Mutation
