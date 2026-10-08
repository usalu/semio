//! 📎️ `AttachLocalFolder` is the authoritative direct Rust leaf for remembering which local folder one program's document is
//! attached to on this device.

use super::super::{absorb_keyed_rows, KeyedEdit};
use super::detach_local_folder::DetachLocalFolder;
use super::LocalFoldersConfigMutation;
use protocol::{MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Schema
/// 📁️ Where a document's folder is on this device: an absolute path the host opens.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum LocalFolderRef {
    Path { path: String },
}

/// 📎️ One document this device attached to a local folder: the document's own identity, the program that holds it and the
/// folder its archive persists in.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct LocalFolderBinding {
    pub document_id: String,
    pub plugin_id: String,
    pub app_id: String,
    pub folder: LocalFolderRef,
}

/// 📁️ `os.config.local-folders` — every folder binding of this device (persisted local-only: never shared, never in a URL),
/// one per document, ordered by document id.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase", default)]
pub struct LocalFolderBindings {
    pub bindings: Vec<LocalFolderBinding>,
}

/// 📁️ The schema id for the local folder binding config facet.
pub const LOCAL_FOLDERS_CONFIG_SCHEMA: &str = "os.config.local-folders";

/// 🔺️ Sparse diff of [`LocalFolderBindings`]: one absolute row per touched document id.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LocalFoldersDiff {
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub bindings: Vec<KeyedEdit<LocalFolderBinding>>,
}

impl MutationDiff<LocalFolderBindings> for LocalFoldersDiff {
    fn apply(&self, base: &LocalFolderBindings, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<LocalFolderBindings> {
        let mut bindings = base.bindings.clone();
        for row in &self.bindings {
            bindings.retain(|entry| entry.document_id != row.key);
            if let Some(binding) = &row.value {
                bindings.push(binding.clone());
            }
        }
        bindings.sort_by(|left, right| left.document_id.cmp(&right.document_id));
        Ok(LocalFolderBindings { bindings })
    }

    fn absorb(&mut self, other: Self) {
        absorb_keyed_rows(&mut self.bindings, other.bindings);
    }
}

impl protocol::DiffAlgebra<LocalFolderBindings> for LocalFoldersDiff {
    fn inverse(&self, base: &LocalFolderBindings) -> Self {
        Self { bindings: self.bindings.iter().map(|row| KeyedEdit::new(row.key.clone(), base.bindings.iter().find(|entry| entry.document_id == row.key).cloned())).collect() }
    }

    fn between(base: &LocalFolderBindings, other: &LocalFolderBindings) -> Self {
        let mut bindings: Vec<KeyedEdit<LocalFolderBinding>> = base
            .bindings
            .iter()
            .filter(|entry| other.bindings.iter().find(|candidate| candidate.document_id == entry.document_id) != Some(*entry))
            .map(|entry| KeyedEdit::new(entry.document_id.clone(), other.bindings.iter().find(|candidate| candidate.document_id == entry.document_id).cloned()))
            .collect();
        bindings.extend(other.bindings.iter().filter(|entry| !base.bindings.iter().any(|candidate| candidate.document_id == entry.document_id)).map(|entry| KeyedEdit::new(entry.document_id.clone(), Some(entry.clone()))));
        Self { bindings }
    }

    fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}
//#endregion 🔖️Schema

//#region 🔖️Mutation
/// 📎️ Remembers one document's folder, replacing any binding of the same document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AttachLocalFolder {
    pub document_id: String,
    pub plugin_id: String,
    pub app_id: String,
    pub folder: LocalFolderRef,
}

/// 🏗️ Wraps an attach payload in the local-folders dispatch enum.
pub fn attach_local_folder(binding: LocalFolderBinding) -> LocalFoldersConfigMutation {
    LocalFoldersConfigMutation::AttachLocalFolder(AttachLocalFolder::from(binding))
}

impl From<LocalFolderBinding> for AttachLocalFolder {
    fn from(binding: LocalFolderBinding) -> Self {
        Self { document_id: binding.document_id, plugin_id: binding.plugin_id, app_id: binding.app_id, folder: binding.folder }
    }
}

impl From<&AttachLocalFolder> for LocalFolderBinding {
    fn from(payload: &AttachLocalFolder) -> Self {
        Self { document_id: payload.document_id.clone(), plugin_id: payload.plugin_id.clone(), app_id: payload.app_id.clone(), folder: payload.folder.clone() }
    }
}

impl MutationKind<LocalFolderBindings, LocalFoldersConfigMutation> for AttachLocalFolder {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "local-folder", kind: "attach-local-folder", record: "Set" };

    fn diff(&self, base: &LocalFolderBindings) -> MutationOutcome<LocalFoldersDiff> {
        let attached = LocalFolderBinding::from(self);
        if base.bindings.iter().any(|entry| *entry == attached) {
            return MutationOutcome::new(LocalFoldersDiff::default()).warning("mutation.no-op", format!("\"{}\" is already attached to this folder.", self.document_id));
        }
        MutationOutcome::new(LocalFoldersDiff { bindings: vec![KeyedEdit::new(self.document_id.clone(), Some(attached))] })
    }

    fn inverse(&self, base: &LocalFolderBindings) -> Result<Vec<LocalFoldersConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        match base.bindings.iter().find(|entry| entry.document_id == self.document_id) {
            Some(prior) => vec![attach_local_folder(prior.clone())],
            None => vec![LocalFoldersConfigMutation::DetachLocalFolder(DetachLocalFolder { document_id: self.document_id.clone() })],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Attach \"{}\" to a folder on this device", self.document_id), &format!("\"{}\" mit einem Ordner auf diesem Gerät verbinden", self.document_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.document_id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🌉️MutationCodecBridge
/// ↩️ Computes the mutation's inverse steps from the pre-mutation bindings.
pub fn inverse_local_folders_config_mutation(snapshot: &LocalFolderBindings, mutation: &LocalFoldersConfigMutation) -> Result<Vec<LocalFoldersConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(snapshot)?

    })
}

/// 📥️ Decodes the internally tagged local-folders mutation JSON projection.
pub fn decode_local_folders_config_mutation_json(text: &str) -> Result<LocalFoldersConfigMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📤️ Encodes the local folder bindings to their canonical camel-case JSON projection.
pub fn encode_local_folder_bindings_json(snapshot: &LocalFolderBindings) -> String {
    serde_json::to_string(snapshot).expect("LocalFolderBindings serialization is infallible")
}

/// 📥️ Decodes the canonical local folder bindings JSON projection.
pub fn decode_local_folder_bindings_json(text: &str) -> Result<LocalFolderBindings, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_local_folders_config_mutation_steps(mutation: &LocalFoldersConfigMutation, base: &LocalFolderBindings) -> Result<Vec<LocalFoldersConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(base)?

    })
}
//#endregion 🌉️MutationCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📎️remembers-the-folder-beside-another-document/🦀️.rs"]
mod tests_remembers_the_folder_beside_another_document;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
