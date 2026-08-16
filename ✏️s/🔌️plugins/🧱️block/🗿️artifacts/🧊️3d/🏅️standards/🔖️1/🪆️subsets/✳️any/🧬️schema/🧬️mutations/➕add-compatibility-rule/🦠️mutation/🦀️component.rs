//! ➕ Block3d mutation — `AddCompatibilityRule`: a handle/vortex-kind compatibility rule attachment.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockCompatibilityRule};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕ `add-compatibility-rule` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-compatibility-rule")]
pub struct AddCompatibilityRule {
    #[dsl(block)]
    pub rule: BlockCompatibilityRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_compatibility_rule(rule: BlockCompatibilityRule) -> Block3dMutation {
    Block3dMutation::AddCompatibilityRule(AddCompatibilityRule { rule })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for AddCompatibilityRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "compatibility-rule", kind: "add-compatibility-rule", record: "AddedCompatibilityRule" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
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
