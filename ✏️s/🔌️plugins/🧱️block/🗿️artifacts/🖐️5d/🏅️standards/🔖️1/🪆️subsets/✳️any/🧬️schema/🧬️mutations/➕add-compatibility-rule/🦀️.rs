//! ➕ Block5d mutation — `AddCompatibilityRule`: a grip-kind compatibility rule attachment.

use crate::BlockCompatibilityRule;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dCompatibilityDelta, Block5dDiff};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// ➕ `add-compatibility-rule` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-compatibility-rule")]
pub struct AddCompatibilityRule {
    #[dsl(block)]
    pub rule: BlockCompatibilityRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_compatibility_rule(rule: BlockCompatibilityRule) -> Block5dMutation {
    Block5dMutation::AddCompatibilityRule(AddCompatibilityRule { rule })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for AddCompatibilityRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "compatibility-rule", kind: "add-compatibility-rule", record: "AddedCompatibilityRule" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Add compatibility rule \"{}\"", self.rule.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.rule.id.clone()]
    }
}
//#endregion 🔖️Mutation
