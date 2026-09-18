//! 🔁️ Authoritative PDF mutation payload, diff, inverse, and tests for `replace-content`.

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
pub struct ReplaceContent {
    pub index: usize,
    pub at: usize,
    pub op: PdfOp,
}

impl MutationKind<PdfSnapshot, PdfMutation> for ReplaceContent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "content", kind: "replace-content", record: "Replace" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_replace_content(self.index, self.at, self.op.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.pages.get(self.index).and_then(|page| page.content.get(self.at)).map(|op| PdfMutation::ReplaceContent(ReplaceContent { index: self.index, at: self.at, op: op.clone() })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Replace operator {} on page {}", self.at, self.index)
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
