//! ↩️ Inverse for `RemoveTableColumn`.
use super::RemoveTableColumn;
use crate::schema::mutations::InsertTableColumn;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveTableColumn, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Table { columns, .. }) if columns.len() > 1 => vec![NoteMutation::InsertTableColumn(InsertTableColumn { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
