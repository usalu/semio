//! 💬 Block3d mutation — `ChangeMetaDescription`: the editing-session `meta.description`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 💬 `change-meta-description` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-meta-description")]
pub struct ChangeMetaDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_meta_description(new_description: String) -> Block3dMutation {
    Block3dMutation::ChangeMetaDescription(ChangeMetaDescription { new_description })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ChangeMetaDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "meta", kind: "change-meta-description", record: "ChangedMetaDescription" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Change meta description".to_string()
    }
}
//#endregion 🔖️Mutation
