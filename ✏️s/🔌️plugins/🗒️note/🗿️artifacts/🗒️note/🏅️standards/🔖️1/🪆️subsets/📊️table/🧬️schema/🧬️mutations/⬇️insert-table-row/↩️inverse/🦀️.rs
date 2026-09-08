//! ↩️ Inverse for `InsertTableRow`.
use super::InsertTableRow;
use crate::schema::mutations::NoteMutation;
use crate::schema::mutations::RemoveTableRow;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &InsertTableRow, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Table { .. }) => vec![NoteMutation::RemoveTableRow(RemoveTableRow { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
