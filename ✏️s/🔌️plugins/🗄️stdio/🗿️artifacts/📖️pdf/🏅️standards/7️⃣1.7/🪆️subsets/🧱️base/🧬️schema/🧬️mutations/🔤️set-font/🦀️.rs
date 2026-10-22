//! 🔤️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-font`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetFont {
    pub font: PdfFont,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetFont
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetFont> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_font::RemoveFont>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "font", kind: "set-font", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set font {}", self.font.id), &format!("Schrift {} setzen", self.font.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.font.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetFont
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetFont> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_font::RemoveFont>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_font(base, self.font.clone(), self.index))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.fonts.iter().find(|item| item.id == self.font.id) { Some(previous) => vec![Op::from(SetFont { font: previous.clone(), index: None })], None => vec![Op::from(crate::standards::v1_7::subsets::base::schema::mutations::remove_font::RemoveFont { id: self.font.id.clone() })] }
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

