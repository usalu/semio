//! 💬 Block5d mutation — `ChangeMetaDescription`: the editing-session `meta.description`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
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
pub async fn change_meta_description(new_description: String) -> Block5dMutation {
    Block5dMutation::ChangeMetaDescription(ChangeMetaDescription { new_description })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeMetaDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "meta", kind: "change-meta-description", record: "ChangedMetaDescription" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Change meta description".to_string()
    }
}
//#endregion 🔖️Mutation
