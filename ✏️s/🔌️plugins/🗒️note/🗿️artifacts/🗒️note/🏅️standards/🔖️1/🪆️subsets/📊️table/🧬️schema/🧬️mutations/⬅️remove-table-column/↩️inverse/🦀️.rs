//! ↩️ Inverse for `RemoveTableColumn`.
use super::RemoveTableColumn;
use crate::schema::mutations::{InsertTableColumn, NoteMutation};
use crate::{NoteBlockNode, NoteSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveTableColumn, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok(match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(NoteBlockNode::Table { columns, rows, .. }) if columns.len() > 1 => vec![NoteMutation::InsertTableColumn(InsertTableColumn {
            id: payload.id.clone(),
            name: columns.last().cloned(),
            cells: Some(rows.iter().filter_map(|line| line.last().cloned()).collect()),
        })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
