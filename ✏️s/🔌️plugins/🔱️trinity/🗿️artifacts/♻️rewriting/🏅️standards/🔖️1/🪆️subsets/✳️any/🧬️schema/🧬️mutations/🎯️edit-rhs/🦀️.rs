//! 🎯️ Direct rewriting mutation — `EditRhs`: replaces the authored RHS rewriting body (JSON).
use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::mutations::RewriteRuleMutation;
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Mutation
/// 🎯️ `edit-rhs` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-rhs")]
pub struct EditRhs {
    #[dsl(lang = "json")]
    pub new_rhs_json: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_rhs(new_rhs_json: String) -> RewriteRuleMutation {
    RewriteRuleMutation::EditRhs(EditRhs { new_rhs_json })
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for EditRhs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "rhs", kind: "edit-rhs", record: "EditedRhs" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Edit rhs".to_string()
    }
}
//#endregion 🔖️Mutation
