//! ↩️ Inverse for `RemoveTableColumn`.
use super::RemoveTableColumn;
use crate::standards::v1::subsets::table::schema::mutations::insert_table_column::InsertTableColumn;
use crate::{NoteBlockNode, NoteSnapshot};

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RemoveTableColumn, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::table::schema::mutations::insert_table_column::InsertTableColumn>,
{
    Ok(match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(NoteBlockNode::Table { columns, rows, .. }) if columns.len() > 1 => vec![Op::from(InsertTableColumn {
            id: payload.id.clone(),
            name: columns.last().cloned(),
            cells: Some(rows.iter().filter_map(|line| line.last().cloned()).collect()),
        })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
