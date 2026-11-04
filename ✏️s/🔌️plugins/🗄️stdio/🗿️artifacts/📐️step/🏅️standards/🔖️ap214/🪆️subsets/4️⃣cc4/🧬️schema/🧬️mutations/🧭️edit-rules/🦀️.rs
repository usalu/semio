//! 🧭️ The details-pane vocabulary of the STEP AP214 class 4 editor: which JSON-pointer edit raises which concrete kind.
//!
//! The only header slot with a kind is `FILE_SCHEMA`, so [`EDIT_RULES`] names it alone; every edit of the entity list writes the edited entity
//! (or clears it) through the class's single `restore-entities` mutation, which [`special`] builds from the addressed entity. The file
//! description, the file name and the document schema have no kind in this class and are refused.

use super::*;
use semio_s_artifact_stdio_step_ladder::standards::v_ap214::subsets::base::io::ladder;
use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, SnapshotEditError, SnapshotEditEvent};

/// 📚 The header slot this class edits.
pub const EDIT_RULES: EditRules = EditRules { entities: &[EntityRule::new("/header/fileSchema/schemas", "set-file-schema", "schemas")], inserts: &[], removes: &[] };

/// 🎯 The kind of the product identity gesture (`/product`), of a shape representation gesture (`/shapeRepresentations/<id>`) and the `restore-entities` mutation of an edit below `/entities/<row>`; `None` hands the edit on to the table.
pub fn special(event: &SnapshotEditEvent, snapshot: &StepSnapshot) -> Result<Option<Vec<StepCc4Mutation>>, SnapshotEditError> {
    if let Some(rows) = ladder::transform_restore_rows(event, snapshot)? {
        return Ok(Some(restored(rows)));
    }
    if let Some(identity) = ladder::product_identity_edit(event, snapshot)? {
        return Ok(Some(vec![StepCc4Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity })]));
    }
    if let Some(edit) = ladder::shape_representation_edit(event, snapshot)? {
        return match edit {
            ladder::ShapeRepresentationEdit::Set { id, row } => Ok(Some(vec![StepCc4Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id, representation: Some(row), index: None })])),
            ladder::ShapeRepresentationEdit::Remove { id } => Ok(Some(vec![StepCc4Mutation::DemoteShapeRepresentation(demote_shape_representation::DemoteShapeRepresentation { id })])),
        };
    }
    Ok(ladder::edit_restore_rows(event, snapshot)?.map(restored))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
