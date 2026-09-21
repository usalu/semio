//! 🧻️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-content`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveContent {
    pub index: usize,
    pub at: usize,
    pub count: usize,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemoveContent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "content", kind: "remove-content", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_content(self.index, self.at, self.count))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.pages.get(self.index).filter(|page| self.at + self.count <= page.content.len()).map(|page| PdfMutation::InsertContent(super::insert_content::InsertContent { index: self.index, at: self.at, content: page.content[self.at..self.at + self.count].to_vec() })).into_iter().collect()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Remove {} operators at {} on page {}", self.count, self.at, self.index), &format!("{} Operatoren an {} auf Seite {} entfernen", self.count, self.at, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string(), self.at.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
