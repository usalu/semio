//! ↩️ Inverse for `RemoveTableRow`.
use super::RemoveTableRow;
use crate::schema::mutations::InsertTableRow;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveTableRow, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Table { rows, .. }) if rows.len() > 1 => vec![NoteMutation::InsertTableRow(InsertTableRow { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
