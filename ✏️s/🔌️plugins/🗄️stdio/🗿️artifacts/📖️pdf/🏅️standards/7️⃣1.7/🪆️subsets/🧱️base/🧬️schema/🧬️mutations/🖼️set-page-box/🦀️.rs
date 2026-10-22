//! 🖼️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-box`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use crate::standards::v1_7::subsets::base::schema::diff::PdfPageBox;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageBox {
    pub index: usize,
    pub kind: PdfPageBox,
    pub rect: Option<PdfRect>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetPageBox
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetPageBox>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-box", kind: "set-page-box", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set page {} {:?} box", self.index, self.kind), &format!("Seite {}: {:?}-Box setzen", self.index, self.kind))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetPageBox
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetPageBox>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_page_box(self.index, self.kind, self.rect))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.pages.get(self.index).map(|page| Op::from(SetPageBox { index: self.index, kind: self.kind, rect: match self.kind { PdfPageBox::Crop => page.crop_box, PdfPageBox::Bleed => page.bleed_box, PdfPageBox::Trim => page.trim_box, PdfPageBox::Art => page.art_box } })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

