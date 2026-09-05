//! 📃️ Block2d mutation — `ChangeNodeKindDescription`: the node kind's free-text `description`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// 📃️ `change-node-kind-description` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-node-kind-description")]
pub struct ChangeNodeKindDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_node_kind_description(new_description: String) -> Block2dMutation {
    Block2dMutation::ChangeNodeKindDescription(ChangeNodeKindDescription { new_description })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for ChangeNodeKindDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-kind", kind: "change-node-kind-description", record: "ChangedNodeKindDescription" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Change node kind description".to_string()
    }
}
//#endregion 🔖️Mutation
