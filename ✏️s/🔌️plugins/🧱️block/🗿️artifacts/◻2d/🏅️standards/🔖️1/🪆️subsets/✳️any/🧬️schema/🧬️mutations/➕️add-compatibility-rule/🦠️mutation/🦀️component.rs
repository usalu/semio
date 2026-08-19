//! ➕️ Block2d mutation — `AddCompatibilityRule`: a handle-kind compatibility rule attachment.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockCompatibilityRule};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕️ `add-compatibility-rule` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-compatibility-rule")]
pub struct AddCompatibilityRule {
    #[dsl(block)]
    pub rule: BlockCompatibilityRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_compatibility_rule(rule: BlockCompatibilityRule) -> Block2dMutation {
    Block2dMutation::AddCompatibilityRule(AddCompatibilityRule { rule })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for AddCompatibilityRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "compatibility-rule", kind: "add-compatibility-rule", record: "AddedCompatibilityRule" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
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
