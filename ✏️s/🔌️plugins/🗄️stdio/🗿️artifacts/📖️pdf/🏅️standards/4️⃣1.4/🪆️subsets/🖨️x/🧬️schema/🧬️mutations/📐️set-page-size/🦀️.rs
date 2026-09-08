//! 📐️ Direct set-page-size payload, sparse diff, concrete inverse, and laws.

use super::PdfX1Mutation;
use crate::standards::v1_4::subsets::base::schema::{
    diff::{PdfDiff, PdfPageDiff, PdfPageModified, PdfPagesDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
/// 📐️ A4 geometry used by the independent PDF/X conformance oracle.
pub const CONFORMANT_WIDTH: f64 = 595.276;
pub const CONFORMANT_HEIGHT: f64 = 841.89;
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetPageSize {
    pub width: f64,
    pub height: f64,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl SetPageSize {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        !base.pages.is_empty() && self.width.is_finite() && self.height.is_finite()
    }
}

impl MutationKind<PdfSnapshot, PdfX1Mutation> for SetPageSize {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page", kind: "set-page-size", record: "SetPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.set-page-size.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { width: Some(self.width), height: Some(self.height), text: None } }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfX1Mutation> {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfX1Mutation::SetPageSize(SetPageSize { width: base.pages[0].width, height: base.pages[0].height })]
    }

    fn label(&self) -> String {
        "set page size".into()
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
