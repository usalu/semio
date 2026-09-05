//! ↩️ Inverse for `InsertTableColumn`.
use super::InsertTableColumn;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::schema::mutations::RemoveTableColumn;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &InsertTableColumn, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Table { .. }) => vec![NoteMutation::RemoveTableColumn(RemoveTableColumn { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
