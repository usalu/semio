//! ✂️ `DetachLocalFolder` is the authoritative direct Rust leaf for forgetting the local folder one document is attached to on
//! this device.

use super::attach_local_folder::{attach_local_folder, LocalFolderBindings};
use super::LocalFoldersConfigMutation;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂️ Forgets the folder of one document, if it has one; the folder and the archive in it stay where they are.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DetachLocalFolder {
    pub document_id: String,
}

/// 🏗️ Wraps a detach payload in the local-folders dispatch enum.
pub fn detach_local_folder(document_id: &str) -> LocalFoldersConfigMutation {
    LocalFoldersConfigMutation::DetachLocalFolder(DetachLocalFolder { document_id: document_id.to_string() })
}

impl MutationKind<LocalFolderBindings, LocalFoldersConfigMutation> for DetachLocalFolder {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "local-folder", kind: "detach-local-folder", record: "Cleared" };

    fn diff(&self, base: &LocalFolderBindings) -> MutationOutcome<LocalFolderBindings> {
        if !base.bindings.iter().any(|entry| entry.document_id == self.document_id) {
            return MutationOutcome::new(base.clone()).warn("mutation.no-op", format!("\"{}\" is not attached to a folder on this device.", self.document_id));
        }
        MutationOutcome::new(LocalFolderBindings { bindings: base.bindings.iter().filter(|entry| entry.document_id != self.document_id).cloned().collect() })
    }

    fn inverse(&self, base: &LocalFolderBindings) -> Vec<LocalFoldersConfigMutation> {
        base.bindings.iter().find(|entry| entry.document_id == self.document_id).map(|prior| vec![attach_local_folder(prior.clone())]).unwrap_or_default()
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Forget the folder of \"{}\"", self.document_id), &format!("Ordner von \"{}\" vergessen", self.document_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.document_id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✂️forgets-the-folder-and-keeps-its-sibling/🦀️.rs"]
mod tests_forgets_the_folder_and_keeps_its_sibling;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
