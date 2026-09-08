//! 📝️ Direct set-page-text payload, sparse diff, concrete inverse, and laws.

use super::PdfA1Mutation;
use crate::standards::v1_4::subsets::base::schema::{
    diff::{PdfDiff, PdfPageDiff, PdfPageModified, PdfPagesDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
/// 📝️ Extractable-text conformance fixture used by the independent PDF/A-1 oracle.
pub const CONFORMANT_TEXT: &str = "Reuse of load-bearing timber components in Swiss building stock";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetPageText {
    pub text: String,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl SetPageText {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        !base.pages.is_empty()
    }
}

impl MutationKind<PdfSnapshot, PdfA1Mutation> for SetPageText {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page", kind: "set-page-text", record: "SetPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.set-page-text.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { text: Some(self.text.clone()), ..Default::default() } }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfA1Mutation> {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfA1Mutation::SetPageText(SetPageText { text: base.pages[0].text.clone() })]
    }

    fn label(&self) -> String {
        "set page text".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["0".into()]
    }
}
//#endregion 🔖️Behavior

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
