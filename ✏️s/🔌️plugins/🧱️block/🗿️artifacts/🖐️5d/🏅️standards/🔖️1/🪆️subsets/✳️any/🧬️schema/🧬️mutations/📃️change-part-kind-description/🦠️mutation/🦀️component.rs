//! 📃️ Block5d mutation — `ChangePartKindDescription`: the part kind's `description`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📃️ `change-part-kind-description` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-part-kind-description")]
pub struct ChangePartKindDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_kind_description(new_description: String) -> Block5dMutation {
    Block5dMutation::ChangePartKindDescription(ChangePartKindDescription { new_description })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangePartKindDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-kind", kind: "change-part-kind-description", record: "ChangedPartKindDescription" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Change part kind description".to_string()
    }
}
//#endregion 🔖️Mutation
