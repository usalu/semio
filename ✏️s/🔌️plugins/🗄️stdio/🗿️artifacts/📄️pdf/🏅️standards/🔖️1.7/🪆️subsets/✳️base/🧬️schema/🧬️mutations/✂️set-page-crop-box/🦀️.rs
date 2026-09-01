//! ✂️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-crop-box`.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageCropBox {
    pub index: usize,
    pub crop_box: Option<[f64; 4]>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageCropBox {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-crop-box", kind: "set-page-crop-box", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_page_crop_box(self.index, self.crop_box))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        base.pages.get(self.index).map(|page| PdfMutation::SetPageCropBox(SetPageCropBox { index: self.index, crop_box: page.crop_box })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Set page {} crop box", self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_identity_is_owned_by_this_leaf() {
        assert_eq!(<SetPageCropBox as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-page-crop-box");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
