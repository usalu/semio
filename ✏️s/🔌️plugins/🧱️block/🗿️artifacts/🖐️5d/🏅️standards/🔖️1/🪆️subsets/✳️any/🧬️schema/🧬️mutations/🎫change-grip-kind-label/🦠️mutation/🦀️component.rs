//! 🎫 Block5d mutation — `ChangeGripKindLabel`: a grip-kind catalog row's `label`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎫 `change-grip-kind-label` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grip-kind-label")]
pub struct ChangeGripKindLabel {
    pub id: String,
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grip_kind_label(id: String, new_label: String) -> Block5dMutation {
    Block5dMutation::ChangeGripKindLabel(ChangeGripKindLabel { id, new_label })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeGripKindLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "grip-kind", kind: "change-grip-kind-label", record: "ChangedGripKindLabel" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change grip kind \"{}\" label to \"{}\"", self.id, self.new_label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
