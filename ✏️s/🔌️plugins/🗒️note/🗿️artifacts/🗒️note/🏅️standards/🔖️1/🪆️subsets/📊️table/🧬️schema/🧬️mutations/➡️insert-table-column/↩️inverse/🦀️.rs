//! ↩️ Inverse for `InsertTableColumn`.
use super::InsertTableColumn;
use crate::schema::mutations::NoteMutation;
use crate::schema::mutations::RemoveTableColumn;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &InsertTableColumn, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Table { .. }) => vec![NoteMutation::RemoveTableColumn(RemoveTableColumn { id: payload.id.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
