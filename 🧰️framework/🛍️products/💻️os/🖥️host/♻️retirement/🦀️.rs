//! ♻️ Exact field ownership for persisted OS space and collection documents.

use crate::space;
use store::os_store::retirement::{RetireOwned, RetirementCursor};
use store::{artifact_retire_leaf as retire_leaf, artifact_retire_struct as retire_struct, artifact_retirement_sequence as seq};

retire_leaf!(space::SpaceKind, space::SpaceVisibility, space::SpaceRole);
retire_struct!(space::SpaceUser { id, name, avatar, role });
retire_struct!(space::CollectionRef { id, name, document_id });
retire_struct!(space::InstalledExtension { extension_id, version, source_uri, package_hash, enabled });
retire_struct!(space::SpaceSnapshot { schema, name, kind, visibility, users, collections, programs, extensions });
retire_struct!(space::CollectionFolder { id, parent_id, name });
retire_struct!(space::CollectionEntry { id, folder_id, name, kind_id, body });
retire_struct!(space::CollectionSnapshot { schema, name, folders, entries });

impl RetireOwned for space::ArtifactBody {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self { Self::Document { schema, document_id } => seq![schema, document_id], Self::Blob { blob } => blob.retirement() }
    }
}

impl RetireOwned for space::SpaceMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetName { name } => name.retirement(),
            Self::SetKind { kind } => kind.retirement(),
            Self::SetVisibility { visibility } => visibility.retirement(),
            Self::UpsertUser { user } => user.retirement(),
            Self::RemoveUser { user_id } => user_id.retirement(),
            Self::AddCollection { collection } => collection.retirement(),
            Self::RemoveCollection { collection_id } => collection_id.retirement(),
            Self::RenameCollection { collection_id, name } => seq![collection_id, name],
            Self::InstallProgram { plugin_id } | Self::UninstallProgram { plugin_id } => plugin_id.retirement(),
            Self::InstallExtension { extension_id, version, source_uri, package_hash, enabled } => seq![extension_id, version, source_uri, package_hash, enabled],
            Self::UninstallExtension { extension_id } => extension_id.retirement(),
            Self::SetExtensionEnabled { extension_id, enabled } => seq![extension_id, enabled],
        }
    }
}

impl RetireOwned for space::CollectionMutation {
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
