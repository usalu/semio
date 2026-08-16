//! 💬 Block5d mutation — `ChangeMetaDescription`: the editing-session `meta.description`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 💬 `change-meta-description` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-meta-description")]
pub struct ChangeMetaDescription {
    pub new_description: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_meta_description(new_description: String) -> Block5dMutation {
    Block5dMutation::ChangeMetaDescription(ChangeMetaDescription { new_description })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeMetaDescription {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "meta", kind: "change-meta-description", record: "ChangedMetaDescription" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Change meta description".to_string()
    }
}
//#endregion 🔖️Mutation
