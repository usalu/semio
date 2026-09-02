//! ↩️ Inverse for `RemoveTableRow`.
use super::RemoveTableRow;
use crate::artifacts::note::schema::mutations::InsertTableRow;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &RemoveTableRow, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Table { rows, .. }) if rows.len() > 1 => vec![NoteMutation::InsertTableRow(InsertTableRow { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
