//! 📎️ `AttachLocalFolder` is the authoritative direct Rust leaf for remembering which local folder one program's document is
//! attached to on this device.

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

impl MutationDiff<LocalFolderBindings> for LocalFolderBindings {
    fn apply(&self, _base: &LocalFolderBindings) -> protocol::MutationApplyResult<LocalFolderBindings> {
        Ok(self.clone())
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
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

    fn diff(&self, base: &LocalFolderBindings) -> MutationOutcome<LocalFolderBindings> {
        let attached = LocalFolderBinding::from(self);
        if base.bindings.iter().any(|entry| *entry == attached) {
            return MutationOutcome::new(base.clone()).warning("mutation.no-op", format!("\"{}\" is already attached to this folder.", self.document_id));
        }
        let mut bindings: Vec<LocalFolderBinding> = base.bindings.iter().filter(|entry| entry.document_id != self.document_id).cloned().collect();
        bindings.push(attached);
        bindings.sort_by(|left, right| left.document_id.cmp(&right.document_id));
        MutationOutcome::new(LocalFolderBindings { bindings })
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
/// 🧮️ Applies one local-folders mutation through its whole-record diff.
pub fn apply_local_folders_config_mutation(snapshot: &mut LocalFolderBindings, mutation: &LocalFoldersConfigMutation) -> protocol::MutationApplyResult<()> {
    use protocol::{Mutation as _, MutationDiff as _};
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot)?;
    Ok(())
}

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

/// ▶️ Applies a mutation and returns its diagnostic `(code, severity)` pairs.
pub fn apply_local_folders_config_mutation_reporting(snapshot: &mut LocalFolderBindings, mutation: &LocalFoldersConfigMutation) -> Vec<(String, String)> {
    use protocol::Mutation as _;
    let outcome = mutation.diff(snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
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
