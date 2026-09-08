//! ♻️ Exact field ownership for persisted space manifests.

use crate::{CollectionRef, InstalledExtension, SpaceKind, SpaceMutation, SpaceRole, SpaceSnapshot, SpaceUser, SpaceVisibility};
use store::os_store::retirement::{RetireOwned, RetirementCursor};
use store::{artifact_retire_leaf as retire_leaf, artifact_retire_struct as retire_struct, artifact_retirement_sequence as seq};

retire_leaf!(SpaceKind, SpaceVisibility, SpaceRole);
retire_struct!(SpaceUser { id, name, avatar, role });
retire_struct!(CollectionRef { id, name, document_id });
retire_struct!(InstalledExtension { extension_id, version, source_uri, package_hash, enabled });
retire_struct!(SpaceSnapshot { schema, name, kind, visibility, users, collections, programs, extensions });

impl RetireOwned for SpaceMutation {
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
