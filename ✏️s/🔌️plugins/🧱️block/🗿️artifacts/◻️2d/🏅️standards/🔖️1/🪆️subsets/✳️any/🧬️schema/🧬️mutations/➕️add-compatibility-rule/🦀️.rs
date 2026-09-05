//! ➕️ Block2d mutation — `AddCompatibilityRule`: a handle-kind compatibility rule attachment.

use crate::BlockCompatibilityRule;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::{Block2dCompatibilityDelta, Block2dDiff};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Mutation
/// ➕️ `add-compatibility-rule` payload.
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
pub fn add_compatibility_rule(rule: BlockCompatibilityRule) -> Block2dMutation {
    Block2dMutation::AddCompatibilityRule(AddCompatibilityRule { rule })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for AddCompatibilityRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "compatibility-rule", kind: "add-compatibility-rule", record: "AddedCompatibilityRule" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add compatibility rule \"{}\"", self.rule.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.rule.id.clone()]
    }
}
//#endregion 🔖️Mutation
