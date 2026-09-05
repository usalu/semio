//! 🔍️ Direct rewriting mutation — `EditLhs`: replaces the authored LHS match-pattern body (JSON).
use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::mutations::RewriteRuleMutation;
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Mutation
/// 🔍️ `edit-lhs` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-lhs")]
pub struct EditLhs {
    pub new_lhs_json: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_lhs(new_lhs_json: String) -> RewriteRuleMutation {
    RewriteRuleMutation::EditLhs(EditLhs { new_lhs_json })
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for EditLhs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "lhs", kind: "edit-lhs", record: "EditedLhs" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Edit lhs".to_string()
    }
}
//#endregion 🔖️Mutation
