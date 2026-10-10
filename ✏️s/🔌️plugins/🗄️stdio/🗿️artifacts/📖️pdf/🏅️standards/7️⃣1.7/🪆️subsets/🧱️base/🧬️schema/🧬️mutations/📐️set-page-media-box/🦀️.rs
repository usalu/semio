//! 📐️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-media-box`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageMediaBox {
    pub index: usize,
    pub media_box: [f64; 4],
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageMediaBox {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-media-box", kind: "set-page-media-box", record: "Set" };

    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_page_media_box(self.index, self.media_box))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        base.pages.get(self.index).map(|page| PdfMutation::SetPageMediaBox(SetPageMediaBox { index: self.index, media_box: page.media_box })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set page {} media box", self.index), &format!("MediaBox von Seite {} setzen", self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

