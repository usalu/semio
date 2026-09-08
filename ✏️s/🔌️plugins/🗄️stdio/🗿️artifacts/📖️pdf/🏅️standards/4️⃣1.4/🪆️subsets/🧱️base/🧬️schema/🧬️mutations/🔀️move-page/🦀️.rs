//! 🔀️ Direct move-page payload, sparse diff, concrete inverse, and laws.

use super::PdfMutation;
use crate::standards::v1_4::subsets::base::schema::{
    diff::{PdfDiff, PdfPageAdded, PdfPagesDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct MovePage {
    pub from: usize,
    pub to: usize,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl MovePage {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        self.from < base.pages.len() && self.to < base.pages.len()
    }
}

impl MutationKind<PdfSnapshot, PdfMutation> for MovePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "page", kind: "move-page", record: "MovedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.move-page.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        if self.from == self.to {
            return MutationOutcome::new(PdfDiff::default());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { removed: vec![self.from], added: vec![PdfPageAdded { index: self.to, page: base.pages[self.from].clone() }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if !self.valid(base) || self.from == self.to {
            return Vec::new();
        }
        vec![PdfMutation::MovePage(MovePage { from: self.to, to: self.from })]
    }

    fn label(&self) -> String {
        "move page".into()
    }

    fn target(&self) -> Vec<String> {
        vec![self.from.to_string()]
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
