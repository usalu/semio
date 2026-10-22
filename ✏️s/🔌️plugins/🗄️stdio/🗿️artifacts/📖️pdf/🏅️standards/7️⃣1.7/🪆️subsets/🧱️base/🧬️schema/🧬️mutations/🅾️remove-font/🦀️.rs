//! 🅾️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-font`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveFont {
    pub id: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveFont
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveFont> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_font::SetFont>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "font", kind: "remove-font", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove font {}", self.id), &format!("Schrift {} entfernen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveFont
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveFont> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_font::SetFont>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_font(base, &self.id))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.fonts.iter().position(|item| item.id == self.id).map(|index| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::set_font::SetFont { font: base.fonts[index].clone(), index: Some(index) })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

