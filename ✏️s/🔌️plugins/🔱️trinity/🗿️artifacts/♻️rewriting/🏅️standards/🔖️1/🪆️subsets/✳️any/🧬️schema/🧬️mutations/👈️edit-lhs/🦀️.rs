//! 🔍️ Direct rewriting mutation — `EditLhs`: replaces the authored LHS match-pattern body (JSON).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Mutation
/// 🔍️ `edit-lhs` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-lhs")]
pub struct EditLhs {
    pub new_lhs: crate::standards::v1::subsets::any::schema::rule::Lhs,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_lhs<Op>(new_lhs: crate::standards::v1::subsets::any::schema::rule::Lhs) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::edit_lhs::EditLhs>,
{
    Op::from(EditLhs { new_lhs })
}

impl<Op> protocol::MutationKind<RewritingSnapshot, Op> for EditLhs
where
    Op: protocol::Mutation<RewritingSnapshot, Diff = RewritingDiff> + From<EditLhs>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "lhs", kind: "edit-lhs", record: "EditedLhs" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Edit lhs", "Linke Regelseite bearbeiten")
    }
}

impl<Op> protocol::ColdMutationKind<RewritingSnapshot, Op> for EditLhs
where
    Op: protocol::ColdMutation<RewritingSnapshot, ReferenceDiff = RewritingDiff> + From<EditLhs>,
{
    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
