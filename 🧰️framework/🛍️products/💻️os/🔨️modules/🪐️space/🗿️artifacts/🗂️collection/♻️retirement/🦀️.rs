//! ♻️ Exact field ownership for persisted collection documents.

use crate::CollectionMutation;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value::artifact_retirement_sequence as seq;

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
