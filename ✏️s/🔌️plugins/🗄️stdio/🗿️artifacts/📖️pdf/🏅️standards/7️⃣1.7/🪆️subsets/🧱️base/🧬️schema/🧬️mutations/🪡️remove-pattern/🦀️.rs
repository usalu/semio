//! 🪡️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-pattern`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemovePattern {
    pub id: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemovePattern
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemovePattern> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_pattern::SetPattern>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "pattern", kind: "remove-pattern", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove pattern {}", self.id), &format!("Muster {} entfernen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemovePattern
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemovePattern> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_pattern::SetPattern>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_pattern(base, &self.id))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.patterns.iter().position(|item| item.id == self.id).map(|index| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::set_pattern::SetPattern { pattern: base.patterns[index].clone(), index: Some(index) })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

