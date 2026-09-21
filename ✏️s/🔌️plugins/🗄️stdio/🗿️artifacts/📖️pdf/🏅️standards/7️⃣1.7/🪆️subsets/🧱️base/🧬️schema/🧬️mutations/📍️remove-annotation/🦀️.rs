//! 📍️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-annotation`.

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
pub struct RemoveAnnotation {
    pub index: usize,
    pub at: usize,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemoveAnnotation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "annotation", kind: "remove-annotation", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_annotation(self.index, self.at))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.pages.get(self.index).and_then(|page| page.annotations.get(self.at)).map(|annotation| PdfMutation::InsertAnnotation(super::insert_annotation::InsertAnnotation { index: self.index, at: self.at, annotation: annotation.clone() })).into_iter().collect()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Remove annotation {} on page {}", self.at, self.index), &format!("Anmerkung {} auf Seite {} entfernen", self.at, self.index))
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
