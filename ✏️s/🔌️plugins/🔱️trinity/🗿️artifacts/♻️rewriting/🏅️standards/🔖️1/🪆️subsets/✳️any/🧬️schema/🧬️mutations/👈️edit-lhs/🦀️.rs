//! 🔍️ Direct rewriting mutation — `EditLhs`: replaces the authored LHS match-pattern body (JSON).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🔍️ `edit-lhs` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-lhs")]
pub struct EditLhs {
    pub new_lhs: crate::standards::v1::subsets::any::schema::Lhs,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_lhs(new_lhs: crate::standards::v1::subsets::any::schema::Lhs) -> RewriteRuleMutation {
    RewriteRuleMutation::EditLhs(EditLhs { new_lhs })
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for EditLhs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "lhs", kind: "edit-lhs", record: "EditedLhs" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Edit lhs", "Linke Regelseite bearbeiten")
    }
}
//#endregion 🔖️Mutation
