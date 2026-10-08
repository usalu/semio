//! 🧭️ The details-pane vocabulary of the STEP AP214 class 5 editor: which JSON-pointer edit raises which concrete kind.
//!
//! The only header slot with a kind is `FILE_SCHEMA`, so [`EDIT_RULES`] names it alone; every edit of the entity list writes the edited entity
//! (or clears it) through the class's single `restore-entities` mutation, which [`special`] builds from the addressed entity. The file
//! description, the file name and the document schema have no kind in this class and are refused.

use super::*;
use crate::standards::v_ap214::engine::ladder;
use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, SnapshotEditError, SnapshotEditEvent};

/// 📚 The header slot this class edits.
pub const EDIT_RULES: EditRules = EditRules { entities: &[EntityRule::new("/header/fileSchema/schemas", "set-file-schema", "schemas")], inserts: &[], removes: &[] };

/// 🎯 The `restore-entities` mutation of an edit below `/entities/<row>`; `None` hands the edit on to the table.
pub fn special(event: &SnapshotEditEvent, snapshot: &StepSnapshot) -> Result<Option<Vec<StepCc5Mutation>>, SnapshotEditError> {
    Ok(ladder::edit_restore_rows(event, snapshot)?.map(restored))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
