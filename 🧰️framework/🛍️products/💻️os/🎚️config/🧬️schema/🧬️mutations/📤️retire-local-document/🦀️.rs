//! 📤️ `RetireLocalDocument` is the authoritative direct Rust leaf for unlisting one document from this device's local catalog.

use super::admit_local_document::{admit_local_document, LocalCatalog};
use super::LocalCatalogConfigMutation;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📤️ Removes one document from the local catalog, if listed; its own events stay where they persist.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct RetireLocalDocument {
    pub document_id: String,
}

/// 🏗️ Wraps a retire payload in the local-catalog dispatch enum.
pub fn retire_local_document(document_id: &str) -> LocalCatalogConfigMutation {
    LocalCatalogConfigMutation::RetireLocalDocument(RetireLocalDocument { document_id: document_id.to_string() })
}

impl MutationKind<LocalCatalog, LocalCatalogConfigMutation> for RetireLocalDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "local-document", kind: "retire-local-document", record: "Cleared" };

    fn diff(&self, base: &LocalCatalog) -> MutationOutcome<LocalCatalog> {
        if !base.documents.iter().any(|entry| entry.document_id == self.document_id) {
            return MutationOutcome::new(base.clone()).warn("mutation.no-op", format!("\"{}\" is not listed in the local catalog.", self.document_id));
        }
        MutationOutcome::new(LocalCatalog { documents: base.documents.iter().filter(|entry| entry.document_id != self.document_id).cloned().collect() })
    }

    fn inverse(&self, base: &LocalCatalog) -> Vec<LocalCatalogConfigMutation> {
        base.documents.iter().find(|entry| entry.document_id == self.document_id).map(|prior| vec![admit_local_document(prior.clone())]).unwrap_or_default()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Stop keeping \"{}\" on this device", self.document_id), &format!("\"{}\" nicht mehr auf diesem Gerät behalten", self.document_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.document_id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📤️unlists-the-studio-and-keeps-its-sibling/🦀️.rs"]
mod tests_unlists_the_studio_and_keeps_its_sibling;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
