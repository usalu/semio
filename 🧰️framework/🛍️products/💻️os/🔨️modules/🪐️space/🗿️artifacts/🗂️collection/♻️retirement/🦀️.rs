//! ♻️ Exact field ownership for persisted collection documents.

use crate::{ArtifactBody, CollectionEntry, CollectionFolder, CollectionMutation, CollectionSnapshot};
use store::os_store::retirement::{RetireOwned, RetirementCursor};
use store::{artifact_retire_struct as retire_struct, artifact_retirement_sequence as seq};

retire_struct!(CollectionFolder { id, parent_id, name });
retire_struct!(CollectionEntry { id, folder_id, name, kind_id, body });
retire_struct!(CollectionSnapshot { schema, name, folders, entries });

impl RetireOwned for ArtifactBody {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self { Self::Document { schema, document_id } => seq![schema, document_id], Self::Blob { blob } => blob.retirement() }
    }
}

impl RetireOwned for CollectionMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::RenameCollection { new_name } => new_name.retirement(),
            Self::CreateFolder { folder, index } => seq![folder, index],
            Self::DeleteFolder { folder_id } => folder_id.retirement(),
            Self::MoveToCollection { folder_id, new_parent } => seq![folder_id, new_parent],
            Self::RenameFolder { folder_id, new_name } => seq![folder_id, new_name],
            Self::CreateEntry { entry, index } => seq![entry, index],
            Self::DeleteEntry { entry_id } => entry_id.retirement(),
            Self::MoveToFolder { entry_id, new_folder } => seq![entry_id, new_folder],
            Self::RenameEntry { entry_id, new_name } => seq![entry_id, new_name],
            Self::ReplaceEntryBody { entry_id, new_body } => seq![entry_id, new_body],
        }
    }
}
