//! 🖼️ Block5d mutation — `ChangePartKindIcon`: the part kind's optional `icon`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ `change-part-kind-icon` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-part-kind-icon")]
pub struct ChangePartKindIcon {
    pub new_icon: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_part_kind_icon(new_icon: Option<String>) -> Block5dMutation {
    Block5dMutation::ChangePartKindIcon(ChangePartKindIcon { new_icon })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangePartKindIcon {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-kind", kind: "change-part-kind-icon", record: "ChangedPartKindIcon" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change part kind icon to {:?}", self.new_icon)
    }
}
//#endregion 🔖️Mutation
