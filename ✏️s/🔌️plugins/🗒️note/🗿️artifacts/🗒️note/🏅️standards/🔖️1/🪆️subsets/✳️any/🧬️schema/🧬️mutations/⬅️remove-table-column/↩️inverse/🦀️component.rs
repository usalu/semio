//! ↩️ Inverse for `RemoveTableColumn`.
use super::mutation::RemoveTableColumn;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::schema::mutations::InsertTableColumn;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &RemoveTableColumn, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Table { columns, .. }) if columns.len() > 1 => vec![NoteMutation::InsertTableColumn(InsertTableColumn { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
