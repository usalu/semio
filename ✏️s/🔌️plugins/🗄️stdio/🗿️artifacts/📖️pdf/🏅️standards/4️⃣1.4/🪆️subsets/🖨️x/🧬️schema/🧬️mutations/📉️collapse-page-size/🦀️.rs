//! 📉️ Direct collapse-page-size payload, sparse diff, concrete inverse, and laws.

use super::PdfX1Mutation;
use crate::standards::v1_4::subsets::base::schema::{
    diff::{PdfDiff, PdfPageDiff, PdfPageModified, PdfPagesDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollapsePageSize {}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl CollapsePageSize {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        !base.pages.is_empty()
    }
}

impl MutationKind<PdfSnapshot, PdfX1Mutation> for CollapsePageSize {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "page", kind: "collapse-page-size", record: "ResizedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.collapse-page-size.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { width: Some(0.0), ..Default::default() } }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfX1Mutation> {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfX1Mutation::SetPageSize(super::SetPageSize { width: base.pages[0].width, height: base.pages[0].height })]
    }

    fn label(&self) -> String {
        "collapse page size".into()
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
