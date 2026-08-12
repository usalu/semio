//! 🎯️ Rewrite mutation — `EditRhs`: replaces the authored RHS rewrite body (JSON).
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎯️ `edit-rhs` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "edit-rhs")]
pub struct EditRhs {
    #[dsl(lang = "json")]
    pub new_rhs_json: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_rhs(new_rhs_json: String) -> RewriteRuleMutation {
    RewriteRuleMutation::EditRhs(EditRhs { new_rhs_json })
}

impl protocol::MutationKind<RewriteSnapshot, RewriteRuleMutation> for EditRhs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "rhs", kind: "edit-rhs", record: "EditedRhs" };

    fn diff(&self, base: &RewriteSnapshot) -> RewriteDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Edit rhs".to_string()
    }
}
//#endregion 🔖️Mutation
