//! ✏️ Block5d mutation — `RenamePartKind`: the part kind's identity `name`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ `rename-part-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-part-kind")]
pub struct RenamePartKind {
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn rename_part_kind(new_name: String) -> Block5dMutation {
    Block5dMutation::RenamePartKind(RenamePartKind { new_name })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for RenamePartKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "part-kind", kind: "rename-part-kind", record: "RenamedPartKind" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename part kind to \"{}\"", self.new_name)
    }
}
//#endregion 🔖️Mutation
