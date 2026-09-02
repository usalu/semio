//! 📃️ Block5d mutation — `ChangePartKindDescription`: the part kind's `description`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 📃️ `change-part-kind-description` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-part-kind-description")]
pub struct ChangePartKindDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_part_kind_description(new_description: String) -> Block5dMutation {
    Block5dMutation::ChangePartKindDescription(ChangePartKindDescription { new_description })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangePartKindDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-kind", kind: "change-part-kind-description", record: "ChangedPartKindDescription" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Change part kind description".to_string()
    }
}
//#endregion 🔖️Mutation
