//! ↩️ Inverse for `InsertTableColumn`.
use super::InsertTableColumn;
use crate::schema::mutations::{NoteMutation, RemoveTableColumn};
use crate::{NoteBlockNode, NoteSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &InsertTableColumn, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok(match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(NoteBlockNode::Table { rows, .. }) if payload.cells.as_ref().is_none_or(|cells| cells.len() == rows.len()) => vec![NoteMutation::RemoveTableColumn(RemoveTableColumn { id: payload.id.clone() })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
