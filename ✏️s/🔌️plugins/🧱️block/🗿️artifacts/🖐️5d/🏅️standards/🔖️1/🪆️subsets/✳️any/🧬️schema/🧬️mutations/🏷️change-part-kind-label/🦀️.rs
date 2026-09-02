//! 🏷️ Block5d mutation — `ChangePartKindLabel`: the part kind's `label`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🏷️ `change-part-kind-label` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-part-kind-label")]
pub struct ChangePartKindLabel {
    pub new_label: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_part_kind_label(new_label: String) -> Block5dMutation {
    Block5dMutation::ChangePartKindLabel(ChangePartKindLabel { new_label })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangePartKindLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-kind", kind: "change-part-kind-label", record: "ChangedPartKindLabel" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change part kind label to \"{}\"", self.new_label)
    }
}
//#endregion 🔖️Mutation
