//! 🗑️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-page`.

use super::insert_page::InsertPage;
use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovePage {
    pub index: usize,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemovePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "page", kind: "remove-page", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
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
mod tests {
    use super::*;

    #[test]
    fn semantic_identity_is_owned_by_this_leaf() {
        assert_eq!(<RemovePage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-page");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
