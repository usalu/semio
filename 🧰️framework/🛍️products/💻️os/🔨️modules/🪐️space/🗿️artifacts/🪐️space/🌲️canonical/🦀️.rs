//! 🌲️ Space mutations project the externally tagged `ToValue` wire `{Variant:{field:..}}` directly from the original owner.
use super::SpaceMutation;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError, ValueRefusalKind};

fn absent(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, reason) }

/// 🪟️ Borrowed view of the variant's field record; it has the layout of the mutation it is cast from.
#[repr(transparent)]
struct Fields(SpaceMutation);

fn variant(mutation: &SpaceMutation) -> &'static str {
    match mutation {
        SpaceMutation::SetName { .. } => "SetName",
        SpaceMutation::SetKind { .. } => "SetKind",
        SpaceMutation::SetVisibility { .. } => "SetVisibility",
        SpaceMutation::UpsertUser { .. } => "UpsertUser",
        SpaceMutation::RemoveUser { .. } => "RemoveUser",
        SpaceMutation::AddCollection { .. } => "AddCollection",
        SpaceMutation::RemoveCollection { .. } => "RemoveCollection",
        SpaceMutation::RenameCollection { .. } => "RenameCollection",
        SpaceMutation::InstallProgram { .. } => "InstallProgram",
        SpaceMutation::UninstallProgram { .. } => "UninstallProgram",
        SpaceMutation::InstallExtension { .. } => "InstallExtension",
        SpaceMutation::UninstallExtension { .. } => "UninstallExtension",
        SpaceMutation::SetExtensionEnabled { .. } => "SetExtensionEnabled",
    }
}

fn keys(mutation: &SpaceMutation) -> &'static [&'static str] {
    match mutation {
        SpaceMutation::SetName { .. } => &["name"],
        SpaceMutation::SetKind { .. } => &["kind"],
        SpaceMutation::SetVisibility { .. } => &["visibility"],
        SpaceMutation::UpsertUser { .. } => &["user", "index"],
        SpaceMutation::RemoveUser { .. } => &["user_id"],
        SpaceMutation::AddCollection { .. } => &["collection", "index"],
        SpaceMutation::RemoveCollection { .. } => &["collection_id"],
        SpaceMutation::RenameCollection { .. } => &["collection_id", "name"],
        SpaceMutation::InstallProgram { .. } => &["plugin_id", "index"],
        SpaceMutation::UninstallProgram { .. } => &["plugin_id"],
        SpaceMutation::InstallExtension { .. } => &["extension_id", "version", "source_uri", "package_hash", "enabled", "index"],
        SpaceMutation::UninstallExtension { .. } => &["extension_id"],
        SpaceMutation::SetExtensionEnabled { .. } => &["extension_id", "enabled"],
    }
}

impl Tree for SpaceMutation {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Object(1)) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        match ordinal {
            // SAFETY: `Fields` is `repr(transparent)` over exactly the borrowed mutation.
            0 => Ok(unsafe { &*(self as *const SpaceMutation as *const Fields) }),
            _ => Err(absent("canonical mutation ordinal is absent")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        match ordinal { 0 => Ok(variant(self).into()), _ => Err(absent("canonical mutation key is absent")) }
    }
}

impl Tree for Fields {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Object(keys(&self.0).len())) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        match (&self.0, ordinal) {
            (SpaceMutation::SetName { name, .. }, 0) => Ok(name),
            (SpaceMutation::SetKind { kind, .. }, 0) => Ok(kind),
            (SpaceMutation::SetVisibility { visibility, .. }, 0) => Ok(visibility),
            (SpaceMutation::UpsertUser { user, .. }, 0) => Ok(user),
            (SpaceMutation::UpsertUser { index, .. }, 1) => Ok(index),
            (SpaceMutation::RemoveUser { user_id, .. }, 0) => Ok(user_id),
            (SpaceMutation::AddCollection { collection, .. }, 0) => Ok(collection),
            (SpaceMutation::AddCollection { index, .. }, 1) => Ok(index),
            (SpaceMutation::RemoveCollection { collection_id, .. }, 0) => Ok(collection_id),
            (SpaceMutation::RenameCollection { collection_id, .. }, 0) => Ok(collection_id),
            (SpaceMutation::RenameCollection { name, .. }, 1) => Ok(name),
            (SpaceMutation::InstallProgram { plugin_id, .. }, 0) => Ok(plugin_id),
            (SpaceMutation::InstallProgram { index, .. }, 1) => Ok(index),
            (SpaceMutation::UninstallProgram { plugin_id, .. }, 0) => Ok(plugin_id),
            (SpaceMutation::InstallExtension { extension_id, .. }, 0) => Ok(extension_id),
            (SpaceMutation::InstallExtension { version, .. }, 1) => Ok(version),
            (SpaceMutation::InstallExtension { source_uri, .. }, 2) => Ok(source_uri),
            (SpaceMutation::InstallExtension { package_hash, .. }, 3) => Ok(package_hash),
            (SpaceMutation::InstallExtension { enabled, .. }, 4) => Ok(enabled),
            (SpaceMutation::InstallExtension { index, .. }, 5) => Ok(index),
            (SpaceMutation::UninstallExtension { extension_id, .. }, 0) => Ok(extension_id),
            (SpaceMutation::SetExtensionEnabled { extension_id, .. }, 0) => Ok(extension_id),
            (SpaceMutation::SetExtensionEnabled { enabled, .. }, 1) => Ok(enabled),
            _ => Err(absent("canonical mutation field ordinal is absent")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        keys(&self.0).get(ordinal).map(|key| Text::from(*key)).ok_or_else(|| absent("canonical mutation field key is absent"))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
