//! 🌲️ Collection mutations project the externally tagged `ToValue` wire `{Variant:{field:..}}` directly from the original owner.
use super::CollectionMutation;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError, ValueRefusalKind};

fn absent(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, reason) }

/// 🪟️ Borrowed view of the variant's field record; it has the layout of the mutation it is cast from.
#[repr(transparent)]
struct Fields(CollectionMutation);

fn variant(mutation: &CollectionMutation) -> &'static str {
    match mutation {
        CollectionMutation::RenameCollection { .. } => "RenameCollection",
        CollectionMutation::CreateFolder { .. } => "CreateFolder",
        CollectionMutation::DeleteFolder { .. } => "DeleteFolder",
        CollectionMutation::MoveToCollection { .. } => "MoveToCollection",
        CollectionMutation::RenameFolder { .. } => "RenameFolder",
        CollectionMutation::CreateEntry { .. } => "CreateEntry",
        CollectionMutation::DeleteEntry { .. } => "DeleteEntry",
        CollectionMutation::MoveToFolder { .. } => "MoveToFolder",
        CollectionMutation::RenameEntry { .. } => "RenameEntry",
        CollectionMutation::ReplaceEntryBody { .. } => "ReplaceEntryBody",
    }
}

fn keys(mutation: &CollectionMutation) -> &'static [&'static str] {
    match mutation {
        CollectionMutation::RenameCollection { .. } => &["new_name"],
        CollectionMutation::CreateFolder { .. } => &["folder", "index"],
        CollectionMutation::DeleteFolder { .. } => &["folder_id"],
        CollectionMutation::MoveToCollection { .. } => &["folder_id", "new_parent"],
        CollectionMutation::RenameFolder { .. } => &["folder_id", "new_name"],
        CollectionMutation::CreateEntry { .. } => &["entry", "index"],
        CollectionMutation::DeleteEntry { .. } => &["entry_id"],
        CollectionMutation::MoveToFolder { .. } => &["entry_id", "new_folder"],
        CollectionMutation::RenameEntry { .. } => &["entry_id", "new_name"],
        CollectionMutation::ReplaceEntryBody { .. } => &["entry_id", "new_body"],
    }
}

impl Tree for CollectionMutation {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Object(1)) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        match ordinal {
            // SAFETY: `Fields` is `repr(transparent)` over exactly the borrowed mutation.
            0 => Ok(unsafe { &*(self as *const CollectionMutation as *const Fields) }),
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
            (CollectionMutation::RenameCollection { new_name, .. }, 0) => Ok(new_name),
            (CollectionMutation::CreateFolder { folder, .. }, 0) => Ok(folder),
            (CollectionMutation::CreateFolder { index, .. }, 1) => Ok(index),
            (CollectionMutation::DeleteFolder { folder_id, .. }, 0) => Ok(folder_id),
            (CollectionMutation::MoveToCollection { folder_id, .. }, 0) => Ok(folder_id),
            (CollectionMutation::MoveToCollection { new_parent, .. }, 1) => Ok(new_parent),
            (CollectionMutation::RenameFolder { folder_id, .. }, 0) => Ok(folder_id),
            (CollectionMutation::RenameFolder { new_name, .. }, 1) => Ok(new_name),
            (CollectionMutation::CreateEntry { entry, .. }, 0) => Ok(entry),
            (CollectionMutation::CreateEntry { index, .. }, 1) => Ok(index),
            (CollectionMutation::DeleteEntry { entry_id, .. }, 0) => Ok(entry_id),
            (CollectionMutation::MoveToFolder { entry_id, .. }, 0) => Ok(entry_id),
            (CollectionMutation::MoveToFolder { new_folder, .. }, 1) => Ok(new_folder),
            (CollectionMutation::RenameEntry { entry_id, .. }, 0) => Ok(entry_id),
            (CollectionMutation::RenameEntry { new_name, .. }, 1) => Ok(new_name),
            (CollectionMutation::ReplaceEntryBody { entry_id, .. }, 0) => Ok(entry_id),
            (CollectionMutation::ReplaceEntryBody { new_body, .. }, 1) => Ok(new_body),
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
