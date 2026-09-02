//! 🖋 Block5d mutation — `RenameGripKind`: a grip-kind catalog row's `name`.

use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripKindsDelta, Block5dGripKindsPatch, Block5dGripKindsPatchEntry};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🖋 `rename-grip-kind` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rename-grip-kind")]
pub struct RenameGripKind {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn rename_grip_kind(id: String, new_name: String) -> Block5dMutation {
    Block5dMutation::RenameGripKind(RenameGripKind { id, new_name })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for RenameGripKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "grip-kind", kind: "rename-grip-kind", record: "RenamedGripKind" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename grip kind \"{}\" to \"{}\"", self.id, self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
