//! ✂ Block5d mutation — `RemoveCompatibilityRule`: a compatibility rule attachment.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dCompatibilityDelta, Block5dDiff};
use crate::artifacts::block5d::mutations::Block5dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂ `remove-compatibility-rule` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-compatibility-rule")]
pub struct RemoveCompatibilityRule {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_compatibility_rule(id: String) -> Block5dMutation {
    Block5dMutation::RemoveCompatibilityRule(RemoveCompatibilityRule { id })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for RemoveCompatibilityRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "compatibility-rule", kind: "remove-compatibility-rule", record: "RemovedCompatibilityRule" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove compatibility rule \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
