//! 📥️ Authoritative PDF mutation payload, diff, inverse, and tests for `insert-page`.

use super::remove_page::RemovePage;
use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{diff::{self, PdfDiff}, snapshot::{PdfPage, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertPage {
    pub index: usize,
    pub page: PdfPage,
}

impl MutationKind<PdfSnapshot, PdfMutation> for InsertPage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "page", kind: "insert-page", record: "Insert" };

    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_insert_page(self.index, self.page.clone()))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfMutation> {
        vec![PdfMutation::RemovePage(RemovePage { index: self.index })]
    }

    fn label(&self) -> String {
        format!("Insert page {}", self.index)
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

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
