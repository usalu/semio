//! ↩️ Inverse for `RemoveTableRow`.
use super::RemoveTableRow;
use crate::schema::mutations::{InsertTableRow, NoteMutation};
use crate::{NoteBlockNode, NoteSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveTableRow, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok(match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(NoteBlockNode::Table { rows, .. }) if rows.len() > 1 => vec![NoteMutation::InsertTableRow(InsertTableRow { id: payload.id.clone(), cells: rows.last().cloned() })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
