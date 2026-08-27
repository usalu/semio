//! 🧳️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-trailer-entry`.

use super::remove_trailer_entry::RemoveTrailerEntry;
use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{diff::{self, PdfDiff}, snapshot::{PdfObject, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTrailerEntry {
    pub key: String,
    pub value: PdfObject,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetTrailerEntry {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "trailer-entry", kind: "set-trailer-entry", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_trailer_entry(base, &self.key, self.value.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        match base.trailer.iter().find(|entry| entry.key == self.key) { Some(entry) => vec![PdfMutation::SetTrailerEntry(SetTrailerEntry { key: self.key.clone(), value: entry.value.clone() })], None => vec![PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key: self.key.clone() })] }
    }

    fn label(&self) -> String {
        format!("Set trailer entry {}", self.key)
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
        assert_eq!(<SetTrailerEntry as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-trailer-entry");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
