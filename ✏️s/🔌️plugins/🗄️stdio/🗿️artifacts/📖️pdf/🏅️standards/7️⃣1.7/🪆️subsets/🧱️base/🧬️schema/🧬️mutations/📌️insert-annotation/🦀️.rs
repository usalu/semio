//! 📌️ Authoritative PDF mutation payload, diff, inverse, and tests for `insert-annotation`.

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
pub struct InsertAnnotation {
    pub index: usize,
    pub at: usize,
    pub annotation: PdfAnnotation,
}

impl MutationKind<PdfSnapshot, PdfMutation> for InsertAnnotation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "annotation", kind: "insert-annotation", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_insert_annotation(self.index, self.at, self.annotation.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        vec![PdfMutation::RemoveAnnotation(super::remove_annotation::RemoveAnnotation { index: self.index, at: self.at })]
    }

    fn label(&self) -> String {
        format!("Insert annotation at {} on page {}", self.at, self.index)
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
