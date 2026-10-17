//! ✂️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-crop-box`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageCropBox {
    pub index: usize,
    pub crop_box: Option<[f64; 4]>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetPageCropBox
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetPageCropBox>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-crop-box", kind: "set-page-crop-box", record: "Set" };

    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_page_crop_box(self.index, self.crop_box))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        base.pages.get(self.index).map(|page| Op::from(SetPageCropBox { index: self.index, crop_box: page.crop_box })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set page {} crop box", self.index), &format!("CropBox von Seite {} setzen", self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

