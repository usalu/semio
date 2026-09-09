//! 🗑️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-page`.

use super::insert_page::InsertPage;
use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemovePage {
    pub index: usize,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemovePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "page", kind: "remove-page", record: "Remove" };

    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_remove_page(self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        base.pages.get(self.index).cloned().map(|page| PdfMutation::InsertPage(InsertPage { index: self.index, page })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Remove page {}", self.index)
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
