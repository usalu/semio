//! 🧽️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-trailer-entry`.

use super::set_trailer_entry::SetTrailerEntry;
use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{diff::{self, PdfDiff}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveTrailerEntry {
    pub key: String,
}

impl MutationKind<PdfSnapshot, PdfMutation> for RemoveTrailerEntry {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "trailer-entry", kind: "remove-trailer-entry", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_remove_trailer_entry(base, &self.key))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        base.trailer.iter().find(|entry| entry.key == self.key).map(|entry| PdfMutation::SetTrailerEntry(SetTrailerEntry { key: self.key.clone(), value: entry.value.clone() })).into_iter().collect()
    }

    fn label(&self) -> String {
        format!("Remove trailer entry {}", self.key)
    }

    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_identity_is_owned_by_this_leaf() {
        assert_eq!(<RemoveTrailerEntry as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "remove-trailer-entry");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
