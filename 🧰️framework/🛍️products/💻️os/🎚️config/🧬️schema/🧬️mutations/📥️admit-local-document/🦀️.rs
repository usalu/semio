//! 📥️ `AdmitLocalDocument` is the authoritative direct Rust leaf for listing one document in this device's local catalog.

use semio_framework_os_kernel::ColdMutation;
use semio_framework_os_kernel::ColdMutationDiff;
use semio_framework_os_kernel::ColdDiffAlgebra;
use super::super::{absorb_keyed_rows, KeyedEdit};
use super::retire_local_document::RetireLocalDocument;
use super::LocalCatalogConfigMutation;
use protocol::{MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
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

/// 🪟️ The selected presentation coordinate of a kept artifact.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", try_from = "LocalDocumentSurfaceFields")]
#[value(rename_all = "camelCase", deny_unknown_fields, validate = "validate_surface", validate_controlled = "validate_surface_controlled")]
pub struct LocalDocumentSurface {
    pub plugin_id: String,
    pub app_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LocalDocumentSurfaceFields { plugin_id: String, app_id: String }

fn validate_surface(surface: &LocalDocumentSurface) -> Result<(), semio_framework_value::ValueError> {
    if [&surface.plugin_id, &surface.app_id].into_iter().any(|id| id.is_empty() || id.len() > 1024 || id.chars().count() > 256) {
        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "selected surface ids require 1 to 256 characters"));
    }
    Ok(())
}

fn validate_surface_controlled(surface: &LocalDocumentSurface, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), semio_framework_value::ValueError> {
    let work = surface.plugin_id.len().saturating_add(surface.app_id.len()).saturating_add(2);
    control.begin_stage(work)?;
    control.advance(work)?;
    validate_surface(surface)
}

impl TryFrom<LocalDocumentSurfaceFields> for LocalDocumentSurface {
    type Error = semio_framework_value::ValueError;
    fn try_from(value: LocalDocumentSurfaceFields) -> Result<Self, Self::Error> {
        let surface = Self { plugin_id: value.plugin_id, app_id: value.app_id };
        validate_surface(&surface)?;
        Ok(surface)
    }
}

fn deserialize_surface<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<LocalDocumentSurface>, D::Error> {
    LocalDocumentSurface::deserialize(deserializer).map(Some)
}

fn surface_from_value(value: semio_framework_value::DslValue) -> Result<Option<LocalDocumentSurface>, semio_framework_value::ValueError> {
    <LocalDocumentSurface as semio_framework_value::FromValue>::from_value(value).map(Some)
}

fn surface_from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Option<LocalDocumentSurface>, semio_framework_value::ValueError> {
    <LocalDocumentSurface as semio_framework_value::FromValue>::from_value_controlled(value, control).map(Some)
}

fn surface_default_controlled(control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Option<LocalDocumentSurface>, semio_framework_value::ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(1)?;
        control.step()?;
        Ok(None)
    })
}

fn retire_surface(value: Option<LocalDocumentSurface>) {
    if let Some(surface) = value { <LocalDocumentSurface as semio_framework_value::FromValue>::retire_decoded(surface); }
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
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_surface")]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "surface_from_value", deserialize_controlled_with = "surface_from_value_controlled", default_controlled = "surface_default_controlled", retire_with = "retire_surface")]
    pub surface: Option<LocalDocumentSurface>,
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



}

impl ColdMutationDiff<LocalCatalog> for LocalCatalogDiff {
    fn absorb(&mut self, other: Self) {
        absorb_keyed_rows(&mut self.documents, other.documents);
    }

    fn apply(&self, base: &mut LocalCatalog, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let mut documents = base.documents.clone();
        for row in &self.documents {
            documents.retain(|entry| entry.document_id != row.key);
            if let Some(document) = &row.value {
                documents.push(document.clone());
            }
        }
        documents.sort_by(|left, right| left.document_id.cmp(&right.document_id));
                base.documents = documents;
        Ok(())
    }
}

impl protocol::DiffAlgebra<LocalCatalog> for LocalCatalogDiff {


    fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<LocalCatalog> for LocalCatalogDiff {
    fn inverse(&self, base: &LocalCatalog) -> Self {
        Self { documents: self.documents.iter().map(|row| KeyedEdit::new(row.key.clone(), base.documents.iter().find(|entry| entry.document_id == row.key).cloned())).collect() }
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
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_surface")]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "surface_from_value", deserialize_controlled_with = "surface_from_value_controlled", default_controlled = "surface_default_controlled", retire_with = "retire_surface")]
    pub surface: Option<LocalDocumentSurface>,
}

/// 🏗️ Wraps an admit payload in the local-catalog dispatch enum.
pub fn admit_local_document(document: LocalDocument) -> LocalCatalogConfigMutation {
    LocalCatalogConfigMutation::AdmitLocalDocument(AdmitLocalDocument::from(document))
}

impl From<LocalDocument> for AdmitLocalDocument {
    fn from(document: LocalDocument) -> Self {
        Self { document_id: document.document_id, schema: document.schema, name: document.name, storage: document.storage, target: document.target, admitted_at_ms: document.admitted_at_ms, surface: document.surface }
    }
}

impl From<&AdmitLocalDocument> for LocalDocument {
    fn from(payload: &AdmitLocalDocument) -> Self {
        Self { document_id: payload.document_id.clone(), schema: payload.schema.clone(), name: payload.name.clone(), storage: payload.storage, target: payload.target.clone(), admitted_at_ms: payload.admitted_at_ms, surface: payload.surface.clone() }
    }
}

impl MutationKind<LocalCatalog, LocalCatalogConfigMutation> for AdmitLocalDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "local-document", kind: "admit-local-document", record: "Set" };





    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Keep \"{}\" on this device", self.name), &format!("\"{}\" auf diesem Gerät behalten", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.document_id.clone()]
    }
}

impl semio_framework_os_command::ColdMutationKind<LocalCatalog, LocalCatalogConfigMutation> for AdmitLocalDocument {
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
