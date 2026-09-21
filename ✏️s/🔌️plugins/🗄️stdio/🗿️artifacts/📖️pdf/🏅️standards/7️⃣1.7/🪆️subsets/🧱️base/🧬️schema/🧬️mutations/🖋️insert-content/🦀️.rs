//! 🖋️ Authoritative PDF mutation payload, diff, inverse, and tests for `insert-content`.

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
pub struct InsertContent {
    pub index: usize,
    pub at: usize,
    pub content: Vec<PdfOp>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for InsertContent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "content", kind: "insert-content", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_insert_content(self.index, self.at, &self.content))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        vec![PdfMutation::RemoveContent(super::remove_content::RemoveContent { index: self.index, at: self.at, count: self.content.len() })]
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Insert {} operators at {} on page {}", self.content.len(), self.at, self.index), &format!("{} Operatoren an {} auf Seite {} einfügen", self.content.len(), self.at, self.index))
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
