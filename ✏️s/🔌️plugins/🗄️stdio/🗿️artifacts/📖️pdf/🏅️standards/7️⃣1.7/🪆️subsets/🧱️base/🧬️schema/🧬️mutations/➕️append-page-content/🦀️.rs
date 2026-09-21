//! ➕️ Authoritative PDF mutation payload, diff, inverse, and tests for `append-page-content`.

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
pub struct AppendPageContent {
    pub index: usize,
    pub content: Vec<PdfOp>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for AppendPageContent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "append", entity: "page-content", kind: "append-page-content", record: "Append" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_append_page_content(base, self.index, &self.content))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.pages.get(self.index).map(|page| PdfMutation::RemoveContent(super::remove_content::RemoveContent { index: self.index, at: page.content.len(), count: self.content.len() })).into_iter().collect()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Append {} operators to page {}", self.content.len(), self.index), &format!("{} Operatoren an Seite {} anhängen", self.content.len(), self.index))
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
