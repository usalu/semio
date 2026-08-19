//! ↩️ Inverse for `InsertTableRow`.
use super::mutation::InsertTableRow;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::schema::mutations::RemoveTableRow;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &InsertTableRow, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Table { .. }) => vec![NoteMutation::RemoveTableRow(RemoveTableRow { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
