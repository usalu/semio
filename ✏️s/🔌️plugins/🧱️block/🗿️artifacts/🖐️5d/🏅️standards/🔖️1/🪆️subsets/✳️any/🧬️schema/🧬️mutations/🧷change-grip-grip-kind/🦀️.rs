//! 🧷 Block5d mutation — `ChangeGripGripKind`: a grip's `gripKind` catalog reference (rebind).

use crate::artifacts::block5d::{Block5dGripTemplate, Block5dSnapshot};
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripsDelta, Block5dGripsPatch, Block5dGripsPatchEntry};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🧷 `change-grip-grip-kind` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
