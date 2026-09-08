//! 📐️ Direct resize-page payload, sparse diff, concrete inverse, and laws.

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
pub struct ResizePage {
    pub index: usize,
    pub width: f64,
    pub height: f64,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl ResizePage {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        self.index < base.pages.len() && self.width.is_finite() && self.height.is_finite()
    }
}

impl MutationKind<PdfSnapshot, PdfMutation> for ResizePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "page", kind: "resize-page", record: "ResizedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.resize-page.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: self.index, diff: PdfPageDiff { width: Some(self.width), height: Some(self.height), text: None } }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfMutation::ResizePage(ResizePage { index: self.index, width: base.pages[self.index].width, height: base.pages[self.index].height })]
    }

    fn label(&self) -> String {
        "resize page".into()
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
