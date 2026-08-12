//! 🔍️ Rewrite mutation — `EditLhs`: replaces the authored LHS match-pattern body (JSON).
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔍️ `edit-lhs` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-lhs")]
pub struct EditLhs {
    pub new_lhs_json: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_lhs(new_lhs_json: String) -> RewriteRuleMutation {
    RewriteRuleMutation::EditLhs(EditLhs { new_lhs_json })
}

impl protocol::MutationKind<RewriteSnapshot, RewriteRuleMutation> for EditLhs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "lhs", kind: "edit-lhs", record: "EditedLhs" };

    fn diff(&self, base: &RewriteSnapshot) -> RewriteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Edit lhs".to_string()
    }
}
//#endregion 🔖️Mutation
