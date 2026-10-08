//! 📥️ `AdmitLocalDocument` is the authoritative direct Rust leaf for listing one document in this device's local catalog.

use super::super::{absorb_keyed_rows, KeyedEdit};
use super::retire_local_document::RetireLocalDocument;
use super::LocalCatalogConfigMutation;
use protocol::{MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Schema
/// 🗄️ Where a local document's events persist on this device: a folder event log or one file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum LocalDocumentStorage {
    Folder,
    File,
}

/// 📄️ One document this device keeps locally: its id, artifact schema, name and where its events persist.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct LocalDocument {
    pub document_id: String,
    pub schema: String,
    pub name: String,
    pub storage: LocalDocumentStorage,
    pub target: String,
    pub admitted_at_ms: u64,
}

/// 🗂️ `os.config.local-catalog` — every document this device keeps locally (persisted local-only), ordered by id.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase", default)]
pub struct LocalCatalog {
    pub documents: Vec<LocalDocument>,
}

/// 🗂️ The schema id for the local document catalog config facet.
pub const LOCAL_CATALOG_CONFIG_SCHEMA: &str = "os.config.local-catalog";

/// 🔺️ Sparse diff of [`LocalCatalog`]: one absolute row per touched document id.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LocalCatalogDiff {
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub documents: Vec<KeyedEdit<LocalDocument>>,
}

impl MutationDiff<LocalCatalog> for LocalCatalogDiff {
    fn apply(&self, base: &LocalCatalog, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<LocalCatalog> {
        let mut documents = base.documents.clone();
        for row in &self.documents {
            documents.retain(|entry| entry.document_id != row.key);
            if let Some(document) = &row.value {
                documents.push(document.clone());
            }
        }
        documents.sort_by(|left, right| left.document_id.cmp(&right.document_id));
        Ok(LocalCatalog { documents })
    }

    fn absorb(&mut self, other: Self) {
        absorb_keyed_rows(&mut self.documents, other.documents);
    }
}

impl protocol::DiffAlgebra<LocalCatalog> for LocalCatalogDiff {
    fn inverse(&self, base: &LocalCatalog) -> Self {
        Self { documents: self.documents.iter().map(|row| KeyedEdit::new(row.key.clone(), base.documents.iter().find(|entry| entry.document_id == row.key).cloned())).collect() }
    }

    fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}
//#endregion 🔖️Schema

//#region 🔖️Mutation
/// 📥️ Lists one document in the local catalog, replacing any entry with the same id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AdmitLocalDocument {
    pub document_id: String,
    pub schema: String,
    pub name: String,
    pub storage: LocalDocumentStorage,
    pub target: String,
    pub admitted_at_ms: u64,
}

/// 🏗️ Wraps an admit payload in the local-catalog dispatch enum.
pub fn admit_local_document(document: LocalDocument) -> LocalCatalogConfigMutation {
    LocalCatalogConfigMutation::AdmitLocalDocument(AdmitLocalDocument::from(document))
}

impl From<LocalDocument> for AdmitLocalDocument {
    fn from(document: LocalDocument) -> Self {
        Self { document_id: document.document_id, schema: document.schema, name: document.name, storage: document.storage, target: document.target, admitted_at_ms: document.admitted_at_ms }
    }
}

impl From<&AdmitLocalDocument> for LocalDocument {
    fn from(payload: &AdmitLocalDocument) -> Self {
        Self { document_id: payload.document_id.clone(), schema: payload.schema.clone(), name: payload.name.clone(), storage: payload.storage, target: payload.target.clone(), admitted_at_ms: payload.admitted_at_ms }
    }
}

impl MutationKind<LocalCatalog, LocalCatalogConfigMutation> for AdmitLocalDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "local-document", kind: "admit-local-document", record: "Set" };

    fn diff(&self, base: &LocalCatalog) -> MutationOutcome<LocalCatalogDiff> {
        let admitted = LocalDocument::from(self);
        if base.documents.iter().any(|entry| *entry == admitted) {
            return MutationOutcome::new(LocalCatalogDiff::default()).warning("mutation.no-op", format!("\"{}\" is already listed in the local catalog.", self.document_id));
        }
        MutationOutcome::new(LocalCatalogDiff { documents: vec![KeyedEdit::new(self.document_id.clone(), Some(admitted))] })
    }

    fn inverse(&self, base: &LocalCatalog) -> Result<Vec<LocalCatalogConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        match base.documents.iter().find(|entry| entry.document_id == self.document_id) {
            Some(prior) => vec![admit_local_document(prior.clone())],
            None => vec![LocalCatalogConfigMutation::RetireLocalDocument(RetireLocalDocument { document_id: self.document_id.clone() })],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Keep \"{}\" on this device", self.name), &format!("\"{}\" auf diesem Gerät behalten", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.document_id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🌉️MutationCodecBridge
/// ↩️ Computes the mutation's inverse steps from the pre-mutation catalog.
pub fn inverse_local_catalog_config_mutation(snapshot: &LocalCatalog, mutation: &LocalCatalogConfigMutation) -> Result<Vec<LocalCatalogConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(snapshot)?

    })
}







/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_local_catalog_config_mutation_steps(mutation: &LocalCatalogConfigMutation, base: &LocalCatalog) -> Result<Vec<LocalCatalogConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(base)?

    })
}
//#endregion 🌉️MutationCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📥️lists-a-persisted-studio-beside-an-imported-one/🦀️.rs"]
mod tests_lists_a_persisted_studio_beside_an_imported_one;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
