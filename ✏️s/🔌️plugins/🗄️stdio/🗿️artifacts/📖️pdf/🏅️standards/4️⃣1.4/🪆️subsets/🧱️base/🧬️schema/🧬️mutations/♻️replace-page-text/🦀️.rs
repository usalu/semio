//! 📝️ Direct replace-page-text payload, sparse diff, concrete inverse, and laws.

use super::PdfMutation;
use crate::standards::v1_4::subsets::base::schema::{
    diff::{PdfDiff, PdfPageDiff, PdfPageModified, PdfPagesDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplacePageText {
    pub index: usize,
    pub text: String,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl ReplacePageText {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        self.index < base.pages.len()
    }
}

impl MutationKind<PdfSnapshot, PdfMutation> for ReplacePageText {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "page", kind: "replace-page-text", record: "ReplacedPageText" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.replace-page-text.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: self.index, diff: PdfPageDiff { text: Some(self.text.clone()), ..Default::default() } }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfMutation::ReplacePageText(ReplacePageText { index: self.index, text: base.pages[self.index].text.clone() })]
    }

    fn label(&self) -> String {
        "replace page text".into()
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Behavior

//#region 🔖️Codecs
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Codecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔄️round-trips-the-concrete-inverse/🦀️.rs"]
mod tests_round_trips_the_concrete_inverse;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
