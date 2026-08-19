//! 🧷 Block5d mutation — `ChangeGripGripKind`: a grip's `gripKind` catalog reference (rebind).
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧷 `change-grip-grip-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grip-grip-kind")]
pub struct ChangeGripGripKind {
    pub id: String,
    pub new_grip_kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_grip_grip_kind(id: String, new_grip_kind: String) -> Block5dMutation {
    Block5dMutation::ChangeGripGripKind(ChangeGripGripKind { id, new_grip_kind })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeGripGripKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "grip", kind: "change-grip-grip-kind", record: "ChangedGripGripKind" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change grip \"{}\" grip kind to \"{}\"", self.id, self.new_grip_kind)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
