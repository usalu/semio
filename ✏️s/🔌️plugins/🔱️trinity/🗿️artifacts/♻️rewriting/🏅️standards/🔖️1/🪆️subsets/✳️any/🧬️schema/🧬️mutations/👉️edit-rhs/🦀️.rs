//! 🎯️ Direct rewriting mutation — `EditRhs`: replaces the authored RHS rewriting body (JSON).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🎯️ `edit-rhs` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-rhs")]
pub struct EditRhs {
    #[dsl(lang = "json")]
    pub new_rhs: crate::standards::v1::subsets::any::schema::Rhs,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_rhs(new_rhs: crate::standards::v1::subsets::any::schema::Rhs) -> RewriteRuleMutation {
    RewriteRuleMutation::EditRhs(EditRhs { new_rhs })
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for EditRhs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "rhs", kind: "edit-rhs", record: "EditedRhs" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Edit rhs", "Rechte Regelseite bearbeiten")
    }
}
//#endregion 🔖️Mutation
